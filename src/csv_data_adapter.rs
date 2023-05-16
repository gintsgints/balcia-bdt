use std::error::Error;

use csv::WriterBuilder;

use crate::bdt::{column_value::ColumnValueType, Bdt};

const FORMAT: &'static str = "%Y-%m-%d";

pub fn write_csv_data(path: &String, bdt: &Bdt) -> Result<(), Box<dyn Error>> {
    let data = make_data_rows(bdt);
    let mut wtr = WriterBuilder::new().from_path(path)?;
    for row in data {
        wtr.write_record(row)?;
    }
    Ok(())
}

fn make_data_rows(bdt: &Bdt) -> Vec<Vec<String>> {
    let csv_hader = make_header(&bdt);
    make_rows(bdt, csv_hader)
}

fn make_header(bdt: &Bdt) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for column in &bdt.columns {
        result.push(column.name.clone());
    }
    result
}

fn make_rows(bdt: &Bdt, header: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];
    result.push(header);
    for row in &bdt.data {
        let mut string_row: Vec<String> = vec![];
        for column in &bdt.columns {
            let mut str_value = "".to_string();
            for value in &row.values {
                if column.ref_code.eq(&value.ref_code) {
                    match &value.value {
                        ColumnValueType::Cdf(str_val) => str_value = str_val.clone(),
                        ColumnValueType::Text(str_val) => str_value = str_val.clone(),
                        ColumnValueType::Date(date_opt_val) => {
                            if let Some(date_val) = date_opt_val {
                                str_value = format!("{}", date_val.clone().format(FORMAT));
                            }
                        }
                        ColumnValueType::Num(num_opt_val) => {
                            if let Some(num_val) = num_opt_val {
                                str_value = num_val.clone().to_string()
                            }
                        }
                    }
                }
            }
            string_row.push(str_value);
        }
        result.push(string_row);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use chrono::NaiveDate;

    use crate::bdt::{
        column_type::ColumnType,
        column_value::{ColumnValue, ColumnValueType},
        table_name::TableNameList,
        Column, RowValues,
    };

    use super::*;

    fn create_test_data() -> Bdt {
        let col1 = Column {
            skip: "".to_string(),
            id: Some(1),
            name: "VALID_FROM".to_string(),
            title: "".to_string(),
            ref_code: "VALID_FROM".to_string(),
            col_type: ColumnType::Date,
            sequence: Some(1),
            is_key: false,
            options: "".to_string(),
        };
        let col2 = Column {
            skip: "".to_string(),
            id: Some(2),
            name: "VALID_TO".to_string(),
            title: "".to_string(),
            ref_code: "VALID_TO".to_string(),
            col_type: ColumnType::Date,
            sequence: Some(1),
            is_key: false,
            options: "".to_string(),
        };
        let col3 = Column {
            skip: "".to_string(),
            id: Some(3),
            name: "CONFIG_TYPE".to_string(),
            title: "".to_string(),
            ref_code: "CDF1_ID".to_string(),
            col_type: ColumnType::Cdf {
                codificator_id: "TT_CONFIG_TYPE_ID".to_string(),
                select_params: "".to_string(),
            },
            sequence: Some(1),
            is_key: false,
            options: "".to_string(),
        };
        let col4 = Column {
            skip: "".to_string(),
            id: Some(4),
            name: "ADDITIONAL_COL".to_string(),
            title: "".to_string(),
            ref_code: "CDF2_ID".to_string(),
            col_type: ColumnType::Cdf {
                codificator_id: "ADDITIONAL_ID".to_string(),
                select_params: "".to_string(),
            },
            sequence: Some(1),
            is_key: false,
            options: "".to_string(),
        };
        let col5 = Column {
            skip: "".to_string(),
            id: Some(5),
            name: "CONFIG_VALUE".to_string(),
            title: "".to_string(),
            ref_code: "TEXT1".to_string(),
            col_type: ColumnType::Text,
            sequence: Some(1),
            is_key: false,
            options: "".to_string(),
        };
        let col6 = Column {
            skip: "".to_string(),
            id: Some(6),
            name: "CONFIG_NUM_VALUE".to_string(),
            title: "".to_string(),
            ref_code: "CONFIG_NUM_VALUE".to_string(),
            col_type: ColumnType::Num,
            sequence: Some(1),
            is_key: false,
            options: "".to_string(),
        };

        let mut row1 = RowValues::new();
        row1.push(ColumnValue {
            value: ColumnValueType::Date(NaiveDate::from_ymd_opt(1997, 12, 1)),
            name: "VALID_FROM".to_string(),
            ref_code: "VALID_FROM".to_string(),
        });
        row1.push(ColumnValue {
            value: ColumnValueType::Date(None),
            name: "VALID_TO".to_string(),
            ref_code: "VALID_TO".to_string(),
        });
        row1.push(ColumnValue {
            value: ColumnValueType::Cdf("BORDER_POLICY_SERIES".to_string()),
            name: "CONFIG_TYPE".to_string(),
            ref_code: "CDF1_ID".to_string(),
        });
        row1.push(ColumnValue {
            value: ColumnValueType::Text("ANA".to_string()),
            name: "CONFIG_VALUE".to_string(),
            ref_code: "TEXT1".to_string(),
        });
        row1.push(ColumnValue {
            value: ColumnValueType::Num(Some(10.0)),
            name: "CONFIG_NUM_VALUE".to_string(),
            ref_code: "NUM1".to_string(),
        });

        Bdt {
            skip: "".to_string(),
            ic: "".to_string(),
            names: TableNameList::new(vec![]),
            valid_from: None,
            valid_to: None,
            columns: vec![col1, col2, col3, col4, col5, col6],
            data: vec![row1],
        }
    }

    #[test]
    fn make_header_test() {
        let bdt = create_test_data();
        let csv_hader = make_header(&bdt);
        assert_eq!(csv_hader.get(0).unwrap(), &"VALID_FROM".to_string());
        assert_eq!(csv_hader.get(1).unwrap(), &"VALID_TO".to_string());
    }

    #[test]
    fn make_row_test() {
        let bdt = create_test_data();
        let csv_row = make_rows(&bdt, vec![]);
        assert_eq!(
            csv_row.get(1).unwrap().get(0).unwrap(),
            &"01.12.1997".to_string()
        );
        assert_eq!(csv_row.get(1).unwrap().len(), 6);
    }
}
