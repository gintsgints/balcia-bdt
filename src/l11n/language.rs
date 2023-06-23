use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Language {
    #[default]
    EN,
    LV,
    PL,
    LT,
}

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
