//! Helpers shared by several built-ins (argument shapes, numbers, randomness).

use crate::{Bound, FnError};
use diracord_value::Value;
use std::cell::Cell;

/// Message content: text (numbers/bools become text), an embed object, or a
/// Components v2 layout (`{ layout: [...] }`, also what `layout { }` builds).
pub(crate) fn content(b: &Bound, i: usize) -> Result<Value, FnError> {
    match b.get(i) {
        v @ Value::Object(_) if crate::layout::is_layout(v) => match crate::layout::validate(v) {
            Ok(()) => Ok(v.clone()),
            Err(e) => Err(FnError::new(
                diracord_diagnostics::codes::E0117,
                format!("Layout inválido em {}: {}", e.path, e.message),
            )
            .at(crate::ArgRef::Param(i))
            .with_help(
                "veja os componentes e onde cada um pode ficar em docs/language.md#components-v2",
            )),
        },
        v @ Value::Object(_) => Ok(v.clone()),
        Value::Text(s) if s.is_empty() => Err(b.wrong(
            i,
            "um texto não vazio (o Discord não envia mensagem vazia)",
            b.get(i),
        )),
        Value::Null => Err(b.wrong(i, "um texto ou embed", b.get(i))),
        _ => Ok(b.text(i)?.into()),
    }
}

/// Optional text: `null` when left out.
pub(crate) fn opt_text(b: &Bound, i: usize) -> Result<Value, FnError> {
    match b.opt(i) {
        None | Some(Value::Null) => Ok(Value::Null),
        Some(_) => Ok(b.text(i)?.into()),
    }
}

/// A role given as object/id/mention (→ id text) or as a name (→ the name text).
/// The host resolves names.
pub(crate) fn role_ref(b: &Bound, i: usize) -> Result<Value, FnError> {
    match b.get(i).entity_id() {
        Some(id) => Ok(id.into()),
        None => Ok(b.text(i)?.into()),
    }
}

/// A saved-data name: non-empty text up to 100 characters.
pub(crate) fn data_name(b: &Bound, i: usize) -> Result<String, FnError> {
    let name = b.text(i)?;
    if name.is_empty() || name.chars().count() > 100 {
        return Err(b.wrong(i, "um nome de 1 a 100 letras", b.get(i)));
    }
    Ok(name)
}

/// A number as `Int` when whole, else `Float`.
pub(crate) fn number(n: f64) -> Value {
    if n.fract() == 0.0 && n.abs() < 9.0e15 {
        Value::Int(n as i64)
    } else {
        Value::Float(n)
    }
}

thread_local! {
    static SEED: Cell<u64> = const { Cell::new(0) };
}

/// A pseudo-random number in `[0, 1)`. Not for cryptography: it is for dice
/// and giveaways. xorshift64*, seeded from the clock, no dependency.
pub(crate) fn random_unit() -> f64 {
    SEED.with(|s| {
        let mut x = s.get();
        if x == 0 {
            let t = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x9E37_79B9_7F4A_7C15);
            x = t | 1;
        }
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        s.set(x);
        (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11) as f64 / (1u64 << 53) as f64
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_in_unit_range() {
        for _ in 0..1000 {
            let r = random_unit();
            assert!((0.0..1.0).contains(&r));
        }
    }

    #[test]
    fn number_shape() {
        assert_eq!(number(2.0), Value::Int(2));
        assert_eq!(number(2.5), Value::Float(2.5));
    }
}
