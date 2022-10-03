use crate::cli::file_opts::FileOption;
use crate::cli::subcommands::FormatSubCommand;
use crate::cli::{AddArgs, Cli, Opts, Plan};
use anyhow::{anyhow, Result};

// fn parse_input_format(file_format: &String) -> Result<FileOption> {
//     let lower = file_format.to_lowercase();
//     match lower.as_ref() {
//         "parquet" => Ok(FileOption::Parquet),
//         "csv" => Ok(FileOption::Csv),
//         "json" => Ok(FileOption::Json),
//         "avro" => Ok(FileOption::Avro),
//         _ => Err(anyhow!("Input format: {} is unknown", lower)),
//     }
// }

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

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_input_format() {
        let p = String::from("Parquet");
        let valid_format = Some(&p);
        let result_valid = parse_input_format(valid_format);
        assert_eq!(result_valid.unwrap(), Some(FileOption::Parquet));
    }

    #[test]
    fn test_invalid_parse_input_format() {
        let np = String::from("not_a_format");
        let invalid_format = Some(&np);
        let result_invalid = parse_input_format(invalid_format);
        assert_eq!(
            result_invalid.unwrap_err(),
            ParseError::ArgParseError(String::from("invalid input format"))
        );
    }

    #[test]
    fn test_parse_fmt() {
        let s = String::from("PARQUet");
        let par = parse_output_format(Some(&s)).unwrap();
        assert_eq!(par, FileOption::Parquet);
    }

    #[test]
    fn test_parse_arguments() {
        let mut input_path = PathBuf::new();
        input_path.push("test");
        input_path.push("file.parquet");
        let in_copy = &input_path.clone();
        let argg = Cli {
            input_path,
            input_format: Some(String::from("csv")),
            output_path: Some(PathBuf::new()),
            output_format: Some(String::from("csv")),
            num_cols: None,
            num_rows: None,
            string_len: None,
        };
        let p = parse_args(&argg).expect("error parsing test");
        assert_eq!(p.input_path, *in_copy);
        assert_eq!(p.output_format, FileOption::Csv);
    }
}
