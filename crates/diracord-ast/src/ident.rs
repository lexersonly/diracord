//! Names.

use diracord_diagnostics::Span;

/// A name in the source (`x`, `user`, `reply`) with its span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident {
    /// The name as written.
    pub name: String,
    /// Where it was written.
    pub span: Span,
}

impl Ident {
    /// Creates an identifier.
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Ident { name: name.into(), span }
    }

    /// The name.
    pub fn as_str(&self) -> &str {
        &self.name
    }
}
