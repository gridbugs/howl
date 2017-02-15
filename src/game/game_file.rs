use std::result;
use std::path;
use std::fs;
use std::io::Read;

use serde::de::Deserialize;
use toml;

#[derive(Debug, Clone, Copy)]
pub enum FileError {
    MissingFile,
    InvalidFile,
    InvalidFormat,
}

pub type FileResult<T> = result::Result<T, FileError>;

pub fn read_string<P: AsRef<path::Path>>(path: P) -> FileResult<String> {
    let mut file = fs::File::open(path).map_err(|_| FileError::MissingFile)?;
    let mut string = String::new();
    file.read_to_string(&mut string).map_err(|_| FileError::InvalidFile)?;

    Ok(string)
}

pub fn read_toml<P: AsRef<path::Path>, T: Deserialize>(path: P) -> FileResult<T> {
    let s = read_string(path)?;
    toml::from_str(&s).map_err(|_| FileError::InvalidFormat)
}
