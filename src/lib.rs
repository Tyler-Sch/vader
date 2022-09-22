use clap::Parser;
use std::path::Path;
mod file_utils;
pub mod io;
pub mod parse_args;

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(name = "vader")]
#[clap(about = "CLI For Querying Files")]
pub struct Args {
    /// Path to file
    #[clap(value_parser)]
    input_path: std::path::PathBuf,

    /// format of input file
    #[clap(short = 'i', value_parser)]
    input_format: Option<String>,

    /// output path (leave emptry for stdout)
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
}

pub struct Plan<'a> {
    pub input_path: &'a Path,
    pub input_format: Option<FileOption>,
    transform: Option<Vec<String>>, // to be implemeted at a future date
    pub output_format: FileOption,
    pub output_path: Option<&'a Path>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum FileOption {
    Csv,
    Parquet,
    Pretty,
    Json,
    Avro,
}

pub fn set_env(ars: &Args) {
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
