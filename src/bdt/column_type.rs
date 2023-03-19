use regex::Regex;
use serde::{Serialize};

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

impl From<&super::super::flat_bdt::ColumnRow> for ColumnType {
    fn from(value: &super::super::flat_bdt::ColumnRow) -> Self {
        let re_fromto = Regex::new(r"VALID_(FROM|TO)").unwrap();
        let re_num = Regex::new(r"NUM[[:digit:]]+").unwrap();
        let re_text = Regex::new(r"TEXT[[:digit:]]+").unwrap();
        let re_cdf = Regex::new(r"CDF[[:digit:]]+_ID").unwrap();

        match re_fromto.find(value.ref_code.as_str()) {
            Some(_) => ColumnType::Date,
            None => match re_num.find(value.ref_code.as_str()) {
                Some(_) => ColumnType::Num,
                None => match re_text.find(value.ref_code.as_str()) {
                    Some(_) => ColumnType::Text,
                    None => match re_cdf.find(value.ref_code.as_str()) {
                        Some(_) => ColumnType::Cdf {
                            codificator_id: value.adm_codificator_id.clone(),
                            select_params: value.select_params.clone(),
                        },
                        None => panic!("Wrong match for {}", value.ref_code.as_str())
                    },
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_colum_type_from_string_valid_from() {
        let row = super::super::super::flat_bdt::ColumnRow {
            skip: String::from(""),
            id: None,
            table_type_id: String::from("AL01_SIMILAR_AGREEMENTS_CRITERIA"),
            title: String::from("Valid from"),
            col_name: String::from("VALID_FROM"),
            ref_code: String::from("VALID_FROM"),
            adm_codificator_id: String::from(""),
            sequence: Some(1),
            is_key: String::from("Y"),
            options: String::from(""),
            select_params: String::from(""),
        };
        let result = ColumnType::from(&row);
        assert!(matches!(result, ColumnType::Date));
    }

    #[test]
    fn create_colum_type_from_string_valid_to() {
        let row = super::super::super::flat_bdt::ColumnRow {
            skip: String::from(""),
            id: None,
            table_type_id: String::from("AL01_SIMILAR_AGREEMENTS_CRITERIA"),
            title: String::from("Valid to"),
            col_name: String::from("VALID_TO"),
            ref_code: String::from("VALID_TO"),
            adm_codificator_id: String::from(""),
            sequence: Some(2),
            is_key: String::from("Y"),
            options: String::from(""),
            select_params: String::from(""),
        };
        let result = ColumnType::from(&row);
        assert!(matches!(result, ColumnType::Date));
    }

    #[test]
    fn create_colum_type_from_string_num1() {
        let row = super::super::super::flat_bdt::ColumnRow {
            skip: String::from("skip"),
            id: None,
            table_type_id: String::from("AL01_BASE_PREMIUM_FACTOR"),
            title: String::from("Factor"),
            col_name: String::from("FACTOR"),
            ref_code: String::from("NUM1"),
            adm_codificator_id: String::from(""),
            sequence: Some(3),
            is_key: String::from("N"),
            options: String::from(""),
            select_params: String::from(""),
        };
        let result = ColumnType::from(&row);
        assert!(matches!(result, ColumnType::Num));
    }

    #[test]
    fn create_colum_type_from_string_num2() {
        let row = super::super::super::flat_bdt::ColumnRow {
            skip: String::from("skip"),
            id: None,
            table_type_id: String::from("AL01_BASE_PREMIUM_FACTOR"),
            title: String::from("Factor"),
            col_name: String::from("FACTOR"),
            ref_code: String::from("NUM2"),
            adm_codificator_id: String::from(""),
            sequence: Some(3),
            is_key: String::from("N"),
            options: String::from(""),
            select_params: String::from(""),
        };
        let result = ColumnType::from(&row);
        assert!(matches!(result, ColumnType::Num));
    }

    #[test]
    fn create_colum_type_from_string_cdf1_id() {
        let row = super::super::super::flat_bdt::ColumnRow {
            skip: String::from(""),
            id: None,
            table_type_id: String::from("AL01_SPORT_TYPE_FACTOR"),
            title: String::from("Risk type"),
            col_name: String::from("RISK_TYPE"),
            ref_code: String::from("CDF1_ID"),
            adm_codificator_id: String::from("RISK_TYPE"),
            sequence: Some(4),
            is_key: String::from("Y"),
            options: String::from(""),
            select_params: String::from("icprefix=AL01_R"),
        };
        let result = ColumnType::from(&row);
        match result {
            ColumnType::Cdf { codificator_id, select_params } => {
                assert_eq!(codificator_id, String::from("RISK_TYPE"));
                assert_eq!(select_params, String::from("icprefix=AL01_R"));
            },
            _ => {assert!(false)}
        }
    }
}
