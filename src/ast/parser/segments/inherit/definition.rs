use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::{Expression, inherit::Inherit},
    },
    lexer::token::{
        Token,
        types::{KeywordToken, MiscToken, TokenType},
    },
};

impl AstParser {
    pub fn parse_inherit(&mut self) -> ParseResult {
        trace!("parse_inherit");

        let pre_idx = self.index;

        match self.is_match(&[TokenType::Keyword(KeywordToken::Inherit)]) {
            Some(_) => {}
            None => {
                return Err(self.craft_error("Expected 'inherit'", None).into());
            }
        }

        let maybe_pre_idx = self.index;
        let maybe_from = (|| {
            self.is_match(&[TokenType::Misc(MiscToken::LeftParen)])?;

            let expr = {
                if let Ok(identifier) = self.parse_identifier() {
                    Some(identifier)
                } else {
                    self.parse_attrset().ok()
                }
            };

            self.is_match(&[TokenType::Misc(MiscToken::RightParen)])?;

            expr
        })();

        if maybe_from.is_none() {
            self.set_index(maybe_pre_idx);
        }

        let mut tokens: Vec<Token> = Vec::new();

        while let Ok(value) = self.parse_identifier() {
            match value {
                Expression::Literal(literal) => tokens.push(literal.literal),

                _ => unreachable!("parse_identifier always returns a literal!"),
            }
        }

        match self.is_match(&[TokenType::Misc(MiscToken::Semicolon)]) {
            Some(_) => {}
            None => {
                let err = self.craft_error("Expected ';'", None);
                self.set_index(pre_idx);
                return Err(err.into());
            }
        }

        Ok(Inherit::new_expr(maybe_from, tokens))
    }
}
