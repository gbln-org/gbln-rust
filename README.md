# GBLN Rust Reference Implementation

<div align="center">

**The reference parser implementation for GBLN (Goblin Bounded Lean Notation)**

[![Crates.io](https://img.shields.io/crates/v/gbln)](https://crates.io/crates/gbln)
[![Documentation](https://docs.rs/gbln/badge.svg)](https://docs.rs/gbln)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)

</div>

---

## About

This is the **reference implementation** of the GBLN parser written in Rust. It provides:

- ✅ **Complete GBLN parser** - All types, validation, error handling
- ✅ **Type-safe parsing** - Parse-time validation with bounded types
- ✅ **Detailed errors** - Line/column numbers with helpful suggestions
- ✅ **Serialization** - Parse → modify → serialize back to GBLN
- ✅ **Zero-copy where possible** - Efficient memory usage
- ✅ **No unsafe code** - Safe Rust throughout

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gbln = "0.1"
```

### Feature Flags

```toml
[dependencies]
gbln = { version = "0.1", features = ["io", "compression"] }
```

**Available features:**
- **`io`** - Enable I/O format read/write functions (`write_io`, `read_io`)
- **`compression`** - Enable XZ compression (requires `xz2` crate)
- **`default`** - Includes core parsing and serialization only

**Dependencies by feature:**
- Core: No external dependencies (zero-dependency parser)
- `io`: Adds file I/O support
- `compression`: Adds `xz2` for XZ compression/decompression

---

## Quick Start

```rust
use gbln::{parse, Value};

fn main() -> Result<(), gbln::Error> {
    let input = r#"
        user{
            id<u32>(12345)
            name<s64>(Alice Johnson)
            active<b>(true)
        }
    "#;
    
    let value = parse(input)?;
    
    // Access values
    if let Some(id) = value["user"]["id"].as_u32() {
        println!("User ID: {}", id);
    }
    
    Ok(())
}
```

---

## The I/O Format Architecture

**GBLN uses a dual-file system:**

### File Types

#### `.gbln` - Human-Editable Source
- Purpose: Edited by developers, committed to Git
- Format: Pretty-printed with 2-space indentation
- Comments: Preserved
- Whitespace: Readable formatting

#### `.io.gbln.xz` - Compressed I/O Format
- Purpose: Optimised for storage and transmission
- Format: MINI GBLN + XZ compression (level 6 default)
- Size: ~65-75% smaller than `.gbln`
- Comments: Stripped
- Whitespace: Removed (except within values)

#### `.io.gbln` - Intermediate Format
- Purpose: MINI GBLN without compression
- Rarely used directly (debugging/testing)

### Workflow

```bash
# 1. Developer edits source
vim config.gbln

# 2. Generate I/O format
gbln write config.gbln  # → config.io.gbln.xz

# 3. Application uses I/O format
myapp --config config.io.gbln.xz

# 4. Update source from I/O (if needed)
gbln read config.gbln  # Reads config.io.gbln.xz
```

---

## Features

### Parse-Time Type Validation

```rust
use gbln::parse;

// Valid - fits in u8 range
let valid = parse("count<u8>(200)");
assert!(valid.is_ok());

// Error - out of range for u8
let invalid = parse("count<u8>(300)");
assert!(invalid.is_err());
```

### Bounded String Types

```rust
use gbln::parse;

// Valid - 5 characters ≤ 32
let valid = parse("name<s32>(Alice)");
assert!(valid.is_ok());

// Error - exceeds s8 limit
let invalid = parse("name<s8>(VeryLongName)");
assert!(invalid.is_err());
```

### Detailed Error Messages

```rust
use gbln::parse;

let result = parse("age<i8>(999)");

match result {
    Err(e) => {
        // Error: Integer out of range
        //   at field: age
        //   value: 999
        //   type: i8
        //   valid range: -128 to 127
        //   line: 1
        //   column: 5
        //   
        //   suggestion: Use i16 or i32 for larger values
        println!("{}", e);
    }
    Ok(_) => {}
}
```

---

## Project Structure

```
src/
├── lib.rs              # Public API
├── lexer.rs            # Tokenization
├── parser.rs           # Parsing logic
├── types.rs            # Type system (TypeHint, bounds)
├── value.rs            # Value representation
├── validator.rs        # Validation rules
├── error.rs            # Error types with context
├── serializer.rs       # GBLN output (MINI & pretty)
├── config.rs           # GblnConfig configuration
├── io.rs               # I/O format read/write
└── compression.rs      # XZ compression/decompression

tests/
├── parse_test.rs       # Parsing tests
├── validate_test.rs    # Validation tests
├── serialize_test.rs   # Serialization tests
├── io_test.rs          # I/O format tests
└── fixtures/           # Test data files
```

---

## Type System

All GBLN types are supported:

### Integer Types
- **Signed**: `i8`, `i16`, `i32`, `i64`
- **Unsigned**: `u8`, `u16`, `u32`, `u64`

### Float Types
- `f32`, `f64`

### String Types
- `s2`, `s4`, `s8`, `s16`, `s32`, `s64`, `s128`, `s256`, `s512`, `s1024`
- Character count validation (UTF-8 aware)

### Other Types
- `b` - Boolean (t/f/true/false/0/1)
- `n` - Null (empty/null/n)

---

## API Overview

### Parsing

```rust
use gbln::{parse, Value};

// Parse GBLN string
let value: Value = parse("user{id<u32>(123)}")?;

// Access nested values
let id = value["user"]["id"].as_u32().unwrap();
```

### Configuration

```rust
use gbln::GblnConfig;

// Default configuration (MINI mode enabled)
let config = GblnConfig::default();

// Custom configuration
let config = GblnConfig {
    mini_mode: true,          // Enable MINI GBLN (no structural whitespace)
    compress: true,           // Enable XZ compression
    compression_level: 6,     // XZ compression level (0-9)
    indent: 2,                // Indentation width for pretty printing
    strip_comments: true,     // Strip comments in I/O format
};

// Preset configurations
let dev_config = GblnConfig::development();   // Pretty, no compression
let io_config = GblnConfig::io_format();      // MINI + XZ compression
```

### Serialization

```rust
use gbln::{parse, to_string, to_string_pretty, to_string_with_config, GblnConfig};

let value = parse("user{id<u32>(123)}")?;

// MINI GBLN (default for internal use)
let mini = to_string(&value);
// => "user{id<u32>(123)}"

// Pretty-printed (for source files)
let formatted = to_string_pretty(&value);
// => "user{
//       id<u32>(123)
//     }"

// Custom configuration
let config = GblnConfig { indent: 4, ..Default::default() };
let custom = to_string_with_config(&value, &config);
```

### I/O Format Generation

```rust
use gbln::{parse, write_io, read_io, GblnConfig};
use std::path::Path;

// Parse source file
let value = parse_file("config.gbln")?;

// Generate I/O format (MINI + XZ compressed)
let config = GblnConfig::io_format();
write_io(&value, Path::new("config.io.gbln.xz"), &config)?;
// Generates: config.io.gbln.xz (~65-75% smaller)

// Read from I/O format
let value = read_io(Path::new("config.io.gbln.xz"))?;

// Without compression (debugging)
let config = GblnConfig {
    mini_mode: true,
    compress: false,
    ..Default::default()
};
write_io(&value, Path::new("config.io.gbln"), &config)?;
```

### Value Access

```rust
use gbln::{parse, Value};

let value = parse("data{count<u32>(42)}")?;

// Type-safe accessors
if let Some(n) = value["data"]["count"].as_u32() {
    println!("Count: {}", n);
}

// Check value type
if value["data"]["count"].is_u32() {
    // ...
}
```

---

## Complete Example: Configuration File Workflow

```rust
use gbln::{parse, write_io, read_io, to_string_pretty, GblnConfig};
use std::path::Path;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. DEVELOPMENT: Create human-readable config
    let source_config = r#"
        :| Application Configuration
        app{
          name<s32>(MyApplication)
          version<s16>(1.0.0)
          server{
            host<s64>(localhost)
            port<u16>(8080)
            workers<u8>(4)
          }
          features{
            debug<b>(false)
            logging<b>(true)
          }
        }
    "#;
    
    // 2. Parse and validate
    let value = parse(source_config)?;
    println!("✓ Configuration validated");
    
    // 3. Write pretty source file (for Git)
    let pretty = to_string_pretty(&value);
    fs::write("config.gbln", pretty)?;
    println!("✓ Written: config.gbln (234 bytes, human-readable)");
    
    // 4. PRODUCTION: Generate I/O format
    let io_config = GblnConfig::io_format();
    write_io(&value, Path::new("config.io.gbln.xz"), &io_config)?;
    println!("✓ Generated: config.io.gbln.xz (~80 bytes, 66% smaller)");
    
    // 5. APPLICATION: Read from I/O format
    let loaded_value = read_io(Path::new("config.io.gbln.xz"))?;
    
    // 6. Access configuration values
    let port = loaded_value["app"]["server"]["port"].as_u16().unwrap();
    let workers = loaded_value["app"]["server"]["workers"].as_u8().unwrap();
    println!("✓ Server will run on port {} with {} workers", port, workers);
    
    // 7. DEBUGGING: Generate uncompressed I/O format
    let debug_config = GblnConfig {
        mini_mode: true,
        compress: false,
        strip_comments: true,
        ..Default::default()
    };
    write_io(&value, Path::new("config.io.gbln"), &debug_config)?;
    println!("✓ Debug: config.io.gbln (156 bytes, MINI without compression)");
    
    Ok(())
}
```

**Generated Files:**

```bash
config.gbln          # 234 bytes - Source (Git-tracked)
config.io.gbln.xz    #  80 bytes - Production I/O (66% smaller)
config.io.gbln       # 156 bytes - Debug (MINI only, optional)
```

**File Contents:**

`config.gbln` (Source):
```gbln
:| Application Configuration
app{
  name<s32>(MyApplication)
  version<s16>(1.0.0)
  server{
    host<s64>(localhost)
    port<u16>(8080)
    workers<u8>(4)
  }
  features{
    debug<b>(false)
    logging<b>(true)
  }
}
```

`config.io.gbln` (MINI GBLN):
```gbln
app{name<s32>(MyApplication)version<s16>(1.0.0)server{host<s64>(localhost)port<u16>(8080)workers<u8>(4)}features{debug<b>(false)logging<b>(true)}}
```

`config.io.gbln.xz` (Binary - XZ compressed MINI GBLN)

---

## Development Standards

This implementation follows strict standards:

- **BBC English** - All comments in British English
- **File size <400 lines** - Split large modules
- **No generic names** - Specific, descriptive names
- **Separate test files** - `module.rs` + `module_test.rs`
- **Single responsibility** - One function, one job
- **Comprehensive tests** - 100+ tests covering all cases

---

## Performance Targets

- **Parse speed**: ~65ms for 1000 records
- **Memory**: 70% smaller than equivalent JSON in memory
- **Zero-copy**: String slices where possible

---

## Benchmarks

Compare with `serde_json`:

```bash
cargo bench
```

---

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test parse_integers

# Run benchmarks
cargo bench
```

---

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) in the main repository.

---

## License

Apache License 2.0 - see [LICENSE](LICENSE)

---

## Links

- **Main Repository**: https://github.com/gbln-org/gbln
- **Specification**: https://github.com/gbln-org/gbln/tree/main/docs
- **Crates.io**: https://crates.io/crates/gbln (coming soon)
- **Documentation**: https://docs.rs/gbln (coming soon)

---

**Part of the GBLN ecosystem**  
*The first type-safe, memory-bounded LLM-native data format*
