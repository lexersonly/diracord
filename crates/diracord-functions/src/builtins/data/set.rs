//! `set` — Salva um valor (fica guardado mesmo depois de reiniciar o bot).
//!
//! ```text
//! set("coins", get("coins", 0) + 1, scope: user)
//! ```

use super::super::common;
use crate::{Bound, Category, DataOp, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("name", ParamKind::Text, "nome da variável salva"),
    ParamSpec::required("value", ParamKind::Any, "o valor"),
    ParamSpec::optional("scope", ParamKind::Scope, "user, server, member ou global"),
];

/// The `set` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "set",
    category: Category::Data,
    summary: "Salva um valor (fica guardado mesmo depois de reiniciar o bot).",
    params: PARAMS,
    returns: "null",
    example: "set(\"coins\", get(\"coins\", 0) + 1, scope: user)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Data(DataOp::Set {
        name: common::data_name(b, 0)?,
        value: b.get(1).clone(),
        scope: b.scope(2)?,
    }))
}
