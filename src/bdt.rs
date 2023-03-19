use chrono::{DateTime, Utc};
use serde::{Serialize};

mod column_type;
mod column_value;

use crate::en_date_format;

use column_value::RowValues;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum Language {
    EN,
    LV,
    PL,
    LT,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Column {
    skip: String,
    id: Option<u64>,
    name: String,
    title: String,
    ref_code: String,
    col_type: column_type::ColumnType,
    sequence: Option<u16>,
    is_key: bool,
    options: String,
}

#[derive(Debug, Serialize)]
pub struct TableName {
    pub lang: Language,
    pub name: String,
    pub print_name: String,
    pub short_print_name: String,
}

#[derive(Debug, Serialize)]
pub struct Bdt {
    pub skip: Option<String>,
    pub ic: String,
    pub names: Vec<TableName>,
    #[serde(with = "en_date_format")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(with = "en_date_format")]
    pub valid_to: Option<DateTime<Utc>>,
    pub columns: Vec<Column>,
    pub data: Vec<RowValues>,
}

