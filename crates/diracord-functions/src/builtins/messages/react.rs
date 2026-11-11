//! `react` — Reage à mensagem atual com um emoji.
//!
//! ```text
//! react "👍"
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Action;

const PARAMS: &[ParamSpec] = &[ParamSpec::required(
    "emoji",
    ParamKind::Text,
    "emoji, como \"👍\" ou <:nome:id>",
)];

/// The `react` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "react",
    category: Category::Messages,
    summary: "Reage à mensagem atual com um emoji.",
    params: PARAMS,
    returns: "null",
    example: "react \"👍\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Action(Action::new(
        "react",
        vec![b.text(0)?.into()],
    )))
}
