use crate::ast::types::{Assignment, Binary, Function, Grouping, List, Literal, Preamble, Unary};

pub trait Visitor<TY> {
    fn visit_binary(&self, b: &Binary) -> TY;
    fn visit_grouping(&self, g: &Grouping) -> TY;
    fn visit_unary(&self, u: &Unary) -> TY;
    fn visit_literal(&self, l: &Literal) -> TY;
    fn visit_preamble(&self, p: &Preamble) -> TY;
    fn visit_list(&self, l: &List) -> TY;
    fn visit_function(&self, f: &Function) -> TY;
    fn visit_assignment(&self, a: &Assignment) -> TY;
}

pub trait Expr {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Preamble(Preamble),
    List(List),
    Function(Function),
    Assignment(Assignment),
}

impl Expr for Expression {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        match self {
            Expression::Binary(binary) => binary.accept(visitor),
            Expression::Grouping(grouping) => grouping.accept(visitor),
            Expression::Literal(literal) => literal.accept(visitor),
            Expression::Unary(unary) => unary.accept(visitor),
            Expression::Preamble(preamble) => preamble.accept(visitor),
            Expression::List(list) => list.accept(visitor),
            Expression::Function(function) => function.accept(visitor),
            Expression::Assignment(assignment) => assignment.accept(visitor),
        }
    }
}
