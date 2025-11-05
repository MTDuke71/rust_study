# Day 39 - Workspace Management (multi-crate projects)

**Learning Focus**: Managing complex Rust projects with workspaces, organizing related crates, and coordinating shared dependencies

**Date**: November 5, 2025 (Wednesday)

**Mission Alignment**: Mission 10 Path Compression Optimization

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- How to create and configure Cargo workspaces
- Organizing related crates within a workspace
- Managing shared dependencies and version consistency
- Inter-crate dependencies and path references
- Workspace-wide operations and commands
- Best practices for scaling multi-crate projects

---

## 📚 Core Concepts

### **1. What Are Cargo Workspaces?**

A Cargo workspace is a collection of related crates that share common configuration and dependencies. Workspaces are essential for:
- **Code organization**: Logical separation of concerns
- **Dependency management**: Shared `Cargo.lock` file
- **Build optimization**: Incremental compilation across crates
- **Testing coordination**: Workspace-wide test execution
- **Release management**: Coordinated versioning and publishing

```toml
# Root Cargo.toml (workspace manifest)
[workspace]
members = [
    "core",           # Core data structures
    "algorithms",     # Algorithm implementations
    "cli",           # Command-line interface
    "web-api",       # REST API server
    "benchmarks",    # Performance benchmarks
    "examples",      # Usage examples
]

# Workspace-wide dependency management
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
clap = { version = "4.0", features = ["derive"] }

# Shared metadata
[workspace.package]
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/project"
```

### **2. Workspace Structure Patterns**

#### **Library + Binary Pattern**
```
my-project/
├── Cargo.toml          # Workspace manifest
├── Cargo.lock          # Shared lock file
├── README.md
├── core/               # Core library crate
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   └── tests/
├── cli/                # CLI binary crate
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── tests/
└── web-api/            # Web API binary crate
    ├── Cargo.toml
    ├── src/
    │   └── main.rs
    └── tests/
```

#### **Domain-Driven Design Pattern**
```
e-commerce/
├── Cargo.toml
├── user-management/    # User domain
│   ├── Cargo.toml
│   └── src/
├── inventory/          # Inventory domain
│   ├── Cargo.toml
│   └── src/
├── orders/            # Orders domain
│   ├── Cargo.toml
│   └── src/
└── shared/            # Shared utilities
    ├── Cargo.toml
    └── src/
```

### **3. Inter-Crate Dependencies**

#### **Path Dependencies**
```toml
# cli/Cargo.toml
[dependencies]
core = { path = "../core" }                    # Local path dependency
algorithms = { path = "../algorithms" }        # Relative path
shared = { path = "../shared", version = "0.1" } # With version constraint
```

#### **Workspace Dependencies**
```toml
# Individual crate Cargo.toml can inherit workspace dependencies
[dependencies]
serde = { workspace = true }                   # Use workspace version
tokio = { workspace = true, features = ["rt"] } # Add extra features
```

### **4. Dependency Resolution Strategy**

#### **Unified Cargo.lock**
- **Single lock file** at workspace root
- **Consistent versions** across all crates
- **Dependency deduplication** saves disk space and compilation time
- **Reproducible builds** for the entire workspace

```bash
# All crates use the same version of dependencies
├── Cargo.lock  ← Single source of truth
├── core/       ← No individual Cargo.lock
├── cli/        ← No individual Cargo.lock
└── web-api/    ← No individual Cargo.lock
```

---

## 💻 Complete Runnable Example

Let's build a comprehensive multi-crate workspace for a Union-Find data structure project:

```toml
# Root Cargo.toml
[workspace]
resolver = "2"
members = [
    "union-find-core",
    "union-find-cli", 
    "union-find-web",
    "union-find-benchmarks",
    "union-find-examples",
]

[workspace.dependencies]
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# CLI
clap = { version = "4.0", features = ["derive"] }

# Web framework
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"

# Benchmarking
criterion = { version = "0.5", features = ["html_reports"] }

# Testing
proptest = "1.0"

[workspace.package]
version = "0.1.0"
authors = ["MTDuke71 <matt.laduke@gmail.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/MTDuke71/union-find-workspace"
keywords = ["data-structures", "union-find", "algorithms"]
categories = ["data-structures", "algorithms"]
```

