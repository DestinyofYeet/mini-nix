use crate::{
    ast::types::{
        Expression,
        expression::{Expr, Visitor},
    },
    lexer::token::Token,
};

pub struct Literal {
    pub literal: Token,
}

impl Literal {
    pub fn create(literal: Token) -> Expression {
        let literal = Self { literal };

        Expression::Literal(literal)
    }
}

impl Expr for Literal {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_literal(self)
    }
}
