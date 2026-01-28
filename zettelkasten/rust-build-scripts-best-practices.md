# Build Scripts Best Practices

**Created**: 2026-01-27  
**Status**: 🟢 Active  
**Tags**: #rust #build-scripts #cargo #compilation #optimization

## Overview

Build scripts (`build.rs`) are Rust programs that run **before** your main crate compilation. They enable compile-time code generation, FFI setup, platform-specific configuration, and build-time validation.

## Core Concepts

### What Are Build Scripts?

A `build.rs` file in your crate root that:
- Runs during `cargo build` **before** compiling your crate
- Can emit compiler directives via `cargo:` protocol
- Has access to environment variables from Cargo
- Cannot use dependencies from your main crate (separate dependency graph)

### When to Use Build Scripts

✅ **Good use cases**:
- Generating code from external sources (protobuf, schemas)
- Compiling C/C++ libraries (FFI bindings)
- Platform-specific configuration detection
- Embedding build metadata (git hash, version, build time)
- Validating environment prerequisites

❌ **Avoid for**:
- Logic that could be done at runtime
- Heavy computations (slows every build)
- Operations that could use proc macros instead
- Anything that makes builds non-reproducible

## Build Script Structure

### Basic Template

```rust
// build.rs
fn main() {
    // Rerun build script only when these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=schema.proto");
    
    // Set environment variable for compile time
    println!("cargo:rustc-env=BUILD_TIME={}", now());
    
    // Add link directive for C library
    println!("cargo:rustc-link-lib=static=mylib");
    
    // Set compiler flags
    println!("cargo:rustc-cfg=feature=\"custom\"");
}
```

### Common Cargo Directives

| Directive | Purpose | Example |
|-----------|---------|---------|
| `cargo:rerun-if-changed=PATH` | Trigger rebuild only if file changes | `cargo:rerun-if-changed=proto/api.proto` |
| `cargo:rerun-if-env-changed=VAR` | Rebuild if env var changes | `cargo:rerun-if-env-changed=CC` |
| `cargo:rustc-env=KEY=VALUE` | Set env var for compile time | `cargo:rustc-env=GIT_HASH=abc123` |
| `cargo:rustc-cfg=KEY="VALUE"` | Enable `#[cfg(KEY)]` in code | `cargo:rustc-cfg=has_sse="true"` |
| `cargo:rustc-link-lib=TYPE=NAME` | Link external library | `cargo:rustc-link-lib=static=ssl` |
| `cargo:rustc-link-search=PATH` | Add library search path | `cargo:rustc-link-search=/usr/local/lib` |
| `cargo:warning=MESSAGE` | Emit compiler warning | `cargo:warning=OpenSSL not found` |

## Best Practices

### 1. Minimize Rebuild Triggers

**Problem**: Build scripts rerun on every build by default.

**Solution**: Use `rerun-if-changed` to specify exact dependencies:

```rust
fn main() {
    // Only rerun if these specific files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/codegen/schema.json");
    println!("cargo:rerun-if-env-changed=TARGET");
    
    // Now won't rerun on unrelated changes!
}
```

**Impact**: Faster incremental builds (seconds vs minutes for large projects).

### 2. Handle Errors Gracefully

**Problem**: Build script panics are cryptic.

**Solution**: Use `Result` and `?` operator with clear error messages:

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let proto = std::fs::read_to_string("schema.proto")
        .map_err(|e| format!("Failed to read schema.proto: {}", e))?;
    
    generate_code(&proto)?;
    
    Ok(())
}
```

### 3. Make Builds Reproducible

**Problem**: Different build outputs on different machines.

**Avoid**:
```rust
// ❌ Non-reproducible: depends on local time
println!("cargo:rustc-env=BUILD_TIME={}", chrono::Local::now());

// ❌ Non-reproducible: depends on file system order
for entry in std::fs::read_dir("src")? {
    // File system order is unspecified!
}
```

**Better**:
```rust
// ✅ Reproducible: use UTC time or SOURCE_DATE_EPOCH
let build_time = std::env::var("SOURCE_DATE_EPOCH")
    .unwrap_or_else(|_| chrono::Utc::now().timestamp().to_string());

