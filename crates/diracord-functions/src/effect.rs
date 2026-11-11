//! What a built-in function asks the interpreter to do.

use crate::ArgRef;
use diracord_diagnostics::Code;
use diracord_value::{Action, Scope, Value};
use std::fmt;

/// The result of calling a built-in function.
#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    /// A plain result.
    Value(Value),
    /// Ask the host to do something on Discord; the host's answer becomes the result.
    Action(Action),
    /// Read or write saved data; the interpreter runs it on the store.
    Data(DataOp),
}

/// A data operation. `scope: None` means "the project's default scope".
///
/// The interpreter builds the [`diracord_value::StoreKey`] owner from the
/// current context (user id, server id, ...).
#[derive(Clone, Debug, PartialEq)]
pub enum DataOp {
    /// `get(name, default?, scope?)` → the saved value or `default`.
    Get {
        /// Variable name.
        name: String,
        /// Scope, or default.
        scope: Option<Scope>,
        /// Returned when nothing is saved.
        default: Value,
    },
    /// `set(name, value, scope?)` → `null`.
    Set {
        /// Variable name.
        name: String,
        /// Scope, or default.
        scope: Option<Scope>,
        /// Value to save.
        value: Value,
    },
    /// `delete(name, scope?)` → `true` if something was removed.
    Delete {
        /// Variable name.
        name: String,
        /// Scope, or default.
        scope: Option<Scope>,
    },
}

/// A failure inside a built-in function, turned into a diagnostic by the interpreter.
#[derive(Clone, Debug, PartialEq)]
pub struct FnError {
    /// Stable code (usually `E03xx`).
    pub code: Code,
    /// Plain-language message.
    pub message: String,
    /// Optional help line.
    pub help: Option<String>,
    /// Which argument is wrong, so the arrow points at it.
    pub arg: Option<ArgRef>,
}

impl FnError {
    /// Creates an error.
    pub fn new(code: Code, message: impl Into<String>) -> Self {
        FnError { code, message: message.into(), help: None, arg: None }
    }

    /// Sets the help line.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Points at an argument.
    pub fn at(mut self, arg: ArgRef) -> Self {
        self.arg = Some(arg);
        self
    }
}

impl fmt::Display for FnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for FnError {}
