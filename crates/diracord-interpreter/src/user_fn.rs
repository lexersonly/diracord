//! Calls to user-defined functions (`fn name(...) { }`).
//!
//! Rules (SPEC §4.7, `docs/rfcs/functions.md`):
//! - arguments by position or by name, with defaults; typed parameters are
//!   converted like command arguments;
//! - the function only sees its parameters, its own `local`s and the automatic
//!   variables (`user`, `server`...), never the caller's `local`s;
//! - `return value` gives the result (`null` without it); `stop` ends the whole run;
//! - nesting is limited (recursion that never ends is stopped with E0211).

use crate::engine::{coerce, Flow, Machine, R};
use crate::env::Env;
use crate::error::RuntimeError;
use diracord_ast::{Arg, FunctionDef, ItemKind};
use diracord_diagnostics::{codes, did_you_mean, Span};
use diracord_value::Value;

/// Functions running inside each other, at most.
pub(crate) const MAX_CALL_DEPTH: u32 = 64;

impl<'r> Machine<'r> {
    /// The definition of user function `name`, if there is one.
    pub(crate) fn user_fn(&self, name: &str) -> Option<&'r FunctionDef> {
        let program = self.program;
        let &(_, idx) = self.functions.get(name)?;
        match &program.items[idx].kind {
            ItemKind::Function(f) => Some(f),
            _ => None,
        }
    }

    /// Runs user function `name` with `args` written at `span`.
    pub(crate) async fn call_user(&mut self, name: &str, args: &[Arg], span: Span) -> R<Value> {
        let Some(def) = self.user_fn(name) else {
            return Err(RuntimeError::new(
                codes::E0201,
                format!("A função `{name}` não existe"),
                span,
                "aqui",
            ));
        };
        let file = self.functions[name].0;
        let usage = || format!("uso: {}", def.signature());

        // 1. Evaluate and bind arguments in the caller's variables.
        let mut slots: Vec<Option<(Value, Span)>> = vec![None; def.params.len()];
        let mut next = 0;
        for a in args {
            let v = self.eval(&a.value).await?;
            let i = match &a.name {
                None => {
                    if next >= def.params.len() {
                        return Err(RuntimeError::new(
                            codes::E0301,
                            format!(
                                "`{name}` aceita no máximo {} argumento(s)",
                                def.params.len()
                            ),
                            a.span,
                            "argumento a mais",
                        )
                        .with_help(usage()));
                    }
                    next += 1;
                    next - 1
                }
                Some(n) => match def.params.iter().position(|p| p.name.name == n.name) {
                    Some(i) => i,
                    None => {
                        let mut e = RuntimeError::new(
                            codes::E0303,
                            format!("`{name}` não tem o argumento `{}`", n.name),
                            n.span,
                            "argumento desconhecido",
                        );
                        e = match did_you_mean(
                            &n.name,
                            def.params.iter().map(|p| p.name.name.as_str()),
                        ) {
                            Some(s) => e.with_help(format!("você quis dizer `{s}:`?")),
                            None => e.with_help(usage()),
                        };
                        return Err(e);
                    }
                },
            };
            if slots[i].is_some() {
                return Err(RuntimeError::new(
                    codes::E0302,
                    format!(
                        "O argumento `{}` foi passado duas vezes",
                        def.params[i].name.name
                    ),
                    a.span,
                    "repetido",
                ));
            }
            let v = match &def.params[i].ty {
                Some(t) if v != Value::Null => coerce(
                    &def.params[i].name.name,
                    t.node,
                    a.span,
                    v,
                    &format!("da função `{name}`"),
                )?,
                _ => v,
            };
            slots[i] = Some((v, a.span));
        }
        for (p, slot) in def.params.iter().zip(&slots) {
            if slot.is_none() && p.default.is_none() {
                return Err(RuntimeError::new(
                    codes::E0300,
                    format!("Faltou o argumento `{}` da função `{name}`", p.name.name),
                    span,
                    "faltam argumentos",
                )
                .with_help(usage()));
            }
        }
        if self.depth >= MAX_CALL_DEPTH {
            return Err(RuntimeError::limit(
                codes::E0211,
                format!(
                    "Funções chamando funções demais (mais de {MAX_CALL_DEPTH} ao mesmo tempo)"
                ),
                span,
                "uma função que chama ela mesma precisa de um `if` que pare em algum momento",
            ));
        }

        // 2. Run the body with its own variables.
        self.depth += 1;
        let saved = std::mem::replace(&mut self.env, Env::new());
        self.outer_bytes += saved.bytes();
        let result = async {
            for (k, v) in self.ctx.variables() {
                self.env.declare(&k, v);
            }
            for (p, slot) in def.params.iter().zip(slots) {
                let v = match (slot, &p.default) {
                    (Some((v, _)), _) => v,
                    (None, Some(d)) => {
                        let v = self.eval(d).await?;
                        match &p.ty {
                            Some(t) if v != Value::Null => coerce(
                                &p.name.name,
                                t.node,
                                d.span,
                                v,
                                &format!("da função `{name}`"),
                            )?,
                            _ => v,
                        }
                    }
                    (None, None) => Value::Null,
                };
                self.env.declare(&p.name.name, v);
            }
            self.check_memory(span)?;
            self.exec_block(&def.body).await
        }
        .await;
        self.outer_bytes -= saved.bytes();
        self.env = saved;
        self.depth -= 1;

        match result {
            Ok(Flow::Return(v)) => Ok(v),
            Ok(Flow::Stop) => Err(RuntimeError::stop_signal()),
            Ok(_) => Ok(Value::Null),
            Err(mut e) => {
                if !e.located && !e.stop {
                    e.file = file;
                    e.located = true;
                }
                Err(e)
            }
        }
    }
}
