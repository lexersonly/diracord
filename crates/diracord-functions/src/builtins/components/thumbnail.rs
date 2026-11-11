//! `thumbnail` — Imagem pequena ao lado de uma section, com opções.
//!
//! ```text
//! section("Hi", thumbnail: thumbnail(user.avatar, description: "avatar"))
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Map, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("url", ParamKind::Text, "endereço da imagem (https://...)"),
    ParamSpec::optional("description", ParamKind::Text, "texto alternativo"),
    ParamSpec::optional("spoiler", ParamKind::Bool, "esconde até clicar"),
];

/// The `thumbnail` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "thumbnail",
    category: Category::Components,
    summary: "Imagem pequena ao lado de uma section (use em thumbnail: quando precisar de opções).",
    params: PARAMS,
    returns: "component",
    example: "reply container(section(\"Hi\", thumbnail: thumbnail(user.avatar, description: \"avatar\")))",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let mut obj = Map::new();
    obj.insert("thumbnail".into(), b.text(0)?.into());
    super::pieces::options(b, &mut obj, &[(1, "description"), (2, "spoiler")]);
    Ok(Effect::Value(Value::Object(obj)))
}
