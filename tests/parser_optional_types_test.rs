//! Tests for optional type hints (Ticket #004A Phase 1)
//!
//! Verifies that the parser correctly handles values WITHOUT type hints,
//! using type inference to determine appropriate types.

use gbln::{parse, Value};

#[test]
fn test_single_untyped_string() {
    let input = "name(Alice)";
    let result = parse(input).expect("Should parse untyped string");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj.get("name"), Some(&Value::Str("Alice".to_string())));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_single_untyped_integer() {
    let input = "age(25)";
    let result = parse(input).expect("Should parse untyped integer");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj.get("age"), Some(&Value::I64(25)));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_single_untyped_negative_integer() {
    let input = "temperature(-15)";
    let result = parse(input).expect("Should parse untyped negative integer");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj.get("temperature"), Some(&Value::I64(-15)));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_single_untyped_float() {
    let input = "price(19.99)";
    let result = parse(input).expect("Should parse untyped float");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj.get("price"), Some(&Value::F64(19.99)));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_single_untyped_boolean_true() {
    let input = "active(true)";
    let result = parse(input).expect("Should parse untyped boolean true");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj.get("active"), Some(&Value::Bool(true)));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_single_untyped_boolean_false_short() {
    let input = "disabled(f)";
    let result = parse(input).expect("Should parse untyped boolean f");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj.get("disabled"), Some(&Value::Bool(false)));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_single_untyped_null_empty() {
    let input = "optional()";
    let result = parse(input).expect("Should parse untyped empty as null");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj.get("optional"), Some(&Value::Null));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_single_untyped_null_explicit() {
    let input = "optional(null)";
    let result = parse(input).expect("Should parse untyped null");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            assert_eq!(obj.get("optional"), Some(&Value::Null));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_object_with_multiple_untyped_fields() {
    let input = r#"{
        name(Alice)
        age(25)
        active(true)
        score(98.5)
    }"#;

    let result = parse(input).expect("Should parse object with untyped fields");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 4);
            assert_eq!(obj.get("name"), Some(&Value::Str("Alice".to_string())));
            assert_eq!(obj.get("age"), Some(&Value::I64(25)));
            assert_eq!(obj.get("active"), Some(&Value::Bool(true)));
            assert_eq!(obj.get("score"), Some(&Value::F64(98.5)));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_object_mixed_typed_and_untyped() {
    let input = r#"{
        id<u32>(12345)
        name(Alice)
        age<i8>(25)
        active(true)
    }"#;

    let result = parse(input).expect("Should parse object with mixed typed/untyped fields");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 4);
            assert_eq!(obj.get("id"), Some(&Value::U32(12345)));
            assert_eq!(obj.get("name"), Some(&Value::Str("Alice".to_string())));
            assert_eq!(obj.get("age"), Some(&Value::I8(25)));
            assert_eq!(obj.get("active"), Some(&Value::Bool(true)));
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_array_untyped_integers() {
    let input = "numbers[1 2 3 4 5]";
    let result = parse(input).expect("Should parse untyped integer array");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            match obj.get("numbers") {
                Some(Value::Array(arr)) => {
                    assert_eq!(arr.len(), 5);
                    assert_eq!(arr[0], Value::I64(1));
                    assert_eq!(arr[1], Value::I64(2));
                    assert_eq!(arr[2], Value::I64(3));
                    assert_eq!(arr[3], Value::I64(4));
                    assert_eq!(arr[4], Value::I64(5));
                }
                _ => panic!("Expected Array"),
            }
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_array_untyped_strings() {
    let input = "tags[rust python golang]";
    let result = parse(input).expect("Should parse untyped string array");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            match obj.get("tags") {
                Some(Value::Array(arr)) => {
                    assert_eq!(arr.len(), 3);
                    assert_eq!(arr[0], Value::Str("rust".to_string()));
                    assert_eq!(arr[1], Value::Str("python".to_string()));
                    assert_eq!(arr[2], Value::Str("golang".to_string()));
                }
                _ => panic!("Expected Array"),
            }
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_array_untyped_mixed_types() {
    let input = "mixed[42 hello 3.14 true]";
    let result = parse(input).expect("Should parse untyped mixed array");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.len(), 1);
            match obj.get("mixed") {
                Some(Value::Array(arr)) => {
                    assert_eq!(arr.len(), 4);
                    assert_eq!(arr[0], Value::I64(42));
                    assert_eq!(arr[1], Value::Str("hello".to_string()));
                    assert_eq!(arr[2], Value::F64(3.14));
                    assert_eq!(arr[3], Value::Bool(true));
                }
                _ => panic!("Expected Array"),
            }
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_nested_object_untyped() {
    let input = r#"{
        user{
            name(Alice)
            age(25)
            settings{
                theme(dark)
                notifications(true)
            }
        }
    }"#;

    let result = parse(input).expect("Should parse nested objects with untyped fields");

    match result {
        Value::Object(obj) => match obj.get("user") {
            Some(Value::Object(user)) => {
                assert_eq!(user.get("name"), Some(&Value::Str("Alice".to_string())));
                assert_eq!(user.get("age"), Some(&Value::I64(25)));

                match user.get("settings") {
                    Some(Value::Object(settings)) => {
                        assert_eq!(settings.get("theme"), Some(&Value::Str("dark".to_string())));
                        assert_eq!(settings.get("notifications"), Some(&Value::Bool(true)));
                    }
                    _ => panic!("Expected nested settings object"),
                }
            }
            _ => panic!("Expected user object"),
        },
        _ => panic!("Expected Object, got {:?}", result),
    }
}

#[test]
fn test_type_inference_with_special_strings() {
    // Strings that could be mistaken for other types but aren't
    let input = r#"{
        not_bool(truthy)
        not_null(nullable)
        not_number(123abc)
    }"#;

    let result = parse(input).expect("Should parse as strings");

    match result {
        Value::Object(obj) => {
            assert_eq!(obj.get("not_bool"), Some(&Value::Str("truthy".to_string())));
            assert_eq!(
                obj.get("not_null"),
                Some(&Value::Str("nullable".to_string()))
            );
            assert_eq!(
                obj.get("not_number"),
                Some(&Value::Str("123abc".to_string()))
            );
        }
        _ => panic!("Expected Object, got {:?}", result),
    }
}
