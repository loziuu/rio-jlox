use std::rc::Rc;

use super::{
    error,
    token::{Token, TokenLiteral, TokenType},
};

type ParseResult = Result<Rc<Expr>, ParseError>;

#[derive(Debug)]
pub enum Expr {
    Literal(TokenLiteral), // Is it really?
    Unary(Token, Rc<Expr>),
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
    Conditional(Rc<Expr>, Rc<Expr>, Rc<Expr>),
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

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            ParseError::Generic(_, msg) => msg.to_string(),
        }
    }
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

    pub fn parse(&mut self) -> Option<Rc<Expr>> {
        self.expression().ok()
    }

    fn expression(&mut self) -> ParseResult {
        self.comma_expression()
    }

    fn comma_expression(&mut self) -> ParseResult {
        let mut expr = self.ternary()?;

        while self.match_token(&[TokenType::Comma]) {
            let token = self.previous();
            let right = self.equality()?;
            expr = Rc::new(Expr::Binary(expr, token.as_ref().clone(), right));
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> ParseResult {
        let expr = self.equality()?;

        if self.match_token(&[TokenType::QuestionMark]) {
            let then_ex = self.equality()?;
            if self.match_token(&[TokenType::Colon]) {
                let else_ex = self.equality()?;
                return Ok(Rc::new(Expr::Conditional(expr, then_ex, else_ex)));
            }
            return Err(ParseError::Generic(
                Rc::new(self.previous().as_ref().clone()),
                "Colon exptected.".to_owned(),
            ));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let prev_token = self.previous();
            expr = Rc::new(Expr::Binary(
                expr,
                prev_token.as_ref().clone(),
                self.comparison()?,
            ))
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let prev_token = self.previous();

            expr = Rc::new(Expr::Binary(
                expr,
                prev_token.as_ref().clone(),
                self.term()?,
            ));
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let prev_token = self.previous();

            expr = Rc::new(Expr::Binary(
                expr,
                prev_token.as_ref().clone(),
                self.factor()?,
            ));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let prev_token = self.previous();

            expr = Rc::new(Expr::Binary(
                expr,
                prev_token.as_ref().clone(),
                self.unary()?,
            ));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let previous = self.previous();
            return Ok(Rc::new(Expr::Unary(
                previous.as_ref().clone(),
                self.unary()?,
            )));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult {
        if self.match_token(&[TokenType::False]) {
            return Ok(Rc::new(Expr::Literal(TokenLiteral::Bool(false))));
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Rc::new(Expr::Literal(TokenLiteral::Bool(true))));
        }

        if self.match_token(&[TokenType::Number]) {
            let previous = self.previous();
            return Ok(Rc::new(Expr::Literal(previous.literal().clone())));
        }

        if self.match_token(&[TokenType::String]) {
            let previous = self.previous();
            return Ok(Rc::new(Expr::Literal(previous.literal().clone())));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            if self.match_token(&[TokenType::RightParen]) {
                return Ok(Rc::new(Expr::Grouping(expr)));
            }

            // Change to consume I guess?
            return Self::error(ParseError::Generic(
                self.previous(),
                "Expected ')' after expression".to_owned(),
            ));
        }

        if self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let prev = self.previous();
            let _ = self.equality();
            return Self::error(ParseError::Generic(
                prev,
                "Expected left hand operand".to_owned(),
            ));
        }

        if self.match_token(&[TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual]) {
            let prev = self.previous();
            let _ = self.comparison();
            return Self::error(ParseError::Generic(
                prev,
                "Expected left hand operand".to_owned(),
            ));
        }

        if self.match_token(&[TokenType::Plus]) {
            let prev = self.previous();
            let _ = self.term();
            return Self::error(ParseError::Generic(
                prev,
                "Expected left hand operand".to_owned(),
            ));
        }

        if self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let prev = self.previous();
            let _ = self.factor();
            return Self::error(ParseError::Generic(
                prev,
                "Expected left hand operand".to_owned(),
            ));
        }

        Self::error(ParseError::Generic(
            self.peek().clone().into(),
            "Expected expression.".to_owned(),
        ))
    }

    fn error(error: ParseError) -> ParseResult {
        let token = error.token();
        super::error(token.line(), &error.to_string());
        Err(error)
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.finished() {
            if self.previous().token_type() == &TokenType::Semicolon {
                return;
            }
            match self.peek().token_type() {
                TokenType::Return
                | TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print => return,
                _ => {}
            }

            self.advance();
        }
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
