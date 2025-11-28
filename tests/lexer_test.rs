// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

//! Integration tests for GBLN Lexer

use gbln::{Lexer, Token};

#[test]
fn t_empty_input() {
    let mut lexer = Lexer::new("");
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_single_tokens() {
    let mut lexer = Lexer::new("( ) { } [ ] < >");

    assert_eq!(lexer.next_token().unwrap(), Token::LParen);
    assert_eq!(lexer.next_token().unwrap(), Token::RParen);
    assert_eq!(lexer.next_token().unwrap(), Token::LBrace);
    assert_eq!(lexer.next_token().unwrap(), Token::RBrace);
    assert_eq!(lexer.next_token().unwrap(), Token::LBracket);
    assert_eq!(lexer.next_token().unwrap(), Token::RBracket);
    assert_eq!(lexer.next_token().unwrap(), Token::LAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::RAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_keys() {
    let mut lexer = Lexer::new("user name age_in_years _internal");

    assert_eq!(lexer.next_token().unwrap(), Token::Key("user".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Key("name".to_string()));
    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("age_in_years".to_string())
    );
    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("_internal".to_string())
    );
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_whitespace_handling() {
    let mut lexer = Lexer::new("  user  \n\t  name  ");

    assert_eq!(lexer.next_token().unwrap(), Token::Key("user".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Key("name".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_comments() {
    let input = r#"
        user :| User object
        name :| User name
    "#;

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token().unwrap(), Token::Key("user".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Key("name".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_simple_object() {
    let mut lexer = Lexer::new("user{name<s32>(Alice)}");

    assert_eq!(lexer.next_token().unwrap(), Token::Key("user".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::LBrace);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("name".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::LAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("s32".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::RAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::LParen);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("Alice".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::RParen);
    assert_eq!(lexer.next_token().unwrap(), Token::RBrace);
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_nested_object() {
    let input = r#"
        user{
            name<s64>(Alice Johnson)
            profile{
                age<i8>(25)
            }
        }
    "#;

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token().unwrap(), Token::Key("user".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::LBrace);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("name".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::LAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("s64".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::RAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::LParen);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("Alice".to_string()));
    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("Johnson".to_string())
    );
    assert_eq!(lexer.next_token().unwrap(), Token::RParen);

    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("profile".to_string())
    );
    assert_eq!(lexer.next_token().unwrap(), Token::LBrace);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("age".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::LAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("i8".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::RAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::LParen);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("25".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::RParen);
    assert_eq!(lexer.next_token().unwrap(), Token::RBrace);
    assert_eq!(lexer.next_token().unwrap(), Token::RBrace);
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_array() {
    let mut lexer = Lexer::new("tags<s16>[rust python golang]");

    assert_eq!(lexer.next_token().unwrap(), Token::Key("tags".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::LAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("s16".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::RAngle);
    assert_eq!(lexer.next_token().unwrap(), Token::LBracket);
    assert_eq!(lexer.next_token().unwrap(), Token::Key("rust".to_string()));
    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("python".to_string())
    );
    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("golang".to_string())
    );
    assert_eq!(lexer.next_token().unwrap(), Token::RBracket);
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_line_column_tracking() {
    let input = "user\nname\nage";
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.current_line(), 1);
    assert_eq!(lexer.current_column(), 1);

    lexer.next_token().unwrap(); // "user"
    lexer.next_token().unwrap(); // "name" (after skipping \n, now on line 2)
    assert_eq!(lexer.current_line(), 2);

    lexer.next_token().unwrap(); // "age" (after skipping \n, now on line 3)
    assert_eq!(lexer.current_line(), 3);
}

#[test]
fn t_invalid_character_error() {
    let mut lexer = Lexer::new("user @ name");

    assert_eq!(lexer.next_token().unwrap(), Token::Key("user".to_string()));

    let result = lexer.next_token();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.line, 1);
}

#[test]
fn t_underscore_keys() {
    let mut lexer = Lexer::new("_private user_id MAX_VALUE");

    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("_private".to_string())
    );
    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("user_id".to_string())
    );
    assert_eq!(
        lexer.next_token().unwrap(),
        Token::Key("MAX_VALUE".to_string())
    );
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}

#[test]
fn t_multiline_comments() {
    let input = r#"
        :| First comment
        user
        :| Second comment
        name
    "#;

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token().unwrap(), Token::Key("user".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Key("name".to_string()));
    assert_eq!(lexer.next_token().unwrap(), Token::Eof);
}
