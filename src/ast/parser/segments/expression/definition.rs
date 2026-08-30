use tracing::trace;

use crate::ast::parser::{AstParser, ParseResult, error::AstError};

impl AstParser {
    pub fn parse_expression(&mut self) -> ParseResult {
        trace!("parse_expression");

        let assignment: AstError = match self.parse_assignment() {
            Ok(value) => return Ok(value),
            Err(errors) => self.craft_error("Failed to parse assignment", errors),
        };

        let expr_no_assign_err: AstError = match self.parse_expression_no_assignemnt() {
            Ok(value) => return Ok(value),
            Err(e) => self.craft_error("Failed to parse expression_no_assignment", e),
        };

        Err(vec![assignment, expr_no_assign_err])
    }
}
