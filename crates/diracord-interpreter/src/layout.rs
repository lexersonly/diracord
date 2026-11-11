//! Evaluates `layout { ... }` into the object form `{ layout: [...] }`
//! (see `diracord_functions::layout`, which validates it).

use crate::engine::{Machine, R};
use crate::error::RuntimeError;
use crate::host::BoxFuture;
use diracord_ast::{Arg, ElementKind, Layout, LayoutItemKind};
use diracord_diagnostics::{codes, did_you_mean, Span};
use diracord_value::{Map, Value};

/// Binds element arguments to `kind.params()`: positional first, then by name.
pub(crate) fn bind_names(
    kind: ElementKind,
    args: &[Arg],
    span: Span,
) -> Result<Vec<(usize, &Arg)>, RuntimeError> {
    let params = kind.params();
    let mut out: Vec<(usize, &Arg)> = Vec::new();
    let mut pos = 0;
    for a in args {
        let i = match &a.name {
            None => {
                pos += 1;
                if pos > params.len() {
                    return Err(RuntimeError::new(
                        codes::E0301,
                        format!(
                            "`{}` aceita no máximo {} argumento(s) sem nome",
                            kind.as_str(),
                            params.len()
                        ),
                        a.span,
                        "argumento a mais",
                    )
                    .with_help(usage(kind)));
                }
                pos - 1
            }
            Some(n) => match params.iter().position(|p| p.0 == n.name) {
                Some(i) => i,
                None => {
                    let mut e = RuntimeError::new(
                        codes::E0303,
                        format!("`{}` não tem a opção `{}`", kind.as_str(), n.name),
                        n.span,
                        "opção desconhecida",
                    );
                    e = match did_you_mean(&n.name, params.iter().map(|p| p.0)) {
                        Some(s) => e.with_help(format!("você quis dizer `{s}:`?")),
                        None => e.with_help(usage(kind)),
                    };
                    return Err(e);
                }
            },
        };
        if out.iter().any(|(j, _)| *j == i) {
            return Err(RuntimeError::new(
                codes::E0302,
                format!("`{}` foi passado duas vezes", params[i].0),
                a.span,
                "repetido",
            ));
        }
        out.push((i, a));
    }
    for (i, p) in params.iter().enumerate() {
        if p.1 && !out.iter().any(|(j, _)| *j == i) {
            return Err(RuntimeError::new(
                codes::E0300,
                format!("Faltou `{}` em `{}`", p.0, kind.as_str()),
                span,
                "falta argumento",
            )
            .with_help(usage(kind)));
        }
    }
    Ok(out)
}

/// `button(id?, label?, style?, ...)`
pub(crate) fn usage(kind: ElementKind) -> String {
    let ps: Vec<String> = kind
        .params()
        .iter()
        .map(|(n, req)| if *req { n.to_string() } else { format!("{n}?") })
        .collect();
    format!("uso: {}({})", kind.as_str(), ps.join(", "))
}

impl<'r> Machine<'r> {
    /// `layout { ... }` → `{ layout: [...] }`.
    pub(crate) async fn eval_layout(&mut self, l: &Layout) -> R<Value> {
        let items = self.layout_items(l).await?;
        Ok(Value::object([("layout", Value::List(items))]))
    }

    /// Items of one block, in their object form. `local`s live until the end of the block.
    fn layout_items<'a>(&'a mut self, l: &'a Layout) -> BoxFuture<'a, R<Vec<Value>>> {
        Box::pin(async move {
            self.env.push();
            let r = self.layout_items_inner(l).await;
            self.env.pop();
            r
        })
    }

