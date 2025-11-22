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
└── serializer.rs       # GBLN output (compact & formatted)

tests/
├── parse_test.rs       # Parsing tests
├── validate_test.rs    # Validation tests
├── serialize_test.rs   # Serialization tests
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

### Serialization

```rust
use gbln::{parse, to_string, to_string_pretty};

let value = parse("user{id<u32>(123)}")?;

// Compact format (production)
let compact = to_string(&value);
// => "user{id<u32>(123)}"

// Formatted (development)
let formatted = to_string_pretty(&value);
// => "user{
//       id<u32>(123)
//     }"
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
