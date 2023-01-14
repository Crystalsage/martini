use std::io::{BufRead, BufReader};
use std::io::Lines;

use std::fs::File;

mod lexer;
mod parser;

use crate::lexer::lex;
use crate::lexer::Token;
use parser::parse;

type INIContent = Vec<String>;

/// The INI struct is the primary struct that's used as an interface into the 
/// INI File. 
/// It has four primary fields:
///   - Content: All the content of the INI File.
///   - Sections: All the sections and the properties contained within.
///   - Comments: All the comments of the File.
///   - ctx: Context of the parser. 
#[derive(Debug)]
pub struct INI {
    // The entire file that has been read.
    content: INIContent,

    // Sections that have been parsed. At the end of the parsing stage, this 
    // vector should contain all of the sections.
    sections: Vec<Section>,

    // All the comments of the INI file.
    comments: Vec<String>,
}

#[derive(Debug)]
enum INIValueType {
    INIString(String),
    INIInteger(i64),
    INIFloat(f64),
}

#[derive(Debug)]
struct Property {
    key: String,
    value: Option<INIValueType>,
}

impl Property {
    fn new_with_key(key: String) -> Self {
        Property { 
            key,
            value: None,
        }
    }

    fn set_value(self: &mut Self, value: INIValueType) {
        self.value = Some(value);
    }
}


#[derive(Debug)]
struct Section {
    name: String,
    properties: Vec<Property>,
}

fn get_all_comments(ini: INI) -> Vec<String> { 
    return ini.comments;
}

pub fn read_file(filename: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    return Ok(reader.lines()); 
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

impl INI {
    fn new(content: INIContent) -> Self {
        return INI {
            content: content,
            sections: Vec::new(),
            comments: Vec::new(),
        };
    }
}

impl INI {
    pub fn parse_ini(filename: &str) -> Self {
        let lines: Vec<String> = read_file(filename)
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let tokens: Vec<Token> = lex(&lines);

        let mut ini = INI::new(lines);
        parse(&mut ini, tokens);

        dbg!(&ini);

        return ini;
    }
}
