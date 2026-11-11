//! Variables, in nested block scopes, with memory accounting.

use diracord_value::Value;

struct Slot {
    name: String,
    value: Value,
    size: usize,
}

/// Block-scoped variables. The first frame holds the automatic variables.
pub(crate) struct Env {
    frames: Vec<Vec<Slot>>,
    bytes: usize,
}

impl Env {
    pub(crate) fn new() -> Self {
        Env {
            frames: vec![Vec::new()],
            bytes: 0,
        }
    }

    pub(crate) fn bytes(&self) -> usize {
        self.bytes
    }

    pub(crate) fn push(&mut self) {
        self.frames.push(Vec::new());
    }

    pub(crate) fn pop(&mut self) {
        if let Some(f) = self.frames.pop() {
            self.bytes -= f.iter().map(|s| s.size).sum::<usize>();
        }
    }

    /// `local name = value` in the innermost frame (shadows outer ones).
    pub(crate) fn declare(&mut self, name: &str, value: Value) {
        let size = value.approx_size() + name.len();
        self.bytes += size;
        let frame = self.frames.last_mut().expect("at least one frame");
        if let Some(slot) = frame.iter_mut().find(|s| s.name == name) {
            self.bytes -= slot.size;
            slot.value = value;
            slot.size = size;
        } else {
            frame.push(Slot {
                name: name.to_string(),
                value,
                size,
            });
        }
    }

    fn find(&self, name: &str) -> Option<(usize, usize)> {
        for (fi, f) in self.frames.iter().enumerate().rev() {
            if let Some(si) = f.iter().position(|s| s.name == name) {
                return Some((fi, si));
            }
        }
        None
    }

    pub(crate) fn get(&self, name: &str) -> Option<&Value> {
        self.find(name).map(|(f, s)| &self.frames[f][s].value)
    }

    pub(crate) fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.find(name).map(|(f, s)| &mut self.frames[f][s].value)
    }

    /// Recomputes the size of `name` after an in-place change.
    pub(crate) fn resize(&mut self, name: &str) {
        if let Some((f, s)) = self.find(name) {
            let slot = &mut self.frames[f][s];
            let size = slot.value.approx_size() + slot.name.len();
            self.bytes = self.bytes - slot.size + size;
            slot.size = size;
        }
    }

    /// All visible names (for "você quis dizer").
    pub(crate) fn names(&self) -> Vec<&str> {
        self.frames
            .iter()
            .flatten()
            .map(|s| s.name.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scopes_and_bytes() {
        let mut e = Env::new();
        e.declare("x", Value::Int(1));
        let base = e.bytes();
        e.push();
        e.declare("x", Value::from("a".repeat(100)));
        assert_eq!(e.get("x"), Some(&Value::from("a".repeat(100))));
        assert!(e.bytes() > base + 100);
        e.pop();
        assert_eq!(e.get("x"), Some(&Value::Int(1)));
        assert_eq!(e.bytes(), base);
        *e.get_mut("x").unwrap() = Value::from("b".repeat(50));
        e.resize("x");
        assert!(e.bytes() > base + 40);
    }
}
