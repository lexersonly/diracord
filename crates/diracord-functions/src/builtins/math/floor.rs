//! `floor` — Arredonda para baixo.
//!
//! ```text
//! floor(2.5)
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("number", ParamKind::Number, "o número")];

/// The `floor` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "floor",
    category: Category::Math,
    summary: "Arredonda para baixo.",
    params: PARAMS,
    returns: "number",
    example: "floor(2.5)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let n = b.number(0)?.floor();
    Ok(Effect::Value(common::number(n)))
}
