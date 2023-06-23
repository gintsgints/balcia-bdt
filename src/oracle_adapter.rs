use std::str::FromStr;

use chrono::NaiveDate;
use dotenv::dotenv;
use include_oracle_sql::{impl_sql, include_sql};
use sibyl as oracle;

use crate::bdt::column_type::ColumnType;
use crate::bdt::column_value::RowValues;
use crate::bdt::table_name::{Language, Name, NameList};
use crate::bdt::*;
use crate::csv_adapter::csv_model::{ColumnRow, DataRow};

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

    dotenv().ok();

    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");
    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;

    let mut bdt_list: Vec<Bdt> = Vec::new();

    session.get_tables(table_ic, |row| {
        let ic: &str = row.get("IC")?;
        let valid_from: Option<oracle::Date> = row.get("VALID_FROM")?;
        let valid_to: Option<oracle::Date> = row.get("VALID_TO")?;
        let mut bdt = Bdt {
            skip: "".to_string(),
            ic: ic.to_string(),
            names: NameList::new(Vec::new()),
            valid_from: to_naive_date(valid_from),
            valid_to: to_naive_date(valid_to),
            columns: Vec::new(),
            data: Vec::new(),
        };

        session.get_table_names(ic, |row| {
            let table_name = Name::new(
                Language::from_str(row.get("LNG_CODE")?).unwrap(),
                row.get("NAME").unwrap_or_default(),
                row.get("PRINT_NAME").unwrap_or_default(),
                row.get("SHORT_PRINT_NAME").unwrap_or_default(),
            );
            bdt.names.push(table_name);
            Ok(())
        })?;

        session.business_table_column_definition(ic, |row| {
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
                sequence: row.get("SEQUENCE")?,
                col_type: ColumnType::from(&ColumnRow {
                    skip: "".to_string(),
                    id: None,
                    table_type_id: "".to_string(),
                    title: "".to_string(),
                    col_name: "".to_string(),
                    ref_code: row.get("REF_CODE")?,
                    adm_codificator_id: cdf_ic.unwrap_or_default(),
                    sequence: None,
                    is_key: "".to_string(),
                    options: "".to_string(),
                    select_params: select_params.unwrap_or_default(),
                }),
                is_key: "Y".eq(is_key_str),
                options: options.unwrap_or_default(),
            };
            bdt.columns.push(column);
            Ok(())
        })?;

        session.business_table_data(ic, |row| {
            let valid_from: Option<oracle::Date> = row.get("VALID_FROM")?;
            let valid_to: Option<oracle::Date> = row.get("VALID_TO")?;
            let cdf1: Option<String> = row.get("CDF1_IC")?;
            let cdf2: Option<String> = row.get("CDF2_IC")?;
            let cdf3: Option<String> = row.get("CDF3_IC")?;
            let cdf4: Option<String> = row.get("CDF4_IC")?;
            let cdf5: Option<String> = row.get("CDF5_IC")?;
            let cdf6: Option<String> = row.get("CDF6_IC")?;
            let cdf7: Option<String> = row.get("CDF7_IC")?;
            let cdf8: Option<String> = row.get("CDF8_IC")?;
            let cdf9: Option<String> = row.get("CDF9_IC")?;
            let cdf10: Option<String> = row.get("CDF10_IC")?;
            let cdf11: Option<String> = row.get("CDF11_IC")?;
            let cdf12: Option<String> = row.get("CDF12_IC")?;
            let cdf13: Option<String> = row.get("CDF13_IC")?;
            let cdf14: Option<String> = row.get("CDF14_IC")?;
            let cdf15: Option<String> = row.get("CDF15_IC")?;
            let text1: Option<String> = row.get("TEXT1")?;
            let text2: Option<String> = row.get("TEXT2")?;
            let text3: Option<String> = row.get("TEXT3")?;
            let text4: Option<String> = row.get("TEXT4")?;
            let text5: Option<String> = row.get("TEXT5")?;
            let row = DataRow {
                skip: "".to_string(),
                id: row.get("ID")?,
                table_type: bdt.ic.clone(),
                valid_from: to_naive_date(valid_from),
                valid_to: to_naive_date(valid_to),
                cdf1: cdf1.unwrap_or_default(),
                cdf2: cdf2.unwrap_or_default(),
                cdf3: cdf3.unwrap_or_default(),
                cdf4: cdf4.unwrap_or_default(),
                cdf5: cdf5.unwrap_or_default(),
                cdf6: cdf6.unwrap_or_default(),
                cdf7: cdf7.unwrap_or_default(),
                cdf8: cdf8.unwrap_or_default(),
                cdf9: cdf9.unwrap_or_default(),
                cdf10: cdf10.unwrap_or_default(),
                cdf11: cdf11.unwrap_or_default(),
                cdf12: cdf12.unwrap_or_default(),
                cdf13: cdf13.unwrap_or_default(),
                cdf14: cdf14.unwrap_or_default(),
                cdf15: cdf15.unwrap_or_default(),
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
                text1: text1.unwrap_or_default(),
                text2: text2.unwrap_or_default(),
                text3: text3.unwrap_or_default(),
                text4: text4.unwrap_or_default(),
                text5: text5.unwrap_or_default(),
            };
            let row_values = RowValues::from_data_row(&bdt.columns, &row);
            bdt.data.push(row_values);
            Ok(())
        })?;

        bdt_list.push(bdt);
        Ok(())
    })?;

    Ok(bdt_list)
}
