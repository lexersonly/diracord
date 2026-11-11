//! Operators on values, shared by the interpreter and the built-in functions.
//!
//! Rules (kept simple on purpose):
//! - `+` adds numbers; if either side is text, it joins the texts; lists concatenate.
//! - `- * / %` only work on numbers. `/` between whole numbers gives a decimal
//!   only when needed (`7 / 2 = 3.5`, `6 / 2 = 3`).
//! - `==` compares numbers by value (`1 == 1.0`), everything else structurally.
//! - `< <= > >=` work on two numbers or two texts.

use crate::Value;
use std::cmp::Ordering;
use std::fmt;

/// Why an operation failed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpError {
    /// Operand types do not fit the operator.
    TypeMismatch {
        /// Operator symbol, e.g. `"-"`.
        op: &'static str,
        /// Left type name.
        left: &'static str,
        /// Right type name (empty for unary operators).
        right: &'static str,
    },
    /// `/` or `%` by zero.
    DivisionByZero,
    /// A whole-number result does not fit in 64 bits.
    Overflow,
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpError::TypeMismatch {
                op,
                left,
                right: "",
            } => {
                write!(f, "não dá para usar `{op}` com {left}")
            }
            OpError::TypeMismatch { op, left, right } => {
                write!(f, "não dá para usar `{op}` entre {left} e {right}")
            }
            OpError::DivisionByZero => f.write_str("divisão por zero"),
            OpError::Overflow => f.write_str("o número ficou grande demais"),
        }
    }
}

impl std::error::Error for OpError {}

type R = Result<Value, OpError>;

fn mismatch(op: &'static str, a: &Value, b: &Value) -> OpError {
    OpError::TypeMismatch {
        op,
        left: a.type_name(),
        right: b.type_name(),
    }
}

fn num_op(
    op: &'static str,
    a: &Value,
    b: &Value,
    int: fn(i64, i64) -> Option<i64>,
    float: fn(f64, f64) -> f64,
) -> R {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => int(*x, *y).map(Value::Int).ok_or(OpError::Overflow),
        _ => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) => Ok(Value::Float(float(x, y))),
            _ => Err(mismatch(op, a, b)),
        },
    }
}

/// `a + b`
pub fn add(a: &Value, b: &Value) -> R {
    match (a, b) {
        (Value::Text(x), y) => Ok(Value::Text(format!("{x}{y}"))),
        (x, Value::Text(y)) => Ok(Value::Text(format!("{x}{y}"))),
        (Value::List(x), Value::List(y)) => Ok(Value::List(x.iter().chain(y).cloned().collect())),
        _ => num_op("+", a, b, i64::checked_add, |x, y| x + y),
    }
}

/// `a - b`
pub fn sub(a: &Value, b: &Value) -> R {
    num_op("-", a, b, i64::checked_sub, |x, y| x - y)
}

/// `a * b`
pub fn mul(a: &Value, b: &Value) -> R {
    num_op("*", a, b, i64::checked_mul, |x, y| x * y)
}

/// `a / b`
pub fn div(a: &Value, b: &Value) -> R {
    if b.as_f64() == Some(0.0) && a.as_f64().is_some() {
        return Err(OpError::DivisionByZero);
    }
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => match (x.checked_rem(*y), x.checked_div(*y)) {
            (Some(0), Some(q)) => Ok(Value::Int(q)),
            _ => Ok(Value::Float(*x as f64 / *y as f64)),
        },
        _ => num_op("/", a, b, |_, _| None, |x, y| x / y),
    }
}

/// `a % b`
pub fn rem(a: &Value, b: &Value) -> R {
    if b.as_f64() == Some(0.0) && a.as_f64().is_some() {
        return Err(OpError::DivisionByZero);
    }
    num_op("%", a, b, i64::checked_rem, |x, y| x % y)
}

/// `-a`
pub fn neg(a: &Value) -> R {
    match a {
        Value::Int(x) => x.checked_neg().map(Value::Int).ok_or(OpError::Overflow),
        Value::Float(x) => Ok(Value::Float(-x)),
        _ => Err(OpError::TypeMismatch {
            op: "-",
            left: a.type_name(),
            right: "",
        }),
    }
}

/// `!a` / `not a` (never fails).
pub fn not(a: &Value) -> Value {
    Value::Bool(!a.is_truthy())
}

/// `a == b`
pub fn equals(a: &Value, b: &Value) -> bool {
    match (a.as_f64(), b.as_f64()) {
        (Some(x), Some(y)) => x == y,
        _ => a == b,
    }
}

/// Ordering for `< <= > >=`. Only number–number and text–text compare.
pub fn compare(op: &'static str, a: &Value, b: &Value) -> Result<Ordering, OpError> {
    match (a, b) {
        (Value::Text(x), Value::Text(y)) => Ok(x.cmp(y)),
        _ => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) => x.partial_cmp(&y).ok_or(mismatch(op, a, b)),
            _ => Err(mismatch(op, a, b)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i(x: i64) -> Value {
        Value::Int(x)
    }

    #[test]
    fn arithmetic() {
        assert_eq!(add(&i(1), &i(2)), Ok(i(3)));
        assert_eq!(add(&i(1), &Value::Float(0.5)), Ok(Value::Float(1.5)));
        assert_eq!(sub(&i(5), &i(7)), Ok(i(-2)));
        assert_eq!(mul(&i(3), &i(4)), Ok(i(12)));
        assert_eq!(div(&i(6), &i(2)), Ok(i(3)));
        assert_eq!(div(&i(7), &i(2)), Ok(Value::Float(3.5)));
        assert_eq!(rem(&i(7), &i(3)), Ok(i(1)));
        assert_eq!(neg(&i(3)), Ok(i(-3)));
    }

    #[test]
    fn text_concat() {
        assert_eq!(add(&"a".into(), &i(1)), Ok("a1".into()));
        assert_eq!(add(&i(1), &"a".into()), Ok("1a".into()));
    }

    #[test]
    fn errors() {
        assert_eq!(div(&i(1), &i(0)), Err(OpError::DivisionByZero));
        assert_eq!(rem(&i(1), &Value::Float(0.0)), Err(OpError::DivisionByZero));
        let e = sub(&i(1), &"a".into()).unwrap_err();
        assert_eq!(e.to_string(), "não dá para usar `-` entre number e text");
        assert_eq!(add(&i(i64::MAX), &i(1)), Err(OpError::Overflow));
        assert_eq!(
            div(&i(i64::MIN), &i(-1)),
            Ok(Value::Float(i64::MIN as f64 / -1.0))
        );
    }

    #[test]
    fn comparison() {
        assert!(equals(&i(1), &Value::Float(1.0)));
        assert!(!equals(&i(1), &"1".into()));
        assert_eq!(compare("<", &i(1), &i(2)), Ok(Ordering::Less));
        assert_eq!(
            compare("<", &"b".into(), &"a".into()),
            Ok(Ordering::Greater)
        );
        assert!(compare("<", &i(1), &"a".into()).is_err());
        assert_eq!(not(&Value::Null), Value::Bool(true));
    }
}
