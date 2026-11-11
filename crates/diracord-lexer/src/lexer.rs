//! The main tokenizer loop.

use crate::cursor::{is_ident_continue, is_ident_start, Cursor};
use crate::string::lex_string;
use crate::token::{Token, TokenKind};
use diracord_diagnostics::{codes, Diagnostic, Fix, Span};

/// Splits `source` into tokens. The last token is always [`TokenKind::Eof`].
/// Errors are returned next to the tokens; lexing never stops early.
pub fn tokenize(source: &str) -> (Vec<Token>, Vec<Diagnostic>) {
    let mut c = Cursor::new(source);
    let mut tokens = Vec::new();
    let mut diags = Vec::new();
    let mut newline = true;
    let mut space = true;
    loop {
        // Whitespace and comments.
        match c.peek() {
            Some('\n') => {
                c.bump();
                newline = true;
                space = true;
                continue;
            }
            Some(ch) if ch.is_whitespace() => {
                c.bump();
                space = true;
                continue;
            }
            Some('/') if c.peek2() == Some('/') => {
                c.eat_while(|ch| ch != '\n');
                space = true;
                continue;
            }
            _ => {}
        }
        let start = c.pos();
        let Some(ch) = c.bump() else {
            tokens.push(Token {
                kind: TokenKind::Eof,
                span: Span::point(start),
                newline_before: true,
                space_before: true,
            });
            break;
        };
        let kind = match ch {
            '"' => TokenKind::Str(lex_string(&mut c, start, &mut diags)),
            '0'..='9' => lex_number(&mut c, start, &mut diags),
            ch if is_ident_start(ch) => {
                c.eat_while(is_ident_continue);
                TokenKind::Ident(c.slice(start).to_string())
            }
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semi,
            '.' => TokenKind::Dot,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '$' => TokenKind::Dollar,
            '=' if c.eat('>') => TokenKind::FatArrow,
            '=' if c.eat('=') => TokenKind::EqEq,
            '=' => TokenKind::Assign,
            '!' if c.eat('=') => TokenKind::NotEq,
            '!' => TokenKind::Bang,
            '<' if c.eat('=') => TokenKind::Le,
            '<' => TokenKind::Lt,
            '>' if c.eat('=') => TokenKind::Ge,
            '>' => TokenKind::Gt,
            '&' if c.eat('&') => TokenKind::AndAnd,
            '|' if c.eat('|') => TokenKind::OrOr,
            other => {
                diags.push(unexpected_char(other, Span::new(start, c.pos())));
                space = true;
                continue;
            }
        };
        tokens.push(Token {
            kind,
            span: Span::new(start, c.pos()),
            newline_before: newline,
            space_before: space,
        });
        newline = false;
        space = false;
    }
    (tokens, diags)
}

fn lex_number(c: &mut Cursor<'_>, start: u32, diags: &mut Vec<Diagnostic>) -> TokenKind {
    c.eat_while(|ch| ch.is_ascii_digit() || ch == '_');
    let mut is_float = false;
    if c.peek() == Some('.') && c.peek2().is_some_and(|ch| ch.is_ascii_digit()) {
        c.bump();
        c.eat_while(|ch| ch.is_ascii_digit() || ch == '_');
        is_float = true;
    }
    // `15abc` is a mistake, not `15` followed by `abc`.
    if c.peek().is_some_and(is_ident_start) {
        c.eat_while(is_ident_continue);
        let span = Span::new(start, c.pos());
        diags.push(
            Diagnostic::error(
                codes::E0003,
                format!("`{}` não é um número válido", c.slice(start)),
            )
            .with_label(span, "número inválido")
            .with_help(
                "números usam só dígitos, como 15 ou 1.5; nomes não podem começar com dígito",
            ),
        );
        return TokenKind::Int(0);
    }
    let text: String = c.slice(start).chars().filter(|&ch| ch != '_').collect();
    if is_float {
        TokenKind::Float(text.parse().unwrap_or(0.0))
    } else {
        match text.parse::<i64>() {
            Ok(i) => TokenKind::Int(i),
            Err(_) => {
                diags.push(
                    Diagnostic::error(codes::E0003, "Esse número é grande demais")
                        .with_label(Span::new(start, c.pos()), "não cabe num número inteiro")
                        .with_help(
                            "para ids do Discord, use texto entre aspas: \"123456789012345678901\"",
                        ),
                );
                TokenKind::Int(0)
            }
        }
    }
}

