#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(clippy::implicit_return)]

use anyhow::Result;
use clap::Parser;
use vader::cli::Cli;
use vader::io;
use vader::cli::parse_args;
use vader::set_env;
// use vader::Cli;
use std::process::exit;

// TODO: add command to print schema
// TODO: env var for default in/out formats
// TODO: integrate with aws
// TODO: make format specific options only come up with specific formats (ie: header for csv)
// TODO: avro to read who directory

fn main() {
    let args = Cli::parse();
    set_env(&args);
    let result = run(args);

    match result {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }
}

fn run(args: Cli) -> Result<String> {
    let plan = parse_args(args)?;
    let df = crate::io::read(&plan)?;
    crate::io::write(plan, df)?;
    Ok(String::from("success"))
}
