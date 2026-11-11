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
        let mut r = Registry {
            specs: Vec::new(),
            by_name: HashMap::new(),
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_core_functions() {
        let r = registry();
        for name in [
            "reply", "send", "ban", "kick", "timeout", "get", "set", "channel", "upper", "random",
        ] {
            assert!(r.get(name).is_some(), "falta {name}");
        }
        assert!(r.len() >= 40);
    }

    #[test]
    fn names_are_unique_and_categories_match_folders() {
        let mut seen = std::collections::HashSet::new();
        for s in registry().iter() {
            assert!(seen.insert(s.name), "repetido: {}", s.name);
            assert!(
                !s.summary.is_empty() && !s.example.is_empty(),
                "{} sem doc",
                s.name
            );
            assert!(
                s.example.contains(s.name),
                "o exemplo de {} não usa a função",
                s.name
            );
        }
    }

    #[test]
    fn optional_params_come_last() {
        for s in registry().iter() {
            let first_opt = s
                .params
                .iter()
                .position(|p| p.optional)
                .unwrap_or(s.params.len());
            assert!(
                s.params[first_opt..].iter().all(|p| p.optional),
                "{}",
                s.name
            );
        }
    }

    #[test]
    fn suggests_typos() {
        let r = registry();
        assert_eq!(
            diracord_diagnostics::did_you_mean("rpely", r.names()),
            Some("reply")
        );
    }
}
