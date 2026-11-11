//! `delete` — Apaga um valor salvo. Devolve true se existia.
//!
//! ```text
//! delete("coins", scope: user)
//! ```

use super::super::common;
use crate::{Bound, Category, DataOp, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("name", ParamKind::Text, "nome da variável salva"),
    ParamSpec::optional("scope", ParamKind::Scope, "user, server, member ou global"),
];

/// The `delete` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "delete",
    category: Category::Data,
    summary: "Apaga um valor salvo. Devolve true se existia.",
    params: PARAMS,
    returns: "bool",
    example: "delete(\"coins\", scope: user)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Data(DataOp::Delete {
        name: common::data_name(b, 0)?,
        scope: b.scope(1)?,
    }))
}
