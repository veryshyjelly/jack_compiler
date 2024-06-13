use crate::parser::Parser;
use std::env::args;
use std::fs::File;
use std::io::Read;

mod grammar;
mod lexer;
mod parser;

fn main() {
    let filename = args().nth(1).unwrap();

    let mut data = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let content = data.chars().collect::<Vec<char>>();

    let mut parser = Parser::new(&content);

    while let Ok(_class) = parser.next_class() {}
}
