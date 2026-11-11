//! `row` — Uma linha com até 5 botões, ou com um menu.
//!
//! ```text
//! row(button("yes", "Yes"), button("no", "No", style: "danger"))
//! ```

use super::pieces;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[ParamSpec::rest(
    "items",
    ParamKind::Component,
    "até 5 button(...), ou um menu(...)",
)];

/// The `row` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "row",
    category: Category::Components,
    summary: "Uma linha com até 5 botões, ou com um menu.",
    params: PARAMS,
    returns: "component",
    example: "reply row(button(\"yes\", \"Yes\"), button(\"no\", \"No\", style: \"danger\"))",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let items = pieces::children(b, 0, "row")?;
    let v = Value::object([("row", Value::List(items))]);
    pieces::check(&v)?;
    Ok(Effect::Value(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    fn btn() -> Value {
        Value::object([("button", "a".into()), ("label", "A".into())])
    }

    #[test]
    fn limits() {
        assert!(SPEC.invoke(Args::positional(vec![btn(); 5])).is_ok());
        let e = SPEC.invoke(Args::positional(vec![btn(); 6])).unwrap_err();
        assert!(e.message.contains("5 botões"), "{e}");
        let e = SPEC
            .invoke(Args::positional(vec![Value::object([(
                "text",
                "x".into(),
            )])]))
            .unwrap_err();
        assert!(
            e.message
                .contains("`text(...)` não pode ficar dentro de `row(...)`"),
            "{e}"
        );
    }
}
