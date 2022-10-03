pub mod io;
pub mod parse_args;
pub mod cli;
pub mod file_utils;

use cli::Cli;

pub fn set_env(ars: &Cli) {
    if let Some(ncols) = &ars.num_cols {
        std::env::set_var("POLARS_FMT_MAX_COLS", ncols);
    }
    if let Some(nrows) = &ars.num_rows {
        std::env::set_var("POLARS_FMT_MAX_ROWS", nrows);
    }
    if let Some(strlen) = &ars.string_len {
        std::env::set_var("POLARS_FMT_STR_LEN", strlen);
    }
}
