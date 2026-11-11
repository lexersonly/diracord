//! Expressions, including calls with and without parentheses.

use crate::parser::{PResult, Parser};
use diracord_ast::{
    known, Arg, BinaryOp, CallStyle, Expr, ExprKind, Ident, Literal, ObjectField, TemplatePart,
    UnaryOp,
};
use diracord_diagnostics::{codes, Diagnostic, Fix, Span};
use diracord_lexer::{StrPart, Token, TokenKind};

impl Parser<'_> {
    /// An expression, or a call without parentheses (`reply "oi"`).
    /// Used at statement start and after `local x =`, `x =`, `return`.
    pub(crate) fn expr_or_bare(&mut self) -> PResult<Expr> {
        let is_layout = self.at_word("layout") && self.nth(1).kind == TokenKind::LBrace;
        if self.at_plain_ident() && !is_layout {
            let next = self.nth(1);
            if !next.newline_before && next.space_before && starts_bare_arg(next, self.nth(2)) {
                return self.bare_call();
            }
        }
        self.expr(true)
    }

    fn bare_call(&mut self) -> PResult<Expr> {
        let t = self.bump();
        let TokenKind::Ident(name) = t.kind else {
            unreachable!()
        };
        let callee = Expr::new(ExprKind::Name(Ident::new(name, t.span)), t.span);
        let mut args = Vec::new();
        loop {
            let tok = self.peek();
            if tok.newline_before || !starts_bare_arg(tok, self.nth(1)) {
                break;
            }
            args.push(self.arg()?);
            if self.at(&TokenKind::Comma) && !self.nth(1).newline_before {
                self.bump();
            }
        }
        let span = t.span.to(Span::point(self.prev_end()));
        Ok(Expr::new(
            ExprKind::Call {
                callee: Box::new(callee),
                args,
                style: CallStyle::Bare,
            },
            span,
        ))
    }

    /// `name: value` or `value`.
    pub(crate) fn arg(&mut self) -> PResult<Arg> {
        let start = self.span();
        if self.at_plain_ident() && self.nth(1).kind == TokenKind::Colon {
            let t = self.bump();
            self.bump(); // :
            let TokenKind::Ident(n) = t.kind else {
                unreachable!()
            };
            let value = self.expr(true)?;
            let span = start.to(value.span);
            return Ok(Arg {
                name: Some(Ident::new(n, t.span)),
                value,
                span,
            });
        }
        let value = self.expr(true)?;
        Ok(Arg {
            name: None,
            span: value.span,
            value,
        })
    }

    /// A full expression. `allow_obj = false` in conditions, where `{` opens the block.
    pub(crate) fn expr(&mut self, allow_obj: bool) -> PResult<Expr> {
        self.enter()?;
        let r = self.binary(1, allow_obj);
        self.leave();
        r
    }

    fn binary(&mut self, min_prec: u8, allow_obj: bool) -> PResult<Expr> {
        let mut lhs = self.unary(allow_obj)?;
        loop {
            let t = self.peek();
            // An operator at the start of a new line begins a new statement.
            if t.newline_before {
                break;
            }
            let Some(op) = binary_op(&t.kind) else { break };
            let prec = op.precedence();
            if prec < min_prec {
                break;
            }
            self.bump();
            let rhs = self.binary(prec + 1, allow_obj)?;
            let span = lhs.span.to(rhs.span);
            lhs = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                span,
            );
        }
        Ok(lhs)
    }

    fn unary(&mut self, allow_obj: bool) -> PResult<Expr> {
        let op = match self.kind() {
            TokenKind::Minus => Some(UnaryOp::Neg),
            TokenKind::Bang => Some(UnaryOp::Not),
            TokenKind::Ident(s) if s == "not" => Some(UnaryOp::Not),
            _ => None,
        };
        if let Some(op) = op {
            let start = self.bump().span;
            self.enter()?;
            let e = self.unary(allow_obj);
            self.leave();
            let e = e?;
            let span = start.to(e.span);
            return Ok(Expr::new(
                ExprKind::Unary {
                    op,
                    expr: Box::new(e),
                },
                span,
            ));
        }
        self.postfix(allow_obj)
    }

    fn postfix(&mut self, allow_obj: bool) -> PResult<Expr> {
        let mut e = self.primary(allow_obj)?;
        loop {
            let t = self.peek().clone();
            match t.kind {
                TokenKind::Dot => {
                    self.bump();
                    let field = self.field_name()?;
                    let span = e.span.to(field.span);
                    e = Expr::new(
                        ExprKind::Member {
                            object: Box::new(e),
                            field,
                        },
                        span,
                    );
                }
                TokenKind::LBracket if !t.space_before => {
                    self.bump();
                    let index = self.expr(true)?;
                    self.close(TokenKind::RBracket, t.span)?;
                    let span = e.span.to(Span::point(self.prev_end()));
                    e = Expr::new(
                        ExprKind::Index {
                            object: Box::new(e),
                            index: Box::new(index),
                        },
                        span,
                    );
                }
                TokenKind::LParen if !t.newline_before => {
                    self.bump();
                    let args = self.paren_args(t.span)?;
                    let span = e.span.to(Span::point(self.prev_end()));
                    e = Expr::new(
                        ExprKind::Call {
                            callee: Box::new(e),
                            args,
                            style: CallStyle::Parens,
                        },
                        span,
                    );
                }
                _ => break,
            }
        }
        Ok(e)
    }

    /// A field after `.`: any name, reserved words included (`x.in` is fine).
    fn field_name(&mut self) -> PResult<Ident> {
        match self.kind().clone() {
            TokenKind::Ident(n) => {
                let t = self.bump();
                Ok(Ident::new(n, t.span))
            }
            _ => self.unexpected("o nome do campo depois do `.`"),
        }
    }

    /// Arguments after `(` (already consumed) up to `)`.
    pub(crate) fn paren_args(&mut self, open: Span) -> PResult<Vec<Arg>> {
        let mut args = Vec::new();
        loop {
            if self.eat(&TokenKind::RParen) {
                return Ok(args);
            }
            if matches!(
                self.kind(),
                TokenKind::Eof | TokenKind::RBrace | TokenKind::LBrace
            ) {
                self.close_paren(open)?;
                return Ok(args);
            }
            args.push(self.arg()?);
            if !self.eat(&TokenKind::Comma) {
                self.close_paren(open)?;
                return Ok(args);
            }
        }
    }

    /// Expects a closing `]` (or other closer) matching `open`.
    pub(crate) fn close(&mut self, closer: TokenKind, open: Span) -> PResult<()> {
        if self.eat(&closer) {
            return Ok(());
        }
        let t = self.peek().clone();
        let here = Span::point(self.prev_end());
        let sym = closer.symbol();
        let (code, msg) = match closer {
            TokenKind::RBracket => (codes::E0111, "Faltou fechar o colchete"),
            TokenKind::RBrace => (codes::E0101, "Faltou fechar a chave"),
            _ => (codes::E0102, "Faltou fechar o parêntese"),
        };
        self.fail(
            Diagnostic::error(code, msg)
                .with_label(open, "aberto aqui")
                .with_secondary(
                    t.span,
                    format!("esperava `{sym}`, encontrei {}", t.kind.describe()),
                )
                .with_fix(Fix::replace(format!("colocar `{sym}`"), here, sym, false)),
        )
    }

    fn primary(&mut self, allow_obj: bool) -> PResult<Expr> {
        let t = self.peek().clone();
        let lit = |l| Ok(Expr::new(ExprKind::Literal(l), t.span));
        match &t.kind {
            TokenKind::Int(i) => {
                self.bump();
                lit(Literal::Int(*i))
            }
            TokenKind::Float(f) => {
                self.bump();
                lit(Literal::Float(*f))
            }
            TokenKind::Str(parts) => {
                self.bump();
                Ok(string_expr(parts, t.span))
            }
            TokenKind::Ident(s) => match s.as_str() {
                "true" => {
                    self.bump();
                    lit(Literal::Bool(true))
                }
                "false" => {
                    self.bump();
                    lit(Literal::Bool(false))
                }
                "null" => {
                    self.bump();
                    lit(Literal::Null)
                }
                kw if known::is_keyword(kw) => self.fail(
                    Diagnostic::error(
                        codes::E0103,
                        format!("Esperava um valor, mas `{kw}` é uma palavra reservada"),
                    )
                    .with_label(t.span, "esperava um valor aqui"),
                ),
                "layout"
                    if allow_obj
                        && self.nth(1).kind == TokenKind::LBrace
                        && !self.nth(1).newline_before =>
                {
                    let l = self.layout_expr()?;
                    let span = t.span.to(l.span);
                    Ok(Expr::new(ExprKind::Layout(l), span))
                }
                name => {
                    self.bump();
                    Ok(Expr::new(ExprKind::Name(Ident::new(name, t.span)), t.span))
                }
            },
            TokenKind::LParen => {
                self.bump();
                let e = self.expr(true)?;
                self.close(TokenKind::RParen, t.span)?;
                Ok(Expr::new(e.kind, t.span.to(Span::point(self.prev_end()))))
            }
            TokenKind::LBracket => self.list(),
            TokenKind::LBrace if allow_obj => self.object(),
            TokenKind::Dollar => self.dollar_call(),
            _ => {
                let mut d = Diagnostic::error(
                    codes::E0103,
                    format!("Esperava um valor, mas encontrei {}", t.kind.describe()),
                )
                .with_label(t.span, "esperava um valor aqui");
                if matches!(t.kind, TokenKind::Eof | TokenKind::RBrace) || t.newline_before {
                    d = d.with_help("a linha terminou antes do valor; complete a expressão");
                }
                self.fail(d)
            }
        }
    }

    fn list(&mut self) -> PResult<Expr> {
        let open = self.bump().span;
        let mut items = Vec::new();
        loop {
            if self.eat(&TokenKind::RBracket) {
                break;
            }
            if self.at_eof() {
                self.close(TokenKind::RBracket, open)?;
            }
            items.push(self.expr(true)?);
            if !self.eat(&TokenKind::Comma) {
                self.close(TokenKind::RBracket, open)?;
                break;
            }
        }
        Ok(Expr::new(
            ExprKind::List(items),
            open.to(Span::point(self.prev_end())),
        ))
    }

    /// `{ key: value, ... }` — commas or line breaks between fields.
    fn object(&mut self) -> PResult<Expr> {
        let open = self.bump().span;
        let mut fields = Vec::new();
        loop {
            while self.eat(&TokenKind::Comma) {}
            if self.eat(&TokenKind::RBrace) {
                break;
            }
            if self.at_eof() {
                self.close(TokenKind::RBrace, open)?;
            }
            let kt = self.peek().clone();
            let key = match &kt.kind {
                TokenKind::Ident(n) => Ident::new(n.clone(), kt.span),
                TokenKind::Str(parts) => match parts.as_slice() {
                    [StrPart::Text(s)] => Ident::new(s.clone(), kt.span),
                    _ => return self.unexpected("o nome do campo (sem `$`)"),
                },
                _ => return self.unexpected("o nome de um campo, como `title:`"),
            };
            self.bump();
            if !self.eat(&TokenKind::Colon) {
                let span = self.span();
                return self.fail(
                    Diagnostic::error(
                        codes::E0100,
                        format!("Faltou `:` depois do campo `{}`", key.name),
                    )
                    .with_label(span, "esperava `:`")
                    .with_help(format!("exemplo: {{ {}: \"valor\" }}", key.name)),
                );
            }
            let value = self.expr(true)?;
            let span = key.span.to(value.span);
            fields.push(ObjectField { key, value, span });
            let t = self.peek();
            if !matches!(t.kind, TokenKind::Comma | TokenKind::RBrace) && !t.newline_before {
                return self.unexpected("`,` ou `}` depois do campo");
            }
        }
        Ok(Expr::new(
            ExprKind::Object(fields),
            open.to(Span::point(self.prev_end())),
        ))
    }
}

