//! GBLN (Goblin Bounded Lean Notation)
//!
//! A type-safe, LLM-native data serialisation format with parse-time validation.

pub mod error;
pub mod lexer;
pub mod parser;
pub mod serializer;
pub mod types;
pub mod value;

pub use error::{Error, ErrorKind};
pub use lexer::{Lexer, Token};
pub use parser::parse;
pub use serializer::{to_string, to_string_pretty};
pub use types::TypeHint;
pub use value::Value;
