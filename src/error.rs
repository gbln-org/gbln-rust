//! GBLN Error Types
//!
//! Detailed error types with line/column information and suggestions.

use std::fmt;

/// GBLN parsing/validation error
#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub suggestion: Option<String>,
}

/// Error categories
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // Lexer errors
    UnexpectedCharacter,
    UnterminatedString,

    // Parser errors
    UnexpectedToken,
    UnexpectedEof,
    InvalidSyntax,

    // Type errors
    IntegerOutOfRange,
    StringTooLong,
    TypeMismatch,
    InvalidTypeHint,

    // Structural errors
    DuplicateKey,

    // I/O errors
    IoError,
}

impl Error {
    pub fn new(kind: ErrorKind, line: usize, column: usize, message: String) -> Self {
        Self {
            kind,
            line,
            column,
            message,
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// Create an I/O error (no line/column context)
    pub fn io(message: String) -> Self {
        Self {
            kind: ErrorKind::IoError,
            line: 0,
            column: 0,
            message,
            suggestion: None,
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::io(message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error: {:?}", self.kind)?;
        writeln!(f, "  {}", self.message)?;
        writeln!(f, "  at line {}, column {}", self.line, self.column)?;

        if let Some(suggestion) = &self.suggestion {
            writeln!(f)?;
            writeln!(f, "  suggestion: {}", suggestion)?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {}
