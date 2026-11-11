//! Statements.

use crate::engine::{Flow, Machine, R};
use crate::error::RuntimeError;
use crate::host::BoxFuture;
use diracord_ast::{Block, Expr, ExprKind, PatternKind, Stmt, StmtKind};
use diracord_diagnostics::{codes, did_you_mean};
use diracord_value::{ops, Value};

/// One step on the way from a variable to the place being assigned.
enum Seg {
    Field(String),
    Index(Value),
}

impl<'r> Machine<'r> {
    /// Runs a block in its own scope.
    pub(crate) fn exec_block<'a>(&'a mut self, block: &'a Block) -> BoxFuture<'a, R<Flow>> {
        Box::pin(async move {
            self.env.push();
            let mut result = Ok(Flow::Normal);
            for s in &block.stmts {
                match self.exec(s).await {
                    Ok(Flow::Normal) => {}
                    other => {
                        result = other;
                        break;
                    }
                }
            }
            self.env.pop();
            result
        })
    }

    async fn exec(&mut self, s: &Stmt) -> R<Flow> {
        self.tick(s.span)?;
        match &s.kind {
            StmtKind::Local { name, value } => {
                let v = self.eval(value).await?;
                self.env.declare(&name.name, v);
                self.check_memory(s.span)?;
            }
            StmtKind::Assign { target, value } => {
                let v = self.eval(value).await?;
                self.assign(target, v).await?;
                self.check_memory(s.span)?;
            }
            StmtKind::Expr(e) => {
                self.eval(e).await?;
            }
            StmtKind::If {
                branches,
                otherwise,
            } => {
                for b in branches {
                    if self.eval(&b.cond).await?.is_truthy() {
                        return self.exec_block(&b.body).await;
                    }
                }
                if let Some(o) = otherwise {
                    return self.exec_block(o).await;
                }
            }
            StmtKind::For { var, iter, body } => {
                let items = self.iterate(iter).await?;
                for item in items {
                    self.tick(var.span)?;
                    self.env.push();
                    self.env.declare(&var.name, item);
                    let flow = self.exec_block(body).await;
                    self.env.pop();
                    match flow? {
                        Flow::Break => break,
                        Flow::Normal | Flow::Continue => {}
                        f => return Ok(f),
                    }
                }
            }
            StmtKind::While { cond, body } => loop {
                if !self.eval(cond).await?.is_truthy() {
                    break;
                }
                match self.exec_block(body).await? {
                    Flow::Break => break,
                    Flow::Normal | Flow::Continue => {}
                    f => return Ok(f),
                }
            },
            StmtKind::Chain { subject, arms } => {
                let v = self.eval(subject).await?;
                for arm in arms {
                    let hit = match &arm.pattern.kind {
                        PatternKind::Wildcard => true,
                        PatternKind::Literal(l) => ops::equals(&v, &crate::eval::literal(l)),
                    };
                    if hit {
                        return self.exec_block(&arm.body).await;
                    }
                }
            }
            StmtKind::Try {
                body,
                catch_var,
                handler,
            } => {
                return match self.exec_block(body).await {
                    Err(e) if e.catchable => {
                        self.env.push();
                        if let Some(var) = catch_var {
                            self.env
                                .declare(&var.name, Value::Text(e.message().to_string()));
                        }
                        let r = self.exec_block(handler).await;
                        self.env.pop();
                        r
                    }
                    other => other,
                };
            }
            StmtKind::Return(e) => {
                let v = match e {
                    Some(e) => self.eval(e).await?,
                    None => Value::Null,
                };
                return Ok(Flow::Return(v));
            }
            StmtKind::Stop => return Ok(Flow::Stop),
            StmtKind::Break => return Ok(Flow::Break),
            StmtKind::Continue => return Ok(Flow::Continue),
        }
        Ok(Flow::Normal)
    }

    /// Values a `for` walks: list items, text characters, `0..n` for a number, object keys.
    async fn iterate(&mut self, e: &Expr) -> R<Vec<Value>> {
        let v = self.eval(e).await?;
        Ok(match v {
            Value::List(l) => l,
            Value::Text(s) => s.chars().map(|c| Value::Text(c.to_string())).collect(),
            Value::Object(m) => m.into_keys().map(Value::Text).collect(),
            Value::Int(n) if n >= 0 => {
                let n = n.min(self.opts.limits.max_steps as i64);
                (0..n).map(Value::Int).collect()
            }
            Value::Null => Vec::new(),
            other => {
                return Err(RuntimeError::new(
                    codes::E0202,
                    format!("Não dá para usar `for` em {}", other.type_name()),
                    e.span,
                    "esperava uma lista, um texto ou um número",
                ))
            }
        })
    }

    async fn assign(&mut self, target: &Expr, value: Value) -> R<()> {
        // Collect the path from the root variable, evaluating indexes first.
        let mut segs = Vec::new();
        let mut cur = target;
        let root = loop {
            match &cur.kind {
                ExprKind::Name(id) => break id,
                ExprKind::Member { object, field } => {
                    segs.push(Seg::Field(field.name.clone()));
                    cur = object;
                }
                ExprKind::Index { object, index } => {
                    let i = self.eval(index).await?;
                    segs.push(Seg::Index(i));
                    cur = object;
                }
                _ => {
                    return Err(RuntimeError::new(
                        codes::E0106,
                        "Não dá para guardar um valor aqui",
                        cur.span,
                        "não é uma variável",
                    ))
                }
            }
        };
        segs.reverse();
        let names: Vec<String> = self.env.names().into_iter().map(String::from).collect();
        let Some(mut slot) = self.env.get_mut(&root.name) else {
            let mut e = RuntimeError::new(
                codes::E0200,
                format!("A variável `{}` não existe", root.name),
                root.span,
                "variável não criada",
            )
            .with_help(format!("crie antes com `local {} = ...`", root.name));
            if let Some(s) = did_you_mean(&root.name, names.iter().map(String::as_str)) {
                e = e.with_help(format!("você quis dizer `{s}`?"));
            }
            return Err(e);
        };
        for seg in &segs {
            slot = match (slot, seg) {
                (Value::Object(m), Seg::Field(f)) => m.entry(f.clone()).or_insert(Value::Null),
                (Value::Object(m), Seg::Index(Value::Text(k))) => {
                    m.entry(k.clone()).or_insert(Value::Null)
                }
                (Value::List(l), Seg::Index(i)) => {
                    let len = l.len();
                    match i.as_i64().filter(|&n| n >= 0 && (n as usize) < len) {
                        Some(n) => &mut l[n as usize],
                        None => return Err(RuntimeError::new(
                            codes::E0209,
                            format!("A posição {i} não existe numa lista de {len} itens"),
                            target.span,
                            "posição fora da lista",
                        )
                        .with_help(
                            "posições começam em 0; para adicionar no fim, use push(lista, valor)",
                        )),
                    }
                }
                (other, _) => {
                    return Err(RuntimeError::new(
                        codes::E0202,
                        format!("Não dá para mudar um campo de {}", other.type_name()),
                        target.span,
                        "só objetos e listas têm campos",
                    ))
                }
            };
        }
        *slot = value;
        self.env.resize(&root.name);
        Ok(())
    }
}
