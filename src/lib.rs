//! GBLN (Goblin Bounded Lean Notation)
//!
//! A type-safe, LLM-native data serialisation format with parse-time validation.

pub mod error;
pub mod types;
pub mod value;

pub use error::{Error, ErrorKind};
pub use types::TypeHint;
pub use value::Value;

/// Parse a GBLN string into a Value
pub fn parse(_input: &str) -> Result<Value, Error> {
    todo!("Parser implementation coming next")
}

/// Serialise a Value to compact GBLN string
pub fn to_string(_value: &Value) -> String {
    todo!("Serialiser implementation coming next")
}

/// Serialise a Value to formatted GBLN string (with indentation)
pub fn to_string_pretty(_value: &Value) -> String {
    todo!("Serialiser implementation coming next")
}
