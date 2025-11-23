//! GBLN Serializer
//!
//! Converts Value to GBLN string format (compact and pretty).

use crate::value::Value;

/// Serialise a Value to compact GBLN string
pub fn to_string(value: &Value) -> String {
    serialize_value(value, false, 0)
}

/// Serialise a Value to formatted GBLN string with indentation
pub fn to_string_pretty(value: &Value) -> String {
    serialize_value(value, true, 0)
}

/// Internal serialisation with formatting control
fn serialize_value(value: &Value, pretty: bool, indent: usize) -> String {
    match value {
        Value::I8(n) => format!("<i8>({})", n),
        Value::I16(n) => format!("<i16>({})", n),
        Value::I32(n) => format!("<i32>({})", n),
        Value::I64(n) => format!("<i64>({})", n),
        Value::U8(n) => format!("<u8>({})", n),
        Value::U16(n) => format!("<u16>({})", n),
        Value::U32(n) => format!("<u32>({})", n),
        Value::U64(n) => format!("<u64>({})", n),
        Value::F32(n) => format!("<f32>({})", n),
        Value::F64(n) => format!("<f64>({})", n),
        Value::Str(s) => {
            // Infer string type from length
            let len = s.chars().count();
            let type_hint = infer_string_type(len);
            format!("<{}>({})", type_hint, s)
        }
        Value::Bool(b) => {
            let val = if *b { "t" } else { "f" };
            format!("<b>({})", val)
        }
        Value::Null => "<n>()".to_string(),
        Value::Object(map) => serialize_object(map, pretty, indent),
        Value::Array(arr) => serialize_array(arr, pretty, indent),
    }
}

/// Serialise an object
fn serialize_object(
    map: &std::collections::HashMap<String, Value>,
    pretty: bool,
    indent: usize,
) -> String {
    if map.is_empty() {
        return "{}".to_string();
    }

    let mut result = String::new();

    // Collect and sort keys for deterministic output
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();

    for (i, key) in keys.iter().enumerate() {
        let value = &map[*key];

        if pretty && i > 0 {
            result.push('\n');
            result.push_str(&"    ".repeat(indent + 1));
        }

        // Determine how to serialize based on value type
        match value {
            Value::Object(inner_map) => {
                // Nested object: key{...}
                result.push_str(key);
                result.push_str(&serialize_object(inner_map, pretty, indent + 1));
            }
            Value::Array(arr) => {
                // Array field: key[...] (NO type hint in objects)
                result.push_str(key);
                result.push_str(&serialize_array_in_object(arr, pretty, indent + 1));
            }
            _ => {
                // Typed single value: key<type>(value)
                result.push_str(key);
                result.push_str(&serialize_typed_value(value, pretty, indent));
            }
        }
    }

    if pretty {
        format!(
            "{{\n{}{}\n{}}}",
            "    ".repeat(indent + 1),
            result,
            "    ".repeat(indent)
        )
    } else {
        format!("{{{}}}", result)
    }
}

/// Serialise an array in object context (NO type hint, always mixed/object array format)
fn serialize_array_in_object(arr: &[Value], pretty: bool, indent: usize) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    }

    let mut result = String::new();

    for (i, value) in arr.iter().enumerate() {
        if i > 0 {
            if pretty {
                result.push('\n');
                result.push_str(&"    ".repeat(indent + 1));
            }
        } else if pretty {
            result.push('\n');
            result.push_str(&"    ".repeat(indent + 1));
        }

        result.push_str(&serialize_value(value, pretty, indent + 1));
    }

    if pretty {
        format!(
            "[\n{}{}\n{}]",
            "    ".repeat(indent + 1),
            result,
            "    ".repeat(indent)
        )
    } else {
        format!("[{}]", result)
    }
}

/// Serialise an array (top-level, can use typed array format)
fn serialize_array(arr: &[Value], pretty: bool, indent: usize) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    }

    // Check if it's a homogeneous typed array
    if let Some(first) = arr.first() {
        if is_simple_type(first) && arr.iter().all(|v| same_type(v, first)) {
            // Typed array: type[val1 val2 val3]
            return serialize_typed_array(arr, pretty);
        }
    }

    // Mixed array or object array
    let mut result = String::new();

    for (i, value) in arr.iter().enumerate() {
        if i > 0 {
            if pretty {
                result.push('\n');
                result.push_str(&"    ".repeat(indent + 1));
            }
        } else if pretty {
            result.push('\n');
            result.push_str(&"    ".repeat(indent + 1));
        }

        result.push_str(&serialize_value(value, pretty, indent + 1));
    }

    if pretty {
        format!(
            "[\n{}{}\n{}]",
            "    ".repeat(indent + 1),
            result,
            "    ".repeat(indent)
        )
    } else {
        format!("[{}]", result)
    }
}

