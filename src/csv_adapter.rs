#![allow(dead_code)]

use std::fs::File;
use std::error::Error;
use regex::Regex;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use csv::{ReaderBuilder, DeserializeRecordsIntoIter};

use crate::bdt::*;
use crate::bdt::column_type::ColumnType;
use crate::bdt::column_value::{ColumnValue, ColumnValueType};
use crate::bdt::column_value::RowValues;
use crate::en_date_format;

#[derive(Debug, Deserialize, Serialize)]
pub struct TableRow {
    pub skip: String,
    pub id: Option<u64>,
    pub adm_codificator_ic: String,
    pub parent_ic: String,
    pub ic: String,
    pub notes: String,
    pub code: String,
    #[serde(with = "en_date_format")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(with = "en_date_format")]
    pub valid_to: Option<DateTime<Utc>>,
    pub sequence: Option<u16>,
    pub name_lv: String,
    pub print_name_lv: String,
    pub short_print_name_lv: String,
    pub name_en: String,
    pub print_name_en: String,
    pub short_print_name_en: String,
    pub agc_tariff_plan_id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColumnRow {
    pub skip: String,
    pub id: Option<u64>,
    pub table_type_id: String,
    pub title: String,
    pub col_name: String,
    pub ref_code: String,
    pub adm_codificator_id: String,
    pub sequence: Option<u16>,
    pub is_key: String,
    pub options: String,
    pub select_params: String,
}

impl ColumnRow {
    fn to_column_type(&self) -> ColumnType {
        let re_fromto = Regex::new(r"VALID_(FROM|TO)").unwrap();
        let re_num = Regex::new(r"NUM[[:digit:]]+").unwrap();
        let re_text = Regex::new(r"TEXT[[:digit:]]+").unwrap();
        let re_cdf = Regex::new(r"CDF[[:digit:]]+_ID").unwrap();

        match re_fromto.find(self.ref_code.as_str()) {
            Some(_) => ColumnType::Date,
            None => match re_num.find(self.ref_code.as_str()) {
                Some(_) => ColumnType::Num,
                None => match re_text.find(self.ref_code.as_str()) {
                    Some(_) => ColumnType::Text,
                    None => match re_cdf.find(self.ref_code.as_str()) {
                        Some(_) => ColumnType::Cdf {
                            codificator_id: self.adm_codificator_id.clone(),
                            select_params: self.select_params.clone(),
                        },
                        None => panic!("Wrong match for {}", self.ref_code.as_str())
                    },
                },
            },
        }
    }

    fn to_column(&self) -> Column {
        Column {
            skip: self.skip.clone(),
            id: self.id,
            name: self.col_name.clone(),
            title: self.title.clone(),
            ref_code: self.ref_code.clone(),
            col_type: self.to_column_type(),
            sequence: self.sequence,
            is_key: self.is_key.eq("Y"),
            options: self.options.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataRow {
    pub skip: String,
    pub id: Option<u64>,
    pub table_type: String,
    #[serde(with = "en_date_format")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(with = "en_date_format")]
    pub valid_to: Option<DateTime<Utc>>,
    pub cdf1: String,
    pub cdf2: String,
    pub cdf3: String,
    pub cdf4: String,
    pub cdf5: String,
    pub cdf6: String,
    pub cdf7: String,
    pub cdf8: String,
    pub cdf9: String,
    pub cdf10: String,
    pub cdf11: String,
    pub cdf12: String,
    pub cdf13: String,
    pub cdf14: String,
    pub cdf15: String,
    pub num1: Option<f64>,
    pub num2: Option<f64>,
    pub num3: Option<f64>,
    pub num4: Option<f64>,
    pub num5: Option<f64>,
    pub num6: Option<f64>,
    pub num7: Option<f64>,
    pub num8: Option<f64>,
    pub num9: Option<f64>,
    pub num10: Option<f64>,
    pub text1: String,
    pub text2: String,
    pub text3: String,
    pub text4: String,
    pub text5: String,
}

impl DataRow {
    fn to_column_value(&self, columns: &Vec<Column>) -> RowValues {
        let mut values = RowValues::new();
        for col in columns.iter() {
            let column_value = match col.ref_code.as_str() {
                "VALID_FROM" => { ColumnValue::new(col.name.clone(), ColumnValueType::Date(self.valid_from)) }
                "VALID_TO" => { ColumnValue::new(col.name.clone(), ColumnValueType::Date(self.valid_to)) }
                "NUM1" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num1)) }
                "NUM2" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num2)) }
                "NUM3" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num3)) }
                "NUM4" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num4)) }
                "NUM5" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num5)) }
                "NUM6" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num6)) }
                "NUM7" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num7)) }
                "NUM8" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num8)) }
                "NUM9" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num9)) }
                "NUM10" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(self.num10)) }
                "TEXT1" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(self.text1.clone())) }
                "TEXT2" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(self.text2.clone())) }
                "TEXT3" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(self.text3.clone())) }
                "TEXT4" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(self.text4.clone())) }
                "TEXT5" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(self.text5.clone())) }
                "CDF1_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf1.clone())) }
                "CDF2_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf2.clone())) }
                "CDF3_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf3.clone())) }
                "CDF4_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf4.clone())) }
                "CDF5_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf5.clone())) }
                "CDF6_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf6.clone())) }
                "CDF7_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf7.clone())) }
                "CDF8_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf8.clone())) }
                "CDF9_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf9.clone())) }
                "CDF10_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf10.clone())) }
                "CDF11_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf11.clone())) }
                "CDF12_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf12.clone())) }
                "CDF13_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf13.clone())) }
                "CDF14_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf14.clone())) }
                "CDF15_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(self.cdf15.clone())) }
                &_ => todo!()
            };
            values.push(column_value);
        }
        return values;
    }
}

