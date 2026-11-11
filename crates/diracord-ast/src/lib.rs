//! Syntax tree for the Diracord language.
//!
//! This crate is the contract between the parser (which builds the tree) and
//! the interpreter / checker (which read it). **Every node has a [`Span`].**
//!
//! Layout of a program:
//!
//! ```text
//! Program
//! └── Item           command("ping") { ... } | memberJoin { ... } | button("id") { ... } | fn name(...) { ... }
//!     └── Block      { stmt; stmt; ... }
//!         └── Stmt   local / assign / if / for / while / chain / testing / return / ...
//!             └── Expr  literal / name / "text $var" / call / binary / ...
//! ```
//!

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod expr;
mod ident;
mod item;
pub mod known;
mod stmt;

pub use diracord_diagnostics::{FileId, Span};
pub use expr::{
    Arg, BinaryOp, CallStyle, Expr, ExprKind, Literal, ObjectField, TemplatePart, UnaryOp,
};
pub use ident::Ident;
pub use item::{
    ArgType, CommandDef, EventDef, FnParam, FunctionDef, InteractionDef, InteractionKind, Item,
    ItemKind, Param, Program,
};
pub use stmt::{Block, ChainArm, IfBranch, Pattern, PatternKind, Stmt, StmtKind};
