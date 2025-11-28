// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

//! GBLN Lexer
//!
//! Tokenizes GBLN input into a stream of tokens.

use crate::error::{Error, ErrorKind};

/// A token in the GBLN input stream
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Object key (e.g., "user", "name", "age")
    Key(String),

    /// Type hint (e.g., "u32", "s64", "b")
    Type(String),

    /// Left parenthesis (
    LParen,

    /// Right parenthesis )
    RParen,

    /// Left brace {
    LBrace,

    /// Right brace }
    RBrace,

    /// Left bracket [
    LBracket,

    /// Right bracket ]
    RBracket,

    /// Less than < (starts type hint)
    LAngle,

    /// Greater than > (ends type hint)
    RAngle,

    /// End of file
    Eof,
}

/// Lexer for tokenizing GBLN input
pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.char_indices().peekable(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespace_and_comments();

        match self.peek_char() {
            None => Ok(Token::Eof),
            Some('(') => {
                self.advance();
                Ok(Token::LParen)
            }
            Some(')') => {
                self.advance();
                Ok(Token::RParen)
            }
            Some('{') => {
                self.advance();
                Ok(Token::LBrace)
            }
            Some('}') => {
                self.advance();
                Ok(Token::RBrace)
            }
            Some('[') => {
                self.advance();
                Ok(Token::LBracket)
            }
            Some(']') => {
                self.advance();
                Ok(Token::RBracket)
            }
            Some('<') => {
                self.advance();
                Ok(Token::LAngle)
            }
            Some('>') => {
                self.advance();
                Ok(Token::RAngle)
            }
            Some('-') => {
                // Hyphens can appear in identifiers (rust-fan) or as negative numbers (-42)
                // Since we now allow hyphens in read_identifier(), just treat it like any identifier start
                self.read_identifier()
            }
            Some(ch) if is_identifier_start(ch) || ch.is_ascii_digit() => self.read_identifier(),
            Some(ch) => Err(Error::new(
                ErrorKind::UnexpectedCharacter,
                self.line,
                self.column,
                format!("Unexpected character: '{}'", ch),
            )),
        }
    }

    /// Peek at the current character without consuming it
    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, ch)| *ch)
    }

    /// Advance to the next character
    fn advance(&mut self) -> Option<char> {
        if let Some((pos, ch)) = self.chars.next() {
            self.position = pos + ch.len_utf8();

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            Some(ch)
        } else {
            None
        }
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek_char() {
                Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                    self.advance();
                }
                Some(':') => {
                    // Check if this is a comment :|
                    self.advance(); // consume ':'

                    if self.peek_char() == Some('|') {
                        // This is a comment, skip until newline
                        self.advance(); // consume '|'
                        while let Some(ch) = self.peek_char() {
                            if ch == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        // Not a comment, ':' alone is an error
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    /// Read a key, identifier, or value
    fn read_identifier(&mut self) -> Result<Token, Error> {
        let mut ident = String::new();

        // First character already validated by caller
        if let Some(ch) = self.advance() {
            ident.push(ch);
        }

        // Read remaining characters (alphanumeric + underscore + hyphen + optional decimal point for floats)
        let mut has_dot = false;
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ident.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot && ident.chars().all(|c| c.is_ascii_digit() || c == '-')
            {
                // Allow one decimal point in numeric tokens for float support
                has_dot = true;
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Ok(Token::Key(ident))
    }

    /// Get current line number (for error reporting)
    pub fn current_line(&self) -> usize {
        self.line
    }

    /// Get current column number (for error reporting)
    pub fn current_column(&self) -> usize {
        self.column
    }

    /// Read raw content until matching closing parenthesis
    /// Used for reading values between ( and )
    /// Consumes the closing ) as well
    pub fn read_parenthesized_content(&mut self) -> Result<String, Error> {
        let mut content = String::new();
        let mut depth = 0;

        loop {
            match self.peek_char() {
                None => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedEof,
                        self.line,
                        self.column,
                        "Unexpected end of input while reading parenthesized content".to_string(),
                    ))
                }
                Some('(') => {
                    depth += 1;
                    content.push('(');
                    self.advance();
                }
                Some(')') => {
                    if depth == 0 {
                        // Found matching closing paren - consume it
                        self.advance();
                        return Ok(content);
                    }
                    depth -= 1;
                    content.push(')');
                    self.advance();
                }
                Some(ch) => {
                    content.push(ch);
                    self.advance();
                }
            }
        }
    }
}

/// Check if character can start an identifier
fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
