//! Shared code of the Components v2 pieces (the files next to this one).

use crate::layout::{allowed_in, kind_of, placement_hint, validate};
use crate::{ArgRef, Bound, FnError};
use diracord_diagnostics::codes;
use diracord_value::{Map, Value};

/// The children given to `parent(...)` (variadic parameter `i`), each checked
/// to be a piece allowed inside `parent`. The arrow points at the wrong child.
pub(crate) fn children(b: &Bound, i: usize, parent: &str) -> Result<Vec<Value>, FnError> {
    let items = b.list(i)?.to_vec();
    for (j, child) in items.iter().enumerate() {
        let at = ArgRef::Positional(i + j);
        match kind_of(child) {
            Some(k) if allowed_in(k, Some(parent)) => {}
            Some(k) => {
                return Err(FnError::new(
                    codes::E0117,
                    format!("`{k}(...)` não pode ficar dentro de `{parent}(...)`"),
                )
                .at(at)
                .with_help(format!("`{k}` vai {}", placement_hint(k))))
            }
            None => {
                let hint = if matches!(child, Value::Text(_)) {
                    format!("para texto, use `text(\"...\")` dentro de `{parent}(...)`")
                } else {
                    format!("use peças como text(...), row(...) ou button(...) dentro de `{parent}(...)`")
                };
                return Err(FnError::new(
                    codes::E0117,
                    format!(
                        "`{parent}(...)` recebeu {}, mas espera peças do layout",
                        child.type_name()
                    ),
                )
                .at(at)
                .with_help(hint));
            }
        }
    }
    Ok(items)
}

/// Copies the optional parameters that were given (not `null`) into `obj`.
pub(crate) fn options(b: &Bound, obj: &mut Map, names: &[(usize, &str)]) {
    for (i, name) in names {
        if let Some(v) = b.opt(*i).filter(|v| **v != Value::Null) {
            obj.insert((*name).to_string(), v.clone());
        }
    }
}

/// Runs the full layout check on one top-level piece, so limits (5 buttons per
/// row, one accessory per section...) are reported where the piece is built.
pub(crate) fn check(piece: &Value) -> Result<(), FnError> {
    let msg = Value::object([("layout", Value::List(vec![piece.clone()]))]);
    validate(&msg).map_err(|e| {
        let path = e.path.replacen("layout[0]", "", 1);
        let place = if path.is_empty() {
            String::new()
        } else {
            format!(" (em {})", path.trim_start_matches('.'))
        };
        FnError::new(codes::E0117, format!("{}{place}", e.message))
            .with_help("veja as peças e onde cada uma fica em docs/language.md#components-v2")
    })
}
