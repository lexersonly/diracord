//! Requests the engine sends to the host.

use crate::{Map, Value};

/// Something the host must do on Discord, such as `reply` or `ban`.
///
/// Built-in functions validate and normalize their arguments, then return an
/// `Action`; the interpreter passes it to the host, which performs it and
/// returns a [`Value`] (for example the sent message). Keeping actions as
/// name + values makes the boundary easy to serialize (JSON, napi, socket).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Action {
    /// Action name, e.g. `"reply"`, `"ban"`, `"send"`.
    pub name: String,
    /// Positional arguments, already validated by the function.
    pub args: Vec<Value>,
    /// Named options (e.g. `ephemeral: true`).
    pub options: Map,
}

impl Action {
    /// Creates an action with positional arguments.
    pub fn new(name: impl Into<String>, args: Vec<Value>) -> Self {
        Action { name: name.into(), args, options: Map::new() }
    }

    /// Adds a named option.
    pub fn with_option(mut self, key: impl Into<String>, value: Value) -> Self {
        self.options.insert(key.into(), value);
        self
    }

    /// Positional argument `i`, or `null`.
    pub fn arg(&self, i: usize) -> &Value {
        const NULL: Value = Value::Null;
        self.args.get(i).unwrap_or(&NULL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let a = Action::new("reply", vec!["oi".into()]).with_option("ephemeral", true.into());
        assert_eq!(a.arg(0), &Value::from("oi"));
        assert_eq!(a.arg(5), &Value::Null);
        assert_eq!(a.options.get("ephemeral"), Some(&Value::Bool(true)));
    }
}
