use super::file_opts::FileOption;
use super::subcommands::FormatSubCommand;
use super::{AddArgs, Cli, Opts, Plan};
use anyhow::{anyhow, Result};

fn parse_output_format(fmt: Option<&String>) -> Result<FileOption> {
    if let Some(file_fmt) = fmt {
        let lower = file_fmt.to_lowercase();
        match lower.as_ref() {
            "parquet" => Ok(FileOption::Parquet),
            "csv" => Ok(FileOption::Csv),
            "pretty" => Ok(FileOption::Pretty),
            "json" => Ok(FileOption::Json),
            "avro" => Ok(FileOption::Avro),
            _ => Err(anyhow!("Output format: {} is unknown", lower)),
        }
    } else {
        Ok(FileOption::Pretty)
    }
}

fn parse_additional_args(more_args: AddArgs) -> Vec<Opts> {
    let mut v = vec![];
    if more_args.infile_header {
        v.push(Opts::InfileHeader)
    }
    if more_args.outfile_header {
        v.push(Opts::OutFileHeader)
    }
    v
}

pub fn parse_args(args: Cli) -> Result<Plan> {
    let (gen_args, input_format) = match args.commands {
        FormatSubCommand::csv(c) => (c.gen_args, FileOption::Csv),
        FormatSubCommand::avro(a) => (a.gen_args, FileOption::Avro),
        FormatSubCommand::parquet(p) => (p.gen_args, FileOption::Parquet),
        FormatSubCommand::json(j) => (j.gen_args, FileOption::Json),
    };

    let input_path = gen_args.input_path;
    let output_path = gen_args.output_path;
    let output_format = parse_output_format(gen_args.output_format.as_ref())?;
    let additional_args = parse_additional_args(gen_args.add_args);

    Ok(Plan {
        input_path,
        input_format,
        transform: None,
        output_format,
        output_path,
        additional_args,
    })
}

#[cfg(test)]
mod test_parse_args {

}
