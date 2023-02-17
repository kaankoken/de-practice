mod extensions;
mod models;
mod utils;

use clap::{Args, Command};
use extensions::StringExt;
use models::Cli;
use std::{env, process::exit};

type Error = color_eyre::Report;

#[tokio::main]
async fn main() -> color_eyre::Result<(), Error> {
    color_eyre::install()?;

    let cli = Command::new("data-ingest");
    let mut cli = Cli::augment_args(cli);

    if env::args().count() <= 1 {
        cli.print_help().err();

        exit(0);
    }

    let matches = cli.get_matches();

    let url = matches.get_one::<String>("url");
    let file = matches.get_one::<String>("file");

    let file = match url.is_some() {
        true => url.unwrap(),
        _ => file.unwrap(),
    };

    let file_name = if url.is_some() {
        utils::download_file(file.clone()).await?
    } else {
        file.to_string()
    };

    let file_type = file_name.file_type();

    match (
        file_type.is_csv(),
        file_type.is_parquet(),
        file_type.is_unknown(),
    ) {
        (true, _, _) => utils::read_csv_file(file_name),
        (_, true, _) => utils::read_parquet_file(file_name),
        _ => Err(color_eyre::eyre::eyre!(
            "could not process, unknown file type {file_name}",
        )),
    }?;

    utils::clean_downloads()?;

    Ok(())
}
