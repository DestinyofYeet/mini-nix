use tracing::trace;

use crate::ast::parser::{AstParser, ParseResult, error::SyntaxError};

impl AstParser {
    pub fn parse_expression(&mut self) -> ParseResult {
        trace!("parse_expression");
        let mut errors: Vec<SyntaxError> = Vec::new();

        match self.parse_expression_no_assignment() {
            Ok(value) => {
                trace!("expr: {value:?}");
                return Ok(value);
            }
            Err(e) => errors.push(e),
        }

        match self.parse_assignment() {
            Ok(value) => {
                trace!("expr: {value:?}");
                return Ok(value);
            }
            Err(e) => {
                errors.push(e);
            }
        };

        Err(self.craft_error(format!(
            "Expected expression or assignment\n{}",
            errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )))
    }
}
