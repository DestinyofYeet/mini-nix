use tracing::trace;

use crate::ast::parser::{AstParser, ParseResult, error::SyntaxError};

impl AstParser {
    pub fn parse_expression_no_assignment(&mut self) -> ParseResult {
        trace!("parse_expression_no_assignment");
        let mut errors: Vec<SyntaxError> = Vec::new();

        //parse attrset
        // functions

        match self.parse_arithmetic() {
            Ok(value) => {
                trace!("expr: {value:?}");
                return Ok(value);
            }
            Err(e) => errors.push(e),
        };

        Err(self.craft_error(format!(
            "Expected an attrset, function or arithmetic\n\t{}",
            errors
                .iter_mut()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )))
    }
}
