mod bdt;
mod classifier;
mod csv_adapter;
mod csv_data_adapter;
mod format;
mod json_adapter;
mod l11n;
#[cfg(feature = "oracle")]
mod oracle_adapter;
mod sql_adapter;
mod sqlite_adapter;

use std::error::Error;

use clap::{Args, Parser, Subcommand};
use csv_adapter::CsvWriter;

use crate::bdt::Bdt;
use crate::csv_adapter::CsvAdapter;
use crate::json_adapter::JsonAdapter;
use crate::sqlite_adapter::SqliteAdapter;

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
    /// Write BDT to CSV in form of exported data
    Data(CsvDataCommand),
}

#[derive(Debug, Args)]
pub struct CsvReadCommand {
    /// path to csv file directory
    path: String,
    /// json output file path with filename
    filename: String,
}

#[derive(Debug, Args)]
pub struct CsvWriteCommand {
    /// json input file path with filename
    filename: String,
    /// path to csv file directory
    path: String,
}

#[derive(Debug, Args)]
pub struct CsvDataCommand {
    /// json input file path with filename
    filename: String,
    /// path to csv file directory
    path: String,
    /// Name of the table to extrace
    table: String,
}

#[derive(Debug, Args)]
#[cfg(feature = "oracle")]
pub struct OracleCommand {
    /// business table IC code or LIKE mask
    table_ic_code: String,
    /// json output file path with filename
    filename: String,
}

#[derive(Debug, Args)]
pub struct SqliteCommand {
    /// json output file path with filename
    filename: String,
    /// sql output file full path
    sqlfile: String,
    /// (optional) business table IC code to filter out
    table_ic_code: Option<String>,
}

#[derive(Debug, Args)]
pub struct SqlCommand {
    /// json input file path with filename
    filename: String,
    /// sql output file full path
    sqlfile: String,
    /// (optional) business table IC code to filter out
    table_ic_code: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match &args.command {
        Adapter::Csv(args) => match &args.subcommand {
            CsvSubCommand::Read(args) => {
                let adapter = CsvAdapter::new(String::from(&args.path));
                let v: Vec<Bdt> = adapter.collect();
                JsonAdapter::write_bdt(v, &args.filename)?;
            }
            CsvSubCommand::Write(args) => {
                let v: Vec<Bdt> = JsonAdapter::read_bdt(&args.filename)?;
                let writer = CsvWriter::new();
                writer.write_bdt(v, String::from(&args.path))?;
            }
            CsvSubCommand::Data(args) => {
                let v: Vec<Bdt> = JsonAdapter::read_bdt(&args.filename)?;

                if let Some(bdt) = v.into_iter().find(|bdt| bdt.ic == args.table) {
                    csv_data_adapter::write_csv_data(&args.path, &bdt)?;
                } else {
                    panic!("Table with name: {} not found.", args.table)
                }
            }
        },
        #[cfg(feature = "oracle")]
        Adapter::Oracle(args) => {
            let v: Vec<Bdt> = oracle_adapter::read_oracle(&args.table_ic_code)?;
            JsonAdapter::write_bdt(v, &args.filename)?;
        }
        Adapter::Sqlite(args) => {
            let v: Vec<Bdt> = JsonAdapter::read_bdt(&args.filename)?;
            match &args.table_ic_code {
                Some(table) => {
                    let filtered: Vec<Bdt> =
                        v.into_iter().filter(|flt| table.eq(&flt.ic)).collect();
                    SqliteAdapter::write_bdt(filtered, &args.sqlfile)?;
                }
                None => {
                    SqliteAdapter::write_bdt(v, &args.sqlfile)?;
                }
            }
        }
        Adapter::Sql(args) => {
            let v: Vec<Bdt> = JsonAdapter::read_bdt(&args.filename)?;
            match &args.table_ic_code {
                Some(table) => {
                    let filtered: Vec<Bdt> =
                        v.into_iter().filter(|flt| table.eq(&flt.ic)).collect();
                    crate::sql_adapter::write_bdt(filtered, &args.sqlfile)?;
                }
                None => {
                    crate::sql_adapter::write_bdt(v, &args.sqlfile)?;
                }
            }
        }
    }
    Ok(())
}
