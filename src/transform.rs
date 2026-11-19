use base64::prelude::*;
use std::fmt;

pub enum Value {
    Bytes(Vec<u8>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bytes(b) => write!(f, "{}", String::from_utf8_lossy(b)),
        }
    }
}

fn hex_decode(v: &Value) -> Option<Value> {
    match v {
        Value::Bytes(b) => {
            let s = std::str::from_utf8(b).ok()?;
            hex::decode(s.trim()).ok().map(Value::Bytes)
        }
    }
}

fn base64_decode(v: &Value) -> Option<Value> {
    match v {
        Value::Bytes(b) => {
            let s = std::str::from_utf8(b).ok()?;
            BASE64_STANDARD.decode(s.trim()).ok().map(Value::Bytes)
        }
    }
}

pub fn try_all(v: &Value) -> Vec<(&'static str, Value)> {
    let mut out = Vec::new();

    if let Some(r) = hex_decode(v) {
        out.push(("hex", r));
    }
    if let Some(r) = base64_decode(v) {
        out.push(("base64", r));
    }

    out
}
