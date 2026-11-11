//! `min` — O menor de dois números.
//!
//! ```text
//! min(5, 3)
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("a", ParamKind::Number, "primeiro"),
    ParamSpec::required("b", ParamKind::Number, "segundo"),
];

/// The `min` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "min",
    category: Category::Math,
    summary: "O menor de dois números.",
    params: PARAMS,
    returns: "number",
    example: "min(5, 3)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(if b.number(0)? <= b.number(1)? {
        b.get(0).clone()
    } else {
        b.get(1).clone()
    }))
}