```toml
# union-find-core/Cargo.toml
[package]
name = "union-find-core"
version.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
serde = { workspace = true, optional = true }

[features]
default = []
serde = ["dep:serde"]

[dev-dependencies]
proptest = { workspace = true }
```

```rust
// union-find-core/src/lib.rs
//! Core Union-Find (Disjoint Set) data structure implementation
//! 
//! This crate provides efficient union-find operations with path compression
//! and union by rank optimizations.

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Union-Find data structure with path compression and union by rank
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnionFind<T> 
where 
    T: Clone + Eq + std::hash::Hash,
{
    parent: HashMap<T, T>,
    rank: HashMap<T, usize>,
    count: usize,
}

impl<T> UnionFind<T> 
where 
    T: Clone + Eq + std::hash::Hash,
{
    /// Create a new empty Union-Find structure
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
            rank: HashMap::new(),
            count: 0,
        }
    }

    /// Make a new set containing the given element
    pub fn make_set(&mut self, x: T) -> bool {
        if self.parent.contains_key(&x) {
            return false; // Element already exists
        }
        
        self.parent.insert(x.clone(), x.clone());
        self.rank.insert(x, 0);
        self.count += 1;
        true
    }

    /// Find the representative of the set containing x (with path compression)
    pub fn find(&mut self, x: &T) -> Option<T> {
        if !self.parent.contains_key(x) {
            return None;
        }

        // Path compression: make every node point directly to root
        let mut current = x.clone();
        let mut path = Vec::new();
        
        // Find root and collect path
        while self.parent[&current] != current {
            path.push(current.clone());
            current = self.parent[&current].clone();
        }
        
        // Compress path: point all nodes directly to root
        for node in path {
            self.parent.insert(node, current.clone());
        }
        
        Some(current)
    }

    /// Union two sets by rank (returns true if they were in different sets)
    pub fn union(&mut self, x: &T, y: &T) -> bool {
        let root_x = match self.find(x) {
            Some(root) => root,
            None => return false,
        };
        
        let root_y = match self.find(y) {
            Some(root) => root,
            None => return false,
        };

        if root_x == root_y {
            return false; // Already in same set
        }

        let rank_x = self.rank[&root_x];
        let rank_y = self.rank[&root_y];

        // Union by rank: attach smaller tree under root of larger tree
        match rank_x.cmp(&rank_y) {
            std::cmp::Ordering::Less => {
                self.parent.insert(root_x, root_y);
            }
            std::cmp::Ordering::Greater => {
                self.parent.insert(root_y, root_x);
            }
            std::cmp::Ordering::Equal => {
                self.parent.insert(root_y, root_x.clone());
                *self.rank.get_mut(&root_x).unwrap() += 1;
            }
        }

        self.count -= 1;
        true
    }

    /// Check if two elements are in the same set
    pub fn connected(&mut self, x: &T, y: &T) -> bool {
        match (self.find(x), self.find(y)) {
            (Some(root_x), Some(root_y)) => root_x == root_y,
            _ => false,
        }
    }

    /// Get the number of disjoint sets
    pub fn count(&self) -> usize {
        self.count
    }

    /// Get all elements in the same set as the given element
    pub fn get_set_members(&mut self, x: &T) -> Vec<T> {
        let root = match self.find(x) {
            Some(r) => r,
            None => return Vec::new(),
        };

        self.parent
            .keys()
            .filter(|&k| self.find(k) == Some(root.clone()))
            .cloned()
            .collect()
    }
}

impl<T> Default for UnionFind<T> 
where 
    T: Clone + Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut uf = UnionFind::new();
        
        // Make sets
        assert!(uf.make_set(1));
        assert!(uf.make_set(2));
        assert!(uf.make_set(3));
        assert_eq!(uf.count(), 3);
        
        // Initially disconnected
        assert!(!uf.connected(&1, &2));
        
        // Union and test connection
        assert!(uf.union(&1, &2));
        assert!(uf.connected(&1, &2));
        assert_eq!(uf.count(), 2);
        
        // Union with third element
        assert!(uf.union(&2, &3));
        assert!(uf.connected(&1, &3));
        assert_eq!(uf.count(), 1);
    }

    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new();
        
        // Create a chain: 1 -> 2 -> 3 -> 4 -> 5
        for i in 1..=5 {
            uf.make_set(i);
        }
        
        uf.union(&1, &2);
        uf.union(&2, &3);
        uf.union(&3, &4);
        uf.union(&4, &5);
        
        // All should be connected
        assert!(uf.connected(&1, &5));
        
        // After path compression, all should point to same root
        let root1 = uf.find(&1).unwrap();
        let root5 = uf.find(&5).unwrap();
        assert_eq!(root1, root5);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization() {
        let mut uf = UnionFind::new();
        uf.make_set(1);
        uf.make_set(2);
        uf.union(&1, &2);
        
        let json = serde_json::to_string(&uf).unwrap();
        let deserialized: UnionFind<i32> = serde_json::from_str(&json).unwrap();
        
        // Note: After deserialization, we need to rebuild internal state
        // This is a limitation of the current implementation
        assert_eq!(deserialized.count(), uf.count());
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn union_find_consistency(elements in prop::collection::vec(1i32..100, 1..20)) {
            let mut uf = UnionFind::new();
            
            // Make all sets
            for &elem in &elements {
                uf.make_set(elem);
            }
            
            let initial_count = uf.count();
            
            // Union some random pairs
            for i in 0..elements.len().saturating_sub(1) {
                uf.union(&elements[i], &elements[i + 1]);
            }
            
            // Count should decrease
            assert!(uf.count() <= initial_count);
            
            // Reflexivity: everything connected to itself
            for &elem in &elements {
                assert!(uf.connected(&elem, &elem));
            }
        }
    }
}
```

