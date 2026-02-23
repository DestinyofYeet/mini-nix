use crate::ast::types::{Binary, Grouping, Literal, Preamble, Unary};

pub trait Visitor<TY> {
    fn visit_binary(&self, b: &Binary) -> TY;
    fn visit_grouping(&self, g: &Grouping) -> TY;
    fn visit_unary(&self, u: &Unary) -> TY;
    fn visit_literal(&self, l: &Literal) -> TY;
    fn visit_preamble(&self, p: &Preamble) -> TY;
}

pub trait Expr {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Preamble(Preamble),
}

impl Expr for Expression {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        match self {
            Expression::Binary(binary) => binary.accept(visitor),
            Expression::Grouping(grouping) => grouping.accept(visitor),
            Expression::Literal(literal) => literal.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Preamble(preamble) => preamble.accept(visitor),
        }
    }
}
