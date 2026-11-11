//! `kick` — Expulsa um membro do servidor.
//!
//! ```text
//! kick user "regras"
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Action;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("user", ParamKind::User, "quem será expulso"),
    ParamSpec::optional("reason", ParamKind::Text, "motivo"),
];

/// The `kick` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "kick",
    category: Category::Moderation,
    summary: "Expulsa um membro do servidor.",
    params: PARAMS,
    returns: "null",
    example: "kick user \"regras\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let user = b.id(0)?;
    let reason = common::opt_text(b, 1)?;
    Ok(Effect::Action(Action::new(
        "kick",
        vec![user.into(), reason],
    )))
}
