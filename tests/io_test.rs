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

// Unit tests for internal compression functions
// (These were previously inline in src/io.rs but moved here per Standard #6)

#[test]
fn t_is_xz_compressed_detects_magic_bytes() {
    // Test XZ magic bytes detection
    let xz_data = vec![0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00, 0x01, 0x02];

    // Create temp file with XZ magic bytes
    let path = Path::new("/tmp/test_xz_magic.bin");
    fs::write(path, &xz_data).unwrap();
    let content = fs::read(path).unwrap();
    assert!(content.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]));
    fs::remove_file(path).ok();

    // Test non-XZ data
    let plain_data = vec![0x00, 0x01, 0x02, 0x03];
    let path2 = Path::new("/tmp/test_plain.bin");
    fs::write(path2, &plain_data).unwrap();
    let content2 = fs::read(path2).unwrap();
    assert!(!content2.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]));
    fs::remove_file(path2).ok();
}

#[test]
fn t_compress_decompress_roundtrip() {
    let input = "user{id<u32>(123)}";
    let value = parse(input).unwrap();

    // Compress and decompress via file I/O
    let path = Path::new("/tmp/test_compress_roundtrip.io.gbln.xz");
    let config = GblnConfig::new().compress(true);
    write_io(&value, path, &config).unwrap();

    let loaded = read_io(path).unwrap();
    assert_eq!(loaded, value);

    fs::remove_file(path).ok();
}

#[test]
fn t_compress_produces_xz_format() {
    let input = "test{data<s32>(Test data for compression)}";
    let value = parse(input).unwrap();

    let path = Path::new("/tmp/test_xz_format.io.gbln.xz");
    let config = GblnConfig::new().compress(true);
    write_io(&value, path, &config).unwrap();

    // Check file has XZ magic bytes
    let bytes = fs::read(path).unwrap();
    assert!(bytes.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]));

    fs::remove_file(path).ok();
}

#[test]
fn t_higher_compression_level_produces_smaller_output() {
    let data_str = "A".repeat(1000);
    let input = format!("data{{text<s1024>({})}}", data_str);
    let value = parse(&input).unwrap();

    let temp_dir = std::env::temp_dir();
    let path_0 = temp_dir.join("test_level_0.io.gbln.xz");
    let path_9 = temp_dir.join("test_level_9.io.gbln.xz");

    let config_0 = GblnConfig::new().compress(true).compression_level(0);
    write_io(&value, &path_0, &config_0).unwrap();

    let config_9 = GblnConfig::new().compress(true).compression_level(9);
    write_io(&value, &path_9, &config_9).unwrap();

    let size_0 = fs::metadata(&path_0).unwrap().len();
    let size_9 = fs::metadata(&path_9).unwrap().len();

    // Level 9 should be smaller or equal to level 0
    assert!(size_9 <= size_0);

    fs::remove_file(&path_0).ok();
    fs::remove_file(&path_9).ok();
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
