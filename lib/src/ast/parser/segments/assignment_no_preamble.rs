use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::Binary,
    },
    lexer::token::types::{LogicToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_assignment_no_preamble(&mut self) -> ParseResult {
        trace!("parse_assignment_no_preamble");

        let left = self.parse_identifier()?;

        let middle = match self.is_match(&[TokenType::Logic(LogicToken::Equal)]) {
            Some(token) => token,

            None => {
                return Err(self.craft_error("= expected"));
            }
        };

        let right = self.parse_expression_no_assignment()?;

        let final_expr = Binary::create(left, middle, right);

        if self
            .is_match(&[TokenType::Misc(MiscToken::Semicolon)])
            .is_none()
        {
            return Err(self.craft_error("Expected ';'"));
        }

        trace!("expr: {:?}", final_expr);

        Ok(final_expr)
    }
}
