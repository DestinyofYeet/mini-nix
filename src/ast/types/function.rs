use crate::{
    ast::types::{Expr, Expression, Preamble},
    lexer::token::Token,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    pub identifier: Token,
    pub preamble: Option<Preamble>,
    pub expr: Box<Expression>,
}

impl Function {
    pub fn new_expr(
        identifier: Token,
        preamble: impl Into<Option<Preamble>>,
        expr: Expression,
    ) -> Expression {
        let func = Self {
            identifier,
            preamble: preamble.into(),
            expr: Box::new(expr),
        };

        Expression::Function(func)
    }
}

impl Expr for Function {
    fn accept<TY, T: super::Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_function(self)
    }
}
