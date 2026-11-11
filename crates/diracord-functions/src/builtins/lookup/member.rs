//! `member` — Procura um membro do servidor pelo id, menção ou nome. Devolve null se não achar.
//!
//! ```text
//! local m = member("123")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Action, Value};

const PARAMS: &[ParamSpec] = &[ParamSpec::required(
    "query",
    ParamKind::Any,
    "nome, id ou menção",
)];

/// The `member` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "member",
    category: Category::Lookup,
    summary: "Procura um membro do servidor pelo id, menção ou nome. Devolve null se não achar.",
    params: PARAMS,
    returns: "member",
    example: "local m = member(\"123\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let query = match b.get(0) {
        v @ Value::Object(_) => v.entity_id().map(Value::from).unwrap_or(Value::Null),
        _ => b.text(0)?.into(),
    };
    Ok(Effect::Action(Action::new("member", vec![query])))
}
