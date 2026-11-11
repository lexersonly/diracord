//! String literals with `$var` interpolation.

use crate::cursor::{is_ident_continue, is_ident_start, Cursor};
use crate::token::{StrPart, VarPath};
use diracord_diagnostics::{codes, Diagnostic, Fix, Span};

/// Lexes a string; the cursor is just after the opening `"`.
pub(crate) fn lex_string(
    c: &mut Cursor<'_>,
    open: u32,
    diags: &mut Vec<Diagnostic>,
) -> Vec<StrPart> {
    let mut parts = Vec::new();
    let mut text = String::new();
    loop {
        let here = c.pos();
        match c.peek() {
            None | Some('\n') => {
                diags.push(
                    Diagnostic::error(codes::E0001, "Faltou fechar o texto com aspas")
                        .with_label(Span::new(open, open + 1), "o texto começa aqui")
                        .with_secondary(Span::point(here), "e não termina nesta linha")
                        .with_help("coloque `\"` no fim do texto")
                        .with_fix(Fix::replace(
                            "fechar as aspas",
                            Span::point(here),
                            "\"",
                            false,
                        )),
                );
                break;
            }
            Some('"') => {
                c.bump();
                break;
            }
            Some('\\') => {
                c.bump();
                match c.bump() {
                    Some('n') => text.push('\n'),
                    Some('t') => text.push('\t'),
                    Some('"') => text.push('"'),
                    Some('\\') => text.push('\\'),
                    Some('$') => text.push('$'),
                    Some('\n') | None => {
                        // Let the outer loop report the unterminated string.
                        text.push('\\');
                    }
                    Some(other) => {
                        let span = Span::new(here, c.pos());
                        diags.push(
                            Diagnostic::error(
                                codes::E0004,
                                format!("O escape `\\{other}` não existe"),
                            )
                            .with_label(span, "escape desconhecido")
                            .with_help(
                                "use \\n, \\t, \\\", \\\\ ou \\$; para uma barra, escreva \\\\",
                            )
                            .with_fix(Fix::replace(
                                "escrever a barra",
                                Span::new(here, here + 1),
                                "\\\\",
                                true,
                            )),
                        );
                        text.push(other);
                    }
                }
            }
            Some('$') => {
                if c.peek2().is_some_and(is_ident_start) {
                    if !text.is_empty() {
                        parts.push(StrPart::Text(std::mem::take(&mut text)));
                    }
                    c.bump();
                    parts.push(StrPart::Var(lex_path(c, here)));
                } else {
                    c.bump();
                    diags.push(
                        Diagnostic::error(
                            codes::E0005,
                            "Um `$` no texto precisa de um nome depois",
                        )
                        .with_label(Span::new(here, here + 1), "falta o nome da variável")
                        .with_help("para escrever o símbolo $, use \\$")
                        .with_fix(Fix::replace(
                            "escrever \\$",
                            Span::point(here),
                            "\\",
                            true,
                        )),
                    );
                    text.push('$');
                }
            }
            Some(ch) => {
                c.bump();
                text.push(ch);
            }
        }
    }
    if !text.is_empty() || parts.is_empty() {
        parts.push(StrPart::Text(text));
    }
    parts
}

/// `name(.name)*` after a `$` at `dollar`.
fn lex_path(c: &mut Cursor<'_>, dollar: u32) -> VarPath {
    let mut segments = Vec::new();
    loop {
        let start = c.pos();
        let name = c.eat_while(is_ident_continue).to_string();
        segments.push((name, Span::new(start, c.pos())));
        if c.peek() == Some('.') && c.peek2().is_some_and(is_ident_start) {
            c.bump();
        } else {
            break;
        }
    }
    let end = segments.last().map(|s| s.1.end).unwrap_or(dollar + 1);
    VarPath {
        segments,
        span: Span::new(dollar, end),
    }
}
