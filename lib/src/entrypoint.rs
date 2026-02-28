use thiserror::Error;
use tracing::{debug, trace};

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
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

impl From<SyntaxError> for Error {
    fn from(value: SyntaxError) -> Self {
        Self::Syntax(value.to_string())
    }
}

pub fn run(source: String) -> Result<(), Error> {
    let tokens = parse_text(source)?;
    trace!("{tokens:?}");

    let mut parser = AstParser::new(tokens);

    let expr = parser.parse()?;

    let result = AstPrinter::print(expr);
    debug!("ast: {result}");

    Ok(())
}
