# Day 37 - Crate Organization (lib vs bin, module trees)

**Learning Focus**: Understanding how to structure Rust projects with library and binary crates, and organizing module hierarchies

**Date**: November 3, 2025 (Monday)

**Mission Alignment**: Mission 10 Requirements Definition & Basic Structure

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- The difference between library crates (`lib.rs`) and binary crates (`main.rs`)
- When to use `lib.rs` vs `main.rs` vs both
- How to structure module trees for scalability
- The relationship between files, folders, and module paths
- Best practices for public API design in library crates
- How to organize tests alongside your code

---

## 📚 Core Concepts

### **1. Library vs Binary Crates**

Rust projects can have three types of crates:
- **Library crate** (`src/lib.rs`) - Reusable code that other crates can depend on
- **Binary crate** (`src/main.rs`) - Executable program
- **Hybrid** - Both `lib.rs` and `main.rs` in the same project

```rust
// src/lib.rs - Library crate
// This is the public API that other crates will import

/// Union-Find data structure (Disjoint Set Union)
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    /// Creates a new Union-Find structure with n elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }
    
    /// Find the root of element x with path compression
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    /// Union two elements
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return false;
        }
        
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        
        self.count -= 1;
        true
    }
}
```

```rust
// src/main.rs - Binary crate (optional)
// Uses the library crate to create an executable

use mission10::UnionFind; // Import from lib.rs

fn main() {
    let mut uf = UnionFind::new(10);
    uf.union(0, 1);
    uf.union(1, 2);
    
    println!("Root of 0: {}", uf.find(0));
    println!("Root of 2: {}", uf.find(2));
}
```

**When to use what:**
- **lib.rs only**: Pure library (like `serde`, `tokio`)
- **main.rs only**: Simple CLI tools or scripts
- **Both**: Complex applications with reusable logic (most real projects)

---

### **2. Module Tree Organization**

Rust uses the filesystem to mirror module structure:

```
src/
├── lib.rs                  # Crate root (declares modules)
├── main.rs                 # Binary entry point (optional)
├── core.rs                 # Inline module file
├── utils/                  # Module as directory
│   ├── mod.rs             # Module definition
│   ├── helpers.rs         # Submodule
│   └── validators.rs      # Submodule
└── algorithms/
    ├── mod.rs             # Module definition
    ├── union_find.rs      # Submodule
    └── graph.rs           # Submodule
```

```rust
// src/lib.rs
// Declare modules (makes them part of the crate)
pub mod core;           // Looks for src/core.rs or src/core/mod.rs
pub mod utils;          // Looks for src/utils.rs or src/utils/mod.rs
pub mod algorithms;     // Looks for src/algorithms.rs or src/algorithms/mod.rs

// Re-export important types from submodules
pub use algorithms::union_find::UnionFind;
pub use algorithms::graph::Graph;

// Prelude pattern: common imports users will want
pub mod prelude {
    pub use crate::algorithms::union_find::UnionFind;
    pub use crate::algorithms::graph::Graph;
    pub use crate::core::*;
}
```

```rust
// src/algorithms/mod.rs
// Declares submodules within the algorithms module
pub mod union_find;
pub mod graph;

// Can re-export at this level too
pub use union_find::UnionFind;
pub use graph::Graph;
```

```rust
// src/algorithms/union_find.rs
// Actual implementation

/// Optimized Union-Find with path compression and union by rank
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }
    
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}

// Internal helpers (not pub, so private to this module)
fn validate_index(x: usize, n: usize) -> Result<(), String> {
    if x >= n {
        Err(format!("Index {} out of bounds (max: {})", x, n - 1))
    } else {
        Ok(())
    }
}
```

**Key Rules:**
1. **`mod.rs` or inline**: `src/foo/mod.rs` or `src/foo.rs` (not both!)
2. **Privacy by default**: Use `pub mod` to expose modules
3. **Path resolution**: `crate::module::submodule::Item`

---

### **3. Public API Design**

When creating a library, carefully choose what to expose:

```rust
// src/lib.rs - Well-designed public API

// Internal implementation details (not pub)
mod internal {
    pub(crate) struct InternalHelper {
        // Only visible within this crate
    }
}

// Public API
pub struct UnionFind {
    // Private fields - users can't access directly
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    // Public constructor
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }
    
    // Public methods
    pub fn find(&mut self, x: usize) -> usize {
        self.validate_bounds(x);
        self.find_internal(x)
    }
    
    // Private helper (not pub, so internal only)
    fn find_internal(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find_internal(self.parent[x]);
        }
        self.parent[x]
    }
    
    fn validate_bounds(&self, x: usize) {
        assert!(x < self.parent.len(), "Index out of bounds");
    }
}

// Re-export commonly used types
pub use internal::InternalHelper; // If we want to expose it
```

