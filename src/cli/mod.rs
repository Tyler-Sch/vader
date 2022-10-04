
pub(crate) mod file_opts;
pub(self) mod subcommands;
pub(self) mod cli_args;
mod parse_args;
pub use self::parse_args::parse_args;

use clap::{Parser, Args};
use std::path::PathBuf;
use file_opts::FileOption;


#[derive(Parser, Debug)]
#[command(author, version)]
#[command(name = "vader")]
#[command(about = "CLI For Querying Files")]
pub struct Cli {

    /// in pretty: number of columns to display
    #[arg(long, value_parser)]
    pub(crate) num_cols: Option<String>,

    /// in pretty: number of rows to display
    #[arg(long, value_parser)]
    pub(crate) num_rows: Option<String>,

    /// in pretty: max string length
    #[arg(long, value_parser)]
    pub(crate) string_len: Option<String>,
    
    /// Options for format or action
    #[command(subcommand)]
    pub(self) commands: subcommands::FormatSubCommand
}

#[derive(Debug, Args)]
pub(crate) struct AddArgs {
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
    pub transform: Option<Vec<String>>, 
    pub output_format: FileOption,
    pub output_path: Option<PathBuf>,
    pub additional_args: Vec<Opts>,
}

#[derive(PartialEq, Eq)]
pub enum Opts {
    InfileHeader,
    OutFileHeader,
}
