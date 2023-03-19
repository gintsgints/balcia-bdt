use chrono::{DateTime, Utc};
use serde::{Serialize};

mod column_type;
mod column_value;

use self::column_value::RowValues;
use super::flat_bdt::en_date_format;

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

impl From<&super::flat_bdt::ColumnRow> for Column {
    fn from(row: &super::flat_bdt::ColumnRow) -> Self {
        Column {
            skip: row.skip.clone(),
            id: row.id,
            name: row.col_name.clone(),
            title: row.title.clone(),
            ref_code: row.ref_code.clone(),
            col_type: column_type::ColumnType::from(row),
            sequence: row.sequence,
            is_key: row.is_key.eq("Y"),
            options: row.options.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TableName {
    pub lang: Language,
    pub name: String,
    pub print_name: String,
    pub short_print_name: String,
}

impl TableName {
    fn new(
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

impl Bdt {
    fn new(ic: String, valid_from: Option<DateTime<Utc>>, valid_to: Option<DateTime<Utc>>) -> Bdt {
        Bdt {
            skip: None,
            ic,
            names: Vec::new(),
            valid_from,
            valid_to,
            columns: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl From<super::flat_bdt::TableRow> for Bdt {
    fn from(value: super::flat_bdt::TableRow) -> Self {
        Bdt::new(value.ic, value.valid_from, value.valid_to)
    }
}

#[derive(Debug, Serialize)]
pub struct BdtList {
    pub tables: Vec<Bdt>,
}

impl BdtList {
    fn new() -> BdtList {
        BdtList { tables: Vec::new() }
    }

    fn push(&mut self, value: Bdt) {
        self.tables.push(value)
    }

    #[allow(dead_code)]
    fn get(&mut self, index: usize) -> Option<&Bdt> {
        self.tables.get(index)
    }
}

impl From<super::flat_bdt::FlatBdt> for BdtList {
    fn from(flat_bdt: super::flat_bdt::FlatBdt) -> Self {
        let mut list = BdtList::new();
        for row in flat_bdt.tables.iter() {
            let mut bdt = Bdt::new(row.ic.clone(), row.valid_from, row.valid_to);
            let lv_name = TableName::new(
                Language::LV,
                row.name_lv.clone(),
                row.print_name_lv.clone(),
                row.short_print_name_lv.clone(),
            );
            bdt.names.push(lv_name);
            let en_name = TableName::new(
                Language::EN,
                row.name_en.clone(),
                row.print_name_en.clone(),
                row.short_print_name_en.clone(),
            );
            bdt.names.push(en_name);

            for row in flat_bdt.columns.iter() {
                if bdt.ic.eq(&row.table_type_id) {
                    bdt.columns.push(Column::from(row))
                }
            }

            for row in flat_bdt.data.iter() {
                if bdt.ic.eq(&row.table_type) {
                    bdt.data.push(RowValues::from((&bdt.columns, row)))
                }
            }

            list.push(bdt)
        }
        return list;
    }
}
