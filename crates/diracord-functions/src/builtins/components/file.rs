//! `file` — Mostra um arquivo enviado junto (`attachment://nome`).
//!
//! ```text
//! file("attachment://report.pdf")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Map, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("url", ParamKind::Text, "\"attachment://nome-do-arquivo\""),
    ParamSpec::optional("spoiler", ParamKind::Bool, "esconde até clicar"),
];

/// The `file` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "file",
    category: Category::Components,
    summary: "Mostra um arquivo enviado junto (attachment://nome).",
    params: PARAMS,
    returns: "component",
    example: "reply file(\"attachment://report.pdf\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let mut obj = Map::new();
    obj.insert("file".into(), b.text(0)?.into());
    super::pieces::options(b, &mut obj, &[(1, "spoiler")]);
    let v = Value::Object(obj);
    super::pieces::check(&v)?;
    Ok(Effect::Value(v))
}
