use tracing::trace;

use crate::ast::parser::{AstParser, ParseResult};

impl AstParser {
    pub fn parse_arithmetic_or_primary(&mut self) -> ParseResult {
        trace!("parse_arithmetic_or_primary");

        if let Ok(value) = self.parse_primary() {
            trace!("expr: {value:?}");
            return Ok(value);
        }

        if let Ok(value) = self.parse_identifier() {
            trace!("expr: {value:?}");
            return Ok(value);
        }

        if let Ok(value) = self.parse_arithmetic() {
            trace!("expr: {value:?}");
            return Ok(value);
        }

        Err(self.craft_error("Expected primary, arithemtic or identifier"))
    }
}
