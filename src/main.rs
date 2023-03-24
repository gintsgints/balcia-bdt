use std::error::Error;

mod csv_adapter;
mod bdt;
mod en_date_format;
mod sql_adapter;

use crate::csv_adapter::CsvAdapter;
use crate::bdt::Bdt;
use crate::sql_adapter::write_bdt;

fn main() -> Result<(), Box<dyn Error>> {
    let adapter = CsvAdapter::new(String::from("./data/TT/"));
    let v: Vec<Bdt> = adapter.collect();

    write_bdt(v)?;

    Ok(())
}
