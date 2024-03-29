// The lexer parses the entire document into a stream of tokens. 
use crate::INIContent;

#[derive(Debug, PartialEq)]
pub enum Token {
    // The token that implies a comment. Default is ';'.
    Comment(String),

    Section(String),

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
            #[cfg(feature="hash_for_comments")]
            Some('#') => {
                tokens.push(Token::Comment(line[1..].trim().to_string()));
            }
            Some(';') => {
                tokens.push(Token::Comment(line[1..].trim().to_string()));
            },

            // Lex a section in one go.
            Some('[') => {
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
                tokens.push(Token::Section(name.trim().to_string()));
            },

            // Handled in Name(_) anyway. Just for the tests.
            #[cfg(not(any(feature="spaceprops", feature="colonprops")))]
            Some('=') => {
                tokens.push(Token::MapsTo);
            }

            #[cfg(feature="spaceprops")]
            Some(' ') => {
                tokens.push(Token::MapsTo);
            }

            #[cfg(feature="colonprops")]
            Some(':') => {
                tokens.push(Token::MapsTo);
            }

            // Probably a property.
            Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') => {
                let (key, value) = line.split_once(
                    match () {
                        #[cfg(feature="colonprops")]
                        () => ":",
                        #[cfg(feature="spaceprops")]
                        () => " ",
                        // Otherwise just use the default mapping character ('=')
                        // #[cfg(not(any(feature="spaceprops", feature="colonprops")))]
                        () => "=",
                    }
                ).unwrap();
                tokens.push(Token::Name(key.trim().to_string()));
                tokens.push(Token::MapsTo);
                tokens.push(Token::Name(value.trim().replace('"', "").to_string()));
            }

            _ => {
                // TODO: To panic or not panic?
                continue;
            }
        }
    }

    return tokens;
}


#[cfg(test)]
mod tests {
    use super::{lex, Token};

    #[test]
    fn test_comment_token_semicolon() {
        let comment_character = vec!["; test comment".to_string()];
        let lexed_token = lex(&comment_character);

        assert_eq!(lexed_token.get(0), Some(&Token::Comment("test comment".to_string())));
    }

    #[test]
    fn test_mapsto_equals_token() {
        let maps_to_character = vec!["=".to_string()];
        let lexed_token = lex(&maps_to_character);

        assert_eq!(lexed_token.get(0), Some(&Token::MapsTo));
    }

    #[test]
    fn test_section_token() {
        let section_string = vec!["[test_section]".to_string()];
        let lexed_token = lex(&section_string);

        assert_eq!(lexed_token.get(0), Some(&Token::Section("test_section".to_string())));
    }

    #[test]
    fn test_name_tokens_for_property() {
        let property_string = vec!["key=value".to_string()];
        let lexed_token = lex(&property_string);

        assert_eq!(lexed_token.get(0), Some(&Token::Name("key".to_string())));
        assert_eq!(lexed_token.get(2), Some(&Token::Name("value".to_string())));
    }

    #[test]
    fn test_that_newline_returns_no_token() {
        let property_string = vec!["\n".to_string()];
        let lexed_token = lex(&property_string);

        assert!(lexed_token.is_empty());
    }


    #[test]
    fn test_invalid_token() {
        let property_string = vec!["`[section]`".to_string()];
        let lexed_token = lex(&property_string);

        assert!(lexed_token.is_empty());
    }
}
