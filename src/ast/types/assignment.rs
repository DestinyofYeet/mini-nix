use crate::{
    ast::types::{Expr, Expression},
    lexer::token::Token,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Assignment {
    pub identifier: Token,
    pub expr: Box<Expression>,
}

impl Assignment {
    pub fn new_expr(identifier: Token, expr: Expression) -> Expression {
        let assignment = Self {
            identifier,
            expr: Box::new(expr),
        };

        Expression::Assignment(assignment)
    }
}

impl Expr for Assignment {
    fn accept<TY, T: super::Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_assignment(self)
    }
}
