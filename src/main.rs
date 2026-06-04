use std::io::{self, BufReader};
use crate::lexer::lexer_types::TokenType;

pub mod ast;
pub mod lexer;


fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lex = crate::lexer::lexer_types::Lexer::new(reader);

    loop {
        let tok = lex.scan();
        println!("{:?} : {:?}", tok.value, tok.token_t);

        if tok.token_t == TokenType::Eof { return; }
    }
}
