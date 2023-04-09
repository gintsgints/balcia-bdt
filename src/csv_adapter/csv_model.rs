use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::bdt::column_type::ColumnType;
use crate::bdt::*;
use crate::format::lv_date_format;



#[derive(Debug, Deserialize, Serialize)]
pub struct TableRow {
    pub skip: String,
    pub id: Option<u64>,
    pub adm_codificator_ic: String,
    pub parent_ic: String,
    pub ic: String,
    pub notes: String,
    pub code: String,
    #[serde(with = "lv_date_format")]
    pub valid_from: Option<NaiveDate>,
    #[serde(with = "lv_date_format")]
    pub valid_to: Option<NaiveDate>,
    pub sequence: Option<u16>,
    pub name_lv: String,
    pub print_name_lv: String,
    pub short_print_name_lv: String,
    pub name_en: String,
    pub print_name_en: String,
    pub short_print_name_en: String,
    pub agc_tariff_plan_id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColumnRow {
    pub skip: String,
    pub id: Option<u64>,
    pub table_type_id: String,
    pub title: String,
    pub col_name: String,
    pub ref_code: String,
    pub adm_codificator_id: String,
    pub sequence: Option<u16>,
    pub is_key: String,
    pub options: String,
    pub select_params: String,
}

impl ColumnRow {
    pub fn to_column_type(&self) -> ColumnType {
        ColumnType::from_data_row(
            self.ref_code.as_str(),
            self.adm_codificator_id.clone(),
            self.select_params.clone(),
        )
    }

    pub fn to_column(&self) -> Column {
        Column {
            skip: self.skip.clone(),
            id: self.id,
            name: self.col_name.clone(),
            title: self.title.clone(),
            ref_code: self.ref_code.clone(),
            col_type: self.to_column_type(),
            sequence: self.sequence,
            is_key: self.is_key.eq("Y"),
            options: self.options.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataRow {
    pub skip: String,
    pub id: Option<u64>,
    pub table_type: String,
    #[serde(with = "lv_date_format")]
    pub valid_from: Option<NaiveDate>,
    #[serde(with = "lv_date_format")]
    pub valid_to: Option<NaiveDate>,
    pub cdf1: String,
    pub cdf2: String,
    pub cdf3: String,
    pub cdf4: String,
    pub cdf5: String,
    pub cdf6: String,
    pub cdf7: String,
    pub cdf8: String,
    pub cdf9: String,
    pub cdf10: String,
    pub cdf11: String,
    pub cdf12: String,
    pub cdf13: String,
    pub cdf14: String,
    pub cdf15: String,
    pub num1: Option<f64>,
    pub num2: Option<f64>,
    pub num3: Option<f64>,
    pub num4: Option<f64>,
    pub num5: Option<f64>,
    pub num6: Option<f64>,
    pub num7: Option<f64>,
    pub num8: Option<f64>,
    pub num9: Option<f64>,
    pub num10: Option<f64>,
    pub text1: String,
    pub text2: String,
    pub text3: String,
    pub text4: String,
    pub text5: String,
}
