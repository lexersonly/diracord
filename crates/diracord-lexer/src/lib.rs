//! Lexer for the Diracord language: source text → [`Token`]s.
//!
//! - Blocks use `{ }`; `//` starts a comment until the end of the line.
//! - Strings are `"..."` and interpolate variables: `"Hi $user.name"`.
//!   Escapes: `\n`, `\t`, `\"`, `\\`, `\$`.
//! - `$` outside strings becomes [`TokenKind::Dollar`] (for `$ban[a;b]`);
//!   whether that syntax is allowed is decided by the parser's options.
//! - Each token records whether a line break or a space came before it,
//!   because calls without parentheses (`reply "hi"`) end at the line break.
//!
//! The lexer never stops at the first error: it reports a diagnostic and keeps going.
//!
//! ```
//! use diracord_lexer::{tokenize, TokenKind};
//! let (tokens, diags) = tokenize("reply \"Hi $user.name\"");
//! assert!(diags.is_empty());
//! assert!(matches!(tokens[0].kind, TokenKind::Ident(ref s) if s == "reply"));
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod cursor;
mod lexer;
mod string;
mod token;

pub use lexer::tokenize;
pub use token::{StrPart, Token, TokenKind, VarPath};
