# 🛠️ Rust Development Tools

*Essential toolchain and utilities for professional Rust development*

---

## 📋 Core Concept

**Definition**: Rust's development toolchain provides a comprehensive set of integrated tools for building, testing, documenting, and maintaining Rust projects with professional engineering standards.

**Philosophy**: "Batteries included" - Rust ships with production-quality tools that enforce best practices and enable confident development.

**Key Principle**: All tools integrate seamlessly through `cargo`, providing consistent interfaces and workflows.

---

## 🔧 Core Toolchain Components

### **1. rustc - The Rust Compiler**

**Purpose**: Transforms Rust source code into optimized machine code

**Key Features**:
- Ownership and borrow checking at compile time
- Zero-cost abstractions with aggressive optimization
- Detailed error messages with actionable suggestions
- Cross-compilation support for multiple targets

```bash
# Direct compiler usage (rarely needed - use cargo instead)
rustc main.rs                    # Compile single file
rustc --edition 2021 main.rs     # Specify Rust edition
rustc -O main.rs                 # Optimized build
rustc --explain E0308            # Explain error code
```

**Best Practice**: Use `cargo build` instead of invoking `rustc` directly for proper dependency management.

---

### **2. cargo - Build System & Package Manager**

**Purpose**: Unified interface for building, testing, and managing Rust projects

**Essential Commands**:

```bash
# Project Management
cargo new my_project             # Create new binary project
cargo new --lib my_lib           # Create new library project
cargo init                       # Initialize in existing directory

# Build & Run
cargo build                      # Debug build
cargo build --release            # Optimized release build
cargo run                        # Build and run binary
cargo run --example demo         # Run specific example
cargo check                      # Fast syntax/type check without codegen

# Testing & Quality
cargo test                       # Run all tests
cargo test --lib                 # Unit tests only
cargo test --workspace           # Test all workspace members
cargo bench                      # Run benchmarks

# Documentation
cargo doc                        # Generate documentation
cargo doc --open                 # Generate and open in browser

# Dependency Management
cargo add serde                  # Add dependency (cargo-edit)
cargo update                     # Update dependencies
cargo tree                       # Show dependency tree

# Publishing
cargo publish                    # Publish to crates.io
cargo yank --vers 1.0.1          # Yank published version
```

**Workspace Support**: This repository uses workspace configuration in root `Cargo.toml` with 80+ crate members.

---

### **3. rustfmt - Code Formatter**

**Purpose**: Automatic code formatting following Rust style guidelines

**Usage**:

```bash
cargo fmt                        # Format all workspace files
cargo fmt --all                  # Explicit workspace formatting
cargo fmt --check                # Check without modifying files
cargo fmt -- --check             # CI-friendly check mode
```

**Configuration** (`rustfmt.toml` or `.rustfmt.toml`):

```toml
edition = "2021"
max_width = 100
hard_tabs = false
tab_spaces = 4
```

**This Workspace**: Uses `cargo fmt --all --check` in pre-commit validation and nightly CI workflows.

---

### **4. clippy - Linter**

**Purpose**: Catch common mistakes and enforce Rust idioms through 500+ lint rules

**Usage**:

```bash
cargo clippy                              # Run on current package
cargo clippy --workspace                  # Check all workspace members
cargo clippy -- -D warnings               # Treat warnings as errors
cargo clippy -- -W clippy::all            # Enable all clippy lints
cargo clippy --fix                        # Auto-fix issues where possible
```

**Common Lint Categories**:
- **Correctness**: Bugs and logic errors
- **Style**: Idiomatic Rust patterns
- **Complexity**: Overly complex code
- **Performance**: Inefficient code patterns
- **Pedantic**: Extra-strict suggestions

**This Workspace Policy**:
- **Zero warnings tolerance**: `cargo clippy --workspace -- -D warnings` must pass
- Runs in nightly CI: `.github/workflows/nightly-clippy.yml`
- Uses `#[allow(dead_code)]` for demonstration code in `daily_study/`

**See**: [`.github/CLIPPY_AUTOMATION.md`](../.github/CLIPPY_AUTOMATION.md) for workspace clippy workflow

---

### **5. rust-analyzer - Language Server**

**Purpose**: IDE support providing real-time diagnostics, completion, and refactoring

**Features**:
- ✅ Real-time error checking as you type
- ✅ Intelligent code completion
- ✅ Go-to-definition and find references
- ✅ Inline type hints and documentation
- ✅ Refactoring tools (rename, extract function)
- ✅ Macro expansion viewing

**VS Code Integration**:
- Extension: `rust-lang.rust-analyzer`
- Configured via `settings.json`

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.allFeatures": true,
  "rust-analyzer.inlayHints.typeHints.enable": true
}
```

---

### **6. rustdoc - Documentation Generator**

**Purpose**: Generate HTML documentation from Rust source code and doc comments

**Documentation Comments**:

```rust
/// Outer documentation for items
///
/// # Examples
///
/// ```
/// let result = my_function(42);
/// assert_eq!(result, 84);
/// ```
pub fn my_function(x: i32) -> i32 {
    x * 2
}

