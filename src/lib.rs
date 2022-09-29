use clap::{Parser, Args};
use std::path::PathBuf;
mod file_utils;
pub mod io;
pub mod parse_args;

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(name = "vader")]
#[clap(about = "CLI For Querying Files")]
pub struct Cli {
    /// Path to file
    #[clap(value_parser)]
    input_path: std::path::PathBuf,

    /// format of input file
    #[clap(short = 'i', value_parser)]
    input_format: String,

    /// output path (leave empty for stdout)
    #[clap(value_parser)]
    output_path: Option<std::path::PathBuf>,

    /// output format [avro, parquet, csv, json, pretty]
    #[clap(long, short = 'o', value_parser)]
    output_format: Option<String>,

    /// in pretty: number of columns to display
    #[clap(long, value_parser)]
    num_cols: Option<String>,

    /// in pretty: number of rows to display
    #[clap(long, value_parser)]
    num_rows: Option<String>,

    /// in pretty: max string length
    #[clap(long, value_parser)]
    string_len: Option<String>,

    #[clap(flatten)]
    add_args: AddArgs,
}

#[derive(Debug, Args)]
struct AddArgs {
    /// with header input (csv only)
    #[clap(short = 'm', value_parser)]
    infile_header: bool,

    /// with header output (csv only)
    #[clap(short = 'n', value_parser)]
    outfile_header: bool,
}
pub struct Plan {
    pub input_path: PathBuf,
    pub input_format: FileOption,
    transform: Option<Vec<String>>, // to be implemeted at a future date
    pub output_format: FileOption,
    pub output_path: Option<PathBuf>,
    pub additional_args: Vec<Opts>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum FileOption {
    Csv,
    Parquet,
    Pretty,
    Json,
    Avro,
}
#[derive(PartialEq, Eq)]
pub enum Opts {
    InfileHeader,
    OutFileHeader,
}

pub fn set_env(ars: &Cli) {
    if let Some(ncols) = &ars.num_cols {
        std::env::set_var("POLARS_FMT_MAX_COLS", ncols);
    }
    if let Some(nrows) = &ars.num_rows {
        std::env::set_var("POLARS_FMT_MAX_ROWS", nrows);
    }
    if let Some(strlen) = &ars.string_len {
        std::env::set_var("POLARS_FMT_STR_LEN", strlen);
    }
}
