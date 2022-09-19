use crate::{Args, FileOption, Plan};
use std::path::{Path, PathBuf};

fn parse_input_format(s: Option<&String>) -> Result<Option<FileOption>, ParseError> {
    if let Some(file_format) = s {
        let lower = file_format.to_lowercase();
        match lower.as_ref() {
            "parquet" => Ok(Some(FileOption::Parquet)),
            "csv" => Ok(Some(FileOption::Csv)),
            "json" => Ok(Some(FileOption::Json)),
            "avro" => Ok(Some(FileOption::Avro)),
            _ => Err(ParseError::ArgParseError(String::from(
                "invalid input format",
            ))),
        }
    } else {
        Ok(None)
    }
}

fn parse_output_format(fmt: Option<&String>) -> Result<FileOption, ParseError> {
    if let Some(file_fmt) = fmt {
        let lower = file_fmt.to_lowercase();
        match lower.as_ref() {
            "parquet" => Ok(FileOption::Parquet),
            "csv" => Ok(FileOption::Csv),
            "pretty" => Ok(FileOption::Pretty),
            "json" => Ok(FileOption::Json),
            "avro" => Ok(FileOption::Avro),
            _ => Err(ParseError::ArgParseError(String::from(
                "invalid output format",
            ))),
        }
    } else {
        Ok(FileOption::Pretty)
    }
}

fn parse_output_path<'a>(s: Option<&'a PathBuf>) -> Option<&'a Path> {
    match s {
        Some(pbuf) => Some(pbuf.as_path().as_ref()),
        None => None,
    }
}

pub fn parse_args(args: &Args) -> Result<Plan, ParseError> {
    let input_path = args.input_path.as_path();
    let input_format = parse_input_format(args.input_format.as_ref())?;
    let output_path = parse_output_path(args.output_path.as_ref());
    let output_format = parse_output_format(args.output_format.as_ref())?;

    Ok(Plan {
        input_path,
        input_format,
        transform: None,
        output_format,
        output_path,
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ArgParseError(String),
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
        let argg = Args {
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
