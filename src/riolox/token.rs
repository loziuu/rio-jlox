use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    Class,
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

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenLiteral {
    Str(String),
    Num(f64),
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    t: TokenType,
    literal: TokenLiteral,
    line: usize,
    lexeme: String,
}

impl Token {
    pub fn new(t: TokenType, literal: String, lexeme: &str, line: usize) -> Self {
        Token {
            t,
            literal: TokenLiteral::Str(literal),
            line,
            lexeme: lexeme.to_string(),
        }
    }

    pub fn with_value(t: TokenType, literal: TokenLiteral, lexeme: &str, line: usize) -> Self {
        Token {
            t,
            literal,
            line,
            lexeme: lexeme.to_string(),
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.t
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn literal(&self) -> &TokenLiteral {
        &self.literal
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {}", self.t, self.literal, self.lexeme)
    }
}
