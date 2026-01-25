
# 📘 Rust for Rustaceans — Chapter 5: Project Structure

**Zettelkasten**: [[rust-for-rustaceans]] — Rustaceans book knowledge hub  
**Related**: [[feature-flags-design-patterns]] | [[workspace-dependency-management]] | [[msrv-and-versioning-strategies]]

---

## 🧩 Chapter Overview

This chapter focuses on tools provided by Rust and Cargo to manage project complexity, configuration, and dependencies as a codebase grows. It covers five major areas: Features, Workspaces, Project Configuration, Conditional Compilation, and Versioning.

**Key Takeaway**: As projects grow, proper structuring becomes essential for maintainability, compilation speed, and user flexibility.

---

## I. Features

**Features are build flags** used to customize crates by enabling optional dependencies or functionality.

### The Additive Principle

Features should be **additive** — enabling a feature should not break code that compiled without it. Mutually exclusive features should be avoided because Cargo compiles dependencies using the **union of all requested features** across the dependency tree.

**Why it matters**: If crate A enables feature `x` and crate B enables feature `y`, and both depend on crate C, then crate C gets compiled with **both** features enabled.

### Defining Features

Features are defined in `Cargo.toml`:

```toml
[features]
default = ["std"]  # Enabled by default
std = ["dep:std-dependent-crate"]
advanced = ["feature-x", "feature-y"]
experimental = []

[dependencies]
std-dependent-crate = { version = "1.0", optional = true }
serde = { version = "1.0", optional = true }  # Creates "serde" feature automatically
```

**Key Points**:
- **Optional dependencies** automatically generate a feature of the same name
- Features can enable other features or dependencies
- **Default features**: Consumers can opt out using `default-features = false` to reduce compile times

### Using Features in Code

```rust
// Conditional compilation
#[cfg(feature = "advanced")]
fn advanced_operation() {
    // Only compiled when "advanced" feature is enabled
}

// Runtime check (though dead code elimination usually removes unused branch)
if cfg!(feature = "experimental") {
    experimental_code();
}
```

**Syntax**:
- `#[cfg(feature = "name")]`: Conditionally compiles the annotated item
- `cfg!(feature = "name")`: Evaluates to a boolean at compile time

---

## II. Workspaces

**Workspaces** allow splitting large projects into multiple crates (subcrates) to improve compilation times and organization.

### Structure

A top-level `Cargo.toml` defines the workspace:

```toml
[workspace]
members = [
    "crate-a",
    "crate-b",
    "utilities/*",
]
resolver = "2"  # Use version 2 feature resolver
```

### Shared Resources

- **Single `Cargo.lock`**: All member crates share one lock file
- **Single `target/` directory**: Shared dependencies compiled only once
- **Massive compilation speed improvements** for multi-crate projects

### Dependencies Between Subcrates

Subcrates can depend on each other using path dependencies:

```toml
[dependencies]
crate-a = { path = "../crate-a" }
```

**Best Practice**: For public subcrates, use version dependencies normally:

```toml
[dependencies]
crate-a = "1.0"  # Normal version

[dev-dependencies]
crate-a = { path = "../crate-a" }  # Path for local development
```

This ensures published crates don't have broken path dependencies.

---

## III. Project Configuration

### Crate Metadata

Control which files are packaged during publishing:

```toml
[package]
include = ["src/**/*", "LICENSE", "README.md"]
# OR
exclude = ["tests/fixtures/*", "benches/large-data/*"]
```

**Tip**: Use `cargo package --list` to see what will be included.

### Build Configuration

#### Patching Dependencies

The `[patch]` section allows **overriding a dependency source** globally:

```toml
[patch.crates-io]
tokio = { git = "https://github.com/tokio-rs/tokio", branch = "main" }
# OR
serde = { path = "../serde-fix" }
```

**Use Cases**:
- Testing bug fixes in transitive dependencies
- Using unreleased versions from Git
- Local development with forked dependencies

**Critical**: Patches apply to the **entire dependency tree**, not just direct dependencies.

#### Profiles

The `[profile]` section controls compiler options for different build modes:

```toml
[profile.dev]
opt-level = 0          # No optimization (fast compile)

[profile.release]
opt-level = 3          # Maximum optimization
lto = true            # Link-Time Optimization (slower build, faster runtime)
codegen-units = 1     # Single codegen unit (better optimization, slower compile)
panic = "abort"       # Smaller binary, no stack unwinding
```

**Key Settings**:
- **`opt-level`**: Optimization aggressiveness (0-3, or "s"/"z" for size)
- **`codegen-units`**: Parallel compilation vs. optimization quality tradeoff
- **`lto`**: Link-Time Optimization (enables cross-crate optimizations)
- **`panic`**: `"unwind"` (default, allows cleanup) or `"abort"` (immediate exit, smaller binary)

