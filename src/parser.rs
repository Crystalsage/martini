use crate::{INI, Property, Token};

/// Manages the context of the INI Parser. This is used while parsing the names to 
struct INIContext {
    current_section: Option<Section>,
    current_property: Option<Property>,
}

impl INIContext {
    fn new() -> Self {
        INIContext { 
            current_section: None,
            current_property: None,
        }
    }
}

#[derive(Debug)]
pub struct Section {
    name: String,
    properties: Vec<Property>,
}

impl Section {
    /// Return a new named section.
    fn create_section(name: String) -> Self {
        Section { 
            name,
            properties: Vec::new(),
        }
    }

    /// Insert a property into a section.
    fn insert_property(self: &mut Self, property: Property) {
        self.properties.push(property);
    }
}




/// Wrapper types 
#[derive(Debug)]
pub enum INIValueType {
    INIString(String),
    INIInteger(i64),
    INIFloat(f64),
}

impl INIValueType {
    /// Converts a Rust data type to a INIValueType. 
    /// i64    ==> INIValueType::INIInteger(i64),
    /// f64    ==> INIValueType::INIFloat(f64),
    /// String ==> INIValueType::String(String),
    fn to_ini_type(value: &String) -> Self {
        if let Ok(res) = value.parse::<i64>() {
            return INIValueType::INIInteger(res);
        } else if let Ok(res) = value.parse::<f64>() {
            return INIValueType::INIFloat(res);
        } else {
            return INIValueType::INIString(value.to_owned());
        }
    }
}

pub fn parse(ini: &mut INI, tokens: Vec<Token>) {
    let mut token_iterator = tokens.iter();
    let mut ctx: INIContext = INIContext::new();

    loop {
        let token: Option<&Token> = token_iterator.next();

        // If we've reached the end of the iterator, push the last section and return.
        if token.is_none() {
            ini.sections.push(ctx.current_section.unwrap());
            return;
        }

        match token.unwrap() {
            Token::Comment(content) => {
                ini.comments.push(content.to_string());
            },
            Token::SectionOpen => {
                if ctx.current_section.is_some() {
                    ini.sections.push(ctx.current_section.unwrap());
                    ctx.current_section = None;
                } else {
                    // In case of the first section, `ctx.current_section` would be None.
                    continue;
                }
            },
            Token::SectionClose => continue,

            Token::MapsTo =>  continue,

            Token::Name(name) => {
                // If `ctx.current_section` is None, then there's probably a section name to be 
                // parsed. This is because we set `ctx.current_section` to `None` in the 
                // `SectionOpen` token handler. This should also handle the case of the first
                // section.
                if ctx.current_section.is_none() {
                    ctx.current_section = Some(Section::create_section(name.to_owned()));
                    continue;
                }  

                // Otherwise, if a section exists contextually, then the name probably belongs to a
                // property.
                if ctx.current_property.is_none() {
                    ctx.current_property = Some(Property::new_with_key(name.to_owned()));
                } else {
                    // If there is a property in the current context, then we are past a `MapsTo`
                    // token and the current name is probably a property value.
                    
                    let ini_value: INIValueType = INIValueType::to_ini_type(name);
                    ctx.current_property
                        .as_mut()
                        .unwrap()
                        .set_value(ini_value);

                    // Once the property is parsed completely, we can add it into `current_section`
                    // and be done with it.
                    ctx.current_section
                        .as_mut()
                        .unwrap()
                        .insert_property(ctx.current_property.unwrap());
                    ctx.current_property = None;
                }
            }
        }
    }
}
