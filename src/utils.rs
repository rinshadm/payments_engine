use std::io;
use std::error::Error;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub fn check_file_exists(file_name: &str) -> bool {
    match Path::new(file_name).try_exists() {
        Ok(exists) => exists,
        Err(_) => false
    }
}

pub fn read_csv<T>(file_name: &str) -> Result<Vec<T>, Box<dyn Error>>
    where for<'a> T: Deserialize<'a>
{
    let mut rdr = csv::Reader::from_path(file_name)?;
    let mut records: Vec<T> = Vec::new();

    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

pub fn write_csv_stdout<T>(records: &Vec<T>) -> Result<(), Box<dyn Error>>
    where T: Serialize
{
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for record in records {
        wtr.serialize(record)?;
    }

    Ok(())
}