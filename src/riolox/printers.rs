use super::{visitor::Visitor, Expr};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.visit(self)
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut val = name.to_string();

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
            Expr::Binary(left, op, right) => self.parenthesize(
                &op.literal().to_string(),
                vec![left.as_ref(), right.as_ref()],
            ),
            Expr::Grouping(expr) => self.parenthesize("group", vec![expr.as_ref()]),
            Expr::Conditional(if_ex, then_ex, else_ex) => {
                self.parenthesize("?", vec![if_ex, then_ex, else_ex])
            }
        }
    }

    fn visit_mut(&mut self, expr: &Expr) -> String {
        self.visit(expr)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use tests::{
        parser::Expr,
        token::{Token, TokenLiteral, TokenType},
    };

    use crate::riolox::*;

    use super::*;

    #[test]
    fn test_first() {
        let minus = Token::new(TokenType::Minus, "-".to_owned(), "-", 1);
        let unary = Rc::new(Expr::Unary(
            minus,
            Rc::new(Expr::Literal(TokenLiteral::from("123"))),
        ));
        let grouping = Rc::new(Expr::Grouping(Rc::new(Expr::Literal(TokenLiteral::from(
            "45.67",
        )))));
        let expr = Expr::Binary(
            unary,
            Token::new(TokenType::Star, "*".to_owned(), "*", 1),
            grouping,
        );

        let printer = AstPrinter {};
        let result = printer.print(&expr);

        assert_eq!("(* (- 123) (group 45.67))", result)
    }
}
