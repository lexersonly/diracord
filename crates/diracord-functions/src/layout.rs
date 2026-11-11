//! Components v2: the object form every layout ends up in, its validation, and
//! helpers for the pieces (`container(...)`, `row(...)`...).
//!
//! The object form:
//!
//! ```text
//! { layout: [
//!     { container: [ { text: "## Oi" }, { separator: true },
//!                    { row: [ { button: "ok", label: "Ok", style: "success" } ] } ],
//!       color: "purple" }
//! ] }
//! ```
//!
//! The pieces (`container(text("## Oi"), row(button("ok", "Ok")))`) build
//! exactly this shape, so everything is checked here before going to Discord.

use diracord_value::{Map, Value};
use std::fmt;

/// Discord's limit of components in one message (all levels counted).
pub const MAX_COMPONENTS: usize = 40;

/// What is wrong, and where (`layout[0].container[2]`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutError {
    /// Path to the bad component.
    pub path: String,
    /// Plain-language message.
    pub message: String,
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

/// Component type keys, in the order they are looked for.
const TOP_TYPES: [&str; 7] = [
    "container",
    "section",
    "row",
    "text",
    "separator",
    "gallery",
    "file",
];
const BUTTON_STYLES: [&str; 5] = ["primary", "secondary", "success", "danger", "link"];

/// `true` when a message object uses Components v2 (`{ layout: ... }`).
pub fn is_layout(v: &Value) -> bool {
    v.field("layout").is_some()
}

/// Every piece kind, top-level kinds first.
pub const KINDS: [&str; 10] = [
    "container",
    "section",
    "row",
    "text",
    "separator",
    "gallery",
    "file",
    "button",
    "menu",
    "thumbnail",
];

/// Which piece a value is (`"row"`, `"button"`...), or `None` for anything else.
pub fn kind_of(v: &Value) -> Option<&'static str> {
    let Value::Object(m) = v else { return None };
    KINDS.iter().copied().find(|k| m.contains_key(*k))
}

/// Can a `child` piece go directly inside `parent` (`None` = the message itself)?
pub fn allowed_in(child: &str, parent: Option<&str>) -> bool {
    match parent {
        None => TOP_TYPES.contains(&child),
        Some("container") => matches!(
            child,
            "section" | "row" | "text" | "separator" | "gallery" | "file"
        ),
        Some("row") => matches!(child, "button" | "menu"),
        Some(_) => false,
    }
}

/// Where a piece may go, in words.
pub fn placement_hint(kind: &str) -> &'static str {
    match kind {
        "button" => "dentro de `row(...)` (ou como `button:` de uma `section(...)`)",
        "menu" => "dentro de `row(...)`",
        "thumbnail" => "como `thumbnail:` de uma `section(...)`",
        "container" => "direto na mensagem (não dentro de outro container)",
        _ => "direto na mensagem ou dentro de `container(...)`",
    }
}

/// Turns what was given to `reply`/`send` into a `{ layout: [...] }` message when
/// it is a piece, a list of pieces, or already a layout. Other values → `None`.
pub fn as_message(v: &Value) -> Option<Value> {
    if is_layout(v) {
        return Some(v.clone());
    }
    match v {
        Value::Object(_) if kind_of(v).is_some() => {
            Some(Value::object([("layout", Value::List(vec![v.clone()]))]))
        }
        Value::List(l) if !l.is_empty() && l.iter().all(|x| kind_of(x).is_some()) => {
            Some(Value::object([("layout", v.clone())]))
        }
        _ => None,
    }
}

/// Checks a whole message object `{ layout: [...] }`.
pub fn validate(message: &Value) -> Result<(), LayoutError> {
    let err = |path: &str, m: String| {
        Err(LayoutError {
            path: path.to_string(),
            message: m,
        })
    };
    let Value::Object(m) = message else {
        return err("mensagem", "precisa ser um objeto".into());
    };
    if let Some(other) = m.keys().find(|k| *k != "layout") {
        return err(
            "mensagem",
            format!("`layout` não pode ser misturado com `{other}` (embeds e texto não combinam com Components v2; use `text` dentro do layout)"),
        );
    }
    let Some(Value::List(items)) = m.get("layout") else {
        return err("layout", "precisa ser uma lista de componentes".into());
    };
    if items.is_empty() {
        return err("layout", "o layout está vazio".into());
    }
    let mut v = Validator { count: 0 };
    for (i, item) in items.iter().enumerate() {
        v.item(item, &format!("layout[{i}]"), None)?;
    }
    if v.count > MAX_COMPONENTS {
        return err(
            "layout",
            format!(
                "tem {} componentes; o Discord aceita no máximo {MAX_COMPONENTS}",
                v.count
            ),
        );
    }
    Ok(())
}

struct Validator {
    count: usize,
}

