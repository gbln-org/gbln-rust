// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

//! Tests for GblnConfig

use gbln::GblnConfig;

#[test]
fn t_default_config() {
    let config = GblnConfig::default();
    assert_eq!(config.mini_mode, true);
    assert_eq!(config.compress, true);
    assert_eq!(config.compression_level, 6);
    assert_eq!(config.indent, 2);
    assert_eq!(config.strip_comments, true);
}

#[test]
fn t_development_config() {
    let config = GblnConfig::development();
    assert_eq!(config.mini_mode, false);
    assert_eq!(config.compress, false);
    assert_eq!(config.indent, 2);
    assert_eq!(config.strip_comments, false);
}

#[test]
fn t_io_format_config() {
    let config = GblnConfig::io_format();
    assert_eq!(config.mini_mode, true);
    assert_eq!(config.compress, true);
    assert_eq!(config.compression_level, 6);
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
fn t_builder_pattern() {
    let config = GblnConfig::new()
        .mini(false)
        .compress(true)
        .compression_level(9)
        .indent(4)
        .strip_comments(false);

    assert_eq!(config.mini_mode, false);
    assert_eq!(config.compress, true);
    assert_eq!(config.compression_level, 9);
    assert_eq!(config.indent, 4);
    assert_eq!(config.strip_comments, false);
}

#[test]
fn t_compression_level_clamped() {
    let config = GblnConfig::new().compression_level(99);
    assert_eq!(config.compression_level, 9);
}
