//! In-memory driver.

use crate::{Driver, DriverError};
use std::collections::HashMap;

/// Keeps everything in a `HashMap`. Nothing is saved.
#[derive(Debug, Default, Clone)]
pub struct MemoryDriver {
    map: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemoryDriver {
    /// Number of keys.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// `true` when empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Driver for MemoryDriver {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DriverError> {
        Ok(self.map.get(key).cloned())
    }
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), DriverError> {
        self.map.insert(key.to_vec(), value.to_vec());
        Ok(())
    }
    fn remove(&mut self, key: &[u8]) -> Result<bool, DriverError> {
        Ok(self.map.remove(key).is_some())
    }
    fn flush(&mut self) -> Result<(), DriverError> {
        Ok(())
    }
}
