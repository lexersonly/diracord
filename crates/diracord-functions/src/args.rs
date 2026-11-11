//! Argument binding shared by every built-in.

use crate::{FnError, FunctionSpec, ParamKind};
use diracord_diagnostics::codes;
use diracord_value::{Scope, Value};

/// Raw arguments, as evaluated by the interpreter.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Args {
    /// Positional values in order.
    pub positional: Vec<Value>,
    /// `name: value` pairs in order.
    pub named: Vec<(String, Value)>,
}

impl Args {
    /// Positional-only arguments.
    pub fn positional(values: Vec<Value>) -> Self {
        Args {
            positional: values,
            named: Vec::new(),
        }
    }

    /// Adds a named argument.
    pub fn with_named(mut self, name: impl Into<String>, value: Value) -> Self {
        self.named.push((name.into(), value));
        self
    }

    /// Matches arguments to `spec.params`: positional first, then named.
    ///
    /// Errors: too many (`E0301`), unknown name (`E0303`), a parameter given
    /// twice (`E0302`), a required one missing (`E0300`).
    pub fn bind(self, spec: &FunctionSpec) -> Result<Bound, FnError> {
        let params = spec.params;
        if self.positional.len() > params.len() {
            let i = params.len();
            return Err(FnError::new(
                codes::E0301,
                format!(
                    "`{}` aceita no máximo {} argumento(s), mas recebeu {}",
                    spec.name,
                    params.len(),
                    self.positional.len()
                ),
            )
            .at(ArgRef::Positional(i))
            .with_help(format!("uso: {}", spec.signature())));
        }
        let mut slots: Vec<Option<Value>> = vec![None; params.len()];
        for (i, v) in self.positional.into_iter().enumerate() {
            slots[i] = Some(v);
        }
        for (name, v) in self.named {
            let Some(i) = params.iter().position(|p| p.name == name) else {
                let names: Vec<&str> = params.iter().map(|p| p.name).collect();
                let mut err = FnError::new(
                    codes::E0303,
                    format!("`{}` não tem o argumento `{name}`", spec.name),
                )
                .at(ArgRef::Named(name.clone()));
                if let Some(s) = diracord_diagnostics::did_you_mean(&name, names.iter().copied()) {
                    err = err.with_help(format!("você quis dizer `{s}:`?"));
                } else {
                    err = err.with_help(format!("uso: {}", spec.signature()));
                }
                return Err(err);
            };
            if slots[i].is_some() {
                return Err(FnError::new(
                    codes::E0302,
                    format!("o argumento `{name}` foi passado duas vezes"),
                )
                .at(ArgRef::Named(name)));
            }
            slots[i] = Some(v);
        }
        for (i, p) in params.iter().enumerate() {
            if slots[i].is_none() && !p.optional {
                return Err(FnError::new(
                    codes::E0300,
                    format!("faltou o argumento `{}` em `{}`", p.name, spec.name),
                )
                .with_help(format!("uso: {}", spec.signature())));
            }
        }
        Ok(Bound {
            function: spec.name,
            names: params.iter().map(|p| (p.name, p.kind)).collect(),
            values: slots,
        })
    }
}

/// Points at the argument that caused an error, so the interpreter can draw
/// the arrow under it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgRef {
    /// The i-th positional argument (0-based); may equal the count for "one too many".
    Positional(usize),
    /// A named argument.
    Named(String),
    /// The parameter with this index in the spec, however it was passed.
    Param(usize),
}

/// Arguments matched to parameters. Index `i` is `spec.params[i]`.
#[derive(Clone, Debug, PartialEq)]
pub struct Bound {
    function: &'static str,
    names: Vec<(&'static str, ParamKind)>,
    values: Vec<Option<Value>>,
}

impl Bound {
    /// Name of the function being called (for error messages).
    pub fn function(&self) -> &'static str {
        self.function
    }

    /// The value of parameter `i`, or `None` if an optional one was left out.
    pub fn opt(&self, i: usize) -> Option<&Value> {
        self.values.get(i).and_then(Option::as_ref)
    }

    /// The value of parameter `i`, or `null` if left out.
    pub fn get(&self, i: usize) -> &Value {
        const NULL: Value = Value::Null;
        self.opt(i).unwrap_or(&NULL)
    }

    /// Parameter `i` as text. Numbers and bools are converted; other types fail with `E0302`.
    pub fn text(&self, i: usize) -> Result<String, FnError> {
        match self.get(i) {
            Value::Text(s) => Ok(s.clone()),
            v @ (Value::Int(_) | Value::Float(_) | Value::Bool(_)) => Ok(v.to_string()),
            v => Err(self.wrong(i, "um texto", v)),
        }
    }

