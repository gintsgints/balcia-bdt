use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use serde::Serialize;
use std::error::Error;
use std::fs::File;

use crate::Bdt;

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
    use super::*;
    use std::collections::HashMap;

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
}
