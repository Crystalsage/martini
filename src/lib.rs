use std::io::{BufRead, BufReader};
use std::io::Lines;

use std::fs::File;

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

#[derive(Debug)]
struct Context {
    // All the content of the ini file.
    content: Vec<Lines<String>>,

    // The current, ongoing section.
    current_section: Section,

    // Sections that have been parsed. At the end of the parsing stage, this 
    // vector should contain all of the sections.
    sections: Vec<Section>,
}


fn read_file(filename: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    return Ok(reader.lines()); 
}

pub fn parse_ini(filename: &str) {
    let lines = read_file(filename).unwrap();

    for line in lines {
        println!("{:?}", line.unwrap());
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
