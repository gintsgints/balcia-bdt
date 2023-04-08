use std::error::Error;

use clap::{Args, Parser, Subcommand};

use crate::bdt::Bdt;
use crate::csv_adapter::CsvAdapter;
use crate::json_adapter::JsonAdapter;

mod bdt;
mod csv_adapter;
mod format;
mod json_adapter;
#[cfg(feature = "oracle")]
mod oracle_adapter;
mod sql_adapter;

/// Convert BDT from one format to other
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Adapter
    #[clap(subcommand)]
    pub command: Adapter,
}

#[derive(Debug, Subcommand)]
pub enum Adapter {
    /// Reads CSV files from dir and outputs JSON to stdout
    Csv(CsvCommand),
    /// Read from oracle DB
    #[cfg(feature = "oracle")]
    Oracle(OracleCommand),
    /// Write business tables from stdin as SQL scripts
    Sql(SqlCommand),
}

#[derive(Debug, Args)]
pub struct CsvCommand {
    /// path to csv file directory
    path: String,
}

#[derive(Debug, Args)]
pub struct CsvReadCommand {}

#[derive(Debug, Args)]
#[cfg(feature = "oracle")]
pub struct OracleCommand {
    /// business table IC code
    table_ic_code: String,
}

#[derive(Debug, Args)]
pub struct SqlCommand {
    /// business table IC code
    table_ic_code: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match &args.command {
        Adapter::Csv(args) => {
            let adapter = CsvAdapter::new(String::from(&args.path));
            let v: Vec<Bdt> = adapter.collect();
            JsonAdapter::write_bdt(v)?;
        }
        #[cfg(feature = "oracle")]
        Adapter::Oracle(args) => {
            let v: Vec<Bdt> = oracle_adapter::read_oracle(&args.table_ic_code)?;
            JsonAdapter::write_bdt(v)?;
        }
        Adapter::Sql(_args) => {
            let v: Vec<Bdt> = JsonAdapter::read_bdt()?;
            crate::sql_adapter::write_bdt(v)?;
        }
    }
    Ok(())
}
