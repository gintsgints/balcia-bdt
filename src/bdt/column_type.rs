use regex::Regex;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Serialize)]
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
    pub fn from_data_row(input: &str, codificator_id: String, select_params: String) -> ColumnType {
        let re_fromto = Regex::new(r"VALID_(FROM|TO)").unwrap();
        let re_num = Regex::new(r"NUM[[:digit:]]+").unwrap();
        let re_text = Regex::new(r"TEXT[[:digit:]]+").unwrap();
        let re_cdf = Regex::new(r"CDF[[:digit:]]+_ID").unwrap();

        match re_fromto.find(input) {
            Some(_) => ColumnType::Date,
            None => match re_num.find(input) {
                Some(_) => ColumnType::Num,
                None => match re_text.find(input) {
                    Some(_) => ColumnType::Text,
                    None => match re_cdf.find(input) {
                        Some(_) => ColumnType::Cdf {
                            codificator_id,
                            select_params,
                        },
                        None => panic!("Wrong match for {}", input),
                    },
                },
            },
        }
    }
}
