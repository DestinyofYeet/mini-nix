use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Syntax error at {}:{}. {}", .line, .column, .msg)]
    SyntaxError {
        line: usize,
        column: usize,
        msg: String,
    },

    #[error("Out of tokens")]
    OutOfTokens,
}
