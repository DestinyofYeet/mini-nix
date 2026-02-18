use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unrecognized Token: '{}' at character {} on line {}", .token, .column, .line)]
    UnrecognizedToken {
        token: String,
        line: usize,
        column: usize,
    },

    #[error("Closing String expected on line {}", .line)]
    UnfinishedString { line: usize },

    #[error("Unable to convert '{}' to {}: {}", .value, .to_type, .err)]
    Conversion {
        value: String,
        to_type: String,
        err: String,
    },
}
