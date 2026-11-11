//! `hasRole` — Diz se um membro tem um cargo (true/false).
//!
//! ```text
//! if hasRole(user, "VIP") { reply "vip!" }
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

/// The `hasRole` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "hasRole",
    category: Category::Roles,
    summary: "Diz se um membro tem um cargo (true/false).",
    params: PARAMS,
    returns: "bool",
    example: "if hasRole(user, \"VIP\") { reply \"vip!\" }",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let user = b.id(0)?;
    let role = common::role_ref(b, 1)?;
    Ok(Effect::Action(Action::new(
        "hasRole",
        vec![user.into(), role],
    )))
}
