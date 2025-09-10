use std::{
    fmt,
    fs::{Metadata, metadata},
};

pub struct FileNotFoundError;
impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file not found")
    }
}

pub fn load_meta_from_path(path: &str) -> Result<Metadata, FileNotFoundError> {
    match metadata(&path) {
        Ok(meta) => Ok(meta),
        Err(_) => Err(FileNotFoundError),
    }
}
