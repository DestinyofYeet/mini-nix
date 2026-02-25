use tracing::trace;

use crate::ast::parser::{AstParser, ParseResult};

impl AstParser {
    pub fn parse_primary_or_identifier(&mut self) -> ParseResult {
        trace!("parse_primary_or_identifier");

        if let Ok(value) = self.parse_primary() {
            trace!("expr: {value:?}");
            return Ok(value);
        }

        if let Ok(value) = self.parse_identifier() {
            trace!("expr: {value:?}");
            return Ok(value);
        }

        Err(self.craft_error("Expected primary, arithemtic or identifier"))
    }
}
