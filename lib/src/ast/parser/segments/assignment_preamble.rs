use tracing::trace;

use crate::ast::{
    parser::{AstParser, error::SyntaxError},
    types::{Expression, Preamble, PreambleType},
};

impl AstParser {
    pub fn parse_assignment_preamble(&mut self) -> Result<PreambleType, Vec<SyntaxError>> {
        trace!("parse_assignment_preamble");
        Err(vec![self.craft_error("todo")])
    }
}
