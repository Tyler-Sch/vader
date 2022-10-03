
pub mod file_opts;
pub mod subcommands;
pub mod cli_args;

use clap::{Parser, Args};
use std::path::PathBuf;
use file_opts::FileOption;


#[derive(Parser, Debug)]
#[command(author, version)]
#[command(name = "vader")]
#[command(about = "CLI For Querying Files")]
pub struct Cli {
    // /// Path to file
    // #[arg(value_parser)]
    // pub(crate) input_path: std::path::PathBuf,

    // /// format of input file
    // #[arg(short = 'i', value_parser)]
    // pub(crate) input_format: String,

    // /// output path (leave empty for stdout)
    // #[arg(value_parser)]
    // pub(crate) output_path: Option<std::path::PathBuf>,

    // /// output format [avro, parquet, csv, json, pretty]
    // #[arg(long, short = 'o', value_parser)]
    // pub(crate) output_format: Option<String>,

    /// in pretty: number of columns to display
    #[arg(long, value_parser)]
    pub(crate) num_cols: Option<String>,

    /// in pretty: number of rows to display
    #[arg(long, value_parser)]
    pub(crate) num_rows: Option<String>,

    /// in pretty: max string length
    #[arg(long, value_parser)]
    pub(crate) string_len: Option<String>,

    // #[command(flatten)]
    // pub(crate) add_args: AddArgs,
    
    #[command(subcommand)]
    pub(crate) commands: subcommands::FormatSubCommand
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// with header input (csv only)
    #[arg(short = 'm', value_parser)]
    pub infile_header: bool,

    /// with header output (csv only)
    #[arg(short = 'n', value_parser)]
    pub outfile_header: bool,
}
pub struct Plan {
    pub input_path: PathBuf,
    pub input_format: FileOption,
    pub transform: Option<Vec<String>>, // to be implemeted at a future date
    pub output_format: FileOption,
    pub output_path: Option<PathBuf>,
    pub additional_args: Vec<Opts>,
}

#[derive(PartialEq, Eq)]
pub enum Opts {
    InfileHeader,
    OutFileHeader,
}
