//! The JSON-lines protocol loop. See `docs/engine-protocol.md`.

use crate::json;
use crate::project::Analysis;
use crate::wire::{self, obj, text};
use diracord_diagnostics::{render, SourceFile};
use diracord_dirabank::Dirabank;
use diracord_interpreter::{block_on, BoxFuture, Host, HostError, Interpreter, Options};
use diracord_value::{Action, MemoryStore, Scope, Store, Value};
use std::io::{self, BufRead, Write};

/// Protocol version, sent in `hello` and `loaded`.
pub const PROTOCOL: i64 = 1;

/// Reads requests from `input` and writes responses to `output` until
/// `shutdown` or end of input.
pub fn serve<R: BufRead, W: Write>(input: R, output: W) -> io::Result<()> {
    let mut s = Server {
        input,
        out: output,
        engine: None,
        files: Vec::new(),
        store: Box::new(MemoryStore::new()),
    };
    s.send(&obj([
        ("op", "hello".into()),
        ("protocol", Value::Int(PROTOCOL)),
        ("version", env!("CARGO_PKG_VERSION").into()),
    ]))?;
    let mut line = String::new();
    loop {
        line.clear();
        if s.input.read_line(&mut line)? == 0 {
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        let req = match json::parse(line.trim()) {
            Ok(v) => v,
            Err(e) => {
                s.error(None, &format!("JSON inválido: {e}"))?;
                continue;
            }
        };
        let id = req.field("id").cloned();
        let r = match text(&req, "op").as_deref() {
            Some("check") => s.check(&req, false),
            Some("fix") => s.check(&req, true),
            Some("load") => s.load(&req),
            Some("dispatch") => s.dispatch(&req),
            Some("shutdown") => {
                let _ = s.store.flush();
                s.send(&obj([("op", "bye".into())]))?;
                return Ok(());
            }
            Some(other) => Err(format!("op desconhecida: {other}")),
            None => Err("faltou `op`".into()),
        };
        match r {
            Ok(()) => {}
            Err(msg) => s.error(id, &msg)?,
        }
    }
    let _ = s.store.flush();
    Ok(())
}

struct Server<R, W> {
    input: R,
    out: W,
    engine: Option<Interpreter>,
    files: Vec<SourceFile>,
    store: Box<dyn Store>,
}

fn send_line(out: &mut impl Write, v: &Value) -> io::Result<()> {
    let mut s = json::to_string(v);
    s.push('\n');
    out.write_all(s.as_bytes())?;
    out.flush()
}

impl<R: BufRead, W: Write> Server<R, W> {
    fn send(&mut self, v: &Value) -> io::Result<()> {
        send_line(&mut self.out, v)
    }

    fn error(&mut self, id: Option<Value>, message: &str) -> io::Result<()> {
        let mut m = obj([("op", "error".into()), ("message", message.into())]);
        if let (Value::Object(map), Some(id)) = (&mut m, id) {
            map.insert("id".into(), id);
        }
        self.send(&m)
    }

    /// `check` (report only) or `fix` (also return fixed sources).
    fn check(&mut self, req: &Value, fix: bool) -> Result<(), String> {
        let files = wire::files(req)?;
        let color = req.field("color").is_some_and(Value::is_truthy);
        let a = Analysis::new(&files, &wire::parse_options(req));
        let mut resp = vec![
            (
                "op".to_string(),
                Value::from(if fix { "fixed" } else { "checked" }),
            ),
            ("ok".to_string(), Value::Bool(a.errors() == 0)),
            ("errors".to_string(), Value::Int(a.errors() as i64)),
            ("warnings".to_string(), Value::Int(a.warnings() as i64)),
            ("diagnostics".to_string(), wire::diagnostics(&a, color)),
            ("rendered".to_string(), Value::from(a.render(color))),
        ];
        if fix {
            let fixed = a
                .fixed_sources()
                .into_iter()
                .map(|(path, source, changed)| {
                    obj([
                        ("path", path.into()),
                        ("source", source.into()),
                        ("changed", changed.into()),
                    ])
                })
                .collect();
            resp.push(("files".to_string(), Value::List(fixed)));
        }
        self.send(&Value::object(resp)).map_err(|e| e.to_string())
    }

    fn load(&mut self, req: &Value) -> Result<(), String> {
        let files = wire::files(req)?;
        let color = req.field("color").is_some_and(Value::is_truthy);
        let a = Analysis::new(&files, &wire::parse_options(req));
        let ok = a.errors() == 0;
        let mut resp = vec![
            ("op".to_string(), Value::from("loaded")),
            ("ok".to_string(), Value::Bool(ok)),
            ("protocol".to_string(), Value::Int(PROTOCOL)),
            ("diagnostics".to_string(), wire::diagnostics(&a, color)),
            ("rendered".to_string(), Value::from(a.render(color))),
        ];
        if ok {
            let default_scope = text(req, "defaultScope")
                .and_then(|s| Scope::from_name(&s))
                .unwrap_or(Scope::User);
            let options = Options {
                limits: wire::limits(req.field("limits")),
                default_scope,
                registry: diracord_functions::registry(),
            };
            let data_path = req.field("data").and_then(|d| text(d, "path"));
            let _ = self.store.flush();
            self.store = match data_path {
                Some(p) => Box::new(
                    Dirabank::open(&p)
                        .map_err(|e| format!("não consegui abrir o banco {p}: {e}"))?,
                ),
                None => Box::new(MemoryStore::new()),
            };
            resp.extend(wire::manifest(&a.program));
            self.files = a.files;
            self.engine = Some(Interpreter::new(a.program, options));
        }
        self.send(&Value::object(resp)).map_err(|e| e.to_string())
    }

    fn dispatch(&mut self, req: &Value) -> Result<(), String> {
        let id = req
            .field("id")
            .and_then(Value::as_i64)
            .ok_or("dispatch precisa de `id` numérico")?;
        let Some(engine) = &self.engine else {
            return Err("nenhum projeto carregado (envie `load` antes)".into());
        };
        let trigger = wire::trigger(req.field("trigger").ok_or("faltou `trigger`")?)?;
        let ctx = wire::context(req.field("context"));
        let mut host = StdioHost {
            id,
            seq: 0,
            input: &mut self.input,
            out: &mut self.out,
        };
        let result = block_on(engine.run(&trigger, ctx, &mut host, self.store.as_mut()));
        let _ = self.store.flush();
        let resp = match result {
            Ok(o) => obj([
                ("op", "done".into()),
                ("id", Value::Int(id)),
                ("ok", true.into()),
                ("steps", Value::Int(o.steps as i64)),
                ("actions", Value::Int(o.actions as i64)),
                ("value", o.value),
            ]),
            Err(e) => {
                let file = self
                    .files
                    .get(e.file.0 as usize)
                    .cloned()
                    .unwrap_or_else(|| SourceFile::new("?", ""));
                obj([
                    ("op", "done".into()),
                    ("id", Value::Int(id)),
                    ("ok", false.into()),
                    (
                        "error",
                        obj([
                            ("code", e.diagnostic.code.to_string().into()),
                            ("message", e.diagnostic.message.as_str().into()),
                            ("file", file.name().into()),
                            ("rendered", render(&e.diagnostic, &file, false).into()),
                        ]),
                    ),
                ])
            }
        };
        self.send(&resp).map_err(|e| e.to_string())
    }
}

/// Forwards actions to the other side of the pipe and waits for the result.
struct StdioHost<'a, R, W> {
    id: i64,
    seq: i64,
    input: &'a mut R,
    out: &'a mut W,
}

