//! `text` — Texto (com markdown) dentro do layout.
//!
//! ```text
//! text("## Hello, $user.name")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[ParamSpec::required(
    "content",
    ParamKind::Text,
    "o texto; aceita markdown (## título, **negrito**)",
)];

/// The `text` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "text",
    category: Category::Components,
    summary: "Texto (com markdown) dentro do layout.",
    params: PARAMS,
    returns: "component",
    example: "reply text(\"## Hello, $user.name\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let t = b.text(0)?;
    if t.is_empty() {
        return Err(b.wrong(0, "um texto não vazio", b.get(0)));
    }
    if t.chars().count() > 4000 {
        return Err(b.wrong(0, "um texto de até 4000 letras", b.get(0)));
    }
    Ok(Effect::Value(Value::object([("text", t.into())])))
}
