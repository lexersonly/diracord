//! Expressions.

use crate::Ident;
use diracord_diagnostics::Span;

/// An expression.
#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    /// What it is.
    pub kind: ExprKind,
    /// Its full extent.
    pub span: Span,
}

impl Expr {
    /// Creates an expression.
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Expr { kind, span }
    }

    /// The name, when this expression is a bare name.
    pub fn as_ident(&self) -> Option<&Ident> {
        match &self.kind {
            ExprKind::Name(id) => Some(id),
            _ => None,
        }
    }
}

/// The kinds of expressions.
#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    /// `15`, `1.5`, `"plain"`, `true`, `null`.
    Literal(Literal),
    /// A variable or function name: `x`, `user`.
    Name(Ident),
    /// A string with interpolation: `"Oi $user.name"`.
    Template(Vec<TemplatePart>),
    /// `[1, 2, 3]`
    List(Vec<Expr>),
    /// `{ title: "Oi", color: "red" }` (embeds use this).
    Object(Vec<ObjectField>),
    /// `-x`, `!x`, `not x`.
    Unary {
        /// Operator.
        op: UnaryOp,
        /// Operand.
        expr: Box<Expr>,
    },
    /// `a + b`, `a == b`, `a and b`, ...
    Binary {
        /// Operator.
        op: BinaryOp,
        /// Left side.
        lhs: Box<Expr>,
        /// Right side.
        rhs: Box<Expr>,
    },
    /// `user.name`
    Member {
        /// The object.
        object: Box<Expr>,
        /// The field.
        field: Ident,
    },
    /// `list[0]`, `obj["key"]`
    Index {
        /// The list/object/text.
        object: Box<Expr>,
        /// The index or key.
        index: Box<Expr>,
    },
    /// `reply("oi")`, `reply "oi"`, `$ban[user;reason]`.
    Call {
        /// What is called (almost always a [`ExprKind::Name`]).
        callee: Box<Expr>,
        /// Arguments in order.
        args: Vec<Arg>,
        /// How it was written (for messages and formatting only; meaning is the same).
        style: CallStyle,
    },
}

/// A literal value.
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// Whole number.
    Int(i64),
    /// Decimal number.
    Float(f64),
    /// Text without interpolation.
    Text(String),
    /// `true` / `false`.
    Bool(bool),
    /// `null`.
    Null,
}

/// A piece of an interpolated string.
#[derive(Clone, Debug, PartialEq)]
pub enum TemplatePart {
    /// Literal text (escapes already resolved).
    Text(String),
    /// `$user.name` — an expression (a name followed by `.field`s).
    Expr(Expr),
}

/// `key: value` inside `{ }`.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectField {
    /// The key (a name or a string literal, stored as text).
    pub key: Ident,
    /// The value.
    pub value: Expr,
    /// The whole `key: value`.
    pub span: Span,
}

/// A call argument, positional (`"oi"`) or named (`scope: user`).
#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    /// `Some` for `name: value`.
    pub name: Option<Ident>,
    /// The value.
    pub value: Expr,
    /// The whole argument.
    pub span: Span,
}

/// How a call was written. All three mean the same thing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CallStyle {
    /// `reply("oi")`
    Parens,
    /// `reply "oi"`
    Bare,
    /// `$reply[oi]` (only when `[syntax] dollar = true`).
    Dollar,
}

/// Unary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    /// `-x`
    Neg,
    /// `!x` or `not x`
    Not,
}

impl UnaryOp {
    /// Canonical spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
        }
    }
}

/// Binary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    /// `+` (numbers add, text concatenates)
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Rem,
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `&&` or `and` (short-circuit)
    And,
    /// `||` or `or` (short-circuit)
    Or,
}

impl BinaryOp {
    /// Canonical spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Rem => "%",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
        }
    }

    /// Binding power: higher binds tighter. `or` < `and` < comparison < `+ -` < `* / %`.
    pub fn precedence(self) -> u8 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::Eq | BinaryOp::Ne => 3,
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 4,
            BinaryOp::Add | BinaryOp::Sub => 5,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence_order() {
        assert!(BinaryOp::Mul.precedence() > BinaryOp::Add.precedence());
        assert!(BinaryOp::Add.precedence() > BinaryOp::Lt.precedence());
        assert!(BinaryOp::Lt.precedence() > BinaryOp::Eq.precedence());
        assert!(BinaryOp::Eq.precedence() > BinaryOp::And.precedence());
        assert!(BinaryOp::And.precedence() > BinaryOp::Or.precedence());
    }

    #[test]
    fn as_ident() {
        let e = Expr::new(
            ExprKind::Name(Ident::new("x", Span::new(0, 1))),
            Span::new(0, 1),
        );
        assert_eq!(e.as_ident().map(Ident::as_str), Some("x"));
        let l = Expr::new(ExprKind::Literal(Literal::Null), Span::new(0, 4));
        assert!(l.as_ident().is_none());
    }
}
