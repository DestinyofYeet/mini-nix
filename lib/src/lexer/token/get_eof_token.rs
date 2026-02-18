use crate::lexer::token::{
    Token,
    types::{MiscToken, TokenType},
};

impl Token {
    pub(crate) fn get_eof_token(line: usize) -> Self {
        Self::new(TokenType::Misc(MiscToken::Eof), String::new(), line)
    }
}
