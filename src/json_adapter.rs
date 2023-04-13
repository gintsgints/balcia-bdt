use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufReader};

use crate::bdt::Bdt;

pub struct JsonAdapter {}

impl JsonAdapter {
    pub fn write_bdt(value: Vec<Bdt>) -> Result<(), Box<dyn Error>> {
        let output_file = stdout();
        serde_json::to_writer(output_file, &value)?;
        Ok(())
    }

    pub fn read_bdt() -> Result<Vec<Bdt>, Box<dyn Error>> {
        let input_file = stdin();
        let result: Vec<Bdt> = serde_json::from_reader(input_file)?;
        Ok(result)
    }

    #[allow(dead_code)]
    pub fn read_bdt_from_file(path: &str) -> Result<Vec<Bdt>, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let result: Vec<Bdt> = serde_json::from_reader(reader)?;
        Ok(result)
    }
}
