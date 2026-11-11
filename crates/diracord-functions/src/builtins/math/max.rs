//! `max` — O maior de dois números.
//!
//! ```text
//! max(5, 3)
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("a", ParamKind::Number, "primeiro"),
    ParamSpec::required("b", ParamKind::Number, "segundo"),
];

/// The `max` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "max",
    category: Category::Math,
    summary: "O maior de dois números.",
    params: PARAMS,
    returns: "number",
    example: "max(5, 3)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(if b.number(0)? >= b.number(1)? {
        b.get(0).clone()
    } else {
        b.get(1).clone()
    }))
}
