//! Static checks that need more than syntax: unknown functions/events/variables,
//! argument counts, `break` outside loops, repeated names.

use diracord_ast::{
    known, Arg, Block, Expr, ExprKind, FunctionDef, ItemKind, Program, Stmt, StmtKind, TemplatePart,
};
use diracord_diagnostics::{codes, did_you_mean, Diagnostic, FileId, Fix, Span};
use diracord_functions::{ParamKind, Registry};
use diracord_value::Scope;
use std::collections::HashMap;

/// Checks a (merged) program. Returns each diagnostic with the file it belongs to.
pub fn check(program: &Program, registry: &Registry) -> Vec<(FileId, Diagnostic)> {
    let mut out = Vec::new();
    let mut commands: HashMap<&str, (FileId, Span)> = HashMap::new();
    let mut interactions: HashMap<(&str, &str), (FileId, Span)> = HashMap::new();
    let mut fns: HashMap<&str, &FunctionDef> = HashMap::new();
    for item in &program.items {
        let ItemKind::Function(f) = &item.kind else {
            continue;
        };
        let name = f.name.name.as_str();
        if registry.get(name).is_some() {
            out.push((
                item.file,
                Diagnostic::error(
                    codes::E0116,
                    format!("`{name}` já é uma função embutida do Diracord"),
                )
                .with_label(f.name.span, "nome já usado")
                .with_help("escolha outro nome para a sua função"),
            ));
        } else if fns.insert(name, f).is_some() {
            out.push((
                item.file,
                Diagnostic::error(
                    codes::E0116,
                    format!("Já existe outra função chamada `{name}`"),
                )
                .with_label(f.name.span, "nome repetido"),
            ));
        }
    }
    for item in &program.items {
        let mut c = Checker {
            registry,
            fns: &fns,
            scopes: Vec::new(),
            loops: 0,
            diags: Vec::new(),
        };
        let mut vars: Vec<String> = known::AUTO_VARS.iter().map(|s| s.to_string()).collect();
        let body = match &item.kind {
            ItemKind::Command(cmd) => {
                let name = cmd.name.node.as_str();
                if let Some((f, s)) = commands.insert(name, (item.file, cmd.name.span)) {
                    out.push((
                        item.file,
                        Diagnostic::error(
                            codes::E0116,
                            format!("Já existe outro comando chamado `{name}`"),
                        )
                        .with_label(cmd.name.span, "nome repetido")
                        .with_note(format!(
                            "o primeiro está no arquivo {} (posição {})",
                            f.0, s.start
                        )),
                    ));
                }
                c.scopes.push(vars.clone());
                for p in &cmd.params {
                    if let Some(d) = &p.default {
                        c.expr(d);
                    }
                    vars.push(p.name.name.clone());
                }
                &cmd.body
            }
            ItemKind::Event(e) => {
                if !known::is_event(&e.name.name) {
                    let names = known::EVENTS.iter().map(|x| x.name);
                    let mut d = Diagnostic::error(
                        codes::E0113,
                        format!("`{}` não é um evento conhecido", e.name.name),
                    )
                    .with_label(e.name.span, "evento desconhecido");
                    match did_you_mean(&e.name.name, names) {
                        Some(s) => {
                            let only_case = s.eq_ignore_ascii_case(&e.name.name);
                            d = d.with_help(format!("você quis dizer `{s}`?")).with_fix(
                                Fix::replace(
                                    format!("trocar por `{s}`"),
                                    e.name.span,
                                    s,
                                    only_case,
                                ),
                            );
                        }
                        None => {
                            let list: Vec<&str> = known::EVENTS.iter().map(|x| x.name).collect();
                            d = d.with_note(format!("eventos: {}", list.join(", ")));
                        }
                    }
                    out.push((item.file, d));
                }
                &e.body
            }
            ItemKind::Interaction(i) => {
                let key = (i.kind.as_str(), i.id.node.as_str());
                if interactions.insert(key, (item.file, i.id.span)).is_some() {
                    out.push((
                        item.file,
                        Diagnostic::error(
                            codes::E0116,
                            format!("Já existe outro {} com o id `{}`", key.0, key.1),
                        )
                        .with_label(i.id.span, "id repetido"),
                    ));
                }
                &i.body
            }
            ItemKind::Function(f) => {
                c.scopes.push(vars.clone());
                for p in &f.params {
                    if let Some(d) = &p.default {
                        c.expr(d);
                    }
                    c.declare(&p.name.name);
                    vars.push(p.name.name.clone());
                }
                c.scopes.clear();
                &f.body
            }
        };
        c.scopes = vec![vars];
        c.block(body);
        out.extend(c.diags.into_iter().map(|d| (item.file, d)));
    }
    out
}

