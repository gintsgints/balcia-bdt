use chrono::NaiveDate;
pub use column_value::RowValues;
use serde::{Deserialize, Serialize};

use crate::bdt::table_name::TableNameList;
use crate::format::lv_date_format;

pub mod column_type;
pub mod column_value;
pub mod table_name;

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Bdt {
    pub skip: String,
    pub ic: String,
    pub names: TableNameList,
    #[serde(with = "lv_date_format")]
    pub valid_from: Option<NaiveDate>,
    #[serde(with = "lv_date_format")]
    pub valid_to: Option<NaiveDate>,
    pub columns: Vec<Column>,
    pub data: Vec<RowValues>,
}
