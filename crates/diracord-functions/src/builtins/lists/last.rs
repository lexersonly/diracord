//! `last` — Último item da lista (ou null).
//!
//! ```text
//! last(args)
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("list", ParamKind::List, "a lista")];

/// The `last` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "last",
    category: Category::Lists,
    summary: "Último item da lista (ou null).",
    params: PARAMS,
    returns: "any",
    example: "last(args)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(
        b.list(0)?.last().cloned().unwrap_or_default(),
    ))
}
