//! Built-in functions of Diracord.
//!
//! Layout: **one folder per category, one file per function**
//! (`src/builtins/moderation/ban.rs`). Each file exports a single
//! [`FunctionSpec`] constant named `SPEC`.
//!
//! Functions are **pure**: they never talk to Discord or to the database.
//! They validate their arguments and return an [`Effect`]:
//!
//! - [`Effect::Value`]: the result (e.g. `upper("a")` → `"A"`);
//! - [`Effect::Action`]: something the host must do (`reply`, `ban`);
//! - [`Effect::Data`]: a `get`/`set`/`delete` the interpreter runs on the store.
//!
//! The interpreter turns effects into host calls and store operations, so this
//! crate stays small and easy to test.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod args;
mod builtins;
mod effect;
pub mod layout;
mod registry;
mod spec;

pub use args::{ArgRef, Args, Bound};
pub use effect::{DataOp, Effect, FnError};
pub use registry::{registry, Registry};
pub use spec::{Category, FunctionSpec, ParamKind, ParamSpec};
