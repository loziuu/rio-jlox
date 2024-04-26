use super::token::{Token, TokenType};

pub enum Expr {
    Literal(String), // Is it really?
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

impl Expr {
    pub fn visit<V, R>(&self, v: &V) -> R
    where
        R: Sized,
        V: Visitor<R>,
    {
        v.visit(self)
    }
}

pub trait Visitor<R: Sized> {
    fn visit(&self, expr: &Expr) -> R;
    fn visit_mut(&mut self, expr: &Expr) -> R;
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
                let prev_token = self.previous();
                expr = Expr::Binary(Box::new(expr), prev_token.clone(), Box::new(self.comparison()))
        }

        expr
    }



    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn comparison(&self) -> Expr {
        Expr::Literal("Test".to_owned())
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        if self.finished() {
            return false;
        }

        for t in types {
            let curr = self.peek();
            if curr.token_type() == t {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) {
        if !self.finished() {
            self.current += 1
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn finished(&self) -> bool {
        self.tokens[self.current].token_type() == &TokenType::Eof
    }
}
