//! File I/O operations for GBLN
//!
//! This module provides functions to read and write GBLN files in various formats,
//! including XZ-compressed I/O format.

use crate::{Error, GblnConfig, Value};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;

/// Write a GBLN value to a file in I/O format
///
/// This function serialises the value according to the configuration and writes
/// it to the specified file. If compression is enabled, the file will be XZ compressed.
///
/// # File Extensions
///
/// - `.io.gbln.xz` - MINI GBLN + XZ compression (default)
/// - `.io.gbln` - MINI GBLN without compression
/// - `.gbln` - Pretty-printed source format
///
/// # Examples
///
/// ```no_run
/// use gbln::{parse, write_io, GblnConfig};
/// use std::path::Path;
///
/// let value = parse("user{id<u32>(123)name<s32>(Alice)}")?;
///
/// // Write with XZ compression (default)
/// let config = GblnConfig::io_format();
/// write_io(&value, Path::new("config.io.gbln.xz"), &config)?;
///
/// // Write without compression (debugging)
/// let config = GblnConfig::new().compress(false);
/// write_io(&value, Path::new("config.io.gbln"), &config)?;
/// # Ok::<(), gbln::Error>(())
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Serialisation fails
/// - Compression fails
/// - File cannot be written
pub fn write_io(value: &Value, path: &Path, config: &GblnConfig) -> Result<(), Error> {
    // 1. Serialise based on mini_mode
    let content = if config.mini_mode {
        crate::to_string(value)
    } else {
        crate::to_string_pretty(value)
    };

    // 2. Optionally compress
    let bytes = if config.compress {
        compress_xz(content.as_bytes(), config.compression_level)?
    } else {
        content.into_bytes()
    };

    // 3. Write to file
    fs::write(path, bytes).map_err(|e| Error::io(e.to_string()))?;

    Ok(())
}

/// Read a GBLN file from I/O format
///
/// This function reads a file and automatically detects if it's XZ compressed.
/// The content is then parsed into a GBLN value.
///
/// # Auto-Detection
///
/// The function checks for XZ magic bytes (`FD 37 7A 58 5A 00`) and automatically
/// decompresses if detected.
///
/// # Examples
///
/// ```no_run
/// use gbln::read_io;
/// use std::path::Path;
///
/// // Reads and auto-decompresses if .xz
/// let value = read_io(Path::new("config.io.gbln.xz"))?;
///
/// // Also works with uncompressed files
/// let value = read_io(Path::new("config.io.gbln"))?;
/// # Ok::<(), gbln::Error>(())
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - File cannot be read
/// - Decompression fails (for .xz files)
/// - Content is not valid UTF-8
/// - Parsing fails
pub fn read_io(path: &Path) -> Result<Value, Error> {
    // 1. Read file
    let bytes = fs::read(path).map_err(|e| Error::io(e.to_string()))?;

    // 2. Decompress if needed
    let text = if is_xz_compressed(&bytes) {
        let decompressed = decompress_xz(&bytes)?;
        String::from_utf8(decompressed).map_err(|e| Error::io(e.to_string()))?
    } else {
        String::from_utf8(bytes).map_err(|e| Error::io(e.to_string()))?
    };

    // 3. Parse
    crate::parse(&text)
}

/// Compress data using XZ compression
///
/// # Arguments
///
/// * `data` - Data to compress
/// * `level` - Compression level (0-9, where 9 is maximum)
///
/// # Errors
///
/// Returns an error if compression fails
fn compress_xz(data: &[u8], level: u8) -> Result<Vec<u8>, Error> {
    let mut encoder = XzEncoder::new(Vec::new(), level as u32);
    encoder
        .write_all(data)
        .map_err(|e| Error::io(format!("XZ compression failed: {}", e)))?;
    encoder
        .finish()
        .map_err(|e| Error::io(format!("XZ compression failed: {}", e)))
}

/// Decompress XZ-compressed data
///
/// # Arguments
///
/// * `data` - XZ-compressed data
///
/// # Errors
///
/// Returns an error if decompression fails
fn decompress_xz(data: &[u8]) -> Result<Vec<u8>, Error> {
    let mut decoder = XzDecoder::new(data);
    let mut result = Vec::new();
    decoder
        .read_to_end(&mut result)
        .map_err(|e| Error::io(format!("XZ decompression failed: {}", e)))?;
    Ok(result)
}

/// Check if data is XZ compressed
///
/// XZ files start with magic bytes: `FD 37 7A 58 5A 00`
///
/// # Arguments
///
/// * `data` - Data to check
///
/// # Returns
///
/// `true` if data starts with XZ magic bytes, `false` otherwise
fn is_xz_compressed(data: &[u8]) -> bool {
    data.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_is_xz_compressed_detects_magic_bytes() {
        let xz_data = vec![0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00, 0x01, 0x02];
        assert_eq!(is_xz_compressed(&xz_data), true);

        let plain_data = vec![0x00, 0x01, 0x02, 0x03];
        assert_eq!(is_xz_compressed(&plain_data), false);
    }

    #[test]
    fn t_compress_decompress_roundtrip() {
        let original = b"Hello, GBLN!";
        let compressed = compress_xz(original, 6).unwrap();
        let decompressed = decompress_xz(&compressed).unwrap();

        assert_eq!(decompressed, original);
    }

    #[test]
    fn t_compress_produces_xz_format() {
        let data = b"Test data for compression";
        let compressed = compress_xz(data, 6).unwrap();

        // Should have XZ magic bytes
        assert!(is_xz_compressed(&compressed));
    }

    #[test]
    fn t_higher_compression_level_produces_smaller_output() {
        let data = b"A".repeat(1000);

        let level_0 = compress_xz(&data, 0).unwrap();
        let level_9 = compress_xz(&data, 9).unwrap();

        // Level 9 should be smaller or equal to level 0
        assert!(level_9.len() <= level_0.len());
    }
}
