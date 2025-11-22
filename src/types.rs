//! GBLN Type System
//!
//! Defines all type hints and their validation bounds.

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
                let len = len_str.parse::<usize>()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_types() {
        assert_eq!(TypeHint::from_str("i8").unwrap(), TypeHint::I8);
        assert_eq!(TypeHint::from_str("u32").unwrap(), TypeHint::U32);
        assert_eq!(TypeHint::from_str("i64").unwrap(), TypeHint::I64);
    }

    #[test]
    fn test_parse_string_types() {
        assert_eq!(TypeHint::from_str("s32").unwrap(), TypeHint::Str(32));
        assert_eq!(TypeHint::from_str("s256").unwrap(), TypeHint::Str(256));
    }

    #[test]
    fn test_parse_other_types() {
        assert_eq!(TypeHint::from_str("b").unwrap(), TypeHint::Bool);
        assert_eq!(TypeHint::from_str("n").unwrap(), TypeHint::Null);
        assert_eq!(TypeHint::from_str("f32").unwrap(), TypeHint::F32);
    }

    #[test]
    fn test_invalid_types() {
        assert!(TypeHint::from_str("unknown").is_err());
        assert!(TypeHint::from_str("s").is_err());
        assert!(TypeHint::from_str("sabc").is_err());
    }
}
