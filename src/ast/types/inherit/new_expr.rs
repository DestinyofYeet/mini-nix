use crate::{
    ast::types::{Expression, inherit::definition::Inherit},
    lexer::token::Token,
};

impl Inherit {
    pub fn new_expr(
        inherit_from: impl Into<Option<Expression>>,
        inherit_values: Vec<Token>,
    ) -> Expression {
        let inherit = Self {
            inherit_from: inherit_from.into().map(Box::new),
            inherit_values,
        };

        Expression::Inherit(inherit)
    }
}
