use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Expr, Expression},
    },
    lexer::token::types::{LiteralToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_expression(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        trace!("parse_expression");
        let mut errors: Vec<SyntaxError> = Vec::new();

        let expr_no_assignment = match self.parse_expression_no_assignment() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);
                None
            }
        };

        let expr_assignment = match self.parse_assignment() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);
                None
            }
        };

        if let Some(expr) = expr_no_assignment {
            return Ok(expr);
        }

        if let Some(expr) = expr_assignment {
            return Ok(expr);
        }

        errors.push(self.craft_error("Expected expression or assignment"));
        Err(errors)
    }
}