**API Design Principles:**
- **Minimize public surface**: Only expose what users need
- **Encapsulate internals**: Keep implementation details private
- **Use re-exports**: Flatten complex hierarchies for users
- **Provide preludes**: Common imports in one place

---

### **4. Module Tree Patterns**

#### **Flat Organization** (small projects)
```
src/
├── lib.rs
├── union_find.rs
├── graph.rs
└── utils.rs
```

```rust
// src/lib.rs
pub mod union_find;
pub mod graph;
mod utils; // Private module

pub use union_find::UnionFind;
pub use graph::Graph;
```

#### **Hierarchical Organization** (larger projects)
```
src/
├── lib.rs
├── data_structures/
│   ├── mod.rs
│   ├── union_find.rs
│   ├── heap.rs
│   └── tree.rs
├── algorithms/
│   ├── mod.rs
│   ├── sorting.rs
│   └── searching.rs
└── utils/
    ├── mod.rs
    └── helpers.rs
```

```rust
// src/lib.rs
pub mod data_structures;
pub mod algorithms;
mod utils;

// Flatten for users
pub use data_structures::union_find::UnionFind;
pub use algorithms::sorting::quick_sort;
```

#### **Feature-Based Organization** (domain-driven)
```
src/
├── lib.rs
├── connectivity/      # Feature: connectivity queries
│   ├── mod.rs
│   ├── union_find.rs
│   └── dynamic.rs
├── pathfinding/       # Feature: path algorithms
│   ├── mod.rs
│   ├── dijkstra.rs
│   └── astar.rs
└── components/        # Feature: component analysis
    ├── mod.rs
    └── detector.rs
```

---

## 🧪 Complete Runnable Example

Here's a complete example demonstrating crate organization:

```rust
// This would be in separate files in a real project
// Shown inline here for demonstration

// ============================================
// src/lib.rs
// ============================================

/// The mission10 crate provides efficient disjoint set data structures
pub mod core;
pub mod algorithms;

// Re-export main types
pub use algorithms::union_find::UnionFind;

pub mod prelude {
    pub use crate::algorithms::union_find::UnionFind;
}

// ============================================
// src/core.rs
// ============================================

pub mod core {
    /// Error types for the crate
    #[derive(Debug)]
    pub enum Error {
        IndexOutOfBounds { index: usize, max: usize },
        InvalidOperation(String),
    }
    
    pub type Result<T> = std::result::Result<T, Error>;
}

// ============================================
// src/algorithms/mod.rs
// ============================================

pub mod algorithms {
    pub mod union_find;
}

// ============================================
// src/algorithms/union_find.rs
// ============================================

pub mod union_find {
    use crate::core::{Error, Result};
    
    pub struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
        count: usize,
    }
    
    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
                rank: vec![0; n],
                count: n,
            }
        }
        
        pub fn find(&mut self, x: usize) -> Result<usize> {
            if x >= self.parent.len() {
                return Err(Error::IndexOutOfBounds {
                    index: x,
                    max: self.parent.len() - 1,
                });
            }
            Ok(self.find_unchecked(x))
        }
        
        fn find_unchecked(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find_unchecked(self.parent[x]);
            }
            self.parent[x]
        }
        
        pub fn union(&mut self, x: usize, y: usize) -> Result<bool> {
            let root_x = self.find(x)?;
            let root_y = self.find(y)?;
            
            if root_x == root_y {
                return Ok(false);
            }
            
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
            
            self.count -= 1;
            Ok(true)
        }
        
        pub fn connected(&mut self, x: usize, y: usize) -> Result<bool> {
            Ok(self.find(x)? == self.find(y)?)
        }
        
        pub fn count(&self) -> usize {
            self.count
        }
    }
}

// ============================================
// DEMONSTRATION (would be in main.rs or test)
// ============================================

fn main() {
    println!("=== Mission 10: Crate Organization Demo ===\n");
    
    // Using the prelude
    use union_find::UnionFind;
    
    // Create Union-Find with 10 elements
    let mut uf = UnionFind::new(10);
    println!("Created Union-Find with 10 elements");
    println!("Initial component count: {}", uf.count());
    
    // Perform unions
    println!("\n--- Performing Unions ---");
    match uf.union(0, 1) {
        Ok(changed) => println!("union(0, 1): {}", if changed { "merged" } else { "already connected" }),
        Err(e) => println!("Error: {:?}", e),
    }
    
    match uf.union(2, 3) {
        Ok(changed) => println!("union(2, 3): {}", if changed { "merged" } else { "already connected" }),
        Err(e) => println!("Error: {:?}", e),
    }
    
    match uf.union(0, 2) {
        Ok(changed) => println!("union(0, 2): {}", if changed { "merged" } else { "already connected" }),
        Err(e) => println!("Error: {:?}", e),
    }
    
    println!("Component count after unions: {}", uf.count());
    
    // Check connectivity
    println!("\n--- Connectivity Queries ---");
    match uf.connected(0, 3) {
        Ok(connected) => println!("0 and 3 connected: {}", connected),
        Err(e) => println!("Error: {:?}", e),
    }
    
    match uf.connected(0, 4) {
        Ok(connected) => println!("0 and 4 connected: {}", connected),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Error handling demo
    println!("\n--- Error Handling ---");
    match uf.find(100) {
        Ok(root) => println!("Root of 100: {}", root),
        Err(e) => println!("Error finding 100: {:?}", e),
    }
    
    println!("\n✅ Demonstration complete!");
}
```

