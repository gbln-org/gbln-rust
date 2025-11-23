//! GBLN Type System
//!
//! Defines all type hints and their validation bounds.

use crate::value::Value;

/// Type hint for GBLN values
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeHint {
    // Signed integers
    I8,
    I16,
    I32,
    I64,

    // Unsigned integers
    U8,
    U16,
    U32,
    U64,

    // Floats
    F32,
    F64,

    // String with max character count
    Str(usize),

    // Boolean
    Bool,

    // Null
    Null,
}

impl TypeHint {
    /// Parse type hint from string (e.g., "u32", "s64", "b")
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "i8" => Ok(TypeHint::I8),
            "i16" => Ok(TypeHint::I16),
            "i32" => Ok(TypeHint::I32),
            "i64" => Ok(TypeHint::I64),
            "u8" => Ok(TypeHint::U8),
            "u16" => Ok(TypeHint::U16),
            "u32" => Ok(TypeHint::U32),
            "u64" => Ok(TypeHint::U64),
            "f32" => Ok(TypeHint::F32),
            "f64" => Ok(TypeHint::F64),
            "b" => Ok(TypeHint::Bool),
            "n" => Ok(TypeHint::Null),
            _ if s.starts_with('s') => {
                let len_str = &s[1..];
                let len = len_str
                    .parse::<usize>()
                    .map_err(|_| format!("Invalid string type: {}", s))?;
                Ok(TypeHint::Str(len))
            }
            _ => Err(format!("Unknown type hint: {}", s)),
        }
    }

    /// Get type name as string
    pub fn as_str(&self) -> String {
        match self {
            TypeHint::I8 => "i8".to_string(),
            TypeHint::I16 => "i16".to_string(),
            TypeHint::I32 => "i32".to_string(),
            TypeHint::I64 => "i64".to_string(),
            TypeHint::U8 => "u8".to_string(),
            TypeHint::U16 => "u16".to_string(),
            TypeHint::U32 => "u32".to_string(),
            TypeHint::U64 => "u64".to_string(),
            TypeHint::F32 => "f32".to_string(),
            TypeHint::F64 => "f64".to_string(),
            TypeHint::Str(n) => format!("s{}", n),
            TypeHint::Bool => "b".to_string(),
            TypeHint::Null => "n".to_string(),
        }
    }

    /// Parse value from string according to type hint with validation
    pub fn parse_value(&self, s: &str) -> Result<Value, String> {
        match self {
            TypeHint::I8 => {
                let val = s
                    .parse::<i8>()
                    .map_err(|_| format!("Cannot parse '{}' as i8", s))?;
                Ok(Value::I8(val))
            }
            TypeHint::I16 => {
                let val = s
                    .parse::<i16>()
                    .map_err(|_| format!("Cannot parse '{}' as i16", s))?;
                Ok(Value::I16(val))
            }
            TypeHint::I32 => {
                let val = s
                    .parse::<i32>()
                    .map_err(|_| format!("Cannot parse '{}' as i32", s))?;
                Ok(Value::I32(val))
            }
            TypeHint::I64 => {
                let val = s
                    .parse::<i64>()
                    .map_err(|_| format!("Cannot parse '{}' as i64", s))?;
                Ok(Value::I64(val))
            }
            TypeHint::U8 => {
                let val = s
                    .parse::<u8>()
                    .map_err(|_| format!("Cannot parse '{}' as u8", s))?;
                Ok(Value::U8(val))
            }
            TypeHint::U16 => {
                let val = s
                    .parse::<u16>()
                    .map_err(|_| format!("Cannot parse '{}' as u16", s))?;
                Ok(Value::U16(val))
            }
            TypeHint::U32 => {
                let val = s
                    .parse::<u32>()
                    .map_err(|_| format!("Cannot parse '{}' as u32", s))?;
                Ok(Value::U32(val))
            }
            TypeHint::U64 => {
                let val = s
                    .parse::<u64>()
                    .map_err(|_| format!("Cannot parse '{}' as u64", s))?;
                Ok(Value::U64(val))
            }
            TypeHint::F32 => {
                let val = s
                    .parse::<f32>()
                    .map_err(|_| format!("Cannot parse '{}' as f32", s))?;
                Ok(Value::F32(val))
            }
            TypeHint::F64 => {
                let val = s
                    .parse::<f64>()
                    .map_err(|_| format!("Cannot parse '{}' as f64", s))?;
                Ok(Value::F64(val))
            }
            TypeHint::Str(max_len) => {
                let char_count = s.chars().count();
                if char_count > *max_len {
                    return Err(format!(
                        "String too long: {} characters (max {})",
                        char_count, max_len
                    ));
                }
                Ok(Value::Str(s.to_string()))
            }
            TypeHint::Bool => match s {
                "t" | "true" => Ok(Value::Bool(true)),
                "f" | "false" => Ok(Value::Bool(false)),
                _ => Err(format!(
                    "Invalid boolean value: '{}' (expected t/f or true/false)",
                    s
                )),
            },
            TypeHint::Null => {
                if s.is_empty() || s == "null" {
                    Ok(Value::Null)
                } else {
                    Err(format!(
                        "Invalid null value: '{}' (expected empty or 'null')",
                        s
                    ))
                }
            }
        }
    }
}
