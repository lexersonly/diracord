//! Runtime errors.

use diracord_diagnostics::{Code, Diagnostic, FileId, Span};
use std::fmt;

/// An error that stopped a run. It carries a full [`Diagnostic`] (message,
/// arrow, help) and the file it points into.
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeError {
    /// What to show (boxed so `Result`s stay small on the hot path).
    pub diagnostic: Box<Diagnostic>,
    /// Which file of the project the spans refer to.
    pub file: FileId,
    /// `false` for limit errors, which `try/catch` cannot catch.
    pub catchable: bool,
}

impl RuntimeError {
    /// A catchable error with one label.
    pub fn new(
        code: Code,
        message: impl Into<String>,
        span: Span,
        label: impl Into<String>,
    ) -> Self {
        RuntimeError {
            diagnostic: Box::new(Diagnostic::error(code, message).with_label(span, label)),
            file: FileId::default(),
            catchable: true,
        }
    }

    /// A limit error (not catchable).
    pub(crate) fn limit(code: Code, message: impl Into<String>, span: Span, help: &str) -> Self {
        RuntimeError {
            diagnostic: Box::new(
                Diagnostic::error(code, message)
                    .with_label(span, "parou aqui")
                    .with_help(help),
            ),
            file: FileId::default(),
            catchable: false,
        }
    }

    /// Adds a help line.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.diagnostic.help.push(help.into());
        self
    }

    /// The text bound to `err` in `catch err { }`.
    pub fn message(&self) -> &str {
        &self.diagnostic.message
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.diagnostic.code, self.diagnostic.message)
    }
}

impl std::error::Error for RuntimeError {}