//! Inner documentation for modules/crates
```

**Commands**:

```bash
cargo doc                        # Generate docs
cargo doc --open                 # Generate and open in browser
cargo doc --no-deps              # Skip dependency documentation
cargo doc --document-private-items  # Include private items
```

**This Workspace Standard**: All public APIs must have rustdoc with examples (see mission READMEs).

---

## 🚀 Additional Cargo Tools

### **cargo-edit** - Enhanced Dependency Management

```bash
# Install
cargo install cargo-edit

# Usage
cargo add serde --features derive    # Add dependency with features
cargo rm serde                       # Remove dependency
cargo upgrade                        # Upgrade dependencies interactively
```

---

### **cargo-watch** - Auto-Rebuild on Changes

```bash
# Install
cargo install cargo-watch

# Usage
cargo watch -x check                 # Run check on file changes
cargo watch -x test                  # Run tests on changes
cargo watch -x "run --example demo"  # Run example on changes
```

---

### **cargo-audit** - Security Vulnerability Scanning

```bash
# Install
cargo install cargo-audit

# Usage
cargo audit                          # Check for known vulnerabilities
cargo audit fix                      # Auto-update vulnerable dependencies
```

---

### **cargo-expand** - Macro Expansion Viewer

```bash
# Install
cargo install cargo-expand

# Usage
cargo expand                         # Expand all macros in lib
cargo expand module::function        # Expand specific item
```

---

### **cargo-criterion** - Advanced Benchmarking

```bash
# Criterion already used in missions for benchmarks
cargo bench                          # Run benchmarks
cargo bench -- --save-baseline main  # Save performance baseline
cargo bench -- --baseline main       # Compare to baseline
```

**This Workspace**: Mission 5, 6, 7+ include `benches/` directories with Criterion benchmarks.

---

### **tarpaulin** - Code Coverage

```bash
# Install
cargo install cargo-tarpaulin

# Usage
cargo tarpaulin --out Html           # Generate HTML coverage report
cargo tarpaulin --workspace          # Coverage for all workspace members
```

**This Workspace**: Quality pipeline generates coverage reports in `coverage/` directory.

---

## 📊 Workspace-Specific Tool Usage

### **This Repository's Tool Integration**

```bash
# Quality Pipeline (scripts/quality-pipeline.ps1)
cargo fmt --all --check              # ✅ Formatting check
cargo clippy --workspace -- -D warnings  # ✅ Zero warnings policy
cargo test --workspace               # ✅ All tests pass
cargo build --workspace              # ✅ Full workspace build

# Nightly Automation
.github/workflows/nightly-clippy.yml              # Scheduled clippy
.github/workflows/nightly-comprehensive-tests.yml # Full test suite
```

---

### **Development Workflow**

```bash
# 1. Feature Development
cargo new feature_crate              # Create new crate
# Add to workspace Cargo.toml members = [...]

# 2. Iterative Development
cargo check                          # Fast feedback loop
cargo test                           # Run tests frequently
cargo clippy                         # Check for issues

# 3. Pre-Commit Validation
cargo fmt --all                      # Format code
cargo clippy --workspace -- -D warnings  # Lint check
cargo test --workspace               # Full test suite
cargo build --workspace              # Verify builds

# 4. Documentation
cargo doc --open                     # Review generated docs
# Ensure all public APIs documented
```

---

## 🎯 Tool Selection Guide

### **When to Use Each Tool**

| **Tool** | **Use Case** | **Frequency** |
|----------|--------------|---------------|
| `cargo check` | Quick syntax validation during coding | Constantly (save-on-type) |
| `cargo build` | Compile for testing locally | After feature completion |
| `cargo test` | Run unit/integration tests | After each logical change |
| `cargo clippy` | Catch idioms and potential bugs | Before commits |
| `cargo fmt` | Format code to style guide | Before commits |
| `cargo doc` | Generate documentation | Before PRs, releases |
| `cargo bench` | Performance validation | After optimization work |
| `cargo build --release` | Production builds | Release preparation |

---

## 🔍 Advanced Cargo Features

### **Workspaces**

This repository demonstrates workspace usage:

```toml
# Root Cargo.toml
[workspace]
members = [
    "missions/Mission1",
    "missions/Mission2",
    # ... 80+ members
    "rust_book/Ch16/*",
    "rust_book/Ch17/*",
]

[workspace.dependencies]
# Shared dependencies across workspace
```

**Benefits**:
- ✅ Single `target/` directory (saves disk space)
- ✅ Consistent dependency versions
- ✅ Unified testing and linting

---

### **Features and Conditional Compilation**

```toml
[features]
default = ["std"]
std = []
async = ["tokio", "futures"]

