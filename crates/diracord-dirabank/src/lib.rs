//! Dirabank: the embedded key-value store behind `get` / `set`.
//!
//! - One file, no server, no dependencies.
//! - Low RAM: the file driver keeps only an index (key → position) in memory;
//!   values are read from disk on demand.
//! - Pluggable: [`Dirabank`] works over any [`Driver`] (file, memory, and
//!   later others such as Turso), chosen by configuration.
//!
//! ```
//! use diracord_dirabank::Dirabank;
//! use diracord_value::{Scope, Store, StoreKey, Value};
//!
//! let mut db = Dirabank::memory();
//! let key = StoreKey::new(Scope::User, "42", "coins");
//! db.set(&key, Value::Int(10)).unwrap();
//! assert_eq!(db.get(&key).unwrap(), Some(Value::Int(10)));
//! ```
//!
//! # File format
//!
//! The on-disk format is **provisional (v0)**: the SPEC lists it as an open
//! decision. See `docs/rfcs/dirabank-format.md`. The header carries a version
//! so files can be migrated when the final format is chosen.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod codec;
mod crc;
mod driver;
mod file;
mod memory;

pub use driver::{Driver, DriverError};
pub use file::{FileDriver, FileStats};
pub use memory::MemoryDriver;

use diracord_value::{Store, StoreError, StoreKey, Value};
use std::path::Path;

/// A [`Store`] over a [`Driver`]. Keys and values are encoded to bytes here,
/// so drivers only deal with bytes.
#[derive(Debug)]
pub struct Dirabank<D: Driver = FileDriver> {
    driver: D,
}

impl Dirabank<FileDriver> {
    /// Opens (or creates) a database file.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DriverError> {
        Ok(Dirabank {
            driver: FileDriver::open(path)?,
        })
    }
}

impl Dirabank<MemoryDriver> {
    /// A database that lives only in memory (tests, `diracord check`).
    pub fn memory() -> Self {
        Dirabank {
            driver: MemoryDriver::default(),
        }
    }
}

impl<D: Driver> Dirabank<D> {
    /// Uses a custom driver.
    pub fn with_driver(driver: D) -> Self {
        Dirabank { driver }
    }

    /// The driver.
    pub fn driver(&self) -> &D {
        &self.driver
    }

    /// The driver, mutable (e.g. to compact a file).
    pub fn driver_mut(&mut self) -> &mut D {
        &mut self.driver
    }
}

fn store_err(e: DriverError) -> StoreError {
    StoreError::new(e.to_string())
}

impl<D: Driver> Store for Dirabank<D> {
    fn get(&self, key: &StoreKey) -> Result<Option<Value>, StoreError> {
        match self
            .driver
            .get(&codec::encode_key(key))
            .map_err(store_err)?
        {
            Some(bytes) => codec::decode_value(&bytes)
                .map(Some)
                .map_err(|e| StoreError::new(e.to_string())),
            None => Ok(None),
        }
    }

    fn set(&mut self, key: &StoreKey, value: Value) -> Result<(), StoreError> {
        let bytes = codec::encode_value(&value);
        self.driver
            .put(&codec::encode_key(key), &bytes)
            .map_err(store_err)
    }

    fn delete(&mut self, key: &StoreKey) -> Result<bool, StoreError> {
        self.driver
            .remove(&codec::encode_key(key))
            .map_err(store_err)
    }

    fn flush(&mut self) -> Result<(), StoreError> {
        self.driver.flush().map_err(store_err)
    }
}
