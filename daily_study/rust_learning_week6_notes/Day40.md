# Day 40: Publishing Crates - Cargo.toml Metadata & Documentation

**Date**: Thursday, November 6, 2025  
**Week 6 Focus**: Modules, Crates & Cargo Mastery  
**Mission Alignment**: Mission 10 - Union by Rank Optimization

---

## 🎯 Learning Objectives

By the end of Day 40, you will:
- Master `Cargo.toml` metadata for publishing
- Understand crate documentation requirements
- Apply semantic versioning principles
- Perform pre-publish validation and checks
- Prepare crates for crates.io publication

---

## 📚 Core Concepts

### 1. Publishing to crates.io

**Why Publish?**
- Share code with the Rust community
- Enable others to use your work
- Build your developer portfolio
- Contribute to the ecosystem

**Publishing Workflow**:
```
1. Write code and tests
2. Add comprehensive documentation
3. Configure Cargo.toml metadata
4. Run pre-publish checks
5. Publish to crates.io
6. Maintain and version
```

### 2. Cargo.toml Metadata

**Required Fields**:
```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
edition = "2021"
```

**Publishing Fields** (required for crates.io):
```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
description = "A brief description of what this crate does"
repository = "https://github.com/username/repo"
```

**Optional but Recommended**:
```toml
[package]
# ... required fields ...
homepage = "https://example.com"
documentation = "https://docs.rs/my-awesome-crate"
readme = "README.md"
keywords = ["data-structures", "union-find", "disjoint-sets"]
categories = ["algorithms", "data-structures"]
exclude = [
    "tests/*",
    "benches/*",
    ".github/*",
]
```

### 3. Semantic Versioning (SemVer)

**Format**: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes (incompatible API changes)
- **MINOR**: New features (backward-compatible)
- **PATCH**: Bug fixes (backward-compatible)

**Examples**:
```
0.1.0 → 0.1.1  (bug fix)
0.1.1 → 0.2.0  (new feature)
0.2.0 → 1.0.0  (stable release / breaking change)
1.0.0 → 1.0.1  (bug fix)
1.0.1 → 1.1.0  (new feature)
1.1.0 → 2.0.0  (breaking change)
```

**Pre-1.0 Versions**:
- `0.x.y` - Initial development
- Breaking changes allowed in MINOR version
- `0.1.0 → 0.2.0` can have breaking changes

**Version Ranges in Dependencies**:
```toml
[dependencies]
serde = "1.0"           # ^1.0.0 (>=1.0.0, <2.0.0)
regex = "1.5.4"         # ^1.5.4 (>=1.5.4, <2.0.0)
tokio = "~1.20"         # >=1.20.0, <1.21.0
rand = ">= 0.8, < 0.9"  # Explicit range
```

### 4. Documentation Requirements

