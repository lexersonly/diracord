//! `channel` — Procura um canal do servidor pelo nome, id ou menção. Devolve null se não achar.
//!
//! ```text
//! send channel("welcome") "Oi"
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Action, Value};

const PARAMS: &[ParamSpec] = &[ParamSpec::required(
    "query",
    ParamKind::Any,
    "nome, id ou menção",
)];

/// The `channel` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "channel",
    category: Category::Lookup,
    summary: "Procura um canal do servidor pelo nome, id ou menção. Devolve null se não achar.",
    params: PARAMS,
    returns: "channel",
    example: "send channel(\"welcome\") \"Oi\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let query = match b.get(0) {
        v @ Value::Object(_) => v.entity_id().map(Value::from).unwrap_or(Value::Null),
        _ => b.text(0)?.into(),
    };
    Ok(Effect::Action(Action::new("channel", vec![query])))
}
