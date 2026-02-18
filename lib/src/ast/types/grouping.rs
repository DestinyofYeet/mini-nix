use crate::ast::types::expression::{Expr, Visitor};

pub struct Grouping<E>
where
    E: Expr,
{
    pub expr: E,
}

impl<E: Expr> Expr for Grouping<E> {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_grouping(self)
    }
}
