//! `layout { ... }`: the Components v2 block syntax.

use crate::parser::{Fail, PResult, Parser};
use diracord_ast::{Arg, ElementKind, Layout, LayoutBranch, LayoutItem, LayoutItemKind};
use diracord_diagnostics::{codes, did_you_mean, Diagnostic, Span};
use diracord_lexer::TokenKind;

impl Parser<'_> {
    /// `layout` followed by `{` (the `layout` word is current).
    pub(crate) fn layout_expr(&mut self) -> PResult<Layout> {
        self.bump(); // layout
        self.layout_block(None)
    }

    /// `{ item* }`, checking that each element fits inside `parent`.
    fn layout_block(&mut self, parent: Option<ElementKind>) -> PResult<Layout> {
        if !self.at(&TokenKind::LBrace) {
            return self
                .block()
                .map(|_| unreachable!("block() fails without `{{`"));
        }
        self.enter()?;
        let open = self.bump().span;
        let mut items = Vec::new();
        loop {
            while self.eat(&TokenKind::Semi) || self.eat(&TokenKind::Comma) {}
            if self.eat(&TokenKind::RBrace) {
                break;
            }
            if self.at_eof() {
                let end = Span::point(self.src.len() as u32);
                self.report(
                    Diagnostic::error(codes::E0101, "Faltou fechar a chave do layout")
                        .with_label(open, "esta chave nunca foi fechada")
                        .with_secondary(end, "o arquivo acabou aqui"),
                );
                break;
            }
            let before = self.peek().span.start;
            match self.layout_item(parent) {
                Ok(item) => items.push(item),
                Err(Fail) => self.sync_stmt(),
            }
            if self.peek().span.start == before && !self.at(&TokenKind::RBrace) && !self.at_eof() {
                self.bump();
            }
        }
        self.leave();
        Ok(Layout {
            items,
            span: open.to(Span::point(self.prev_end())),
        })
    }

    fn layout_item(&mut self, parent: Option<ElementKind>) -> PResult<LayoutItem> {
        let start = self.span();
        let word = match self.kind() {
            TokenKind::Ident(s) => s.clone(),
            _ => return self.unexpected("um componente do layout, como `text` ou `row`"),
        };
        let kind = match word.as_str() {
            "local" => {
                self.bump();
                let name = self.ident("o nome da variável")?;
                if !self.eat(&TokenKind::Assign) {
                    return self.unexpected("`=`");
                }
                let value = self.expr_or_bare()?;
                LayoutItemKind::Local { name, value }
            }
            "if" => {
                let mut branches = Vec::new();
                let mut otherwise = None;
                let s = self.bump().span;
                let cond = self.expr(false)?;
                let body = self.layout_block(parent)?;
                branches.push(LayoutBranch {
                    cond,
                    body,
                    span: s.to(Span::point(self.prev_end())),
                });
                while self.at_word("else") {
                    self.bump();
                    if self.at_word("if") {
                        let s = self.bump().span;
                        let cond = self.expr(false)?;
                        let body = self.layout_block(parent)?;
                        branches.push(LayoutBranch {
                            cond,
                            body,
                            span: s.to(Span::point(self.prev_end())),
                        });
                    } else {
                        otherwise = Some(self.layout_block(parent)?);
                        break;
                    }
                }
                LayoutItemKind::If {
                    branches,
                    otherwise,
                }
            }
            "for" => {
                self.bump();
                let var = self.ident("o nome da variável do for")?;
                if !self.at_word("in") {
                    return self.unexpected("`in`");
                }
                self.bump();
                let iter = self.expr(false)?;
                let body = self.layout_block(parent)?;
                LayoutItemKind::For { var, iter, body }
            }
            name => match ElementKind::from_name(name) {
                Some(k) => self.element(k, parent)?,
                None => {
                    let mut d = Diagnostic::error(
                        codes::E0118,
                        format!("`{name}` não é um componente do layout"),
                    )
                    .with_label(start, "componente desconhecido");
                    let names = ElementKind::ALL.map(ElementKind::as_str);
                    d = match did_you_mean(name, names) {
                        Some(s) => d.with_help(format!("você quis dizer `{s}`?")),
                        None => d.with_help("componentes: container, section, row, text, thumbnail, separator, gallery, file, button, menu"),
                    };
                    if matches!(name, "reply" | "send") {
                        d = d.with_note("o layout só descreve a mensagem; chame `reply layout { ... }` fora dele");
                    }
                    return self.fail(d);
                }
            },
        };
        let span = start.to(Span::point(self.prev_end()));
        let t = self.peek();
        if !(t.newline_before
            || matches!(
                t.kind,
                TokenKind::RBrace | TokenKind::Semi | TokenKind::Comma | TokenKind::Eof
            ))
        {
            return self.unexpected("o fim da linha (um componente por linha, ou separe com `;`)");
        }
        Ok(LayoutItem { kind, span })
    }

    fn element(
        &mut self,
        kind: ElementKind,
        parent: Option<ElementKind>,
    ) -> PResult<LayoutItemKind> {
        let name = self.bump();
        if !kind.allowed_in(parent) {
            let place = match parent {
                None => "no topo do layout".to_string(),
                Some(p) => format!("dentro de `{}`", p.as_str()),
            };
            self.report(
                Diagnostic::error(
                    codes::E0117,
                    format!("`{}` não pode ficar {place}", kind.as_str()),
                )
                .with_label(name.span, "fora do lugar")
                .with_help(format!(
                    "`{}` vai {}",
                    kind.as_str(),
                    kind.placement_hint()
                )),
            );
        }
        let args: Vec<Arg> = if self.at(&TokenKind::LParen) && !self.peek().newline_before {
            let open = self.bump().span;
            self.paren_args(open)?
        } else {
            let mut args = Vec::new();
            loop {
                let t = self.peek();
                let starts = !t.newline_before
                    && !matches!(
                        t.kind,
                        TokenKind::LBrace
                            | TokenKind::RBrace
                            | TokenKind::Semi
                            | TokenKind::Comma
                            | TokenKind::Eof
                    );
                if !starts {
                    break;
                }
                args.push(self.arg()?);
                if self.at(&TokenKind::Comma)
                    && !self.nth(1).newline_before
                    && !self.nth(1).kind.eq(&TokenKind::RBrace)
                {
                    self.bump();
                }
            }
            args
        };
        let children = if kind.has_children() {
            if !self.at(&TokenKind::LBrace) {
                let span = self.span();
                return self.fail(
                    Diagnostic::error(
                        codes::E0105,
                        format!("`{}` precisa de um bloco `{{ ... }}`", kind.as_str()),
                    )
                    .with_label(span, "esperava `{`")
                    .with_help(format!("exemplo: {} {{ text \"oi\" }}", kind.as_str())),
                );
            }
            Some(self.layout_block(Some(kind))?)
        } else {
            if self.at(&TokenKind::LBrace) && !self.peek().newline_before {
                let span = self.span();
                let mut d = Diagnostic::error(
                    codes::E0100,
                    format!("`{}` não tem bloco `{{ }}`", kind.as_str()),
                )
                .with_label(span, "bloco inesperado");
                if kind == ElementKind::Button {
                    d = d.with_help("o código do clique fica fora do layout: button(\"id\") { reply \"...\" } no topo do arquivo");
                }
                return self.fail(d);
            }
            None
        };
        Ok(LayoutItemKind::Element {
            kind,
            name_span: name.span,
            args,
            children,
        })
    }
}
