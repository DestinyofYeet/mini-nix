use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult, error::AstError},
        types::Binary,
    },
    lexer::token::types::{MathToken, TokenType},
};

impl AstParser {
    pub fn parse_expression_no_assignemnt(&mut self) -> ParseResult {
        trace!("parse_expression_no_assignemnt");

        let arithmetic_err: AstError = match self.parse_arithmetic() {
            Ok(value) => return Ok(value),
            Err(errors) => self.craft_error("Failed to parse arithmetic:", errors),
        };

        // todo!("expression_no_assignment")

        Err(vec![arithmetic_err])
    }
}
