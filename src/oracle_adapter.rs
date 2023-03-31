use include_oracle_sql::{include_sql, impl_sql};

include_sql!("sql/bdt.sql");

#[cfg(not(feature = "tokio"))]
pub fn read_oracle() -> sibyl::Result<()> {
    // let oracle = sibyl::env()?;
    //
    // let session = oracle.connect("localhost/xe", "bta", "bta_234")?;
    //
    // session.get_tables("TT_CONFIG", |row| {
    //     let table_name: &str = row.get("NAME")?;
    //     print!("Table name: {}", table_name);
    //     Ok(())
    // })?;
    //
    // Ok(())

    let oracle = sibyl::env()?;
    let session = oracle.connect("localhost/xe", "bta", "bta_234")?;

    let stmt = session.prepare("
        select IC from adm_codif_entry where ic = :IC_CODE
    ")?;

    let rows = stmt.query("MG_TAB")?;

    while let Some(row) = rows.next()? {
        let ic : &str = row.get(0)?;
        println!("{:25}", ic);
    }
    Ok(())
}

