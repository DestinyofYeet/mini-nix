use crate::ast::types::{Binary, Expr, Expression};

#[derive(Debug, PartialEq, Eq)]
pub struct PreambleLetIn {
    expr: Vec<Binary>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PreambleWith {
    expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PreambleType {
    LetIn(PreambleLetIn),
    With(PreambleWith),
}

#[derive(Debug, PartialEq, Eq)]
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
