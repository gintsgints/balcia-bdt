use serde::{Deserialize, Serialize};
use std::{str::FromStr};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Language {
    #[default]
    EN,
    LV,
    PL,
    LT,
}

#[allow(dead_code)]

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "EN" => Ok(Language::EN),
            "LV" => Ok(Language::LV),
            "PL" => Ok(Language::PL),
            "LT" => Ok(Language::LT),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TableNameList {
    names: Vec<TableName>,
}

impl TableNameList {
    pub fn new(names: Vec<TableName>) -> Self {
        TableNameList { names }
    }

    pub fn push(&mut self, name: TableName) {
        self.names.push(name)
    }

    pub fn get_table_name(&self, lang: Language) -> Option<TableName> {
        self.names.clone()
            .into_iter()
            .find(|foundname| foundname.lang == lang)
    }
}
