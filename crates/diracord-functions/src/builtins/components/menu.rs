//! `menu` — Menu de opções. A escolha roda o `menu("id") { }`; os valores ficam em `message.values`.
//!
//! ```text
//! menu("color", ["Red", "Blue"], placeholder: "Pick one")
//! ```

use crate::{Bound, Category, Effect, FnError, FunctionSpec, ParamKind, ParamSpec};
use diracord_value::{Map, Value};

const PARAMS: &[ParamSpec] = &[
    ParamSpec::required("id", ParamKind::Text, "id que liga ao menu(\"id\") { }"),
    ParamSpec::required(
        "options",
        ParamKind::List,
        "lista de textos ou { label, value, description }",
    ),
    ParamSpec::optional("placeholder", ParamKind::Text, "texto antes de escolher"),
    ParamSpec::optional("min", ParamKind::Number, "mínimo de escolhas"),
    ParamSpec::optional("max", ParamKind::Number, "máximo de escolhas"),
];

/// The `menu` function.
pub const SPEC: FunctionSpec = FunctionSpec {
    name: "menu",
    category: Category::Components,
    summary:
        "Menu de opções. A escolha roda o menu(\"id\") { }; os valores ficam em message.values.",
    params: PARAMS,
    returns: "component",
    example: "reply row(menu(\"color\", [\"Red\", \"Blue\"], placeholder: \"Pick one\"))",
    call,
};

fn call(b: &Bound) -> Result<Effect, FnError> {
    let mut obj = Map::new();
    obj.insert("menu".into(), b.text(0)?.into());
    obj.insert("options".into(), Value::List(b.list(1)?.to_vec()));
    super::pieces::options(b, &mut obj, &[(2, "placeholder"), (3, "min"), (4, "max")]);
    let v = Value::Object(obj);
    super::pieces::check(&Value::object([("row", Value::List(vec![v.clone()]))]))?;
    Ok(Effect::Value(v))
}
