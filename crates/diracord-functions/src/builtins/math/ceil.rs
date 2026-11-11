//! `ceil` — Arredonda para cima.
//!
//! ```text
//! ceil(2.5)
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("number", ParamKind::Number, "o número")];

/// The `ceil` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "ceil",
    category: Category::Math,
    summary: "Arredonda para cima.",
    params: PARAMS,
    returns: "number",
    example: "ceil(2.5)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let n = b.number(0)?.ceil();
    Ok(Effect::Value(common::number(n)))
}
