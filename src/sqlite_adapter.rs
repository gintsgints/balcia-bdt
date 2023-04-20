use std::error::Error;
use std::io::stdout;
use serde::Serialize;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

use crate::Bdt;

pub struct SqliteAdapter {}

#[derive(Debug, Serialize)]
struct BdtList {
    tables: Vec<Bdt>,
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
        Some(_value) => {
            write!(out, "TEXT")?;
        }
        _ => {
            write!(out, "")?;
        }
    }
    Ok(())
}

impl SqliteAdapter {
    pub fn write_bdt(tables: Vec<Bdt>) -> Result<(), Box<dyn Error>> {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("some", Box::new(some_helper));
        handlebars
            .register_template_file("template", "./render/sqlite.hbs")
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
}