impl<R: BufRead, W: Write> StdioHost<'_, R, W> {
    fn round_trip(&mut self, action: Action) -> Result<Value, HostError> {
        self.seq += 1;
        let msg = obj([
            ("op", "action".into()),
            ("id", Value::Int(self.id)),
            ("seq", Value::Int(self.seq)),
            ("action", wire::action(&action)),
        ]);
        send_line(self.out, &msg).map_err(|e| HostError::new(e.to_string()))?;
        let mut line = String::new();
        loop {
            line.clear();
            match self.input.read_line(&mut line) {
                Ok(0) => return Err(HostError::new("a conexão com o Discord foi encerrada")),
                Ok(_) => {}
                Err(e) => return Err(HostError::new(e.to_string())),
            }
            let Ok(v) = json::parse(line.trim()) else {
                continue;
            };
            let is_ours = text(&v, "op").as_deref() == Some("result")
                && v.field("id").and_then(Value::as_i64) == Some(self.id)
                && v.field("seq").and_then(Value::as_i64) == Some(self.seq);
            if !is_ours {
                continue;
            }
            return if v.field("ok").is_some_and(Value::is_truthy) {
                Ok(v.field("value").cloned().unwrap_or(Value::Null))
            } else {
                Err(HostError::new(
                    text(&v, "error").unwrap_or_else(|| "erro desconhecido".into()),
                ))
            };
        }
    }
}

impl<R: BufRead, W: Write> Host for StdioHost<'_, R, W> {
    fn perform(&mut self, action: Action) -> BoxFuture<'_, Result<Value, HostError>> {
        let r = self.round_trip(action);
        Box::pin(std::future::ready(r))
    }
}
