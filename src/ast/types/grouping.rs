use crate::ast::types::{
    Expression,
    expression::{Expr, Visitor},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grouping {
    pub expr: Box<Expression>,
}

impl Grouping {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(expr: Expression) -> Expression {
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
