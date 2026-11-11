//! The boundary between the engine ("brain") and Discord ("hands").

use diracord_value::{Action, Value};
use std::fmt;
use std::future::Future;
use std::pin::Pin;

/// A boxed future, not required to be `Send` (the engine is single-threaded per run).
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// Performs [`Action`]s on Discord for the interpreter.
///
/// One host value serves one run (it knows which message/interaction to
/// answer). Built-in action names and their arguments are documented in
/// `docs/engine-protocol.md`: `reply`, `send`, `dm`, `react`, `deleteMessage`,
/// `ban`, `unban`, `kick`, `timeout`, `addRole`, `removeRole`, `hasRole`,
/// `channel`, `member`, `role`.
pub trait Host {
    /// Performs `action`. The returned value becomes the result of the call
    /// in Diracord code (for example the sent message, or a found channel).
    fn perform(&mut self, action: Action) -> BoxFuture<'_, Result<Value, HostError>>;
}

/// The host could not perform an action (missing permission, unknown user...).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostError {
    /// Plain-language reason, shown to the bot author (and in `catch err`).
    pub message: String,
}

impl HostError {
    /// Creates an error.
    pub fn new(message: impl Into<String>) -> Self {
        HostError {
            message: message.into(),
        }
    }
}

impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for HostError {}
