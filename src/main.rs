use std::error::Error;

// use crate::csv_adapter::TableRow;

mod csv_adapter;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}
