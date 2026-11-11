//! Expressions and calls.

use crate::engine::{Machine, R};
use crate::error::RuntimeError;
use crate::host::BoxFuture;
use diracord_ast::{Arg, BinaryOp, Expr, ExprKind, Literal, TemplatePart, UnaryOp};
use diracord_diagnostics::{codes, did_you_mean, Span};
use diracord_functions::{ArgRef, Args, DataOp, Effect, FnError, FunctionSpec, ParamKind};
use diracord_value::{ops, Map, OpError, Scope, StoreKey, Value};
use std::time::Instant;

/// The runtime value of a literal.
pub(crate) fn literal(l: &Literal) -> Value {
    match l {
        Literal::Int(i) => Value::Int(*i),
        Literal::Float(f) => Value::Float(*f),
        Literal::Text(s) => Value::Text(s.clone()),
        Literal::Bool(b) => Value::Bool(*b),
        Literal::Null => Value::Null,
    }
}

fn op_error(e: OpError, span: Span) -> RuntimeError {
    match e {
        OpError::DivisionByZero => {
            RuntimeError::new(codes::E0203, "Divisão por zero", span, "o divisor é 0")
                .with_help("confira o valor antes com if")
        }
        other => RuntimeError::new(
            codes::E0202,
            format!("Tipo errado: {other}"),
            span,
            "tipos incompatíveis",
        )
        .with_help(
            "para juntar texto e número use +; para converter, use toNumber(...) ou toText(...)",
        ),
    }
}

impl<'r> Machine<'r> {
    pub(crate) fn eval<'a>(&'a mut self, e: &'a Expr) -> BoxFuture<'a, R<Value>> {
        Box::pin(async move {
            self.tick(e.span)?;
            match &e.kind {
                ExprKind::Literal(l) => Ok(literal(l)),
                ExprKind::Name(id) => {
                    if let Some(v) = self.env.get(&id.name) {
                        return Ok(v.clone());
                    }
                    // A function name alone is a call with no arguments (`deleteMessage`).
                    if let Some(spec) = self.opts.registry.get(&id.name) {
                        return self.call(spec, &[], e.span).await;
                    }
                    Err(self.undefined(&id.name, e.span))
                }
                ExprKind::Template(parts) => {
                    let mut out = String::new();
                    for p in parts {
                        match p {
                            TemplatePart::Text(s) => out.push_str(s),
                            TemplatePart::Expr(x) => {
                                let v = self.eval(x).await?;
                                out.push_str(&v.to_string());
                            }
                        }
                        self.check_size(out.len(), e.span)?;
                    }
                    Ok(Value::Text(out))
                }
                ExprKind::List(items) => {
                    let mut out = Vec::with_capacity(items.len());
                    for i in items {
                        out.push(self.eval(i).await?);
                    }
                    Ok(Value::List(out))
                }
                ExprKind::Object(fields) => {
                    let mut m = Map::new();
                    for f in fields {
                        let v = self.eval(&f.value).await?;
                        m.insert(f.key.name.clone(), v);
                    }
                    Ok(Value::Object(m))
                }
                ExprKind::Unary { op, expr } => {
                    let v = self.eval(expr).await?;
                    match op {
                        UnaryOp::Neg => ops::neg(&v).map_err(|err| op_error(err, e.span)),
                        UnaryOp::Not => Ok(ops::not(&v)),
                    }
                }
                ExprKind::Binary { op, lhs, rhs } => self.binary(*op, lhs, rhs, e.span).await,
                ExprKind::Member { object, field } => {
                    let v = self.eval(object).await?;
                    member(v, &field.name, field.span)
                }
                ExprKind::Index { object, index } => {
                    let v = self.eval(object).await?;
                    let i = self.eval(index).await?;
                    index_value(v, &i, e.span)
                }
                ExprKind::Layout(l) => self.eval_layout(l).await,
                ExprKind::Call { callee, args, .. } => {
                    let Some(id) = callee.as_ident() else {
                        return Err(RuntimeError::new(
                            codes::E0207,
                            "Isso não é uma função",
                            callee.span,
                            "não dá para chamar",
                        )
                        .with_help("só nomes de funções podem ser chamados, como reply(...)"));
                    };
                    if self.env.get(&id.name).is_some()
                        && self.opts.registry.get(&id.name).is_none()
                    {
                        return Err(RuntimeError::new(
                            codes::E0207,
                            format!("`{}` é uma variável, não uma função", id.name),
                            id.span,
                            "variável",
                        )
                        .with_help("talvez falte um operador entre os valores"));
                    }
                    match self.opts.registry.get(&id.name) {
                        Some(spec) => self.call(spec, args, e.span).await,
                        None => Err(self.unknown_function(&id.name, id.span)),
                    }
                }
            }
        })
    }

