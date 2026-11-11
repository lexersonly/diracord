//! Parser state, token helpers and error recovery.

use crate::ParseOptions;
use diracord_ast::known;
use diracord_diagnostics::{codes, Diagnostic, Span};
use diracord_lexer::{Token, TokenKind};

/// Marker: an error was already reported; the caller should recover.
#[derive(Debug)]
pub(crate) struct Fail;

pub(crate) type PResult<T> = Result<T, Fail>;

/// Maximum nesting of blocks/expressions before giving up (protects the stack).
const MAX_DEPTH: u32 = 200;

pub(crate) struct Parser<'a> {
    pub(crate) src: &'a str,
    toks: Vec<Token>,
    pos: usize,
    pub(crate) opts: ParseOptions,
    diags: Vec<Diagnostic>,
    depth: u32,
    prev_end: u32,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(src: &'a str, toks: Vec<Token>, opts: ParseOptions) -> Self {
        Parser {
            src,
            toks,
            pos: 0,
            opts,
            diags: Vec::new(),
            depth: 0,
            prev_end: 0,
        }
    }

    pub(crate) fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diags
    }

    // ---- tokens ----

    pub(crate) fn peek(&self) -> &Token {
        self.nth(0)
    }

    pub(crate) fn nth(&self, n: usize) -> &Token {
        let i = (self.pos + n).min(self.toks.len() - 1);
        &self.toks[i]
    }

    pub(crate) fn kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    pub(crate) fn span(&self) -> Span {
        self.peek().span
    }

    /// End offset of the last consumed token.
    pub(crate) fn prev_end(&self) -> u32 {
        self.prev_end
    }

    pub(crate) fn at_eof(&self) -> bool {
        matches!(self.kind(), TokenKind::Eof)
    }

    pub(crate) fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if !matches!(t.kind, TokenKind::Eof) {
            self.pos += 1;
            self.prev_end = t.span.end;
        }
        t
    }

    pub(crate) fn at(&self, kind: &TokenKind) -> bool {
        self.kind() == kind
    }

    pub(crate) fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// `true` if the current token is the name `word`.
    pub(crate) fn at_word(&self, word: &str) -> bool {
        matches!(self.kind(), TokenKind::Ident(s) if s == word)
    }

    /// The current token is a name that is not a reserved word.
    pub(crate) fn at_plain_ident(&self) -> bool {
        matches!(self.kind(), TokenKind::Ident(s) if !known::is_keyword(s))
    }

    /// Text of a span in the source.
    pub(crate) fn text(&self, span: Span) -> &'a str {
        &self.src[span.range()]
    }

    // ---- diagnostics ----

    pub(crate) fn report(&mut self, d: Diagnostic) {
        // One error per position is enough; avoid cascades.
        let start = d.primary_span().map(|s| s.start);
        if self
            .diags
            .last()
            .is_some_and(|l| l.primary_span().map(|s| s.start) == start)
        {
            return;
        }
        self.diags.push(d);
    }

    pub(crate) fn fail<T>(&mut self, d: Diagnostic) -> PResult<T> {
        self.report(d);
        Err(Fail)
    }

    /// Generic "unexpected token" error at the current token.
    pub(crate) fn unexpected<T>(&mut self, expected: &str) -> PResult<T> {
        let t = self.peek().clone();
        let d = Diagnostic::error(
            codes::E0100,
            format!("Esperava {expected}, mas encontrei {}", t.kind.describe()),
        )
        .with_label(t.span, format!("esperava {expected}"));
        self.fail(d)
    }

    /// Consumes an identifier, or reports `E0104`.
    pub(crate) fn ident(&mut self, what: &str) -> PResult<diracord_ast::Ident> {
        match self.kind().clone() {
            TokenKind::Ident(name) if !known::is_keyword(&name) => {
                let t = self.bump();
                Ok(diracord_ast::Ident::new(name, t.span))
            }
            TokenKind::Ident(name) => {
                let span = self.span();
                self.fail(
                    Diagnostic::error(
                        codes::E0104,
                        format!("`{name}` é uma palavra reservada e não pode ser {what}"),
                    )
                    .with_label(span, "palavra reservada")
                    .with_help("escolha outro nome"),
                )
            }
            other => {
                let span = self.span();
                self.fail(
                    Diagnostic::error(codes::E0104, format!("Faltou {what}")).with_label(
                        span,
                        format!("esperava um nome, encontrei {}", other.describe()),
                    ),
                )
            }
        }
    }

    // ---- nesting guard ----

    pub(crate) fn enter(&mut self) -> PResult<()> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            let span = self.span();
            return self.fail(
                Diagnostic::error(codes::E0211, "Tem blocos demais um dentro do outro")
                    .with_label(span, "aninhamento demais aqui")
                    .with_help("divida o código em partes menores"),
            );
        }
        Ok(())
    }

    pub(crate) fn leave(&mut self) {
        self.depth -= 1;
    }

    // ---- recovery ----

    /// Skips to the start of the next statement: a token on a new line, `;`,
    /// or the `}` closing the current block (not consumed).
    pub(crate) fn sync_stmt(&mut self) {
        let mut depth = 0i32;
        let mut first = true;
        loop {
            let t = self.peek();
            match t.kind {
                TokenKind::Eof => return,
                TokenKind::RBrace if depth == 0 => return,
                TokenKind::Semi if depth == 0 => {
                    self.bump();
                    return;
                }
                _ if depth == 0 && t.newline_before && !first => return,
                TokenKind::LBrace | TokenKind::LParen | TokenKind::LBracket => depth += 1,
                TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket => depth -= 1,
                _ => {}
            }
            first = false;
            self.bump();
            if depth < 0 {
                depth = 0;
            }
        }
    }

    /// Skips to the next line that starts at brace depth 0 (a likely item start).
    pub(crate) fn sync_item(&mut self) {
        let mut depth = 0i32;
        let mut first = true;
        loop {
            let t = self.peek();
            if matches!(t.kind, TokenKind::Eof) {
                return;
            }
            if depth <= 0 && t.newline_before && !first && matches!(t.kind, TokenKind::Ident(_)) {
                return;
            }
            match t.kind {
                TokenKind::LBrace => depth += 1,
                TokenKind::RBrace => depth -= 1,
                _ => {}
            }
            first = false;
            self.bump();
        }
    }
}
