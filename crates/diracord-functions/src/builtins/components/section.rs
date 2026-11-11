//! `section` — 1 a 3 textos com uma imagem pequena ou um botão ao lado.
//!
//! ```text
//! section("Read the rules first.", thumbnail: user.avatar)
//! ```

use super::pieces;
use crate::layout::kind_of;
use crate::{ArgRef, Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_diagnostics::codes;
use diracord_value::{Map, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::rest("texts", ParamKind::Text, "de 1 a 3 textos (ou text(...))"),
    ParamSpec::optional(
        "thumbnail",
        ParamKind::Any,
        "imagem ao lado: url ou thumbnail(...)",
    ),
    ParamSpec::optional(
        "button",
        ParamKind::Component,
        "ou um botão ao lado: button(...)",
    ),
];

/// The `section` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "section",
    category: Category::Components,
    summary: "1 a 3 textos com uma imagem pequena (thumbnail) ou um botão ao lado.",
    params: PARAMS,
    returns: "component",
    example: "reply container(section(\"Read the rules first.\", thumbnail: user.avatar))",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let mut texts = Vec::new();
    for (j, t) in b.list(0)?.iter().enumerate() {
        let s = match (kind_of(t), t) {
            (Some("text"), _) => t.field("text").cloned().unwrap_or_default(),
            (None, Value::Text(_) | Value::Int(_) | Value::Float(_)) => Value::Text(t.to_string()),
            (Some(k), _) => {
                return Err(FnError::new(codes::E0117, format!("`{k}(...)` não pode ser texto de `section(...)`"))
                    .at(ArgRef::Positional(j))
                    .with_help("os argumentos sem nome da section são textos; o botão vai em `button:` e a imagem em `thumbnail:`"))
            }
            _ => return Err(FnError::new(codes::E0302, "os argumentos sem nome da section precisam ser textos").at(ArgRef::Positional(j))),
        };
        texts.push(s);
    }
    let mut obj = Map::new();
    obj.insert("section".into(), Value::List(texts));
    if let Some(t) = b.opt(1).filter(|v| **v != Value::Null) {
        let thumb = match t {
            Value::Object(m) if m.contains_key("thumbnail") => {
                let mut m = m.clone();
                let url = m.remove("thumbnail").unwrap_or_default();
                if m.is_empty() {
                    url
                } else {
                    m.insert("url".into(), url);
                    Value::Object(m)
                }
            }
            other => other.clone(),
        };
        obj.insert("thumbnail".into(), thumb);
    }
    if let Some(btn) = b.opt(2).filter(|v| **v != Value::Null) {
        if kind_of(btn) != Some("button") {
            return Err(FnError::new(
                codes::E0117,
                "`button:` da section precisa ser um button(...)",
            )
            .at(ArgRef::Named("button".into())));
        }
        obj.insert("button".into(), btn.clone());
    }
    let v = Value::Object(obj);
    pieces::check(&v)?;
    Ok(Effect::Value(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn needs_one_accessory() {
        let e = SPEC.invoke(Args::positional(vec!["a".into()])).unwrap_err();
        assert!(e.message.contains("acessório"), "{e}");
        let ok = SPEC
            .invoke(
                Args::positional(vec!["a".into(), Value::object([("text", "b".into())])])
                    .with_named("thumbnail", "https://x/a.png".into()),
            )
            .unwrap();
        assert_eq!(
            ok,
            Effect::Value(Value::object([
                ("section", Value::List(vec!["a".into(), "b".into()])),
                ("thumbnail", "https://x/a.png".into())
            ]))
        );
    }
}