struct Checker<'a> {
    registry: &'a Registry,
    fns: &'a HashMap<&'a str, &'a FunctionDef>,
    scopes: Vec<Vec<String>>,
    loops: u32,
    diags: Vec<Diagnostic>,
}

impl Checker<'_> {
    fn declared(&self, name: &str) -> bool {
        self.scopes.iter().any(|s| s.iter().any(|n| n == name))
    }

    fn declare(&mut self, name: &str) {
        if let Some(s) = self.scopes.last_mut() {
            s.push(name.to_string());
        }
    }

    fn block(&mut self, b: &Block) {
        self.scopes.push(Vec::new());
        for s in &b.stmts {
            self.stmt(s);
        }
        self.scopes.pop();
    }

    fn stmt(&mut self, s: &Stmt) {
        match &s.kind {
            StmtKind::Local { name, value } => {
                self.expr(value);
                self.declare(&name.name);
            }
            StmtKind::Assign { target, value } => {
                self.expr(value);
                match &target.kind {
                    ExprKind::Name(id) if !self.declared(&id.name) => {
                        self.diags.push(
                            Diagnostic::error(
                                codes::E0200,
                                format!("A variável `{}` não existe", id.name),
                            )
                            .with_label(id.span, "variável não criada")
                            .with_help(format!("para criar, use `local {} = ...`", id.name))
                            .with_fix(Fix::replace(
                                "adicionar `local`",
                                Span::point(id.span.start),
                                "local ",
                                true,
                            )),
                        );
                        self.declare(&id.name);
                    }
                    _ => self.expr(target),
                }
            }
            StmtKind::Expr(e) => self.expr(e),
            StmtKind::If {
                branches,
                otherwise,
            } => {
                for b in branches {
                    self.expr(&b.cond);
                    self.block(&b.body);
                }
                if let Some(o) = otherwise {
                    self.block(o);
                }
            }
            StmtKind::For { var, iter, body } => {
                self.expr(iter);
                self.loops += 1;
                self.scopes.push(vec![var.name.clone()]);
                self.block(body);
                self.scopes.pop();
                self.loops -= 1;
            }
            StmtKind::While { cond, body } => {
                self.expr(cond);
                self.loops += 1;
                self.block(body);
                self.loops -= 1;
            }
            StmtKind::Chain { subject, arms } => {
                self.expr(subject);
                for a in arms {
                    self.block(&a.body);
                }
            }
            StmtKind::Testing {
                body,
                catch_var,
                handler,
            } => {
                self.block(body);
                self.scopes
                    .push(catch_var.iter().map(|v| v.name.clone()).collect());
                self.block(handler);
                self.scopes.pop();
            }
            StmtKind::Return(e) => {
                if let Some(e) = e {
                    self.expr(e);
                }
            }
            StmtKind::Break | StmtKind::Continue if self.loops == 0 => {
                let word = if s.kind == StmtKind::Break {
                    "break"
                } else {
                    "continue"
                };
                self.diags.push(
                    Diagnostic::error(codes::E0115, format!("`{word}` fora de um laço"))
                        .with_label(s.span, "não está dentro de for/while")
                        .with_help("para encerrar o comando, use `stop`"),
                );
            }
            StmtKind::Stop | StmtKind::Break | StmtKind::Continue => {}
        }
    }

    fn expr(&mut self, e: &Expr) {
        match &e.kind {
            ExprKind::Literal(_) => {}
            ExprKind::Name(id)
                if !self.declared(&id.name)
                    && (self.registry.get(&id.name).is_some()
                        || self.fns.contains_key(id.name.as_str())) =>
            {
                // A function name alone is a call with no arguments.
                self.call(e, &[], e.span);
            }
            ExprKind::Name(id) => {
                if !self.declared(&id.name) {
                    let names: Vec<String> = self.scopes.iter().flatten().cloned().collect();
                    let mut d = Diagnostic::error(
                        codes::E0200,
                        format!("A variável `{}` não existe", id.name),
                    )
                    .with_label(id.span, "variável não criada");
                    if let Some(to) = known::renamed(&id.name) {
                        self.diags.push(
                            d.with_help(format!("no Diracord, o servidor se chama `{to}`"))
                                .with_fix(Fix::replace(
                                    format!("trocar `{}` por `{to}`", id.name),
                                    id.span,
                                    to,
                                    true,
                                )),
                        );
                        return;
                    }
                    let mut candidates: Vec<&str> = names.iter().map(String::as_str).collect();
                    for n in self.registry.names() {
                        candidates.push(n);
                    }
                    match did_you_mean(&id.name, candidates.iter().copied()) {
                        Some(s) => {
                            d = d.with_help(format!("você quis dizer `{s}`?")).with_fix(
                                Fix::replace(format!("trocar por `{s}`"), id.span, s, false),
                            )
                        }
                        None => {
                            d = d.with_help(format!("crie antes com `local {} = ...`", id.name))
                        }
                    }
                    self.diags.push(d);
                }
            }
            ExprKind::Template(parts) => {
                for p in parts {
                    if let TemplatePart::Expr(x) = p {
                        self.expr(x);
                    }
                }
            }
            ExprKind::List(items) => items.iter().for_each(|i| self.expr(i)),
            ExprKind::Object(fields) => fields.iter().for_each(|f| self.expr(&f.value)),
            ExprKind::Unary { expr, .. } => self.expr(expr),
            ExprKind::Binary { lhs, rhs, .. } => {
                self.expr(lhs);
                self.expr(rhs);
            }
            ExprKind::Member { object, .. } => self.expr(object),
            ExprKind::Index { object, index } => {
                self.expr(object);
                self.expr(index);
            }
            ExprKind::Call { callee, args, .. } => self.call(callee, args, e.span),
        }
    }

    /// Argument checks for a call to a user function.
    fn user_call(&mut self, def: &FunctionDef, args: &[Arg], span: Span) {
        let usage = format!("uso: {}", def.signature());
        let mut given = vec![false; def.params.len()];
        let mut next = 0;
        for a in args {
            self.expr(&a.value);
            match &a.name {
                None if next >= def.params.len() => {
                    self.diags.push(
                        Diagnostic::error(
                            codes::E0301,
                            format!(
                                "`{}` aceita no máximo {} argumento(s)",
                                def.name.name,
                                def.params.len()
                            ),
                        )
                        .with_label(a.span, "argumento a mais")
                        .with_help(usage.clone()),
                    );
                    return;
                }
                None => {
                    given[next] = true;
                    next += 1;
                }
                Some(n) => match def.params.iter().position(|p| p.name.name == n.name) {
                    Some(i) => given[i] = true,
                    None => {
                        let mut d = Diagnostic::error(
                            codes::E0303,
                            format!("`{}` não tem o argumento `{}`", def.name.name, n.name),
                        )
                        .with_label(n.span, "argumento desconhecido");
                        d = match did_you_mean(
                            &n.name,
                            def.params.iter().map(|p| p.name.name.as_str()),
                        ) {
                            Some(s) => d.with_help(format!("você quis dizer `{s}:`?")),
                            None => d.with_help(usage.clone()),
                        };
                        self.diags.push(d);
                    }
                },
            }
        }
        let missing: Vec<&str> = def
            .params
            .iter()
            .zip(&given)
            .filter(|(p, g)| p.default.is_none() && !**g)
            .map(|(p, _)| p.name.name.as_str())
            .collect();
        if !missing.is_empty() {
            self.diags.push(
                Diagnostic::error(
                    codes::E0300,
                    format!(
                        "Faltou o argumento `{}` em `{}`",
                        missing.join("`, `"),
                        def.name.name
                    ),
                )
                .with_label(span, "faltam argumentos")
                .with_help(usage),
            );
        }
    }

    fn call(&mut self, callee: &Expr, args: &[Arg], span: Span) {
        let Some(id) = callee.as_ident() else {
            self.expr(callee);
            args.iter().for_each(|a| self.expr(&a.value));
            return;
        };
        if self.registry.get(&id.name).is_none() {
            if let Some(def) = self.fns.get(id.name.as_str()).copied() {
                self.user_call(def, args, span);
                return;
            }
        }
        let Some(spec) = self.registry.get(&id.name) else {
            let mut d =
                Diagnostic::error(codes::E0201, format!("A função `{}` não existe", id.name))
                    .with_label(id.span, "função desconhecida");
            let mut names: Vec<&str> = self.fns.keys().copied().collect();
            for n in self.registry.names() {
                names.push(n);
            }
            if let Some(s) = did_you_mean(&id.name, names.iter().copied()) {
                d = d
                    .with_help(format!("você quis dizer `{s}`?"))
                    .with_fix(Fix::replace(
                        format!("trocar `{}` por `{s}`", id.name),
                        id.span,
                        s,
                        false,
                    ));
            } else {
                d = d.with_help("veja a lista de funções em docs/functions.md");
            }
            self.diags.push(d);
            args.iter().for_each(|a| self.expr(&a.value));
            return;
        };
        let positional = args.iter().filter(|a| a.name.is_none()).count();
        let rest_at = spec.params.iter().position(|p| p.variadic);
        let fixed = rest_at.unwrap_or(spec.params.len());
        let mut given = vec![false; spec.params.len()];
        for slot in given.iter_mut().take(positional.min(fixed)) {
            *slot = true;
        }
        if let Some(r) = rest_at.filter(|_| positional > fixed) {
            given[r] = true;
        }
        for (i, a) in args.iter().enumerate() {
            let param = match &a.name {
                Some(n) => spec.params.iter().position(|p| p.name == n.name),
                None => {
                    let k = args[..i].iter().filter(|x| x.name.is_none()).count();
                    match rest_at {
                        Some(r) if k >= r => Some(r),
                        _ => Some(k).filter(|&k| k < spec.params.len()),
                    }
                }
            };
            if let Some(n) = &a.name {
                match param {
                    Some(k) => given[k] = true,
                    None => {
                        let mut d = Diagnostic::error(
                            codes::E0303,
                            format!("`{}` não tem o argumento `{}`", spec.name, n.name),
                        )
                        .with_label(n.span, "argumento desconhecido");
                        let names = spec.params.iter().map(|p| p.name);
                        d = match did_you_mean(&n.name, names) {
                            Some(s) => d.with_help(format!("você quis dizer `{s}:`?")).with_fix(
                                Fix::replace(format!("trocar por `{s}`"), n.span, s, false),
                            ),
                            None => d.with_help(format!("uso: {}", spec.signature())),
                        };
                        self.diags.push(d);
                    }
                }
            }
            let is_scope_word = param.is_some_and(|k| spec.params[k].kind == ParamKind::Scope)
                && a.value
                    .as_ident()
                    .is_some_and(|v| Scope::from_name(&v.name).is_some());
            if !is_scope_word {
                self.expr(&a.value);
            }
        }
        if rest_at.is_none() && positional > spec.params.len() {
            let extra = args
                .iter()
                .filter(|a| a.name.is_none())
                .nth(spec.params.len())
                .map(|a| a.span)
                .unwrap_or(span);
            self.diags.push(
                Diagnostic::error(
                    codes::E0301,
                    format!(
                        "`{}` aceita no máximo {} argumento(s), mas recebeu {}",
                        spec.name,
                        spec.params.len(),
                        positional
                    ),
                )
                .with_label(extra, "argumento a mais")
                .with_help(format!("uso: {}", spec.signature())),
            );
        }
        let missing: Vec<&str> = spec
            .params
            .iter()
            .zip(&given)
            .filter(|(p, g)| !p.optional && !**g)
            .map(|(p, _)| p.name)
            .collect();
        if !missing.is_empty() {
            self.diags.push(
                Diagnostic::error(
                    codes::E0300,
                    format!(
                        "Faltou o argumento `{}` em `{}`",
                        missing.join("`, `"),
                        spec.name
                    ),
                )
                .with_label(span, "faltam argumentos")
                .with_help(format!("uso: {}", spec.signature())),
            );
        }
    }
}
