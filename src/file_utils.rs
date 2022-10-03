use anyhow::{anyhow, Result};
use std::fs::{create_dir_all, File};
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn create_file(p: &PathBuf) -> File {
    let path = p.as_path();
    File::create(path).unwrap()
}

pub fn open_file(p: &Path) -> Result<File> {
    let f = File::open(p);
    match f {
        Ok(file) => Ok(file),
        Err(e) => Err(anyhow!("Error opening file {:?}", p)),
    }
}

pub fn create_dir(p: &Path) -> Result<(), Error> {
    create_dir_all(p)
}

// pub fn get_file_format(path: &PathBuf) -> FileOption {
//     let p = path.as_path();
//     let end = p.extension();
//     match end {
//         Some(fformat) => get_format(fformat.to_str()),
//         None => FileOption::StdOut,
//     }
// }

// fn get_format(s: Option<&str>) -> FileOption {
//     let st = s.unwrap_or("stdout");
//     match st {
//         "parquet" => FileOption::Parquet,
//         "csv" => FileOption::Csv,
//     }
// }

// #[test]
// fn test_os() {
//     let mut pbuf = PathBuf::new();
//     pbuf.push("test");
//     pbuf.push("path");
//     pbuf.push("p.parquet");
//     println!("{:?}", pbuf);
//     let r = get_file_format(&pbuf);
//     println!("{:?}", r);
// }