fn fail<T>(path: &str, message: impl Into<String>) -> Result<T, LayoutError> {
    Err(LayoutError {
        path: path.to_string(),
        message: message.into(),
    })
}

fn type_of(m: &Map, types: &[&'static str]) -> Option<&'static str> {
    types.iter().copied().find(|t| m.contains_key(*t))
}

fn only_keys(m: &Map, allowed: &[&str], path: &str, what: &str) -> Result<(), LayoutError> {
    match m.keys().find(|k| !allowed.contains(&k.as_str())) {
        Some(k) => fail(
            path,
            format!(
                "`{what}` não tem a opção `{k}` (opções: {})",
                allowed[1..].join(", ")
            ),
        ),
        None => Ok(()),
    }
}

fn text_of(v: &Value) -> Option<String> {
    match v {
        Value::Text(s) => Some(s.clone()),
        Value::Int(_) | Value::Float(_) | Value::Bool(_) => Some(v.to_string()),
        _ => None,
    }
}

impl Validator {
    fn item(&mut self, item: &Value, path: &str, parent: Option<&str>) -> Result<(), LayoutError> {
        self.count += 1;
        let Value::Object(m) = item else {
            return fail(path, "cada componente é um objeto, como { text: \"oi\" }");
        };
        let Some(kind) = type_of(m, &TOP_TYPES) else {
            if m.contains_key("button") || m.contains_key("menu") {
                return fail(path, "botões e menus precisam ficar dentro de `row` (ou o botão como acessório de `section`)");
            }
            if m.contains_key("thumbnail") {
                return fail(path, "`thumbnail` só pode ficar dentro de `section`");
            }
            return fail(
                path,
                format!(
                    "componente desconhecido; use um destes: {}",
                    TOP_TYPES.join(", ")
                ),
            );
        };
        if kind == "container" && parent.is_some() {
            return fail(path, "`container` só pode ficar no topo do layout");
        }
        match kind {
            "container" => {
                only_keys(m, &["container", "color", "spoiler"], path, "container")?;
                let Some(Value::List(children)) = m.get("container") else {
                    return fail(path, "`container` precisa de uma lista de componentes");
                };
                if children.is_empty() {
                    return fail(path, "o `container` está vazio");
                }
                for (i, c) in children.iter().enumerate() {
                    self.item(c, &format!("{path}.container[{i}]"), Some("container"))?;
                }
                Ok(())
            }
            "text" => {
                only_keys(m, &["text"], path, "text")?;
                match m.get("text").and_then(text_of) {
                    Some(s) if s.is_empty() => fail(path, "o texto está vazio"),
                    Some(s) if s.chars().count() > 4000 => {
                        fail(path, "o texto passa de 4000 letras")
                    }
                    Some(_) => Ok(()),
                    None => fail(path, "`text` precisa ser um texto"),
                }
            }
            "section" => self.section(m, path),
            "separator" => {
                only_keys(m, &["separator", "spacing", "line"], path, "separator")?;
                match m.get("spacing") {
                    None | Some(Value::Null) => Ok(()),
                    Some(Value::Text(s)) if s == "small" || s == "large" => Ok(()),
                    Some(_) => fail(path, "`spacing` precisa ser \"small\" ou \"large\""),
                }
            }
            "gallery" => {
                only_keys(m, &["gallery"], path, "gallery")?;
                let Some(Value::List(l)) = m.get("gallery") else {
                    return fail(path, "`gallery` precisa de uma lista de imagens");
                };
                if l.is_empty() || l.len() > 10 {
                    return fail(
                        path,
                        format!("`gallery` aceita de 1 a 10 imagens, recebeu {}", l.len()),
                    );
                }
                for (i, x) in l.iter().enumerate() {
                    media(x, &format!("{path}.gallery[{i}]"))?;
                }
                Ok(())
            }
            "file" => {
                only_keys(m, &["file", "spoiler"], path, "file")?;
                match m.get("file") {
                    Some(Value::Text(s)) if s.starts_with("attachment://") => Ok(()),
                    _ => fail(path, "`file` precisa ser \"attachment://nome-do-arquivo\""),
                }
            }
            "row" => self.row(m, path),
            _ => unreachable!(),
        }
    }

    fn section(&mut self, m: &Map, path: &str) -> Result<(), LayoutError> {
        only_keys(m, &["section", "thumbnail", "button"], path, "section")?;
        let Some(Value::List(texts)) = m.get("section") else {
            return fail(path, "`section` precisa de uma lista de textos");
        };
        if texts.is_empty() || texts.len() > 3 {
            return fail(
                path,
                format!("`section` aceita de 1 a 3 textos, recebeu {}", texts.len()),
            );
        }
        for (i, t) in texts.iter().enumerate() {
            let ok = text_of(t)
                .or_else(|| t.field("text").and_then(text_of))
                .is_some_and(|s| !s.is_empty());
            if !ok {
                return fail(
                    &format!("{path}.section[{i}]"),
                    "cada item da `section` é um texto",
                );
            }
        }
        self.count += texts.len();
        match (m.get("thumbnail"), m.get("button")) {
            (Some(_), Some(_)) => fail(
                path,
                "`section` aceita só um acessório: `thumbnail` ou `button`",
            ),
            (None, None) => fail(
                path,
                "`section` precisa de um acessório: um `thumbnail` ou um `button`",
            ),
            (Some(t), None) => {
                self.count += 1;
                media(t, &format!("{path}.thumbnail"))
            }
            (None, Some(Value::Object(b))) => {
                self.count += 1;
                button(b, &format!("{path}.button"))
            }
            (None, Some(_)) => fail(&format!("{path}.button"), "o botão precisa ser um objeto"),
        }
    }

    fn row(&mut self, m: &Map, path: &str) -> Result<(), LayoutError> {
        only_keys(m, &["row"], path, "row")?;
        let Some(Value::List(items)) = m.get("row") else {
            return fail(path, "`row` precisa de uma lista de botões ou um menu");
        };
        if items.is_empty() {
            return fail(path, "a `row` está vazia");
        }
        let mut buttons = 0;
        let mut menus = 0;
        for (i, x) in items.iter().enumerate() {
            let p = format!("{path}.row[{i}]");
            self.count += 1;
            let Value::Object(o) = x else {
                return fail(&p, "cada item da `row` é um botão ou um menu");
            };
            match type_of(o, &["button", "menu"]) {
                Some("button") => {
                    buttons += 1;
                    button(o, &p)?
                }
                Some(_) => {
                    menus += 1;
                    menu(o, &p)?
                }
                None => return fail(&p, "dentro de `row` só vão `button` e `menu`"),
            }
        }
        if menus > 0 && (menus > 1 || buttons > 0) {
            return fail(
                path,
                "uma `row` com menu não pode ter mais nada: use uma `row` só para o menu",
            );
        }
        if buttons > 5 {
            return fail(
                path,
                format!("uma `row` aceita no máximo 5 botões, recebeu {buttons}"),
            );
        }
        Ok(())
    }
}

fn media(v: &Value, path: &str) -> Result<(), LayoutError> {
    let url = match v {
        Value::Text(s) => Some(s.as_str()),
        Value::Object(m) => {
            only_keys(m, &["url", "description", "spoiler"], path, "imagem")?;
            m.get("url").and_then(Value::as_str)
        }
        _ => None,
    };
    match url {
        Some(u)
            if u.starts_with("https://")
                || u.starts_with("http://")
                || u.starts_with("attachment://") =>
        {
            Ok(())
        }
        Some(_) => fail(
            path,
            "o endereço da imagem precisa começar com https:// ou attachment://",
        ),
        None => fail(path, "faltou o endereço (url) da imagem"),
    }
}

fn button(b: &Map, path: &str) -> Result<(), LayoutError> {
    only_keys(
        b,
        &["button", "label", "style", "emoji", "disabled", "url"],
        path,
        "button",
    )?;
    let label = b.get("label").and_then(text_of).filter(|s| !s.is_empty());
    if label.is_none() && b.get("emoji").and_then(text_of).is_none() {
        return fail(path, "o botão precisa de um texto (label) ou de um emoji");
    }
    if label.is_some_and(|l| l.chars().count() > 80) {
        return fail(path, "o texto do botão passa de 80 letras");
    }
    let style = b.get("style").and_then(Value::as_str);
    if let Some(s) = style {
        if !BUTTON_STYLES.contains(&s) {
            return fail(
                path,
                format!("estilo `{s}` não existe; use {}", BUTTON_STYLES.join(", ")),
            );
        }
    }
    let url = b.get("url").and_then(Value::as_str);
    let id = b.get("button").and_then(text_of).filter(|s| !s.is_empty());
    match (url, id) {
        (Some(_), Some(_)) => fail(
            path,
            "botão de link (url) não tem id: o clique abre o endereço",
        ),
        (Some(u), None) if !u.starts_with("https://") && !u.starts_with("http://") => {
            fail(path, "a url do botão precisa começar com https://")
        }
        (Some(_), None) => Ok(()),
        (None, None) => fail(
            path,
            "o botão precisa de um id, ligado a button(\"id\") { ... } no código",
        ),
        (None, Some(_)) if style == Some("link") => fail(path, "estilo link precisa de `url`"),
        (None, Some(i)) if i.chars().count() > 100 => {
            fail(path, "o id do botão passa de 100 letras")
        }
        (None, Some(_)) => Ok(()),
    }
}

fn menu(m: &Map, path: &str) -> Result<(), LayoutError> {
    only_keys(
        m,
        &["menu", "options", "placeholder", "min", "max"],
        path,
        "menu",
    )?;
    if m.get("menu").and_then(text_of).is_none_or(|s| s.is_empty()) {
        return fail(
            path,
            "o menu precisa de um id, ligado a menu(\"id\") { ... } no código",
        );
    }
    let Some(Value::List(opts)) = m.get("options") else {
        return fail(path, "o menu precisa de `options`: uma lista de opções");
    };
    if opts.is_empty() || opts.len() > 25 {
        return fail(
            path,
            format!("o menu aceita de 1 a 25 opções, recebeu {}", opts.len()),
        );
    }
    for (i, o) in opts.iter().enumerate() {
        let ok = match o {
            Value::Object(x) => x.get("label").and_then(text_of).is_some(),
            other => text_of(other).is_some(),
        };
        if !ok {
            return fail(
                &format!("{path}.options[{i}]"),
                "cada opção é um texto ou { label: ..., value: ... }",
            );
        }
    }
    for k in ["min", "max"] {
        if let Some(v) = m.get(k).filter(|v| **v != Value::Null) {
            if !v.as_i64().is_some_and(|n| (0..=25).contains(&n)) {
                return fail(path, format!("`{k}` precisa ser um número de 0 a 25"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj<const N: usize>(p: [(&str, Value); N]) -> Value {
        Value::object(p)
    }
    fn list(v: Vec<Value>) -> Value {
        Value::List(v)
    }
    fn msg(items: Vec<Value>) -> Value {
        obj([("layout", list(items))])
    }
    fn btn(id: &str) -> Value {
        obj([("button", id.into()), ("label", "Ok".into())])
    }

    #[test]
    fn accepts_the_rfc_example() {
        let m = msg(vec![obj([
            (
                "container",
                list(vec![
                    obj([("text", "## Oi".into())]),
                    obj([
                        ("section", list(vec!["Leia".into()])),
                        ("thumbnail", "https://x/a.png".into()),
                    ]),
                    obj([("separator", true.into()), ("spacing", "large".into())]),
                    obj([("row", list(vec![btn("a"), btn("b")]))]),
                ]),
            ),
            ("color", "purple".into()),
        ])]);
        assert_eq!(validate(&m), Ok(()));
    }

    #[test]
    fn explains_mistakes() {
        let e = validate(&msg(vec![btn("a")])).unwrap_err();
        assert!(e.message.contains("dentro de `row`"), "{e}");
        let e = validate(&msg(vec![obj([("row", list(vec![btn("a"); 6]))])])).unwrap_err();
        assert!(e.message.contains("5 botões"), "{e}");
        let e = validate(&msg(vec![obj([("section", list(vec!["a".into()]))])])).unwrap_err();
        assert!(e.message.contains("acessório"), "{e}");
        let e = validate(&obj([
            ("layout", list(vec![obj([("text", "a".into())])])),
            ("title", "x".into()),
        ]))
        .unwrap_err();
        assert!(e.message.contains("misturado"), "{e}");
        let e = validate(&msg(vec![obj([(
            "container",
            list(vec![obj([("container", list(vec![]))])]),
        )])]))
        .unwrap_err();
        assert_eq!(e.path, "layout[0].container[0]");
        let e = validate(&msg(vec![obj([(
            "row",
            list(vec![obj([
                ("button", "x".into()),
                ("label", "X".into()),
                ("style", "red".into()),
            ])]),
        )])]))
        .unwrap_err();
        assert!(e.message.contains("estilo `red`"), "{e}");
        let menu_row = obj([(
            "row",
            list(vec![
                obj([("menu", "m".into()), ("options", list(vec!["a".into()]))]),
                btn("b"),
            ]),
        )]);
        assert!(validate(&msg(vec![menu_row])).is_err());
        assert!(validate(&msg(vec![])).is_err());
    }

    #[test]
    fn link_buttons() {
        let link = obj([
            ("button", Value::Null),
            ("label", "Site".into()),
            ("url", "https://a.b".into()),
        ]);
        assert_eq!(
            validate(&msg(vec![obj([("row", list(vec![link]))])])),
            Ok(())
        );
        let both = obj([
            ("button", "x".into()),
            ("label", "Site".into()),
            ("url", "https://a.b".into()),
        ]);
        assert!(validate(&msg(vec![obj([("row", list(vec![both]))])])).is_err());
    }

    #[test]
    fn counts_components() {
        let rows: Vec<Value> = (0..9)
            .map(|_| obj([("row", list(vec![btn("a"); 5]))]))
            .collect();
        let e = validate(&msg(rows)).unwrap_err();
        assert!(e.message.contains("no máximo 40"), "{e}");
    }
}
