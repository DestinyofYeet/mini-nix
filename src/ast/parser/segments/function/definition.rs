use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::{Expression, Function},
    },
    lexer::token::types::{MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_function(&mut self) -> ParseResult {
        trace!("parse_function");
        let pre_idx = self.index;

        // it's okay to early exit, since an identifier is needed
        let identifier = {
            if let Expression::Literal(token) = self.parse_identifier()? {
                token.literal
            } else {
                unreachable!("parse_identifier should always return a literal!")
            }
        };

        match self.is_match(&[TokenType::Misc(MiscToken::Colon)]) {
            Some(_) => {}
            None => {
                let err = self.craft_error("Expected ':'", None).into();
                self.set_index(pre_idx);
                return Err(err);
            }
        }

        // todo!("missing 'preamble'");

        let expression = match self.parse_expression_no_assignemnt() {
            Ok(value) => value,
            Err(e) => {
                self.set_index(pre_idx);
                return Err(e);
            }
        };

        match self.is_match(&[TokenType::Misc(MiscToken::Semicolon)]) {
            Some(_) => {}
            None => {
                let err = self.craft_error("Expected ';'", None).into();
                self.set_index(pre_idx);
                return Err(err);
            }
        }

        Ok(Function::new_expr(identifier, None, expression))
    }
}
