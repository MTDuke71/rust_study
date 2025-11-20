# Cargo Release Profiles

*Date: 2025-11-19*
*Tags: #rust #cargo #optimization #build-configuration #performance*

---

## Overview

Cargo profiles control compilation settings for different build scenarios. Profiles define optimization levels, debug information, and other compilation parameters that affect binary size, compile time, and runtime performance.

## Core Concepts

### Main Profiles

1. **dev** - Development builds (`cargo build`)
   - Fast compilation with debug symbols
   - No optimization (opt-level = 0)
   - Debug assertions and overflow checks enabled
   - Incremental compilation for faster rebuilds

2. **release** - Production builds (`cargo build --release`)
   - Maximum optimization (opt-level = 3)
   - No debug symbols by default
   - No runtime checks (faster execution)
   - Single codegen unit for better optimization

3. **test** - Test builds (`cargo test`)
   - Similar to dev but for test execution

4. **bench** - Benchmark builds (`cargo bench`)
   - Similar to release but for benchmarks

### Workspace vs Package Configuration

**Critical Rule**: In a workspace, profiles MUST be defined at the workspace root `Cargo.toml`, not in individual packages.

```toml
# ✅ Correct: workspace_root/Cargo.toml
[profile.release]
opt-level = 3

# ❌ Wrong: workspace_root/package/Cargo.toml
# Generates "profiles will be ignored" warning
```

## Key Profile Settings

### Optimization Levels

| Level | Description | Compile Time | Binary Size | Performance |
|-------|-------------|--------------|-------------|-------------|
| `0` | No optimization | Fastest | Largest | Slowest |
| `1` | Basic optimization | Fast | Large | Moderate |
| `2` | Some optimization | Moderate | Medium | Good |
| `3` | Full optimization | Slow | Small | Best |
| `"s"` | Optimize for size | Slow | Smaller | Good |
| `"z"` | Aggressively optimize for size | Slowest | Smallest | Good |

### Link-Time Optimization (LTO)

```toml
lto = false      # No LTO (fastest linking)
lto = true       # LTO within crate only
lto = "thin"     # Thin LTO (balance)
lto = "fat"      # Full LTO (slowest, best optimization)
```

**Trade-off**: LTO improves runtime performance but significantly increases compile time.

### Codegen Units

```toml
codegen-units = 256   # Maximum parallelism (dev default)
codegen-units = 1     # No parallelism (release default)
```

**Trade-off**: More units = faster compilation but less optimization opportunity.

### Debug Information

```toml
debug = false    # No debug symbols (smaller binary)
debug = true     # Full debug symbols
debug = 1        # Line tables only
debug = 2        # Full debug info
```

## Common Custom Profiles

### Release with Debug Info (Profiling)

```toml
[profile.release-with-debug]
inherits = "release"
debug = true           # Keep symbols for perf/flamegraph
```

Usage: `cargo build --profile release-with-debug`

### Minimum Binary Size

```toml
[profile.release-min-size]
inherits = "release"
opt-level = "z"        # Optimize for size
lto = true
codegen-units = 1
strip = true           # Strip symbols
panic = 'abort'        # Smaller panic handler
```

### Maximum Performance

```toml
[profile.release-max-perf]
inherits = "release"
opt-level = 3
lto = "fat"            # Full LTO
codegen-units = 1
```

## Real-World Applications

### Development Workflow
- Use **dev** profile for rapid iteration
- Optionally set `opt-level = 1` for better dev performance

### CI/CD Pipeline
- Use **release** profile with `lto = true`
- Run tests in **release** mode to catch optimization bugs

### Embedded/WebAssembly
- Use **release-min-size** profile
- Critical for size-constrained environments

### Performance-Critical Applications
- Use **release-max-perf** profile
- Accept longer compile times for maximum runtime speed

## Integration with Other Concepts

