use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::format::lv_date_format;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifierRecord {
    pub adm_codificator_id: String,
    pub parent_ic: String,
    pub ic: String,
    pub code: String,
    #[serde(with = "lv_date_format")]
    pub valid_from: Option<NaiveDate>,
    #[serde(with = "lv_date_format")]
    pub valid_to: Option<NaiveDate>,
    pub sequence: Option<u16>,
    pub notes: String,
}
