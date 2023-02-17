use crate::models::FileFormat;

pub trait StringExt {
    fn file_type(&self) -> FileFormat;
}

impl StringExt for String {
    fn file_type(&self) -> FileFormat {
        FileFormat::from(self.clone())
    }
}
