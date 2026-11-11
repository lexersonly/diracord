//! `gallery` — De 1 a 10 imagens grandes.
//!
//! ```text
//! gallery("https://a.png", "https://b.png")
//! ```

use super::pieces;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[ParamSpec::rest(
    "images",
    ParamKind::Any,
    "de 1 a 10 urls (ou { url, description, spoiler })",
)];

/// The `gallery` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "gallery",
    category: Category::Components,
    summary: "De 1 a 10 imagens grandes.",
    params: PARAMS,
    returns: "component",
    example: "reply gallery(\"https://example.com/a.png\", \"https://example.com/b.png\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    // `gallery(list)` also works: one list argument is spread.
    let items = match b.list(0)? {
        [Value::List(inner)] => inner.clone(),
        many => many.to_vec(),
    };
    let v = Value::object([("gallery", Value::List(items))]);
    pieces::check(&v)?;
    Ok(Effect::Value(v))
}
