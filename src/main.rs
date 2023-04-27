use std::error::Error;

use clap::{Args, Parser, Subcommand};
use csv_adapter::CsvWriter;

use crate::bdt::Bdt;
use crate::csv_adapter::CsvAdapter;
use crate::csv_data_adapter::CsvDataAdapter;
use crate::json_adapter::JsonAdapter;
use crate::sqlite_adapter::SqliteAdapter;

mod bdt;
mod csv_adapter;
mod csv_data_adapter;
mod format;
mod json_adapter;
#[cfg(feature = "oracle")]
mod oracle_adapter;
mod sql_adapter;
mod sqlite_adapter;

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
    /// process CSV files
    Csv(CsvCommand),
    /// Read from oracle DB (Appropriate environment variables should be set for
    /// DBNAME - database name example: localhost/xe
    /// DBUSER and DBPASS
    #[cfg(feature = "oracle")]
    Oracle(OracleCommand),
    /// Write business tables from stdin as SQL scripts
    Sql(SqlCommand),
    /// Write to stdout sqlite load script from stdin JSON bdt
    Sqlite(SqliteCommand),
}

#[derive(Debug, Args)]
pub struct CsvCommand {
    #[clap(subcommand)]
    pub subcommand: CsvSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum CsvSubCommand {
    /// Reads CSV files from provided directory and outputs as JSON
    Read(CsvReadCommand),
    /// Reads JSON from stdin and Write CSV data to files at provided path
    Write(CsvWriteCommand),
    /// Write csv data for only defined columns. This also skip header at top.
    Data(CsvDataCommand),
}

#[derive(Debug, Args)]
pub struct CsvReadCommand {
    /// path to csv file directory
    path: String,
}

#[derive(Debug, Args)]
pub struct CsvWriteCommand {
    /// path to csv file directory
    path: String,
}

#[derive(Debug, Args)]
pub struct CsvDataCommand {}

#[derive(Debug, Args)]
#[cfg(feature = "oracle")]
pub struct OracleCommand {
    /// business table IC code
    table_ic_code: String,
}

#[derive(Debug, Args)]
pub struct SqliteCommand {}

#[derive(Debug, Args)]
pub struct SqlCommand {
    /// business table IC code
    table_ic_code: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match &args.command {
        Adapter::Csv(args) => match &args.subcommand {
            CsvSubCommand::Read(args) => {
                let adapter = CsvAdapter::new(String::from(&args.path));
                let v: Vec<Bdt> = adapter.collect();
                JsonAdapter::write_bdt(v)?;
            }
            CsvSubCommand::Write(args) => {
                let v: Vec<Bdt> = JsonAdapter::read_bdt()?;
                let writer = CsvWriter::new();
                writer.write_bdt(v, String::from(&args.path))?;
            }
            CsvSubCommand::Data(_args) => {
                let v = JsonAdapter::read_bdt()?;
                if v.len() != 1 {
                    panic!("CSV data could be done for one only BDT.")
                }
                for bdt in v {
                    CsvDataAdapter::write_bdt(bdt)?;
                }
            }
        },
        #[cfg(feature = "oracle")]
        Adapter::Oracle(args) => {
            let v: Vec<Bdt> = oracle_adapter::read_oracle(&args.table_ic_code)?;
            JsonAdapter::write_bdt(v)?;
        }
        Adapter::Sqlite(_args) => {
            let v: Vec<Bdt> = JsonAdapter::read_bdt()?;
            SqliteAdapter::write_bdt(v)?;
        }
        Adapter::Sql(_args) => {
            let v: Vec<Bdt> = JsonAdapter::read_bdt()?;
            crate::sql_adapter::write_bdt(v)?;
        }
    }
    Ok(())
}
