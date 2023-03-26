use std::error::Error;

mod csv_adapter;
mod json_adapter;
mod bdt;
mod en_date_format;
mod sql_adapter;

use crate::csv_adapter::CsvAdapter;
use crate::bdt::Bdt;

fn main() -> Result<(), Box<dyn Error>> {
    let adapter = CsvAdapter::new(String::from("./data/TT/"));
    let v: Vec<Bdt> = adapter.collect();

    crate::sql_adapter::write_bdt(v)?;
    // crate::json_adapter::JsonAdapter::write_bdt(v)?;

    Ok(())
}
