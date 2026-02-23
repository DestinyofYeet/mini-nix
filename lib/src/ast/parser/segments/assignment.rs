use tracing::trace;

use crate::ast::{
    parser::{AstParser, ParseResult, error::SyntaxError},
    types::{Binary, Expression, Preamble},
};

impl AstParser {
    pub fn parse_assignment(&mut self) -> ParseResult {
        trace!("parse_assignment");
        let mut errors = Vec::<SyntaxError>::new();

        let preamble = match self.parse_assignment_preamble() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);
                None
            }
        };

        let assignment = match self.parse_assignment_no_preamble() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);
                None
            }
        };

        if assignment.is_none() {
            errors.push(self.craft_error("Expected assignment"));
            return Err(errors);
        }

        let expr = match preamble {
            None => assignment.unwrap(),
            Some(preamble) => Preamble::create(preamble, assignment.unwrap()),
        };

        trace!("expr: {:?}", expr);

        Ok(expr)
    }
}