// ✅ Reproducible: sort entries
let mut entries: Vec<_> = std::fs::read_dir("src")?.collect();
entries.sort_by_key(|e| e.as_ref().unwrap().path());
```

### 4. Use `OUT_DIR` for Generated Files

**Problem**: Where to put generated code?

**Solution**: Use Cargo's `OUT_DIR` environment variable:

```rust
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let gen_file = out_dir.join("generated.rs");
    
    std::fs::write(&gen_file, "pub const FOO: u32 = 42;").unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
}
```

**Usage in code**:
```rust
// src/lib.rs
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn use_generated() -> u32 {
    FOO  // From generated.rs
}
```

### 5. Conditional Compilation for Features

**Enable `#[cfg(...)]` based on build-time checks**:

```rust
fn main() {
    // Check if OpenSSL is available
    if pkg_config::probe_library("openssl").is_ok() {
        println!("cargo:rustc-cfg=has_openssl");
    }
    
    // Check CPU features
    if std::env::var("TARGET").unwrap().contains("x86_64") {
        println!("cargo:rustc-cfg=has_sse2");
    }
}
```

**Usage**:
```rust
#[cfg(has_openssl)]
mod ssl_backend;

#[cfg(has_sse2)]
fn optimized_compute() { /* SIMD version */ }
```

### 6. Separate Build Script Dependencies

Build scripts have their own dependency graph:

```toml
# Cargo.toml
[dependencies]
serde = "1.0"  # Used by main crate

[build-dependencies]
cc = "1.0"           # Only for build.rs
bindgen = "0.69"     # Generate FFI bindings
prost-build = "0.12" # Compile protobuf
```

**Why separate?**
- Build tools don't bloat final binary
- Different optimization needs (build scripts run once)
- Faster clean builds (don't recompile unused deps)

### 7. Emit Useful Build Information

```rust
fn main() {
    // Git commit hash (great for version tracking)
    let git_hash = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());
    
    println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
    println!("cargo:rerun-if-changed=.git/HEAD");
    
    // Build profile
    let profile = std::env::var("PROFILE").unwrap();
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);
    
    // Target triple
    let target = std::env::var("TARGET").unwrap();
    println!("cargo:rustc-env=BUILD_TARGET={}", target);
}
```

**Access in code**:
```rust
pub const GIT_HASH: &str = env!("GIT_HASH");
pub const BUILD_PROFILE: &str = env!("BUILD_PROFILE");

pub fn version_info() -> String {
    format!("{} ({} build, git {})", 
        env!("CARGO_PKG_VERSION"),
        BUILD_PROFILE,
        GIT_HASH)
}
```

### 8. Platform-Specific Configuration

```rust
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-cfg=has_windows_sockets");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-cfg=has_pthreads");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
        }
        _ => {}
    }
}
```

## Common Patterns

### Pattern 1: Code Generation from Schema

```rust
// build.rs
use std::path::PathBuf;

fn main() {
    let proto_path = "proto/api.proto";
    
    // Rerun only if schema changes
    println!("cargo:rerun-if-changed={}", proto_path);
    
    // Generate Rust code
    prost_build::compile_protos(&[proto_path], &["proto/"])
        .expect("Failed to compile protobuf");
}
```

### Pattern 2: Compiling C/C++ Code

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/native/crypto.c")
        .include("src/native/include")
        .flag("-O3")
        .compile("crypto");
    
    println!("cargo:rerun-if-changed=src/native/crypto.c");
    println!("cargo:rerun-if-changed=src/native/include/crypto.h");
}
```

### Pattern 3: Embedding Build Metadata

```rust
// build.rs
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("build_info.rs");
    
    let mut f = std::fs::File::create(&dest).unwrap();
    
    writeln!(f, "pub const BUILD_TIME: &str = \"{}\";", 
             chrono::Utc::now().to_rfc3339()).unwrap();
    
    writeln!(f, "pub const RUSTC_VERSION: &str = \"{}\";",
             rustc_version_runtime::version()).unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
}
```

## Performance Considerations

### Build Script Cost

**Measurement**: Build scripts add overhead to every clean build.

```bash
# Measure build script time
cargo clean
cargo build --timings

