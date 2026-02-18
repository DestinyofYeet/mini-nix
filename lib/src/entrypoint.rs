use thiserror::Error;
use tracing::trace;

use crate::{
    ast::ast_test,
    lexer::{error::ParserError, parse_text},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing: \n{0}")]
    Parse(String),
}

impl From<Vec<ParserError>> for Error {
    fn from(value: Vec<ParserError>) -> Self {
        let mut string = String::new();

        for error in value {
            string += &error.to_string()
        }

        Self::Parse(string)
    }
}

pub fn run(source: String) -> Result<(), Error> {
    ast_test();
    let tokens = parse_text(source)?;
    trace!("{tokens:?}");

    Ok(())
}