    async fn layout_items_inner(&mut self, l: &Layout) -> R<Vec<Value>> {
        let mut out = Vec::new();
        for item in &l.items {
            self.tick(item.span)?;
            match &item.kind {
                LayoutItemKind::Local { name, value } => {
                    let v = self.eval(value).await?;
                    self.env.declare(&name.name, v);
                    self.check_memory(item.span)?;
                }
                LayoutItemKind::If {
                    branches,
                    otherwise,
                } => {
                    let mut done = false;
                    for b in branches {
                        if self.eval(&b.cond).await?.is_truthy() {
                            out.extend(self.layout_items(&b.body).await?);
                            done = true;
                            break;
                        }
                    }
                    if let (false, Some(o)) = (done, otherwise) {
                        out.extend(self.layout_items(o).await?);
                    }
                }
                LayoutItemKind::For { var, iter, body } => {
                    let items = match self.eval(iter).await? {
                        Value::List(l) => l,
                        Value::Int(n) if n >= 0 => (0..n.min(1000)).map(Value::Int).collect(),
                        Value::Text(s) => s.chars().map(|c| Value::Text(c.to_string())).collect(),
                        Value::Null => Vec::new(),
                        other => {
                            return Err(RuntimeError::new(
                                codes::E0202,
                                format!("Não dá para usar `for` em {}", other.type_name()),
                                iter.span,
                                "esperava uma lista",
                            ))
                        }
                    };
                    for x in items {
                        self.env.push();
                        self.env.declare(&var.name, x);
                        let r = self.layout_items(body).await;
                        self.env.pop();
                        out.extend(r?);
                    }
                }
                LayoutItemKind::Element {
                    kind,
                    args,
                    children,
                    ..
                } => {
                    out.push(
                        self.element(*kind, args, children.as_ref(), item.span)
                            .await?,
                    );
                }
            }
            self.check_size(out.iter().map(Value::approx_size).sum(), item.span)?;
        }
        Ok(out)
    }

    async fn element(
        &mut self,
        kind: ElementKind,
        args: &[Arg],
        children: Option<&Layout>,
        span: Span,
    ) -> R<Value> {
        let params = kind.params();
        let mut opts = Map::new();
        for (i, a) in bind_names(kind, args, span)? {
            let v = self.eval(&a.value).await?;
            if v != Value::Null {
                opts.insert(params[i].0.to_string(), v);
            }
        }
        let kids = match children {
            Some(c) => self.layout_items(c).await?,
            None => Vec::new(),
        };
        let take = |opts: &mut Map, k: &str| opts.remove(k).unwrap_or(Value::Null);
        let mut obj = Map::new();
        match kind {
            ElementKind::Container => {
                obj.insert("container".into(), Value::List(kids));
                obj.extend(opts);
            }
            ElementKind::Row => {
                obj.insert("row".into(), Value::List(kids));
            }
            ElementKind::Section => {
                let mut texts = Vec::new();
                for k in kids {
                    let Value::Object(mut m) = k else { continue };
                    if let Some(t) = m.remove("text") {
                        texts.push(t);
                        continue;
                    }
                    let accessory = if m.contains_key("thumbnail") {
                        "thumbnail"
                    } else {
                        "button"
                    };
                    if obj.contains_key("thumbnail") || obj.contains_key("button") {
                        return Err(RuntimeError::new(
                            codes::E0117,
                            "Uma `section` aceita só um acessório (`thumbnail` ou `button`)",
                            span,
                            "acessório a mais",
                        ));
                    }
                    let value = if accessory == "thumbnail" {
                        let url = m.remove("thumbnail").unwrap_or(Value::Null);
                        if m.is_empty() {
                            url
                        } else {
                            m.insert("url".into(), url);
                            Value::Object(m)
                        }
                    } else {
                        Value::Object(m)
                    };
                    obj.insert(accessory.into(), value);
                }
                obj.insert("section".into(), Value::List(texts));
            }
            ElementKind::Text => {
                let t = take(&mut opts, "content");
                obj.insert("text".into(), Value::Text(t.to_string()));
            }
            ElementKind::Thumbnail => {
                obj.insert("thumbnail".into(), take(&mut opts, "url"));
                obj.extend(opts);
            }
            ElementKind::Separator => {
                obj.insert("separator".into(), Value::Bool(true));
                obj.extend(opts);
            }
            ElementKind::Gallery => {
                obj.insert("gallery".into(), take(&mut opts, "items"));
            }
            ElementKind::File => {
                obj.insert("file".into(), take(&mut opts, "url"));
                obj.extend(opts);
            }
            ElementKind::Button => {
                obj.insert("button".into(), take(&mut opts, "id"));
                obj.extend(opts);
            }
            ElementKind::Menu => {
                obj.insert("menu".into(), take(&mut opts, "id"));
                obj.extend(opts);
            }
        }
        Ok(Value::Object(obj))
    }
}
