use super::cli_args::GeneralArgs;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub(crate) enum FormatSubCommand {
    csv(Csv),
    parquet(Parquet),
    avro(Avro),
    json(Json),
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
