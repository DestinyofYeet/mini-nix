use crate::ast::types::{Binary, Grouping, Literal, Unary};

pub trait Visitor<TY> {
    fn visit_binary<L: Expr, R: Expr>(&self, b: &Binary<L, R>) -> TY;
    fn visit_grouping<E: Expr>(&self, g: &Grouping<E>) -> TY;
    fn visit_unary<R: Expr>(&self, u: &Unary<R>) -> TY;
    fn visit_literal(&self, l: &Literal) -> TY;
}

pub trait Expr {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY;
}
