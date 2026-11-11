//! `toNumber` — Converte texto em número. Devolve null se não for número.
//!
//! ```text
//! local n = toNumber("42")
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[ParamSpec::required("value", ParamKind::Any, "o valor")];

/// The `toNumber` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "toNumber",
    category: Category::Utility,
    summary: "Converte texto em número. Devolve null se não for número.",
    params: PARAMS,
    returns: "number",
    example: "local n = toNumber(\"42\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(match b.get(0) {
        v @ (Value::Int(_) | Value::Float(_)) => v.clone(),
        Value::Bool(x) => Value::Int(i64::from(*x)),
        Value::Text(s) => {
            let s = s.trim();
            s.parse::<i64>()
                .map(Value::Int)
                .or_else(|_| s.parse::<f64>().map(common::number))
                .unwrap_or(Value::Null)
        }
        _ => Value::Null,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn converts() {
        assert_eq!(
            SPEC.invoke(Args::positional(vec![" 42 ".into()])).unwrap(),
            Effect::Value(Value::Int(42))
        );
        assert_eq!(
            SPEC.invoke(Args::positional(vec!["1.5".into()])).unwrap(),
            Effect::Value(Value::Float(1.5))
        );
        assert_eq!(
            SPEC.invoke(Args::positional(vec!["x".into()])).unwrap(),
            Effect::Value(Value::Null)
        );
    }
}