**Output:**
```
=== Mission 10: Crate Organization Demo ===

Created Union-Find with 10 elements
Initial component count: 10

--- Performing Unions ---
union(0, 1): merged
union(2, 3): merged
union(0, 2): merged
Component count after unions: 7

--- Connectivity Queries ---
0 and 3 connected: true
0 and 4 connected: false

--- Error Handling ---
Error finding 100: IndexOutOfBounds { index: 100, max: 9 }

✅ Demonstration complete!
```

---

## 🔍 Deep Dive Analysis

### **How Module Resolution Works**

When you write `mod foo;`, Rust looks for:
1. `src/foo.rs` (inline module file)
2. `src/foo/mod.rs` (directory module)

**Never have both!** Pick one pattern and stick with it.

### **Modern Style (Rust 2018+)**

Prefer `foo.rs` over `foo/mod.rs` when possible:

```
src/
├── lib.rs
├── algorithms.rs      # ✅ Modern style
└── utils.rs           # ✅ Modern style

# Instead of:
src/
├── lib.rs
├── algorithms/
│   └── mod.rs         # ❌ Old style
└── utils/
    └── mod.rs         # ❌ Old style
```

Only use `mod.rs` when you have submodules:

```
src/
├── lib.rs
├── algorithms/        # Has submodules, so use mod.rs
│   ├── mod.rs
│   ├── union_find.rs
│   └── graph.rs
└── utils.rs           # No submodules, inline is fine
```

### **The `crate::` Path**

Always use `crate::` for absolute paths within your crate:

```rust
// src/algorithms/union_find.rs

// ✅ Good: Absolute path from crate root
use crate::core::Error;

// ❌ Avoid: Relative paths are confusing
use super::super::core::Error;
```

### **Re-exports for Better APIs**

```rust
// src/lib.rs

// Internal organization
pub mod data_structures {
    pub mod union_find {
        pub struct UnionFind { /* ... */ }
    }
}

// Flatten for users
pub use data_structures::union_find::UnionFind;

// Users can now write:
// use mylib::UnionFind;
// Instead of:
// use mylib::data_structures::union_find::UnionFind;
```

---

## 🚨 Common Pitfalls

### **Pitfall 1: Circular Dependencies**

```rust
// ❌ BAD: Circular dependency

// src/a.rs
use crate::b::TypeB;
pub struct TypeA {
    b: TypeB,
}

// src/b.rs
use crate::a::TypeA;
pub struct TypeB {
    a: TypeA,  // Error: circular dependency!
}
```

**Solution**: Use traits or extract common types to a third module

```rust
// ✅ GOOD: Break the cycle

// src/common.rs
pub trait Thing {
    fn do_thing(&self);
}

// src/a.rs
use crate::common::Thing;
pub struct TypeA;
impl Thing for TypeA {
    fn do_thing(&self) { }
}

// src/b.rs
use crate::common::Thing;
pub struct TypeB {
    thing: Box<dyn Thing>,  // Use trait object
}
```

### **Pitfall 2: Over-nesting**

```rust
// ❌ BAD: Too many levels

// use mylib::data_structures::graphs::directed::weighted::algorithms::shortest_path::dijkstra::Dijkstra;
```

**Solution**: Use re-exports to flatten

