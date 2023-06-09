use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::bdt::Column;
use crate::csv_adapter::csv_model::DataRow;
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
    pub values: Vec<ColumnValue>,
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

    pub fn get_by_ref_as_date(&self, column_ref: &str) -> Option<NaiveDate> {
        let value = self
            .values
            .iter()
            .find(|column_value| column_value.ref_code == column_ref);
        let mut date_value = None;
        if let Some(column_value) = value {
            if let ColumnValueType::Date(date) = column_value.value {
                date_value = date;
            }
        }
        date_value
    }

    pub fn get_by_ref_as_num(&self, column_ref: &str) -> Option<f64> {
        let value = self
            .values
            .iter()
            .find(|column_value| column_value.ref_code == column_ref);
        let mut num_value = None;
        if let Some(column_value) = value {
            if let ColumnValueType::Num(num) = column_value.value {
                num_value = num;
            }
        }
        num_value
    }

    pub fn get_by_ref_as_text(&self, column_ref: &str) -> String {
        let value = self
            .values
            .iter()
            .find(|column_value| column_value.ref_code == column_ref);
        let mut text_value = "".to_string();
        if let Some(column_value) = value {
            if let ColumnValueType::Text(text) = column_value.value.clone() {
                text_value = text;
            }
        }
        text_value
    }

    pub fn get_by_ref_as_cdf(&self, column_ref: &str) -> String {
        let value = self
            .values
            .iter()
            .find(|column_value| column_value.ref_code == column_ref);
        let mut cdf_value = "".to_string();
        if let Some(column_value) = value {
            if let ColumnValueType::Cdf(text) = column_value.value.clone() {
                cdf_value = text;
            }
        }
        cdf_value
    }

    pub fn from_data_row(columns: &[Column], row: &DataRow) -> RowValues {
        let mut values = RowValues::new();
        if let Some(column_value) =
            RowValues::check_and_add_date_field(columns, "VALID_FROM", row.valid_from)
        {
            values.push(column_value)
        };
        if let Some(column_value) =
            RowValues::check_and_add_date_field(columns, "VALID_TO", row.valid_to)
        {
            values.push(column_value)
        };
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM1", row.num1) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM2", row.num2) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM3", row.num3) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM4", row.num4) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM5", row.num5) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM6", row.num6) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM7", row.num7) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM8", row.num8) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM9", row.num9) {
            values.push(column_value)
        }
        if let Some(column_value) = RowValues::check_and_add_num_field(columns, "NUM10", row.num10)
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_text_field(columns, "TEXT1", row.text1.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_text_field(columns, "TEXT2", row.text2.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_text_field(columns, "TEXT3", row.text3.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_text_field(columns, "TEXT4", row.text4.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_text_field(columns, "TEXT5", row.text5.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF1_ID", row.cdf1.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF2_ID", row.cdf2.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF3_ID", row.cdf3.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF4_ID", row.cdf4.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF5_ID", row.cdf5.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF6_ID", row.cdf6.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF7_ID", row.cdf7.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF8_ID", row.cdf8.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF9_ID", row.cdf9.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF10_ID", row.cdf10.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF11_ID", row.cdf11.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF12_ID", row.cdf12.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF13_ID", row.cdf13.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF14_ID", row.cdf14.clone())
        {
            values.push(column_value)
        }
        if let Some(column_value) =
            RowValues::check_and_add_cdf_field(columns, "CDF15_ID", row.cdf15.clone())
        {
            values.push(column_value)
        }
        values
    }

    fn check_and_add_date_field(
        columns: &[Column],
        ref_code: &str,
        date: Option<NaiveDate>,
    ) -> Option<ColumnValue> {
        if date.is_some() {
            let find_col = columns.iter().find(|col| col.ref_code == ref_code);
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
        None
    }

    fn check_and_add_num_field(
        columns: &[Column],
        ref_code: &str,
        num: Option<f64>,
    ) -> Option<ColumnValue> {
        if num.is_some() {
            let find_col = columns.iter().find(|col| col.ref_code == ref_code);
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
        None
    }

    fn check_and_add_text_field(
        columns: &[Column],
        ref_code: &str,
        str_data: String,
    ) -> Option<ColumnValue> {
        if !str_data.is_empty() {
            let find_col = columns.iter().find(|col| col.ref_code == ref_code);
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
        None
    }

    fn check_and_add_cdf_field(
        columns: &[Column],
        ref_code: &str,
        str_data: String,
    ) -> Option<ColumnValue> {
        if !str_data.is_empty() {
            let find_col = columns.iter().find(|col| col.ref_code == ref_code);
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
        None
    }
}
