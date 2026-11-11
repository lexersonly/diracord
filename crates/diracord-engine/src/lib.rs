//! The Diracord engine process.
//!
//! It loads a project (many source files), checks it, and runs commands,
//! events and interactions. It talks to the TypeScript side (CLI and Discord
//! connection) over **JSON lines** on stdin/stdout — the "service" mode of
//! `docs/ARCHITECTURE.md`. The protocol is documented in
//! `docs/engine-protocol.md`.
//!
//! - [`project`]: parse + check many files, render diagnostics, apply fixes.
//! - [`server`]: the protocol loop ([`server::serve`]).
//! - [`json`]: a tiny JSON codec over [`diracord_value::Value`].

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod json;
pub mod project;
pub mod server;
mod wire;
