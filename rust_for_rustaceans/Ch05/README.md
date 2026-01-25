# Rust for Rustaceans - Chapter 5: Project Structure

**Zettelkasten**: [[rust-for-rustaceans]] | [[rfr-ch05-summary]]  
**Week 5 Study Plan**: [[weekly plans/2026-W05]]

---

## 📚 Chapter Summary

This directory contains comprehensive examples for Chapter 5 on Project Structure. See [`rfr-ch05-summary.md`](rfr-ch05-summary.md) for detailed notes.

**Chapter Topics**:
1. **Features** - Optional functionality via compile-time flags
2. **Workspaces** - Multi-crate project organization
3. **Project Configuration** - Metadata, profiles, patching
4. **Conditional Compilation** - Platform-specific code
5. **Versioning** - SemVer, MSRV, dependency management

---

## 🎯 Weekly Plan Integration

Based on **Week 5 Plan (2026-W05)** - Sunday 1/25 to Saturday 1/31:

| Day | Topic | Example | Focus |
|-----|-------|---------|-------|
| **Sunday (1/25)** | Features | `day1_features.rs` | Feature flags, additive principle, optional deps |
| **Monday (1/26)** | Workspaces | `day2_workspaces.rs` | Multi-crate organization, shared resources |
| **Tuesday (1/27)** | Configuration | `day3_configuration.rs` | Profiles, metadata, patching |
| **Wednesday (1/28)** | Conditional | `day4_conditional.rs` | Platform detection, cfg attributes |
| **Thursday (1/29)** | Versioning | `day5_versioning.rs` | SemVer, MSRV, minimal versions |
| **Friday (1/30)** | Review | `day6_integration.rs` | All concepts integrated |
| **Saturday (1/31)** | Practice | Apply to workspace | Real-world usage |

---

## 🏃 Running Examples

### Quick Start

```bash
# Run all examples in order
cargo run --example day1_features
cargo run --example day2_workspaces
cargo run --example day3_configuration
cargo run --example day4_conditional
cargo run --example day5_versioning
cargo run --example day6_integration
```

### Day 1: Features

```bash
# Basic run (default features)
cargo run --example day1_features

# No features
cargo run --example day1_features --no-default-features

# Specific features
cargo run --example day1_features --features advanced
cargo run --example day1_features --features "advanced,networking"

# All features
cargo run --example day1_features --all-features
```

**Demonstrates**:
- Feature flag definition
- Optional dependencies
- `#[cfg(feature = "name")]` syntax
- `cfg!(feature = "name")` runtime checks
- Additive principle

### Day 2: Workspaces

```bash
cargo run --example day2_workspaces
```

**Demonstrates**:
- Workspace structure
- Path dependencies
- Shared Cargo.lock and target/
- Multi-crate coordination
- Real rust_study workspace example

### Day 3: Project Configuration

```bash
# Debug build
cargo run --example day3_configuration

# Release build (see optimization impact)
cargo run --example day3_configuration --release
```

**Demonstrates**:
- Crate metadata (include/exclude)
- Build profiles (dev vs release)
- Optimization levels (opt-level)
- LTO (Link-Time Optimization)
- Panic modes (unwind vs abort)
- Codegen units
- Dependency patching

**Compare build artifacts**:
```bash
ls -lh target/debug/examples/day3_configuration
ls -lh target/release/examples/day3_configuration
```

### Day 4: Conditional Compilation

```bash
cargo run --example day4_conditional
```

**Demonstrates**:
- Platform detection (OS, architecture)
- `#[cfg(target_os = "...")]` patterns
- `cfg!()` combinators (all, any, not)
- Target-specific code
- Debug vs release differences
- Test-only code

**Cross-compilation examples**:
```bash
# (requires installing targets)
cargo build --example day4_conditional --target x86_64-pc-windows-msvc
cargo build --example day4_conditional --target x86_64-unknown-linux-gnu
cargo build --example day4_conditional --target aarch64-apple-darwin
```

