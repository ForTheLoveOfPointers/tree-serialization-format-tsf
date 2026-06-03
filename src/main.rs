pub mod ast;
pub mod lexer;


use crate::lexer::lexer_types::Tokens;

fn main() {
    let doc = "0 html\n1 head\n2 title \"My page\"\n1 body \"contents\"\n2 title \"My page\"";
    let mut lex = lexer::lexer_types::Lexer::new(doc);

    loop {
        let tok = lex.scan();
        println!("{:?}", tok);

        if tok == Tokens::Eof {return;}
    }
}
