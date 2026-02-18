use crate::{
    ast::types::expression::{Expr, Visitor},
    lexer::token::Token,
};

pub struct Unary<R>
where
    R: Expr,
{
    pub operator: Token,
    pub right: R,
}

impl<R: Expr> Expr for Unary<R> {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_unary(self)
    }
}
