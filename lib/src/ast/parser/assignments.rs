use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Binary, Expr, Expression},
    },
    lexer::token::types::{LogicToken, TokenType},
};

impl AstParser {
    pub fn parse_assignment(&mut self) -> Result<Expression, SyntaxError> {
        let expr = self.parse_identifier()?;

        let operator = match self.is_match(&[TokenType::Logic(LogicToken::Equal)]) {
            Some(token) => token,

            None => {
                let (mut line, mut column) = (0, 0);
                if let Some(current) = self.current() {
                    line = current.line;
                    column = current.column;
                }
                return Err(SyntaxError::SyntaxError {
                    line,
                    column,
                    msg: "= expected".to_string(),
                });
            }
        };

        let primary = self.parse_primary()?;

        Ok(Binary::create(expr, operator, primary))
    }
}
