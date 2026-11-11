//! The [`Value`] type.

use std::collections::BTreeMap;
use std::fmt;

/// Object fields, kept sorted so output is stable.
pub type Map = BTreeMap<String, Value>;

/// A runtime value. Diracord is dynamically typed; only command arguments
/// declare types.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Value {
    /// Nothing.
    #[default]
    Null,
    /// `true` / `false`.
    Bool(bool),
    /// Whole number.
    Int(i64),
    /// Decimal number.
    Float(f64),
    /// Text.
    Text(String),
    /// Ordered list.
    List(Vec<Value>),
    /// Key/value object (embeds, users, channels...).
    Object(Map),
}

impl Value {
    /// Name of the type as shown to users: `null`, `bool`, `number`, `text`, `list`, `object`.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::Int(_) | Value::Float(_) => "number",
            Value::Text(_) => "text",
            Value::List(_) => "list",
            Value::Object(_) => "object",
        }
    }

    /// Truthiness for `if`/`while`: `null`, `false`, `0`, `""` and `[]` are false.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Object(_) => true,
        }
    }

    /// The number as `f64`, if this is a number.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(i) => Some(*i as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// The number as `i64` if it is a whole number.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            Value::Float(f) if f.fract() == 0.0 && f.is_finite() => Some(*f as i64),
            _ => None,
        }
    }

    /// The text, if this is text.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }

    /// A field of an object, or `None`.
    pub fn field(&self, name: &str) -> Option<&Value> {
        match self {
            Value::Object(m) => m.get(name),
            _ => None,
        }
    }

    /// The Discord id of an entity: the `id` field of an object, or the text
    /// itself when it is already an id or mention (`<@123>` → `123`).
    pub fn entity_id(&self) -> Option<String> {
        match self {
            Value::Object(m) => m.get("id").and_then(|v| match v {
                Value::Text(s) => Some(s.clone()),
                Value::Int(i) => Some(i.to_string()),
                _ => None,
            }),
            Value::Text(s) => {
                let digits: String = s.chars().filter(char::is_ascii_digit).collect();
                let only_mention = s
                    .chars()
                    .all(|c| c.is_ascii_digit() || "<@!&#>".contains(c));
                (only_mention && !digits.is_empty()).then_some(digits)
            }
            Value::Int(i) if *i >= 0 => Some(i.to_string()),
            _ => None,
        }
    }

    /// Builds an object from pairs.
    pub fn object<K: Into<String>>(pairs: impl IntoIterator<Item = (K, Value)>) -> Value {
        Value::Object(pairs.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }

    /// Rough number of bytes this value uses, for the memory limit.
    pub fn approx_size(&self) -> usize {
        const BASE: usize = std::mem::size_of::<Value>();
        match self {
            Value::Null | Value::Bool(_) | Value::Int(_) | Value::Float(_) => BASE,
            Value::Text(s) => BASE + s.len(),
            Value::List(l) => BASE + l.iter().map(Value::approx_size).sum::<usize>(),
            Value::Object(m) => {
                BASE + m
                    .iter()
                    .map(|(k, v)| k.len() + 32 + v.approx_size())
                    .sum::<usize>()
            }
        }
    }
}

/// Text form used by string interpolation and `reply`.
///
/// - `null` → `""` (so `"Hi $nothing"` prints `"Hi "`)
/// - numbers: whole floats print without `.0`
/// - objects with a `mention` field print the mention, else `name`, else JSON-like
/// - lists print as `a, b, c`
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => Ok(()),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Float(x) => fmt_float(*x, f),
            Value::Text(s) => f.write_str(s),
            Value::List(l) => {
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{v}")?;
                }
                Ok(())
            }
            Value::Object(m) => {
                if let Some(v) = m.get("mention").or_else(|| m.get("name")) {
                    return write!(f, "{v}");
                }
                write_json(self, f)
            }
        }
    }
}

fn fmt_float(x: f64, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        write!(f, "{}", x as i64)
    } else {
        write!(f, "{x}")
    }
}

fn write_json(v: &Value, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match v {
        Value::Null => f.write_str("null"),
        Value::Text(s) => write!(f, "{s:?}"),
        Value::List(l) => {
            f.write_str("[")?;
            for (i, x) in l.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                write_json(x, f)?;
            }
            f.write_str("]")
        }
        Value::Object(m) => {
            f.write_str("{")?;
            for (i, (k, x)) in m.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{k}: ")?;
                write_json(x, f)?;
            }
            f.write_str("}")
        }
        other => write!(f, "{other}"),
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}
impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}
impl From<f64> for Value {
    fn from(x: f64) -> Self {
        Value::Float(x)
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}
impl From<Vec<Value>> for Value {
    fn from(l: Vec<Value>) -> Self {
        Value::List(l)
    }
}
impl From<Map> for Value {
    fn from(m: Map) -> Self {
        Value::Object(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truthiness() {
        assert!(!Value::Null.is_truthy());
        assert!(!Value::Int(0).is_truthy());
        assert!(!Value::from("").is_truthy());
        assert!(Value::from("a").is_truthy());
        assert!(Value::object([("a", Value::Null)]).is_truthy());
        assert!(!Value::List(vec![]).is_truthy());
    }

    #[test]
    fn display() {
        assert_eq!(Value::Null.to_string(), "");
        assert_eq!(Value::Float(2.0).to_string(), "2");
        assert_eq!(Value::Float(2.5).to_string(), "2.5");
        assert_eq!(
            Value::List(vec![1i64.into(), "a".into()]).to_string(),
            "1, a"
        );
        let user = Value::object([("name", "Ana".into()), ("mention", "<@1>".into())]);
        assert_eq!(user.to_string(), "<@1>");
        let embed = Value::object([("title", Value::from("Oi")), ("n", Value::Int(1))]);
        assert_eq!(embed.to_string(), "{n: 1, title: \"Oi\"}");
    }

    #[test]
    fn entity_ids() {
        assert_eq!(
            Value::object([("id", Value::from("42"))])
                .entity_id()
                .as_deref(),
            Some("42")
        );
        assert_eq!(Value::from("<@!123>").entity_id().as_deref(), Some("123"));
        assert_eq!(Value::from("hello").entity_id(), None);
        assert_eq!(Value::Int(7).entity_id().as_deref(), Some("7"));
    }

    #[test]
    fn numbers() {
        assert_eq!(Value::Float(3.0).as_i64(), Some(3));
        assert_eq!(Value::Float(3.5).as_i64(), None);
        assert_eq!(Value::Int(3).as_f64(), Some(3.0));
    }

    #[test]
    fn size_grows_with_content() {
        assert!(Value::from("a".repeat(100)).approx_size() > Value::from("a").approx_size());
    }
}
