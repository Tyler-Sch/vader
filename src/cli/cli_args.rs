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


    #[command(flatten)]
    pub(crate) add_args: AddArgs,
}