//! `replace` — Troca todas as ocorrências de um pedaço do texto.
//!
//! ```text
//! replace("hello world", "world", "Diracord")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("text", ParamKind::Text, "o texto"),
    ParamSpec::required("from", ParamKind::Text, "o que procurar"),
    ParamSpec::required("to", ParamKind::Text, "pelo que trocar"),
];

/// The `replace` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "replace",
    category: Category::Text,
    summary: "Troca todas as ocorrências de um pedaço do texto.",
    params: PARAMS,
    returns: "text",
    example: "replace(\"hello world\", \"world\", \"Diracord\")",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let from = b.text(1)?;
    if from.is_empty() {
        return Err(b.wrong(1, "um texto não vazio", b.get(1)));
    }
    Ok(Effect::Value(b.text(0)?.replace(&from, &b.text(2)?).into()))
}
