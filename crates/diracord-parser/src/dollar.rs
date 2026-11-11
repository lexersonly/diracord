//! The aoi.js-style shortcut `$name[a;b]` (= `name(a, b)`).

use crate::parser::{PResult, Parser};
use diracord_ast::{Arg, CallStyle, Expr, ExprKind, Ident};
use diracord_diagnostics::{codes, Diagnostic, Fix, Span};
use diracord_lexer::TokenKind;

impl Parser<'_> {
    /// `$name` or `$name[arg; arg]`. Reports `E0108` (with a safe fix) when
    /// the shortcut is turned off, but still builds the call.
    pub(crate) fn dollar_call(&mut self) -> PResult<Expr> {
        let dollar = self.bump().span;
        let t = self.peek().clone();
        let name = match &t.kind {
            TokenKind::Ident(n) if !t.space_before => n.clone(),
            _ => {
                return self.fail(
                    Diagnostic::error(
                        codes::E0104,
                        "Depois do `$` vem o nome de uma função, colado",
                    )
                    .with_label(dollar, "faltou o nome")
                    .with_help("exemplo: $ban[user;reason]"),
                )
            }
        };
        self.bump();
        let callee = Expr::new(ExprKind::Name(Ident::new(name.clone(), t.span)), t.span);
        let mut args: Vec<Arg> = Vec::new();
        if self.at(&TokenKind::LBracket) && !self.peek().space_before {
            let open = self.bump().span;
            loop {
                if self.eat(&TokenKind::RBracket) {
                    break;
                }
                args.push(self.arg()?);
                if !(self.eat(&TokenKind::Semi) || self.eat(&TokenKind::Comma)) {
                    self.close(TokenKind::RBracket, open)?;
                    break;
                }
            }
        }
        let span = dollar.to(Span::point(self.prev_end()));
        if !self.opts.dollar {
            let parts: Vec<&str> = args.iter().map(|a| self.text(a.span)).collect();
            let replacement = format!("{name}({})", parts.join(", "));
            self.report(
                Diagnostic::error(
                    codes::E0108,
                    format!("O atalho `${name}[...]` está desligado"),
                )
                .with_label(span, "atalho $ desligado")
                .with_help("ligue em diracord.toml: [syntax] dollar = true")
                .with_fix(Fix::replace(
                    format!("escrever `{replacement}`"),
                    span,
                    replacement.clone(),
                    true,
                )),
            );
        }
        Ok(Expr::new(
            ExprKind::Call {
                callee: Box::new(callee),
                args,
                style: CallStyle::Dollar,
            },
            span,
        ))
    }
}
