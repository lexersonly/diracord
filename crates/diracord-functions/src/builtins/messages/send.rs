//! `send` — Envia uma mensagem em um canal.
//!
//! ```text
//! send channel("general") "Hello!"
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Action;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required(
        "channel",
        ParamKind::Channel,
        "canal (objeto, id ou #menção)",
    ),
    ParamSpec::required("content", ParamKind::Content, "texto ou embed"),
];

/// The `send` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "send",
    category: Category::Messages,
    summary: "Envia uma mensagem em um canal.",
    params: PARAMS,
    returns: "message",
    example: "send channel(\"general\") \"Hello!\"",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let channel = b.id(0)?;
    let content = common::content(b, 1)?;
    Ok(Effect::Action(Action::new(
        "send",
        vec![channel.into(), content],
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;
    use diracord_value::Value;

    #[test]
    fn uses_channel_id() {
        let ch = Value::object([("id", Value::from("9"))]);
        let e = SPEC
            .invoke(Args::positional(vec![ch, "oi".into()]))
            .unwrap();
        assert_eq!(
            e,
            Effect::Action(Action::new("send", vec!["9".into(), "oi".into()]))
        );
    }
}
