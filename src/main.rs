use crate::compiler::Compiler;
use crate::parser::Parser;
use std::env::args;
use std::fs::{File, read_dir};
use std::io::{Read, Write};
use std::path::Path;

mod compiler;
mod grammar;
mod lexer;
mod parser;
mod symbol_table;

fn main() {
    let path_input = args().nth(1).expect("Usage: cargo run <filename>");
    let path = Path::new(&path_input);

    let file_paths: Vec<_>;

    if path.is_dir() {
        let files = read_dir(path).unwrap();
        file_paths = files
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() && path.extension().unwrap() == "jack" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        if !file_paths
            .iter()
            .any(|x| x.file_name().unwrap() == "Main.jack")
        {
            panic!("Main.jack not found in the directory");
        }
    } else {
        // Otherwise simply convert the file
        file_paths = vec![path.with_extension("jack")];
    }
    
    for file in file_paths {
        let mut data = String::new();
        File::open(&file)
            .unwrap()
            .read_to_string(&mut data)
            .unwrap();
        let content = data.chars().collect::<Vec<char>>();

        let mut parser = Parser::new(&content);

        let mut file = File::create(file.with_extension("vm")).unwrap();

        let class = parser.next_class().unwrap();
        let mut compiler = Compiler::new();
        let commands = compiler.compile_class(class).unwrap();
        writeln!(file, "{}", commands.join("\n")).unwrap();
    }

}
