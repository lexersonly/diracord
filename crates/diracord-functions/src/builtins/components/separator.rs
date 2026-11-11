//! `separator` — Espaço ou linha entre peças.
//!
//! ```text
//! separator(spacing: "large")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Map, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::optional("spacing", ParamKind::Text, "\"small\" ou \"large\""),
    ParamSpec::optional("line", ParamKind::Bool, "mostra a linha (padrão: true)"),
];

/// The `separator` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "separator",
    category: Category::Components,
    summary: "Espaço ou linha entre peças.",
    params: PARAMS,
    returns: "component",
    example: "reply container(text(\"a\"), separator(spacing: \"large\"), text(\"b\"))",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let mut obj = Map::new();
    obj.insert("separator".into(), Value::Bool(true));
    super::pieces::options(b, &mut obj, &[(0, "spacing"), (1, "line")]);
    let v = Value::Object(obj);
    super::pieces::check(&v)?;
    Ok(Effect::Value(v))
}
