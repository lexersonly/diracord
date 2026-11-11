//! `upper` — Texto em MAIÚSCULAS.
//!
//! ```text
//! reply upper("oi")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[ParamSpec::required("text", ParamKind::Text, "o texto")];

/// The `upper` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "upper",
    category: Category::Text,
    summary: "Texto em MAIÚSCULAS.",
    params: PARAMS,
    returns: "text",
    example: "reply upper(\"oi\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    Ok(Effect::Value(b.text(0)?.to_uppercase().into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn works() {
        assert_eq!(
            SPEC.invoke(Args::positional(vec!["ação".into()])).unwrap(),
            Effect::Value("AÇÃO".into())
        );
    }
}
