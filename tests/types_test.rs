//! Integration tests for GBLN Type System

use gbln::TypeHint;

#[test]
fn t_parse_integer_types() {
    assert_eq!(TypeHint::from_str("i8").unwrap(), TypeHint::I8);
    assert_eq!(TypeHint::from_str("i16").unwrap(), TypeHint::I16);
    assert_eq!(TypeHint::from_str("i32").unwrap(), TypeHint::I32);
    assert_eq!(TypeHint::from_str("i64").unwrap(), TypeHint::I64);
    assert_eq!(TypeHint::from_str("u8").unwrap(), TypeHint::U8);
    assert_eq!(TypeHint::from_str("u16").unwrap(), TypeHint::U16);
    assert_eq!(TypeHint::from_str("u32").unwrap(), TypeHint::U32);
    assert_eq!(TypeHint::from_str("u64").unwrap(), TypeHint::U64);
}

#[test]
fn t_parse_float_types() {
    assert_eq!(TypeHint::from_str("f32").unwrap(), TypeHint::F32);
    assert_eq!(TypeHint::from_str("f64").unwrap(), TypeHint::F64);
}

#[test]
fn t_parse_string_types() {
    assert_eq!(TypeHint::from_str("s2").unwrap(), TypeHint::Str(2));
    assert_eq!(TypeHint::from_str("s4").unwrap(), TypeHint::Str(4));
    assert_eq!(TypeHint::from_str("s8").unwrap(), TypeHint::Str(8));
    assert_eq!(TypeHint::from_str("s16").unwrap(), TypeHint::Str(16));
    assert_eq!(TypeHint::from_str("s32").unwrap(), TypeHint::Str(32));
    assert_eq!(TypeHint::from_str("s64").unwrap(), TypeHint::Str(64));
    assert_eq!(TypeHint::from_str("s128").unwrap(), TypeHint::Str(128));
    assert_eq!(TypeHint::from_str("s256").unwrap(), TypeHint::Str(256));
    assert_eq!(TypeHint::from_str("s512").unwrap(), TypeHint::Str(512));
    assert_eq!(TypeHint::from_str("s1024").unwrap(), TypeHint::Str(1024));
}

#[test]
fn t_parse_bool_type() {
    assert_eq!(TypeHint::from_str("b").unwrap(), TypeHint::Bool);
}

#[test]
fn t_parse_null_type() {
    assert_eq!(TypeHint::from_str("n").unwrap(), TypeHint::Null);
}

#[test]
fn t_invalid_type_unknown() {
    assert!(TypeHint::from_str("unknown").is_err());
    assert!(TypeHint::from_str("x32").is_err());
}

#[test]
fn t_invalid_type_incomplete() {
    assert!(TypeHint::from_str("s").is_err());
    assert!(TypeHint::from_str("i").is_err());
}

#[test]
fn t_invalid_type_non_numeric() {
    assert!(TypeHint::from_str("sabc").is_err());
    assert!(TypeHint::from_str("s12abc").is_err());
}

#[test]
fn t_type_to_string() {
    assert_eq!(TypeHint::I8.as_str(), "i8");
    assert_eq!(TypeHint::U32.as_str(), "u32");
    assert_eq!(TypeHint::F64.as_str(), "f64");
    assert_eq!(TypeHint::Str(64).as_str(), "s64");
    assert_eq!(TypeHint::Bool.as_str(), "b");
    assert_eq!(TypeHint::Null.as_str(), "n");
}

#[test]
fn t_roundtrip_conversion() {
    let types = vec!["i8", "u32", "f64", "s128", "b", "n"];

    for type_str in types {
        let hint = TypeHint::from_str(type_str).unwrap();
        assert_eq!(hint.as_str(), type_str);
    }
}
