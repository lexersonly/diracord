//! Runtime values shared by the Diracord interpreter, built-in functions,
//! hosts and the data store.
//!
//! - [`Value`]: the dynamic value (`null`, bool, number, text, list, object).
//! - [`ops`]: arithmetic and comparison with clear errors.
//! - [`Action`]: a request for the host to do something on Discord (`reply`, `ban`).
//! - [`Store`]: the data interface behind `get`/`set`, with [`Scope`]s.
//!
//! Discord things (users, members, channels, roles, servers, messages) are
//! plain objects with at least an `id` field, so the engine never depends on
//! Discord types.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod action;
pub mod ops;
mod store;
mod value;

pub use action::Action;
pub use ops::OpError;
pub use store::{MemoryStore, Scope, Store, StoreError, StoreKey};
pub use value::{Map, Value};
