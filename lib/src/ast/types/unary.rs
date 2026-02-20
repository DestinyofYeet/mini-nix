use crate::{
    ast::types::{
        Expression,
        expression::{Expr, Visitor},
    },
    lexer::token::Token,
};

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Unary {
    pub fn create(operator: Token, right: Expression) -> Expression {
        let unary = Self {
            operator,
            right: Box::new(right),
        };

        Expression::Unary(unary)
    }
}

impl Expr for Unary {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_unary(self)
    }
}
