//! Tests for typed arrays in objects (Ticket #004A Phase 2)
//!
//! Verifies that the parser correctly handles typed arrays within objects
//! using syntax: key<type>[value1 value2 value3]

use gbln::{parse, Value};

#[test]
fn test_typed_string_array_in_object() {
    let input = r#"{
        tags<s16>[rust python golang]
    }"#;

    let result = parse(input).expect("Should parse typed string array in object");

    match result {
        Value::Object(obj) => match obj.get("tags") {
            Some(Value::Array(arr)) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Value::Str("rust".to_string()));
                assert_eq!(arr[1], Value::Str("python".to_string()));
                assert_eq!(arr[2], Value::Str("golang".to_string()));
            }
            _ => panic!("Expected Array, got {:?}", obj.get("tags")),
        },
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_typed_integer_array_in_object() {
    let input = r#"{
        scores<i32>[98 87 92 76 88]
    }"#;

    let result = parse(input).expect("Should parse typed integer array in object");

    match result {
        Value::Object(obj) => match obj.get("scores") {
            Some(Value::Array(arr)) => {
                assert_eq!(arr.len(), 5);
                assert_eq!(arr[0], Value::I32(98));
                assert_eq!(arr[1], Value::I32(87));
                assert_eq!(arr[2], Value::I32(92));
                assert_eq!(arr[3], Value::I32(76));
                assert_eq!(arr[4], Value::I32(88));
            }
            _ => panic!("Expected Array"),
        },
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_typed_small_integer_array_with_validation() {
    let input = r#"{
        ages<i8>[25 30 42 18]
    }"#;

    let result = parse(input).expect("Should parse typed i8 array in object");

    match result {
        Value::Object(obj) => match obj.get("ages") {
            Some(Value::Array(arr)) => {
                assert_eq!(arr.len(), 4);
                assert_eq!(arr[0], Value::I8(25));
                assert_eq!(arr[1], Value::I8(30));
                assert_eq!(arr[2], Value::I8(42));
                assert_eq!(arr[3], Value::I8(18));
            }
            _ => panic!("Expected Array"),
        },
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_typed_array_validation_fails_on_overflow() {
    let input = r#"{
        ages<i8>[25 300 42]
    }"#;

    let result = parse(input);
    assert!(result.is_err(), "Should fail: 300 out of range for i8");
}

#[test]
fn test_typed_float_array_in_object() {
    let input = r#"{
        prices<f32>[19.99 29.99 9.99]
    }"#;

    let result = parse(input).expect("Should parse typed float array in object");

    match result {
        Value::Object(obj) => match obj.get("prices") {
            Some(Value::Array(arr)) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Value::F32(19.99));
                assert_eq!(arr[1], Value::F32(29.99));
                assert_eq!(arr[2], Value::F32(9.99));
            }
            _ => panic!("Expected Array"),
        },
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_typed_boolean_array_in_object() {
    let input = r#"{
        flags<b>[t f t t f]
    }"#;

    let result = parse(input).expect("Should parse typed boolean array in object");

    match result {
        Value::Object(obj) => match obj.get("flags") {
            Some(Value::Array(arr)) => {
                assert_eq!(arr.len(), 5);
                assert_eq!(arr[0], Value::Bool(true));
                assert_eq!(arr[1], Value::Bool(false));
                assert_eq!(arr[2], Value::Bool(true));
                assert_eq!(arr[3], Value::Bool(true));
                assert_eq!(arr[4], Value::Bool(false));
            }
            _ => panic!("Expected Array"),
        },
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_multiple_typed_arrays_in_object() {
    let input = r#"{
        tags<s16>[rust python golang]
        scores<i32>[98 87 92]
        active<b>[t f t]
    }"#;

    let result = parse(input).expect("Should parse multiple typed arrays in object");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 3);

            // Check tags array
            match obj.get("tags") {
                Some(Value::Array(arr)) => {
                    assert_eq!(arr.len(), 3);
                    assert_eq!(arr[0], Value::Str("rust".to_string()));
                }
                _ => panic!("Expected tags array"),
            }

            // Check scores array
            match obj.get("scores") {
                Some(Value::Array(arr)) => {
                    assert_eq!(arr.len(), 3);
                    assert_eq!(arr[0], Value::I32(98));
                }
                _ => panic!("Expected scores array"),
            }

            // Check active array
            match obj.get("active") {
                Some(Value::Array(arr)) => {
                    assert_eq!(arr.len(), 3);
                    assert_eq!(arr[0], Value::Bool(true));
                }
                _ => panic!("Expected active array"),
            }
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_typed_array_with_string_length_validation() {
    let input = r#"{
        codes<s4>[ABC DEFG HI]
    }"#;

    let result = parse(input).expect("Should parse typed string array with length validation");

    match result {
        Value::Object(obj) => match obj.get("codes") {
            Some(Value::Array(arr)) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Value::Str("ABC".to_string()));
                assert_eq!(arr[1], Value::Str("DEFG".to_string()));
                assert_eq!(arr[2], Value::Str("HI".to_string()));
            }
            _ => panic!("Expected Array"),
        },
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_typed_array_string_length_validation_fails() {
    let input = r#"{
        codes<s4>[ABC TOOLONG]
    }"#;

    let result = parse(input);
    assert!(
        result.is_err(),
        "Should fail: TOOLONG exceeds s4 limit (4 characters)"
    );
}

#[test]
fn test_nested_object_with_typed_arrays() {
    let input = r#"{
        user{
            name<s32>(Alice)
            tags<s16>[developer rust-fan]
            scores<i16>[98 87 92]
        }
    }"#;

    let result = parse(input).expect("Should parse nested object with typed arrays");

    match result {
        Value::Object(obj) => {
            match obj.get("user") {
                Some(Value::Object(user)) => {
                    assert_eq!(user.len(), 3);

                    // Check name field
                    assert_eq!(user.get("name"), Some(&Value::Str("Alice".to_string())));

                    // Check tags array
                    match user.get("tags") {
                        Some(Value::Array(arr)) => {
                            assert_eq!(arr.len(), 2);
                            assert_eq!(arr[0], Value::Str("developer".to_string()));
                            assert_eq!(arr[1], Value::Str("rust-fan".to_string()));
                        }
                        _ => panic!("Expected tags array"),
                    }

                    // Check scores array
                    match user.get("scores") {
                        Some(Value::Array(arr)) => {
                            assert_eq!(arr.len(), 3);
                            assert_eq!(arr[0], Value::I16(98));
                        }
                        _ => panic!("Expected scores array"),
                    }
                }
                _ => panic!("Expected user object"),
            }
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_empty_typed_array_in_object() {
    let input = r#"{
        tags<s16>[]
    }"#;

    let result = parse(input).expect("Should parse empty typed array in object");

    match result {
        Value::Object(obj) => match obj.get("tags") {
            Some(Value::Array(arr)) => {
                assert_eq!(arr.len(), 0);
            }
            _ => panic!("Expected Array"),
        },
        _ => panic!("Expected Object, got {:?}", result),
    }
}
