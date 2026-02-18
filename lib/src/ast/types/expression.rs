use crate::ast::types::{Binary, Grouping, Literal, Unary};

pub trait Visitor<TY> {
    fn visit_binary(&self, b: &Binary) -> TY;
    fn visit_grouping(&self, g: &Grouping) -> TY;
    fn visit_unary(&self, u: &Unary) -> TY;
    fn visit_literal(&self, l: &Literal) -> TY;
}

pub trait Expr {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY;
}

pub enum Expression {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Expr for Expression {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        match self {
            Expression::Binary(binary) => binary.accept(visitor),
            Expression::Grouping(grouping) => grouping.accept(visitor),
            Expression::Literal(literal) => literal.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
        }
    }
}
