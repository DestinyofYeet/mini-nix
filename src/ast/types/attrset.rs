use crate::ast::types::{Expr, Expression};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attrset {
    pub values: Vec<Expression>,
}

impl Attrset {
    pub fn new_expr(values: Vec<Expression>) -> Expression {
        let attr = Self { values };

        Expression::Attrset(attr)
    }
}

impl Expr for Attrset {
    fn accept<TY, T: super::Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_attrset(self)
    }
}
