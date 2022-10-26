use crate::cli::file_opts::FileOption;
use crate::cli::{Opts, Plan};
use anyhow::Result;
use aws_s3::s3::get_s3_data;
use aws_s3::AggregatedBytes;
use polars::{
    io::avro::{AvroReader, AvroWriter},
    prelude::*,
};
use std::io::{Cursor, Write};
use std::path::Path;

use crate::file_utils;
// TODO: break out into submodules and add more options for read/write

pub fn read(plan: &Plan) -> Result<LazyFrame> {
    let path = plan.input_path.as_path();

    let df = match plan.input_format {
        FileOption::Avro => read_avro(path),
        FileOption::Parquet => read_parquet(path),
        FileOption::Csv => read_csv(path, &plan.additional_args.as_ref()),
        FileOption::Pretty => unimplemented!(),
        FileOption::Json => read_json(path),
    };
    Ok(df?)
}

fn read_avro(path: &Path) -> Result<LazyFrame, PolarsError> {
    let f = file_utils::open_file(path)?;
    let df = AvroReader::new(f).finish()?;
    Ok(df.lazy())
}

fn read_parquet(path: &Path) -> Result<LazyFrame, PolarsError> {
    if path.starts_with("s3://") {
        let df = read_parquet_s3(&path)?;
        Ok(df.lazy())
    } else {
        read_parquet_local(path)
    }
}

fn read_parquet_local(path: &Path) -> Result<LazyFrame, PolarsError> {
    LazyFrame::scan_parquet(path, Default::default())
}

fn read_parquet_s3(path: &Path) -> Result<DataFrame, PolarsError> {
    let mut prefix: Vec<&str> = path
        .into_iter()
        .map(|x| x.to_str().unwrap())
        .filter(|x| !(x == &"s3:"))
        .collect();

    let (bucket, file_name, prefix) = {
        // begin the yuck phase
        let bucke = prefix[0];
        let file_nam = prefix.pop().unwrap();
        let prefi = prefix[1..].join("/");
        (bucke, file_nam, prefi)
    };
    let data = get_s3_data(bucket, &prefix, file_name).unwrap();
    let p = data
        .iter()
        .map(move |data| {
            let bytes = data.to_owned().into_bytes();
            let cursor = Cursor::new(bytes);
            let reader = ParquetReader::new(cursor);
            reader.finish().unwrap()
        })
        .reduce(|df, df2| df.vstack(&df2).unwrap())
        .unwrap();
    Ok(p)
}

fn read_csv(path: &Path, add_args: &Vec<Opts>) -> Result<LazyFrame, PolarsError> {
    let reader = LazyCsvReader::new(path);
    let with_header = add_args.contains(&Opts::InfileHeader);
    let reader = reader.has_header(with_header);
    reader.finish()
}

fn read_json(path: &Path) -> Result<LazyFrame, PolarsError> {
    let string_path = path
        .to_str()
        .expect("could not read path to json file")
        .to_string();
    LazyJsonLineReader::new(string_path).finish()
}

pub fn write(plan: Plan, df: LazyFrame) -> Result<()> {
    let data = df.collect()?;
    let out_path = get_output_path(&plan);
    let _ = match plan.output_format {
        FileOption::Avro => write_avro(data, out_path),
        FileOption::Parquet => write_parquet(data, out_path),
        FileOption::Csv => write_csv(data, out_path, &plan.additional_args),
        FileOption::Json => write_json(data, out_path),
        FileOption::Pretty => write_pretty(data),
    }?;
    Ok(())
}

pub fn write_schema(df: LazyFrame) -> Result<(), PolarsError> {
    let data = df.collect()?;
    println!("{:#?}", data.schema());
    Ok(())
}

fn get_output_path(plan: &Plan) -> Box<dyn Write> {
    match &plan.output_path {
        Some(p) => Box::new(crate::file_utils::create_file(p)),
        None => Box::new(std::io::stdout()),
    }
}

fn write_avro(mut df: DataFrame, w: Box<dyn Write>) -> Result<(), PolarsError> {
    AvroWriter::new(w).finish(&mut df)
}

fn write_json(mut df: DataFrame, w: Box<dyn Write>) -> Result<(), PolarsError> {
    JsonWriter::new(w).finish(&mut df)
}

fn write_parquet(mut df: DataFrame, w: Box<dyn Write>) -> Result<(), PolarsError> {
    ParquetWriter::new(w).finish(&mut df)
}

fn write_csv(
    mut df: DataFrame,
    w: Box<dyn Write>,
    add_opts: &Vec<Opts>,
) -> Result<(), PolarsError> {
    let writer = CsvWriter::new(w);
    let has_header = add_opts.contains(&Opts::OutFileHeader);
    let mut writer = writer.has_header(has_header);
    writer.finish(&mut df)
}

fn write_pretty(mut df: DataFrame) -> Result<(), PolarsError> {
    println!("{}", df);
    Ok(())
}

#[cfg(test)]
mod test_io {
    use super::*;
    use crate::file_utils::create_file;
    use std::env;
    use std::path::PathBuf;

    static TEST_DIR: &str = "vader_base_dir";

    #[test]
    fn test_write() -> Result<()> {
        let p = create_dir_in_tmp(vec!["test.parquet"]);
        let f = Box::new(create_file(&p));
        let s1 = Series::new("c1", &[1, 2, 3, 4, 5]);
        let s2 = Series::new("c2", &["foo", "bar", "baz", "hades", "zeus"]);
        let df = DataFrame::new(vec![s1, s2])?;
        let test_df = df.clone();
        write_parquet(test_df, f);

        let data = read_parquet(&p)?.collect()?;
        assert_eq!(df, data);

        delete_test_dir();
        Ok(())
    }

    fn create_dir_in_tmp(path_vec: Vec<&str>) -> PathBuf {
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push(TEST_DIR);
        path_vec.iter().for_each(|entry| tmp_dir.push(entry));
        std::fs::create_dir_all(tmp_dir.parent().unwrap());
        tmp_dir
    }

    fn delete_test_dir() -> Result<()> {
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push(TEST_DIR);
        std::fs::remove_dir_all(tmp_dir);
        Ok(())
    }

    use aws_s3::s3::get_s3_data;
    use polars::io::csv::CsvReader;
    use std::io::Cursor;

    #[test]
    fn test_read_parquet() {
        let data = get_s3_data("some""stuff", "parquet")// get your fake data here
        let df = read_parquet_s3(data);
        let ddf = df.unwrap();
        println!("{:?}", ddf);
    }
    #[test]
    fn test_read_parquet_fn() {

    }
}
