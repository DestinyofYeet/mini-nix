use itertools::Itertools;
use tracing::trace;

use crate::{
    ast::{parser::error::AstError, types::Expression},
    lexer::token::{Token, types::TokenType},
};

/// This will take tokens and generate an AST
pub struct AstParser {
    pub tokens: Vec<Token>,
    pub index: u64,
}

pub type ParseResult = Result<Expression, Vec<AstError>>;

impl From<AstError> for Vec<AstError> {
    fn from(val: AstError) -> Self {
        vec![val]
    }
}

#[allow(dead_code)]
impl AstParser {
    pub fn new(tokenstream: Vec<Token>) -> Self {
        Self {
            tokens: tokenstream,
            index: 0,
        }
    }

    #[inline(always)]
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index_usize() + 1)
    }

    #[inline(always)]
    pub fn index_usize(&self) -> usize {
        self.index as usize
    }

    #[inline(always)]
    pub fn advance(&mut self) {
        self.advance_by(1);
    }

    #[inline(always)]
    pub fn advance_by(&mut self, by: u64) {
        trace!("advance_by: {} -> {}", self.index, self.index + by);
        self.index += by;
    }

    #[inline(always)]
    pub fn revert(&mut self) {
        self.revert_by(1);
    }

    #[inline(always)]
    pub fn revert_by(&mut self, by: u64) {
        trace!("revert_by: {} -> {}", self.index, self.index - by);

        self.index -= by;
    }

    pub fn is_done(&self) -> bool {
        self.index_usize() >= self.tokens.len()
    }

    /// Returns the current value and advances
    pub fn next(&mut self) -> Option<&Token> {
        let current = self.tokens.get(self.index_usize());
        trace!("next: idx: {} | {current:?}", self.index);
        self.index += 1;

        current
    }

    fn current(&self) -> Option<&Token> {
        trace!("current: {}", self.index);
        self.tokens.get(self.index_usize())
    }

    pub fn is_match(&mut self, tokens: &[TokenType]) -> Option<Token> {
        let mut returned = None;

        if let Some(current) = self.current() {
            for token in tokens {
                if *token == current.r#type {
                    returned = Some(current.clone());
                    break;
                }
            }
        }

        if returned.is_some() {
            self.advance();
        }

        returned
    }

    #[inline(always)]
    pub fn set_index(&mut self, idx: u64) {
        trace!("set_index: {} -> {}", self.index, idx);
        self.index = idx
    }

    pub fn craft_error(
        &self,
        message: impl Into<String>,
        errors: impl Into<Option<Vec<AstError>>>,
    ) -> AstError {
        let mut line = 0;
        let mut column = 0;

        if let Some(current) = self.current() {
            line = current.line;
            column = current.column;
        }

        let errors = errors.into();

        AstError::SyntaxError {
            line,
            column,
            msg: message.into(),
            error: errors.unwrap_or(Vec::new()),
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        let expr = self.parse_expression()?;
        trace!("expr: {:?}", expr);
        Ok(expr)
    }
}
