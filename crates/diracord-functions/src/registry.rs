//! Lookup of built-in functions by name.

use crate::{builtins, Category, FunctionSpec};
use std::collections::HashMap;
use std::sync::OnceLock;

/// All built-in functions, indexed by name.
#[derive(Debug)]
pub struct Registry {
    specs: Vec<&'static FunctionSpec>,
    by_name: HashMap<&'static str, usize>,
}

impl Registry {
    /// Builds a registry from specs. Later duplicates are ignored.
    pub fn new(specs: impl IntoIterator<Item = &'static FunctionSpec>) -> Self {
        let mut r = Registry { specs: Vec::new(), by_name: HashMap::new() };
        for s in specs {
            if !r.by_name.contains_key(s.name) {
                r.by_name.insert(s.name, r.specs.len());
                r.specs.push(s);
            }
        }
        r
    }

    /// Finds a function.
    pub fn get(&self, name: &str) -> Option<&'static FunctionSpec> {
        self.by_name.get(name).map(|&i| self.specs[i])
    }

    /// Every function, in registration order.
    pub fn iter(&self) -> impl Iterator<Item = &'static FunctionSpec> + '_ {
        self.specs.iter().copied()
    }

    /// Every function name (for "você quis dizer ...?").
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.specs.iter().map(|s| s.name)
    }

    /// Functions of one category.
    pub fn in_category(&self, c: Category) -> impl Iterator<Item = &'static FunctionSpec> + '_ {
        self.iter().filter(move |s| s.category == c)
    }

    /// Number of functions.
    pub fn len(&self) -> usize {
        self.specs.len()
    }

    /// `true` when empty.
    pub fn is_empty(&self) -> bool {
        self.specs.is_empty()
    }
}

/// The built-in registry, created once.
pub fn registry() -> &'static Registry {
    static REG: OnceLock<Registry> = OnceLock::new();
    REG.get_or_init(|| Registry::new(builtins::all().iter().copied()))
}
