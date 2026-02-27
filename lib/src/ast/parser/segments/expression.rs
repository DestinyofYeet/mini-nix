use tracing::trace;

use crate::ast::{
    parser::{AstParser, ParseResult, error::SyntaxError},
    types::{Expression, Preamble},
};

impl AstParser {
    pub fn parse_expression(&mut self) -> ParseResult {
        trace!("parse_expression");
        let mut errors: Vec<SyntaxError> = Vec::new();

        let mut final_expr: Option<Expression> = None;

        let preamble = self.parse_preamble().ok();

        let index_post_preamble = self.index;

        match self.parse_expression_no_assignment() {
            Ok(value) => {
                trace!("expr: {value:?}");
                if self.is_done() {
                    final_expr = Some(value)
                } else {
                    errors.push(self.craft_error("Tokens left :("));
                }
            }
            Err(e) => {
                errors.push(e);
            }
        }

        if final_expr.is_none() {
            self.index = index_post_preamble;
            match self.parse_assignment() {
                Ok(value) => {
                    trace!("expr: {value:?}");

                    final_expr = Some(value)
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        match final_expr {
            Some(value) => match preamble {
                None => Ok(value),
                Some(preamble) => Ok(Preamble::create(preamble, value)),
            },
            None => Err(self.craft_error(format!(
                "Expected expression or assignment\n\t{}",
                errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ))),
        }
    }
}
