// The lexer parses the entire document into a stream of tokens. 
use crate::INIContent;

#[derive(Debug)]
pub enum Token {
    // The token that implies a comment. Default is ';'.
    Comment(String),

    // The token that opens a section. Standard is `[`.
    SectionOpen,

    // The token that closes a section. Standard is `]`.
    SectionClose,

    // The token that maps a key to a value. Default is `=`.
    MapsTo,
    
    // A name 
    Name(String),
}

pub fn lex(lines: &INIContent) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for line in lines {
        // It's a newline.
        if line.is_empty() {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();

        match chars.get(0) {
            Some(';') => {
                tokens.push(Token::Comment(line[1..].trim().to_string()));
            },

            // Lex a section in one go.
            Some('[') => {
                tokens.push(Token::SectionOpen);

                let mut name: Vec<char> = Vec::new();
                for chr in chars[1..].iter() {
                    match chr {
                        // Ignore quotes
                        '"' => continue,

                        // End section
                        ']' => break,

                        // Letter in the name
                        _ => name.push(chr.to_owned()),
                    }
                }

                let name: String = name.into_iter().collect::<String>();
                tokens.push(Token::Name(name.trim().to_string()));

                tokens.push(Token::SectionClose);
            },

            // Probably a property.
            Some(_) => {
                let (key, value) = line.split_once('=').unwrap();
                tokens.push(Token::Name(key.trim().to_string()));
                tokens.push(Token::MapsTo);
                tokens.push(Token::Name(value.trim().replace('"', "").to_string()));
            }
            _ => {
                panic!("Found invalid character!");
            }
        }
    }

    return tokens;
}
