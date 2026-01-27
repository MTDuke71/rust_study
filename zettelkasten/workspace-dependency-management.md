# Workspace Dependency Management

**Source**: Rust for Rustaceans Ch5 - Project Structure  
**Created**: January 26, 2026  
**Category**: Rust Ecosystem, Build System, Multi-Crate Patterns

## What Are Cargo Workspaces?

A **Cargo workspace** is a collection of related Rust packages (crates) that share:
- Common `Cargo.lock` file
- Build artifacts (`target/` directory)
- Dependency resolution
- Workspace-level commands

**Purpose**: Organize large projects with multiple related crates while maintaining consistent dependency versions and enabling code reuse.

## Real-World Example: rust_study Workspace

**Structure**: 80+ member workspace organized by learning track:

```toml
[workspace]
resolver = "3"  # MSRV-aware dependency resolution
members = [
    # Data structure implementations (V-Cycle engineered)
    "missions/Mission1",    # Stack
    "missions/Mission2",    # Queue
    # ... Mission10
    
    # Step-by-step learning progressions
    "tutorials/Mission1_tut",
    # ... Mission10_tut
    
    # Official Rust Book exercises by chapter
    "rust_book/Ch1/*",
    "rust_book/Ch17/*",     # Async examples
    
    # Problem solving practice
    "advent_of_code/aoc2015",
    "advent_of_code/aoc2023",
    "advent_of_code/aoc2024",
    
    # Mathematical algorithms
    "project_euler",
    
    # Advanced composition examples
    "advanced_examples/*",
]
```

**Benefits discovered**:
- ✅ Single `cargo build --workspace` compiles all 80+ crates
- ✅ Shared dependency versions across all members
- ✅ Incremental builds reuse artifacts
- ✅ Test isolation: `cargo test -p mission5`
- ✅ Modular architecture with clear ownership

## Key Workspace Patterns

### Pattern 1: Multi-Track Learning Organization

Separate logical groups as workspace members:

```
missions/       → V-Cycle data structures (production quality)
tutorials/      → Progressive learning paths (step-by-step)
rust_book/      → Official content by chapter
advent_of_code/ → Problem solving practice
project_euler/  → Mathematical algorithms
```

**Advantage**: Each track has independent crates, clear purpose, reusable components.

### Pattern 2: Glob Patterns for Expansion

Use wildcards to include multiple crates without editing root `Cargo.toml`:

```toml
[workspace]
members = [
    "rust_book/Ch17/*",  # All async examples as separate crates
    "advanced_examples/*",
]
```

**Advantage**: Add new examples without modifying workspace configuration.

### Pattern 3: Intra-Workspace Dependencies

Workspace members can depend on each other using **path dependencies**:

```toml
# In advent_of_code/aoc2023/Cargo.toml
[dependencies]
mission6 = { path = "../../missions/Mission6" }  # Grid<T>
mission8 = { path = "../../missions/Mission8" }  # Graph, BFS
```

Then use in code:
```rust
// advent_of_code/aoc2023/src/solver/day10.rs
use mission6::Grid;
use mission8::{Graph, bfs};

fn solve(input: &str) -> usize {
    let grid = Grid::from_input(input);
    bfs(&grid, start, end)
}
```

**Advantage**: Compose validated mission components in AoC solutions - integrator approach!

### Pattern 4: Workspace-Level Commands

```bash
# Build everything
cargo build --workspace

# Test all packages
cargo test --workspace

# Lint entire workspace
cargo clippy --workspace -- -D warnings

# Format all code
cargo fmt --all

# Test specific package
cargo test -p mission5
cargo run -p project_euler -- 2

# Check without building
cargo check --workspace
```

**Advantage**: Quality gates across entire repository, selective iteration.

## Dependency Resolution - Resolver Versions

### Resolver v1 (Pre-2021) - Avoid!

**Problem**: Feature pollution

```toml
# If ANY member enables serde/derive, ALL members get it
# mission1/Cargo.toml
[dependencies]
serde = "1.0"  # No features

# mission2/Cargo.toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }

# With v1: mission1 ALSO gets "derive" feature (unnecessary!)
```

**Issues**:
- Dev-dependencies leak features to main code
- Build scripts force features on regular dependencies
- Wasteful compilation, larger binaries

### Resolver v2 (Rust 2021+) - Good

**Improvements**:
- Per-target feature resolution
- Dev-dependencies isolated from main code
- Build script features don't leak
- Default for edition 2021

```toml
[workspace]
resolver = "2"  # Or implicit with edition = "2021"

# Each member gets ONLY the features it requests
# No pollution!
```

### Resolver v3 (Rust 1.78+) - Best! 🚀

**Additional improvement**: **MSRV-aware dependency resolution**

