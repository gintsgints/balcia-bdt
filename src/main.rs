use std::error::Error;

mod bdt;
mod csv_adapter;
mod format;
mod json_adapter;
mod oracle_adapter;
mod sql_adapter;

// use crate::csv_adapter::CsvAdapter;
use crate::bdt::Bdt;

fn main() -> Result<(), Box<dyn Error>> {
    // let adapter = CsvAdapter::new(String::from("./data/TT/"));
    // let v: Vec<Bdt> = adapter.collect();

    // crate::sql_adapter::write_bdt(v)?;
    let v: Vec<Bdt> = oracle_adapter::read_oracle("TT_CONFIG")?;
    crate::json_adapter::JsonAdapter::write_bdt(v)?;
    Ok(())
}