```toml
# union-find-cli/Cargo.toml
[package]
name = "union-find-cli"
version.workspace = true
authors.workspace = true
license.workspace = true

[[bin]]
name = "uf"
path = "src/main.rs"

[dependencies]
union-find-core = { path = "../union-find-core", features = ["serde"] }
clap = { workspace = true }
serde_json = { workspace = true }
```

```rust
// union-find-cli/src/main.rs
//! Command-line interface for Union-Find operations

use clap::{Parser, Subcommand};
use std::collections::HashMap;
use union_find_core::UnionFind;

#[derive(Parser)]
#[command(name = "uf")]
#[command(about = "Union-Find operations via CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Union-Find structure
    New {
        /// Elements to initialize (space-separated)
        elements: Vec<String>,
    },
    /// Union two elements
    Union {
        /// First element
        x: String,
        /// Second element  
        y: String,
        /// JSON file containing Union-Find state
        #[arg(short, long, default_value = "uf_state.json")]
        file: String,
    },
    /// Find root of element
    Find {
        /// Element to find
        element: String,
        /// JSON file containing Union-Find state
        #[arg(short, long, default_value = "uf_state.json")]
        file: String,
    },
    /// Check if two elements are connected
    Connected {
        /// First element
        x: String,
        /// Second element
        y: String,
        /// JSON file containing Union-Find state
        #[arg(short, long, default_value = "uf_state.json")]
        file: String,
    },
    /// Show statistics
    Stats {
        /// JSON file containing Union-Find state
        #[arg(short, long, default_value = "uf_state.json")]
        file: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { elements } => {
            let mut uf = UnionFind::new();
            for element in elements {
                uf.make_set(element);
            }
            
            save_state(&uf, "uf_state.json")?;
            println!("Created new Union-Find with {} elements", uf.count());
        }
        
        Commands::Union { x, y, file } => {
            let mut uf: UnionFind<String> = load_state(&file)?;
            let was_connected = uf.connected(&x, &y);
            let union_result = uf.union(&x, &y);
            
            if union_result {
                println!("United {} and {} (previously separate)", x, y);
            } else if was_connected {
                println!("{} and {} were already connected", x, y);
            } else {
                println!("Failed to union {} and {} (elements not found)", x, y);
            }
            
            save_state(&uf, &file)?;
        }
        
        Commands::Find { element, file } => {
            let mut uf: UnionFind<String> = load_state(&file)?;
            match uf.find(&element) {
                Some(root) => println!("Root of {}: {}", element, root),
                None => println!("Element {} not found", element),
            }
        }
        
        Commands::Connected { x, y, file } => {
            let mut uf: UnionFind<String> = load_state(&file)?;
            let connected = uf.connected(&x, &y);
            println!("{} and {} are {}", x, y, if connected { "connected" } else { "not connected" });
        }
        
        Commands::Stats { file } => {
            let uf: UnionFind<String> = load_state(&file)?;
            println!("Number of disjoint sets: {}", uf.count());
        }
    }

    Ok(())
}

fn save_state(uf: &UnionFind<String>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(uf)?;
    std::fs::write(filename, json)?;
    Ok(())
}

fn load_state(filename: &str) -> Result<UnionFind<String>, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(filename)?;
    let uf = serde_json::from_str(&json)?;
    Ok(uf)
}
```

