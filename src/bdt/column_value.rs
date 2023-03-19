use chrono::{DateTime, Utc};
use serde::{Serialize};

use super::super::bdt::Column;
use super::super::flat_bdt::DataRow;
use super::super::flat_bdt::en_date_format;

#[derive(Debug, Serialize)]
pub enum ColumnValueType {
    #[serde(with = "en_date_format")]
    Date(Option<DateTime<Utc>>),
    Cdf(String),
    Num(Option<f64>),
    Text(String),
}

#[derive(Debug, Serialize)]
pub struct ColumnValue {
    pub name: String,
    pub value: ColumnValueType,
}

impl ColumnValue {
    fn new(name: String, value: ColumnValueType) -> ColumnValue {
        ColumnValue { name, value }
    }
}

#[derive(Debug, Serialize)]
pub struct RowValues {
    values: Vec<ColumnValue>,
}

impl RowValues {
    fn new() -> RowValues {
        RowValues { values: Vec::new() }
    }

    fn push(&mut self, value: ColumnValue) {
        self.values.push(value)
    }

    #[allow(dead_code)]
    fn get(&mut self, index: usize) -> Option<&ColumnValue> {
        self.values.get(index)
    }
}

impl From<(&Vec<Column>, &DataRow)> for RowValues {
    fn from(value: (&Vec<Column>, &DataRow)) -> RowValues {
        let mut values = RowValues::new();
        for col in value.0.iter() {
            let column_value:ColumnValue = match col.ref_code.as_str() {
                "VALID_FROM" => { ColumnValue::new(col.name.clone(), ColumnValueType::Date(value.1.valid_from)) }
                "VALID_TO" => { ColumnValue::new(col.name.clone(), ColumnValueType::Date(value.1.valid_to)) }
                "NUM1" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num1)) }
                "NUM2" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num2)) }
                "NUM3" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num3)) }
                "NUM4" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num4)) }
                "NUM5" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num5)) }
                "NUM6" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num6)) }
                "NUM7" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num7)) }
                "NUM8" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num8)) }
                "NUM9" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num9)) }
                "NUM10" => { ColumnValue::new(col.name.clone(), ColumnValueType::Num(value.1.num10)) }
                "TEXT1" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(value.1.text1.clone())) }
                "TEXT2" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(value.1.text2.clone())) }
                "TEXT3" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(value.1.text3.clone())) }
                "TEXT4" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(value.1.text4.clone())) }
                "TEXT5" => { ColumnValue::new(col.name.clone(), ColumnValueType::Text(value.1.text5.clone())) }
                "CDF1_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf1.clone())) }
                "CDF2_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf2.clone())) }
                "CDF3_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf3.clone())) }
                "CDF4_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf4.clone())) }
                "CDF5_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf5.clone())) }
                "CDF6_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf6.clone())) }
                "CDF7_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf7.clone())) }
                "CDF8_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf8.clone())) }
                "CDF9_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf9.clone())) }
                "CDF10_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf10.clone())) }
                "CDF11_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf11.clone())) }
                "CDF12_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf12.clone())) }
                "CDF13_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf13.clone())) }
                "CDF14_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf14.clone())) }
                "CDF15_ID" => { ColumnValue::new(col.name.clone(), ColumnValueType::Cdf(value.1.cdf15.clone())) }
                &_ => todo!()
            };
            values.push(column_value);
        }
        return values;
    }
}
