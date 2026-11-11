//! `contains` — Diz se um texto contém outro, ou se uma lista contém um valor.
//!
//! ```text
//! if contains(message.content, "palavrão") { deleteMessage }
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{ops, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("haystack", ParamKind::Any, "texto ou lista"),
    ParamSpec::required("needle", ParamKind::Any, "o que procurar"),
];

/// The `contains` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "contains",
    category: Category::Text,
    summary: "Diz se um texto contém outro, ou se uma lista contém um valor.",
    params: PARAMS,
    returns: "bool",
    example: "if contains(message.content, \"palavrão\") { deleteMessage }",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let found = match b.get(0) {
        Value::List(l) => l.iter().any(|v| ops::equals(v, b.get(1))),
        Value::Object(m) => m.contains_key(&b.text(1)?),
        _ => b.text(0)?.contains(&b.text(1)?),
    };
    Ok(Effect::Value(Value::Bool(found)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn text_and_list() {
        let t = SPEC
            .invoke(Args::positional(vec!["abc".into(), "b".into()]))
            .unwrap();
        assert_eq!(t, Effect::Value(Value::Bool(true)));
        let l = SPEC
            .invoke(Args::positional(vec![
                Value::List(vec![Value::Int(1)]),
                Value::Float(1.0),
            ]))
            .unwrap();
        assert_eq!(l, Effect::Value(Value::Bool(true)));
    }
}