    async fn binary(&mut self, op: BinaryOp, lhs: &Expr, rhs: &Expr, span: Span) -> R<Value> {
        let a = self.eval(lhs).await?;
        match op {
            BinaryOp::And if !a.is_truthy() => return Ok(Value::Bool(false)),
            BinaryOp::Or if a.is_truthy() => return Ok(Value::Bool(true)),
            BinaryOp::And | BinaryOp::Or => {
                return Ok(Value::Bool(self.eval(rhs).await?.is_truthy()))
            }
            _ => {}
        }
        let b = self.eval(rhs).await?;
        let cmp = |o: &'static str| ops::compare(o, &a, &b).map_err(|err| op_error(err, span));
        let v = match op {
            BinaryOp::Add => ops::add(&a, &b),
            BinaryOp::Sub => ops::sub(&a, &b),
            BinaryOp::Mul => ops::mul(&a, &b),
            BinaryOp::Div => ops::div(&a, &b),
            BinaryOp::Rem => ops::rem(&a, &b),
            BinaryOp::Eq => Ok(Value::Bool(ops::equals(&a, &b))),
            BinaryOp::Ne => Ok(Value::Bool(!ops::equals(&a, &b))),
            BinaryOp::Lt => return Ok(Value::Bool(cmp("<")?.is_lt())),
            BinaryOp::Le => return Ok(Value::Bool(cmp("<=")?.is_le())),
            BinaryOp::Gt => return Ok(Value::Bool(cmp(">")?.is_gt())),
            BinaryOp::Ge => return Ok(Value::Bool(cmp(">=")?.is_ge())),
            BinaryOp::And | BinaryOp::Or => unreachable!("handled above"),
        }
        .map_err(|err| op_error(err, span))?;
        self.check_size(v.approx_size(), span)?;
        Ok(v)
    }

    fn undefined(&self, name: &str, span: Span) -> RuntimeError {
        let mut e = RuntimeError::new(
            codes::E0200,
            format!("A variável `{name}` não existe"),
            span,
            "variável não criada",
        )
        .with_help(format!("crie antes com `local {name} = ...`"));
        let mut names = self.env.names();
        for n in self.opts.registry.names() {
            names.push(n);
        }
        if let Some(s) = did_you_mean(name, names.iter().copied()) {
            e = e.with_help(format!("você quis dizer `{s}`?"));
        }
        e
    }

    fn unknown_function(&self, name: &str, span: Span) -> RuntimeError {
        let mut e = RuntimeError::new(
            codes::E0201,
            format!("A função `{name}` não existe"),
            span,
            "função desconhecida",
        );
        if let Some(s) = did_you_mean(name, self.opts.registry.names()) {
            e = e.with_help(format!("você quis dizer `{s}`?"));
        }
        e
    }