```toml
# union-find-benchmarks/Cargo.toml
[package]
name = "union-find-benchmarks"
version.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
union-find-core = { path = "../union-find-core" }
criterion = { workspace = true }

[[bench]]
name = "union_find_performance"
harness = false
```

```rust
// union-find-benchmarks/benches/union_find_performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use union_find_core::UnionFind;

fn benchmark_union_find_operations(c: &mut Criterion) {
    let sizes = [100, 1000, 10000];
    
    let mut group = c.benchmark_group("UnionFind Operations");
    
    for &size in &sizes {
        group.bench_with_input(
            BenchmarkId::new("make_set", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut uf = UnionFind::new();
                    for i in 0..size {
                        uf.make_set(black_box(i));
                    }
                    black_box(uf)
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("union_sequential", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut uf = UnionFind::new();
                    // Setup
                    for i in 0..size {
                        uf.make_set(i);
                    }
                    
                    // Sequential unions: 0-1, 1-2, 2-3, ...
                    for i in 0..size-1 {
                        uf.union(&black_box(i), &black_box(i + 1));
                    }
                    black_box(uf)
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("find_after_unions", size),
            &size,
            |b, &size| {
                // Setup: create chain of unions
                let mut uf = UnionFind::new();
                for i in 0..size {
                    uf.make_set(i);
                }
                for i in 0..size-1 {
                    uf.union(&i, &i + 1);
                }
                
                b.iter(|| {
                    // Find operations on the connected structure
                    let mut uf_copy = uf.clone();
                    for i in 0..size {
                        black_box(uf_copy.find(&black_box(i)));
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn benchmark_path_compression_effect(c: &mut Criterion) {
    let mut group = c.benchmark_group("Path Compression");
    
    // Create worst-case scenario: linear chain
    let size = 10000;
    let mut uf = UnionFind::new();
    for i in 0..size {
        uf.make_set(i);
    }
    
    // Create linear chain: 0 -> 1 -> 2 -> ... -> 9999
    for i in 0..size-1 {
        uf.union(&i, &i + 1);
    }
    
    group.bench_function("find_deepest_before_compression", |b| {
        let mut uf_copy = uf.clone();
        b.iter(|| {
            // Find the deepest element (should trigger path compression)
            black_box(uf_copy.find(&black_box(size - 1)));
        });
    });
    
    // After path compression, subsequent finds should be faster
    let mut compressed_uf = uf.clone();
    compressed_uf.find(&(size - 1)); // Trigger path compression
    
    group.bench_function("find_deepest_after_compression", |b| {
        let mut uf_copy = compressed_uf.clone();
        b.iter(|| {
            black_box(uf_copy.find(&black_box(size - 1)));
        });
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_union_find_operations, benchmark_path_compression_effect);
criterion_main!(benches);
```

```toml
# union-find-examples/Cargo.toml
[package]
name = "union-find-examples"
version.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
union-find-core = { path = "../union-find-core" }

[[example]]
name = "basic_usage"

[[example]]
name = "maze_generation"

[[example]]  
name = "network_connectivity"
```