```toml
[workspace]
resolver = "3"  # Cutting edge!

[workspace.package]
rust-version = "1.75.0"  # Minimum supported Rust version
```

**How it works**:
1. Reads your `rust-version` field
2. Checks dependency versions
3. Automatically picks versions compatible with your MSRV
4. Prevents "requires Rust X.Y" build failures

**Example**:
- You specify: `rust-version = "1.75.0"`
- Dependency `serde` has versions:
  - `1.0.195` (requires Rust 1.75+) ✓
  - `1.0.200` (requires Rust 1.80+) ❌
- Cargo picks `1.0.195` automatically!

**Benefits**:
- ✅ Stable since Rust 1.78 (May 2024)
- ✅ Backward compatible with v2
- ✅ More correct dependency resolution
- ✅ Future-proof (will be default in edition 2024)
- ✅ Enables publishing with broad Rust version support

## Workspace Inheritance (Rust 1.64+)

Share common configuration across workspace members:

### Old Way (Duplication)
```toml
# missions/Mission5/Cargo.toml
[dependencies]
criterion = "0.5"
anyhow = "1.0"

# missions/Mission6/Cargo.toml
[dependencies]
criterion = "0.5"  # Duplicate!
anyhow = "1.0"     # Duplicate!
```

**Problem**: Version drift, inconsistency, tedious updates.

### New Way (Workspace Dependencies)
```toml
# Root Cargo.toml
[workspace]
members = ["missions/*"]

[workspace.dependencies]
criterion = "0.5"
anyhow = "1.0"
tokio = { version = "1.35", features = ["full"] }

# missions/Mission5/Cargo.toml
[dependencies]
criterion = { workspace = true }  # Use workspace version!
anyhow = { workspace = true }

# missions/Mission6/Cargo.toml
[dependencies]
criterion = { workspace = true }  # Same version guaranteed!
tokio = { workspace = true }      # Inherits features from workspace
```

**Benefits**:
- ✅ Single source of truth for versions
- ✅ Update once in root, all members get it
- ✅ Consistent dependency graph
- ✅ Can override features per-member if needed

### Metadata Inheritance
```toml
# Root Cargo.toml
[workspace.package]
edition = "2021"
rust-version = "1.75.0"
authors = ["Your Name"]
license = "MIT"

# missions/Mission5/Cargo.toml
[package]
name = "mission5"
version = "0.1.0"
edition.workspace = true        # Inherit from workspace
rust-version.workspace = true   # Inherit from workspace
authors.workspace = true
license.workspace = true
```

**Advantage**: Consistent metadata across all crates, easy updates.

## Critical Gotcha: Mixing Dependency Sources

### The Problem

**Scenario**: Same crate from different sources = incompatible types!

```
Your workspace:
  └─ mission6 (path = "missions/Mission6")

External crate on crates.io:
  └─ mission6 = "1.0.0"

Your code depends on both:
  ├─ mission6 (path)        ← Local version
  └─ some_external_crate
      └─ mission6 (crates.io) ← Published version
```

**Cargo sees TWO DIFFERENT mission6 crates!**

### The Confusing Error

```rust
use mission6::Grid;  // From path dependency

fn main() {
    let grid = Grid::new(10, 10);  // Type: mission6(path)::Grid
    external_fn(&grid);             // ❌ ERROR!
}

// external_fn expects mission6(crates.io)::Grid
```

**Compiler says**:
```
error: expected mission6::Grid, got mission6::Grid
                                    ^^^^
```

**What?!** Same name, but Cargo distinguishes by **source + name + version**:
- `mission6(path://../../missions/Mission6)` ≠ `mission6(registry+https://crates.io)`

### Why This Happens

Cargo's crate identification:

| **Source** | **Name** | **Version** | **Cargo's Internal ID** |
|------------|----------|-------------|-------------------------|
| Path | mission6 | 0.1.0 | `mission6(path)` |
| crates.io | mission6 | 0.1.0 | `mission6(crates.io)` |

**Different IDs → Different crates → Types incompatible!**

Even if the code is byte-for-byte identical!

### The Solution

**Rule**: **Never mix sources** for the same crate across workspace!

**Option 1: All Path Dependencies** (Development)
```toml
# Consistent - all local
mission6 = { path = "../../missions/Mission6" }
```

**Option 2: All Published** (After publishing)
```toml
# Consistent - all from crates.io
mission6 = "0.1.0"
```

**Option 3: Override External Dependencies** (Best of both)
```toml
# Root Cargo.toml
[patch.crates-io]
mission6 = { path = "missions/Mission6" }

# Now any crate asking for mission6 from crates.io
# gets your local version instead!
```

**Current rust_study status**: ✅ Safe - all path-based, no mixing!