### Day 5: Versioning

```bash
cargo run --example day5_versioning
```

**Demonstrates**:
- Semantic versioning (SemVer)
- Version comparison logic
- Pre-release versions
- MSRV (Minimum Supported Rust Version)
- Minimal dependency versions
- Changelog best practices
- Git workflow for versions

**Testing versioning**:
```bash
# Test minimal versions (requires nightly)
cargo +nightly update -Zminimal-versions
cargo build

# Check MSRV (requires cargo-msrv)
cargo msrv

# Semver compatibility (requires cargo-semver-checks)
cargo semver-checks
```

### Day 6: Integration

```bash
# Run with various feature combinations
cargo run --example day6_integration
cargo run --example day6_integration --all-features
cargo run --example day6_integration --release
```

**Demonstrates**:
- All Ch5 concepts together
- Feature-gated functionality
- Platform-specific resource management
- Workspace organization review
- Versioning workflow checklist
- Best practices summary

---

## 🧪 Running Tests

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run specific test
cargo test test_semver_comparisons

# Run with verbose output
cargo test -- --nocapture
```

---

## 📊 Feature Flags Reference

This crate defines several features to demonstrate Chapter 5 concepts:

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `default` | Standard library support | `std` |
| `std` | Enable std library features | `serde?/std` |
| `advanced` | Advanced functionality | `networking`, `compression` |
| `networking` | Network operations | - |
| `compression` | Data compression | - |
| `experimental` | Unstable APIs | - |

**Optional Dependencies**:
- `serde` - Automatically creates `serde` feature

---

## 🎓 Learning Objectives

By completing these examples, you will understand:

### 1. Features
- ✅ How to define features in Cargo.toml
- ✅ The additive principle and why it matters
- ✅ Optional dependencies and feature activation
- ✅ Default features and opting out
- ✅ Feature-gated code with `#[cfg(feature = "...")]`

### 2. Workspaces
- ✅ When to use workspaces vs single crate
- ✅ Workspace structure and organization
- ✅ Shared resources (Cargo.lock, target/)
- ✅ Path dependencies between crates
- ✅ Version management in workspaces

### 3. Project Configuration
- ✅ Crate metadata (include/exclude)
- ✅ Build profiles and optimization levels
- ✅ LTO and codegen-units tradeoffs
- ✅ Panic modes (unwind vs abort)
- ✅ Dependency patching with [patch]
- ✅ Performance tuning strategies

### 4. Conditional Compilation
- ✅ Platform-specific code patterns
- ✅ Architecture detection
- ✅ cfg combinators (all, any, not)
- ✅ Target-specific dependencies
- ✅ Debug vs release differences
- ✅ Test-only code

### 5. Versioning
- ✅ Semantic versioning rules
- ✅ Pre-release version workflow
- ✅ MSRV policy and testing
- ✅ Minimal dependency versions
- ✅ Changelog maintenance
- ✅ Git workflow best practices

---

## 🔗 Integration with rust_study Workspace

This chapter directly applies to the `rust_study` workspace structure:

```
rust_study/
├── Cargo.toml          # Workspace root with 80+ members
├── Cargo.lock          # Single lock file for all crates
├── target/             # Shared build directory
├── rust_for_rustaceans/
│   └── Ch05/           # ← You are here
├── missions/           # Mission1 through Mission12
├── tutorials/          # Mission*_tut companion tutorials
├── rust_book/          # Ch1-Ch17 exercises
├── advent_of_code/     # aoc2015-aoc2025
└── project_euler/      # Mathematical problems
```

**Workspace Benefits in Action**:
- ✅ 80+ crates share one `Cargo.lock`
- ✅ Common dependencies (anyhow, criterion) compiled once
- ✅ Fast incremental builds across projects
- ✅ Easy cross-crate testing and dependencies

---

## 📝 Practice Exercises

### Exercise 1: Feature Flags for Project Euler

Add feature flags to your Project Euler solutions:

