use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParserError {
    #[error("Unrecognized Token: '{}' at character {} on line {}", .token, .column, .line)]
    UnrecognizedToken {
        token: String,
        line: usize,
        column: usize,
    },

    #[error("Closing String expected on line {} at char {}", .line, .column)]
    UnfinishedString { line: usize, column: usize },

    #[error("Unable to convert '{}' to {}: {}", .value, .to_type, .err)]
    Conversion {
        value: String,
        to_type: String,
        err: String,
    },

    #[error("(internal) Index resulted in None. start: {} | end: {}", .start, .end)]
    WrongIndex { start: usize, end: usize },
}
