#[derive(Debug, PartialEq, Eq)]
pub enum FileOption {
    Csv,
    Parquet,
    Pretty,
    Json,
    Avro,
}