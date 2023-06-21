use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::bdt::Bdt;

pub struct JsonAdapter {}

impl JsonAdapter {
    pub fn write_bdt(value: Vec<Bdt>, path: &str) -> Result<(), Box<dyn Error>> {
        let output_file = File::create(path)?;
        let mut writer = BufWriter::new(output_file);
        serde_json::to_writer_pretty(&mut writer, &value)?;
        Ok(())
    }

    pub fn read_bdt(path: &str) -> Result<Vec<Bdt>, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let result: Vec<Bdt> = serde_json::from_reader(reader)?;
        Ok(result)
    }
}
