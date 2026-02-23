use tracing::trace;

use crate::ast::{
    parser::{AstParser, error::SyntaxError},
    types::Expression,
};

impl AstParser {
    pub fn parse_expression_no_assignment(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        trace!("parse_expression_no_assignment");
        let mut errors = Vec::<SyntaxError>::new();

        //parse attrset
        // functions

        let arithmetic = match self.parse_primary() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);
                None
            }
        };

        if arithmetic.is_none() {
            errors.push(self.craft_error("Expected arithmetic"));
            return Err(errors);
        }

        Ok(arithmetic.unwrap())
    }
}
