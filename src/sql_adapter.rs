use std::error::Error;
use std::io::stdout;

use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use serde::Serialize;

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
        .and_then(|ref v| v.value().as_array())
        .ok_or(RenderError::new(
            "Param 0 with field value object is required for data field helper.",
        ))?;
    let field_ref = h
        .param(1)
        .and_then(|ref v| v.value().as_str())
        .ok_or(RenderError::new(
            "Param 1 with field ref string is required for data field helper.",
        ))?;

    for value in field_value.iter() {
        let test_code = value["ref_code"].as_str();
        match test_code {
            Some(v) => {
                if v == field_ref {
                    let print_value = value["value"].as_object();
                    match print_value {
                        Some(v) => {
                            match v.get("Cdf") {
                                Some(vv) => write!(out, "{}", vv.as_str().get_or_insert(""))?,
                                None => {}
                            }
                            match v.get("Text") {
                                Some(vv) => write!(out, "{}", vv.as_str().get_or_insert(""))?,
                                None => {}
                            }
                            match v.get("Num") {
                                Some(vv) => match vv.as_f64() {
                                    Some(float_val) => write!(out, "{}", float_val)?,
                                    None => {}
                                },
                                None => {}
                            }
                            match v.get("Date") {
                                Some(vv) => match vv.as_str() {
                                    Some(vv) => write!(out, "{}", vv)?,
                                    None => {}
                                },
                                None => {}
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }
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
    handlebars.register_helper("df", Box::new(data_field_helper));
    handlebars
        .register_template_file("template", "./render/bdtlist.hbs")
        .unwrap();
    let mut output_file = stdout();
    let bdtlist = BdtList {
        tables: tables
            .into_iter()
            .filter(|bdt| !"skip".eq(bdt.skip.as_str()))
            .collect(),
    };
    handlebars.render_to_write("template", &bdtlist, &mut output_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::bdt::column_value::ColumnValue;

    use super::*;

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
    fn data_field_helper_test_cdf() {
        let source = r#"{{df data "CDF2_ID" ~}}"#;
        let data_json = r#"[{
                "name": "AGE_FROM",
                "ref_code": "NUM1",
                "value": {"Num": 0.0}
            },{
                "name": "AGE_TILL",
                "ref_code": "NUM2",
                "value": {"Num": 4.0}
            },{
                "name": "READ_ONLY",
                "ref_code": "CDF1_ID",
                "value": {"Cdf": "Y"}
            },{
                "name": "DEFAULT_FIELD_VALUE",
                "ref_code": "CDF2_ID",
                "value": {"Cdf": "Y"}
            }]"#;
        let values: Vec<ColumnValue> = serde_json::from_str(data_json).unwrap();

        let mut handlebars = setup(source);
        handlebars.register_helper("df", Box::new(data_field_helper));

        let mut column_data: HashMap<&str, Vec<ColumnValue>> = HashMap::new();
        column_data.insert("data", values);
        assert_eq!(handlebars.render("testing", &column_data).unwrap(), "Y");
    }

    #[test]
    fn data_field_helper_test_num() {
        let source = r#"{{df data "NUM2" ~}}"#;
        let data_json = r#"[{
                "name": "AGE_FROM",
                "ref_code": "NUM1",
                "value": {"Num": 0.0}
            },{
                "name": "AGE_TILL",
                "ref_code": "NUM2",
                "value": {"Num": 4.0}
            },{
                "name": "READ_ONLY",
                "ref_code": "CDF1_ID",
                "value": {"Cdf": "Y"}
            },{
                "name": "DEFAULT_FIELD_VALUE",
                "ref_code": "CDF2_ID",
                "value": {"Cdf": "Y"}
            }]"#;
        let values: Vec<ColumnValue> = serde_json::from_str(data_json).unwrap();

        let mut handlebars = setup(source);
        handlebars.register_helper("df", Box::new(data_field_helper));

        let mut column_data: HashMap<&str, Vec<ColumnValue>> = HashMap::new();
        column_data.insert("data", values);
        assert_eq!(handlebars.render("testing", &column_data).unwrap(), "4");
    }

    #[test]
    fn data_field_helper_test_date() {
        let source = r#"{{df data "VALID_TO" ~}}"#;
        let data_json = r#"[{
                "name": "AGE_FROM",
                "ref_code": "NUM1",
                "value": {"Num": 0.0}
            },{
                "name": "VALID_TO",
                "ref_code": "VALID_TO",
                "value": {
                    "Date": "01.09.2017"
                }
            },{
                "name": "READ_ONLY",
                "ref_code": "CDF1_ID",
                "value": {"Cdf": "Y"}
            },{
                "name": "DEFAULT_FIELD_VALUE",
                "ref_code": "CDF2_ID",
                "value": {"Cdf": "Y"}
            }]"#;
        let values: Vec<ColumnValue> = serde_json::from_str(data_json).unwrap();

        let mut handlebars = setup(source);
        handlebars.register_helper("df", Box::new(data_field_helper));

        let mut column_data: HashMap<&str, Vec<ColumnValue>> = HashMap::new();
        column_data.insert("data", values);
        assert_eq!(
            handlebars.render("testing", &column_data).unwrap(),
            "01.09.2017"
        );
    }
}
