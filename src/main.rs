#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(clippy::implicit_return)]

use anyhow::Result;
use clap::Parser;
use vader::io;
use vader::parse_args;
use vader::{set_env, Args};
use std::process::exit;
// TODO: stdout format can be pretty table representation
// TODO: format can be easily used by awk/cut/sed (maybe csv output satisfies that)
// TODO: provide value for to print all the rows and all the columns instead of passing nums
// TODO: read files from dir
// TODO: figure out format based on extension
// TODO: have override for that extension
// TODO: fix error propagation in parse_args
// TODO: add command to print schema
// TODO: env var for default in/out formats

fn main() {
    let args = Args::parse();
    set_env(&args);
    let result = run(&args);

    match result {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }
}

fn run(args: &Args) -> Result<String> {
    let plan = parse_args::parse_args(args)?;
    let df = crate::io::read(&plan)?;
    crate::io::write(plan, df)?;
    Ok(String::from("success"))
}
