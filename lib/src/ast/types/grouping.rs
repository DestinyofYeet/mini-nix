use crate::ast::types::{
    Expression,
    expression::{Expr, Visitor},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grouping {
    pub expr: Box<Expression>,
}

impl Grouping {
    pub fn create(expr: Expression) -> Expression {
        let grouping = Self {
            expr: Box::new(expr),
        };

        Expression::Grouping(grouping)
    }
}

impl Expr for Grouping {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_grouping(self)
    }
}
