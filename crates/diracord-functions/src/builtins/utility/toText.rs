//! `toText` — Converte qualquer valor em texto.
//!
//! ```text
//! toText(42)
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("value", ParamKind::Any, "o valor")];

/// The `toText` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "toText",
    category: Category::Utility,
    summary: "Converte qualquer valor em texto.",
    params: PARAMS,
    returns: "text",
    example: "toText(42)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(b.get(0).to_string().into()))
}