[dependencies]
tokio = { version = "1.0", optional = true }
```

```bash
cargo build --features async         # Enable specific feature
cargo build --all-features           # Enable all features
cargo build --no-default-features    # Minimal build
```

---

### **Cross-Compilation**

```bash
# Add target
rustup target add x86_64-pc-windows-gnu

# Build for target
cargo build --target x86_64-pc-windows-gnu
```

---

## 🧪 Testing Tools Integration

### **Test Organization**

```rust
// Unit tests (same file)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() { }
}

// Integration tests (tests/ directory)
// tests/integration_test.rs
#[test]
fn test_public_api() { }

// Documentation tests (in doc comments)
/// ```
/// assert_eq!(2 + 2, 4);
/// ```
```

**This Workspace Pattern**: Missions use `tests/unit_tests.rs` with `req{N}_*` naming convention.

---

### **Test Runners**

```bash
cargo test                           # All tests
cargo test test_name                 # Specific test
cargo test --lib                     # Unit tests only
cargo test --test integration_test   # Specific integration test
cargo test -- --nocapture            # Show println! output
cargo test -- --test-threads=1       # Sequential test execution
```

---

## 🎓 Learning Integration

### **Tool Mastery Progression**

1. **Beginner**: `cargo new`, `cargo run`, `cargo test`
2. **Intermediate**: `cargo check`, `cargo clippy`, `cargo fmt`, workspace management
3. **Advanced**: Custom build scripts, feature flags, cross-compilation, cargo extensions
4. **Expert**: Toolchain customization, cargo plugins, CI/CD integration

---

### **Related Concepts**

- [[V-Cycle Methodology]] - How tools enforce V-Cycle development in missions
- [[rust_book/rust-book-ch1]] - Getting started with cargo and rustc
- [[rust_book/rust-book-ch11]] - Testing with cargo test
- [[rust_book/rust-book-ch14]] - Cargo and crates.io publishing
- [[testing-strategies]] - Comprehensive testing patterns using cargo tools

---

### **Workspace Standards**

- [[V-Cycle in Rust Development]] - Tool integration in V-Cycle workflow
- [`.github/CLIPPY_AUTOMATION.md`](../.github/CLIPPY_AUTOMATION.md) - Clippy enforcement
- [`.github/CONTRIBUTING.md`](../.github/CONTRIBUTING.md) - Development workflow with tools
- [[../../scripts/quality-pipeline.ps1]] - Automated quality checks

---

## 📚 External Resources

### **Official Documentation**

- [Cargo Book](https://doc.rust-lang.org/cargo/) - Comprehensive cargo guide
- [Clippy Documentation](https://rust-lang.github.io/rust-clippy/) - Lint reference
- [rustfmt Guide](https://rust-lang.github.io/rustfmt/) - Formatting options
- [rust-analyzer Manual](https://rust-analyzer.github.io/manual.html) - IDE setup

---

### **Tool Repositories**

- [rust-lang/cargo](https://github.com/rust-lang/cargo) - Cargo source
- [rust-lang/rust-clippy](https://github.com/rust-lang/rust-clippy) - Clippy lints
- [rust-lang/rustfmt](https://github.com/rust-lang/rustfmt) - Formatter
- [rust-lang/rust-analyzer](https://github.com/rust-lang/rust-analyzer) - LSP

---

## 💡 Best Practices

### **Tool Usage Guidelines**

1. **Run `cargo check` frequently** - Fastest feedback during development
2. **Run `cargo clippy` before commits** - Catch issues early
3. **Run `cargo fmt` before commits** - Consistent style
4. **Run `cargo test --workspace` before push** - Ensure nothing breaks
5. **Use `cargo build --release` for benchmarks** - Accurate performance
6. **Generate docs with `cargo doc --open`** - Verify documentation quality
7. **Keep Cargo.lock in version control** - Reproducible builds

---

### **Workspace-Specific Conventions**

- ✅ Zero clippy warnings policy (`-D warnings`)
- ✅ Pre-commit: fmt check, clippy, test, build
- ✅ Nightly CI: Comprehensive quality checks
- ✅ Mission tests: Named `req{N}_*` for traceability
- ✅ Public APIs: Rustdoc with examples required
- ✅ Benchmarks: Criterion for performance validation

---

*This toolchain enables the professional engineering practices demonstrated throughout this rust_study workspace, from mission development to AoC problem solving.*

---

*Tags: #rust-toolchain #cargo #clippy #rustfmt #rust-analyzer #development-tools #build-system #testing #documentation #quality-assurance #workspace-management*

---

*Links: [[zettel-index]] | [[V-Cycle Methodology]] | [[V-Cycle in Rust Development]] | [[rust_book/rust-book-ch1]] | [[rust_book/rust-book-ch11]] | [[rust_book/rust-book-ch14]] | [[testing-strategies]] | [[Missions Overview]]*
