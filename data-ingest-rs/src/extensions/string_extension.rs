pub trait StringExt {
    fn is_csv(&self) -> bool;
}

impl StringExt for String {
    fn is_csv(&self) -> bool {
        self.contains(".csv")
    }
}