/// Can `tok` begin an argument of a call without parentheses?
/// `after` is the token following it (a value followed by `=>` is a chain case).
fn starts_bare_arg(tok: &Token, after: &Token) -> bool {
    if after.kind == TokenKind::FatArrow {
        return false;
    }
    match &tok.kind {
        TokenKind::Ident(s) => {
            !known::is_keyword(s) || matches!(s.as_str(), "true" | "false" | "null" | "not")
        }
        TokenKind::Int(_)
        | TokenKind::Float(_)
        | TokenKind::Str(_)
        | TokenKind::LBracket
        | TokenKind::LBrace
        | TokenKind::Dollar
        | TokenKind::Bang => true,
        _ => false,
    }
}

fn binary_op(k: &TokenKind) -> Option<BinaryOp> {
    Some(match k {
        TokenKind::Plus => BinaryOp::Add,
        TokenKind::Minus => BinaryOp::Sub,
        TokenKind::Star => BinaryOp::Mul,
        TokenKind::Slash => BinaryOp::Div,
        TokenKind::Percent => BinaryOp::Rem,
        TokenKind::EqEq => BinaryOp::Eq,
        TokenKind::NotEq => BinaryOp::Ne,
        TokenKind::Lt => BinaryOp::Lt,
        TokenKind::Le => BinaryOp::Le,
        TokenKind::Gt => BinaryOp::Gt,
        TokenKind::Ge => BinaryOp::Ge,
        TokenKind::AndAnd => BinaryOp::And,
        TokenKind::OrOr => BinaryOp::Or,
        TokenKind::Ident(s) if s == "and" => BinaryOp::And,
        TokenKind::Ident(s) if s == "or" => BinaryOp::Or,
        _ => return None,
    })
}

/// A string token becomes a plain literal or a template.
fn string_expr(parts: &[StrPart], span: Span) -> Expr {
    if let [StrPart::Text(s)] = parts {
        return Expr::new(ExprKind::Literal(Literal::Text(s.clone())), span);
    }
    let parts = parts
        .iter()
        .map(|p| match p {
            StrPart::Text(s) => TemplatePart::Text(s.clone()),
            StrPart::Var(v) => {
                let (first, first_span) = &v.segments[0];
                let mut e = Expr::new(
                    ExprKind::Name(Ident::new(first.clone(), *first_span)),
                    Span::new(v.span.start, first_span.end),
                );
                for (name, s) in &v.segments[1..] {
                    let span = e.span.to(*s);
                    e = Expr::new(
                        ExprKind::Member {
                            object: Box::new(e),
                            field: Ident::new(name.clone(), *s),
                        },
                        span,
                    );
                }
                TemplatePart::Expr(e)
            }
        })
        .collect();
    Expr::new(ExprKind::Template(parts), span)
}
