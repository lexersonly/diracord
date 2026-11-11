//! Parser for the Diracord language: tokens → [`diracord_ast::Program`].
//!
//! ```
//! use diracord_parser::{parse, ParseOptions};
//! let out = parse("command(\"ping\") {\n  reply \"Pong!\"\n}\n", &ParseOptions::default());
//! assert!(out.diagnostics.is_empty());
//! assert!(out.program.command("ping").is_some());
//! ```
//!
//! # Rules worth knowing
//!
//! - **Calls without parentheses** (`reply "hi"`, `ban user reason`) are
//!   allowed at the start of a statement and right after `local x =`, `x =`
//!   and `return`. Arguments are separated by spaces (commas optional) and end
//!   at the line break. An argument that is itself a call needs parentheses:
//!   `reply upper("hi")`.
//! - **Conditions** (`if`, `while`, `for ... in`, `chain`) cannot start an
//!   object with `{`, because `{` opens the block there.
//! - **`$name[a;b]`** is the same as `name(a, b)`, but only when
//!   [`ParseOptions::dollar`] is on; otherwise it is error `E0108` with a safe fix.
//! - The parser **recovers** from errors: it reports as many as it can in one pass.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod dollar;
mod exprs;
mod items;
mod parser;
mod stmts;
#[cfg(test)]
mod tests;

use diracord_ast::Program;
use diracord_diagnostics::Diagnostic;

/// Syntax switches, read from `[syntax]` in `diracord.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParseOptions {
    /// Accept the aoi.js-style shortcut `$ban[user;reason]` (`[syntax] dollar = true`).
    pub dollar: bool,
}

/// The result of parsing one file.
#[derive(Clone, Debug)]
pub struct ParseOutput {
    /// The tree. Always present, even with errors (broken parts are skipped).
    pub program: Program,
    /// Lexer and parser diagnostics, in source order.
    pub diagnostics: Vec<Diagnostic>,
}

impl ParseOutput {
    /// `true` when there is at least one error.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }
}

/// Parses one source file.
pub fn parse(source: &str, options: &ParseOptions) -> ParseOutput {
    let (tokens, mut diagnostics) = diracord_lexer::tokenize(source);
    let mut p = parser::Parser::new(source, tokens, options.clone());
    let program = p.program();
    diagnostics.extend(p.into_diagnostics());
    diagnostics.sort_by_key(|d| d.primary_span().map(|s| s.start).unwrap_or(0));
    ParseOutput {
        program,
        diagnostics,
    }
}
