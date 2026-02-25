use tracing::trace;

use crate::ast::{
    parser::{AstParser, error::SyntaxError},
    types::{Expression, Preamble, PreambleType},
};

impl AstParser {
    pub fn parse_assignment_preamble(&mut self) -> Result<PreambleType, SyntaxError> {
        trace!("parse_assignment_preamble");
        Err(self.craft_error("todo"))
    }
}
