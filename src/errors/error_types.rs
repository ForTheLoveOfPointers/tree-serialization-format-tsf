use std::fmt;
use crate::lexer::lexer_types::Token;

#[derive(Debug)]
pub enum ParseError {
    IoError(std::io::Error),
    DepthJump { current_depth: u32, target_depth: u32 },
    UnexpectedToken { expected: String, found: Token },
    InvalidValue { raw: String },
    EmptyInput,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IoError(e) => write!(f, "I/O error: {}", e),
            ParseError::DepthJump { current_depth, target_depth } => {
                write!(f, "invalid depth jump: from {} to {}", current_depth, target_depth)
            }
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "unexpected token: expected {}, found {:?}", expected, found)
            }
            ParseError::InvalidValue { raw } => {
                write!(f, "invalid attribute value: `{}`", raw)
            }
            ParseError::EmptyInput => write!(f, "empty input"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        ParseError::IoError(e)
    }
}
