use tracing::trace;

use crate::ast::{
    parser::{AstParser, ParseResult},
    types::Preamble,
};

impl AstParser {
    pub fn parse_assignment(&mut self) -> ParseResult {
        trace!("parse_assignment");

        let preamble = self.parse_assignment_preamble().ok();

        let assignment = self.parse_assignment_no_preamble()?;

        let expr = match preamble {
            None => assignment,
            Some(preamble) => Preamble::create(preamble, assignment),
        };

        trace!("expr: {:?}", expr);

        Ok(expr)
    }
}
