# GBLN Rust - Complete Artifact List

**Build Date**: 2025-11-26  
**Host Platform**: macOS 15.1 ARM64

## All Build Artifacts

| # | Platform | Target Triple | Artifact | Size | Location |
|---|----------|---------------|----------|------|----------|
| 1 | FreeBSD ARM64 | `aarch64-unknown-freebsd` | libgbln.rlib | 361 KB | target/aarch64-unknown-freebsd/release/ |
| 2 | FreeBSD x86_64 | `x86_64-unknown-freebsd` | libgbln.rlib | 384 KB | target/x86_64-unknown-freebsd/release/ |
| 3 | Linux ARM64 | `aarch64-unknown-linux-gnu` | libgbln.rlib | 386 KB | target/aarch64-unknown-linux-gnu/release/ |
| 4 | Linux x86_64 | `x86_64-unknown-linux-gnu` | libgbln.rlib | 385 KB | target/x86_64-unknown-linux-gnu/release/ |
| 5 | macOS ARM64 | `aarch64-apple-darwin` (native) | libgbln.rlib | 296 KB | target/release/ |
| 6 | macOS x86_64 | `x86_64-apple-darwin` | libgbln.rlib | 291 KB | target/x86_64-apple-darwin/release/ |
| 7 | Windows x86_64 | `x86_64-pc-windows-gnu` | libgbln.rlib | 290 KB | target/x86_64-pc-windows-gnu/release/ |
| 8 | iOS ARM64 | `aarch64-apple-ios` | libgbln.rlib | 290 KB | target/aarch64-apple-ios/release/ |
| 9 | Android ARM64 | `aarch64-linux-android` | libgbln.rlib | 390 KB | target/aarch64-linux-android/release/ |
| 10 | Android x86_64 | `x86_64-linux-android` | libgbln.rlib | 387 KB | target/x86_64-linux-android/release/ |

## Size Analysis

**Average size**: 344 KB  
**Smallest**: 290 KB (Windows, iOS)  
**Largest**: 390 KB (Android ARM64)  
**Range**: 100 KB difference

**Platform grouping by size**:
- **~290 KB**: Windows, iOS, macOS x86_64, macOS ARM64
- **~360-390 KB**: FreeBSD, Linux, Android

## File Format

All artifacts are Rust library archives (`.rlib`) containing:
- Compiled object code
- Metadata for Rust compiler
- Debug information (in release mode, optimized)

## Usage

These `.rlib` files are used when linking GBLN into other Rust projects:

```toml
# Cargo.toml
[dependencies]
gbln = { path = "../path/to/gbln" }
```

Or for FFI bindings, the C dynamic libraries will be built from these artifacts.

---

*Complete: 10/10 platforms built successfully* ✅
