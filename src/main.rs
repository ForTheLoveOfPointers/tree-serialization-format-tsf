use std::io::{self, BufReader};
use crate::lexer::lexer_types::Token;

pub mod ast;
pub mod lexer;


fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lex = crate::lexer::lexer_types::Lexer::new(reader);

    loop {
        let tok = lex.scan();
        println!("{:?}", tok);

        if tok == Token::Eof { return; }
    }
}
