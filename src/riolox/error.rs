use std::{fmt::Display, rc::Rc};

use super::token::{Token, TokenType};

pub enum LuxError {
    Parser(ParseError),
    Interpreter(TokenType, &'static str),
    Runtime,
}

pub enum ParseError {
    Generic(Rc<Token>, String),
}

impl ParseError {
    pub fn token(&self) -> &Token {
        match self {
            ParseError::Generic(token, _) => token,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Generic(_, msg) => f.write_str(msg),
        }
    }
}
