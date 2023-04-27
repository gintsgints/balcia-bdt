use std::error::Error;
use std::io::stdout;

use handlebars::Handlebars;

use crate::bdt::Bdt;

pub struct CsvDataAdapter {}

impl CsvDataAdapter {
    pub fn write_bdt(table: Bdt) -> Result<(), Box<dyn Error>> {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_file("template", "./render/csv_data.hbs")
            .unwrap();
        let mut output_file = stdout();
        handlebars.render_to_write("template", &table, &mut output_file)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use handlebars::Handlebars;
    use std::error::Error;

    use crate::json_adapter::JsonAdapter;

    #[test]
    fn full_test() -> Result<(), Box<dyn Error>> {
        let v = JsonAdapter::read_bdt_from_file("./data/TT/bdt.json")?;

        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_file("template", "./render/csv_data.hbs")
            .unwrap();
        for bdt in v {
            let res = handlebars.render("table", &bdt).unwrap();
        }
        Ok(())
    }
}
