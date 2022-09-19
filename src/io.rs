use polars::{
    io::avro::{AvroReader, AvroWriter},
    prelude::*,
};
use std::io::Write;
use std::path::Path;

use crate::{file_utils, FileOption, Plan};
// TODO: break out into submodules and add more options for read/write

pub fn read(plan: &Plan) -> LazyFrame {
    let path = plan.input_path;
    if let Some(format) = plan.input_format.as_ref() {
        let df = match format {
            FileOption::Avro => read_avro(path),
            FileOption::Parquet => read_parquet(path),
            FileOption::Csv => read_csv(path),
            FileOption::Pretty => unimplemented!(),
            FileOption::Json => read_json(path),
        };
        df.unwrap_or_else(|_| panic!("could not load dataframe"))
    } else {
        panic!("could not load dataframe")
    }
}

fn read_avro(path: &Path) -> Result<LazyFrame> {
    let f = file_utils::open_file(path);
    let df = AvroReader::new(f).finish().expect("could not parse avro");
    Ok(df.lazy())
}

fn read_parquet(path: &Path) -> Result<LazyFrame> {
    LazyFrame::scan_parquet(path, Default::default())
}

fn read_csv(path: &Path) -> Result<LazyFrame> {
    LazyCsvReader::new(path).finish()
}

fn read_json(path: &Path) -> Result<LazyFrame> {
    let string_path = path
        .to_str()
        .expect("could not read path to json file")
        .to_string();
    LazyJsonLineReader::new(string_path).finish()
}

pub fn write(plan: Plan, df: LazyFrame) -> Result<()> {
    let data = df.collect()?;
    let out_path = get_output_path(&plan);
    match plan.output_format {
        FileOption::Avro => write_avro(data, out_path),
        FileOption::Parquet => write_parquet(data, out_path),
        FileOption::Csv => write_csv(data, out_path),
        FileOption::Json => write_json(data, out_path),
        FileOption::Pretty => println!("{}", data),
    }
    Ok(())
}

fn get_output_path(plan: &Plan) -> Box<dyn Write> {
    match plan.output_path {
        Some(p) => Box::new(crate::file_utils::create_file(p)),
        None => Box::new(std::io::stdout()),
    }
}

fn write_avro(mut df: DataFrame, w: Box<dyn Write>) {
    AvroWriter::new(w)
        .finish(&mut df)
        .expect("error writing avro file")
}

fn write_json(mut df: DataFrame, w: Box<dyn Write>) {
    JsonWriter::new(w)
        .finish(&mut df)
        .expect("error writing json file")
}

fn write_parquet(mut df: DataFrame, w: Box<dyn Write>) {
    ParquetWriter::new(w)
        .finish(&mut df)
        .expect("error writing parquet file");
}

fn write_csv(mut df: DataFrame, w: Box<dyn Write>) {
    CsvWriter::new(w)
        .finish(&mut df)
        .expect("error writing csv");
}

#[cfg(test)]
mod test_io {
    use super::*;
    use crate::file_utils::create_file;
    use std::env;
    use std::path::PathBuf;

    static TEST_DIR: &str = "ptools_base_dir";

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
}
