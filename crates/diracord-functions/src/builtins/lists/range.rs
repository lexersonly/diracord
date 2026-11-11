//! `range` — Lista de números de start até end (sem incluir end). Com um argumento, de 0 até ele.
//!
//! ```text
//! for i in range(1, 4) { reply i }
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

/// Biggest list `range` may build.
const MAX_LEN: i64 = 10_000;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required(
        "start",
        ParamKind::Number,
        "início (ou fim, se for o único)",
    ),
    ParamSpec::optional("end", ParamKind::Number, "fim (não incluído)"),
];

/// The `range` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "range",
    category: Category::Lists,
    summary: "Lista de números de start até end (sem incluir end). Com um argumento, de 0 até ele.",
    params: PARAMS,
    returns: "list",
    example: "for i in range(1, 4) { reply i }",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let (start, end) = match b.opt(1) {
        Some(_) => (b.int(0)?, b.int(1)?),
        None => (0, b.int(0)?),
    };
    if end.saturating_sub(start) > MAX_LEN {
        return Err(b.wrong(1, "um intervalo de até 10000 números", b.get(1)));
    }
    Ok(Effect::Value(Value::List(
        (start..end).map(Value::Int).collect(),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn ranges() {
        let e = SPEC.invoke(Args::positional(vec![3i64.into()])).unwrap();
        assert_eq!(
            e,
            Effect::Value(Value::List(vec![0i64.into(), 1i64.into(), 2i64.into()]))
        );
        assert!(SPEC
            .invoke(Args::positional(vec![0i64.into(), 1_000_000i64.into()]))
            .is_err());
    }
}
