use color_eyre::eyre::Context;
use polars::prelude::*;

pub fn read_parquet_file(file_name: String) -> color_eyre::Result<()> {
    let df = LazyFrame::scan_parquet(file_name.clone(), Default::default())?
        .collect()
        .wrap_err(format!("could not read parquet file {file_name}"))?;

    println!("{}", df.head(Some(10)));

    Ok(())
}

pub fn read_csv_file(file_name: String) -> color_eyre::Result<()> {
    let df = LazyCsvReader::new(file_name.clone())
        .has_header(false)
        .finish()?
        .collect()
        .wrap_err(format!("could not read after csv file {file_name}"))?;

    println!("{}", df.head(Some(10)));
    Ok(())
}
