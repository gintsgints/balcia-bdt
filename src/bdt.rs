use chrono::{DateTime, Utc};
use serde::{Serialize};

pub mod column_type;
pub mod column_value;

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
    pub skip: String,
    pub id: Option<u64>,
    pub name: String,
    pub title: String,
    pub ref_code: String,
    pub col_type: column_type::ColumnType,
    pub sequence: Option<u16>,
    pub is_key: bool,
    pub options: String,
}

#[derive(Debug, Serialize)]
pub struct TableName {
    pub lang: Language,
    pub name: String,
    pub print_name: String,
    pub short_print_name: String,
}

impl TableName {
    pub fn new(
        lang: Language,
        name: String,
        print_name: String,
        short_print_name: String,
    ) -> TableName {
        TableName {
            lang,
            name,
            print_name,
            short_print_name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Bdt {
    pub skip: String,
    pub ic: String,
    pub names: Vec<TableName>,
    #[serde(with = "en_date_format")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(with = "en_date_format")]
    pub valid_to: Option<DateTime<Utc>>,
    pub columns: Vec<Column>,
    pub data: Vec<RowValues>,
}
