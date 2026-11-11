//! The public [`Interpreter`] and the per-run [`Machine`].

use crate::context::{Context, Trigger};
use crate::env::Env;
use crate::error::RuntimeError;
use crate::host::Host;
use crate::limits::{Limits, Meter};
use diracord_ast::{ArgType, Block, CommandDef, ItemKind, Program};
use diracord_diagnostics::{codes, did_you_mean, FileId, Span};
use diracord_functions::Registry;
use diracord_value::{Scope, Store, Value};
use std::collections::HashMap;

/// Interpreter settings.
#[derive(Clone, Debug)]
pub struct Options {
    /// Execution limits for each run.
    pub limits: Limits,
    /// Scope used by `get`/`set` when the code does not say `scope:`.
    pub default_scope: Scope,
    /// Built-in functions.
    pub registry: &'static Registry,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            limits: Limits::default(),
            default_scope: Scope::User,
            registry: diracord_functions::registry(),
        }
    }
}

/// What a finished run used and returned.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    /// Steps executed.
    pub steps: u64,
    /// Host actions performed.
    pub actions: u32,
    /// The value given to `return`, or `null`.
    pub value: Value,
}

/// A loaded program, ready to run triggers. Cheap to share: each run gets
/// its own variables and meter.
#[derive(Clone, Debug)]
pub struct Interpreter {
    program: Program,
    options: Options,
    /// User functions by name → (file, index in `program.items`).
    functions: HashMap<String, (FileId, usize)>,
}

pub(crate) type R<T> = Result<T, RuntimeError>;

/// How a statement finished.
#[derive(Debug, PartialEq)]
pub(crate) enum Flow {
    Normal,
    Break,
    Continue,
    Return(Value),
    Stop,
}

/// State of one run.
pub(crate) struct Machine<'r> {
    pub(crate) opts: &'r Options,
    pub(crate) host: &'r mut dyn Host,
    pub(crate) store: &'r mut dyn Store,
    pub(crate) ctx: &'r Context,
    pub(crate) env: Env,
    pub(crate) meter: Meter,
    /// The whole program (user functions live in it).
    pub(crate) program: &'r Program,
    /// User functions by name.
    pub(crate) functions: &'r HashMap<String, (FileId, usize)>,
    /// How many user functions are running inside each other.
    pub(crate) depth: u32,
    /// Bytes held by callers' variables while a function runs.
    pub(crate) outer_bytes: usize,
}

impl Interpreter {
    /// Wraps a (merged) program.
    pub fn new(program: Program, options: Options) -> Self {
        let functions = program
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| match &item.kind {
                ItemKind::Function(f) => Some((f.name.name.clone(), (item.file, i))),
                _ => None,
            })
            .collect();
        Interpreter {
            program,
            options,
            functions,
        }
    }

    /// The program.
    pub fn program(&self) -> &Program {
        &self.program
    }

    /// The options.
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Runs the handler(s) for `trigger`.
    ///
    /// For events, every handler with that name runs in order; the first
    /// error stops the rest. Errors carry the [`FileId`] of the handler.
    pub async fn run(
        &self,
        trigger: &Trigger,
        ctx: Context,
        host: &mut dyn Host,
        store: &mut dyn Store,
    ) -> Result<Outcome, RuntimeError> {
        let mut m = Machine {
            opts: &self.options,
            host,
            store,
            ctx: &ctx,
            env: Env::new(),
            meter: Meter::new(),
            program: &self.program,
            functions: &self.functions,
            depth: 0,
            outer_bytes: 0,
        };
        let mut value = Value::Null;
        for (file, cmd, body) in self.handlers(trigger)? {
            m.env = Env::new();
            for (name, v) in m.ctx.variables() {
                m.env.declare(&name, v);
            }
            let r = async {
                if let (Some(cmd), Trigger::Command { args, .. }) = (cmd, trigger) {
                    m.bind_params(cmd, args).await?;
                }
                m.exec_block(body).await
            }
            .await;
            match r {
                Ok(crate::engine::Flow::Return(v)) => value = v,
                Ok(_) => {}
                // `stop` inside a function ends the run normally.
                Err(e) if e.stop => break,
                Err(mut e) => {
                    if !e.located {
                        e.file = file;
                    }
                    return Err(e);
                }
            }
        }
        Ok(Outcome {
            steps: m.meter.steps,
            actions: m.meter.actions,
            value,
        })
    }

    fn handlers(&self, trigger: &Trigger) -> R<Vec<(FileId, Option<&CommandDef>, &Block)>> {
        let mut out = Vec::new();
        for item in &self.program.items {
            match (&item.kind, trigger) {
                (ItemKind::Command(c), Trigger::Command { name, .. }) if &c.name.node == name => {
                    out.push((item.file, Some(c), &c.body));
                    break;
                }
                (ItemKind::Event(e), Trigger::Event { name }) if &e.name.name == name => {
                    out.push((item.file, None, &e.body));
                }
                (ItemKind::Interaction(i), Trigger::Interaction { kind, id })
                    if i.kind == *kind && &i.id.node == id =>
                {
                    out.push((item.file, None, &i.body));
                    break;
                }
                _ => {}
            }
        }
        if out.is_empty() {
            if let Trigger::Command { name, .. } = trigger {
                let names: Vec<&str> = self
                    .program
                    .commands()
                    .map(|c| c.name.node.as_str())
                    .collect();
                let mut e = RuntimeError::new(
                    codes::E0201,
                    format!("O comando `{name}` não existe"),
                    Span::default(),
                    "comando desconhecido",
                );
                if let Some(s) = did_you_mean(name, names) {
                    e = e.with_help(format!("você quis dizer `{s}`?"));
                }
                return Err(e);
            }
        }
        Ok(out)
    }
}

