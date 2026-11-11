//! `push` — Devolve uma nova lista com o valor no fim.
//!
//! ```text
//! list = push(list, "novo")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("list", ParamKind::List, "a lista"),
    ParamSpec::required("value", ParamKind::Any, "o valor"),
];

/// The `push` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "push",
    category: Category::Lists,
    summary: "Devolve uma nova lista com o valor no fim.",
    params: PARAMS,
    returns: "list",
    example: "list = push(list, \"novo\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let mut l = b.list(0)?.to_vec();
    l.push(b.get(1).clone());
    Ok(Effect::Value(Value::List(l)))
}
