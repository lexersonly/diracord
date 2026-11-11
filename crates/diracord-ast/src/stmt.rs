//! Statements and blocks.

use crate::{Expr, Ident, Literal};
use diracord_diagnostics::Span;

/// `{ stmt* }`
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    /// Statements in order.
    pub stmts: Vec<Stmt>,
    /// From `{` to `}` inclusive. For a single-statement chain arm body, the statement's span.
    pub span: Span,
}

/// A statement.
#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    /// What it is.
    pub kind: StmtKind,
    /// Its full extent.
    pub span: Span,
}

/// The kinds of statements.
#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    /// `local x = 15` — the variable lives until the end of the block.
    Local {
        /// Variable name.
        name: Ident,
        /// Initial value.
        value: Expr,
    },
    /// `x = x + 1`, `obj.field = 2`, `list[0] = 3`.
    Assign {
        /// A name, member or index expression.
        target: Expr,
        /// New value.
        value: Expr,
    },
    /// An expression used as a statement, usually a call: `reply "oi"`.
    Expr(Expr),
    /// `if a { } else if b { } else { }`
    If {
        /// The `if` and every `else if`, in order (at least one).
        branches: Vec<IfBranch>,
        /// The final `else`, if any.
        otherwise: Option<Block>,
    },
    /// `for item in list { }`
    For {
        /// Loop variable.
        var: Ident,
        /// What is iterated (list, text, number range, object keys).
        iter: Expr,
        /// Body.
        body: Block,
    },
    /// `while cond { }`
    While {
        /// Condition, checked before each iteration.
        cond: Expr,
        /// Body.
        body: Block,
    },
    /// `chain x { 1 => reply "um"  _ => reply "outro" }` — the switch/match.
    Chain {
        /// The value being matched.
        subject: Expr,
        /// Arms in order; the first match wins.
        arms: Vec<ChainArm>,
    },
    /// `testing { } catch err { }` — error handling.
    Testing {
        /// Protected block.
        body: Block,
        /// Name bound to the error message, if written.
        catch_var: Option<Ident>,
        /// Runs when `body` fails.
        handler: Block,
    },
    /// `return` or `return value`.
    Return(Option<Expr>),
    /// `stop` — ends the whole command/event.
    Stop,
    /// `break` — leaves the innermost loop.
    Break,
    /// `continue` — next iteration of the innermost loop.
    Continue,
}

/// One `if` / `else if` condition with its block.
#[derive(Clone, Debug, PartialEq)]
pub struct IfBranch {
    /// Condition.
    pub cond: Expr,
    /// Block to run when true.
    pub body: Block,
    /// From the `if` keyword to the end of the block.
    pub span: Span,
}

/// One arm of a `chain`.
#[derive(Clone, Debug, PartialEq)]
pub struct ChainArm {
    /// What to match.
    pub pattern: Pattern,
    /// What to run. A single statement after `=>` becomes a one-statement block.
    pub body: Block,
    /// The whole arm.
    pub span: Span,
}

/// A `chain` pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    /// What it is.
    pub kind: PatternKind,
    /// Where.
    pub span: Span,
}

/// Kinds of `chain` patterns.
#[derive(Clone, Debug, PartialEq)]
pub enum PatternKind {
    /// `_` — matches anything.
    Wildcard,
    /// A literal: `1`, `"a"`, `true`, `null`.
    Literal(Literal),
}
