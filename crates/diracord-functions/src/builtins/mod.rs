//! Built-in function files, one folder per category.
//!
//! To add a function: create `src/builtins/<category>/<name>.rs` exporting
//! `pub const SPEC: FunctionSpec`, declare it in the category's `mod.rs`, and
//! list it in [`all`].

use crate::FunctionSpec;

/// Every built-in function.
pub fn all() -> &'static [&'static FunctionSpec] {
    &[]
}
