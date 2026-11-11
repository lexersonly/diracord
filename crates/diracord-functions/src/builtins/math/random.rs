//! `random` — Número inteiro aleatório entre min e max (inclusive). Sem argumentos: entre 0 e 1 (decimal).
//!
//! ```text
//! local dice = random(1, 6)
//! ```

use super::super::common;
use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::Value;

const PARAMS: &[ParamSpec] = &[
    ParamSpec::optional("min", ParamKind::Number, "menor valor"),
    ParamSpec::optional("max", ParamKind::Number, "maior valor"),
];

/// The `random` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "random",
    category: Category::Math,
    summary: "Número inteiro aleatório entre min e max (inclusive). Sem argumentos: entre 0 e 1 (decimal).",
    params: PARAMS,
    returns: "number",
    example: "local dice = random(1, 6)",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    if b.opt(0).is_none() {
        return Ok(Effect::Value(Value::Float(common::random_unit())));
    }
    let (min, max) = match b.opt(1) {
        Some(_) => (b.int(0)?, b.int(1)?),
        None => (0, b.int(0)?),
    };
    let (lo, hi) = if min <= max { (min, max) } else { (max, min) };
    let span = (hi as i128 - lo as i128 + 1) as f64;
    let n = lo + (common::random_unit() * span).floor() as i64;
    Ok(Effect::Value(Value::Int(n.min(hi))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    #[test]
    fn stays_in_range() {
        for _ in 0..200 {
            let Effect::Value(Value::Int(n)) = SPEC
                .invoke(Args::positional(vec![1i64.into(), 6i64.into()]))
                .unwrap()
            else {
                panic!()
            };
            assert!((1..=6).contains(&n));
        }
    }
}