**Performance Tips**:
- Use `lto = "thin"` for faster builds with most LTO benefits
- Set `opt-level = 1` in dev profile for dependencies to speed up debug builds

---

## IV. Conditional Compilation

Mechanisms to compile code only under specific conditions.

### Syntax

```rust
// Attribute form
#[cfg(target_os = "linux")]
fn linux_specific() { }

// Expression form
if cfg!(target_arch = "x86_64") {
    x86_optimized_path();
}

// Combinators
#[cfg(all(unix, target_pointer_width = "64"))]
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg(not(target_env = "msvc"))]
```

### Common Conditions

| Condition | Purpose | Examples |
|-----------|---------|----------|
| `feature = "name"` | Feature flags | `feature = "std"` |
| `target_os` | Operating system | `"linux"`, `"windows"`, `"macos"` |
| `target_arch` | CPU architecture | `"x86_64"`, `"aarch64"`, `"wasm32"` |
| `target_endian` | Byte order | `"little"`, `"big"` |
| `test` | Test compilation | Only set during `cargo test` |
| `debug_assertions` | Debug mode | Enabled in dev builds |

### Conditional Dependencies

Target-specific dependencies:

```toml
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

---

## V. Versioning

Rust follows **Semantic Versioning (SemVer)**: `MAJOR.MINOR.PATCH`

### Minimum Supported Rust Version (MSRV)

**The Challenge**: Balance between using new features and supporting users on older compilers.

**Strategies**:
1. **Explicit policy**: "Latest stable minus 2 releases"
2. **Version bumping**: Bump minor version when MSRV increases
3. **Testing**: Use CI to test against MSRV (e.g., with `rust-version` field)

```toml
[package]
rust-version = "1.70"  # Requires Rust 1.70 or newer
```

**Community Best Practice**: Document MSRV policy in README and test it in CI.

### Minimal Dependency Versions

**The Problem**: `version = "1.7.3"` means `>= 1.7.3`, but Cargo defaults to the **latest** version.

**The Solution**: Test with `-Zminimal-versions` to ensure your crate actually compiles with the lowest version listed:

```bash
cargo +nightly update -Zminimal-versions
cargo build
```

This catches cases where you accidentally use APIs from newer versions.

### Changelogs

**Highly recommended**: Keep a manual changelog rather than dumping git logs.

**Format**: Follow [Keep a Changelog](https://keepachangelog.com/):

```markdown
## [Unreleased]
### Added
- New feature X

## [2.0.0] - 2024-01-15
### Breaking
- Changed API for Y

### Added
- Support for Z
```

### Unreleased Versions

**Critical for Git dependencies**: Immediately after a release, bump the version with a pre-release suffix:

```toml
# After releasing 2.0.3, immediately bump to:
version = "2.0.4-alpha.1"
```

**Why?** This allows Cargo to detect breaking changes in Git dependencies correctly. Without this, Cargo can't tell if a Git dependency has changed.

**Workflow**:
1. Release `2.0.3`
2. Immediately commit: `version = "2.0.4-alpha.1"`
3. Continue development
4. Before next release, change to: `version = "2.0.4"`

---

## 🎯 Key Takeaways

1. **Features are additive** — never make them mutually exclusive
2. **Workspaces scale** — use them for large multi-crate projects
3. **Profiles optimize** — tune compilation for dev vs. release
4. **Conditional compilation** — target-specific code without runtime cost
5. **MSRV matters** — document and test your minimum Rust version
6. **Test minimal versions** — don't assume latest dependency versions
7. **Maintain changelogs** — users need clear upgrade paths
8. **Version discipline** — use pre-release versions for unreleased work

---

## 🔗 Related Concepts

- **Rust Book Ch7**: Modules and visibility
- **Rust Book Ch14**: More about Cargo and crates.io
- **Mission Integration**: Apply workspace patterns to `rust_study` multi-crate structure
- **Build Scripts**: `build.rs` for complex build-time logic (covered briefly in book section)

---

## 📝 Practice Applications

### From `rust_study` Workspace

Our workspace demonstrates many Ch5 concepts:

```toml
[workspace]
members = [
    "missions/Mission*",
    "tutorials/Mission*_tut",
    "rust_book/Ch*/*",
    "advent_of_code/aoc*",
]
```

**Observations**:
- Single `Cargo.lock` for all 80+ crates
- Shared compilation of common dependencies (`anyhow`, `criterion`)
- Path dependencies between missions and tutorials

### Project Euler Integration

Consider using features for Project Euler solutions:

```toml
[features]
default = ["problem-1-to-10"]
problem-1-to-10 = []
problem-11-to-20 = []
all-problems = ["problem-1-to-10", "problem-11-to-20"]
```

This allows selective compilation when you have hundreds of problems.

---

*Last Updated: 2026-01-25*  
*Status: Comprehensive summary for Week 5 study*