    /// Evaluates arguments, runs the built-in and carries out its effect.
    async fn call(&mut self, spec: &'static FunctionSpec, args: &[Arg], span: Span) -> R<Value> {
        let mut raw = Args::default();
        let mut pos_spans = Vec::new();
        let mut named_spans = Vec::new();
        for a in args {
            let param = match &a.name {
                Some(n) => spec.params.iter().find(|p| p.name == n.name),
                None => spec.params.get(raw.positional.len()),
            };
            // `scope: user` means the scope keyword, not the `user` variable.
            let v = match (param, a.value.as_ident()) {
                (Some(p), Some(id))
                    if p.kind == ParamKind::Scope && Scope::from_name(&id.name).is_some() =>
                {
                    Value::Text(id.name.clone())
                }
                _ => self.eval(&a.value).await?,
            };
            match &a.name {
                Some(n) => {
                    raw.named.push((n.name.clone(), v));
                    named_spans.push((n.name.clone(), a.span));
                }
                None => {
                    raw.positional.push(v);
                    pos_spans.push(a.span);
                }
            }
        }
        let npos = raw.positional.len();
        let effect = spec.invoke(raw).map_err(|err| {
            let at = match &err.arg {
                Some(ArgRef::Positional(i)) => pos_spans.get(*i).copied(),
                Some(ArgRef::Named(n)) => named_spans.iter().find(|(m, _)| m == n).map(|x| x.1),
                Some(ArgRef::Param(i)) if *i < npos => pos_spans.get(*i).copied(),
                Some(ArgRef::Param(i)) => {
                    let name = spec.params.get(*i).map(|p| p.name).unwrap_or("");
                    named_spans.iter().find(|(m, _)| m == name).map(|x| x.1)
                }
                None => None,
            };
            fn_error(err, at.unwrap_or(span))
        })?;
        match effect {
            Effect::Value(v) => Ok(v),
            Effect::Action(action) => {
                self.meter.actions += 1;
                if self.meter.actions > self.opts.limits.max_actions {
                    return Err(RuntimeError::limit(
                        codes::E0205,
                        format!(
                            "O código fez mais de {} ações no Discord e foi parado",
                            self.opts.limits.max_actions
                        ),
                        span,
                        "procure um laço que manda mensagens demais",
                    ));
                }
                let name = action.name.clone();
                let t0 = Instant::now();
                let r = self.host.perform(action).await;
                self.meter.add_host_time(t0.elapsed());
                r.map_err(|err| {
                    RuntimeError::new(
                        codes::E0208,
                        format!("O Discord recusou `{name}`: {}", err.message),
                        span,
                        "esta ação falhou",
                    )
                    .with_help("use testing { ... } catch err { ... } para tratar")
                })
            }
            Effect::Data(op) => self.data(op, span),
        }
    }

    fn data(&mut self, op: DataOp, span: Span) -> R<Value> {
        let key = |m: &Self, name: String, scope: Option<Scope>| -> R<StoreKey> {
            let scope = scope.unwrap_or(m.opts.default_scope);
            let missing = |what: &str| {
                RuntimeError::new(
                    codes::E0402,
                    format!("O escopo `{scope}` precisa de {what}, e aqui não tem"),
                    span,
                    "sem contexto",
                )
                .with_help(
                    "numa mensagem privada não existe servidor; use scope: user ou scope: global",
                )
            };
            let owner = match scope {
                Scope::User => m.ctx.user_id().ok_or_else(|| missing("um usuário"))?,
                Scope::Server => m.ctx.server_id().ok_or_else(|| missing("um servidor"))?,
                Scope::Member => {
                    let s = m.ctx.server_id().ok_or_else(|| missing("um servidor"))?;
                    let u = m.ctx.user_id().ok_or_else(|| missing("um usuário"))?;
                    format!("{s}:{u}")
                }
                Scope::Global => String::new(),
            };
            Ok(StoreKey::new(scope, owner, name))
        };
        let store_err = |e: diracord_value::StoreError| {
            RuntimeError::new(
                codes::E0400,
                format!("Erro no banco de dados: {e}"),
                span,
                "aqui",
            )
        };
        match op {
            DataOp::Get {
                name,
                scope,
                default,
            } => {
                let k = key(self, name, scope)?;
                Ok(self.store.get(&k).map_err(store_err)?.unwrap_or(default))
            }
            DataOp::Set { name, scope, value } => {
                if value.approx_size() > self.opts.limits.max_saved_value {
                    return Err(RuntimeError::new(
                        codes::E0302,
                        format!(
                            "Esse valor é grande demais para salvar (máximo {} KB)",
                            self.opts.limits.max_saved_value / 1024
                        ),
                        span,
                        "valor grande demais",
                    ));
                }
                let k = key(self, name, scope)?;
                self.store.set(&k, value).map_err(store_err)?;
                Ok(Value::Null)
            }
            DataOp::Delete { name, scope } => {
                let k = key(self, name, scope)?;
                Ok(Value::Bool(self.store.delete(&k).map_err(store_err)?))
            }
        }
    }
}

