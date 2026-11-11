//! Blocks and statements.

use crate::parser::{Fail, PResult, Parser};
use diracord_ast::{
    Block, ChainArm, Expr, ExprKind, IfBranch, Literal, Pattern, PatternKind, Stmt, StmtKind,
};
use diracord_diagnostics::{codes, Diagnostic, Fix, Span};
use diracord_lexer::{StrPart, TokenKind};

impl Parser<'_> {
    /// `{ stmt* }`
    pub(crate) fn block(&mut self) -> PResult<Block> {
        if !self.at(&TokenKind::LBrace) {
            let t = self.peek().clone();
            let here = Span::point(self.prev_end());
            return self.fail(
                Diagnostic::error(
                    codes::E0105,
                    format!(
                        "Esperava um bloco `{{ ... }}`, mas encontrei {}",
                        t.kind.describe()
                    ),
                )
                .with_label(t.span, "esperava `{`")
                .with_help("abra o bloco com `{` e feche com `}`")
                .with_fix(Fix::replace("abrir o bloco", here, " {", false)),
            );
        }
        self.enter()?;
        let open = self.bump().span;
        let mut stmts = Vec::new();
        loop {
            while self.eat(&TokenKind::Semi) {}
            if self.eat(&TokenKind::RBrace) {
                break;
            }
            if self.at_eof() {
                let end = Span::point(self.src.len() as u32);
                self.report(
                    Diagnostic::error(codes::E0101, "Faltou fechar a chave")
                        .with_label(open, "esta chave nunca foi fechada")
                        .with_secondary(end, "o arquivo acabou aqui")
                        .with_help("coloque `}` no fim do bloco")
                        .with_fix(Fix::replace(
                            "fechar a chave no fim do arquivo",
                            end,
                            "}\n",
                            false,
                        )),
                );
                break;
            }
            let before = self.peek().span.start;
            match self.stmt() {
                Ok(s) => {
                    let bare = matches!(
                        &s.kind,
                        StmtKind::Expr(Expr {
                            kind: ExprKind::Call {
                                style: diracord_ast::CallStyle::Bare,
                                ..
                            },
                            ..
                        })
                    );
                    stmts.push(s);
                    if self.end_of_stmt(bare).is_err() {
                        self.sync_stmt();
                    }
                }
                Err(Fail) => self.sync_stmt(),
            }
            if self.peek().span.start == before && !self.at(&TokenKind::RBrace) && !self.at_eof() {
                self.bump();
            }
        }
        self.leave();
        Ok(Block {
            stmts,
            span: open.to(Span::point(self.prev_end())),
        })
    }

    /// After a statement: a line break, `;`, `}` or the end of the file.
    fn end_of_stmt(&mut self, after_bare_call: bool) -> PResult<()> {
        let t = self.peek().clone();
        if t.newline_before
            || matches!(t.kind, TokenKind::Semi | TokenKind::RBrace | TokenKind::Eof)
        {
            return Ok(());
        }
        let mut d = Diagnostic::error(
            codes::E0100,
            format!(
                "Esperava o fim da linha, mas encontrei {}",
                t.kind.describe()
            ),
        )
        .with_label(t.span, "isto deveria estar em outra linha");
        if after_bare_call && is_operator(&t.kind) {
            d = d.with_help("numa chamada sem parênteses, cada argumento é separado por espaço; para fazer contas, use parênteses: reply(\"a\" + b)");
        } else {
            d = d.with_help("coloque uma instrução por linha, ou separe com `;`");
        }
        self.fail(d)
    }

    pub(crate) fn stmt(&mut self) -> PResult<Stmt> {
        let start = self.span();
        let word = match self.kind() {
            TokenKind::Ident(s) => s.clone(),
            _ => String::new(),
        };
        let kind = match word.as_str() {
            "local" => self.local()?,
            "if" => self.if_stmt()?,
            "for" => self.for_stmt()?,
            "while" => {
                self.bump();
                let cond = self.condition("while")?;
                let body = self.block()?;
                StmtKind::While { cond, body }
            }
            "chain" => self.chain()?,
            "testing" => self.try_stmt()?,
            "try" => {
                let span = self.span();
                self.report(
                    Diagnostic::error(
                        codes::E0100,
                        "No Diracord, o tratamento de erro usa `testing`, não `try`",
                    )
                    .with_label(span, "use `testing`")
                    .with_help("exemplo: testing { ban user } catch err { reply \"Failed: $err\" }")
                    .with_fix(Fix::replace(
                        "trocar `try` por `testing`",
                        span,
                        "testing",
                        true,
                    )),
                );
                self.try_stmt()?
            }
            "return" => {
                self.bump();
                let t = self.peek();
                let ends = t.newline_before
                    || matches!(t.kind, TokenKind::RBrace | TokenKind::Semi | TokenKind::Eof);
                StmtKind::Return(if ends {
                    None
                } else {
                    Some(self.expr_or_bare()?)
                })
            }
            "stop" => {
                self.bump();
                StmtKind::Stop
            }
            "break" => {
                self.bump();
                StmtKind::Break
            }
            "continue" => {
                self.bump();
                StmtKind::Continue
            }
            "else" | "catch" => {
                let span = self.span();
                let msg = if word == "else" {
                    "`else` sem `if` antes"
                } else {
                    "`catch` sem `testing` antes"
                };
                return self.fail(
                    Diagnostic::error(codes::E0100, msg)
                        .with_label(span, "não tem com o que combinar")
                        .with_help("confira se a chave `}` antes dele não fechou o bloco errado"),
                );
            }
            "let" | "var" | "const" if matches!(self.nth(1).kind, TokenKind::Ident(_)) => {
                let span = self.span();
                self.report(
                    Diagnostic::error(
                        codes::E0100,
                        format!("No Diracord, variáveis são criadas com `local`, não `{word}`"),
                    )
                    .with_label(span, "use `local`")
                    .with_help("exemplo: local x = 15")
                    .with_fix(Fix::replace(
                        format!("trocar `{word}` por `local`"),
                        span,
                        "local",
                        true,
                    )),
                );
                self.local()?
            }
            _ => self.expr_stmt()?,
        };
        Ok(Stmt {
            kind,
            span: start.to(Span::point(self.prev_end())),
        })
    }

    fn local(&mut self) -> PResult<StmtKind> {
        self.bump(); // local
        let name = self.ident("o nome da variável")?;
        if !self.eat(&TokenKind::Assign) {
            let span = self.span();
            return self.fail(
                Diagnostic::error(
                    codes::E0100,
                    format!("Faltou o valor inicial de `{}`", name.name),
                )
                .with_label(span, "esperava `=`")
                .with_help(format!("exemplo: local {} = 0", name.name)),
            );
        }
        let value = self.expr_or_bare()?;
        Ok(StmtKind::Local { name, value })
    }

    fn expr_stmt(&mut self) -> PResult<StmtKind> {
        let target = self.expr_or_bare()?;
        if !self.at(&TokenKind::Assign) {
            return Ok(StmtKind::Expr(target));
        }
        let eq = self.bump().span;
        if !matches!(
            target.kind,
            ExprKind::Name(_) | ExprKind::Member { .. } | ExprKind::Index { .. }
        ) {
            self.report(
                Diagnostic::error(codes::E0106, "Não dá para guardar um valor aqui")
                    .with_label(target.span, "isto não é uma variável")
                    .with_secondary(eq, "atribuição")
                    .with_help("para comparar, use `==`; para criar uma variável, use `local nome = valor`"),
            );
        }
        let value = self.expr_or_bare()?;
        Ok(StmtKind::Assign { target, value })
    }

    /// Condition of `if`/`while`/`chain`: no `{` objects, and a friendly error for `=`.
    fn condition(&mut self, keyword: &str) -> PResult<Expr> {
        if self.at(&TokenKind::LBrace) {
            let span = self.span();
            return self.fail(
                Diagnostic::error(codes::E0103, format!("Faltou a condição do `{keyword}`"))
                    .with_label(span, "esperava uma condição antes do bloco")
                    .with_help(format!("exemplo: {keyword} x > 5 {{ ... }}")),
            );
        }
        let cond = self.expr(false)?;
        if self.at(&TokenKind::Assign) {
            let span = self.span();
            self.report(
                Diagnostic::error(codes::E0100, "Para comparar, use `==` (dois iguais)")
                    .with_label(span, "`=` guarda valor; `==` compara")
                    .with_fix(Fix::replace("trocar `=` por `==`", span, "==", true)),
            );
            self.bump();
            let rhs = self.expr(false)?;
            let span = cond.span.to(rhs.span);
            return Ok(Expr::new(
                ExprKind::Binary {
                    op: diracord_ast::BinaryOp::Eq,
                    lhs: Box::new(cond),
                    rhs: Box::new(rhs),
                },
                span,
            ));
        }
        Ok(cond)
    }

    fn if_stmt(&mut self) -> PResult<StmtKind> {
        let mut branches = Vec::new();
        let mut otherwise = None;
        let start = self.bump().span; // if
        let cond = self.condition("if")?;
        let body = self.block()?;
        branches.push(IfBranch {
            cond,
            body,
            span: start.to(Span::point(self.prev_end())),
        });
        while self.at_word("else") {
            self.bump();
            if self.at_word("if") {
                let s = self.bump().span;
                let cond = self.condition("if")?;
                let body = self.block()?;
                branches.push(IfBranch {
                    cond,
                    body,
                    span: s.to(Span::point(self.prev_end())),
                });
            } else {
                otherwise = Some(self.block()?);
                break;
            }
        }
        Ok(StmtKind::If {
            branches,
            otherwise,
        })
    }

    fn for_stmt(&mut self) -> PResult<StmtKind> {
        self.bump(); // for
        let var = self.ident("o nome da variável do for")?;
        if !self.at_word("in") {
            let span = self.span();
            return self.fail(
                Diagnostic::error(codes::E0100, "Faltou `in` no for")
                    .with_label(span, "esperava `in`")
                    .with_help(format!("exemplo: for {} in lista {{ ... }}", var.name)),
            );
        }
        self.bump();
        let iter = self.condition("for")?;
        let body = self.block()?;
        Ok(StmtKind::For { var, iter, body })
    }

    fn try_stmt(&mut self) -> PResult<StmtKind> {
        self.bump(); // testing (or `try`, already reported)
        let body = self.block()?;
        if !self.at_word("catch") {
            let span = self.span();
            return self.fail(
                Diagnostic::error(codes::E0100, "Faltou o `catch` depois do `testing`")
                    .with_label(span, "esperava `catch`")
                    .with_help(
                        "exemplo: testing { ban user } catch err { reply \"Failed: $err\" }",
                    ),
            );
        }
        self.bump();
        let catch_var = if self.at(&TokenKind::LBrace) {
            None
        } else {
            Some(self.ident("o nome do erro")?)
        };
        let handler = self.block()?;
        Ok(StmtKind::Testing {
            body,
            catch_var,
            handler,
        })
    }

    fn chain(&mut self) -> PResult<StmtKind> {
        self.bump(); // chain
        let subject = self.condition("chain")?;
        if !self.at(&TokenKind::LBrace) {
            return self
                .block()
                .map(|_| unreachable!("block() fails without `{{`"));
        }
        self.enter()?;
        let open = self.bump().span;
        let mut arms: Vec<ChainArm> = Vec::new();
        loop {
            while self.eat(&TokenKind::Comma) || self.eat(&TokenKind::Semi) {}
            if self.eat(&TokenKind::RBrace) {
                break;
            }
            if self.at_eof() {
                let end = Span::point(self.src.len() as u32);
                self.report(
                    Diagnostic::error(codes::E0101, "Faltou fechar a chave do chain")
                        .with_label(open, "esta chave nunca foi fechada")
                        .with_secondary(end, "o arquivo acabou aqui"),
                );
                break;
            }
            match self.arm() {
                Ok(arm) => arms.push(arm),
                Err(Fail) => self.sync_stmt(),
            }
        }
        self.leave();
        if let Some(i) = arms
            .iter()
            .position(|a| a.pattern.kind == PatternKind::Wildcard)
        {
            if i + 1 < arms.len() {
                self.report(
                    Diagnostic::error(codes::E0110, "O caso `_` precisa ser o último do chain")
                        .with_label(arms[i].pattern.span, "este caso pega qualquer valor")
                        .with_secondary(arms[i + 1].pattern.span, "então este nunca roda")
                        .with_help("mova o `_ => ...` para o fim"),
                );
            }
        }
        Ok(StmtKind::Chain { subject, arms })
    }

    fn arm(&mut self) -> PResult<ChainArm> {
        let pattern = self.pattern()?;
        if !self.eat(&TokenKind::FatArrow) {
            let span = self.span();
            return self.fail(
                Diagnostic::error(codes::E0110, "Faltou `=>` no caso do chain")
                    .with_label(span, "esperava `=>`")
                    .with_help("exemplo: 1 => reply \"one\""),
            );
        }
        let body = if self.at(&TokenKind::LBrace) {
            self.block()?
        } else {
            let s = self.stmt()?;
            Block {
                span: s.span,
                stmts: vec![s],
            }
        };
        Ok(ChainArm {
            span: pattern.span.to(body.span),
            pattern,
            body,
        })
    }

    fn pattern(&mut self) -> PResult<Pattern> {
        let t = self.peek().clone();
        let lit = match &t.kind {
            TokenKind::Ident(s) if s == "_" => {
                self.bump();
                return Ok(Pattern {
                    kind: PatternKind::Wildcard,
                    span: t.span,
                });
            }
            TokenKind::Ident(s) if s == "true" => Literal::Bool(true),
            TokenKind::Ident(s) if s == "false" => Literal::Bool(false),
            TokenKind::Ident(s) if s == "null" => Literal::Null,
            TokenKind::Int(i) => Literal::Int(*i),
            TokenKind::Float(f) => Literal::Float(*f),
            TokenKind::Str(parts) => match parts.as_slice() {
                [StrPart::Text(s)] => Literal::Text(s.clone()),
                _ => {
                    return self.fail(
                        Diagnostic::error(
                            codes::E0110,
                            "O caso do chain não pode ter `$variável` no texto",
                        )
                        .with_label(t.span, "texto com variável")
                        .with_help("use um valor fixo, como \"yes\", ou compare com if"),
                    )
                }
            },
            TokenKind::Minus => {
                self.bump();
                let n = self.peek().clone();
                let lit = match n.kind {
                    TokenKind::Int(i) => Literal::Int(-i),
                    TokenKind::Float(f) => Literal::Float(-f),
                    _ => return self.unexpected("um número depois do `-`"),
                };
                self.bump();
                return Ok(Pattern {
                    kind: PatternKind::Literal(lit),
                    span: t.span.to(n.span),
                });
            }
            _ => {
                return self.fail(
                    Diagnostic::error(
                        codes::E0110,
                        format!("{} não pode ser um caso do chain", t.kind.describe()),
                    )
                    .with_label(t.span, "esperava um valor fixo ou `_`")
                    .with_help(
                        "casos são valores fixos: 1, \"a\", true, null ou _ (qualquer outro)",
                    ),
                )
            }
        };
        self.bump();
        Ok(Pattern {
            kind: PatternKind::Literal(lit),
            span: t.span,
        })
    }
}

pub(crate) fn is_operator(k: &TokenKind) -> bool {
    matches!(
        k,
        TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::EqEq
            | TokenKind::NotEq
            | TokenKind::Lt
            | TokenKind::Le
            | TokenKind::Gt
            | TokenKind::Ge
            | TokenKind::AndAnd
            | TokenKind::OrOr
    )
}
