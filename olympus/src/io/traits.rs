pub trait ImageReaderDispatch {
    fn read(&mut self, filename: &str) -> Result<(), String>;
}

pub trait ImageWriterDispatch {
    fn save(&self, filename: &str) -> Result<(), String>;
}
