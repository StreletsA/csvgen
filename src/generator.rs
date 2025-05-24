use std::fs::File;
use std::io::Write;
use std::path::Path;
use rand::distr::{Alphanumeric, SampleString};

const CSV_DELIMITER: &str = ";";

pub struct GeneratorParams {
    pub columns: Vec<String>,
    pub out: String,
    pub rows_count: i32,
}

pub fn generate_csv(params: GeneratorParams) {
    let path = Path::new(params.out.as_str());
    let mut csv_file = File::create(path).unwrap();
    write_table_header(&params.columns, &mut csv_file);
    fill_rows(params.columns.len() as i32, params.rows_count, &mut csv_file);
}

fn write_table_header(columns: &Vec<String>, csv_file: &mut File) {
    for i in 0..columns.len() {
        if i > 0 {
            csv_file.write(CSV_DELIMITER.as_bytes())
                .expect(format!("Delimiter of column with index {} write error", i).as_str());
        }
        csv_file.write(columns[i].as_bytes())
            .expect(format!("Column with index {} write error", i).as_str());
    }
    println!("Headers wrote");
}

fn fill_rows(columns_count: i32, rows_count: i32, csv_file: &mut File) {
    let row = generate_row(columns_count);
    for i in 0..rows_count {
        csv_file.write("\n".as_bytes())
            .expect(format!("New line of row with index {} write error", i).as_str());
        csv_file.write(row.as_bytes())
            .expect(format!("Row with index {} write error", i).as_str());

        if (i + 1) % (rows_count as f64 * 0.1) as i32 == 0 {
            println!("{} rows wrote", i + 1)
        }
    }
    println!("Finish. {} rows wrote", rows_count)
}

fn generate_row(columns_count: i32) -> String {
    let mut row: String = "".to_string();
    for i in 0..columns_count {
        if i > 0 {
            row = row + ";";
        }
        row = row + Alphanumeric.sample_string(&mut rand::rng(), 16).as_str();
    }
    row
}