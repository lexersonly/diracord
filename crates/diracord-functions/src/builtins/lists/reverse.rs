//! `reverse` — Inverte uma lista ou um texto.
//!
//! ```text
//! reverse([1, 2, 3])
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[ParamSpec::required(
    "value",
    ParamKind::Any,
    "lista ou texto",
)];

/// The `reverse` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "reverse",
    category: Category::Lists,
    summary: "Inverte uma lista ou um texto.",
    params: PARAMS,
    returns: "any",
    example: "reverse([1, 2, 3])",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(match b.get(0) {
        Value::List(l) => Value::List(l.iter().rev().cloned().collect()),
        _ => b.text(0)?.chars().rev().collect::<String>().into(),
    }))
}
