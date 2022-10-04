use std::path::PathBuf;

use super::cli_args::GeneralArgs;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub(crate) enum FormatSubCommand {
    csv(Csv),
    parquet(Parquet),
    avro(Avro),
    json(Json),
    schema(SchemaArgs)
}

#[derive(Args, Debug)]
pub(crate) struct Csv {
    #[command(flatten)]
    pub gen_args: GeneralArgs,
}
#[derive(Args, Debug)]
pub(crate) struct Parquet {
    #[command(flatten)]
    pub gen_args: GeneralArgs,
}

#[derive(Args, Debug)]
pub(crate) struct Avro {
    #[command(flatten)]
    pub gen_args: GeneralArgs,
}

#[derive(Args, Debug)]
pub(crate) struct Json {
    #[command(flatten)]
    pub gen_args: GeneralArgs,
}

#[derive(Args, Debug)]
pub(crate) struct SchemaArgs {
    /// input format [parquet, csv, avro, json]
    #[arg(value_parser)]
    pub input_format: String,

    /// input file path
    #[arg(value_parser)]
    pub input_path: PathBuf
}