    /// Parameter `i` as a number.
    pub fn number(&self, i: usize) -> Result<f64, FnError> {
        let v = self.get(i);
        match v {
            Value::Text(s) => s
                .trim()
                .parse::<f64>()
                .map_err(|_| self.wrong(i, "um número", v)),
            _ => v.as_f64().ok_or_else(|| self.wrong(i, "um número", v)),
        }
    }

    /// Parameter `i` as a whole number.
    pub fn int(&self, i: usize) -> Result<i64, FnError> {
        let n = self.number(i)?;
        if n.fract() == 0.0 && n.is_finite() {
            Ok(n as i64)
        } else {
            Err(self.wrong(i, "um número inteiro", self.get(i)))
        }
    }

    /// Parameter `i` as a list.
    pub fn list(&self, i: usize) -> Result<&[Value], FnError> {
        match self.get(i) {
            Value::List(l) => Ok(l),
            v => Err(self.wrong(i, "uma lista", v)),
        }
    }

    /// Parameter `i` as a Discord id (object with `id`, id text or mention).
    pub fn id(&self, i: usize) -> Result<String, FnError> {
        let v = self.get(i);
        v.entity_id()
            .ok_or_else(|| self.wrong(i, "um usuário, canal ou cargo", v))
    }

    /// Parameter `i` as a data scope (`user`, `server`, `member`, `global`).
    /// Returns `None` when the parameter was left out.
    pub fn scope(&self, i: usize) -> Result<Option<Scope>, FnError> {
        match self.opt(i) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::Text(s)) => Scope::from_name(s).map(Some).ok_or_else(|| {
                let help = if s == "guild" {
                    "no Diracord, o escopo do servidor é `server`"
                } else {
                    "use user, server, member ou global"
                };
                FnError::new(codes::E0401, format!("`{s}` não é um escopo"))
                    .at(ArgRef::Param(i))
                    .with_help(help)
            }),
            Some(v) => Err(self.wrong(i, "um escopo (user, server, member ou global)", v)),
        }
    }

    /// Builds an `E0302` error for parameter `i`.
    pub fn wrong(&self, i: usize, expected: &str, got: &Value) -> FnError {
        let name = self.names.get(i).map(|n| n.0).unwrap_or("?");
        FnError::new(
            codes::E0302,
            format!(
                "o argumento `{name}` de `{}` precisa ser {expected}, mas é {}",
                self.function,
                got.type_name()
            ),
        )
        .at(ArgRef::Param(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Category, Effect, ParamSpec};

    const PARAMS: &[ParamSpec] = &[
        ParamSpec::required("text", ParamKind::Text, "o texto"),
        ParamSpec::optional("times", ParamKind::Number, "quantas vezes"),
    ];

    fn echo(b: &Bound) -> Result<Effect, FnError> {
        Ok(Effect::Value(b.text(0)?.into()))
    }

    const SPEC: FunctionSpec = FunctionSpec {
        name: "echo",
        category: Category::Utility,
        summary: "devolve o texto",
        params: PARAMS,
        returns: "text",
        example: "echo \"oi\"",
        call: echo,
    };

    #[test]
    fn binds_positional_and_named() {
        let b = Args::positional(vec!["a".into()])
            .with_named("times", Value::Int(2))
            .bind(&SPEC)
            .unwrap();
        assert_eq!(b.text(0).unwrap(), "a");
        assert_eq!(b.int(1).unwrap(), 2);
        assert_eq!(
            SPEC.invoke(Args::positional(vec![Value::Int(3)])).unwrap(),
            Effect::Value("3".into())
        );
    }

    #[test]
    fn errors_have_codes() {
        let e = Args::default().bind(&SPEC).unwrap_err();
        assert_eq!(e.code, codes::E0300);
        let e = Args::positional(vec![1i64.into(), 2i64.into(), 3i64.into()])
            .bind(&SPEC)
            .unwrap_err();
        assert_eq!(e.code, codes::E0301);
        assert_eq!(e.arg, Some(ArgRef::Positional(2)));
        let e = Args::positional(vec!["a".into()])
            .with_named("tims", 1i64.into())
            .bind(&SPEC)
            .unwrap_err();
        assert_eq!(e.code, codes::E0303);
        assert_eq!(e.help.as_deref(), Some("você quis dizer `times:`?"));
        let e = Args::positional(vec!["a".into()])
            .with_named("text", 1i64.into())
            .bind(&SPEC)
            .unwrap_err();
        assert_eq!(e.code, codes::E0302);
    }

    #[test]
    fn signature() {
        assert_eq!(SPEC.signature(), "echo(text: text, times?: number)");
    }
}
