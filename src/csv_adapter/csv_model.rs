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

impl From<&Bdt> for TableRow {
    fn from(bdtrec: &Bdt) -> Self {
        TableRow {
            skip: bdtrec.skip.clone(),
            id: None,
            adm_codificator_ic: "AGC_CUSTOM_TABLE_TYPE".to_string(),
            parent_ic: "".to_string(),
            ic: bdtrec.ic.clone(),
            notes: "".to_string(),
            code: "".to_string(),
            valid_from: bdtrec.valid_from,
            valid_to: bdtrec.valid_to,
            sequence: None,
            name_lv: bdtrec
                .names
                .get_table_name(table_name::Language::LV)
                .unwrap_or_default()
                .name
                .clone(),
            print_name_lv: bdtrec
                .names
                .get_table_name(table_name::Language::LV)
                .unwrap_or_default()
                .print_name,
            short_print_name_lv: bdtrec
                .names
                .get_table_name(table_name::Language::LV)
                .unwrap_or_default()
                .short_print_name,
            name_en: bdtrec
                .names
                .get_table_name(table_name::Language::EN)
                .unwrap_or_default()
                .name,
            print_name_en: bdtrec
                .names
                .get_table_name(table_name::Language::EN)
                .unwrap_or_default()
                .print_name,
            short_print_name_en: bdtrec
                .names
                .get_table_name(table_name::Language::EN)
                .unwrap_or_default()
                .short_print_name,
            agc_tariff_plan_id: None,
        }
    }
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
    pub fn to_column(&self) -> Column {
        Column {
            skip: self.skip.clone(),
            id: self.id,
            name: self.col_name.clone(),
            title: self.title.clone(),
            ref_code: self.ref_code.clone(),
            col_type: ColumnType::from(self),
            sequence: self.sequence,
            is_key: self.is_key.eq("Y"),
            options: self.options.clone(),
        }
    }
}

impl From<(&Column, String, u16)> for ColumnRow {
    fn from(value: (&Column, String, u16)) -> Self {
        let is_key = if value.0.is_key {
            "Y".to_string()
        } else {
            "N".to_string()
        };
        let (adm_codificator_id, select_params) = match &value.0.col_type {
            ColumnType::Cdf {
                codificator_id,
                select_params,
            } => (codificator_id.clone(), select_params.clone()),
            _ => ("".to_string(), "".to_string()),
        };
        ColumnRow {
            skip: value.0.skip.clone(),
            id: value.0.id,
            table_type_id: value.1.clone(),
            title: value.0.title.clone(),
            col_name: value.0.name.clone(),
            ref_code: value.0.ref_code.clone(),
            adm_codificator_id,
            sequence: Some(value.2 + 1),
            is_key,
            options: value.0.options.clone(),
            select_params,
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

impl From<(&RowValues, String, u16)> for DataRow {
    fn from(value: (&RowValues, String, u16)) -> Self {
        DataRow {
            skip: "".to_string(),
            id: None,
            table_type: value.1.clone(),
            valid_from: value.0.get_by_ref_as_date("VALID_FROM"),
            valid_to: value.0.get_by_ref_as_date("VALID_TO"),
            cdf1: value.0.get_by_ref_as_cdf("CDF1"),
            cdf2: value.0.get_by_ref_as_cdf("CDF2"),
            cdf3: value.0.get_by_ref_as_cdf("CDF3"),
            cdf4: value.0.get_by_ref_as_cdf("CDF4"),
            cdf5: value.0.get_by_ref_as_cdf("CDF5"),
            cdf6: value.0.get_by_ref_as_cdf("CDF6"),
            cdf7: value.0.get_by_ref_as_cdf("CDF7"),
            cdf8: value.0.get_by_ref_as_cdf("CDF8"),
            cdf9: value.0.get_by_ref_as_cdf("CDF9"),
            cdf10: value.0.get_by_ref_as_cdf("CDF10"),
            cdf11: value.0.get_by_ref_as_cdf("CDF11"),
            cdf12: value.0.get_by_ref_as_cdf("CDF12"),
            cdf13: value.0.get_by_ref_as_cdf("CDF13"),
            cdf14: value.0.get_by_ref_as_cdf("CDF14"),
            cdf15: value.0.get_by_ref_as_cdf("CDF15"),
            num1: value.0.get_by_ref_as_num("NUM1"),
            num2: value.0.get_by_ref_as_num("NUM2"),
            num3: value.0.get_by_ref_as_num("NUM3"),
            num4: value.0.get_by_ref_as_num("NUM4"),
            num5: value.0.get_by_ref_as_num("NUM5"),
            num6: value.0.get_by_ref_as_num("NUM6"),
            num7: value.0.get_by_ref_as_num("NUM7"),
            num8: value.0.get_by_ref_as_num("NUM8"),
            num9: value.0.get_by_ref_as_num("NUM9"),
            num10: value.0.get_by_ref_as_num("NUM10"),
            text1: value.0.get_by_ref_as_text("TEXT1"),
            text2: value.0.get_by_ref_as_text("TEXT2"),
            text3: value.0.get_by_ref_as_text("TEXT3"),
            text4: value.0.get_by_ref_as_text("TEXT4"),
            text5: value.0.get_by_ref_as_text("TEXT5"),
        }
    }
}
