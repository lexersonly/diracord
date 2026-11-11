//! Conversions between protocol JSON and engine types.

use crate::project::Analysis;
use diracord_ast::{FileId, InteractionKind, ItemKind, Program};
use diracord_diagnostics::{render, Diagnostic};
use diracord_interpreter::{Context, Limits, Trigger};
use diracord_value::{Action, Map, Value};
use std::time::Duration;

pub(crate) fn obj<const N: usize>(pairs: [(&str, Value); N]) -> Value {
    Value::object(pairs)
}

pub(crate) fn text(v: &Value, key: &str) -> Option<String> {
    v.field(key).and_then(Value::as_str).map(str::to_string)
}

/// `files: [{path, source}]`
pub(crate) fn files(req: &Value) -> Result<Vec<(String, String)>, String> {
    let Some(Value::List(list)) = req.field("files") else {
        return Err("faltou `files`".into());
    };
    list.iter()
        .map(|f| match (text(f, "path"), text(f, "source")) {
            (Some(p), Some(s)) => Ok((p, s)),
            _ => Err("cada arquivo precisa de `path` e `source`".into()),
        })
        .collect()
}

pub(crate) fn parse_options(req: &Value) -> diracord_parser::ParseOptions {
    let dollar = req
        .field("syntax")
        .and_then(|s| s.field("dollar"))
        .is_some_and(Value::is_truthy);
    diracord_parser::ParseOptions { dollar }
}

pub(crate) fn diagnostic(file: FileId, d: &Diagnostic, a: &Analysis, color: bool) -> Value {
    let src = a.file(file);
    let span = d.primary_span().unwrap_or_default();
    let (start, end) = (src.line_col(span.start), src.line_col(span.end));
    let strings = |v: &[String]| Value::List(v.iter().map(|s| Value::from(s.as_str())).collect());
    let fixes = d
        .fixes
        .iter()
        .map(|f| {
            let edits = f
                .edits
                .iter()
                .map(|e| {
                    obj([
                        ("start", Value::Int(e.span.start as i64)),
                        ("end", Value::Int(e.span.end as i64)),
                        ("replacement", e.replacement.as_str().into()),
                    ])
                })
                .collect();
            obj([
                ("message", f.message.as_str().into()),
                ("safe", f.safe.into()),
                ("edits", Value::List(edits)),
            ])
        })
        .collect();
    obj([
        ("code", d.code.to_string().into()),
        ("severity", d.severity.label().into()),
        ("message", d.message.as_str().into()),
        ("file", src.name().into()),
        ("line", Value::Int(start.line as i64)),
        ("col", Value::Int(start.col as i64)),
        ("endLine", Value::Int(end.line as i64)),
        ("endCol", Value::Int(end.col as i64)),
        ("help", strings(&d.help)),
        ("notes", strings(&d.notes)),
        ("fixes", Value::List(fixes)),
        ("rendered", render(d, src, color).into()),
    ])
}

pub(crate) fn diagnostics(a: &Analysis, color: bool) -> Value {
    Value::List(
        a.diagnostics
            .iter()
            .map(|(f, d)| diagnostic(*f, d, a, color))
            .collect(),
    )
}

/// Commands, events and interactions, so the host knows what to register.
pub(crate) fn manifest(p: &Program) -> Vec<(String, Value)> {
    let mut commands = Vec::new();
    let mut events: Vec<Value> = Vec::new();
    let mut interactions = Vec::new();
    for item in &p.items {
        match &item.kind {
            ItemKind::Command(c) => {
                let params = c
                    .params
                    .iter()
                    .map(|p| {
                        obj([
                            ("name", p.name.name.as_str().into()),
                            ("type", p.ty.node.as_str().into()),
                            ("optional", p.default.is_some().into()),
                        ])
                    })
                    .collect();
                commands.push(obj([
                    ("name", c.name.node.as_str().into()),
                    ("params", Value::List(params)),
                ]));
            }
            ItemKind::Event(e) => {
                let v = Value::from(e.name.name.as_str());
                if !events.contains(&v) {
                    events.push(v);
                }
            }
            ItemKind::Interaction(i) => interactions.push(obj([
                ("kind", i.kind.as_str().into()),
                ("id", i.id.node.as_str().into()),
            ])),
            // User functions run inside the engine; the host does not need them.
            ItemKind::Function(_) => {}
        }
    }
    vec![
        ("commands".into(), Value::List(commands)),
        ("events".into(), Value::List(events)),
        ("interactions".into(), Value::List(interactions)),
    ]
}

pub(crate) fn trigger(v: &Value) -> Result<Trigger, String> {
    match text(v, "kind").as_deref() {
        Some("command") => {
            let name = text(v, "name").ok_or("faltou trigger.name")?;
            let args = match v.field("args") {
                Some(Value::Object(m)) => m.iter().map(|(k, x)| (k.clone(), x.clone())).collect(),
                _ => Vec::new(),
            };
            Ok(Trigger::Command { name, args })
        }
        Some("event") => Ok(Trigger::Event {
            name: text(v, "name").ok_or("faltou trigger.name")?,
        }),
        Some("interaction") => {
            let kind = text(v, "type")
                .and_then(|t| InteractionKind::from_name(&t))
                .ok_or("trigger.type inválido")?;
            Ok(Trigger::Interaction {
                kind,
                id: text(v, "id").ok_or("faltou trigger.id")?,
            })
        }
        _ => Err("trigger.kind precisa ser command, event ou interaction".into()),
    }
}

pub(crate) fn context(v: Option<&Value>) -> Context {
    let Some(v) = v else {
        return Context::default();
    };
    let get = |k: &str| v.field(k).cloned().unwrap_or(Value::Null);
    Context {
        user: get("user"),
        server: get("server"),
        channel: get("channel"),
        message: get("message"),
        is_interaction: v.field("isInteraction").is_some_and(Value::is_truthy),
        extra: match v.field("extra") {
            Some(Value::Object(m)) => m.clone(),
            _ => Map::new(),
        },
    }
}

pub(crate) fn limits(v: Option<&Value>) -> Limits {
    let mut l = Limits::default();
    let Some(v) = v else { return l };
    let n = |k: &str| v.field(k).and_then(Value::as_i64).filter(|&x| x > 0);
    if let Some(x) = n("maxSteps") {
        l.max_steps = x as u64;
    }
    if let Some(x) = n("maxTimeMs") {
        l.max_time = Duration::from_millis(x as u64);
    }
    if let Some(x) = n("maxMemoryKb") {
        l.max_memory = x as usize * 1024;
    }
    if let Some(x) = n("maxActions") {
        l.max_actions = x.min(u32::MAX as i64) as u32;
    }
    l
}

pub(crate) fn action(a: &Action) -> Value {
    obj([
        ("name", a.name.as_str().into()),
        ("args", Value::List(a.args.clone())),
        ("options", Value::Object(a.options.clone())),
    ])
}
