//! `unban` — Tira o banimento de um usuário.
//!
//! ```text
//! unban "123456789"
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Action;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("user", ParamKind::User, "usuário (id)"),
    ParamSpec::optional("reason", ParamKind::Text, "motivo"),
];

/// The `unban` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "unban",
    category: Category::Moderation,
    summary: "Tira o banimento de um usuário.",
    params: PARAMS,
    returns: "null",
    example: "unban \"123456789\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let user = b.id(0)?;
    let reason = common::opt_text(b, 1)?;
    Ok(Effect::Action(Action::new(
        "unban",
        vec![user.into(), reason],
    )))
}
