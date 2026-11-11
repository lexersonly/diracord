//! Byte ranges in a source file.

use std::fmt;

/// A half-open byte range `start..end` inside one source file.
///
/// Offsets are `u32` to keep AST nodes small (a source file over 4 GiB is not
/// a Diracord program). Spans never store the file: the caller knows which
/// [`SourceFile`](crate::SourceFile) the tree came from.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    /// First byte (inclusive).
    pub start: u32,
    /// One past the last byte (exclusive).
    pub end: u32,
}

impl Span {
    /// Creates a span. If `end < start` the values are swapped.
    pub fn new(start: u32, end: u32) -> Self {
        if end < start {
            Span {
                start: end,
                end: start,
            }
        } else {
            Span { start, end }
        }
    }

    /// An empty span at `offset` (used for "something is missing here").
    pub fn point(offset: u32) -> Self {
        Span {
            start: offset,
            end: offset,
        }
    }

    /// Length in bytes.
    pub fn len(self) -> u32 {
        self.end - self.start
    }

    /// `true` when the span covers no bytes.
    pub fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// The smallest span covering both `self` and `other`.
    pub fn to(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// `true` when `offset` is inside the span.
    pub fn contains(self, offset: u32) -> bool {
        self.start <= offset && offset < self.end
    }

    /// The span as a `usize` range, for slicing the source text.
    pub fn range(self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Identifies a source file inside a project (index into the project's file list).
///
/// Single-file tools can use `FileId::default()` (file 0).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct FileId(pub u32);

/// A value together with the span it came from.
#[derive(Clone, Debug, PartialEq)]
pub struct Spanned<T> {
    /// The value.
    pub node: T,
    /// Where it is in the source.
    pub span: Span,
}

impl<T> Spanned<T> {
    /// Wraps `node` with `span`.
    pub fn new(node: T, span: Span) -> Self {
        Spanned { node, span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_orders_bounds() {
        assert_eq!(Span::new(5, 2), Span::new(2, 5));
    }

    #[test]
    fn to_joins() {
        assert_eq!(Span::new(2, 4).to(Span::new(8, 9)), Span::new(2, 9));
        assert_eq!(Span::new(8, 9).to(Span::new(2, 4)), Span::new(2, 9));
    }

    #[test]
    fn point_is_empty() {
        let p = Span::point(3);
        assert!(p.is_empty());
        assert_eq!(p.len(), 0);
        assert!(!p.contains(3));
        assert!(Span::new(3, 4).contains(3));
    }
}
