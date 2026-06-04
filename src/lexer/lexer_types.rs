use std::io::BufRead;
use std::mem;

#[derive(PartialEq, Debug)]
pub enum Token {
    Depth(String),
    Identifier(String),
    Text(String),
    TextDelimit,
    Assign,
    NewLine,
    Eof,
}

#[derive(PartialEq)]
enum LexerState {
    Normal,
    StartOfLine,
    InString,
}

enum ScanOutcome {
    Token(Token),
    Continue,
    Eof,
}


pub struct Lexer<R: BufRead> {
    reader: R,
    state: LexerState,
}

impl<R: BufRead> Lexer<R> {
    /* 
        MAIN FUNCTIONS
    */
    pub fn new(reader: R) -> Self {
        Lexer {
            reader,
            state: LexerState::StartOfLine,
        }
    }

    pub fn scan(&mut self) -> Token {
        let mut accumulator = String::new();
        loop {
            let outcome = match self.state {
                LexerState::Normal => self.handle_normal(&mut accumulator),
                LexerState::StartOfLine => self.handle_start_of_line(&mut accumulator),
                LexerState::InString => self.handle_in_string(&mut accumulator),
            };
            match outcome {
                ScanOutcome::Token(tok) => return tok,
                ScanOutcome::Eof => return Token::Eof,
                ScanOutcome::Continue => {},
            }
        }
    }

    /* 
        HELPER FUNCTIONS
    */

    fn peek_byte(&mut self) -> Option<u8> {
        let buf = self.reader.fill_buf().expect("IO error reading input");
        buf.first().copied()
    }

    fn next_byte(&mut self) -> Option<u8> {
        let byte = self.peek_byte()?;
        self.reader.consume(1);
        Some(byte)
    }

    fn handle_normal(&mut self, acc: &mut String) -> ScanOutcome {
        let Some(b) = self.peek_byte() else { return ScanOutcome::Eof };

        if b == b'\n' {
            if !acc.is_empty() {
                return ScanOutcome::Token(Token::Identifier(mem::take(acc)));
            }
            self.reader.consume(1);
            self.state = LexerState::StartOfLine;
            return ScanOutcome::Token(Token::NewLine);
        }
        if b == b' ' {
            self.reader.consume(1);
            if !acc.is_empty() {
                return ScanOutcome::Token(Token::Identifier(mem::take(acc)));
            }
            return ScanOutcome::Continue;
        }
        if b == b'=' {
            if !acc.is_empty() {
                return ScanOutcome::Token(Token::Identifier(mem::take(acc)));
            }
            self.reader.consume(1);
            return ScanOutcome::Token(Token::Assign);
        }
        if b == b'"' {
            self.reader.consume(1);
            self.state = LexerState::InString;
            return ScanOutcome::Continue;
        }
        acc.push(b as char);
        self.reader.consume(1);
        ScanOutcome::Continue
    }

    fn handle_start_of_line(&mut self, acc: &mut String) -> ScanOutcome {
        let Some(b) = self.next_byte() else { return ScanOutcome::Eof };

        if b == b' ' {
            self.state = LexerState::Normal;
            return ScanOutcome::Token(Token::Depth(mem::take(acc)));
        }
        if b.is_ascii_digit() {
            acc.push(b as char);
        }
        ScanOutcome::Continue
    }

    fn handle_in_string(&mut self, acc: &mut String) -> ScanOutcome {
        let Some(b) = self.next_byte() else { return ScanOutcome::Eof };

        if b == b'"' {
            self.state = LexerState::Normal;
            return ScanOutcome::Token(Token::Text(mem::take(acc)));
        }
        acc.push(b as char);
        ScanOutcome::Continue
    }
}
