use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::csv_adapter::csv_model::ColumnRow;

#[derive(Debug, Serialize, Deserialize)]
pub enum ColumnType {
    Date,
    Text,
    Num,
    Cdf {
        codificator_id: String,
        select_params: String,
    },
}

impl ColumnType {
    pub fn from_data_row(row: &ColumnRow) -> ColumnType {
        let re_fromto = Regex::new(r"VALID_(FROM|TO)").unwrap();
        let re_num = Regex::new(r"NUM[[:digit:]]+").unwrap();
        let re_text = Regex::new(r"TEXT[[:digit:]]+").unwrap();
        let re_cdf = Regex::new(r"CDF[[:digit:]]+_ID").unwrap();

        match re_fromto.find(row.ref_code.as_str()) {
            Some(_) => ColumnType::Date,
            None => match re_num.find(row.ref_code.as_str()) {
                Some(_) => ColumnType::Num,
                None => match re_text.find(row.ref_code.as_str()) {
                    Some(_) => ColumnType::Text,
                    None => match re_cdf.find(row.ref_code.as_str()) {
                        Some(_) => ColumnType::Cdf {
                            codificator_id: row.adm_codificator_id.clone(),
                            select_params: row.select_params.clone(),
                        },
                        None => panic!("Wrong match for {}", row.ref_code.as_str()),
                    },
                },
            },
        }
    }
}
