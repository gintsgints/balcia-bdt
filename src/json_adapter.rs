use std::fs::File;
use std::error::Error;

use crate::bdt::Bdt;

pub struct JsonAdapter {}

impl JsonAdapter {
    pub fn write_bdt(value: Vec<Bdt>) -> Result<(), Box<dyn Error>> {
        let output_file = File::create("data/TT/bdt.json")?;
        serde_json::to_writer(output_file, &value)?;
        Ok(())
    }
}
