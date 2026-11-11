//! `ban` — Bane um membro do servidor.
//!
//! ```text
//! ban user "spam"
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Action;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("user", ParamKind::User, "quem será banido"),
    ParamSpec::optional(
        "reason",
        ParamKind::Text,
        "motivo (aparece no registro de auditoria)",
    ),
];

/// The `ban` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "ban",
    category: Category::Moderation,
    summary: "Bane um membro do servidor.",
    params: PARAMS,
    returns: "null",
    example: "ban user \"spam\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let user = b.id(0)?;
    let reason = common::opt_text(b, 1)?;
    Ok(Effect::Action(Action::new(
        "ban",
        vec![user.into(), reason],
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;
    use diracord_value::Value;

    #[test]
    fn reason_is_optional() {
        let e = SPEC.invoke(Args::positional(vec!["<@5>".into()])).unwrap();
        assert_eq!(
            e,
            Effect::Action(Action::new("ban", vec!["5".into(), Value::Null]))
        );
    }

    #[test]
    fn needs_a_user() {
        let err = SPEC
            .invoke(Args::positional(vec!["fulano".into()]))
            .unwrap_err();
        assert_eq!(err.code, diracord_diagnostics::codes::E0302);
    }
}
