use crate::error::{Error, ErrorKind};
use crate::lexer::{Lexer, Token};
use crate::types::TypeHint;
use crate::value::Value;
use std::collections::HashMap;

/// Parser for GBLN format using recursive descent
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    /// Create a new parser from input string
    pub fn new(input: &'a str) -> Result<Self, Error> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        Ok(Parser {
            lexer,
            current_token,
        })
    }

    /// Advance to next token
    fn advance(&mut self) -> Result<(), Error> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    /// Expect a specific token and advance
    fn expect(&mut self, expected: Token) -> Result<(), Error> {
        if std::mem::discriminant(&self.current_token) != std::mem::discriminant(&expected) {
            return Err(Error::new(
                ErrorKind::UnexpectedToken,
                self.lexer.current_line(),
                self.lexer.current_column(),
                format!("Expected {:?}, found {:?}", expected, self.current_token),
            ));
        }
        self.advance()
    }

    /// Parse the entire document
    pub fn parse(&mut self) -> Result<Value, Error> {
        let value = self.parse_value()?;

        // Ensure we consumed entire input
        if !matches!(self.current_token, Token::Eof) {
            return Err(Error::new(
                ErrorKind::UnexpectedToken,
                self.lexer.current_line(),
                self.lexer.current_column(),
                format!("Unexpected token after value: {:?}", self.current_token),
            ));
        }

        Ok(value)
    }

    /// Parse any value (object, array, or typed single value)
    fn parse_value(&mut self) -> Result<Value, Error> {
        match &self.current_token {
            Token::Key(_) => {
                // Could be object field or typed array start
                self.parse_keyed_value()
            }
            Token::LAngle => {
                // Typed single value: <type>(value)
                self.parse_typed_single_value()
            }
            Token::LBrace => {
                // Anonymous object
                self.parse_object()
            }
            Token::LBracket => {
                // Anonymous array
                self.parse_array()
            }
            _ => Err(Error::new(
                ErrorKind::UnexpectedToken,
                self.lexer.current_line(),
                self.lexer.current_column(),
                format!("Unexpected token: {:?}", self.current_token),
            )),
        }
    }

    /// Parse keyed value (object or typed array)
    fn parse_keyed_value(&mut self) -> Result<Value, Error> {
        let key = match &self.current_token {
            Token::Key(k) => k.clone(),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken,
                    self.lexer.current_line(),
                    self.lexer.current_column(),
                    "Expected key".to_string(),
                ))
            }
        };

        self.advance()?;

        match &self.current_token {
            Token::LBrace => {
                // Object: key{...}
                let inner_obj = self.parse_object()?;
                // Wrap with the key
                let mut obj = HashMap::new();
                obj.insert(key, inner_obj);
                Ok(Value::Object(obj))
            }
            Token::LAngle => {
                // Could be typed single value or typed array
                let type_hint = self.parse_type_hint()?;

                match &self.current_token {
                    Token::LParen => {
                        // Typed single value: key<type>(value)
                        let value = self.parse_single_value_content(&type_hint)?;
                        // Wrap in object with single field
                        let mut obj = HashMap::new();
                        obj.insert(key, value);
                        Ok(Value::Object(obj))
                    }
                    Token::LBracket => {
                        // Typed array: key<type>[...]
                        let values = self.parse_typed_array_content(&type_hint)?;
                        // Wrap in object with single field
                        let mut obj = HashMap::new();
                        obj.insert(key, Value::Array(values));
                        Ok(Value::Object(obj))
                    }
                    _ => Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        self.lexer.current_line(),
                        self.lexer.current_column(),
                        "Expected '(' or '[' after type hint".to_string(),
                    )),
                }
            }
            _ => Err(Error::new(
                ErrorKind::InvalidSyntax,
                self.lexer.current_line(),
                self.lexer.current_column(),
                format!(
                    "Expected '{{' or '<' after key, found {:?}",
                    self.current_token
                ),
            )),
        }
    }

    /// Parse object: {...}
    fn parse_object(&mut self) -> Result<Value, Error> {
        self.expect(Token::LBrace)?;

        let mut fields = HashMap::new();

        while !matches!(self.current_token, Token::RBrace | Token::Eof) {
            let (key, value) = self.parse_object_field()?;

            // Check for duplicate keys
            if fields.contains_key(&key) {
                return Err(Error::new(
                    ErrorKind::DuplicateKey,
                    self.lexer.current_line(),
                    self.lexer.current_column(),
                    format!("Duplicate key: {}", key),
                ));
            }

            fields.insert(key, value);
        }

        self.expect(Token::RBrace)?;
        Ok(Value::Object(fields))
    }

    /// Parse single object field: key<type>(value) or key{...}
    fn parse_object_field(&mut self) -> Result<(String, Value), Error> {
        let key = match &self.current_token {
            Token::Key(k) => k.clone(),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken,
                    self.lexer.current_line(),
                    self.lexer.current_column(),
                    "Expected key in object field".to_string(),
                ))
            }
        };

        self.advance()?;

        // Check what follows the key
        match &self.current_token {
            Token::LBrace => {
                // Nested object: key{...}
                let value = self.parse_object()?;
                Ok((key, value))
            }
            Token::LBracket => {
                // Array: key[...]
                let value = self.parse_array()?;
                Ok((key, value))
            }
            Token::LAngle => {
                // Typed value: key<type>(value)
                let type_hint = self.parse_type_hint()?;
                let value = self.parse_single_value_content(&type_hint)?;
                Ok((key, value))
            }
            _ => Err(Error::new(
                ErrorKind::UnexpectedToken,
                self.lexer.current_line(),
                self.lexer.current_column(),
                format!(
                    "Expected '<', '{{', or '[' after key, found {:?}",
                    self.current_token
                ),
            )),
        }
    }

    /// Parse type hint: <type>
    fn parse_type_hint(&mut self) -> Result<TypeHint, Error> {
        self.expect(Token::LAngle)?;

        let type_str = match &self.current_token {
            Token::Key(t) | Token::Type(t) => t.clone(),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken,
                    self.lexer.current_line(),
                    self.lexer.current_column(),
                    "Expected type hint".to_string(),
                ))
            }
        };

        self.advance()?;
        self.expect(Token::RAngle)?;

        TypeHint::from_str(&type_str).map_err(|e| {
            Error::new(
                ErrorKind::InvalidTypeHint,
                self.lexer.current_line(),
                self.lexer.current_column(),
                e,
            )
        })
    }

    /// Parse single value content: (value)
    fn parse_single_value_content(&mut self, type_hint: &TypeHint) -> Result<Value, Error> {
        // Check for LParen but DON'T call advance() - we need to read raw content
        if !matches!(self.current_token, Token::LParen) {
            return Err(Error::new(
                ErrorKind::UnexpectedToken,
                self.lexer.current_line(),
                self.lexer.current_column(),
                format!("Expected '(', found {:?}", self.current_token),
            ));
        }

        // Read raw content directly from lexer (bypassing tokenization)
        // The lexer is positioned right after the ( token
        // This also consumes the closing )
        let content = self.lexer.read_parenthesized_content()?;

        // Refresh current_token after raw read
        self.current_token = self.lexer.next_token()?;

        // Parse content according to type hint
        type_hint.parse_value(&content).map_err(|e| {
            Error::new(
                ErrorKind::TypeMismatch,
                self.lexer.current_line(),
                self.lexer.current_column(),
                e,
            )
        })
    }

    /// Parse array: [...]
    fn parse_array(&mut self) -> Result<Value, Error> {
        self.expect(Token::LBracket)?;

        let mut items = Vec::new();

        while !matches!(self.current_token, Token::RBracket | Token::Eof) {
            let value = self.parse_array_item()?;
            items.push(value);
        }

        self.expect(Token::RBracket)?;
        Ok(Value::Array(items))
    }

    /// Parse single array item (could be typed value or object)
    fn parse_array_item(&mut self) -> Result<Value, Error> {
        match &self.current_token {
            Token::LAngle => {
                // Typed value in array
                self.parse_typed_single_value()
            }
            Token::LBrace => {
                // Object in array
                self.parse_object()
            }
            _ => Err(Error::new(
                ErrorKind::UnexpectedToken,
                self.lexer.current_line(),
                self.lexer.current_column(),
                format!("Unexpected token in array: {:?}", self.current_token),
            )),
        }
    }

    /// Parse typed single value: <type>(value)
    fn parse_typed_single_value(&mut self) -> Result<Value, Error> {
        let type_hint = self.parse_type_hint()?;
        self.parse_single_value_content(&type_hint)
    }

    /// Parse typed array content: [val1 val2 val3]
    fn parse_typed_array_content(&mut self, type_hint: &TypeHint) -> Result<Vec<Value>, Error> {
        self.expect(Token::LBracket)?;

        let mut items = Vec::new();

        while !matches!(self.current_token, Token::RBracket | Token::Eof) {
            // Read raw value token
            let value_str = match &self.current_token {
                Token::Key(s) | Token::Type(s) => s.clone(),
                _ => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        self.lexer.current_line(),
                        self.lexer.current_column(),
                        "Expected value in typed array".to_string(),
                    ))
                }
            };

            let value = type_hint.parse_value(&value_str).map_err(|e| {
                Error::new(
                    ErrorKind::TypeMismatch,
                    self.lexer.current_line(),
                    self.lexer.current_column(),
                    e,
                )
            })?;

            items.push(value);
            self.advance()?;
        }

        self.expect(Token::RBracket)?;
        Ok(items)
    }
}

/// Public parse function
pub fn parse(input: &str) -> Result<Value, Error> {
    let mut parser = Parser::new(input)?;
    parser.parse()
}