**Crate-Level Documentation**:
```rust
//! # My Awesome Crate
//!
//! This crate provides efficient disjoint-set data structures.
//!
//! ## Features
//!
//! - Path compression optimization
//! - Union by rank
//! - O(α(n)) amortized operations
//!
//! ## Example
//!
//! ```
//! use my_awesome_crate::UnionFind;
//!
//! let mut uf = UnionFind::new(10);
//! uf.union(0, 1).unwrap();
//! assert!(uf.connected(0, 1).unwrap());
//! ```
```

**Public API Documentation**:
```rust
/// Creates a new UnionFind structure with `n` elements.
///
/// # Arguments
///
/// * `n` - The number of elements (0 to n-1)
///
/// # Examples
///
/// ```
/// use my_crate::UnionFind;
///
/// let uf = UnionFind::new(5);
/// assert_eq!(uf.count(), 5);
/// ```
///
/// # Complexity
///
/// Time: O(n), Space: O(n)
pub fn new(n: usize) -> Self { }
```

### 5. License Selection

**Common Licenses**:
- `MIT` - Permissive, simple
- `Apache-2.0` - Permissive, patent protection
- `MIT OR Apache-2.0` - Standard Rust dual-license
- `GPL-3.0` - Copyleft
- `BSD-3-Clause` - Permissive

**License Files**:
```
project/
├── Cargo.toml          # license = "MIT OR Apache-2.0"
├── LICENSE-MIT         # Full MIT license text
└── LICENSE-APACHE      # Full Apache 2.0 license text
```

---

## 💻 Implementation Examples

### Example 1: Complete Cargo.toml for Publishing

```toml
[package]
name = "union-find-rs"
version = "0.1.0"
edition = "2021"
authors = ["Alice Developer <alice@example.com>"]
license = "MIT OR Apache-2.0"
description = "Efficient disjoint-set (union-find) data structure with path compression and union by rank"
homepage = "https://github.com/alice/union-find-rs"
repository = "https://github.com/alice/union-find-rs"
documentation = "https://docs.rs/union-find-rs"
readme = "README.md"
keywords = ["union-find", "disjoint-sets", "data-structures", "algorithms", "graph"]
categories = ["data-structures", "algorithms"]
rust-version = "1.70"  # Minimum Rust version

[dependencies]
# No dependencies for core implementation

[dev-dependencies]
criterion = "0.5"
proptest = "1.0"

[features]
default = []
parallel = []  # Future feature for parallel operations

# Documentation metadata
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]

# Exclude files from published crate
exclude = [
    ".github/*",
    "tests/fixtures/*",
    "benches/data/*",
    ".gitignore",
    ".vscode/*",
]

[[bench]]
name = "performance"
harness = false
```

### Example 2: Comprehensive Documentation

```rust
//! # Union-Find (Disjoint Set Union)
//!
//! An efficient implementation of the union-find data structure with
//! path compression and union by rank optimizations.
//!
//! ## Quick Start
//!
//! ```
//! use union_find_rs::UnionFind;
//!
//! let mut uf = UnionFind::new(10);
//! uf.union(0, 1).unwrap();
//! uf.union(2, 3).unwrap();
//! uf.union(0, 2).unwrap();
//!
//! assert!(uf.connected(0, 3).unwrap());
//! assert!(!uf.connected(0, 4).unwrap());
//! assert_eq!(uf.count(), 7); // 7 disjoint sets remain
//! ```
//!
//! ## Features
//!
//! - **Path Compression**: Flattens tree structure during find operations
//! - **Union by Rank**: Balances trees to minimize height
//! - **O(α(n)) Operations**: Near-constant amortized time complexity
//! - **Generic Support**: Works with any type implementing appropriate traits
//!
//! ## Use Cases
//!
//! - Connected components in graphs
//! - Kruskal's minimum spanning tree algorithm
//! - Cycle detection in undirected graphs
//! - Network connectivity queries
//! - Image segmentation
//!
//! ## Performance
//!
//! All operations (union, find, connected) have O(α(n)) amortized time,
//! where α is the inverse Ackermann function (practically constant).
//!
//! ## Examples
//!
//! ### Graph Connectivity
//!
//! ```
//! use union_find_rs::UnionFind;
//!
//! // Check if graph is connected
//! let edges = vec![(0, 1), (1, 2), (3, 4)];
//! let mut uf = UnionFind::new(5);
//!
//! for (u, v) in edges {
//!     uf.union(u, v).unwrap();
//! }
//!
//! // Nodes 0,1,2 are connected, 3,4 are connected
//! assert!(uf.connected(0, 2).unwrap());
//! assert!(uf.connected(3, 4).unwrap());
//! assert!(!uf.connected(0, 3).unwrap());
//! assert_eq!(uf.count(), 2); // Two connected components
//! ```
//!
//! ### Cycle Detection
//!
//! ```
//! use union_find_rs::UnionFind;
//!
//! let mut uf = UnionFind::new(4);
//! let edges = vec![(0, 1), (1, 2), (2, 3)];
//!
//! for (u, v) in edges {
//!     if uf.connected(u, v).unwrap() {
//!         println!("Cycle detected!");
//!     }
//!     uf.union(u, v).unwrap();
//! }
//! ```