# Check build-script-build in report
```

**Optimization strategies**:
1. Cache expensive computations (write to `OUT_DIR`, check if exists)
2. Use `rerun-if-changed` aggressively
3. Move logic to proc macros if run-time generation acceptable
4. Parallelize independent tasks

### Example: Caching Generated Code

```rust
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let cache_file = out_dir.join("generated_cache.rs");
    
    // Check if cached version exists and is up-to-date
    let schema_modified = std::fs::metadata("schema.proto")
        .unwrap()
        .modified()
        .unwrap();
    
    let cache_valid = cache_file.exists() 
        && std::fs::metadata(&cache_file)
            .unwrap()
            .modified()
            .unwrap() > schema_modified;
    
    if !cache_valid {
        // Regenerate only when needed
        generate_code(&cache_file);
    }
    
    println!("cargo:rerun-if-changed=schema.proto");
}
```

## Debugging Build Scripts

### Enable Verbose Output

```bash
cargo build -vv  # Very verbose, shows build script output
```

### Common Issues

**Issue 1**: Build script runs every time
- **Cause**: Missing `rerun-if-changed`
- **Fix**: Add explicit triggers

**Issue 2**: Generated code not found
- **Cause**: Wrong path to `OUT_DIR`
- **Fix**: Use `concat!(env!("OUT_DIR"), "/file.rs")`

**Issue 3**: Link errors
- **Cause**: Wrong library name or search path
- **Fix**: Verify with `pkg-config --libs libname`

**Issue 4**: Non-reproducible builds
- **Cause**: Using local time, file system order, etc.
- **Fix**: Use UTC time, sort collections, use `SOURCE_DATE_EPOCH`

## Related Concepts

- **Procedural Macros**: Alternative for compile-time code generation (can access crate context)
- **Feature Flags**: Simpler alternative for conditional compilation
- **Environment Variables**: Cargo provides many (`TARGET`, `PROFILE`, `OUT_DIR`, etc.)
- **Cross-compilation**: Build scripts must handle different targets

## Real-World Examples

### Example 1: Embedding Git Hash

```rust
// build.rs
fn main() {
    let output = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rerun-if-changed=.git/HEAD");
}

// src/lib.rs
pub const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("GIT_HASH"),
    ")"
);
```

### Example 2: Platform Detection

```rust
// build.rs
fn main() {
    let target = std::env::var("TARGET").unwrap();
    
    if target.contains("x86_64") {
        println!("cargo:rustc-cfg=has_x86_64");
    }
    
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dl");
    }
}

// src/lib.rs
#[cfg(has_x86_64)]
pub fn optimized() {
    // x86_64 SIMD code
}
```

## Key Takeaways

1. **Use `rerun-if-changed`** to avoid unnecessary rebuilds
2. **Write to `OUT_DIR`** for generated code
3. **Make builds reproducible** (avoid local time, FS order)
4. **Separate build dependencies** from runtime dependencies
5. **Handle errors clearly** with `Result<(), Box<dyn Error>>`
6. **Cache expensive operations** when possible
7. **Emit build metadata** for debugging and version tracking

## Rust Implementations

- [[rust_for_rustaceans/Ch05]] - Project configuration and build scripts chapter
- Example: `rust_for_rustaceans/Ch05/examples/day3_configuration.rs`

## Links

**Related Zettelkasten Pages**:
- [[compiler-optimization-levels]] - How build profiles affect optimization
- [[cargo-features-best-practices]] - Feature flags vs build scripts
- [[proc-macros-vs-build-scripts]] - When to use each

**External Resources**:
- [Cargo Book: Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
- [The `cc` crate](https://docs.rs/cc/) - Compiling C/C++
- [The `bindgen` crate](https://docs.rs/bindgen/) - Auto-generate FFI bindings

---

*Last Updated*: 2026-01-27  
*Quality*: 🟢 Comprehensive - Covers patterns, best practices, performance, debugging
