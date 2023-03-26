use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::super::en_date_format;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ColumnValueType {
    #[serde(with = "en_date_format")]
    Date(Option<DateTime<Utc>>),
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

    pub fn len(&mut self) -> usize {
        self.values.len()
    }

    #[allow(dead_code)]
    pub fn get(&mut self, index: usize) -> Option<&ColumnValue> {
        self.values.get(index)
    }
}
