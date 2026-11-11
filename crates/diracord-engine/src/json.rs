//! Minimal JSON ↔ [`Value`], so the engine needs no serde.
//!
//! Numbers without `.`/`e` that fit in `i64` become `Int`, others `Float`.
//! Non-finite floats are written as `null`.

use diracord_value::{Map, Value};
use std::fmt::Write;

/// Maximum nesting accepted when parsing.
const MAX_DEPTH: u32 = 128;

/// Parses one JSON document.
pub fn parse(text: &str) -> Result<Value, String> {
    let mut p = P {
        s: text.as_bytes(),
        i: 0,
    };
    p.ws();
    let v = p.value(0)?;
    p.ws();
    if p.i != p.s.len() {
        return Err(format!("texto sobrando na posição {}", p.i));
    }
    Ok(v)
}

/// Writes a value as compact JSON.
pub fn to_string(v: &Value) -> String {
    let mut out = String::new();
    write(v, &mut out);
    out
}

fn write(v: &Value, out: &mut String) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Int(i) => {
            let _ = write!(out, "{i}");
        }
        Value::Float(f) if f.is_finite() => {
            let _ = write!(out, "{f:?}");
        }
        Value::Float(_) => out.push_str("null"),
        Value::Text(s) => write_str(s, out),
        Value::List(l) => {
            out.push('[');
            for (i, x) in l.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write(x, out);
            }
            out.push(']');
        }
        Value::Object(m) => {
            out.push('{');
            for (i, (k, x)) in m.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_str(k, out);
                out.push(':');
                write(x, out);
            }
            out.push('}');
        }
    }
}

fn write_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

struct P<'a> {
    s: &'a [u8],
    i: usize,
}

impl P<'_> {
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }

    fn eat(&mut self, b: u8) -> bool {
        if self.s.get(self.i) == Some(&b) {
            self.i += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, word: &str, v: Value) -> Result<Value, String> {
        if self.s[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            Err(format!("valor inválido na posição {}", self.i))
        }
    }

    fn value(&mut self, depth: u32) -> Result<Value, String> {
        if depth > MAX_DEPTH {
            return Err("JSON aninhado demais".into());
        }
        match self.s.get(self.i) {
            None => Err("JSON terminou cedo".into()),
            Some(b'n') => self.expect("null", Value::Null),
            Some(b't') => self.expect("true", Value::Bool(true)),
            Some(b'f') => self.expect("false", Value::Bool(false)),
            Some(b'"') => self.string().map(Value::Text),
            Some(b'[') => {
                self.i += 1;
                let mut l = Vec::new();
                self.ws();
                if self.eat(b']') {
                    return Ok(Value::List(l));
                }
                loop {
                    self.ws();
                    l.push(self.value(depth + 1)?);
                    self.ws();
                    if self.eat(b']') {
                        return Ok(Value::List(l));
                    }
                    if !self.eat(b',') {
                        return Err(format!("esperava , ou ] na posição {}", self.i));
                    }
                }
            }
            Some(b'{') => {
                self.i += 1;
                let mut m = Map::new();
                self.ws();
                if self.eat(b'}') {
                    return Ok(Value::Object(m));
                }
                loop {
                    self.ws();
                    if self.s.get(self.i) != Some(&b'"') {
                        return Err(format!("esperava chave na posição {}", self.i));
                    }
                    let k = self.string()?;
                    self.ws();
                    if !self.eat(b':') {
                        return Err(format!("esperava : na posição {}", self.i));
                    }
                    self.ws();
                    let v = self.value(depth + 1)?;
                    m.insert(k, v);
                    self.ws();
                    if self.eat(b'}') {
                        return Ok(Value::Object(m));
                    }
                    if !self.eat(b',') {
                        return Err(format!("esperava , ou }} na posição {}", self.i));
                    }
                }
            }
            Some(_) => self.number(),
        }
    }

    fn number(&mut self) -> Result<Value, String> {
        let start = self.i;
        while self.i < self.s.len()
            && matches!(
                self.s[self.i],
                b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9'
            )
        {
            self.i += 1;
        }
        let t = std::str::from_utf8(&self.s[start..self.i]).map_err(|e| e.to_string())?;
        if t.is_empty() {
            return Err(format!("caractere inesperado na posição {start}"));
        }
        if !t.contains(['.', 'e', 'E']) {
            if let Ok(i) = t.parse::<i64>() {
                return Ok(Value::Int(i));
            }
        }
        t.parse::<f64>()
            .map(Value::Float)
            .map_err(|_| format!("número inválido: {t}"))
    }

    fn hex4(&mut self) -> Result<u32, String> {
        let h = self
            .s
            .get(self.i..self.i + 4)
            .ok_or("escape \\u incompleto")?;
        self.i += 4;
        u32::from_str_radix(std::str::from_utf8(h).map_err(|e| e.to_string())?, 16)
            .map_err(|e| e.to_string())
    }

    fn string(&mut self) -> Result<String, String> {
        self.i += 1; // "
        let mut out = String::new();
        loop {
            let start = self.i;
            while self.i < self.s.len() && self.s[self.i] != b'"' && self.s[self.i] != b'\\' {
                self.i += 1;
            }
            out.push_str(std::str::from_utf8(&self.s[start..self.i]).map_err(|e| e.to_string())?);
            match self.s.get(self.i) {
                None => return Err("texto sem fechar".into()),
                Some(b'"') => {
                    self.i += 1;
                    return Ok(out);
                }
                _ => {
                    self.i += 1;
                    let c = *self.s.get(self.i).ok_or("escape incompleto")?;
                    self.i += 1;
                    match c {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let mut cp = self.hex4()?;
                            if (0xD800..0xDC00).contains(&cp)
                                && self.s[self.i..].starts_with(b"\\u")
                            {
                                self.i += 2;
                                let lo = self.hex4()?;
                                cp = 0x10000
                                    + ((cp - 0xD800) << 10)
                                    + (lo.wrapping_sub(0xDC00) & 0x3FF);
                            }
                            out.push(char::from_u32(cp).unwrap_or('\u{FFFD}'));
                        }
                        _ => return Err("escape inválido".into()),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let src = r#"{"a":[1,2.5,true,false,null],"b":"x\"y\nç","c":{}}"#;
        let v = parse(src).unwrap();
        assert_eq!(v.field("b"), Some(&Value::from("x\"y\nç")));
        assert_eq!(parse(&to_string(&v)).unwrap(), v);
    }

    #[test]
    fn numbers() {
        assert_eq!(
            parse("123456789012345678").unwrap(),
            Value::Int(123456789012345678)
        );
        assert_eq!(parse("-1.5e2").unwrap(), Value::Float(-150.0));
        assert_eq!(to_string(&Value::Float(2.0)), "2.0");
    }

    #[test]
    fn surrogate_pairs() {
        assert_eq!(parse(r#""👍""#).unwrap(), Value::from("👍"));
    }

    #[test]
    fn errors() {
        assert!(parse("{").is_err());
        assert!(parse("[1,]").is_err());
        assert!(parse("\"abc").is_err());
        assert!(parse("1 2").is_err());
        assert!(parse(&"[".repeat(1000)).is_err());
    }
}
