//! `get` — Lê um valor salvo. Se não existir, devolve o padrão (ou null).
//!
//! ```text
//! get("coins", 0, scope: user)
//! ```

use super::super::common;
use crate::{Bound, Category, DataOp, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("name", ParamKind::Text, "nome da variável salva"),
    ParamSpec::optional("default", ParamKind::Any, "valor se nada foi salvo"),
    ParamSpec::optional("scope", ParamKind::Scope, "user, server, member ou global"),
];

/// The `get` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "get",
    category: Category::Data,
    summary: "Lê um valor salvo. Se não existir, devolve o padrão (ou null).",
    params: PARAMS,
    returns: "any",
    example: "get(\"coins\", 0, scope: user)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Data(DataOp::Get {
        name: common::data_name(b, 0)?,
        default: b.get(1).clone(),
        scope: b.scope(2)?,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;
    use diracord_value::Scope;
    use diracord_value::Value;

    #[test]
    fn builds_get() {
        let e = SPEC
            .invoke(Args::positional(vec!["coins".into()]).with_named("scope", "user".into()))
            .unwrap();
        assert_eq!(
            e,
            Effect::Data(DataOp::Get {
                name: "coins".into(),
                default: Value::Null,
                scope: Some(Scope::User)
            })
        );
    }

    #[test]
    fn bad_scope() {
        let err = SPEC
            .invoke(Args::positional(vec!["c".into()]).with_named("scope", "guild".into()))
            .unwrap_err();
        assert_eq!(err.code, diracord_diagnostics::codes::E0401);
    }
}
