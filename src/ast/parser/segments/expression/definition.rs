use tracing::trace;

use crate::ast::parser::{AstParser, ParseResult};

impl AstParser {
    pub fn parse_expression(&mut self) -> ParseResult {
        trace!("parse_expression");
        self.parse_arithmetic()
    }
}