fn unexpected_char(ch: char, span: Span) -> Diagnostic {
    let mut d = Diagnostic::error(
        codes::E0002,
        format!("O caractere `{ch}` não faz parte do Diracord"),
    )
    .with_label(span, "caractere inesperado");
    match ch {
        '&' => d = d.with_help("para \"e\" use `&&` ou `and`"),
        '|' => d = d.with_help("para \"ou\" use `||` ou `or`"),
        '\'' => {
            d = d
                .with_help("textos usam aspas duplas: \"assim\"")
                .with_fix(Fix::replace("trocar por aspas duplas", span, "\"", false))
        }
        '“' | '”' => {
            d = d
                .with_help("essa é uma aspa \"curva\" (de editor de texto); use a aspa reta \"")
                .with_fix(Fix::replace("trocar por aspa reta", span, "\"", true))
        }
        '#' => d = d.with_help("comentários começam com //"),
        _ => d = d.with_help("apague esse caractere ou coloque-o dentro de um texto entre aspas"),
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StrPart, TokenKind as K};

    fn kinds(src: &str) -> Vec<K> {
        let (t, d) = tokenize(src);
        assert!(d.is_empty(), "{d:?}");
        t.into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn command_header() {
        assert_eq!(
            kinds("command(\"ping\") { }"),
            vec![
                K::Ident("command".into()),
                K::LParen,
                K::Str(vec![StrPart::Text("ping".into())]),
                K::RParen,
                K::LBrace,
                K::RBrace,
                K::Eof
            ]
        );
    }

    #[test]
    fn operators() {
        assert_eq!(
            kinds("== != <= >= => = && || ! < > + - * / %"),
            vec![
                K::EqEq,
                K::NotEq,
                K::Le,
                K::Ge,
                K::FatArrow,
                K::Assign,
                K::AndAnd,
                K::OrOr,
                K::Bang,
                K::Lt,
                K::Gt,
                K::Plus,
                K::Minus,
                K::Star,
                K::Slash,
                K::Percent,
                K::Eof
            ]
        );
    }

    #[test]
    fn numbers() {
        assert_eq!(
            kinds("15 1.5 1_000 x.y"),
            vec![
                K::Int(15),
                K::Float(1.5),
                K::Int(1000),
                K::Ident("x".into()),
                K::Dot,
                K::Ident("y".into()),
                K::Eof
            ]
        );
    }

    #[test]
    fn interpolation() {
        let k = kinds("\"Oi $user.name!\"");
        let K::Str(parts) = &k[0] else { panic!() };
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], StrPart::Text("Oi ".into()));
        let StrPart::Var(v) = &parts[1] else { panic!() };
        let names: Vec<&str> = v.segments.iter().map(|s| s.0.as_str()).collect();
        assert_eq!(names, ["user", "name"]);
        assert_eq!(v.span, Span::new(4, 14));
        assert_eq!(parts[2], StrPart::Text("!".into()));
    }

    #[test]
    fn trailing_dot_is_text() {
        let k = kinds("\"tchau $user.\"");
        let K::Str(parts) = &k[0] else { panic!() };
        assert_eq!(parts.last(), Some(&StrPart::Text(".".into())));
    }

    #[test]
    fn escapes() {
        let k = kinds(r#""a\n\"b\" \$5""#);
        assert_eq!(k[0], K::Str(vec![StrPart::Text("a\n\"b\" $5".into())]));
    }

    #[test]
    fn newline_flags() {
        let (t, _) = tokenize("a b\nc");
        assert!(t[0].newline_before);
        assert!(!t[1].newline_before && t[1].space_before);
        assert!(t[2].newline_before);
        let (t, _) = tokenize("a[0]");
        assert!(!t[1].space_before);
    }

    #[test]
    fn comments_are_skipped() {
        assert_eq!(
            kinds("a // nada aqui\nb"),
            vec![K::Ident("a".into()), K::Ident("b".into()), K::Eof]
        );
    }

    #[test]
    fn errors_are_reported_and_lexing_continues() {
        let (t, d) = tokenize("reply \"oi\nx @ y");
        assert_eq!(d.len(), 2);
        assert_eq!(d[0].code, codes::E0001);
        assert_eq!(d[1].code, codes::E0002);
        assert!(t.iter().any(|t| t.kind == K::Ident("y".into())));
    }

    #[test]
    fn lone_dollar_in_string() {
        let (_, d) = tokenize("\"custa $ 5\"");
        assert_eq!(d[0].code, codes::E0005);
        assert!(d[0].fixes[0].safe);
    }

    #[test]
    fn bad_numbers() {
        let (_, d) = tokenize("15abc 99999999999999999999");
        assert_eq!(d.len(), 2);
        assert!(d.iter().all(|d| d.code == codes::E0003));
    }

    #[test]
    fn dollar_token() {
        assert_eq!(kinds("$ban[a;b]")[0], K::Dollar);
    }
}
