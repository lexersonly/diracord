//! `first` — Primeiro item da lista (ou null).
//!
//! ```text
//! first(args)
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("list", ParamKind::List, "a lista")];

/// The `first` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "first",
    category: Category::Lists,
    summary: "Primeiro item da lista (ou null).",
    params: PARAMS,
    returns: "any",
    example: "first(args)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(
        b.list(0)?.first().cloned().unwrap_or_default(),
    ))
}