/// Serialise a typed array (homogeneous simple types)
fn serialize_typed_array(arr: &[Value], _pretty: bool) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    }

    // For strings, find the longest to determine type hint
    let type_hint = if matches!(arr[0], Value::Str(_)) {
        let max_len = arr
            .iter()
            .filter_map(|v| {
                if let Value::Str(s) = v {
                    Some(s.chars().count())
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);
        infer_string_type(max_len)
    } else {
        get_type_hint(&arr[0])
    };

    let mut values = Vec::new();

    for value in arr {
        let val_str = match value {
            Value::I8(n) => n.to_string(),
            Value::I16(n) => n.to_string(),
            Value::I32(n) => n.to_string(),
            Value::I64(n) => n.to_string(),
            Value::U8(n) => n.to_string(),
            Value::U16(n) => n.to_string(),
            Value::U32(n) => n.to_string(),
            Value::U64(n) => n.to_string(),
            Value::F32(n) => n.to_string(),
            Value::F64(n) => n.to_string(),
            Value::Str(s) => s.clone(),
            Value::Bool(b) => if *b { "t" } else { "f" }.to_string(),
            Value::Null => "null".to_string(),
            _ => continue,
        };
        values.push(val_str);
    }

    // Typed arrays are always compact (no difference between pretty and compact)
    format!("<{}>[{}]", type_hint, values.join(" "))
}

/// Serialise a typed single value (without outer type hint, used in objects)
fn serialize_typed_value(value: &Value, _pretty: bool, _indent: usize) -> String {
    match value {
        Value::I8(n) => format!("<i8>({})", n),
        Value::I16(n) => format!("<i16>({})", n),
        Value::I32(n) => format!("<i32>({})", n),
        Value::I64(n) => format!("<i64>({})", n),
        Value::U8(n) => format!("<u8>({})", n),
        Value::U16(n) => format!("<u16>({})", n),
        Value::U32(n) => format!("<u32>({})", n),
        Value::U64(n) => format!("<u64>({})", n),
        Value::F32(n) => format!("<f32>({})", n),
        Value::F64(n) => format!("<f64>({})", n),
        Value::Str(s) => {
            let len = s.chars().count();
            let type_hint = infer_string_type(len);
            format!("<{}>({})", type_hint, s)
        }
        Value::Bool(b) => {
            let val = if *b { "t" } else { "f" };
            format!("<b>({})", val)
        }
        Value::Null => "<n>()".to_string(),
        _ => String::new(),
    }
}

/// Check if a value is a simple type (not object or array)
fn is_simple_type(value: &Value) -> bool {
    !matches!(value, Value::Object(_) | Value::Array(_))
}

/// Check if two values have the same type
fn same_type(a: &Value, b: &Value) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Get the type hint string for a value
fn get_type_hint(value: &Value) -> String {
    match value {
        Value::I8(_) => "i8".to_string(),
        Value::I16(_) => "i16".to_string(),
        Value::I32(_) => "i32".to_string(),
        Value::I64(_) => "i64".to_string(),
        Value::U8(_) => "u8".to_string(),
        Value::U16(_) => "u16".to_string(),
        Value::U32(_) => "u32".to_string(),
        Value::U64(_) => "u64".to_string(),
        Value::F32(_) => "f32".to_string(),
        Value::F64(_) => "f64".to_string(),
        Value::Str(s) => {
            let len = s.chars().count();
            infer_string_type(len)
        }
        Value::Bool(_) => "b".to_string(),
        Value::Null => "n".to_string(),
        _ => "unknown".to_string(),
    }
}

/// Infer appropriate string type from length
fn infer_string_type(len: usize) -> String {
    match len {
        0..=2 => "s2".to_string(),
        3..=4 => "s4".to_string(),
        5..=8 => "s8".to_string(),
        9..=16 => "s16".to_string(),
        17..=32 => "s32".to_string(),
        33..=64 => "s64".to_string(),
        65..=128 => "s128".to_string(),
        129..=256 => "s256".to_string(),
        257..=512 => "s512".to_string(),
        _ => "s1024".to_string(),
    }
}
