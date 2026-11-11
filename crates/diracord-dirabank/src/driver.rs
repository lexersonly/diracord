//! The pluggable storage interface.

use std::fmt;

/// A storage backend working on raw bytes.
pub trait Driver {
    /// Reads the value of `key`.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DriverError>;
    /// Writes `value` for `key`.
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), DriverError>;
    /// Removes `key`; returns `true` if it existed.
    fn remove(&mut self, key: &[u8]) -> Result<bool, DriverError>;
    /// Makes writes durable (e.g. `fsync`).
    fn flush(&mut self) -> Result<(), DriverError>;
}

/// A storage failure.
#[derive(Debug)]
pub enum DriverError {
    /// Operating-system I/O error.
    Io(std::io::Error),
    /// The file is not a Dirabank file, or has an unknown version.
    BadFile(String),
    /// A key or value is too large for the format.
    TooLarge,
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverError::Io(e) => write!(f, "erro de arquivo: {e}"),
            DriverError::BadFile(m) => write!(f, "arquivo do Dirabank inválido: {m}"),
            DriverError::TooLarge => f.write_str("chave ou valor grande demais"),
        }
    }
}

impl std::error::Error for DriverError {}

impl From<std::io::Error> for DriverError {
    fn from(e: std::io::Error) -> Self {
        DriverError::Io(e)
    }
}
