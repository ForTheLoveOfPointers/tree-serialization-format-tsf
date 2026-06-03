use std::str::Chars;


#[derive(PartialEq, Debug)]
pub enum Tokens {
    Depth,
    Identifier,
    Text,
    TextDelimit,
    Assign,
    NewLine,
    Eof,
}

#[derive(PartialEq)]
enum LexerState {
    Normal,
    StartOfLine,
    InString
}

pub struct Lexer<'a> {
    char: Chars<'a>,
    state: LexerState
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            char: source.chars(),
            state: LexerState::StartOfLine
        }
    }

    // Goes char by char to get new tokens
    // FSM
    pub fn scan(&mut self) -> Tokens {
    

        while let Some(c) = self.char.next() {

            match self.state {
                LexerState::InString => {
                    if c == '"' {
                        self.state = LexerState::Normal;
                        return Tokens::TextDelimit;
                    }

                    return Tokens::Text;
                },
                LexerState::Normal => {
                    if c == ' ' {continue;}

                    if c == '\"' {
                        self.state = LexerState::InString;
                        return Tokens::TextDelimit;
                    }

                    if c == '\n' {
                        self.state = LexerState::StartOfLine;
                        return Tokens::NewLine;
                    }

                    

                    return Tokens::Identifier;
                },
                LexerState::StartOfLine => {
                    if c.is_digit(10) {return Tokens::Depth}
                    if c == ' ' { self.state = LexerState::Normal; }
                }
            }


        }


        Tokens::Eof
    }
}