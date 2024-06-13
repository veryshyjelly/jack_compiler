use crate::parser::Parser;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

mod grammar;
mod lexer;
mod parser;

fn main() {
    let filename = args().nth(1).unwrap();

    let mut data = String::new();
    File::open(&filename)
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let content = data.chars().collect::<Vec<char>>();

    let mut parser = Parser::new(&content);

    let mut file = File::create(Path::new(&filename).with_extension("xml")).unwrap();

    let class = parser.next_class().unwrap();
    let xml_string = serde_xml_rs::to_string(&class).unwrap();
    writeln!(file, "{xml_string}").unwrap();
}
