//! Source files and line/column lookup.

/// A 1-based line and column. Columns count characters, not bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineCol {
    /// Line number, starting at 1.
    pub line: u32,
    /// Column number, starting at 1 (in characters).
    pub col: u32,
}

/// A named source text with a precomputed line index.
#[derive(Clone, Debug)]
pub struct SourceFile {
    name: String,
    text: String,
    line_starts: Vec<u32>,
}

impl SourceFile {
    /// Creates a source file. `name` is shown in error messages (usually a path).
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        let text = text.into();
        let mut line_starts = vec![0u32];
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i as u32 + 1);
            }
        }
        SourceFile {
            name: name.into(),
            text,
            line_starts,
        }
    }

    /// The file name given at creation.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The full text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Number of lines (an empty file has one empty line).
    pub fn line_count(&self) -> u32 {
        self.line_starts.len() as u32
    }

    /// Converts a byte offset to a line/column. Offsets past the end clamp to the end.
    pub fn line_col(&self, offset: u32) -> LineCol {
        let offset = offset.min(self.text.len() as u32);
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let start = self.line_starts[line_idx] as usize;
        let mut end = offset as usize;
        while !self.text.is_char_boundary(end) {
            end -= 1;
        }
        let col = self.text[start..end].chars().count() as u32 + 1;
        LineCol {
            line: line_idx as u32 + 1,
            col,
        }
    }

    /// Text of a 1-based line, without the line break. Out of range gives `""`.
    pub fn line_text(&self, line: u32) -> &str {
        if line == 0 || line > self.line_count() {
            return "";
        }
        let start = self.line_starts[line as usize - 1] as usize;
        let end = self
            .line_starts
            .get(line as usize)
            .map(|&e| e as usize)
            .unwrap_or(self.text.len());
        self.text[start..end].trim_end_matches(['\n', '\r'])
    }

    /// Byte offset where a 1-based line starts.
    pub fn line_start(&self, line: u32) -> u32 {
        let idx = (line.max(1) - 1) as usize;
        self.line_starts
            .get(idx)
            .copied()
            .unwrap_or(self.text.len() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_basic() {
        let f = SourceFile::new("a", "ab\ncd\n");
        assert_eq!(f.line_col(0), LineCol { line: 1, col: 1 });
        assert_eq!(f.line_col(1), LineCol { line: 1, col: 2 });
        assert_eq!(f.line_col(3), LineCol { line: 2, col: 1 });
        assert_eq!(f.line_col(6), LineCol { line: 3, col: 1 });
        assert_eq!(f.line_col(99), LineCol { line: 3, col: 1 });
    }

    #[test]
    fn columns_count_chars() {
        let f = SourceFile::new("a", "ação x");
        // "ação " is 5 chars but 7 bytes.
        assert_eq!(f.line_col(7), LineCol { line: 1, col: 6 });
    }

    #[test]
    fn line_text_strips_crlf() {
        let f = SourceFile::new("a", "um\r\ndois");
        assert_eq!(f.line_text(1), "um");
        assert_eq!(f.line_text(2), "dois");
        assert_eq!(f.line_text(3), "");
    }
}
