//! `round` — Arredonda para o inteiro mais próximo.
//!
//! ```text
//! round(2.5)
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("number", ParamKind::Number, "o número")];

/// The `round` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "round",
    category: Category::Math,
    summary: "Arredonda para o inteiro mais próximo.",
    params: PARAMS,
    returns: "number",
    example: "round(2.5)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let n = b.number(0)?.round();
    Ok(Effect::Value(common::number(n)))
}