```rust
// union-find-examples/examples/basic_usage.rs
//! Basic Union-Find usage examples

use union_find_core::UnionFind;

fn main() {
    println!("=== Basic Union-Find Operations ===");
    
    let mut uf = UnionFind::new();
    
    // Create individual sets
    println!("Creating sets for elements 1-6...");
    for i in 1..=6 {
        uf.make_set(i);
    }
    println!("Initial set count: {}", uf.count());
    
    // Union some elements
    println!("\nUnioning elements:");
    uf.union(&1, &2);
    println!("After union(1, 2): {} sets", uf.count());
    
    uf.union(&3, &4);
    println!("After union(3, 4): {} sets", uf.count());
    
    uf.union(&2, &3);
    println!("After union(2, 3): {} sets", uf.count());
    
    // Check connectivity
    println!("\nConnectivity tests:");
    println!("1 connected to 4? {}", uf.connected(&1, &4));
    println!("1 connected to 5? {}", uf.connected(&1, &5));
    
    // Find representatives
    println!("\nSet representatives:");
    for i in 1..=6 {
        if let Some(root) = uf.find(&i) {
            println!("Representative of {}: {}", i, root);
        }
    }
    
    // Get all members of a set
    println!("\nSet members:");
    let set_1_members = uf.get_set_members(&1);
    println!("Members of set containing 1: {:?}", set_1_members);
}
```

```rust
// union-find-examples/examples/maze_generation.rs  
//! Using Union-Find for maze generation (Kruskal's algorithm variant)

use union_find_core::UnionFind;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    row: usize,
    col: usize,
}

impl Cell {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

struct Maze {
    width: usize,
    height: usize,
    walls: HashMap<(Cell, Cell), bool>, // true = wall exists
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let mut walls = HashMap::new();
        
        // Add all possible walls
        for row in 0..height {
            for col in 0..width {
                let cell = Cell::new(row, col);
                
                // Right wall
                if col + 1 < width {
                    let right_cell = Cell::new(row, col + 1);
                    walls.insert((cell, right_cell), true);
                }
                
                // Bottom wall
                if row + 1 < height {
                    let bottom_cell = Cell::new(row + 1, col);
                    walls.insert((cell, bottom_cell), true);
                }
            }
        }
        
        Self { width, height, walls }
    }
    
    fn generate_maze(&mut self) {
        let mut uf = UnionFind::new();
        
        // Create a set for each cell
        for row in 0..self.height {
            for col in 0..self.width {
                uf.make_set(Cell::new(row, col));
            }
        }
        
        // Collect all walls for random removal
        let mut wall_list: Vec<(Cell, Cell)> = self.walls.keys().cloned().collect();
        
        // Shuffle walls (simplified - in real implementation, use proper random)
        wall_list.reverse(); // Simple "shuffle" for demo
        
        // Remove walls until maze is fully connected
        for (cell1, cell2) in wall_list {
            // If cells are not connected, remove the wall
            if !uf.connected(&cell1, &cell2) {
                uf.union(&cell1, &cell2);
                self.walls.insert((cell1, cell2), false);
                
                // Stop when all cells are connected (spanning tree complete)
                if uf.count() == 1 {
                    break;
                }
            }
        }
    }
    
    fn print(&self) {
        println!("Generated maze ({}x{}):", self.width, self.height);
        
        // Print top border
        for _ in 0..self.width * 2 + 1 {
            print!("-");
        }
        println!();
        
        for row in 0..self.height {
            print!("|"); // Left border
            
            for col in 0..self.width {
                print!(" "); // Cell content
                
                let cell = Cell::new(row, col);
                
                // Print right wall or space
                if col + 1 < self.width {
                    let right_cell = Cell::new(row, col + 1);
                    let has_wall = self.walls.get(&(cell, right_cell))
                        .or_else(|| self.walls.get(&(right_cell, cell)))
                        .unwrap_or(&true);
                    print!("{}", if *has_wall { "|" } else { " " });
                } else {
                    print!("|"); // Right border
                }
            }
            println!();
            
            // Print bottom walls
            for col in 0..self.width {
                let cell = Cell::new(row, col);
                
                if row + 1 < self.height {
                    let bottom_cell = Cell::new(row + 1, col);
                    let has_wall = self.walls.get(&(cell, bottom_cell))
                        .or_else(|| self.walls.get(&(bottom_cell, cell)))
                        .unwrap_or(&true);
                    print!("{}", if *has_wall { "-" } else { " " });
                } else {
                    print!("-"); // Bottom border
                }
                print!("+");
            }
            println!();
        }
    }
}

fn main() {
    println!("=== Maze Generation with Union-Find ===");
    
    let mut maze = Maze::new(8, 6);
    println!("Initial maze (all walls):");
    maze.print();
    
    println!("\nGenerating maze using Union-Find...");
    maze.generate_maze();
    
    println!("\nFinal maze:");
    maze.print();
    
    println!("\nMaze generation complete!");
    println!("Union-Find ensured all cells are reachable from any other cell.");
}
```