use std::error::Error;
use std::fmt;

/// Error type for UnionFind operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnionFindError {
    /// Element index is out of bounds
    IndexOutOfBounds { index: usize, size: usize },
}

impl fmt::Display for UnionFindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnionFindError::IndexOutOfBounds { index, size } => {
                write!(f, "Index {} out of bounds (size: {})", index, size)
            }
        }
    }
}

impl Error for UnionFindError {}

/// A disjoint-set data structure with path compression and union by rank.
///
/// This structure efficiently maintains a collection of disjoint (non-overlapping)
/// sets and supports fast union and find operations.
///
/// # Type Parameters
///
/// Currently works with `usize` indices. Future versions may support generic types.
///
/// # Examples
///
/// ```
/// # use union_find_rs::UnionFind;
/// let mut uf = UnionFind::new(5);
/// uf.union(0, 1).unwrap();
/// uf.union(2, 3).unwrap();
/// assert!(uf.connected(0, 1).unwrap());
/// assert!(!uf.connected(0, 2).unwrap());
/// ```
#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    /// Creates a new UnionFind structure with `n` elements.
    ///
    /// Initially, each element is in its own set (n disjoint sets).
    ///
    /// # Arguments
    ///
    /// * `n` - The number of elements (indexed from 0 to n-1)
    ///
    /// # Examples
    ///
    /// ```
    /// # use union_find_rs::UnionFind;
    /// let uf = UnionFind::new(10);
    /// assert_eq!(uf.count(), 10); // 10 separate sets initially
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(n)
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    /// Finds the root (representative) of the set containing element `x`.
    ///
    /// Uses path compression to flatten the tree structure, making future
    /// operations faster.
    ///
    /// # Arguments
    ///
    /// * `x` - The element to find the root of
    ///
    /// # Returns
    ///
    /// The root element, or an error if `x` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use union_find_rs::UnionFind;
    /// let mut uf = UnionFind::new(5);
    /// uf.union(0, 1).unwrap();
    /// uf.union(1, 2).unwrap();
    /// // All should have the same root
    /// let root = uf.find(0).unwrap();
    /// assert_eq!(uf.find(1).unwrap(), root);
    /// assert_eq!(uf.find(2).unwrap(), root);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(α(n)) amortized, where α is the inverse Ackermann function
    pub fn find(&mut self, x: usize) -> Result<usize, UnionFindError> {
        if x >= self.parent.len() {
            return Err(UnionFindError::IndexOutOfBounds {
                index: x,
                size: self.parent.len(),
            });
        }

        // Path compression: make every node point directly to root
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])?;
        }
        Ok(self.parent[x])
    }

    /// Merges the sets containing elements `x` and `y`.
    ///
    /// Uses union by rank to keep trees balanced.
    ///
    /// # Arguments
    ///
    /// * `x` - First element
    /// * `y` - Second element
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, or an error if indices are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use union_find_rs::UnionFind;
    /// let mut uf = UnionFind::new(5);
    /// assert_eq!(uf.count(), 5);
    ///
    /// uf.union(0, 1).unwrap();
    /// assert_eq!(uf.count(), 4); // One fewer set
    ///
    /// uf.union(0, 1).unwrap(); // Already connected, no change
    /// assert_eq!(uf.count(), 4);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(α(n)) amortized
    pub fn union(&mut self, x: usize, y: usize) -> Result<(), UnionFindError> {
        let root_x = self.find(x)?;
        let root_y = self.find(y)?;

        if root_x == root_y {
            return Ok(()); // Already in same set
        }

        // Union by rank: attach smaller tree to larger
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        self.count -= 1;
        Ok(())
    }

    /// Checks if elements `x` and `y` are in the same set.
    ///
    /// # Arguments
    ///
    /// * `x` - First element
    /// * `y` - Second element
    ///
    /// # Returns
    ///
    /// `Ok(true)` if connected, `Ok(false)` if not, or an error if indices are invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use union_find_rs::UnionFind;
    /// let mut uf = UnionFind::new(5);
    /// uf.union(0, 1).unwrap();
    /// uf.union(2, 3).unwrap();
    ///
    /// assert!(uf.connected(0, 1).unwrap());
    /// assert!(!uf.connected(0, 2).unwrap());
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(α(n)) amortized
    pub fn connected(&mut self, x: usize, y: usize) -> Result<bool, UnionFindError> {
        Ok(self.find(x)? == self.find(y)?)
    }

    /// Returns the number of disjoint sets.
    ///
    /// # Examples
    ///
    /// ```
    /// # use union_find_rs::UnionFind;
    /// let mut uf = UnionFind::new(5);
    /// assert_eq!(uf.count(), 5);
    ///
    /// uf.union(0, 1).unwrap();
    /// assert_eq!(uf.count(), 4);
    ///
    /// uf.union(2, 3).unwrap();
    /// assert_eq!(uf.count(), 3);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1)
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the total number of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use union_find_rs::UnionFind;
    /// let uf = UnionFind::new(10);
    /// assert_eq!(uf.len(), 10);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1)
    pub fn len(&self) -> usize {
        self.parent.len()
    }

    /// Checks if the structure is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use union_find_rs::UnionFind;
    /// let uf = UnionFind::new(0);
    /// assert!(uf.is_empty());
    ///
    /// let uf2 = UnionFind::new(5);
    /// assert!(!uf2.is_empty());
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(1)
    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}
