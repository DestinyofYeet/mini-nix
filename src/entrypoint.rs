use itertools::Itertools;
use thiserror::Error;
use tracing::{debug, trace};

use crate::{
    ast::{
        parser::{AstParser, error::AstError},
        printer::AstPrinter,
    },
    lexer::{error::ParserError, parse_text},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing: \n{0}")]
    Parse(String),

    #[error("Syntax error: \n{0}")]
    Syntax(String),
}

impl From<Vec<ParserError>> for Error {
    fn from(value: Vec<ParserError>) -> Self {
        let mut string = String::new();

        for error in value {
            string += &(error.to_string() + "\n");
        }

        Self::Parse(string)
    }
}

impl From<Vec<AstError>> for Error {
    fn from(value: Vec<AstError>) -> Self {
        Self::Syntax(value.into_iter().join("\n"))
        // Self::Syntax(value.into_iter().last().unwrap().to_string())
    }
}

pub fn run(source: &str) -> Result<(), Error> {
    let tokens = parse_text(source)?;
    trace!("{tokens:?}");

    let mut parser = AstParser::new(tokens);

    let expr = parser.parse()?;

    let result = AstPrinter::print(expr);
    debug!("ast: {result}");

    Ok(())
}