impl Machine<'_> {
    pub(crate) fn tick(&mut self, span: Span) -> R<()> {
        self.meter.tick(&self.opts.limits, span)
    }

    /// Errors with `E0206` when variables hold too much memory.
    pub(crate) fn check_memory(&self, span: Span) -> R<()> {
        self.check_size(self.env.bytes() + self.outer_bytes, span)
    }

    pub(crate) fn check_size(&self, bytes: usize, span: Span) -> R<()> {
        if bytes > self.opts.limits.max_memory {
            return Err(RuntimeError::limit(
                codes::E0206,
                format!(
                    "O código passou do limite de memória ({} KB) e foi parado",
                    self.opts.limits.max_memory / 1024
                ),
                span,
                "evite listas ou textos gigantes",
            ));
        }
        Ok(())
    }

    async fn bind_params(&mut self, cmd: &CommandDef, args: &[(String, Value)]) -> R<()> {
        for p in &cmd.params {
            let value = match args.iter().find(|(n, _)| n == &p.name.name) {
                Some((_, v)) if *v != Value::Null => coerce(
                    &p.name.name,
                    p.ty.node,
                    p.span,
                    v.clone(),
                    &format!("do comando `{}`", cmd.name.node),
                )?,
                _ => match &p.default {
                    Some(d) => self.eval(d).await?,
                    None => {
                        return Err(RuntimeError::new(
                            codes::E0300,
                            format!(
                                "Faltou o argumento `{}` do comando `{}`",
                                p.name.name, cmd.name.node
                            ),
                            p.span,
                            "argumento obrigatório",
                        )
                        .with_help(usage(cmd)))
                    }
                },
            };
            self.env.declare(&p.name.name, value);
        }
        Ok(())
    }
}

/// `ban <user> [reason]`
fn usage(cmd: &CommandDef) -> String {
    let parts: Vec<String> = cmd
        .params
        .iter()
        .map(|p| {
            if p.default.is_some() {
                format!("[{}]", p.name.name)
            } else {
                format!("<{}>", p.name.name)
            }
        })
        .collect();
    format!("uso: {} {}", cmd.name.node, parts.join(" "))
}

/// Converts a host-provided argument to the declared type.
/// Converts a value to a declared type (command arguments and typed `fn` parameters).
/// `owner` is e.g. "do comando `ban`"; `span` is where the arrow points.
pub(crate) fn coerce(name: &str, ty: ArgType, span: Span, v: Value, owner: &str) -> R<Value> {
    let bad = |what: &str, v: &Value| {
        RuntimeError::new(
            codes::E0302,
            format!(
                "O argumento `{name}` {owner} precisa ser {what}, mas recebeu {}",
                v.type_name()
            ),
            span,
            format!("declarado como {ty}"),
        )
    };
    Ok(match ty {
        ArgType::Text => match v {
            Value::Text(_) => v,
            Value::Int(_) | Value::Float(_) | Value::Bool(_) => Value::Text(v.to_string()),
            other => return Err(bad("um texto", &other)),
        },
        ArgType::Number => match &v {
            Value::Int(_) | Value::Float(_) => v,
            Value::Text(s) => {
                let s = s.trim().replace(',', ".");
                match s.parse::<i64>() {
                    Ok(i) => Value::Int(i),
                    Err(_) => match s.parse::<f64>() {
                        Ok(f) if f.is_finite() => Value::Float(f),
                        _ => return Err(bad("um número", &v)),
                    },
                }
            }
            other => return Err(bad("um número", other)),
        },
        ArgType::Bool => match &v {
            Value::Bool(_) => v,
            Value::Text(s) => match s.trim().to_lowercase().as_str() {
                "true" | "sim" | "yes" | "1" | "on" => Value::Bool(true),
                "false" | "não" | "nao" | "no" | "0" | "off" => Value::Bool(false),
                _ => return Err(bad("true ou false", &v)),
            },
            other => return Err(bad("true ou false", other)),
        },
        ArgType::Member | ArgType::User | ArgType::Channel | ArgType::Role => {
            if v.entity_id().is_some() {
                v
            } else {
                return Err(bad(&format!("um {ty}"), &v));
            }
        }
    })
}