```

### Example 3: Pre-Publish Checklist

```rust
// Create a comprehensive pre-publish validation script

use std::process::Command;

fn main() {
    println!("🚀 Pre-Publish Validation Checklist\n");
    
    let checks = vec![
        ("Format check", "cargo", vec!["fmt", "--", "--check"]),
        ("Clippy (strict)", "cargo", vec!["clippy", "--", "-D", "warnings"]),
        ("Tests", "cargo", vec!["test", "--all-features"]),
        ("Doc tests", "cargo", vec!["test", "--doc"]),
        ("Documentation", "cargo", vec!["doc", "--no-deps", "--all-features"]),
        ("Package dry-run", "cargo", vec!["package", "--allow-dirty"]),
    ];
    
    let mut all_passed = true;
    
    for (name, cmd, args) in checks {
        print!("⏳ Running: {} ... ", name);
        
        let output = Command::new(cmd)
            .args(&args)
            .output()
            .expect("Failed to execute command");
        
        if output.status.success() {
            println!("✅ PASSED");
        } else {
            println!("❌ FAILED");
            println!("{}", String::from_utf8_lossy(&output.stderr));
            all_passed = false;
        }
    }
    
    println!("\n" + "=".repeat(50));
    if all_passed {
        println!("✅ All checks passed! Ready to publish.");
        println!("\nNext steps:");
        println!("  1. git commit -am 'Prepare for v0.1.0 release'");
        println!("  2. git tag v0.1.0");
        println!("  3. cargo publish");
        println!("  4. git push origin main --tags");
    } else {
        println!("❌ Some checks failed. Fix issues before publishing.");
    }
}
```

### Example 4: README.md Template

```markdown
# union-find-rs

