//! `startsWith` — Diz se o texto começa com outro.
//!
//! ```text
//! if startsWith(message.content, "!") { stop }
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("text", ParamKind::Text, "o texto"),
    ParamSpec::required("prefix", ParamKind::Text, "o começo"),
];

/// The `startsWith` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "startsWith",
    category: Category::Text,
    summary: "Diz se o texto começa com outro.",
    params: PARAMS,
    returns: "bool",
    example: "if startsWith(message.content, \"!\") { stop }",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(Value::Bool(
        b.text(0)?.starts_with(&b.text(1)?),
    )))
}
