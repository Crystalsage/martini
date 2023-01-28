use std::io::{BufRead, BufReader};
use std::io::Lines;

use std::fs::File;

mod lexer;
mod parser;

use crate::lexer::{lex, Token};
use crate::parser::{INIValueType, Section};
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

fn get_all_comments(ini: INI) -> Vec<String> { 
    return ini.comments;
}

pub fn read_file(filename: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    return Ok(reader.lines()); 
}

impl INI {
    pub fn parse_ini(filename: &str) -> Self {
        let lines: Vec<String> = read_file(filename)
            .unwrap()
            .map(|line| line.unwrap())
            .collect();

        let tokens: Vec<Token> = lex(&lines);

        let mut ini = INI { content: lines, sections: Vec::new(), comments: Vec::new() };
        parse(&mut ini, tokens);
        // dbg!(&ini);

        return ini;
    }
}
