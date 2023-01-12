use crate::Section;
use crate::INI;
use crate::Token;

pub fn parse(ini: &mut INI, tokens: Vec<Token>) {
    let mut token_iterator = tokens.iter();

    loop {
        let token: Option<&Token> = token_iterator.next();

        // If we've reached the end of the iterator
        if token.is_none() {
            return;
        }

        match token.unwrap() {
            Token::Comment(content) => {
                ini.comments.push(content.to_string());
            },
            Token::SectionOpen => {
                let section_name: Option<&Token> = token_iterator.next();

                if section_name.is_none() {
                    panic!("Malformed INI! No section name after opening the section.");
                }

                if let Token::Name(section) = section_name.unwrap() {
                    ini.sections.push(Section::new(section.to_string()));
                } else {
                    panic!("Malformed INI! Token after section opening is not.");
                }
            }

            _ => {}
        }

    }

    // Parse the sections
}
