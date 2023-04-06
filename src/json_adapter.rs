use std::error::Error;
use std::io::{stdin, stdout};

use crate::bdt::Bdt;

pub struct JsonAdapter {}

impl JsonAdapter {
    #![allow(dead_code)]
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
}
