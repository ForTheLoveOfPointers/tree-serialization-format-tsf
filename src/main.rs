use std::env;
use std::io::{self, BufReader};

pub mod ast;
pub mod errors;
pub mod lexer;
pub mod parser;

fn main() {
    let debug = env::args().any(|a| a == "--debug" || a == "-d");

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut parser = parser::parser_types::Parser::new(reader);
    match parser.parse() {
        Ok(doc) => {
            if debug {
                println!("{doc:#?}");
            } else {
                println!("{doc}");
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}
