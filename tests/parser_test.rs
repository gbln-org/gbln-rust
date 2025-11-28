// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

use gbln::parser::parse;
use gbln::value::Value;
use std::collections::HashMap;

#[test]
fn t_single_value_i32() {
    let input = "<i32>(42)";
    let result = parse(input).unwrap();
    assert_eq!(result, Value::I32(42));
}

#[test]
fn t_single_value_string() {
    let input = "<s32>(Hello World)";
    let result = parse(input).unwrap();
    assert_eq!(result, Value::Str("Hello World".to_string()));
}

#[test]
fn t_single_value_bool_true() {
    let input = "<b>(t)";
    let result = parse(input).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn t_single_value_bool_false() {
    let input = "<b>(f)";
    let result = parse(input).unwrap();
    assert_eq!(result, Value::Bool(false));
}

#[test]
fn t_single_value_null() {
    let input = "<n>()";
    let result = parse(input).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn t_simple_object() {
    let input = "user{name<s32>(Alice)age<i8>(25)}";
    let result = parse(input).unwrap();

    let mut expected_user = HashMap::new();
    expected_user.insert("name".to_string(), Value::Str("Alice".to_string()));
    expected_user.insert("age".to_string(), Value::I8(25));

    let mut expected = HashMap::new();
    expected.insert("user".to_string(), Value::Object(expected_user));

    assert_eq!(result, Value::Object(expected));
}

#[test]
fn t_simple_object_with_whitespace() {
    let input = "user{\n    name<s32>(Alice)\n    age<i8>(25)\n}";
    let result = parse(input).unwrap();

    let mut expected_user = HashMap::new();
    expected_user.insert("name".to_string(), Value::Str("Alice".to_string()));
    expected_user.insert("age".to_string(), Value::I8(25));

    let mut expected = HashMap::new();
    expected.insert("user".to_string(), Value::Object(expected_user));

    assert_eq!(result, Value::Object(expected));
}

#[test]
fn t_nested_object() {
    let input = "user{profile{name<s32>(Alice)age<i8>(25)}}";
    let result = parse(input).unwrap();

    let mut profile = HashMap::new();
    profile.insert("name".to_string(), Value::Str("Alice".to_string()));
    profile.insert("age".to_string(), Value::I8(25));

    let mut user = HashMap::new();
    user.insert("profile".to_string(), Value::Object(profile));

    let mut expected = HashMap::new();
    expected.insert("user".to_string(), Value::Object(user));

    assert_eq!(result, Value::Object(expected));
}

#[test]
fn t_typed_array() {
    let input = "tags<s16>[rust python golang]";
    let result = parse(input).unwrap();

    let tags = vec![
        Value::Str("rust".to_string()),
        Value::Str("python".to_string()),
        Value::Str("golang".to_string()),
    ];

    let mut expected = HashMap::new();
    expected.insert("tags".to_string(), Value::Array(tags));

    assert_eq!(result, Value::Object(expected));
}

#[test]
fn t_typed_array_integers() {
    let input = "numbers<i32>[1 2 3 42 100]";
    let result = parse(input).unwrap();

    let numbers = vec![
        Value::I32(1),
        Value::I32(2),
        Value::I32(3),
        Value::I32(42),
        Value::I32(100),
    ];

    let mut expected = HashMap::new();
    expected.insert("numbers".to_string(), Value::Array(numbers));

    assert_eq!(result, Value::Object(expected));
}

#[test]
fn t_array_of_objects() {
    let input = "[{id<u32>(1)name<s32>(Alice)}{id<u32>(2)name<s32>(Bob)}]";
    let result = parse(input).unwrap();

    let mut obj1 = HashMap::new();
    obj1.insert("id".to_string(), Value::U32(1));
    obj1.insert("name".to_string(), Value::Str("Alice".to_string()));

    let mut obj2 = HashMap::new();
    obj2.insert("id".to_string(), Value::U32(2));
    obj2.insert("name".to_string(), Value::Str("Bob".to_string()));

    let expected = Value::Array(vec![Value::Object(obj1), Value::Object(obj2)]);

    assert_eq!(result, expected);
}

#[test]
fn t_mixed_array() {
    let input = "[<i32>(1)<s16>(hello)<b>(t)]";
    let result = parse(input).unwrap();

    let expected = Value::Array(vec![
        Value::I32(1),
        Value::Str("hello".to_string()),
        Value::Bool(true),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn t_nested_parentheses_in_value() {
    let input = "<s64>(f(x) = (x + 1) * (x - 1))";
    let result = parse(input).unwrap();
    assert_eq!(result, Value::Str("f(x) = (x + 1) * (x - 1)".to_string()));
}

#[test]
fn t_angle_brackets_in_value() {
    let input = "<s256>(<h1>Hello</h1>)";
    let result = parse(input).unwrap();
    assert_eq!(result, Value::Str("<h1>Hello</h1>".to_string()));
}

#[test]
fn t_complex_nested_structure() {
    let input = r#"
        company{
            name<s64>(Acme Corp)
            employees[
                {id<u32>(1)name<s32>(Alice)role<s16>(dev)}
                {id<u32>(2)name<s32>(Bob)role<s16>(pm)}
            ]
            active<b>(t)
        }
    "#;
    let result = parse(input).unwrap();

    let mut employee1 = HashMap::new();
    employee1.insert("id".to_string(), Value::U32(1));
    employee1.insert("name".to_string(), Value::Str("Alice".to_string()));
    employee1.insert("role".to_string(), Value::Str("dev".to_string()));

    let mut employee2 = HashMap::new();
    employee2.insert("id".to_string(), Value::U32(2));
    employee2.insert("name".to_string(), Value::Str("Bob".to_string()));
    employee2.insert("role".to_string(), Value::Str("pm".to_string()));

    let mut company_obj = HashMap::new();
    company_obj.insert("name".to_string(), Value::Str("Acme Corp".to_string()));
    company_obj.insert(
        "employees".to_string(),
        Value::Array(vec![Value::Object(employee1), Value::Object(employee2)]),
    );
    company_obj.insert("active".to_string(), Value::Bool(true));

    let mut expected = HashMap::new();
    expected.insert("company".to_string(), Value::Object(company_obj));

    assert_eq!(result, Value::Object(expected));
}

#[test]
fn t_error_duplicate_key() {
    let input = "user{name<s32>(Alice)name<s32>(Bob)}";
    let result = parse(input);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Duplicate key"));
}

#[test]
fn t_optional_type_hints_in_object() {
    // Type hints are optional (Ticket #004A) - this should succeed
    let input = "user{name(Alice)}";
    let result = parse(input);
    assert!(result.is_ok());

    match result.unwrap() {
        Value::Object(obj) => match obj.get("user") {
            Some(Value::Object(user)) => {
                assert_eq!(user.get("name"), Some(&Value::Str("Alice".to_string())));
            }
            _ => panic!("Expected user object"),
        },
        _ => panic!("Expected Object"),
    }
}

#[test]
fn t_error_unclosed_parenthesis() {
    let input = "<s32>(hello";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn t_error_invalid_type() {
    let input = "<xyz>(value)";
    let result = parse(input);
    assert!(result.is_err());
}

#[test]
fn t_error_unexpected_token_after_value() {
    let input = "<i32>(42) extra";
    let result = parse(input);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Unexpected token after value"));
}