[![Crates.io](https://img.shields.io/crates/v/union-find-rs.svg)](https://crates.io/crates/union-find-rs)
[![Documentation](https://docs.rs/union-find-rs/badge.svg)](https://docs.rs/union-find-rs)
[![License](https://img.shields.io/crates/l/union-find-rs.svg)](LICENSE)
[![Build Status](https://github.com/username/union-find-rs/workflows/CI/badge.svg)](https://github.com/username/union-find-rs/actions)

Efficient disjoint-set (union-find) data structure with path compression and union by rank.

## Features

- 🚀 **Fast**: O(α(n)) amortized operations (inverse Ackermann function)
- 🔧 **Optimized**: Path compression and union by rank
- 📚 **Well-documented**: Comprehensive API documentation with examples
- ✅ **Tested**: 100% test coverage
- 🦀 **Pure Rust**: No unsafe code

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
union-find-rs = "0.1"
```

## Quick Start

```rust
use union_find_rs::UnionFind;

let mut uf = UnionFind::new(10);
uf.union(0, 1).unwrap();
uf.union(2, 3).unwrap();

assert!(uf.connected(0, 1).unwrap());
assert!(!uf.connected(0, 2).unwrap());
```

## Use Cases

- Connected components in graphs
- Kruskal's minimum spanning tree
- Cycle detection
- Network connectivity
- Image segmentation

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/union-find-rs).

## Performance

All operations have O(α(n)) amortized time complexity, where α is the inverse Ackermann function (practically constant for all reasonable inputs).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
```

---

## 🎯 Mission 10 Integration: Union by Rank

Apply publishing best practices to Mission 10:

```toml
# missions/Mission10/Cargo.toml
[package]
name = "mission10-unionfind"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
description = "Union-Find disjoint set implementation with path compression and union by rank"
repository = "https://github.com/yourusername/rust_study"
keywords = ["union-find", "disjoint-sets", "graph-algorithms"]
categories = ["data-structures", "algorithms"]

[package.metadata.docs.rs]
all-features = true

[dependencies]
# Core implementation has no dependencies

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "performance"
harness = false
```

**Documentation for Union by Rank**:

```rust
/// Merges the sets containing `x` and `y` using union by rank.
///
/// **Union by Rank Optimization**:
///
/// Instead of arbitrarily choosing which tree to attach, we keep track
/// of each tree's "rank" (upper bound on height). We always attach the
/// shorter tree to the taller one, keeping trees balanced.
///
/// # Rank Rules
///
/// - Initially, all ranks are 0
/// - When unioning trees of different ranks, attach smaller to larger
/// - When ranks are equal, choose arbitrarily and increment rank
/// - Rank never decreases, upper bound on tree height
///
/// # Performance Impact
///
/// Without union by rank: Trees can become chains (O(n) height)
/// With union by rank: Trees stay balanced (O(log n) height)
/// Combined with path compression: O(α(n)) amortized
///
/// # Examples
///
/// ```
/// # use mission10::UnionFind;
/// let mut uf = UnionFind::new(5);
///
/// // Union operations maintain balanced trees
/// uf.union(0, 1).unwrap();  // rank[0] becomes 1
/// uf.union(2, 3).unwrap();  // rank[2] becomes 1
/// uf.union(0, 2).unwrap();  // merge trees of equal rank
/// ```
pub fn union(&mut self, x: usize, y: usize) -> Result<(), String> {
    // Implementation with union by rank
}
```

---

## 🧪 Exercises

### Exercise 1: Complete Cargo.toml
Create a publication-ready `Cargo.toml` for Mission 10 with all required and recommended fields.

### Exercise 2: Add Comprehensive Documentation
Document all public APIs in Mission 10 with examples, complexity analysis, and edge cases.

### Exercise 3: Create README.md
Write a complete README.md with badges, quick start, use cases, and examples.

### Exercise 4: Pre-Publish Validation
Run all pre-publish checks and fix any issues:
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps
cargo package --allow-dirty
```

### Exercise 5: Semantic Versioning Practice
Given these changes, determine the correct version bump:
- Adding a new public method → ?
- Fixing a bug → ?
- Removing a public function → ?
- Adding an optional feature → ?

---

## 📊 Complete Runnable Example

```rust
//! Complete example demonstrating publishing best practices
//!
//! Run with: cargo run --example day40_publishing

use std::process::Command;

fn main() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║  Day 40: Publishing Crates                   ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    // Part 1: Cargo.toml Metadata
    println!("=== Part 1: Cargo.toml Metadata ===");
    demonstrate_cargo_metadata();

    // Part 2: Documentation Standards
    println!("\n=== Part 2: Documentation Standards ===");
    demonstrate_documentation();

    // Part 3: Semantic Versioning
    println!("\n=== Part 3: Semantic Versioning ===");
    demonstrate_semver();

    // Part 4: Pre-Publish Checks
    println!("\n=== Part 4: Pre-Publish Validation ===");
    demonstrate_validation();
}

fn demonstrate_cargo_metadata() {
    println!("Required metadata for publishing:");
    println!("  ✓ name - Unique crate name");
    println!("  ✓ version - SemVer (e.g., '0.1.0')");
    println!("  ✓ edition - Rust edition (2021)");
    println!("  ✓ authors - Your name and email");
    println!("  ✓ license - 'MIT OR Apache-2.0'");
    println!("  ✓ description - One-line summary");
    println!("  ✓ repository - GitHub/GitLab URL");

    println!("\nRecommended metadata:");
    println!("  • readme - 'README.md'");
    println!("  • keywords - 5 max, searchability");
    println!("  • categories - From crates.io list");
    println!("  • homepage - Project website");
    println!("  • documentation - docs.rs URL");
    println!("  • rust-version - Minimum Rust version");

    println!("\nExample:");
    println!(r#"
[package]
name = "awesome-crate"
version = "0.1.0"
edition = "2021"
authors = ["Alice <alice@example.com>"]
license = "MIT OR Apache-2.0"
description = "An awesome crate for doing awesome things"
repository = "https://github.com/alice/awesome-crate"
keywords = ["data-structures", "algorithms"]
categories = ["algorithms"]
"#);
}

fn demonstrate_documentation() {
    println!("Documentation requirements:");
    println!("  1. Crate-level docs (//! in lib.rs)");
    println!("  2. Module-level docs for all modules");
    println!("  3. Public API docs for all public items");
    println!("  4. Examples in documentation");
    println!("  5. Complexity analysis where relevant");

    println!("\nQuality checklist:");
    println!("  ✓ All public items documented");
    println!("  ✓ Examples compile and run");
    println!("  ✓ Clear and concise descriptions");
    println!("  ✓ Links to related items");
    println!("  ✓ Panic conditions documented");

    println!("\nExample documentation:");
    println!(r#"
/// Finds the root of element `x`.
///
/// Uses path compression to flatten the tree.
///
/// # Arguments
///
/// * `x` - Element to find root of
///
/// # Returns
///
/// Root element or error if out of bounds.
///
/// # Examples
///
/// ```
/// use crate::UnionFind;
/// let mut uf = UnionFind::new(5);
/// assert_eq!(uf.find(0).unwrap(), 0);
/// ```
///
/// # Complexity
///
/// O(α(n)) amortized time
pub fn find(&mut self, x: usize) -> Result<usize, Error>
"#);
}

fn demonstrate_semver() {
    println!("Semantic Versioning: MAJOR.MINOR.PATCH");

    println!("\nVersion increments:");
    println!("  PATCH (0.1.0 → 0.1.1):");
    println!("    • Bug fixes");
    println!("    • Documentation updates");
    println!("    • Internal refactoring");

    println!("\n  MINOR (0.1.1 → 0.2.0):");
    println!("    • New features (backward compatible)");
    println!("    • New public APIs");
    println!("    • Deprecations");

    println!("\n  MAJOR (0.2.0 → 1.0.0 or 1.0.0 → 2.0.0):");
    println!("    • Breaking changes");
    println!("    • Removed public APIs");
    println!("    • Changed function signatures");
    println!("    • Changed behavior");

    println!("\nPre-1.0 special rules:");
    println!("  • 0.x.y - Initial development");
    println!("  • Breaking changes allowed in MINOR");
    println!("  • 0.1.0 → 0.2.0 can break compatibility");
    println!("  • 1.0.0 signals 'stable API'");

    println!("\nDependency version examples:");
    println!(r#"
serde = "1.0"           # ^1.0.0 (>=1.0.0, <2.0.0)
tokio = "~1.20"         # >=1.20.0, <1.21.0
rand = ">= 0.8, < 0.9"  # Explicit range
regex = "1"             # Latest 1.x.x
"#);
}

fn demonstrate_validation() {
    println!("Pre-publish validation checklist:");

    let checks = vec![
        ("1. Cargo.toml complete", true),
        ("2. LICENSE files present", true),
        ("3. README.md exists", true),
        ("4. All tests pass", true),
        ("5. Documentation complete", true),
        ("6. No clippy warnings", true),
        ("7. Code formatted", true),
        ("8. No uncommitted changes", false),
    ];

    for (check, passed) in checks {
        let status = if passed { "✅" } else { "❌" };
        println!("  {} {}", status, check);
    }

    println!("\nValidation commands:");
    println!("  cargo fmt --check");
    println!("  cargo clippy -- -D warnings");
    println!("  cargo test");
    println!("  cargo test --doc");
    println!("  cargo doc --no-deps");
    println!("  cargo package --list");
    println!("  cargo package --allow-dirty");

    println!("\nPublishing workflow:");
    println!("  1. Ensure all checks pass");
    println!("  2. Update version in Cargo.toml");
    println!("  3. Update CHANGELOG.md");
    println!("  4. Commit: 'Release v0.1.0'");
    println!("  5. Tag: git tag v0.1.0");
    println!("  6. Dry run: cargo publish --dry-run");
    println!("  7. Publish: cargo publish");
    println!("  8. Push: git push origin main --tags");

    println!("\nPost-publish:");
    println!("  • Verify on crates.io");
    println!("  • Check docs.rs build");
    println!("  • Announce on social media");
    println!("  • Monitor for issues");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_semver_parsing() {
        // In real code, would use semver crate
        let version = "1.2.3";
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_version_increment() {
        let major = 1;
        let minor = 2;
        let patch = 3;

        // Patch increment
        let new_patch = patch + 1;
        assert_eq!(new_patch, 4); // 1.2.4

        // Minor increment (resets patch)
        let new_minor = minor + 1;
        assert_eq!(format!("{}.{}.0", major, new_minor), "1.3.0");

        // Major increment (resets minor and patch)
        let new_major = major + 1;
        assert_eq!(format!("{}.0.0", new_major), "2.0.0");
    }
}
```

---

## 🔑 Key Takeaways

1. **Complete Metadata**: Required fields for crates.io plus recommended fields for discoverability
2. **Comprehensive Documentation**: All public APIs with examples, complexity, and edge cases
3. **Semantic Versioning**: MAJOR.MINOR.PATCH with clear rules for each increment
4. **Pre-Publish Validation**: Automated checks before every release
5. **Quality Standards**: Zero warnings, 100% doc coverage, all tests pass

---

## 📚 Additional Resources

- [Rust Book Chapter 14.2 - Publishing a Crate](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
- [Cargo Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Semantic Versioning](https://semver.org/)
- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [docs.rs Documentation](https://docs.rs/about)

---

## 🔗 Navigation

- **Previous**: [[Day39]] - Workspace Management
- **Next**: [[Day41]] - External Dependencies
- **Week Overview**: [[README]]
- **Mission 10**: [[../../missions/Mission10/README|Union-Find Implementation]]

---

*Tags: #week6 #day40 #publishing #cargo #documentation #semver #crates-io #metadata*