fn fn_error(err: FnError, span: Span) -> RuntimeError {
    let mut e = RuntimeError::new(err.code, err.message, span, "aqui");
    if let Some(h) = err.help {
        e = e.with_help(h);
    }
    e
}

fn member(v: Value, field: &str, span: Span) -> R<Value> {
    match v {
        Value::Object(mut m) => match m.remove(field) {
            Some(x) => Ok(x),
            None => {
                let mut e = RuntimeError::new(codes::E0209, format!("O campo `{field}` não existe"), span, "campo desconhecido");
                if let Some(s) = did_you_mean(field, m.keys().map(String::as_str)) {
                    e = e.with_help(format!("você quis dizer `{s}`?"));
                } else if !m.is_empty() {
                    let keys: Vec<&str> = m.keys().map(String::as_str).take(12).collect();
                    e = e.with_help(format!("campos disponíveis: {}", keys.join(", ")));
                }
                Err(e)
            }
        },
        Value::Null => Err(RuntimeError::new(
            codes::E0209,
            format!("Não dá para ler `.{field}` de null"),
            span,
            "o valor é null",
        )
        .with_help("o valor não existe (por exemplo, channel(\"x\") não achou o canal); confira com if antes")),
        other => Err(RuntimeError::new(
            codes::E0209,
            format!("{} não tem o campo `{field}`", other.type_name()),
            span,
            "só objetos têm campos",
        )
        .with_help(if field == "length" { "para o tamanho use len(...)" } else { "confira o nome da variável" })),
    }
}

fn index_value(v: Value, i: &Value, span: Span) -> R<Value> {
    let out_of_range = |len: usize| {
        RuntimeError::new(
            codes::E0209,
            format!("A posição {i} não existe (tamanho {len})"),
            span,
            "fora do limite",
        )
        .with_help("posições começam em 0")
    };
    match (v, i) {
        (Value::List(l), _) if i.as_i64().is_some() => {
            let n = i.as_i64().unwrap_or(-1);
            let len = l.len();
            if n < 0 {
                return Err(out_of_range(len));
            }
            l.into_iter()
                .nth(n as usize)
                .ok_or_else(|| out_of_range(len))
        }
        (Value::Text(s), _) if i.as_i64().is_some() => {
            let n = i.as_i64().unwrap_or(-1);
            let len = s.chars().count();
            if n < 0 {
                return Err(out_of_range(len));
            }
            s.chars()
                .nth(n as usize)
                .map(|c| Value::Text(c.to_string()))
                .ok_or_else(|| out_of_range(len))
        }
        (Value::Object(mut m), Value::Text(k)) => Ok(m.remove(k).unwrap_or(Value::Null)),
        (v, i) => Err(RuntimeError::new(
            codes::E0202,
            format!("Não dá para usar [{}] em {}", i.type_name(), v.type_name()),
            span,
            "índice inválido",
        )),
    }
}
