use csv::DeserializeRecordsIntoIter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use std::{io, str::FromStr};

pub fn check_file_exists(file_name: &str) -> bool {
    match Path::new(file_name).try_exists() {
        Ok(exists) => exists,
        Err(_) => false,
    }
}

pub fn read_csv<T>(file_name: &str) -> Result<DeserializeRecordsIntoIter<File, T>, Box<dyn Error>>
where
    for<'a> T: Deserialize<'a>,
{
    let rdr = csv::Reader::from_path(file_name)?;

    Ok(rdr.into_deserialize::<T>())
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

    let result = s.parse::<T>().unwrap(); // If data format is wrong, panic.

    Ok(result)
}
