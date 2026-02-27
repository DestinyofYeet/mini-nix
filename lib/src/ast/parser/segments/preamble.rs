use itertools::Itertools;
use tracing::trace;

use crate::ast::{
    parser::{AstParser, error::SyntaxError},
    types::{Expression, PreambleType},
};

impl AstParser {
    // This should be able to parse multiple preambles, but it can't currently
    // because the PreambleType can't hold that information
    pub fn parse_preamble(&mut self) -> Result<PreambleType, SyntaxError> {
        trace!("parse_assignment_preamble");

        let mut errors: Vec<SyntaxError> = Vec::new();

        let let_in = match self.parse_let_in() {
            Ok(value) => Some(value),

            Err(e) => {
                errors.push(e);
                None
            }
        };

        let with = match self.parse_with() {
            Ok(value) => Some(value),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        if let Some(value) = let_in {
            trace!("expr: {value:?}");
            return Ok(value);
        }

        if let Some(value) = with {
            trace!("expr: {value:?}");
            return Ok(value);
        }

        Err(self.craft_error(format!(
            "Expected let-in or with binding: \n\t{}",
            errors.into_iter().map(|e| e.to_string()).join("\n")
        )))
    }
}
