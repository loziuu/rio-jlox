use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TokenType {
    // Single characters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Classs,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub(crate) struct Token {
    t: TokenType,
    lexeme: String,
    line: usize,
    literal: String,
}

impl Token {
    pub fn new(t: TokenType, lexeme: String, literal: &str, line: usize) -> Self {
        Token {
            t,
            lexeme: lexeme.to_string(),
            line,
            literal: literal.to_string(),
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.t
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.t, self.lexeme, self.literal)
    }
}
