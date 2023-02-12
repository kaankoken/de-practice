use clap::{builder, ArgGroup, Args};

#[derive(Args, Debug)]
#[command(name = "data-ingest")]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("input").required(true).args(["url", "file"])))]
pub struct Cli {
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

    /// The URL of the csv or parquet data that will ingest
    #[arg(long, value_parser = builder::NonEmptyStringValueParser::new())]
    url: String,

    /// The file of the csv or parquet data that will ingest
    #[arg(long, value_parser = builder::NonEmptyStringValueParser::new())]
    file: String,
}
