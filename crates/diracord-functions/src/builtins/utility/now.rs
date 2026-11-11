//! `now` — Hora atual em milissegundos desde 1970 (Unix).
//!
//! ```text
//! local start = now()
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[];

/// The `now` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "now",
    category: Category::Utility,
    summary: "Hora atual em milissegundos desde 1970 (Unix).",
    params: PARAMS,
    returns: "number",
    example: "local start = now()",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    let _ = b;
    Ok(Effect::Value(Value::Int(ms)))
}
