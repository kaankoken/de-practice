use std::env;

use clap::{builder, Args, Command};

#[derive(Args, Debug)]
#[command(name = "data-ingest")]
#[command(author, version, about, long_about = None)]
//#[command(group(ArgGroup::new()))]
struct Cli {
    /// The user name to use when connecting to the database.
    #[arg(short, long, value_parser = builder::NonEmptyStringValueParser::new(), default_value = "root")]
    user: String,

    /// The password to use when connecting to the database.
    #[arg(short = 'P', long, value_parser = builder::NonEmptyStringValueParser::new(), default_value = "root")]
    password: String,

    /// The host to connect to.
    #[arg(short = 'H', long, default_value_t = 5432)]
    host: u16,

    /// The port to connect to.
    #[arg(short, long, default_value_t = 5432)]
    port: u16,

    /// The name of the database to connect to.
    #[arg(short = 'n', long = "name", value_parser = builder::NonEmptyStringValueParser::new(), default_value = "postgres")]
    database_name: String,

    /// The name of the table to ingest data into.
    #[arg(short = 't', long = "table", value_parser = builder::NonEmptyStringValueParser::new())]
    table_name: String,

    /// The URL of the csv data that will ingest
    #[arg(long, required = true, value_parser = builder::NonEmptyStringValueParser::new())]
    url: String,
}

fn main() {
    let cli = Command::new("data-ingest");
    let mut cli = Cli::augment_args(cli);

    if env::args().count() <= 1 {
        cli.print_help().unwrap();

        return;
    }

    println!("Hello, world!");

    
}
