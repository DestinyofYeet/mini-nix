use thiserror::Error;

fn format_err(errors: &[AstError], depth: u64) -> String {
    let mut out = String::new();

    if errors.is_empty() {
        return out;
    }

    for error in errors {
        for _ in 0..depth + 1 {
            out += "-"
        }

        out += "> Caused by: ";

        out += &match error {
            AstError::SyntaxError {
                line,
                column,
                msg,
                error,
            } => {
                let mut string = format!("Syntax error at {}:{} {}", line, column, msg);
                string += "\n";

                string += &format_err(error, depth + 1);

                string
            }

            AstError::OutOfTokens => error.to_string(),
        };
    }

    out
}

#[derive(Error, Debug)]
pub enum AstError {
    #[error("Syntax error at {}:{} {}\n{}", .line, .column, .msg, format_err(.error, 0))]
    SyntaxError {
        line: usize,
        column: usize,
        msg: String,
        error: Vec<AstError>,
    },

    #[error("Out of tokens")]
    OutOfTokens,
}
