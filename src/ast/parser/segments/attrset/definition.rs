use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::{Attrset, Expression},
    },
    lexer::token::types::{MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_attrset(&mut self) -> ParseResult {
        trace!("parse_attrset");

        let pre_idx = self.index;

        match self.is_match(&[TokenType::Misc(MiscToken::LeftCurlyParen)]) {
            Some(_) => {}
            None => return Err(self.craft_error("Expected '{", None).into()),
        };

        let mut values: Vec<Expression> = Vec::new();

        loop {
            let assignment = match self.parse_assignment() {
                Ok(value) => {
                    values.push(value);
                    Some(())
                }
                Err(_) => None,
            };

            let inherit = match self.parse_inherit() {
                Ok(value) => {
                    values.push(value);
                    Some(())
                }
                Err(_) => None,
            };

            if assignment.is_none() && inherit.is_none() {
                break;
            }
        }

        match self.is_match(&[TokenType::Misc(MiscToken::RightCurlyParen)]) {
            Some(_) => {}
            None => {
                self.set_index(pre_idx);
                return Err(self.craft_error("Expected '}", None).into());
            }
        };

        Ok(Attrset::new_expr(values))
    }
}
