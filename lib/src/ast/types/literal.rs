use crate::{
    ast::types::expression::{Expr, Visitor},
    lexer::token::Token,
};

pub struct Literal {
    pub literal: Token,
}

impl Expr for Literal {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_literal(self)
    }
}
