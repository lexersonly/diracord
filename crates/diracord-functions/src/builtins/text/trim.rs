//! `trim` — Tira espaços do começo e do fim.
//!
//! ```text
//! local t = trim("  oi ")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("text", ParamKind::Text, "o texto")];

/// The `trim` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "trim",
    category: Category::Text,
    summary: "Tira espaços do começo e do fim.",
    params: PARAMS,
    returns: "text",
    example: "local t = trim(\"  oi \")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(b.text(0)?.trim().to_string().into()))
}
