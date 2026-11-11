//! The [`Diagnostic`] type.

use crate::{Code, Fix, Span};

/// How serious a diagnostic is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Extra information attached to something else.
    Note,
    /// Code runs, but something looks wrong.
    Warning,
    /// Code cannot run.
    Error,
}

impl Severity {
    /// Portuguese label used when rendering ("erro", "aviso", "nota").
    pub fn label(self) -> &'static str {
        match self {
            Severity::Error => "erro",
            Severity::Warning => "aviso",
            Severity::Note => "nota",
        }
    }
}

/// A span in the code with a short message, drawn as `^^^ message`.
#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    /// Where.
    pub span: Span,
    /// What to say there (may be empty).
    pub message: String,
    /// The primary label gets `^`; secondary labels get `-`.
    pub primary: bool,
}

/// One problem found in the code, ready to be shown to a person (or an AI).
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    /// Stable code.
    pub code: Code,
    /// Severity.
    pub severity: Severity,
    /// Main message in plain language.
    pub message: String,
    /// Places in the code. The first primary label decides the `-->` location.
    pub labels: Vec<Label>,
    /// Help lines ("você quis dizer `reply`?").
    pub help: Vec<String>,
    /// Extra notes.
    pub notes: Vec<String>,
    /// Possible automatic fixes.
    pub fixes: Vec<Fix>,
}

impl Diagnostic {
    /// Creates a diagnostic with no labels.
    pub fn new(severity: Severity, code: Code, message: impl Into<String>) -> Self {
        Diagnostic {
            code,
            severity,
            message: message.into(),
            labels: Vec::new(),
            help: Vec::new(),
            notes: Vec::new(),
            fixes: Vec::new(),
        }
    }

    /// Shortcut for an error.
    pub fn error(code: Code, message: impl Into<String>) -> Self {
        Self::new(Severity::Error, code, message)
    }

    /// Shortcut for a warning.
    pub fn warning(code: Code, message: impl Into<String>) -> Self {
        Self::new(Severity::Warning, code, message)
    }

    /// Adds a primary label.
    pub fn with_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            message: message.into(),
            primary: true,
        });
        self
    }

    /// Adds a secondary label.
    pub fn with_secondary(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            message: message.into(),
            primary: false,
        });
        self
    }

    /// Adds a help line.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help.push(help.into());
        self
    }

    /// Adds a note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Adds a fix.
    pub fn with_fix(mut self, fix: Fix) -> Self {
        self.fixes.push(fix);
        self
    }

    /// The span of the first primary label, if any.
    pub fn primary_span(&self) -> Option<Span> {
        self.labels
            .iter()
            .find(|l| l.primary)
            .or(self.labels.first())
            .map(|l| l.span)
    }

    /// `true` for [`Severity::Error`].
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codes;

    #[test]
    fn builder_collects_parts() {
        let d = Diagnostic::error(codes::E0201, "função `rpely` não existe")
            .with_secondary(Span::new(0, 1), "aqui")
            .with_label(Span::new(4, 9), "")
            .with_help("você quis dizer `reply`?");
        assert!(d.is_error());
        assert_eq!(d.primary_span(), Some(Span::new(4, 9)));
        assert_eq!(d.help.len(), 1);
    }
}
