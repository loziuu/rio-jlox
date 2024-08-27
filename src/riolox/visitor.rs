use super::{error::LuxError, Expr};

pub trait Visitor<R: Sized> {
    fn visit(&self, expr: &Expr) -> R;
    fn visit_mut(&mut self, expr: &Expr) -> R;
}

// Visitor but doesn't panic
pub trait SafeVisitor<R: Sized> {
    fn visit(&self, expr: &Expr) -> Result<R, LuxError>;
    fn visit_mut(&mut self, expr: &Expr) -> Result<R, LuxError>;
}
