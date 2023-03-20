use chrono::{DateTime, Utc};
use serde::{Serialize};

use super::super::en_date_format;

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
    pub fn new(name: String, value: ColumnValueType) -> ColumnValue {
        ColumnValue { name, value }
    }
}

#[derive(Debug, Serialize)]
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
    pub fn get(&mut self, index: usize) -> Option<&ColumnValue> {
        self.values.get(index)
    }
}
