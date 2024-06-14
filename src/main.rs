use crate::compiler::Compiler;
use crate::parser::Parser;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

mod compiler;
mod grammar;
mod lexer;
mod parser;
mod symbol_table;

fn main() {
    let filename = args().nth(1).unwrap();

    let mut data = String::new();
    File::open(&filename)
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let content = data.chars().collect::<Vec<char>>();

    let mut parser = Parser::new(&content);

    let mut file = File::create(Path::new(&filename).with_extension("vm")).unwrap();

    let class = parser.next_class().unwrap();
    let mut compiler = Compiler::new();
    let commands = compiler.compile_class(class).unwrap();
    writeln!(file, "{}", commands.join("\n")).unwrap();
}
