//! `typeOf` — Nome do tipo de um valor: null, bool, number, text, list ou object.
//!
//! ```text
//! reply typeOf(x)
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("value", ParamKind::Any, "o valor")];

/// The `typeOf` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "typeOf",
    category: Category::Utility,
    summary: "Nome do tipo de um valor: null, bool, number, text, list ou object.",
    params: PARAMS,
    returns: "text",
    example: "reply typeOf(x)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(b.get(0).type_name().into()))
}
