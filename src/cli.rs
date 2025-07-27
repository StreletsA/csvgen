use std::str::FromStr;
use clap::{command, Arg};
use crate::generator::GeneratorParams;

const OUT_ARG_NAME: &str = "out";
const TABLE_COLUMNS_ARG_NAME: &str = "columns";
const ROWS_COUNT_ARG_NAME: &str = "rows";
const COLUMNS_NAME_ARG_DELIMITER: &str = ",";

pub fn parse_params() -> GeneratorParams {
    let arg_matches = command!()
        .arg(Arg::new(TABLE_COLUMNS_ARG_NAME).short('c').long(TABLE_COLUMNS_ARG_NAME).help("Column names (delimiter = ',')").required(true))
        .arg(Arg::new(OUT_ARG_NAME).short('o').long(OUT_ARG_NAME).help("Output filepath").required(true))
        .arg(Arg::new(ROWS_COUNT_ARG_NAME).short('r').long(ROWS_COUNT_ARG_NAME).help("Rows count").required(true))
        .get_matches();
    let columns_arg_value = arg_matches.get_one::<String>(TABLE_COLUMNS_ARG_NAME).unwrap().as_str();
    let out_arg_value = arg_matches.get_one::<String>(OUT_ARG_NAME).unwrap().as_str();
    let rows_arg_value = i32::from_str(
        arg_matches.get_one::<String>(ROWS_COUNT_ARG_NAME).unwrap().as_str()
    )
        .expect("Rows count must be a number!");
    GeneratorParams {
        columns: parse_column_names(columns_arg_value),
        out: out_arg_value.to_string(),
        rows_count: rows_arg_value,
    }
}

fn parse_column_names(columns_arg_value: &str) -> Vec<String> {
    columns_arg_value.split(COLUMNS_NAME_ARG_DELIMITER)
        .map(|name| strip_column_name(name))
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn strip_column_name(column_name: &str) -> &str {
    let mut name = column_name;
    if name.starts_with(" ") {
        name = name.strip_prefix(" ").unwrap();
    }
    if name.ends_with(" ") {
        name = name.strip_suffix(" ").unwrap();
    }
    name
}