### Rust Book Chapter 14.1 Connection
See [[rust-book-ch14]] for detailed examples and demonstrations.
- Example location: `rust_book/Ch14/release_profiles/`
- Comprehensive guide: `PROFILES.md` in package directory
- Shows impact of optimization levels on performance

### Performance Patterns
Profiles directly impact [[Performance Patterns]] through:
- Optimization level choices
- LTO enabling cross-crate optimizations
- Codegen units affecting compile-time parallelism

### Binary Size Optimization
Profile settings are primary tool for [[binary-size-optimization]]:
- `opt-level = "z"` for aggressive size reduction
- `strip = true` to remove debug symbols
- `panic = 'abort'` for smaller panic handler

### Debugging and Profiling
Profile configuration affects [[debugging-rust]] workflow:
- Debug symbols needed for debuggers (gdb, lldb)
- Release with debug for profiling tools (perf, flamegraph)
- Balance between optimization and debuggability

## Common Pitfalls

1. **Defining profiles in package Cargo.toml**
   - Always use workspace root in workspaces
   - Causes "profiles will be ignored" warnings

2. **Not testing in release mode**
   - Optimization can expose bugs (overflow, undefined behavior)
   - Always run `cargo test --release` before shipping

3. **Excessive LTO in development**
   - Drastically slows incremental builds
   - Reserve "fat" LTO for final release builds

4. **Forgetting strip = true for production**
   - Debug symbols significantly increase binary size
   - Use strip for deployed binaries

5. **One-size-fits-all optimization**
   - Different targets need different profiles
   - Create custom profiles for specific use cases

## Best Practices

1. **Development**: Keep default dev profile for fast iteration
2. **CI**: Use release profile with LTO for final builds
3. **Profiling**: Create release-with-debug custom profile
4. **Size-critical**: Use opt-level "z" with strip and abort
5. **Documentation**: Document custom profiles in project README

## Performance Impact

### Compile Time vs Runtime Trade-offs

| Configuration | Compile Time | Runtime Performance |
|---------------|--------------|---------------------|
| dev (opt-level 0) | ~5s | Baseline (100%) |
| dev (opt-level 1) | ~8s | 2-3x faster |
| release (opt-level 3) | ~15s | 10-20x faster |
| release + LTO "fat" | ~45s | 15-30x faster |

*Note: Times are illustrative and vary by project*

### Binary Size Impact

```
opt-level 0:              5.2 MB
opt-level 3:              1.8 MB
opt-level "z":            1.2 MB
opt-level "z" + strip:    800 KB
```

## Related Commands

```bash
# Build with specific profile
cargo build --profile release-with-debug

# Show active profile settings
cargo build --release --verbose

# Benchmarking (uses bench profile)
cargo bench

# Test with release optimizations
cargo test --release
```

## Further Learning

- Rust Book Ch14.1: Customizing builds with profiles
- [Cargo Reference: Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [[rust-optimization-techniques]] for advanced patterns

---

## Links

*Incoming Links:*
- [[rust-book-ch14]] - Chapter covering profile customization
- [[Performance Patterns]] - Uses profiles for optimization
- [[binary-size-optimization]] - Profile settings for size reduction
- [[debugging-rust]] - Debug symbol configuration
- [[cargo-workspace-management]] - Workspace-level profile configuration

*Outgoing Links:*
- [[rust-optimization-techniques]] - Advanced optimization strategies
- [[cross-compilation]] - Profiles for different targets
- [[rust-toolchain-configuration]] - Related build configuration
- [[CI-CD-rust]] - Profile usage in automated pipelines
- [[embedded-rust]] - Size-critical profile configurations

*Related Concepts:*
- [[compiler-flags]] - Lower-level compilation control
- [[link-time-optimization]] - Deep dive into LTO
- [[incremental-compilation]] - Dev profile feature
- [[panic-strategies]] - Unwind vs abort trade-offs
- [[symbol-stripping]] - Binary size reduction technique

*Tags: #rust #cargo #optimization #build-configuration #performance #compilation #profiling*
