use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::bdt::Column;
use crate::csv_adapter::DataRow;
use crate::format::lv_date_format;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ColumnValueType {
    #[serde(with = "lv_date_format")]
    Date(Option<NaiveDate>),
    Cdf(String),
    Num(Option<f64>),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnValue {
    pub name: String,
    pub ref_code: String,
    pub value: ColumnValueType,
}

impl ColumnValue {
    pub fn new(name: String, ref_code: String, value: ColumnValueType) -> ColumnValue {
        ColumnValue {
            name,
            ref_code,
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowValues {
    values: Vec<ColumnValue>,
}

impl RowValues {
    pub fn new() -> RowValues {
        RowValues { values: Vec::new() }
    }

    pub fn push(&mut self, value: ColumnValue) {
        self.values.push(value)
    }

    #[allow(dead_code)]
    pub fn len(&mut self) -> usize {
        self.values.len()
    }

    #[allow(dead_code)]
    pub fn get(&mut self, index: usize) -> Option<&ColumnValue> {
        self.values.get(index)
    }

    pub fn from_data_row(columns: &Vec<Column>, row: DataRow) -> RowValues {
        let mut values = RowValues::new();
        match RowValues::check_and_add_date_field(columns, "VALID_FROM", row.valid_from) {
            Some(column_value) => values.push(column_value),
            None => {}
        };
        match RowValues::check_and_add_date_field(columns, "VALID_TO", row.valid_from) {
            Some(column_value) => values.push(column_value),
            None => {}
        };
        match RowValues::check_and_add_num_field(columns, "NUM1", row.num1) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM2", row.num2) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM3", row.num3) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM4", row.num4) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM5", row.num5) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM6", row.num6) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM7", row.num7) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM8", row.num8) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM9", row.num9) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_num_field(columns, "NUM10", row.num10) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_text_field(columns, "TEXT1", row.text1.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_text_field(columns, "TEXT2", row.text2.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_text_field(columns, "TEXT3", row.text3.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_text_field(columns, "TEXT4", row.text4.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_text_field(columns, "TEXT5", row.text5.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF1_ID", row.cdf1.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF2_ID", row.cdf2.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF3_ID", row.cdf3.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF4_ID", row.cdf4.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF5_ID", row.cdf5.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF6_ID", row.cdf6.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF7_ID", row.cdf7.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF8_ID", row.cdf8.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF9_ID", row.cdf9.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF10_ID", row.cdf10.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF11_ID", row.cdf11.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF12_ID", row.cdf12.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF13_ID", row.cdf13.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF14_ID", row.cdf14.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        match RowValues::check_and_add_cdf_field(columns, "CDF15_ID", row.cdf15.clone()) {
            Some(column_value) => values.push(column_value),
            None => {}
        }
        return values;
    }

    fn check_and_add_date_field(
        columns: &Vec<Column>,
        ref_code: &str,
        date: Option<NaiveDate>,
    ) -> Option<ColumnValue> {
        if date.is_some() {
            let find_col = columns.into_iter().find(|col| col.ref_code == ref_code);
            let column_value = match find_col {
                Some(value) => ColumnValue::new(
                    value.name.clone(),
                    value.ref_code.clone(),
                    ColumnValueType::Date(date),
                ),
                None => panic!("CSV has data undefined column {}", ref_code),
            };
            return Some(column_value);
        }
        return None;
    }

    fn check_and_add_num_field(
        columns: &Vec<Column>,
        ref_code: &str,
        num: Option<f64>,
    ) -> Option<ColumnValue> {
        if num.is_some() {
            let find_col = columns.into_iter().find(|col| col.ref_code == ref_code);
            let column_value = match find_col {
                Some(value) => ColumnValue::new(
                    value.name.clone(),
                    value.ref_code.clone(),
                    ColumnValueType::Num(num),
                ),
                None => panic!("CSV has data undefined column {}", ref_code),
            };
            return Some(column_value);
        }
        return None;
    }

    fn check_and_add_text_field(
        columns: &Vec<Column>,
        ref_code: &str,
        str_data: String,
    ) -> Option<ColumnValue> {
        if str_data != "" {
            let find_col = columns.into_iter().find(|col| col.ref_code == ref_code);
            let column_value = match find_col {
                Some(value) => ColumnValue::new(
                    value.name.clone(),
                    value.ref_code.clone(),
                    ColumnValueType::Text(str_data),
                ),
                None => panic!("CSV has data undefined column {}", ref_code),
            };
            return Some(column_value);
        }
        return None;
    }

    fn check_and_add_cdf_field(
        columns: &Vec<Column>,
        ref_code: &str,
        str_data: String,
    ) -> Option<ColumnValue> {
        if str_data != "" {
            let find_col = columns.into_iter().find(|col| col.ref_code == ref_code);
            let column_value = match find_col {
                Some(value) => ColumnValue::new(
                    value.name.clone(),
                    value.ref_code.clone(),
                    ColumnValueType::Cdf(str_data),
                ),
                None => panic!("CSV has data undefined column {}", ref_code),
            };
            return Some(column_value);
        }
        return None;
    }
}
