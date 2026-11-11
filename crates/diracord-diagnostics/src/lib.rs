//! Spans, diagnostics and error rendering for Diracord.
//!
//! Errors are part of the product. Every [`Diagnostic`] carries:
//!
//! 1. a plain-language message ("Faltou fechar a chave"),
//! 2. one or more [`Label`]s pointing at the code (rendered with an arrow),
//! 3. optional help text ("você quis dizer `reply`?"),
//! 4. optional [`Fix`]es that can be applied automatically when marked safe,
//! 5. a stable [`Code`] (`E0001`, ...) documented in [`codes`].
//!
//! This crate has no dependencies, so every other crate can use it.
//!
//! ```
//! use diracord_diagnostics::{codes, Diagnostic, SourceFile, Span, render};
//!
//! let src = SourceFile::new("ping.dcs", "command(\"ping\") {\n  reply \"Pong!\"\n");
//! let diag = Diagnostic::error(codes::E0101, "Faltou fechar a chave")
//!     .with_label(Span::new(16, 17), "esta chave nunca foi fechada")
//!     .with_help("coloque `}` no fim do bloco");
//! let text = render(&diag, &src, false);
//! assert!(text.contains("E0101"));
//! assert!(text.contains("^"));
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod codes;
mod diagnostic;
mod fix;
mod render;
mod source;
mod span;
mod suggest;

pub use codes::Code;
pub use diagnostic::{Diagnostic, Label, Severity};
pub use fix::{apply_fixes, Edit, Fix, FixError};
pub use render::{render, render_all};
pub use source::{LineCol, SourceFile};
pub use span::{Span, Spanned};
pub use suggest::{did_you_mean, edit_distance};
