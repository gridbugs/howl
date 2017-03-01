use std::result;
use std::path;
use std::fs::File;
use std::io::{Read, Write};

use serde::ser::Serialize;
use serde::de::Deserialize;
use toml;

#[derive(Debug, Clone, Copy)]
pub enum FileError {
    MissingFile,
    InvalidFile,
    InvalidFormat,
    FailedToWrite,
}

pub type FileResult<T> = result::Result<T, FileError>;

pub fn read_string<P: AsRef<path::Path>>(path: P) -> FileResult<String> {
    let mut file = File::open(path).map_err(|_| FileError::MissingFile)?;
    let mut string = String::new();
    file.read_to_string(&mut string).map_err(|_| FileError::InvalidFile)?;

    Ok(string)
}

pub fn read_toml<P: AsRef<path::Path>, T: Deserialize>(path: P) -> FileResult<T> {
    let s = read_string(path)?;
    toml::from_str(&s).map_err(|_| FileError::InvalidFormat)
}

pub fn write_string<P: AsRef<path::Path>, S: AsRef<str>>(path: P, string: S) -> FileResult<()> {
    let bytes = string.as_ref().as_bytes();
    File::create(path).and_then(|mut f| f.write_all(bytes))
        .map_err(|_| FileError::FailedToWrite)?;
    Ok(())
}

pub fn write_toml<P: AsRef<path::Path>, T: Serialize>(path: P, data: &T) -> FileResult<()> {
    let string = toml::to_string(data).map_err(|_| FileError::InvalidFormat)?;
    write_string(path, string)
}