```rust
// ✅ GOOD: Flatten with re-exports

// src/lib.rs
pub mod algorithms {
    pub use crate::data_structures::graphs::directed::weighted::algorithms::shortest_path::dijkstra::Dijkstra;
}

// Users write:
// use mylib::algorithms::Dijkstra;
```

### **Pitfall 3: Exposing Internal Types**

```rust
// ❌ BAD: Leaking internal types into public API

pub struct UnionFind {
    pub parent: Vec<usize>,  // Users can break invariants!
}
```

**Solution**: Keep fields private, provide methods

```rust
// ✅ GOOD: Encapsulate internals

pub struct UnionFind {
    parent: Vec<usize>,  // Private
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn find(&mut self, x: usize) -> usize {
        // Controlled access
    }
}
```

---

## 🎯 Practice Exercises

### **Exercise 1: Organize a Multi-Module Project**

Create a project structure for a graph library with:
- Union-Find data structure
- Graph representation (adjacency list)
- BFS and DFS algorithms
- Shortest path algorithms (Dijkstra, A*)

Design the module tree and write the `src/lib.rs` with appropriate re-exports.

```rust
// Your solution here
// Structure:
// src/
// ├── lib.rs
// ├── data_structures/
// │   ├── mod.rs
// │   ├── union_find.rs
// │   └── graph.rs
// └── algorithms/
//     ├── mod.rs
//     ├── traversal.rs (BFS/DFS)
//     └── shortest_path.rs (Dijkstra/A*)
```

### **Exercise 2: Design a Public API**

You have an internal module structure. Design a clean public API using re-exports:

```rust
// Internal structure:
mod internal {
    pub mod parsing {
        pub struct Parser { }
        pub struct Token { }
    }
    pub mod validation {
        pub struct Validator { }
    }
}

// Your task: Create a clean public API
// Users should be able to write:
// use mylib::Parser;
// use mylib::Token;
// use mylib::Validator;
```

### **Exercise 3: Convert to Modern Module Style**

Convert this old-style structure to modern Rust 2018+ style:

```
src/
├── lib.rs
├── utils/
│   └── mod.rs
├── algorithms/
│   └── mod.rs
└── data/
    └── mod.rs
```

**Hint**: Use inline module files where possible.

---

## 🔗 Integration Points

### **Mission 10 Connection**

Today's learning directly supports Mission 10 requirements:
- **REQ-1**: Organize Union-Find implementation in a library crate
- **REQ-6**: Design public API with proper visibility
- Mission structure uses `src/lib.rs` for reusable code
- Examples use `src/main.rs` and `examples/` directory

### **Real-World Applications**

```rust
// Mission 10 will have this structure:
missions/Mission10/
├── Cargo.toml
├── src/
│   ├── lib.rs          # UnionFind public API
│   └── main.rs         # Optional demo binary
├── examples/
│   ├── demo.rs
│   ├── network.rs
│   └── kruskal_mst.rs
├── tests/
│   ├── unit_tests.rs
│   └── integration_tests.rs
└── benches/
    └── performance.rs
```

### **Zettelkasten Connections**

- **[[Ownership and Borrowing]]**: Module boundaries affect borrowing rules
- **[[Traits]]**: Use traits to break circular dependencies
- **[[mission-10]]**: Apply today's learning to Mission 10 structure
- **[[Testing Strategies]]**: Module organization affects test organization

---

## 💡 Key Takeaways

1. **Library crates (`lib.rs`)** expose reusable APIs; **binary crates (`main.rs`)** create executables
2. **Module trees** mirror filesystem structure but are controlled by `mod` declarations
3. **Privacy is default**; use `pub` strategically to design clean APIs
4. **Re-exports flatten complexity** for users of your library
5. **Modern style prefers `foo.rs`** over `foo/mod.rs` when there are no submodules
6. **Encapsulation is key**: Hide implementation details, expose only what's needed

---

## 📋 Tomorrow's Preview

Tomorrow (Day 38) we'll dive into **Cargo features** and conditional compilation, learning how to:
- Add optional features to crates with feature flags
- Use `#[cfg]` for conditional compilation
- Design flexible APIs with opt-in functionality
- Manage feature dependencies

This will enhance Mission 10 by allowing optional optimizations and serialization support!

---

*Tags: #daily-study #week-6 #modules #crate-organization #api-design #mission-10*

*Links: [[zettel-index]] | [[daily-study/Day36]] | [[daily-study/Day38]] | [[Week 6 Overview]] | [[mission-10]]*
