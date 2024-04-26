use super::parser::{Expr, Visitor};

struct AstPrinter;

impl AstPrinter {
    fn print(&self, expr: &Expr) -> String {
        expr.visit(self)
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut val = format!("{name}");

        for expr in exprs {
            val = format!("{} {}", val, expr.visit(self))
        }

        format!("({val})")
    }
}

impl Visitor<String> for AstPrinter {
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(val) => val.to_string(),
            Expr::Unary(token, expr) => self.parenthesize(token.lexeme(), vec![expr.as_ref()]),
            Expr::Binary(left, op, right) => {
                self.parenthesize(op.lexeme(), vec![left.as_ref(), right.as_ref()])
            }
            Expr::Grouping(expr) => self.parenthesize("group", vec![expr.as_ref()]),
        }
    }

    fn visit_mut(&mut self, expr: &Expr) -> String {
        self.visit(expr)
    }
}

struct RpnPrinter;

impl RpnPrinter {
    fn print(&self, expr: &Expr) -> String {
        expr.visit(self)
    }
}

impl Visitor<String> for RpnPrinter {
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(val) => val.to_owned(),
            Expr::Unary(token, expr) => format!("{}{}", token.lexeme(), self.visit(expr)),
            Expr::Binary(left, token, right) => format!("{} {} {}", self.visit(left), self.visit(right), token.lexeme()),
            Expr::Grouping(expr) => self.visit(expr),
        }
    }

    fn visit_mut(&mut self, expr: &Expr) -> String {
        self.visit(expr)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use tests::{parser::Expr, token::{Token, TokenType}};

    use crate::riolox::*;

    use super::*;

    #[test]
    fn test_first() {
        let minus = 
            Token::new(TokenType::Minus, "-".to_owned(), "-", 1);
        let unary =
            Rc::new(Expr::Unary(minus,  Rc::new(Expr::Literal("123".to_owned()))));
        let grouping =
            Rc::new(Expr::Grouping(Rc::new(Expr::Literal("45.67".to_owned()))));
        let expr = Expr::Binary(
            unary, Token::new(TokenType::Star, "*".to_owned(), "*", 1), grouping);

        let printer = AstPrinter{};
        let result = printer.print(&expr);

        assert_eq!("(* (- 123) (group 45.67))", result)
    }

    #[test]
    fn test_rpn() {
        let one = Rc::new(Expr::Literal("1".to_owned())); 
        let two  = Rc::new(Expr::Literal("2".to_owned())); 
        let three  = Rc::new(Expr::Literal("3".to_owned())); 
        let four  = Rc::new(Expr::Literal("4".to_owned())); 
        let plus = Token::new(TokenType::Plus, "+".to_owned(), "+", 1);
        let minus = Token::new(TokenType::Minus, "-".to_owned(), "-", 1);
        let times = Token::new(TokenType::Star, "*".to_owned(), "*", 1);

        let left = Expr::Grouping(Rc::new(Expr::Binary(one, plus, two)));
        let right = Expr::Grouping(Rc::new(Expr::Binary(four, minus, three)));
        let expr = Expr::Binary(Rc::new(left), times, Rc::new(right));

        let printer = RpnPrinter{};
        let result = printer.print(&expr);

        assert_eq!("1 2 + 4 3 - *", result)
    }
}