pub struct CsvReader<T> {
    inner: DeserializeRecordsIntoIter<File, T>
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
            inner: rdr.into_deserialize()
        })
    } 
}

impl<T> Iterator for CsvReader<T>
where
  T: for<'a> Deserialize<'a>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
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
        let inner = CsvReader::<TableRow>::new(String::from(path.clone() + "/tables.csv")).expect("Error reading table csv");
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
                        data: Vec::new()
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

                    let columns = CsvReader::<ColumnRow>::new(String::from(self.path.clone() + "/columns.csv")).expect("Error reading column csv");
                    for row in columns.filter(|col_row| col_row.table_type_id == bdt.ic) {
                        let col = row.to_column();
                        bdt.columns.push(col);
                    }

                    let data = CsvReader::<DataRow>::new(String::from(self.path.clone() + "/data.csv")).expect("Error reading data csv");
                    for row in data.filter(|data_row| data_row.table_type == bdt.ic) {
                        let data_row = row.to_column_value(&bdt.columns);
                        bdt.data.push(data_row);
                    }

                    return Some(bdt)
                },
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
        let iter = CsvReader::<TableRow>::new(String::from("./data/TT/tables.csv")).expect("Error reading csv");
        let v: Vec<_> = iter.collect();
        assert_eq!(v.len(), 5);
        assert!(v.get(0).unwrap().valid_to.is_none());
        assert!(v.get(0).unwrap().valid_from.is_some());
    }

    #[test]
    fn read_table_with_skip_csv() {
        let iter = CsvReader::<TableRow>::new(String::from("./data/TT/tables.csv")).expect("Error reading csv");
        let v: Vec<_> = iter.filter(|row|row.skip == String::from("")).collect();
        assert_eq!(v.len(), 3);
        assert_eq!(v.get(0).unwrap().ic, String::from("TT01_POWER_FACTOR_DEFAULT_SEARCH_CONFIG"));
    }

    #[test]
    fn read_column_csv() {
        let iter = CsvReader::<ColumnRow>::new(String::from("./data/TT/columns.csv")).expect("Error reading csv");
        let v: Vec<_> = iter.collect();
        assert_eq!(v.len(), 23);
        assert_eq!(v.get(0).unwrap().table_type_id.as_str(), "TT_CONFIG");
    }

    #[test]
    fn read_data_csv() {
        let iter = CsvReader::<DataRow>::new(String::from("./data/TT/data.csv")).expect("Error reading csv");
        let v: Vec<_> = iter.collect();
        assert_eq!(v.len(), 27);
        assert_eq!(v.get(0).unwrap().table_type.as_str(), "TT_CONFIG");
    }

    #[test]
    fn read_error_data_csv() {
        let iter = CsvReader::<DataRow>::new(String::from("./data/TT/data_error.csv")).expect("Error reading csv");
        let v: Vec<_> = iter.collect();
        assert_eq!(v.len(), 2);
        assert_eq!(v.get(0).unwrap().table_type.as_str(), "TT_CONFIG");
    }

    #[test]
    fn can_iterate_data() {
        let iter = CsvReader::<DataRow>::new(String::from("./data/TT/data.csv")).expect("Error reading csv");
        let mut i = 0;
        for _row in iter.into_iter() {
            i += 1;
        }
        assert_eq!(i, 27);
    }

    #[test]
    fn read_bdt_from_csv() {
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
        assert_eq!(v.get(4).unwrap().ic, "TT02_DEPRECIATION_CONFIG_BY_VEHICLE_AGE");
        assert_eq!(v.get(4).unwrap().columns.get(0).unwrap().name, "AGE_FROM");
        assert_eq!(v.get(4).unwrap().columns.get(0).unwrap().is_key, false);
        assert_eq!(v.get(4).unwrap().data.len(), 3);
    }
}