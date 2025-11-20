# Cargo Release Profiles Guide

This guide covers Cargo's build profiles and how to customize them for different scenarios.

## Overview

Cargo has different build profiles that control compilation settings. The main profiles are:
- **dev** - Used for `cargo build` (fast compilation, debug info)
- **release** - Used for `cargo build --release` (optimized, minimal debug)
- **test** - Used for `cargo test`
- **bench** - Used for `cargo bench`

## Profile Configuration Location

⚠️ **Important**: In a workspace, profiles must be defined at the **workspace root** `Cargo.toml`, not in individual package `Cargo.toml` files.

```toml
# ✅ Correct: workspace root Cargo.toml (d:\repos\rust_study\Cargo.toml)
[profile.dev]
opt-level = 0

# ❌ Wrong: individual package Cargo.toml
# Will generate "profiles will be ignored" warning
```

## Default Profile Settings

### Dev Profile (Fast Compilation)
```toml
[profile.dev]
opt-level = 0          # No optimization - fastest compilation
debug = true           # Include debug symbols for debugging
split-debuginfo = "off"
debug-assertions = true  # Runtime checks for development
overflow-checks = true   # Check for integer overflow
lto = false            # No link-time optimization
panic = 'unwind'       # Allow panic unwinding for debugging
incremental = true     # Incremental compilation for faster rebuilds
codegen-units = 256    # Parallel code generation (faster compile)
rpath = false
```

### Release Profile (Maximum Performance)
```toml
[profile.release]
opt-level = 3          # Maximum optimization level
debug = false          # No debug symbols (smaller binary)
split-debuginfo = "off"
debug-assertions = false  # No runtime checks
overflow-checks = false   # No overflow checks
lto = false            # Link-time optimization off by default
panic = 'unwind'       # Can change to 'abort' for smaller binary
incremental = false    # No incremental compilation
codegen-units = 1      # Single unit for better optimization
rpath = false
```

## Custom Profile Examples

### Profile for Profiling (Release + Debug Info)
```toml
[profile.release-with-debug]
inherits = "release"
debug = true           # Keep debug symbols for profiling tools
```

**Usage:**
```bash
cargo build --profile release-with-debug
```

### Profile for Minimal Binary Size
```toml
[profile.release-min-size]
inherits = "release"
opt-level = "z"        # Optimize for size (smallest binary)
lto = true             # Link-time optimization
codegen-units = 1      # Single codegen unit
strip = true           # Strip symbols (Rust 1.59+)
panic = 'abort'        # Abort on panic (smaller binary)
```

**Usage:**
```bash
cargo build --profile release-min-size
```

### Profile for Maximum Performance
```toml
[profile.release-max-perf]
inherits = "release"
opt-level = 3          # Maximum optimization
lto = "fat"            # Full LTO across all crates
codegen-units = 1      # Single codegen unit for better optimization
```

**Usage:**
```bash
cargo build --profile release-max-perf
```

## Optimization Levels

| Level | Description | Use Case |
|-------|-------------|----------|
| `0` | No optimization | Development (fastest compile) |
| `1` | Basic optimization | Faster builds with some optimization |
| `2` | Some optimization | Balance between compile time and performance |
| `3` | Full optimization | Release builds (best performance) |
| `"s"` | Optimize for size | Smaller binaries, slightly slower than `3` |
| `"z"` | Aggressively optimize for size | Smallest possible binary |

## Link-Time Optimization (LTO)

```toml
lto = false            # No LTO (fastest linking)
lto = true             # LTO within crate only
lto = "thin"           # Thin LTO (faster than "fat", still good optimization)
lto = "fat"            # Full LTO across all crates (slowest, best optimization)
```

## Common Profile Customizations

### Development with Some Optimization
```toml
[profile.dev]
opt-level = 1          # Basic optimization for better runtime performance
```

### Release with Faster Linking
```toml
[profile.release]
lto = "thin"           # Faster linking than "fat" LTO
```

### Release that Aborts on Panic
```toml
[profile.release]
panic = 'abort'        # Smaller binary, no unwinding overhead
```

## Codegen Units

Controls parallel code generation:

```toml
codegen-units = 256    # Maximum parallelism (faster compile, less optimization)
codegen-units = 16     # Balanced
codegen-units = 1      # No parallelism (slower compile, better optimization)
```

**Trade-off**: More units = faster compilation but less optimization opportunity

## Profile Inheritance

Custom profiles can inherit from standard profiles:

```toml
[profile.my-custom]
inherits = "release"   # Start with release settings
opt-level = 2          # Override specific settings
debug = true
```

## Checking Current Profile Settings

```bash
# See what settings are active for a profile
cargo build --release --verbose

# Build with specific profile
cargo build --profile release-with-debug
```

## Examples in This Package

See `src/main.rs` for a program that demonstrates the impact of different optimization levels on performance.

```bash
# Compare build times and runtime performance
cargo build                    # dev profile
cargo build --release          # release profile

# Run and compare execution times
cargo run
cargo run --release
```

## Best Practices

1. **Development**: Use default `dev` profile for fast iteration
2. **CI/CD**: Use `release` profile with `lto = true` for production builds
3. **Profiling**: Use custom profile with `debug = true` and release optimizations
4. **Size-Critical**: Use `opt-level = "z"` with `lto = true` and `strip = true`
5. **Workspace**: Always define profiles at workspace root, not in packages

## Resources

- [Cargo Book: Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Cargo Reference: Profile Settings](https://doc.rust-lang.org/cargo/reference/profiles.html#profile-settings)

---

*Last Updated: November 19, 2025*
