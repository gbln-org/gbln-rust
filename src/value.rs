//! GBLN Value Representation
//!
//! The in-memory representation of parsed GBLN data.

use std::collections::HashMap;

/// A GBLN value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Signed integers
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    // Unsigned integers
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    // Floats
    F32(f32),
    F64(f64),

    // String
    Str(String),

    // Boolean
    Bool(bool),

    // Null
    Null,

    // Object (preserves insertion order)
    Object(HashMap<String, Value>),

    // Array
    Array(Vec<Value>),
}

impl Value {
    // Type checking methods

    pub fn is_i8(&self) -> bool {
        matches!(self, Value::I8(_))
    }

    pub fn is_u32(&self) -> bool {
        matches!(self, Value::U32(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::Str(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    // Value accessors

    pub fn as_u32(&self) -> Option<u32> {
        if let Value::U32(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        if let Value::I32(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        if let Value::Object(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }
}

// Index access for objects (syntactic sugar)
impl std::ops::Index<&str> for Value {
    type Output = Value;

    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Value::Object(map) => map.get(key).unwrap_or(&Value::Null),
            _ => &Value::Null,
        }
    }
}
