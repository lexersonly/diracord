//! `endsWith` — Diz se o texto termina com outro.
//!
//! ```text
//! endsWith(name, ".png")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("text", ParamKind::Text, "o texto"),
    ParamSpec::required("suffix", ParamKind::Text, "o fim"),
];

/// The `endsWith` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "endsWith",
    category: Category::Text,
    summary: "Diz se o texto termina com outro.",
    params: PARAMS,
    returns: "bool",
    example: "endsWith(name, \".png\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(Value::Bool(
        b.text(0)?.ends_with(&b.text(1)?),
    )))
}
