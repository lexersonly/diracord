//! `dm` — Envia uma mensagem privada para um usuário.
//!
//! ```text
//! dm user "Bem-vindo!"
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Action;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("user", ParamKind::User, "usuário (objeto, id ou menção)"),
    ParamSpec::required("content", ParamKind::Content, "texto ou embed"),
];

/// The `dm` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "dm",
    category: Category::Messages,
    summary: "Envia uma mensagem privada para um usuário.",
    params: PARAMS,
    returns: "message",
    example: "dm user \"Bem-vindo!\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let user = b.id(0)?;
    let content = common::content(b, 1)?;
    Ok(Effect::Action(Action::new(
        "dm",
        vec![user.into(), content],
    )))
}
