use crate::{
    ast::types::{Expr, Expression, Visitor},
    lexer::token::Token,
};

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Binary {
    pub fn create(left: Expression, operator: Token, right: Expression) -> Expression {
        let binary = Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };

        Expression::Binary(binary)
    }
}

impl Expr for Binary {
    fn accept<E, T: Visitor<E>>(&self, visitor: &T) -> E {
        visitor.visit_binary(self)
    }
}
