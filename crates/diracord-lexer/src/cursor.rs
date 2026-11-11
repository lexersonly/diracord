//! Character cursor over the source text.

/// Walks the source by `char`, tracking the byte offset.
pub(crate) struct Cursor<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(src: &'a str) -> Self {
        Cursor { src, pos: 0 }
    }

    pub(crate) fn pos(&self) -> u32 {
        self.pos as u32
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    pub(crate) fn peek2(&self) -> Option<char> {
        let mut it = self.src[self.pos..].chars();
        it.next();
        it.next()
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    pub(crate) fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.pos += c.len_utf8();
            true
        } else {
            false
        }
    }

    pub(crate) fn eat_while(&mut self, f: impl Fn(char) -> bool) -> &'a str {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if !f(c) {
                break;
            }
            self.pos += c.len_utf8();
        }
        &self.src[start..self.pos]
    }

    pub(crate) fn slice(&self, start: u32) -> &'a str {
        &self.src[start as usize..self.pos]
    }
}

/// First character of a name.
pub(crate) fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

/// Following characters of a name.
pub(crate) fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
