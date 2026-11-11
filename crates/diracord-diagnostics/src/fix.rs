//! Automatic fixes.

use crate::Span;

/// Replace the text at `span` with `replacement` (empty span = insert).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    /// Range to replace.
    pub span: Span,
    /// New text.
    pub replacement: String,
}

/// A suggested change. Only fixes with `safe == true` are applied by `--fix`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix {
    /// What the fix does ("trocar `rpely` por `reply`").
    pub message: String,
    /// The edits, non-overlapping.
    pub edits: Vec<Edit>,
    /// `true` when applying it can never change what the author meant.
    pub safe: bool,
}

impl Fix {
    /// A single-edit fix.
    pub fn replace(
        message: impl Into<String>,
        span: Span,
        replacement: impl Into<String>,
        safe: bool,
    ) -> Self {
        Fix {
            message: message.into(),
            edits: vec![Edit {
                span,
                replacement: replacement.into(),
            }],
            safe,
        }
    }
}

/// Why fixes could not be applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FixError {
    /// Two edits touch the same bytes.
    Overlap(Span, Span),
    /// An edit points outside the text or splits a character.
    OutOfBounds(Span),
}

impl std::fmt::Display for FixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixError::Overlap(a, b) => write!(f, "correções se sobrepõem em {a:?} e {b:?}"),
            FixError::OutOfBounds(s) => write!(f, "correção fora do texto em {s:?}"),
        }
    }
}

impl std::error::Error for FixError {}

/// Applies every **safe** fix to `text` and returns the new text.
///
/// Unsafe fixes are ignored. Fails (and changes nothing) if edits overlap.
///
/// ```
/// use diracord_diagnostics::{apply_fixes, Fix, Span};
/// let fix = Fix::replace("trocar", Span::new(0, 5), "reply", true);
/// assert_eq!(apply_fixes("rpely \"oi\"", &[fix]).unwrap(), "reply \"oi\"");
/// ```
pub fn apply_fixes(text: &str, fixes: &[Fix]) -> Result<String, FixError> {
    let mut edits: Vec<&Edit> = fixes
        .iter()
        .filter(|f| f.safe)
        .flat_map(|f| f.edits.iter())
        .collect();
    edits.sort_by_key(|e| (e.span.start, e.span.end));
    for e in &edits {
        let r = e.span.range();
        if r.end > text.len() || !text.is_char_boundary(r.start) || !text.is_char_boundary(r.end) {
            return Err(FixError::OutOfBounds(e.span));
        }
    }
    for w in edits.windows(2) {
        if w[1].span.start < w[0].span.end || (w[0].span == w[1].span && w[0].span.is_empty()) {
            return Err(FixError::Overlap(w[0].span, w[1].span));
        }
    }
    let mut out = String::with_capacity(text.len());
    let mut pos = 0usize;
    for e in edits {
        out.push_str(&text[pos..e.span.start as usize]);
        out.push_str(&e.replacement);
        pos = e.span.end as usize;
    }
    out.push_str(&text[pos..]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_in_order_and_skips_unsafe() {
        let fixes = vec![
            Fix::replace("b", Span::new(4, 5), "B", true),
            Fix::replace("a", Span::new(0, 1), "A", true),
            Fix::replace("x", Span::new(2, 3), "X", false),
        ];
        assert_eq!(apply_fixes("a c b", &fixes).unwrap(), "A c B");
    }

    #[test]
    fn insert_at_end() {
        let f = Fix::replace("fechar", Span::point(3), "}", true);
        assert_eq!(apply_fixes("a {", &[f]).unwrap(), "a {}");
    }

    #[test]
    fn rejects_overlap() {
        let fixes = vec![
            Fix::replace("1", Span::new(0, 3), "x", true),
            Fix::replace("2", Span::new(2, 4), "y", true),
        ];
        assert!(matches!(
            apply_fixes("abcd", &fixes),
            Err(FixError::Overlap(..))
        ));
    }

    #[test]
    fn rejects_out_of_bounds() {
        let f = Fix::replace("x", Span::new(0, 10), "y", true);
        assert!(matches!(
            apply_fixes("abc", &[f]),
            Err(FixError::OutOfBounds(_))
        ));
    }
}
