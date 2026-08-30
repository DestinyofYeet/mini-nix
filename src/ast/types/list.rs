use crate::ast::types::{Expr, Expression};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct List {
    pub list: Vec<Expression>,
}

impl List {
    pub fn new_expr(list: Vec<Expression>) -> Expression {
        let list = Self { list };

        Expression::List(list)
    }
}

impl Expr for List {
    fn accept<TY, T: super::Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_list(self)
    }
}
