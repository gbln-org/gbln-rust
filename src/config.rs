//! Configuration for GBLN I/O operations
//!
//! This module provides the `GblnConfig` struct for configuring how GBLN files
//! are serialised and written to disk.

/// Configuration for GBLN I/O operations
///
/// Controls serialisation format, compression, and formatting options.
///
/// # Examples
///
/// ```
/// use gbln::GblnConfig;
///
/// // Default: MINI mode with XZ compression
/// let config = GblnConfig::default();
/// assert_eq!(config.mini_mode, true);
/// assert_eq!(config.compress, true);
///
/// // Development: Pretty-printed, no compression
/// let dev = GblnConfig::development();
/// assert_eq!(dev.mini_mode, false);
/// assert_eq!(dev.compress, false);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GblnConfig {
    /// Enable MINI GBLN format (no structural whitespace)
    pub mini_mode: bool,

    /// Enable XZ compression
    pub compress: bool,

    /// XZ compression level (0-9, where 9 is maximum)
    pub compression_level: u8,

    /// Indentation width for pretty-printing (ignored if mini_mode is true)
    pub indent: usize,

    /// Strip comments when generating I/O format
    pub strip_comments: bool,
}

impl Default for GblnConfig {
    /// Default configuration: MINI mode with XZ compression
    ///
    /// - `mini_mode`: true
    /// - `compress`: true
    /// - `compression_level`: 6
    /// - `indent`: 2
    /// - `strip_comments`: true
    fn default() -> Self {
        Self {
            mini_mode: true,
            compress: true,
            compression_level: 6,
            indent: 2,
            strip_comments: true,
        }
    }
}

impl GblnConfig {
    /// Configuration for development/debugging
    ///
    /// Pretty-printed with comments, no compression.
    ///
    /// - `mini_mode`: false
    /// - `compress`: false
    /// - `compression_level`: 6 (unused)
    /// - `indent`: 2
    /// - `strip_comments`: false
    pub fn development() -> Self {
        Self {
            mini_mode: false,
            compress: false,
            compression_level: 6,
            indent: 2,
            strip_comments: false,
        }
    }

    /// Configuration for I/O format (production)
    ///
    /// MINI GBLN with XZ compression, comments stripped.
    ///
    /// - `mini_mode`: true
    /// - `compress`: true
    /// - `compression_level`: 6
    /// - `indent`: 2 (unused)
    /// - `strip_comments`: true
    pub fn io_format() -> Self {
        Self::default()
    }

    /// Create a custom configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use gbln::GblnConfig;
    ///
    /// let config = GblnConfig::new()
    ///     .mini(true)
    ///     .compress(false)
    ///     .indent(4);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set MINI mode
    pub fn mini(mut self, enabled: bool) -> Self {
        self.mini_mode = enabled;
        self
    }

    /// Set compression
    pub fn compress(mut self, enabled: bool) -> Self {
        self.compress = enabled;
        self
    }

    /// Set compression level (0-9)
    pub fn compression_level(mut self, level: u8) -> Self {
        self.compression_level = level.min(9);
        self
    }

    /// Set indentation width
    pub fn indent(mut self, width: usize) -> Self {
        self.indent = width;
        self
    }

    /// Set whether to strip comments
    pub fn strip_comments(mut self, strip: bool) -> Self {
        self.strip_comments = strip;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
