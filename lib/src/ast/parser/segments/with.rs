use itertools::Itertools;
use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{PreambleType, PreambleWith},
    },
    lexer::token::types::{KeywordToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_with(&mut self) -> Result<PreambleType, SyntaxError> {
        trace!("parse_with");

        if self
            .is_match(&[TokenType::Keyword(KeywordToken::With)])
            .is_none()
        {
            return Err(self.craft_error("Expected with"));
        }

        let mut errors = Vec::new();

        let identifier = match self.parse_identifier() {
            Ok(value) => Some(value),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        // let attrset = match self.parse_attrset() {

        // };

        let err_fmt = errors.into_iter().map(|e| e.to_string()).join("\n");

        if identifier.is_none() {
            // && attrset.is_none()
            // ) || (attrset.is_some() && identifier.is_some())
            return Err(self.craft_error(format!(
                "Expected either identifier or attrset: \n\t{}",
                err_fmt
            )));
        }

        if self
            .is_match(&[TokenType::Misc(MiscToken::Semicolon)])
            .is_none()
        {
            return Err(self.craft_error(format!("Expected ';': \n\t{}", err_fmt)));
        }

        if let Some(value) = identifier {
            return Ok(PreambleType::With(PreambleWith {
                expr: Box::new(value),
            }));
        }

        Err(self.craft_error("todo"))
    }
}
