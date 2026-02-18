use std::{
    io::{Read, Write},
    process::exit,
};

use anyhow::Error;
use clap::Parser;

use mini_nix_lib::parser::parse_text;
use tracing::{debug, trace};
use tracing_subscriber::EnvFilter;

use crate::args::Args;

mod args;

pub fn main() -> Result<(), Error> {
    let args = Args::parse();

    let level = match args.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    }
    .to_string();

    // level.push_str(",reqwest=info,hyper_util=info,qbit_rs=info");

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    debug!("Debug logging enabled");
    trace!("Trace logging enabled");

    if let Some(input_file) = args.input_file {
        if !input_file.exists() {
            eprintln!("File path '{input_file:?}' does not exist!");
            exit(1);
        }

        let input = std::fs::read_to_string(input_file)?;

        match parse_text(input) {
            Ok(_) => {}
            Err(errors) => {
                for error in errors {
                    eprintln!("{error}");
                }
                exit(1);
            }
        }
    } else {
        loop {
            print!("> ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match parse_text(input) {
                Ok(_) => {}
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error}");
                    }
                }
            }
        }
    }

    Ok(())
}
