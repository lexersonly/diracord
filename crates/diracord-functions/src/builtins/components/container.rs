//! `container` — Caixa com cor de destaque que agrupa outras peças.
//!
//! ```text
//! container(text("## Hello"), row(button("ok", "Ok")), color: "purple")
//! ```

use super::pieces;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Map, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::rest(
        "items",
        ParamKind::Component,
        "peças: text, section, row, separator, gallery, file",
    ),
    ParamSpec::optional(
        "color",
        ParamKind::Text,
        "cor da barra: \"purple\", \"#ff0000\"...",
    ),
    ParamSpec::optional("spoiler", ParamKind::Bool, "esconde até clicar"),
];

/// The `container` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "container",
    category: Category::Components,
    summary: "Caixa com cor de destaque que agrupa outras peças.",
    params: PARAMS,
    returns: "component",
    example: "reply container(text(\"## Hello\"), row(button(\"ok\", \"Ok\")), color: \"purple\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let items = pieces::children(b, 0, "container")?;
    let mut obj = Map::new();
    obj.insert("container".into(), Value::List(items));
    pieces::options(b, &mut obj, &[(1, "color"), (2, "spoiler")]);
    let v = Value::Object(obj);
    pieces::check(&v)?;
    Ok(Effect::Value(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArgRef, Args};

    fn text(s: &str) -> Value {
        Value::object([("text", s.into())])
    }

    #[test]
    fn builds_and_points_at_wrong_child() {
        let ok = SPEC
            .invoke(Args::positional(vec![text("a")]).with_named("color", "red".into()))
            .unwrap();
        assert_eq!(
            ok,
            Effect::Value(Value::object([
                ("color", "red".into()),
                ("container", Value::List(vec![text("a")]))
            ]))
        );
        let button = Value::object([("button", "x".into()), ("label", "X".into())]);
        let e = SPEC
            .invoke(Args::positional(vec![text("a"), button]))
            .unwrap_err();
        assert_eq!(e.arg, Some(ArgRef::Positional(1)));
        assert!(
            e.message
                .contains("`button(...)` não pode ficar dentro de `container(...)`"),
            "{e}"
        );
        let e = SPEC
            .invoke(Args::positional(vec!["oi".into()]))
            .unwrap_err();
        assert!(e.help.unwrap().contains("text(\"...\")"));
        assert!(SPEC.invoke(Args::default()).is_err());
    }
}
