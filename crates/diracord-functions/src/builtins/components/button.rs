//! `button` — Um botão. O clique roda o `button("id") { }` com o mesmo id.
//!
//! ```text
//! button("accept", "Accept", style: "success")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Map, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::optional(
        "id",
        ParamKind::Text,
        "id que liga ao button(\"id\") { } (não use com url)",
    ),
    ParamSpec::optional("label", ParamKind::Text, "texto do botão"),
    ParamSpec::optional(
        "style",
        ParamKind::Text,
        "primary, secondary, success, danger",
    ),
    ParamSpec::optional("emoji", ParamKind::Text, "emoji, como \"👍\""),
    ParamSpec::optional("disabled", ParamKind::Bool, "botão desligado"),
    ParamSpec::optional(
        "url",
        ParamKind::Text,
        "vira botão de link (abre o endereço)",
    ),
];

/// The `button` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "button",
    category: Category::Components,
    summary: "Um botão. O clique roda o button(\"id\") { } com o mesmo id; com url, vira link.",
    params: PARAMS,
    returns: "component",
    example: "reply row(button(\"accept\", \"Accept\", style: \"success\"), button(label: \"Site\", url: \"https://example.com\"))",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let mut obj = Map::new();
    let id = b
        .opt(0)
        .filter(|v| **v != Value::Null)
        .map(|_| b.text(0))
        .transpose()?;
    obj.insert("button".into(), id.map(Value::Text).unwrap_or(Value::Null));
    super::pieces::options(
        b,
        &mut obj,
        &[
            (1, "label"),
            (2, "style"),
            (3, "emoji"),
            (4, "disabled"),
            (5, "url"),
        ],
    );
    let v = Value::Object(obj);
    // Checked as a row with only this button, so mistakes show up here.
    super::pieces::check(&Value::object([("row", Value::List(vec![v.clone()]))]))?;
    Ok(Effect::Value(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn plain_and_link() {
        assert!(SPEC
            .invoke(Args::positional(vec!["ok".into(), "Ok".into()]))
            .is_ok());
        assert!(SPEC
            .invoke(
                Args::default()
                    .with_named("label", "Site".into())
                    .with_named("url", "https://a.b".into())
            )
            .is_ok());
        let e = SPEC
            .invoke(Args::positional(vec!["ok".into()]))
            .unwrap_err();
        assert!(e.message.contains("label"), "{e}");
        let e = SPEC
            .invoke(Args::positional(vec![
                "ok".into(),
                "Ok".into(),
                "red".into(),
            ]))
            .unwrap_err();
        assert!(e.message.contains("estilo `red`"), "{e}");
    }
}
