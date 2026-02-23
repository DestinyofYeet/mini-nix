use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Binary, Expression},
    },
    lexer::token::types::{LogicToken, TokenType},
};

impl AstParser {
    pub fn parse_assignment_no_preamble(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        trace!("parse_assignment_no_preamble");
        let mut errors = Vec::<SyntaxError>::new();

        let left = match self.parse_identifier() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);

                None
            }
        };

        let middle = match self.is_match(&[TokenType::Logic(LogicToken::Equal)]) {
            Some(token) => Some(token),

            None => {
                let (mut line, mut column) = (0, 0);
                if let Some(current) = self.current() {
                    line = current.line;
                    column = current.column;
                }
                errors.push(SyntaxError::SyntaxError {
                    line,
                    column,
                    msg: "= expected".to_string(),
                });

                None
            }
        };

        let right = match self.parse_expression_no_assignment() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);

                None
            }
        };

        if left.is_none() | middle.is_none() | right.is_none() {
            return Err(errors);
        }

        let final_expr = Binary::create(left.unwrap(), middle.unwrap(), right.unwrap());

        trace!("expr: {:?}", final_expr);

        Ok(final_expr)
    }
}
