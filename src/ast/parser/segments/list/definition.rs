use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::{Expression, List},
    },
    lexer::token::types::{MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_list(&mut self) -> ParseResult {
        trace!("parse_list");

        let mut expressions: Vec<Expression> = Vec::new();

        let pre_idx = self.index;

        match self.is_match(&[TokenType::Misc(MiscToken::LeftBracket)]) {
            Some(_) => {}
            None => return Err(self.craft_error("Expected '['", None).into()),
        }

        while let Ok(expr) = self.parse_expression_no_assignemnt() {
            expressions.push(expr);
        }

        match self.is_match(&[TokenType::Misc(MiscToken::RightBracket)]) {
            Some(_) => {}
            None => {
                self.set_index(pre_idx);
                return Err(self.craft_error("Expected ']'", None).into());
            }
        }

        Ok(List::new_expr(expressions))
    }
}
