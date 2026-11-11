//! `role` — Procura um cargo pelo nome, id ou menção. Devolve null se não achar.
//!
//! ```text
//! local vip = role("VIP")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Action, Value};

const PARAMS: &[ParamSpec] = &[ParamSpec::required(
    "query",
    ParamKind::Any,
    "nome, id ou menção",
)];

/// The `role` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "role",
    category: Category::Lookup,
    summary: "Procura um cargo pelo nome, id ou menção. Devolve null se não achar.",
    params: PARAMS,
    returns: "role",
    example: "local vip = role(\"VIP\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let query = match b.get(0) {
        v @ Value::Object(_) => v.entity_id().map(Value::from).unwrap_or(Value::Null),
        _ => b.text(0)?.into(),
    };
    Ok(Effect::Action(Action::new("role", vec![query])))
}
