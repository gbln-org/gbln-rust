// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

//! GBLN (Goblin Bounded Lean Notation)
//!
//! A type-safe, LLM-native data serialisation format with parse-time validation.

pub mod config;
pub mod error;
#[cfg(feature = "compression")]
pub mod io;
pub mod lexer;
pub mod parser;
pub mod serializer;
pub mod types;
pub mod value;

pub use config::GblnConfig;
pub use error::{Error, ErrorKind};
#[cfg(feature = "compression")]
pub use io::{read_io, write_io};
pub use lexer::{Lexer, Token};
pub use parser::parse;
pub use serializer::{to_string, to_string_pretty};
pub use types::TypeHint;
pub use value::Value;
