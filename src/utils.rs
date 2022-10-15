use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::{io, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn check_file_exists(file_name: &str) -> bool {
    match Path::new(file_name).try_exists() {
        Ok(exists) => exists,
        Err(_) => false,
    }
}

pub fn read_csv<T>(file_name: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    for<'a> T: Deserialize<'a>,
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
where
    T: Serialize,
{
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for record in records {
        wtr.serialize(record)?;
    }

    Ok(())
}

pub fn set_precision_to_four<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Since x is already an f64,
    // we are confident to parse and unwrap
    s.serialize_f64(format!("{:.4}", x).parse().unwrap())
}

pub fn trim<'de, 'a, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
    T: Default,
{
    let mut s: &str = Deserialize::deserialize(deserializer)?;
    s = s.trim();

    if s.is_empty() {
        return Ok(Default::default());
    }

    let result = s.trim().parse::<T>().unwrap(); // If data format is wrong, panic.

    Ok(result)
}
