use std::rc::Rc;

use super::token::{Token, TokenLiteral, TokenType};

pub enum Expr {
    Literal(TokenLiteral), // Is it really?
    Unary(Token, Rc<Expr>),
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
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
    tokens: Vec<Rc<Token>>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Rc<Token>>) -> Parser {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Rc<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Rc<Expr> {
        let mut expr = self.comparison();

        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
                let prev_token = self.previous();
                expr = Rc::new(Expr::Binary(expr.clone(), prev_token.as_ref().clone(), self.comparison().clone()))
        }

        expr
    }

    fn comparison(&mut self) -> Rc<Expr> {
        let mut expr = self.term();

        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let prev_token = self.previous();

            expr = Rc::new(Expr::Binary(expr.clone(), prev_token.as_ref().clone(), self.term().clone()));
        }

        expr
    }

    fn term(&mut self) -> Rc<Expr> {
        let mut expr = self.factor();

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let prev_token = self.previous();

            expr = Rc::new(Expr::Binary(expr.clone(), prev_token.as_ref().clone(), self.factor().clone()));
        }

        expr
    }

    fn factor(&mut self) -> Rc<Expr> {
        let mut expr = self.unary();

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let prev_token = self.previous();

            expr = Rc::new(Expr::Binary(expr.clone(), prev_token.as_ref().clone(), self.unary().clone()));
        }

        expr
    }

    fn unary(&mut self) -> Rc<Expr> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let previous = self.previous();
            return Rc::new(Expr::Unary(previous.as_ref().clone(), self.unary()))
        }

        self.primary()
    }

    fn primary(&mut self) -> Rc<Expr> {
        // TODO: Implement this.
    }
    
    fn previous(&self) -> Rc<Token> {
        self.tokens[self.current - 1].clone()
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

    fn advance(&mut self) -> &Token {
        if !self.finished() {
            self.current += 1
        }
        &self.tokens[self.current]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn finished(&self) -> bool {
        self.tokens[self.current].token_type() == &TokenType::Eof
    }
}
