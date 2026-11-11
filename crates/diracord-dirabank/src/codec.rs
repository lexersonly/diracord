//! Binary encoding of keys and values (little-endian, length-prefixed).

use diracord_value::{Map, Scope, StoreKey, Value};
use std::fmt;

/// Values deeper than this are rejected when decoding (protects the stack).
const MAX_DEPTH: u32 = 64;

const NULL: u8 = 0;
const FALSE: u8 = 1;
const TRUE: u8 = 2;
const INT: u8 = 3;
const FLOAT: u8 = 4;
const TEXT: u8 = 5;
const LIST: u8 = 6;
const OBJECT: u8 = 7;

/// `scope byte | owner len (u16) | owner | name`
pub(crate) fn encode_key(k: &StoreKey) -> Vec<u8> {
    let scope = match k.scope {
        Scope::User => 1u8,
        Scope::Server => 2,
        Scope::Member => 3,
        Scope::Global => 4,
    };
    let owner = k.owner.as_bytes();
    let owner_len = owner.len().min(u16::MAX as usize);
    let mut out = Vec::with_capacity(3 + owner_len + k.name.len());
    out.push(scope);
    out.extend_from_slice(&(owner_len as u16).to_le_bytes());
    out.extend_from_slice(&owner[..owner_len]);
    out.extend_from_slice(k.name.as_bytes());
    out
}

pub(crate) fn encode_value(v: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    write(v, &mut out);
    out
}

fn write_len(n: usize, out: &mut Vec<u8>) {
    out.extend_from_slice(&(n as u32).to_le_bytes());
}

fn write(v: &Value, out: &mut Vec<u8>) {
    match v {
        Value::Null => out.push(NULL),
        Value::Bool(false) => out.push(FALSE),
        Value::Bool(true) => out.push(TRUE),
        Value::Int(i) => {
            out.push(INT);
            out.extend_from_slice(&i.to_le_bytes());
        }
        Value::Float(f) => {
            out.push(FLOAT);
            out.extend_from_slice(&f.to_le_bytes());
        }
        Value::Text(s) => {
            out.push(TEXT);
            write_len(s.len(), out);
            out.extend_from_slice(s.as_bytes());
        }
        Value::List(l) => {
            out.push(LIST);
            write_len(l.len(), out);
            l.iter().for_each(|x| write(x, out));
        }
        Value::Object(m) => {
            out.push(OBJECT);
            write_len(m.len(), out);
            for (k, x) in m {
                write_len(k.len(), out);
                out.extend_from_slice(k.as_bytes());
                write(x, out);
            }
        }
    }
}

/// A value that could not be decoded.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DecodeError(&'static str);

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "valor salvo corrompido ({})", self.0)
    }
}

pub(crate) fn decode_value(bytes: &[u8]) -> Result<Value, DecodeError> {
    let mut r = Reader { b: bytes, pos: 0 };
    let v = r.value(0)?;
    if r.pos != bytes.len() {
        return Err(DecodeError("bytes sobrando"));
    }
    Ok(v)
}

struct Reader<'a> {
    b: &'a [u8],
    pos: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], DecodeError> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|&e| e <= self.b.len())
            .ok_or(DecodeError("fim inesperado"))?;
        let s = &self.b[self.pos..end];
        self.pos = end;
        Ok(s)
    }

    fn u32(&mut self) -> Result<usize, DecodeError> {
        let s = self.take(4)?;
        Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]) as usize)
    }

    fn eight(&mut self) -> Result<[u8; 8], DecodeError> {
        let s = self.take(8)?;
        let mut a = [0u8; 8];
        a.copy_from_slice(s);
        Ok(a)
    }

    fn text(&mut self) -> Result<String, DecodeError> {
        let n = self.u32()?;
        let s = self.take(n)?;
        String::from_utf8(s.to_vec()).map_err(|_| DecodeError("texto inválido"))
    }

    fn value(&mut self, depth: u32) -> Result<Value, DecodeError> {
        if depth > MAX_DEPTH {
            return Err(DecodeError("aninhamento demais"));
        }
        let tag = self.take(1)?[0];
        Ok(match tag {
            NULL => Value::Null,
            FALSE => Value::Bool(false),
            TRUE => Value::Bool(true),
            INT => Value::Int(i64::from_le_bytes(self.eight()?)),
            FLOAT => Value::Float(f64::from_le_bytes(self.eight()?)),
            TEXT => Value::Text(self.text()?),
            LIST => {
                let n = self.u32()?;
                // Each item needs at least one byte: guards against huge bogus counts.
                if n > self.b.len() - self.pos {
                    return Err(DecodeError("tamanho de lista inválido"));
                }
                let mut l = Vec::with_capacity(n);
                for _ in 0..n {
                    l.push(self.value(depth + 1)?);
                }
                Value::List(l)
            }
            OBJECT => {
                let n = self.u32()?;
                if n > self.b.len() - self.pos {
                    return Err(DecodeError("tamanho de objeto inválido"));
                }
                let mut m = Map::new();
                for _ in 0..n {
                    let k = self.text()?;
                    let v = self.value(depth + 1)?;
                    m.insert(k, v);
                }
                Value::Object(m)
            }
            _ => return Err(DecodeError("tipo desconhecido")),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let v = Value::object([
            (
                "a",
                Value::List(vec![
                    Value::Null,
                    true.into(),
                    false.into(),
                    7i64.into(),
                    1.5.into(),
                ]),
            ),
            ("ç", Value::from("ação")),
            ("o", Value::object([("x", Value::Int(-1))])),
        ]);
        assert_eq!(decode_value(&encode_value(&v)), Ok(v));
    }

    #[test]
    fn rejects_garbage() {
        assert!(decode_value(&[9]).is_err());
        assert!(decode_value(&[TEXT, 255, 255, 255, 255]).is_err());
        assert!(decode_value(&[LIST, 255, 255, 255, 255]).is_err());
        assert!(decode_value(&[NULL, NULL]).is_err());
    }

    #[test]
    fn keys_differ_by_scope_and_owner() {
        let a = encode_key(&StoreKey::new(Scope::User, "1", "x"));
        let b = encode_key(&StoreKey::new(Scope::Server, "1", "x"));
        let c = encode_key(&StoreKey::new(Scope::User, "1x", ""));
        assert_ne!(a, b);
        assert_ne!(a, c);
    }
}
