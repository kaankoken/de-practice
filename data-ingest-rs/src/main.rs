mod util;
mod models;

use clap::{Args, Command};
use std::{env, process::exit};

use crate::models::Cli;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
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

    if let true = url.is_some() {
        util::download_file(file.to_owned(), String::from("test.csv")).await?;
    }

    // TODO: read file

    Ok(())
}
