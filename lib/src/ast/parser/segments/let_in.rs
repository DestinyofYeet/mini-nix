use tracing::trace;

use crate::ast::parser::AstParser;
use crate::ast::parser::error::SyntaxError;
use crate::ast::types::{Expression, PreambleLetIn, PreambleType};
use crate::lexer::token::types::{KeywordToken, TokenType};

impl AstParser {
    pub fn parse_let_in(&mut self) -> Result<PreambleType, SyntaxError> {
        trace!("parse_let_in");

        if self
            .is_match(&[TokenType::Keyword(KeywordToken::Let)])
            .is_none()
        {
            return Err(self.craft_error("Expected 'let' for let-in"));
        }

        let mut assignments: Vec<Expression> = Vec::new();

        while let Ok(value) = self.parse_assignment_no_preamble() {
            assignments.push(value);
        }

        if self
            .is_match(&[TokenType::Keyword(KeywordToken::In)])
            .is_none()
        {
            return Err(self.craft_error("Expected 'in' for let-in"));
        }

        Ok(PreambleType::LetIn(PreambleLetIn { expr: assignments }))
    }
}
