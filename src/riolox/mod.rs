use std::rc::Rc;

use self::{
    token::{Token, TokenLiteral},
    visitor::Visitor,
};

pub(crate) mod error;
pub(crate) mod interpreter;
pub(crate) mod parser;
pub(crate) mod printers;
pub(crate) mod scanner;
pub(crate) mod token;
pub(crate) mod visitor;

pub(crate) type CompilerResult = Result<(), CompilationError>;

pub(crate) enum CompilationError {
    UndefinedError,
}

pub fn error(line: usize, msg: &str) {
    report(line, "", msg)
}

pub fn report(line: usize, error: &str, reason: &str) {
    println!("[line {}] Error {}: {}", line, error, reason)
}

#[derive(Debug)]
pub enum Expr {
    Literal(TokenLiteral), // Is it really?
    Unary(Token, Rc<Expr>),
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
    Conditional(Rc<Expr>, Rc<Expr>, Rc<Expr>),
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
