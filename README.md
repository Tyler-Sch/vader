# Vader
## A CLI Tool for Converting and Inspecting Data

This tool wraps [polars](https://github.com/pola-rs/polars) to provide and easy and clean interface to convert parquet, avro, json, or csv files into other types or to display them to stdout for inspection or piping into another tool.

### Basic usage from bash:
```
# print a parquet file to stdout in new line separated json format
vader -i parquet -o json testfile.parquet

# create a avro file
vader -i parquet -o avro testfile.parquet nextfile.avro

# inspect avro data as csv
vader -i avro -o csv nextfile.avro
