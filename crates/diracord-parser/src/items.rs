//! Top-level items: `command`, events and `button`/`menu`/`modal`.

use crate::parser::{PResult, Parser};
use diracord_ast::{
    known, ArgType, CommandDef, EventDef, FnParam, FunctionDef, InteractionDef, InteractionKind,
    Item, ItemKind, Param, Program,
};
use diracord_diagnostics::{codes, did_you_mean, Diagnostic, FileId, Fix, Span, Spanned};
use diracord_lexer::{StrPart, TokenKind};

impl Parser<'_> {
    pub(crate) fn program(&mut self) -> Program {
        let mut items = Vec::new();
        while !self.at_eof() {
            if self.at(&TokenKind::RBrace) {
                let span = self.span();
                self.report(
                    Diagnostic::error(
                        codes::E0100,
                        "Essa chave `}` fecha um bloco que não foi aberto",
                    )
                    .with_label(span, "chave a mais")
                    .with_help("apague essa chave ou confira se falta um `{` antes")
                    .with_fix(Fix::replace("apagar a chave", span, "", false)),
                );
                self.bump();
                continue;
            }
            let before = self.peek().span.start;
            match self.item() {
                Ok(item) => items.push(item),
                Err(_) => {
                    self.sync_item();
                    if self.peek().span.start == before {
                        self.bump();
                    }
                }
            }
        }
        Program {
            items,
            span: Span::new(0, self.src.len() as u32),
        }
    }

    fn item(&mut self) -> PResult<Item> {
        let start = self.span();
        let name = match self.kind() {
            TokenKind::Ident(s) => s.clone(),
            _ => return self.loose_code(),
        };
        let next = self.nth(1).kind.clone();
        if name == "function" && matches!(next, TokenKind::Ident(_)) {
            let span = self.span();
            self.report(
                Diagnostic::error(
                    codes::E0100,
                    "No Diracord, funções são criadas com `fn`, não `function`",
                )
                .with_label(span, "use `fn`")
                .with_help("exemplo: fn greet(name) { return \"Hi $name\" }")
                .with_fix(Fix::replace(
                    "trocar `function` por `fn`",
                    span,
                    "fn",
                    true,
                )),
            );
        }
        let is_fn = (name == "fn" || name == "function") && matches!(next, TokenKind::Ident(_));
        let kind = if is_fn {
            ItemKind::Function(self.function()?)
        } else if name == "command" && next == TokenKind::LParen {
            ItemKind::Command(self.command()?)
        } else if let (Some(k), TokenKind::LParen) = (InteractionKind::from_name(&name), &next) {
            ItemKind::Interaction(self.interaction(k)?)
        } else if next == TokenKind::LBrace && !known::is_keyword(&name) {
            let id = self.ident("o nome do evento")?;
            let body = self.block()?;
            ItemKind::Event(EventDef { name: id, body })
        } else {
            return self.loose_code();
        };
        Ok(Item {
            kind,
            span: start.to(Span::point(self.prev_end())),
            file: FileId::default(),
        })
    }

    fn loose_code<T>(&mut self) -> PResult<T> {
        let span = self.span();
        let mut d = Diagnostic::error(codes::E0114, "Esse código está solto, fora de um comando ou evento")
            .with_label(span, "código fora de command / evento")
            .with_help("coloque dentro de um comando: command(\"nome\") { ... }, ou de um evento: memberJoin { ... }");
        if let TokenKind::Ident(s) = self.kind() {
            if let Some(sugg) = did_you_mean(s, ["command", "button", "menu", "modal"]) {
                d = d.with_help(format!("você quis dizer `{sugg}`?"));
            }
        }
        self.fail(d)
    }

    /// A string literal without `$var`s (names and ids).
    fn plain_string(&mut self) -> Option<Spanned<String>> {
        if let TokenKind::Str(parts) = self.kind() {
            if let [StrPart::Text(s)] = parts.as_slice() {
                let s = s.clone();
                let t = self.bump();
                return Some(Spanned::new(s, t.span));
            }
        }
        None
    }

    fn command(&mut self) -> PResult<CommandDef> {
        self.bump(); // command
        let open = self.bump().span; // (
        let Some(name) = self.plain_string() else {
            let span = self.span();
            return self.fail(
                Diagnostic::error(codes::E0109, "O comando precisa de um nome em texto")
                    .with_label(span, "esperava o nome entre aspas")
                    .with_help("exemplo: command(\"ping\") { reply \"Pong!\" }"),
            );
        };
        if name.node.trim().is_empty() || name.node.contains(char::is_whitespace) {
            self.report(
                Diagnostic::error(
                    codes::E0109,
                    "O nome do comando não pode ter espaços nem ser vazio",
                )
                .with_label(name.span, "nome inválido")
                .with_help("use uma palavra só, como \"ping\" ou \"user-info\""),
            );
        }
        let mut params: Vec<Param> = Vec::new();
        while self.eat(&TokenKind::Comma) {
            if self.at(&TokenKind::RParen) {
                break;
            }
            let p = self.param()?;
            if let Some(prev) = params.iter().find(|q| q.name.name == p.name.name) {
                self.report(
                    Diagnostic::error(
                        codes::E0116,
                        format!("O argumento `{}` aparece duas vezes", p.name.name),
                    )
                    .with_label(p.name.span, "nome repetido")
                    .with_secondary(prev.name.span, "primeira vez aqui"),
                );
            }
            if p.default.is_none() {
                if let Some(opt) = params.iter().find(|q| q.default.is_some()) {
                    self.report(
                        Diagnostic::error(
                            codes::E0112,
                            format!(
                                "O argumento obrigatório `{}` está depois de um opcional",
                                p.name.name
                            ),
                        )
                        .with_label(p.span, "obrigatório")
                        .with_secondary(opt.span, "opcional antes dele")
                        .with_help("coloque os argumentos com `= valor` no fim"),
                    );
                }
            }
            params.push(p);
        }
        self.close_paren(open)?;
        let body = self.block()?;
        Ok(CommandDef { name, params, body })
    }

    fn param(&mut self) -> PResult<Param> {
        let name = self.ident("o nome do argumento")?;
        if !self.eat(&TokenKind::Colon) {
            let span = self.span();
            return self.fail(
                Diagnostic::error(
                    codes::E0100,
                    format!("Faltou o tipo do argumento `{}`", name.name),
                )
                .with_label(span, "esperava `:` e o tipo")
                .with_help(format!("exemplo: {}: text", name.name))
                .with_fix(Fix::replace(
                    "adicionar `: text`",
                    Span::point(name.span.end),
                    ": text",
                    false,
                )),
            );
        }
        let ty = self.arg_type()?;
        let default = if self.eat(&TokenKind::Assign) {
            Some(self.expr(true)?)
        } else {
            None
        };
        let span = name.span.to(Span::point(self.prev_end()));
        Ok(Param {
            name,
            ty,
            default,
            span,
        })
    }

    /// A type name after `:` (`text`, `number`...), with suggestions for unknown ones.
    fn arg_type(&mut self) -> PResult<Spanned<ArgType>> {
        let ty_tok = self.peek().clone();
        let TokenKind::Ident(ty_name) = &ty_tok.kind else {
            return self.unexpected(
                "o tipo do argumento (text, number, bool, member, user, channel, role)",
            );
        };
        self.bump();
        let ty = match ArgType::from_name(ty_name) {
            Some(t) => t,
            None => {
                let alias = match ty_name.as_str() {
                    "string" | "str" => Some(ArgType::Text),
                    "int" | "integer" | "float" | "num" => Some(ArgType::Number),
                    "boolean" => Some(ArgType::Bool),
                    _ => None,
                };
                let names = ArgType::ALL.map(ArgType::as_str);
                let mut d = Diagnostic::error(
                    codes::E0107,
                    format!("`{ty_name}` não é um tipo de argumento"),
                )
                .with_label(ty_tok.span, "tipo desconhecido")
                .with_note("tipos: text, number, bool, member, user, channel, role");
                let guess =
                    alias.or_else(|| did_you_mean(ty_name, names).and_then(ArgType::from_name));
                if let Some(g) = guess {
                    d = d
                        .with_help(format!("você quis dizer `{g}`?"))
                        .with_fix(Fix::replace(
                            format!("trocar por `{g}`"),
                            ty_tok.span,
                            g.as_str(),
                            alias.is_some(),
                        ));
                }
                self.report(d);
                guess.unwrap_or(ArgType::Text)
            }
        };
        Ok(Spanned::new(ty, ty_tok.span))
    }

    /// `fn name(a, b: number = 1) { ... }` (the `fn` word is current).
    fn function(&mut self) -> PResult<FunctionDef> {
        self.bump(); // fn
        let name = self.ident("o nome da função")?;
        if !self.at(&TokenKind::LParen) {
            let span = self.span();
            return self.fail(
                Diagnostic::error(
                    codes::E0100,
                    format!("Faltou `(` depois de `fn {}`", name.name),
                )
                .with_label(span, "esperava `(`")
                .with_help(format!(
                    "exemplo: fn {}(a, b) {{ return a + b }}",
                    name.name
                )),
            );
        }
        let open = self.bump().span;
        let mut params: Vec<FnParam> = Vec::new();
        while !self.at(&TokenKind::RParen) {
            if matches!(
                self.kind(),
                TokenKind::Eof | TokenKind::LBrace | TokenKind::RBrace
            ) {
                break;
            }
            let pname = self.ident("o nome do argumento")?;
            let ty = if self.eat(&TokenKind::Colon) {
                Some(self.arg_type()?)
            } else {
                None
            };
            let default = if self.eat(&TokenKind::Assign) {
                Some(self.expr(true)?)
            } else {
                None
            };
            let span = pname.span.to(Span::point(self.prev_end()));
            if let Some(prev) = params.iter().find(|q| q.name.name == pname.name) {
                self.report(
                    Diagnostic::error(
                        codes::E0116,
                        format!("O argumento `{}` aparece duas vezes", pname.name),
                    )
                    .with_label(pname.span, "nome repetido")
                    .with_secondary(prev.name.span, "primeira vez aqui"),
                );
            }
            if default.is_none() {
                if let Some(opt) = params.iter().find(|q| q.default.is_some()) {
                    self.report(
                        Diagnostic::error(
                            codes::E0112,
                            format!(
                                "O argumento obrigatório `{}` está depois de um opcional",
                                pname.name
                            ),
                        )
                        .with_label(span, "obrigatório")
                        .with_secondary(opt.span, "opcional antes dele")
                        .with_help("coloque os argumentos com `= valor` no fim"),
                    );
                }
            }
            params.push(FnParam {
                name: pname,
                ty,
                default,
                span,
            });
            if !self.eat(&TokenKind::Comma) {
                break;
            }
        }
        self.close_paren(open)?;
        let body = self.block()?;
        Ok(FunctionDef { name, params, body })
    }

    fn interaction(&mut self, kind: InteractionKind) -> PResult<InteractionDef> {
        self.bump(); // button / menu / modal
        let open = self.bump().span; // (
        let Some(id) = self.plain_string() else {
            let span = self.span();
            return self.fail(
                Diagnostic::error(
                    codes::E0109,
                    format!("O {} precisa de um id em texto", kind.as_str()),
                )
                .with_label(span, "esperava o id entre aspas")
                .with_help(format!(
                    "exemplo: {}(\"confirmar\") {{ reply \"ok\" }}",
                    kind.as_str()
                )),
            );
        };
        self.close_paren(open)?;
        let body = self.block()?;
        Ok(InteractionDef { kind, id, body })
    }

    /// Expects `)` closing the `(` at `open`.
    pub(crate) fn close_paren(&mut self, open: Span) -> PResult<()> {
        if self.eat(&TokenKind::RParen) {
            return Ok(());
        }
        let t = self.peek().clone();
        let at_end = t.newline_before
            || matches!(
                t.kind,
                TokenKind::LBrace | TokenKind::RBrace | TokenKind::Eof
            );
        if at_end {
            let here = Span::point(self.prev_end());
            self.fail(
                Diagnostic::error(codes::E0102, "Faltou fechar o parêntese")
                    .with_label(open, "este parêntese nunca foi fechado")
                    .with_secondary(here, "coloque `)` aqui")
                    .with_fix(Fix::replace("fechar o parêntese", here, ")", false)),
            )
        } else {
            let d = Diagnostic::error(
                codes::E0100,
                format!("Esperava `,` ou `)`, mas encontrei {}", t.kind.describe()),
            )
            .with_label(t.span, "esperava `,` ou `)`")
            .with_secondary(open, "parêntese aberto aqui")
            .with_help("separe os argumentos com vírgula");
            self.fail(d)
        }
    }
}
