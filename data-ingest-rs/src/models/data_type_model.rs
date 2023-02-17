#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileFormat {
    Csv,
    Parquet,
    Unknown,
}

impl From<String> for FileFormat {
    fn from(value: String) -> Self {
        if value.contains(".csv") {
            FileFormat::Csv
        } else if value.contains(".parquet") {
            FileFormat::Parquet
        } else {
            FileFormat::Unknown
        }
    }
}
impl FileFormat {
    pub fn is_csv(&self) -> bool {
        self.eq(&FileFormat::Csv)
    }

    pub fn is_parquet(&self) -> bool {
        self.eq(&FileFormat::Parquet)
    }

    pub fn is_unknown(&self) -> bool {
        self.eq(&FileFormat::Unknown)
    }
}
