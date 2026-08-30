use tracing::trace;

use crate::ast::parser::{AstParser, ParseResult, error::AstError};

impl AstParser {
    pub fn parse_primary_or_identifier(&mut self) -> ParseResult {
        trace!("parse_primary_or_identifier");

        let identifier_err: AstError = match self.parse_identifier() {
            Ok(value) => return Ok(value),
            Err(errors) => self.craft_error("Failed to parse identifier because:", errors),
        };

        let primary_err: AstError = match self.parse_primary() {
            Ok(value) => return Ok(value),
            Err(errors) => self.craft_error("Failed to parse primary because:", errors),
        };

        Err(vec![identifier_err, primary_err])
    }
}
