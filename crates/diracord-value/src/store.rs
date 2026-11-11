//! The data interface behind `get` / `set` (implemented by Dirabank).

use crate::Value;
use std::collections::HashMap;
use std::fmt;

/// Where a saved value lives. A key is `scope + owner id + name`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Scope {
    /// Per user, across all servers. Owner = user id.
    User,
    /// Per server. Owner = server id.
    Server,
    /// Per user inside one server. Owner = `"server_id:user_id"`.
    Member,
    /// One value for the whole bot. Owner = `""`.
    Global,
}

impl Scope {
    /// Every scope.
    pub const ALL: [Scope; 4] = [Scope::User, Scope::Server, Scope::Member, Scope::Global];

    /// The keyword used in code (`scope: user`).
    pub fn as_str(self) -> &'static str {
        match self {
            Scope::User => "user",
            Scope::Server => "server",
            Scope::Member => "member",
            Scope::Global => "global",
        }
    }

    /// Parses a scope keyword.
    pub fn from_name(name: &str) -> Option<Scope> {
        Scope::ALL.into_iter().find(|s| s.as_str() == name)
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Full address of a saved value.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StoreKey {
    /// Scope.
    pub scope: Scope,
    /// Owner id inside the scope (see [`Scope`]).
    pub owner: String,
    /// Variable name chosen by the bot author (`"coins"`).
    pub name: String,
}

impl StoreKey {
    /// Creates a key.
    pub fn new(scope: Scope, owner: impl Into<String>, name: impl Into<String>) -> Self {
        StoreKey {
            scope,
            owner: owner.into(),
            name: name.into(),
        }
    }
}

/// A data store failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreError {
    /// Plain-language message.
    pub message: String,
}

impl StoreError {
    /// Creates an error.
    pub fn new(message: impl Into<String>) -> Self {
        StoreError {
            message: message.into(),
        }
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for StoreError {}

/// Persistent key-value storage used by `get`, `set` and `delete`.
///
/// Implementations: `diracord_dirabank::Dirabank` (file), [`MemoryStore`] (tests).
/// Drivers are pluggable, so other databases can implement this later.
pub trait Store {
    /// Reads a value. Missing keys return `Ok(None)`.
    fn get(&self, key: &StoreKey) -> Result<Option<Value>, StoreError>;
    /// Writes a value (replacing any old one).
    fn set(&mut self, key: &StoreKey, value: Value) -> Result<(), StoreError>;
    /// Removes a value. Returns `true` if it existed.
    fn delete(&mut self, key: &StoreKey) -> Result<bool, StoreError>;
    /// Makes pending writes durable. The default does nothing.
    fn flush(&mut self) -> Result<(), StoreError> {
        Ok(())
    }
}

/// An in-memory [`Store`], for tests and `diracord check`.
#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    data: HashMap<StoreKey, Value>,
}

impl MemoryStore {
    /// An empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of stored values.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// `true` when empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Store for MemoryStore {
    fn get(&self, key: &StoreKey) -> Result<Option<Value>, StoreError> {
        Ok(self.data.get(key).cloned())
    }
    fn set(&mut self, key: &StoreKey, value: Value) -> Result<(), StoreError> {
        self.data.insert(key.clone(), value);
        Ok(())
    }
    fn delete(&mut self, key: &StoreKey) -> Result<bool, StoreError> {
        Ok(self.data.remove(key).is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_roundtrip() {
        for s in Scope::ALL {
            assert_eq!(Scope::from_name(s.as_str()), Some(s));
        }
        assert_eq!(Scope::from_name("guild"), None);
    }

    #[test]
    fn memory_store() {
        let mut s = MemoryStore::new();
        let k = StoreKey::new(Scope::User, "1", "coins");
        assert_eq!(s.get(&k), Ok(None));
        s.set(&k, Value::Int(5)).unwrap();
        assert_eq!(s.get(&k), Ok(Some(Value::Int(5))));
        assert_eq!(s.len(), 1);
        assert_eq!(s.delete(&k), Ok(true));
        assert_eq!(s.delete(&k), Ok(false));
        assert!(s.is_empty());
    }
}
