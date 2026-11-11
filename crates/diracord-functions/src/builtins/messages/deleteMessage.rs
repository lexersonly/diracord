//! `deleteMessage` — Apaga a mensagem atual (ou outra, pelo id).
//!
//! ```text
//! deleteMessage
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Action, Value};

const PARAMS: &[ParamSpec] = &[ParamSpec::optional(
    "message",
    ParamKind::Any,
    "mensagem (objeto ou id); sem nada, a atual",
)];

/// The `deleteMessage` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "deleteMessage",
    category: Category::Messages,
    summary: "Apaga a mensagem atual (ou outra, pelo id).",
    params: PARAMS,
    returns: "null",
    example: "deleteMessage",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let id = match b.opt(0) {
        None | Some(Value::Null) => Value::Null,
        Some(_) => b.id(0)?.into(),
    };
    Ok(Effect::Action(Action::new("deleteMessage", vec![id])))
}
