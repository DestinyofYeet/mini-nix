use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::{Assignment, Expression},
    },
    lexer::token::types::{LogicToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_assignment(&mut self) -> ParseResult {
        trace!("parse_assignment");

        let pre_idx = self.index;

        let token = match self.parse_identifier() {
            Ok(identifier) => match identifier {
                Expression::Literal(literal) => literal.literal,
                _ => unreachable!("parse_identifier always returns a literal!"),
            },
            Err(e) => {
                self.set_index(pre_idx);
                return Err(e);
            }
        };

        match self.is_match(&[TokenType::Logic(LogicToken::Equal)]) {
            Some(_) => {}
            None => {
                let err = self.craft_error("Expected '='", None).into();
                self.set_index(pre_idx);
                return Err(err);
            }
        }

        let expr = self.parse_expression_no_assignemnt()?;

        match self.is_match(&[TokenType::Misc(MiscToken::Semicolon)]) {
            Some(_) => {}
            None => {
                let err = self.craft_error("Expected ';'", None);
                self.set_index(pre_idx);
                return Err(err.into());
            }
        }

        Ok(Assignment::new_expr(token, expr))
    }
}
