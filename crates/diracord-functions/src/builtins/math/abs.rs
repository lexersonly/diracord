//! `abs` — Valor absoluto (sem sinal).
//!
//! ```text
//! abs(-3)
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[ParamSpec::required("number", ParamKind::Number, "o número")];

/// The `abs` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "abs",
    category: Category::Math,
    summary: "Valor absoluto (sem sinal).",
    params: PARAMS,
    returns: "number",
    example: "abs(-3)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(match b.get(0) {
        Value::Int(i) => Value::Int(i.saturating_abs()),
        _ => common::number(b.number(0)?.abs()),
    }))
}
