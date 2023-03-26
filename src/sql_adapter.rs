use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use serde::Serialize;
use std::error::Error;
use std::fs::File;

use crate::Bdt;

fn data_field_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let field_value = h
        .param(0)
        .and_then(|ref v| v.value().as_object())
        .ok_or(RenderError::new(
            "Param 0 with field value object is required for data field helper.",
        ))?;
    let field_ref = h
        .param(1)
        .and_then(|ref v| v.value().as_str())
        .ok_or(RenderError::new(
            "Param 1 with field ref string is required for data field helper.",
        ))?;
    let tst = field_value.get("name").unwrap().as_str().unwrap();
    Ok(())
}

fn yn_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .and_then(|ref v| v.value().as_bool())
        .ok_or(RenderError::new(
            "Param 0 with bool type is required for yn helper.",
        ))?;
    if param {
        write!(out, "Y")?;
    } else {
        write!(out, "N")?;
    }
    Ok(())
}

#[derive(Debug, Serialize)]
struct BdtList {
    tables: Vec<Bdt>,
}

pub fn write_bdt(tables: Vec<Bdt>) -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("yn", Box::new(yn_helper));
    handlebars
        .register_template_file("template", "./render/bdtlist.hbs")
        .unwrap();
    let mut output_file = File::create("data/TT/R__1211_load_agr_table.TT.sql")?;
    let bdtlist = BdtList {
        tables: tables.into_iter().filter(|bdt| bdt.skip == "").collect(),
    };
    handlebars.render_to_write("template", &bdtlist, &mut output_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use chrono::offset::Utc;


    use super::*;
    use crate::bdt::column_value::ColumnValue;
    use crate::bdt::column_value::ColumnValueType;

    fn setup(source: &str) -> Handlebars {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("testing", source)
            .unwrap();
        return handlebars;
    }

    #[test]
    fn skip_when_skipdata_test() {
        let source = r#"{{#if (eq skip "skipData")}}skip{{/if}}"#;
        let handlebars = setup(source);
        let mut data = HashMap::new();
        data.insert("skip", "skipData");
        assert_eq!(handlebars.render("testing", &data).unwrap(), "skip");
    }

    #[test]
    fn skip_when_not_skipdata_test() {
        let source = r#"{{#if (eq skip "skipData")}}skip{{/if}}"#;
        let handlebars = setup(source);
        let mut data = HashMap::new();
        data.insert("skip", "other");
        assert_eq!(handlebars.render("testing", &data).unwrap(), "");
    }

    #[test]
    fn yn_helper_test() {
        let source = "{{yn is_key ~}}";
        let mut handlebars = setup(source);
        handlebars.register_helper("yn", Box::new(yn_helper));

        let mut is_key_data: HashMap<&str, bool> = HashMap::new();
        is_key_data.insert("is_key", true);
        assert_eq!(handlebars.render("testing", &is_key_data).unwrap(), "Y");
        let mut no_key_data: HashMap<&str, bool> = HashMap::new();
        no_key_data.insert("is_key", false);
        assert_eq!(handlebars.render("testing", &no_key_data).unwrap(), "N");
    }

    #[test]
    fn data_field_helper_test() {
        let source = r#"{{df data "VALID_FROM" ~}}"#;
        let mut handlebars = setup(source);
        handlebars.register_helper("df", Box::new(data_field_helper));

        let mut column_data: HashMap<&str, Vec<ColumnValue>> = HashMap::new();
        let col1 = ColumnValue::new("Valid from".to_string(), "VALID_FROM".to_string(), ColumnValueType::Date(Some(Utc::now())));
        let col2 = ColumnValue::new("Long name".to_string(), "NUM1".to_string(), ColumnValueType::Num(Some(354.)));
        let mut data: Vec<ColumnValue> = Vec::new();
        data.push(col1);
        data.push(col2);
        column_data.insert("data", data);
        // assert_eq!(handlebars.render("testing", &column_data).unwrap(), "N");
    }
}
