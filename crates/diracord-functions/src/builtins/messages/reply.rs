//! `reply` — Responde à mensagem ou interação atual. Aceita texto ou embed.
//!
//! ```text
//! reply "Pong!"
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Action, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required(
        "content",
        ParamKind::Content,
        "texto ou embed { title: ... }",
    ),
    ParamSpec::optional(
        "ephemeral",
        ParamKind::Bool,
        "só quem usou o comando vê (slash)",
    ),
];

/// The `reply` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "reply",
    category: Category::Messages,
    summary: "Responde à mensagem ou interação atual. Aceita texto ou embed.",
    params: PARAMS,
    returns: "message",
    example: "reply \"Pong!\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let content = common::content(b, 0)?;
    let mut action = Action::new("reply", vec![content]);
    if let Some(e) = b.opt(1) {
        action = action.with_option("ephemeral", Value::Bool(e.is_truthy()));
    }
    Ok(Effect::Action(action))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn makes_reply_action() {
        let e = SPEC.invoke(Args::positional(vec!["Pong!".into()])).unwrap();
        assert_eq!(
            e,
            Effect::Action(Action::new("reply", vec!["Pong!".into()]))
        );
    }

    #[test]
    fn rejects_lists() {
        assert!(SPEC
            .invoke(Args::positional(vec![Value::List(vec![])]))
            .is_err());
    }
}
