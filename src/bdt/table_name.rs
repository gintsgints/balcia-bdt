use std::str::FromStr;

use serde::{Deserialize, Serialize};

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
pub struct Name {
    pub lang: Language,
    pub name: String,
    pub print_name: String,
    pub short_print_name: String,
}

impl Name {
    pub fn new(
        lang: Language,
        name: String,
        print_name: String,
        short_print_name: String,
    ) -> Name {
        Name {
            lang,
            name,
            print_name,
            short_print_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameList {
    names: Vec<Name>,
}

impl NameList {
    pub fn new(names: Vec<Name>) -> Self {
        NameList { names }
    }

    pub fn push(&mut self, name: Name) {
        self.names.push(name)
    }

    pub fn get_table_name(&self, lang: Language) -> Option<Name> {
        self.names
            .clone()
            .into_iter()
            .find(|foundname| foundname.lang == lang)
    }
}