---

## 🔧 Workspace Commands & Operations

### **Essential Workspace Commands**

```bash
# Create new workspace
mkdir my-workspace && cd my-workspace
cargo init --name workspace-root

# Add workspace members
mkdir core cli web-api
cd core && cargo init --lib
cd ../cli && cargo init --bin  
cd ../web-api && cargo init --bin

# Workspace-wide operations
cargo build                     # Build all crates
cargo test                      # Test all crates
cargo check                     # Check all crates
cargo clippy                    # Lint all crates
cargo fmt                       # Format all crates

# Build specific crate
cargo build -p core             # Build only core crate
cargo test -p cli               # Test only CLI crate
cargo run -p web-api            # Run web API binary

# Documentation
cargo doc --workspace --open    # Generate docs for all crates
cargo doc -p core --open        # Generate docs for specific crate
```

### **Dependency Management**

```bash
# Add dependency to specific crate
cd core && cargo add serde --features derive

# Add workspace dependency
# Edit root Cargo.toml [workspace.dependencies] section

# Update all dependencies
cargo update                    # Update based on Cargo.lock

# Check for outdated dependencies  
cargo outdated                  # Requires cargo-outdated plugin

# Check dependency tree
cargo tree                      # Show dependency graph
cargo tree -p core              # Show dependencies for specific crate
```

### **Publishing & Release**

```bash
# Publish specific crate
cargo publish -p core

# Publish all crates (requires careful ordering)
# Usually: libraries first, then binaries
cargo publish -p core
cargo publish -p cli
cargo publish -p web-api

# Check before publishing
cargo package -p core           # Create package without publishing
cargo package --list -p core    # List files that would be included
```

---

## 🎯 Best Practices

### **1. Workspace Organization**

#### **Logical Separation**
- **Libraries vs binaries**: Keep library crates separate from application crates
- **Domain boundaries**: Organize by business domain or architectural layer  
- **Shared utilities**: Create common crates for shared functionality
- **Testing**: Separate integration test crates when needed

#### **Naming Conventions**
```
project-name/
├── project-core/           # Core library
├── project-cli/            # Command-line tool
├── project-web/            # Web interface
├── project-common/         # Shared utilities
├── project-macros/         # Procedural macros
└── project-examples/       # Usage examples
```

### **2. Dependency Strategy**

#### **Workspace Dependencies**
- **Define once**: Use `[workspace.dependencies]` for shared versions
- **Consistent versioning**: Ensure compatible versions across crates
- **Feature flags**: Enable features only where needed

#### **Version Management**
```toml
# Workspace root Cargo.toml
[workspace.dependencies]
serde = "1.0"               # Base version
tokio = { version = "1.0", default-features = false }

# Individual crate Cargo.toml  
[dependencies]
serde = { workspace = true, features = ["derive"] }  # Add features
tokio = { workspace = true, features = ["rt"] }      # Add different features
```

### **3. Build Optimization**

#### **Incremental Compilation**
- **Shared target directory**: All crates share `target/` for faster builds
- **Dependency caching**: Common dependencies built once
- **Parallel compilation**: Cargo builds independent crates in parallel

