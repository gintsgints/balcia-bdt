use chrono::NaiveDate;
use include_oracle_sql::{impl_sql, include_sql};
use sibyl as oracle;
use std::str::FromStr;

use crate::bdt::column_type::ColumnType;
use crate::bdt::column_value::RowValues;
use crate::bdt::*;
use crate::csv_adapter::DataRow;

include_sql!("sql/bdt.sql");

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const FORMAT_ORACLE: &'static str = "yyyy-mm-dd hh:mi:ss";

fn to_naive_date(date: Option<oracle::Date>) -> Option<NaiveDate> {
    let date_str = date?
        .to_string(FORMAT_ORACLE)
        .expect("Failed toconvert date to sting");
    return Some(NaiveDate::parse_from_str(&date_str[..], FORMAT).expect("Error converting data"));
}

#[cfg(not(feature = "tokio"))]
pub fn read_oracle(table_ic: &str) -> sibyl::Result<Vec<Bdt>> {
    let oracle = sibyl::env()?;
    let session = oracle.connect("localhost/xe", "bta", "bta_234")?;

    let mut bdt_list: Vec<Bdt> = Vec::new();

    session.get_tables(table_ic, |row| {
        let ic: &str = row.get("IC")?;
        let valid_from: Option<oracle::Date> = row.get("VALID_FROM")?;
        let valid_to: Option<oracle::Date> = row.get("VALID_TO")?;
        let mut bdt = Bdt {
            skip: "".to_string(),
            ic: ic.to_string(),
            names: Vec::new(),
            valid_from: to_naive_date(valid_from),
            valid_to: to_naive_date(valid_to),
            columns: Vec::new(),
            data: Vec::new(),
        };

        session.get_table_names(table_ic, |row| {
            let table_name = TableName::new(
                Language::from_str(row.get("LNG_CODE")?).unwrap(),
                row.get("LNG_CODE")?,
                row.get("LNG_CODE")?,
                row.get("LNG_CODE")?,
            );
            bdt.names.push(table_name);
            Ok(())
        })?;

        session.business_table_column_definition(table_ic, |row| {
            let is_key_str: &str = row.get("IS_KEY")?;
            let cdf_ic: Option<String> = row.get("CODIFICATOR_IC")?;
            let select_params: Option<String> = row.get("SELECT_PARAMS")?;
            let options: Option<String> = row.get("OPTIONS")?;
            let column = Column {
                id: None,
                skip: "".to_string(),
                name: row.get("COL_NAME")?,
                title: row.get("TITLE")?,
                ref_code: row.get("REF_CODE")?,
                col_type: ColumnType::from_data_row(
                    row.get("REF_CODE")?,
                    cdf_ic.unwrap_or_default(),
                    select_params.unwrap_or_default(),
                ),
                sequence: row.get("SEQUENCE")?,
                is_key: "Y".eq(is_key_str),
                options: options.unwrap_or_default(),
            };
            bdt.columns.push(column);
            Ok(())
        })?;

        session.business_table_data(table_ic, |row| {
            let valid_from: Option<oracle::Date> = row.get("VALID_FROM")?;
            let valid_to: Option<oracle::Date> = row.get("VALID_TO")?;
            let row = DataRow {
                skip: "".to_string(),
                id: row.get("ID")?,
                table_type: bdt.ic.clone(),
                valid_from: to_naive_date(valid_from),
                valid_to: to_naive_date(valid_to),
                cdf1: row.get("CDF1_IC")?,
                cdf2: row.get("CDF2_IC")?,
                cdf3: row.get("CDF3_IC")?,
                cdf4: row.get("CDF4_IC")?,
                cdf5: row.get("CDF5_IC")?,
                cdf6: row.get("CDF6_IC")?,
                cdf7: row.get("CDF7_IC")?,
                cdf8: row.get("CDF8_IC")?,
                cdf9: row.get("CDF9_IC")?,
                cdf10: row.get("CDF10_IC")?,
                cdf11: row.get("CDF11_IC")?,
                cdf12: row.get("CDF12_IC")?,
                cdf13: row.get("CDF13_IC")?,
                cdf14: row.get("CDF14_IC")?,
                cdf15: row.get("CDF15_IC")?,
                num1: row.get("NUM1")?,
                num2: row.get("NUM2")?,
                num3: row.get("NUM3")?,
                num4: row.get("NUM4")?,
                num5: row.get("NUM5")?,
                num6: row.get("NUM6")?,
                num7: row.get("NUM7")?,
                num8: row.get("NUM8")?,
                num9: row.get("NUM9")?,
                num10: row.get("NUM10")?,
                text1: row.get("TEXT1")?,
                text2: row.get("TEXT2")?,
                text3: row.get("TEXT3")?,
                text4: row.get("TEXT4")?,
                text5: row.get("TEXT5")?,
            };
            let row_values = RowValues::from_data_row(&bdt.columns, row);
            bdt.data.push(row_values);
            Ok(())
        })?;

        bdt_list.push(bdt);
        Ok(())
    })?;

    Ok(bdt_list)
}
