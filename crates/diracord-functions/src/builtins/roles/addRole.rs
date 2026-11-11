//! `addRole` — Dá um cargo para um membro.
//!
//! ```text
//! addRole user "VIP"
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Action;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("user", ParamKind::User, "o membro"),
    ParamSpec::required(
        "role",
        ParamKind::Role,
        "cargo (objeto, id, menção ou nome)",
    ),
];

/// The `addRole` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "addRole",
    category: Category::Roles,
    summary: "Dá um cargo para um membro.",
    params: PARAMS,
    returns: "null",
    example: "addRole user \"VIP\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let user = b.id(0)?;
    let role = common::role_ref(b, 1)?;
    Ok(Effect::Action(Action::new(
        "addRole",
        vec![user.into(), role],
    )))
}
