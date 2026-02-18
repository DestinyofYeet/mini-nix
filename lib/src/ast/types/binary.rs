use crate::{
    ast::types::{Expr, Visitor},
    lexer::token::Token,
};

pub struct Binary<L, R>
where
    L: Expr,
    R: Expr,
{
    pub left: L,
    pub operator: Token,
    pub right: R,
}

impl<L: Expr, R: Expr> Expr for Binary<L, R> {
    fn accept<E, T: Visitor<E>>(&self, visitor: &T) -> E {
        visitor.visit_binary(self)
    }
}
