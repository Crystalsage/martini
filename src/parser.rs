use std::collections::HashSet;

use crate::{INI, Property, Section, Token, ParseStrategy};

/// Manages the context of the INI Parser. This is used while parsing the names to 
struct INIContext {
    current_section: Option<Section>,
    current_property: Option<Property>,
}

impl INIContext {
    fn new() -> Self {
        INIContext { 
            current_section: 
                match cfg!(feature="globalprops") {
                    true => Some(Section::create_section("martini_global".to_string())),
                    false => None,
                },
            current_property: None,
        }
    }
}




/// Wrapper types 
#[derive(Debug, Clone)]
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

fn get_property(section: &Section, name: String) -> Option<&Property> {
    let props = &section.properties;
    for property in props {
        if property.key == name {
            return Some(property);
        }
    }

    return None;
}

pub fn parse(ini: &mut INI, tokens: Vec<Token>) {
    let mut token_iterator = tokens.iter();
    let mut ctx: INIContext = INIContext::new();
    let mut seen_properties: HashSet<String> = HashSet::new();

    loop {
        let token: Option<&Token> = token_iterator.next();

        // If we've reached the end of the iterator, push the last section and return.
        if token.is_none() {
            ini.sections.push(ctx.current_section.unwrap());
            return;
        }

        match token.unwrap() {
            Token::Comment(content) => {
                if !content.is_empty() {
                    ini.comments.push(content.to_string());
                }
            },

            Token::Section(name) => {
                // Add the previous section that was being parsed into global sections.
                if ctx.current_section.is_some() {
                    if ctx.current_section.as_ref().unwrap().name.contains(".") && cfg!(feature="subsections"){
                        let what = ini.sections.last_mut().unwrap();
                        what.children.push(ctx.current_section.unwrap());
                    } else {
                        ini.sections.push(ctx.current_section.unwrap());
                    }

                }
                ctx.current_section = Some(Section::create_section(name.to_owned()));
            },

            Token::MapsTo =>  continue,

            Token::Name(name) => {
                if ctx.current_section.is_none() {
                    if ctx.current_property.is_some() && !cfg!(feature="globalprops"){
                        panic!("Global properties not allowed. Please enable crate feature `globalprops`.");
                    }

                    ctx.current_section = Some(Section::create_section(name.to_owned()));
                    continue;
                }

                // Otherwise, if a section exists contextually, then the name probably belongs to a
                // property.
                if ctx.current_property.is_none() {
                    if seen_properties.contains(name) {
                        match ini.parse_strategy {
                            ParseStrategy::AllowDuplicates => {
                                ctx.current_property = Some(Property::new_with_key(name.to_owned()));
                                seen_properties.insert(name.to_string());
                            },
                            ParseStrategy::IgnoreDuplicates => {
                                continue;
                            },
                            ParseStrategy::OverwriteDuplicates => {
                                // Remove old property 
                                let mut props = ctx.current_section.clone().unwrap().properties;
                                let mut old_idx = 0;
                                for (idx, property) in props.iter().enumerate() {
                                    if &property.key == name {
                                        old_idx = idx;
                                    }
                                }
                                props.remove(old_idx);

                                // Reassign the removed section
                                ctx.current_section.as_mut().unwrap().properties = props;

                                // Continue with the new property
                                ctx.current_property = Some(Property::new_with_key(name.to_owned()));
                                seen_properties.insert(name.to_string());
                            },
                        }
                    } else {
                        ctx.current_property = Some(Property::new_with_key(name.to_owned()));
                        seen_properties.insert(name.to_string());
                    }

                } else {
                    // If there is a property in the current context, then we are past a `MapsTo`
                    // token and the current name is probably a property value.
                    //
                    // dbg!(&ctx.current_property.as_ref().unwrap().key);
                    if name.is_empty() && !cfg!(feature="blankprops") {
                        panic!("Blank properties not allowed. Please enable crate feature `blankprops`");
                    }
                    
                    // Before setting the value, check if the property has been seen before in the 
                    // current section. If so, then we consider the parsing strategy.
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
