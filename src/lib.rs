use std::io::{BufRead, BufReader};
use std::io::Lines;

use std::fs::File;

mod lexer;

use crate::lexer::lex;
use crate::lexer::Token;

type INIContent = Vec<String>;

pub struct INI {
    // The entire file that has been read.
    content: INIContent,

    // Sections that have been parsed. At the end of the parsing stage, this 
    // vector should contain all of the sections.
    sections: Vec<Section>,

    // All the comments of the INI file.
    comments: Vec<String>,

    // The context of the parser.
    ctx: INIContext,
}

#[derive(Debug)]
struct INIContext {
    // The current, ongoing section.
    current_section: Section,
}

#[derive(Debug)]
enum INIValueType {
    INIString(String),
    INIInteger(i64),
}

#[derive(Debug)]
struct Property {
    key: String,
    value: INIValueType,
}

#[derive(Debug)]
struct Section {
    name: String,
    properties: Vec<Property>,
}

fn get_all_comments(ini: INI) -> Vec<String> { 
    return ini.comments;
}

fn read_file(filename: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    return Ok(reader.lines()); 
}

impl Section {
    fn new() -> Self {
        Section {
            name: String::new(),
            properties: Vec::new(),
        }
    }
}

impl INI {
    fn new(content: INIContent) -> Self {
        return INI {
            content: content,
            sections: Vec::new(),
            comments: Vec::new(),
            ctx: INIContext::new()
        };
    }
}

impl INIContext {
    fn new() -> Self {
        INIContext {
            current_section: Section::new(),
        }
    }
}

impl INI {
    pub fn parse_ini(filename: &str) -> Self {
        let lines: Vec<String> = read_file(filename)
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let tokens: Vec<Token> = lex(&lines);
        dbg!(tokens);
        return INI::new(lines);
    }
}


#[cfg(test)]
mod tests {
    use crate::read_file;

    #[test]
    fn read_file_test() {
        let filename: &str = "test_file.ini";
        assert_eq!(read_file(filename), Err);
    }
}
