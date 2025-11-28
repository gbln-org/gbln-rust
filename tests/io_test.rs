// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

//! Integration tests for I/O operations
//!
//! Tests write_io() and read_io() functions with various configurations.

use gbln::{parse, read_io, write_io, GblnConfig};
use std::fs;
use std::path::Path;

#[test]
fn t_write_read_roundtrip_with_compression() {
    let input = r#"user{id<u32>(12345)name<s32>(Alice)active<b>(true)}"#;
    let value = parse(input).unwrap();

    let path = Path::new("/tmp/test_roundtrip.io.gbln.xz");

    // Write with compression
    let config = GblnConfig::io_format();
    write_io(&value, path, &config).unwrap();

    // Verify file exists
    assert!(path.exists());

    // Read back
    let loaded = read_io(path).unwrap();

    // Verify content matches
    assert_eq!(loaded, value);

    // Cleanup
    fs::remove_file(path).ok();
}

#[test]
fn t_write_read_roundtrip_without_compression() {
    let input = r#"config{port<u16>(8080)workers<u8>(4)}"#;
    let value = parse(input).unwrap();

    let path = Path::new("/tmp/test_no_compress.io.gbln");

    // Write without compression
    let config = GblnConfig::new().compress(false);
    write_io(&value, path, &config).unwrap();

    // Verify file exists
    assert!(path.exists());

    // Read back
    let loaded = read_io(path).unwrap();

    // Verify content matches
    assert_eq!(loaded, value);

    // Cleanup
    fs::remove_file(path).ok();
}

#[test]
fn t_write_mini_mode_produces_compact_output() {
    let input = r#"data{a<u8>(1)b<u8>(2)c<u8>(3)}"#;
    let value = parse(input).unwrap();

    let path = Path::new("/tmp/test_mini.io.gbln");

    // Write in MINI mode without compression
    let config = GblnConfig::new().mini(true).compress(false);
    write_io(&value, path, &config).unwrap();

    // Read file content
    let content = fs::read_to_string(path).unwrap();

    // Should be compact (no whitespace)
    assert!(!content.contains('\n'));
    assert!(!content.contains("  "));

    // Cleanup
    fs::remove_file(path).ok();
}

#[test]
fn t_write_pretty_mode_produces_formatted_output() {
    let input = r#"data{a<u8>(1)b<u8>(2)c<u8>(3)}"#;
    let value = parse(input).unwrap();

    let path = Path::new("/tmp/test_pretty.gbln");

    // Write in pretty mode
    let config = GblnConfig::development();
    write_io(&value, path, &config).unwrap();

    // Read file content
    let content = fs::read_to_string(path).unwrap();

    // Should have newlines and indentation
    assert!(content.contains('\n'));

    // Cleanup
    fs::remove_file(path).ok();
}

#[test]
fn t_write_with_different_compression_levels() {
    let input = r#"data{x<s128>(This is some test data for compression testing)y<u32>(12345)}"#;
    let value = parse(input).unwrap();

    // Test multiple compression levels
    for level in [0, 3, 6, 9] {
        let path_str = format!("/tmp/test_level_{}.io.gbln.xz", level);
        let path = Path::new(&path_str);

        let config = GblnConfig::new().compression_level(level);
        write_io(&value, path, &config).unwrap();

        // Verify can be read back
        let loaded = read_io(path).unwrap();
        assert_eq!(loaded, value);

        // Cleanup
        fs::remove_file(path).ok();
    }
}

#[test]
fn t_read_auto_detects_xz_compression() {
    let input = r#"test{value<u32>(999)}"#;
    let value = parse(input).unwrap();

    let compressed_path = Path::new("/tmp/test_compressed.io.gbln.xz");
    let uncompressed_path = Path::new("/tmp/test_uncompressed.io.gbln");

    // Write compressed
    let config = GblnConfig::new().compress(true);
    write_io(&value, compressed_path, &config).unwrap();

    // Write uncompressed
    let config = GblnConfig::new().compress(false);
    write_io(&value, uncompressed_path, &config).unwrap();

    // read_io() should handle both
    let loaded_compressed = read_io(compressed_path).unwrap();
    let loaded_uncompressed = read_io(uncompressed_path).unwrap();

    assert_eq!(loaded_compressed, value);
    assert_eq!(loaded_uncompressed, value);

    // Cleanup
    fs::remove_file(compressed_path).ok();
    fs::remove_file(uncompressed_path).ok();
}

#[test]
fn t_compression_reduces_file_size() {
    // Large repeated data compresses well
    let input = r#"data{a<s256>(AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA)b<s256>(BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB)}"#;
    let value = parse(input).unwrap();

    let compressed_path = Path::new("/tmp/test_size_compressed.io.gbln.xz");
    let uncompressed_path = Path::new("/tmp/test_size_uncompressed.io.gbln");

    // Write both versions
    let config_compressed = GblnConfig::new().compress(true);
    write_io(&value, compressed_path, &config_compressed).unwrap();

    let config_uncompressed = GblnConfig::new().compress(false);
    write_io(&value, uncompressed_path, &config_uncompressed).unwrap();

    // Check sizes
    let compressed_size = fs::metadata(compressed_path).unwrap().len();
    let uncompressed_size = fs::metadata(uncompressed_path).unwrap().len();

    // Compressed should be smaller (but compression has overhead, so just check it's smaller)
    assert!(
        compressed_size < uncompressed_size,
        "Compressed ({} bytes) should be smaller than uncompressed ({} bytes)",
        compressed_size,
        uncompressed_size
    );

    // Cleanup
    fs::remove_file(compressed_path).ok();
    fs::remove_file(uncompressed_path).ok();
}

#[test]
fn t_config_builder_pattern() {
    let config = GblnConfig::new()
        .mini(true)
        .compress(true)
        .compression_level(9)
        .indent(4)
        .strip_comments(false);

    assert_eq!(config.mini_mode, true);
    assert_eq!(config.compress, true);
    assert_eq!(config.compression_level, 9);
    assert_eq!(config.indent, 4);
    assert_eq!(config.strip_comments, false);
}

#[test]
fn t_config_presets() {
    let dev = GblnConfig::development();
    assert_eq!(dev.mini_mode, false);
    assert_eq!(dev.compress, false);

    let io = GblnConfig::io_format();
    assert_eq!(io.mini_mode, true);
    assert_eq!(io.compress, true);

    let default = GblnConfig::default();
    assert_eq!(default, io);
}

#[test]
fn t_roundtrip_complex_nested_structure() {
    let input = r#"
        app{
            name<s32>(TestApp)
            version<s16>(1.0.0)
            server{
                host<s64>(localhost)
                port<u16>(8080)
                workers<u8>(4)
            }
            database{
                host<s64>(db.example.com)
                port<u16>(5432)
                name<s32>(testdb)
            }
        }
    "#;

    let value = parse(input).unwrap();
    let path = Path::new("/tmp/test_complex.io.gbln.xz");

    // Write and read back
    let config = GblnConfig::io_format();
    write_io(&value, path, &config).unwrap();
    let loaded = read_io(path).unwrap();

    assert_eq!(loaded, value);

    // Cleanup
    fs::remove_file(path).ok();
}
