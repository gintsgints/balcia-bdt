#![allow(dead_code)]

use std::error::Error;
use std::fs::File;

use csv::{DeserializeRecordsIntoIter, ReaderBuilder};
use serde::{Deserialize};

use crate::bdt::column_value::RowValues;
use crate::bdt::*;

use self::csv_model::{TableRow, ColumnRow, DataRow};

pub mod csv_model;


pub struct CsvReader<T> {
    inner: DeserializeRecordsIntoIter<File, T>,
}

impl<T> CsvReader<T> {
    pub fn new(path: String) -> Result<Self, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a>,
    {
        let rdr = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_path(path)?;
        Ok(CsvReader {
            inner: rdr.into_deserialize(),
        })
    }
}

impl<T> Iterator for CsvReader<T>
where
    T: for<'a> Deserialize<'a>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(v) => match v {
                    Ok(t) => return Some(t),
                    Err(e) => {
                        println!("Parse error {}", e);
                    } // on error skip
                },
                None => return None,
            }
        }
    }
}

pub struct CsvAdapter {
    path: String,
    inner: CsvReader<TableRow>,
}

impl CsvAdapter {
    pub fn new(path: String) -> CsvAdapter {
        let inner = CsvReader::<TableRow>::new(String::from(path.clone() + "/tables.csv"))
            .expect("Error reading table csv");
        CsvAdapter { path, inner }
    }
}

impl Iterator for CsvAdapter {
    type Item = Bdt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next() {
                Some(row) => {
                    let mut bdt = Bdt {
                        skip: row.skip,
                        ic: row.ic,
                        names: Vec::new(),
                        valid_from: row.valid_from,
                        valid_to: row.valid_to,
                        columns: Vec::new(),
                        data: Vec::new(),
                    };

                    let en_name = TableName::new(
                        Language::EN,
                        row.name_en.clone(),
                        row.print_name_en.clone(),
                        row.short_print_name_en.clone(),
                    );
                    bdt.names.push(en_name);
                    let lv_name = TableName::new(
                        Language::LV,
                        row.name_lv.clone(),
                        row.print_name_lv.clone(),
                        row.short_print_name_lv.clone(),
                    );
                    bdt.names.push(lv_name);

                    let columns = CsvReader::<ColumnRow>::new(String::from(
                        self.path.clone() + "/columns.csv",
                    ))
                    .expect("Error reading column csv");
                    for row in columns.filter(|col_row| col_row.table_type_id == bdt.ic) {
                        let col = row.to_column();
                        bdt.columns.push(col);
                    }

                    let data =
                        CsvReader::<DataRow>::new(String::from(self.path.clone() + "/data.csv"))
                            .expect("Error reading data csv");
                    for row in data.filter(|data_row| data_row.table_type == bdt.ic) {
                        let data_row = RowValues::from_data_row(&bdt.columns, &row);
                        bdt.data.push(data_row);
                    }

                    return Some(bdt);
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_table_csv() {
        let iter = CsvReader::<TableRow>::new(String::from("./data/TT/tables.csv"))
            .expect("Error reading csv");
        let v: Vec<_> = iter.collect();
        assert_eq!(v.len(), 5);
        assert!(v.get(0).unwrap().valid_to.is_none());
        assert!(v.get(0).unwrap().valid_from.is_some());
    }

    #[test]
    fn read_table_with_skip_csv() {
        let iter = CsvReader::<TableRow>::new(String::from("./data/TT/tables.csv"))
            .expect("Error reading csv");
        let v: Vec<_> = iter.filter(|row| row.skip == String::from("")).collect();
        assert_eq!(v.len(), 3);
        assert_eq!(
            v.get(0).unwrap().ic,
            String::from("TT01_POWER_FACTOR_DEFAULT_SEARCH_CONFIG")
        );
    }

    #[test]
    fn read_column_csv() {
        let iter = CsvReader::<ColumnRow>::new(String::from("./data/TT/columns.csv"))
            .expect("Error reading csv");
        let v: Vec<_> = iter.collect();
        assert_eq!(v.len(), 23);
        assert_eq!(v.get(0).unwrap().table_type_id.as_str(), "TT_CONFIG");
    }

    #[test]
    fn read_data_csv() {
        let iter = CsvReader::<DataRow>::new(String::from("./data/TT/data.csv"))
            .expect("Error reading csv");
        let v: Vec<_> = iter.collect();
        assert_eq!(v.len(), 27);
        assert_eq!(v.get(0).unwrap().table_type.as_str(), "TT_CONFIG");
    }

    #[test]
    fn read_error_data_csv() {
        let iter = CsvReader::<DataRow>::new(String::from("./data/TT/data_error.csv"))
            .expect("Error reading csv");
        let v: Vec<_> = iter.collect();
        assert_eq!(v.len(), 2);
        assert_eq!(v.get(0).unwrap().table_type.as_str(), "TT_CONFIG");
    }

    #[test]
    #[should_panic]
    fn data_for_nonexistant_column() {
        let iter_d = CsvReader::<DataRow>::new(String::from("./data/TT/data_extra_data.csv"))
            .expect("Error reading csv");
        let data: Vec<DataRow> = iter_d.collect();
        assert_eq!(data.len(), 1);

        let columns = CsvReader::<ColumnRow>::new(String::from("./data/TT/columns.csv"))
            .expect("Error reading csv");
        let mut test_columns: Vec<Column> = Vec::new();
        for row in columns {
            if row.table_type_id == data.get(0).unwrap().table_type {
                let col = row.to_column();
                test_columns.push(col);
            }
        }
        RowValues::from_data_row(&test_columns, data.get(0).unwrap());
    }

    #[test]
    fn can_iterate_data() {
        let iter = CsvReader::<DataRow>::new(String::from("./data/TT/data.csv"))
            .expect("Error reading csv");
        let mut i = 0;
        for _row in iter.into_iter() {
            i += 1;
        }
        assert_eq!(i, 27);
    }

    #[test]
    fn read_bdt_columns_from_csv() {
        let adapter = CsvAdapter::new(String::from("./data/TT/"));
        let v: Vec<Bdt> = adapter.collect();
        assert_eq!(v.len(), 5);
        assert_eq!(v.get(0).unwrap().ic, String::from("TT_CONFIG"));
        assert_eq!(v.get(0).unwrap().columns.len(), 5);
    }

    #[test]
    fn read_bdt_data_from_csv() {
        let adapter = CsvAdapter::new(String::from("./data/TT/"));
        let v: Vec<Bdt> = adapter.collect();
        assert_eq!(v.len(), 5);
        assert_eq!(v.get(0).unwrap().ic, String::from("TT_CONFIG"));
        assert_eq!(v.get(0).unwrap().data.len(), 15);
    }

    #[test]
    fn test_default_is_key() {
        let adapter = CsvAdapter::new(String::from("./data/TT/"));
        let v: Vec<Bdt> = adapter.collect();
        assert_eq!(v.len(), 5);
        assert_eq!(v.get(4).unwrap().columns.len(), 4);
        assert_eq!(
            v.get(4).unwrap().ic,
            "TT02_DEPRECIATION_CONFIG_BY_VEHICLE_AGE"
        );
        assert_eq!(v.get(4).unwrap().columns.get(0).unwrap().name, "AGE_FROM");
        assert_eq!(v.get(4).unwrap().columns.get(0).unwrap().is_key, false);
        assert_eq!(v.get(4).unwrap().data.len(), 3);
    }
}
