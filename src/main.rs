use std::error::Error;

// use crate::csv_adapter::TableRow;

mod csv_adapter;
mod bdt;
mod en_date_format;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}