```toml
[features]
default = []
problems-1-10 = []
problems-11-20 = []
all-problems = ["problems-1-10", "problems-11-20"]
```

### Exercise 2: Analyze Workspace Structure

Run these commands and observe the results:

```bash
cd d:\repos\rust_study
cargo tree                    # Dependency tree
cargo tree -i anyhow          # Reverse dependencies for anyhow
cargo tree -p mission5        # Dependencies for specific crate
```

### Exercise 3: Profile Tuning

Experiment with different profile settings:

```toml
[profile.dev.package."*"]
opt-level = 1  # Optimize dependencies in dev mode

[profile.release]
lto = "thin"          # Faster than full LTO
codegen-units = 16    # Parallel compilation
```

Measure build time and binary size differences.

### Exercise 4: Platform-Specific Code

Add platform-specific optimizations to an AoC solution:

```rust
#[cfg(target_arch = "x86_64")]
fn optimized_search(data: &[u8]) -> usize {
    // Use SIMD instructions
}

#[cfg(not(target_arch = "x86_64"))]
fn optimized_search(data: &[u8]) -> usize {
    // Generic implementation
}
```

### Exercise 5: MSRV Testing

Set up MSRV testing in GitHub Actions:

```yaml
jobs:
  msrv:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.70
      - run: cargo test --all-features
```

---

## 🛠️ Common Commands

```bash
# Build with specific features
cargo build --features advanced
cargo build --all-features
cargo build --no-default-features

# Test different configurations
cargo test --all-features
cargo test --no-default-features

# Check binary size
cargo build --release
ls -lh target/release/

# Profile comparison
cargo build                   # Debug
cargo build --release        # Release
time cargo build --release   # Measure build time

# Cross-platform builds
cargo check --target wasm32-unknown-unknown
cargo check --target x86_64-pc-windows-msvc

# Workspace operations
cargo build --workspace      # Build all crates
cargo test --workspace       # Test all crates
cargo clean                  # Clean shared target/

# Version management
cargo +nightly update -Zminimal-versions  # Test minimal versions
cargo msrv                   # Find MSRV (requires cargo-msrv)
cargo semver-checks          # Check SemVer compatibility
```

---

## 📖 Additional Resources

### Official Documentation
- [The Cargo Book - Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [The Cargo Book - Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [The Cargo Book - Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)

### Tools
- [cargo-msrv](https://github.com/foresterre/cargo-msrv) - Find MSRV
- [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) - Check compatibility
- [cargo-minimal-versions](https://github.com/rust-lang/cargo/issues/5657) - Test minimal versions

### Best Practices
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)

---

## ✅ Completion Checklist

Week 5 (1/25-1/31) completion criteria:

- [ ] **Sunday**: Run `day1_features.rs` with different feature combinations
- [ ] **Monday**: Study `day2_workspaces.rs` and analyze rust_study structure
- [ ] **Tuesday**: Run `day3_configuration.rs` in debug and release modes
- [ ] **Wednesday**: Run `day4_conditional.rs` and check platform detection
- [ ] **Thursday**: Complete `day5_versioning.rs` and understand SemVer
- [ ] **Friday**: Run `day6_integration.rs` and review all concepts
- [ ] **Saturday**: Apply Ch5 patterns to Mission 12 or Project Euler code

**Tests passing**:
- [ ] `cargo test --all-features`
- [ ] All examples compile and run successfully

**Understanding verified**:
- [ ] Can explain the additive principle
- [ ] Understand workspace benefits
- [ ] Know when to use different profile settings
- [ ] Can write platform-specific code
- [ ] Understand SemVer versioning strategy

---

## 🎯 Next Chapter Preview

**Week 6: Chapter 6 - Testing**
- Unit tests
- Integration tests
- Documentation tests
- Benchmarking with Criterion
- Test organization strategies
- Property-based testing

---

**Status**: Ready for Week 5 study (2026-01-25 to 2026-01-31)  
**Last Updated**: 2026-01-25  
**Examples**: 6 runnable demonstrations (day1-day6)
