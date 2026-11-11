//! `split` — Divide um texto em uma lista. Sem separador, divide nos espaços.
//!
//! ```text
//! local words = split(message.content)
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("text", ParamKind::Text, "o texto"),
    ParamSpec::optional("separator", ParamKind::Text, "separador"),
];

/// The `split` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "split",
    category: Category::Text,
    summary: "Divide um texto em uma lista. Sem separador, divide nos espaços.",
    params: PARAMS,
    returns: "list",
    example: "local words = split(message.content)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let text = b.text(0)?;
    let parts: Vec<Value> = match b.opt(1) {
        None | Some(Value::Null) => text.split_whitespace().map(Value::from).collect(),
        Some(_) => {
            let sep = b.text(1)?;
            if sep.is_empty() {
                text.chars().map(|c| Value::from(c.to_string())).collect()
            } else {
                text.split(sep.as_str()).map(Value::from).collect()
            }
        }
    };
    Ok(Effect::Value(Value::List(parts)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn splits() {
        let e = SPEC
            .invoke(Args::positional(vec!["a  b c".into()]))
            .unwrap();
        assert_eq!(
            e,
            Effect::Value(Value::List(vec!["a".into(), "b".into(), "c".into()]))
        );
        let e = SPEC
            .invoke(Args::positional(vec!["a,b".into(), ",".into()]))
            .unwrap();
        assert_eq!(e, Effect::Value(Value::List(vec!["a".into(), "b".into()])));
    }
}
