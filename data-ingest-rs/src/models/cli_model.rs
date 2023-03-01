use clap::{builder, ArgGroup, Args};

#[derive(Args, Debug)]
#[command(name = "data-ingest")]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("input").required(true).args(["url", "file"])))]
pub struct Cli {
    /// The connection string to connect database instance
    #[arg(short, long = "conn", value_parser = builder::NonEmptyStringValueParser::new())]
    conn_string: String,

    /// The URL of the csv or parquet data that will ingest
    #[arg(long, value_parser = builder::NonEmptyStringValueParser::new())]
    url: String,

    /// The file of the csv or parquet data that will ingest
    #[arg(long, value_parser = builder::NonEmptyStringValueParser::new())]
    file: String,
}
