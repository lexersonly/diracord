//! Syntax tree for the Diracord language.
//!
//! This crate is the contract between the parser (which builds the tree) and
//! the interpreter / checker (which read it). **Every node has a [`Span`].**
//!
//! Layout of a program:
//!
//! ```text
//! Program
//! └── Item           command("ping") { ... } | memberJoin { ... } | button("id") { ... }
//!     └── Block      { stmt; stmt; ... }
//!         └── Stmt   local / assign / if / for / while / chain / try / return / ...
//!             └── Expr  literal / name / "text $var" / call / binary / ...
//! ```
//!
//! Components v2 layouts (`layout { ... }`) have their own small tree, see [`Layout`].
//!
//! Undecided parts of the language (marked `[ABERTO]` in `docs/SPEC.md`), such
//! as user-defined `fn`, are intentionally **not** in the tree yet.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod expr;
mod ident;
mod item;
pub mod known;
mod layout;
mod stmt;

pub use diracord_diagnostics::{FileId, Span};
pub use expr::{
    Arg, BinaryOp, CallStyle, Expr, ExprKind, Literal, ObjectField, TemplatePart, UnaryOp,
};
pub use ident::Ident;
pub use item::{
    ArgType, CommandDef, EventDef, InteractionDef, InteractionKind, Item, ItemKind, Param, Program,
};
pub use layout::{ElementKind, ElementParam, Layout, LayoutBranch, LayoutItem, LayoutItemKind};
pub use stmt::{Block, ChainArm, IfBranch, Pattern, PatternKind, Stmt, StmtKind};
