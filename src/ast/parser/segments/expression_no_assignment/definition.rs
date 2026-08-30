use tracing::trace;

use crate::ast::parser::{AstParser, ParseResult, error::AstError};

impl AstParser {
    pub fn parse_expression_no_assignemnt(&mut self) -> ParseResult {
        trace!("parse_expression_no_assignemnt");

        let function_err: AstError = match self.parse_function() {
            Ok(value) => return Ok(value),
            Err(errors) => self.craft_error("Failed to parse function:", errors),
        };

        let arithmetic_err: AstError = match self.parse_arithmetic() {
            Ok(value) => return Ok(value),
            Err(errors) => self.craft_error("Failed to parse arithmetic:", errors),
        };

        // todo!("missing 'attrset'");

        Err(vec![arithmetic_err, function_err])
    }
}
