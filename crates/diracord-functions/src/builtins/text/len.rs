//! `len` — Tamanho de um texto (em letras) ou de uma lista.
//!
//! ```text
//! if len(reason) > 100 { stop }
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[ParamSpec::required(
    "value",
    ParamKind::Any,
    "texto ou lista",
)];

/// The `len` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "len",
    category: Category::Text,
    summary: "Tamanho de um texto (em letras) ou de uma lista.",
    params: PARAMS,
    returns: "number",
    example: "if len(reason) > 100 { stop }",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let n = match b.get(0) {
        Value::List(l) => l.len(),
        Value::Object(m) => m.len(),
        Value::Null => 0,
        _ => b.text(0)?.chars().count(),
    };
    Ok(Effect::Value(Value::Int(n as i64)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn counts_chars_and_items() {
        assert_eq!(
            SPEC.invoke(Args::positional(vec!["ação".into()])).unwrap(),
            Effect::Value(Value::Int(4))
        );
        assert_eq!(
            SPEC.invoke(Args::positional(vec![Value::List(vec![Value::Null])]))
                .unwrap(),
            Effect::Value(Value::Int(1))
        );
    }
}