## Workspace Best Practices

### 1. Organize by Logical Groups
```
missions/      → Core implementations
tutorials/     → Learning progressions
examples/      → Demonstrations
```

### 2. Use Workspace Dependencies
```toml
[workspace.dependencies]
# Shared across all members
criterion = "0.5"
anyhow = "1.0"
```

### 3. Set Resolver v3
```toml
[workspace]
resolver = "3"  # Latest, MSRV-aware
```

### 4. Specify MSRV
```toml
[workspace.package]
rust-version = "1.75.0"  # Minimum supported version
```

### 5. Consistent Path Dependencies
Never mix path + crates.io for same crate!

### 6. Use Glob Patterns
```toml
members = [
    "rust_book/Ch*/*",  # Expand as chapters added
]
```

### 7. Test Isolation
```bash
# Test specific packages during development
cargo test -p mission5

# Test everything in CI
cargo test --workspace
```

### 8. Quality Gates
```bash
cargo clippy --workspace -- -D warnings  # Zero warnings
cargo fmt --all --check                  # Formatted
cargo test --workspace                   # All pass
```

## Advanced Patterns

### Feature Flags Across Workspace

```toml
# Root Cargo.toml
[workspace.dependencies]
mission6 = { path = "missions/Mission6" }

# aoc2023/Cargo.toml
[dependencies]
mission6 = { workspace = true, features = ["serde"] }  # Add features

# tutorial/Cargo.toml
[dependencies]
mission6 = { workspace = true, default-features = false }  # Minimal
```

### Dev Dependencies Between Members

```toml
# mission6/Cargo.toml
[dev-dependencies]
mission5 = { path = "../Mission5" }  # For testing only
```

**Effect**: mission5 only linked when running tests, not in production builds.

### Virtual Manifests

Workspace with no root package (only members):

```toml
# Cargo.toml (workspace root, not a package)
[workspace]
members = ["crate_a", "crate_b"]
resolver = "3"

# No [package] section - this is not a package itself!
```

**Use case**: Pure workspace, no root-level code.

## Real-World Composition Example

**AoC Day 10**: Compose missions to solve problem

```rust
// advent_of_code/aoc2023/src/solver/day10.rs
use mission6::Grid;          // 2D storage component
use mission8::{Graph, bfs};  // Graph algorithms

struct TopoMap {
    grid: Grid<char>,
}

impl Graph for TopoMap {
    // Implement Graph trait using Grid storage
}

fn solve(input: &str) -> usize {
    let map = TopoMap::from(input);
    bfs(&map, start, end)  // Use mission8's validated BFS
}
```

**Dependencies** (`aoc2023/Cargo.toml`):
```toml
[dependencies]
mission6 = { path = "../../missions/Mission6" }
mission8 = { path = "../../missions/Mission8" }
```

**Philosophy**: **Integrator approach**
- Don't reimplement Grid or BFS
- Compose validated mission components
- Focus on problem-solving logic

## Troubleshooting

### Circular Dependencies
```toml
# mission5/Cargo.toml
[dependencies]
mission6 = { path = "../Mission6" }

# mission6/Cargo.toml
[dependencies]
mission5 = { path = "../Mission5" }  # ❌ CYCLE!
```

**Error**: "cyclic package dependency"

**Solution**: Extract shared code into new crate both depend on.

### Feature Pollution (Resolver v1)
**Problem**: Dev-dependencies leak features to main code

**Solution**: Upgrade to `resolver = "2"` or `"3"`

### MSRV Violations
**Problem**: Dependencies require newer Rust than your MSRV

**Solution**: Use `resolver = "3"` with `rust-version` field

### Mixed Sources Error
**Problem**: "expected Foo, got Foo"

**Solution**: Check for path vs crates.io mixing, use `[patch]` to force consistency

## Related Concepts

- [[cargo-features]] - Feature flags and conditional compilation
- [[rust-editions]] - Edition 2015/2018/2021/2024 differences
- [[semantic-versioning]] - Version compatibility rules
- [[dependency-resolution]] - How Cargo picks versions
- [[build-scripts]] - Custom build logic (build.rs)

## References

- *Rust for Rustaceans* - Chapter 5: Project Structure
- [Cargo Book - Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo Book - Resolver](https://doc.rust-lang.org/cargo/reference/resolver.html)
- [[2026-01-26]] - Daily note documenting workspace learning session

---

*Links:*
- **Practice**: [[2026-01-26]] - RfR Ch5 session, "aha moment"
- **Application**: AoC Day 10 (mission6 + mission8 composition)
- **Theory**: Cargo Book chapters on workspaces and resolver
- **Tags**: #rust #cargo #workspaces #dependency-management #build-system #multi-crate
