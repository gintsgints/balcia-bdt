use std::{error::Error, fs::File};
use std::io::{BufWriter};
use serde::Serialize;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

use crate::Bdt;

pub struct SqliteAdapter {}

#[derive(Debug, Serialize)]
struct BdtList {
    tables: Vec<Bdt>,
}

// handlebars_helper!(last: |array: , index: usize| if array.len() == index { ",".to_string() } else { "".to_string()});

pub fn array_last_comma(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let length = h
        .param(0)
        .as_ref()
        .and_then(|v| v.value().as_array())
        .map(|arr| arr.len())
        .ok_or(RenderError::new(
            "Param 0 with 'array' type is required for array_last_comma helper",
        ))?;

    let index = h
        .param(1)
        .as_ref()
        .and_then(|v| v.value().as_u64())
        .ok_or(RenderError::new(
            "Param 1 with 'usize' type is required for array_last_comma helper",
        ))?;

    if length as u64 != index + 1 {
        write!(out, ",")?;
    } else {
        write!(out, "")?;
    }

    Ok(())
}

fn some_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0);
    match param {
        Some(value) => {
            if value.is_value_missing() {
                write!(out, "")?;
            } else {
                write!(out, "TEXT")?;
            }
        }
        _ => {
            write!(out, "")?;
        }
    }
    Ok(())
}

impl SqliteAdapter {
    pub fn write_bdt(tables: Vec<Bdt>, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("some", Box::new(some_helper));
        handlebars.register_helper("last", Box::new(array_last_comma));
        handlebars
            .register_template_file("template", "./render/sqlite.hbs")
            .unwrap();
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        let bdtlist = BdtList {
            tables: tables
                .into_iter()
                .filter(|bdt| !"skip".eq(bdt.skip.as_str()))
                .collect(),
        };
        handlebars.render_to_write("template", &bdtlist, &mut writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn setup(source: &str) -> Handlebars {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("testing", source)
            .unwrap();
        return handlebars;
    }

    #[test]
    fn give_text_when_value() {
        let source = r#"{{some isvalue ~}}"#;
        let mut handlebars = setup(source);
        handlebars.register_helper("some", Box::new(some_helper));
        let mut data = HashMap::new();
        data.insert("isvalue", "other");
        assert_eq!(handlebars.render("testing", &data).unwrap(), "TEXT");
    }

    #[test]
    fn give_no_text_when_value() {
        let source = r#"{{some isvalue ~}}"#;
        let mut handlebars = setup(source);
        handlebars.register_helper("some", Box::new(some_helper));
        let mut data = HashMap::new();
        data.insert("noValue", "other");
        assert_eq!(handlebars.render("testing", &data).unwrap(), "");
    }

}