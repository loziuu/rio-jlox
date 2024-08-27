use std::any::Any;

type InterpreterResult = Result<Box<dyn Any>, super::error::LuxError>;
use super::{
    error::LuxError,
    token::{TokenLiteral, TokenType},
    visitor::{SafeVisitor, Visitor},
    Expr,
};

struct Interpreter;

impl SafeVisitor<Box<dyn Any>> for Interpreter {
    fn visit(&self, expr: &Expr) -> InterpreterResult {
        match expr {
            Expr::Literal(val) => match val {
                TokenLiteral::Str(v) => Ok(Box::new(v.clone())),
                TokenLiteral::Num(v) => Ok(Box::new(*v)),
                TokenLiteral::Bool(v) => Ok(Box::new(*v)),
            },
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary(token, expr) => {
                let c = self.evaluate(expr)?;

                match token.t {
                    TokenType::Minus => {
                        self.check_num_operand(TokenType::Minus, c.as_ref())?;
                        Ok(Box::new(-c.downcast_ref::<f64>().unwrap()))
                    }
                    TokenType::Bang => Ok(Box::new(!c.downcast_ref::<bool>().unwrap())),
                    _ => todo!(),
                }
            }
            Expr::Binary(a, token, b) => {
                let left = self.evaluate(a)?;
                let right = self.evaluate(b)?;

                match token.t {
                    TokenType::Minus => {
                        self.check_num_operands(TokenType::Minus, left.as_ref(), right.as_ref())?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                - right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    TokenType::Slash => {
                        self.check_num_operands(TokenType::Slash, left.as_ref(), right.as_ref())?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                / right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    TokenType::Star => {
                        self.check_num_operands(TokenType::Slash, left.as_ref(), right.as_ref())?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                * right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    TokenType::Plus => {
                        if left.is::<String>() && right.is::<String>() {
                            Ok(Box::new(format!(
                                "{}{}",
                                left.downcast_ref::<String>().unwrap(),
                                right.downcast_ref::<String>().unwrap()
                            )))
                        } else if left.is::<f64>() && left.is::<f64>() {
                            self.check_num_operands(
                                TokenType::Plus,
                                left.as_ref(),
                                right.as_ref(),
                            )?;
                            Ok(Box::new(
                                left.downcast_ref::<f64>().unwrap()
                                    + right.downcast_ref::<f64>().unwrap(),
                            ))
                        } else {
                            return Err(LuxError::Interpreter(
                                TokenType::Plus,
                                "Operand must be either numbers or strings.",
                            ));
                        }
                    }
                    TokenType::Greater => {
                        self.check_num_operands(TokenType::Greater, left.as_ref(), right.as_ref())?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                > right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    TokenType::GreaterEqual => {
                        self.check_num_operands(
                            TokenType::GreaterEqual,
                            left.as_ref(),
                            right.as_ref(),
                        )?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                >= right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    TokenType::Less => {
                        self.check_num_operands(TokenType::Less, left.as_ref(), right.as_ref())?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                < right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    TokenType::LessEqual => {
                        self.check_num_operands(
                            TokenType::LessEqual,
                            left.as_ref(),
                            right.as_ref(),
                        )?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                <= right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    TokenType::Equal => {
                        self.check_num_operands(
                            TokenType::LessEqual,
                            left.as_ref(),
                            right.as_ref(),
                        )?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                == right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    TokenType::BangEqual => {
                        self.check_num_operands(
                            TokenType::LessEqual,
                            left.as_ref(),
                            right.as_ref(),
                        )?;
                        Ok(Box::new(
                            left.downcast_ref::<f64>().unwrap()
                                != right.downcast_ref::<f64>().unwrap(),
                        ))
                    }
                    _ => panic!("Unsupported binary operation"),
                }
            }
            Expr::Conditional(truthy, a, b) => {
                let predicate = self.evaluate(truthy)?;

                if *predicate.downcast_ref::<bool>().unwrap() {
                    self.evaluate(a)
                } else {
                    self.evaluate(b)
                }
            }
        }
    }

    fn visit_mut(&mut self, _: &Expr) -> InterpreterResult {
        Err(LuxError::Runtime)
    }
}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> InterpreterResult {
        self.visit(expr)
    }

    fn check_num_operand(&self, token: TokenType, operand: &dyn Any) -> Result<(), LuxError> {
        if !operand.is::<f64>() {
            return Err(LuxError::Interpreter(token, "Operand must be a number."));
        }
        Ok(())
    }

    fn check_num_operands(
        &self,
        token: TokenType,
        left: &dyn Any,
        right: &dyn Any,
    ) -> Result<(), LuxError> {
        if !left.is::<f64>() || !right.is::<f64>() {
            return Err(LuxError::Interpreter(token, "Operands must be numbers."));
        }
        Ok(())
    }
}

trait Test {
    fn test(&self);
}
