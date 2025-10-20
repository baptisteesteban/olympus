/// Trait to be implemented by an image to read a file and get its data.
pub trait ImageReaderDispatch {
    fn read(&mut self, filename: &str) -> Result<(), String>;
}

/// Trait to be implemented by an image to save it into a file.
pub trait ImageWriterDispatch {
    fn save(&self, filename: &str) -> Result<(), String>;
}
