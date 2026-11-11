//! `timeout` — Deixa um membro de castigo (sem falar) por alguns segundos. 0 tira o castigo.
//!
//! ```text
//! timeout user 60 "flood"
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Action, Value};

/// Discord's maximum timeout: 28 days.
const MAX_SECONDS: i64 = 28 * 24 * 60 * 60;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("user", ParamKind::User, "quem"),
    ParamSpec::required(
        "seconds",
        ParamKind::Number,
        "duração em segundos (máximo 28 dias)",
    ),
    ParamSpec::optional("reason", ParamKind::Text, "motivo"),
];

/// The `timeout` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "timeout",
    category: Category::Moderation,
    summary: "Deixa um membro de castigo (sem falar) por alguns segundos. 0 tira o castigo.",
    params: PARAMS,
    returns: "null",
    example: "timeout user 60 \"flood\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let user = b.id(0)?;
    let seconds = b.int(1)?;
    if !(0..=MAX_SECONDS).contains(&seconds) {
        return Err(b.wrong(1, "um número entre 0 e 2419200 (28 dias)", b.get(1)));
    }
    let reason = common::opt_text(b, 2)?;
    Ok(Effect::Action(Action::new(
        "timeout",
        vec![user.into(), Value::Int(seconds), reason],
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn checks_range() {
        assert!(SPEC
            .invoke(Args::positional(vec!["1".into(), Value::Int(-1)]))
            .is_err());
        assert!(SPEC
            .invoke(Args::positional(vec!["1".into(), Value::Int(60)]))
            .is_ok());
    }
}
