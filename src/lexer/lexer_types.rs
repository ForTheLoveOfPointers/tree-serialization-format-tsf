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
    pub fn scan(&mut self) -> Tokens {
    

        while let Some(c) = self.char.next() {
            return match c  {
                ' ' => {
                    if self.state != LexerState::InString {
                        self.state = LexerState::Normal;
                    }
                    continue;
                },
                '\"' => {
                    if self.state == LexerState::Normal {
                        self.state = LexerState::InString;
                    } else {
                        self.state = LexerState::Normal;
                    }
                    Tokens::TextDelimit
                },
                _ => {
                    if self.state == LexerState::InString {
                        return Tokens::Text;
                    }
                    if c == '\n' {
                        self.state = LexerState::StartOfLine;
                        return Tokens::NewLine
                    } 
                    if c.is_digit(10) && self.state == LexerState::StartOfLine { 
                        return Tokens::Depth;
                    }
                    
                    Tokens::Identifier
                }
            };


        }


        Tokens::Eof
    }
}