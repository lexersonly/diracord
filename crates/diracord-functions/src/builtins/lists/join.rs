//! `join` — Junta os itens de uma lista em um texto.
//!
//! ```text
//! reply join(nomes, ", ")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("list", ParamKind::List, "a lista"),
    ParamSpec::optional("separator", ParamKind::Text, "separador (padrão \", \")"),
];

/// The `join` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "join",
    category: Category::Lists,
    summary: "Junta os itens de uma lista em um texto.",
    params: PARAMS,
    returns: "text",
    example: "reply join(nomes, \", \")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let sep = match b.opt(1) {
        None | Some(Value::Null) => ", ".to_string(),
        Some(_) => b.text(1)?,
    };
    let parts: Vec<String> = b.list(0)?.iter().map(Value::to_string).collect();
    Ok(Effect::Value(parts.join(&sep).into()))
}