#### **Profile Configuration**
```toml
# Workspace root Cargo.toml
[profile.dev]
opt-level = 0               # Fast compilation
debug = true                # Debug symbols

[profile.release]
opt-level = 3               # Maximum optimization  
lto = true                  # Link-time optimization
codegen-units = 1           # Better optimization

[profile.test]
opt-level = 1               # Some optimization for faster tests
```

### **4. Testing Strategy**

#### **Multi-Level Testing**
- **Unit tests**: Within each crate (`#[cfg(test)]`)
- **Integration tests**: Cross-crate integration (`tests/` directories)
- **End-to-end tests**: Full application testing
- **Example tests**: Ensure examples compile and run

#### **Test Organization**
```bash
# Run tests at different levels
cargo test --lib                    # Only unit tests
cargo test --bins                   # Binary tests  
cargo test --examples              # Example tests
cargo test --workspace             # All tests
cargo test -p core integration_*   # Integration tests in core crate
```

---

## 🧪 Practice Exercises

### **Exercise 1: Create Multi-Crate Workspace**

Build a workspace for a simple blog system:

```bash
mkdir blog-workspace && cd blog-workspace

# Your task: Create this structure
blog-workspace/
├── Cargo.toml              # Workspace manifest
├── blog-core/              # Core domain logic
├── blog-web/               # Web interface  
├── blog-cli/               # CLI tool
└── blog-storage/           # Data persistence

# Requirements:
# 1. blog-core: Define Post, Author, Tag structs
# 2. blog-web: Axum server using blog-core
# 3. blog-cli: Command-line blog management 
# 4. blog-storage: File and database persistence
# 5. Shared dependencies: serde, clap, sqlx
```

### **Exercise 2: Inter-Crate Dependencies**

Set up proper dependencies between crates:

```toml
# Configure these dependency relationships:
# - blog-web depends on blog-core and blog-storage
# - blog-cli depends on blog-core and blog-storage  
# - blog-storage depends on blog-core
# - All use shared workspace dependencies for serde, etc.
```

### **Exercise 3: Feature Flag Coordination**

Implement coordinated feature flags:

```toml
# Features to implement:
# - "database" feature: Enables SQL storage
# - "json-storage" feature: Enables JSON file storage
# - "web-ui" feature: Enables web interface  
# - "cli-tools" feature: Enables CLI utilities
# - Default: json-storage + cli-tools
```

---

## 🔗 Related Topics & Next Steps

### **Today's Connection to Mission 10**
- **Path compression optimization** benefits from workspace organization
- **Multiple Union-Find implementations** can be separate crates
- **CLI, web API, and core logic** naturally separate into workspace members
- **Benchmarking and examples** become separate crates for clean organization

### **Key Takeaways for Mission Work**
1. **Separation of concerns**: Core algorithm, interfaces, and applications
2. **Shared dependencies**: Consistent versions across related crates  
3. **Testing coordination**: Workspace-wide test execution and validation
4. **Documentation**: Generate unified documentation for all components

### **Tomorrow's Preview**: Publishing Crates
- Preparing crates for publication to crates.io
- Documentation standards and API design  
- Semantic versioning and release management
- Building a public API ecosystem

---

## 📚 Additional Resources

### **Official Documentation**
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Dependency Resolution](https://doc.rust-lang.org/cargo/reference/resolver.html) 
- [Publishing Workflow](https://doc.rust-lang.org/cargo/reference/publishing.html)

### **Best Practices Guides**
- [API Guidelines](https://rust-lang.github.io/api-guidelines/) - Standards for public APIs
- [Cargo Best Practices](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [Rust Project Structure](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

### **Real-World Examples**
- [Tokio Workspace](https://github.com/tokio-rs/tokio) - Large-scale async runtime
- [Serde Workspace](https://github.com/serde-rs/serde) - Serialization ecosystem
- [Clap Workspace](https://github.com/clap-rs/clap) - CLI parsing framework

---

*Tags: #day39 #week6 #workspace-management #multi-crate #cargo-workspaces #dependency-management #project-organization*

*Links: [[Day38]] | [[Day40]] | [[Mission 10 Path Compression]] | [[Cargo Features]] | [[Publishing Crates]]*