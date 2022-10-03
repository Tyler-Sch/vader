use clap::Args;
use std::path::PathBuf;
use super::AddArgs;

#[derive(Args, Debug)]
pub struct GeneralArgs {
    #[arg(value_parser)]
    /// Path to file
    pub(crate) input_path:PathBuf,

    /// output path (leave empty for stdout)
    #[arg(value_parser)]
    pub(crate) output_path: Option<std::path::PathBuf>,

    /// output format [avro, parquet, csv, json, pretty]
    #[arg(long, short = 'o', value_parser)]
    pub(crate) output_format: Option<String>,

    /// in pretty: number of columns to display
    #[arg(long, value_parser)]
    pub(crate) num_cols: Option<String>,

    /// in pretty: number of rows to display
    #[arg(long, value_parser)]
    pub(crate) num_rows: Option<String>,

    /// in pretty: max string length
    #[arg(long, value_parser)]
    pub(crate) string_len: Option<String>,

    #[command(flatten)]
    pub(crate) add_args: AddArgs,
}