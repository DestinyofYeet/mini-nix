use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Binary, Expression},
    },
    lexer::token::types::{LogicToken, TokenType},
};

impl AstParser {
    pub fn parse_assignment(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        trace!("parse_assignment");
        let mut errors = Vec::<SyntaxError>::new();
        let expr = match self.parse_identifier() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);

                None
            }
        };

        let operator = match self.is_match(&[TokenType::Logic(LogicToken::Equal)]) {
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

        let primary = match self.parse_primary() {
            Ok(value) => Some(value),
            Err(mut e) => {
                errors.append(&mut e);

                None
            }
        };

        if expr.is_none() | operator.is_none() | primary.is_none() {
            return Err(errors);
        }

        let final_expr = Binary::create(expr.unwrap(), operator.unwrap(), primary.unwrap());

        trace!("expr: {:?}", final_expr);

        Ok(final_expr)
    }
}
