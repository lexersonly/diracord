//! Token types.

use diracord_diagnostics::Span;

/// One token.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// What it is.
    pub kind: TokenKind,
    /// Where.
    pub span: Span,
    /// A line break (or the start of the file) came before this token.
    pub newline_before: bool,
    /// Whitespace (or a line break) came right before this token.
    pub space_before: bool,
}

/// `$user.name` inside a string: a name followed by `.field`s.
#[derive(Clone, Debug, PartialEq)]
pub struct VarPath {
    /// The segments with their spans (`user`, `name`).
    pub segments: Vec<(String, Span)>,
    /// From the `$` to the end of the last segment.
    pub span: Span,
}

/// A piece of a string literal.
#[derive(Clone, Debug, PartialEq)]
pub enum StrPart {
    /// Plain text, escapes resolved.
    Text(String),
    /// An interpolated variable.
    Var(VarPath),
}

/// The kinds of tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// A name or keyword (`reply`, `if`, `true`, `_`). The parser decides which.
    Ident(String),
    /// Whole number.
    Int(i64),
    /// Decimal number.
    Float(f64),
    /// String literal, split into text and `$var` parts.
    Str(Vec<StrPart>),
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `;`
    Semi,
    /// `.`
    Dot,
    /// `=>`
    FatArrow,
    /// `=`
    Assign,
    /// `==`
    EqEq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `!`
    Bang,
    /// `&&`
    AndAnd,
    /// `||`
    OrOr,
    /// `$` (start of `$name[...]`)
    Dollar,
    /// End of input (always the last token).
    Eof,
}

impl TokenKind {
    /// How the token is shown in messages ("`{`", "um nome", "um texto").
    pub fn describe(&self) -> String {
        match self {
            TokenKind::Ident(s) => format!("`{s}`"),
            TokenKind::Int(i) => format!("o número `{i}`"),
            TokenKind::Float(f) => format!("o número `{f}`"),
            TokenKind::Str(_) => "um texto".to_string(),
            TokenKind::Eof => "o fim do arquivo".to_string(),
            other => format!("`{}`", other.symbol()),
        }
    }

    /// The symbol of a punctuation token (empty for others).
    pub fn symbol(&self) -> &'static str {
        match self {
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Comma => ",",
            TokenKind::Colon => ":",
            TokenKind::Semi => ";",
            TokenKind::Dot => ".",
            TokenKind::FatArrow => "=>",
            TokenKind::Assign => "=",
            TokenKind::EqEq => "==",
            TokenKind::NotEq => "!=",
            TokenKind::Lt => "<",
            TokenKind::Le => "<=",
            TokenKind::Gt => ">",
            TokenKind::Ge => ">=",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Bang => "!",
            TokenKind::AndAnd => "&&",
            TokenKind::OrOr => "||",
            TokenKind::Dollar => "$",
            _ => "",
        }
    }
}
