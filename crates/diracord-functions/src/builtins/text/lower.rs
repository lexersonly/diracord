//! `lower` — Texto em minúsculas.
//!
//! ```text
//! local t = lower(message.content)
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("text", ParamKind::Text, "o texto")];

/// The `lower` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "lower",
    category: Category::Text,
    summary: "Texto em minúsculas.",
    params: PARAMS,
    returns: "text",
    example: "local t = lower(message.content)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(b.text(0)?.to_lowercase().into()))
}
