use crate::{ast::types::Expression, lexer::token::Token};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Inherit {
    pub inherit_from: Option<Box<Expression>>,
    pub inherit_values: Vec<Token>,
}
