use crate::ast::types::{Expr, Expression};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PreambleLetIn {
    pub expr: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PreambleWith {
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PreambleType {
    LetIn(PreambleLetIn),
    With(PreambleWith),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Preamble {
    pub kind: PreambleType,
    pub expr: Box<Expression>,
}

impl Preamble {
    pub fn create(preamble: PreambleType, expr: Expression) -> Expression {
        Expression::Preamble(Self {
            kind: preamble,
            expr: Box::new(expr),
        })
    }
}

impl Expr for Preamble {
    fn accept<TY, T: super::Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_preamble(self)
    }
}
