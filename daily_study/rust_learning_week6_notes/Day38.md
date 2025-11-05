# Day 38 - Cargo Features (conditional compilation, feature flags)

**Learning Focus**: Mastering Cargo features for conditional compilation, feature flags, and flexible API design

**Date**: November 4, 2025 (Tuesday)

**Mission Alignment**: Mission 10 Union & Find Operations

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- How to define and use Cargo features in `Cargo.toml`
- Conditional compilation with `#[cfg]` attributes
- Feature flags for optional functionality
- Default features and feature dependencies
- How to design flexible APIs with optional capabilities
- Best practices for feature flag organization

---

## 📚 Core Concepts

### **1. What Are Cargo Features?**

Cargo features allow you to conditionally compile code, making parts of your library optional. This enables:
- **Smaller binaries**: Only compile what you need
- **Optional dependencies**: Include expensive dependencies only when needed
- **Platform-specific code**: Different behavior on different targets
- **API flexibility**: Users choose which functionality to enable

```toml
# Cargo.toml
[package]
name = "mission10"
version = "0.1.0"

[features]
# Default features (enabled by default)
default = ["std"]

# Individual features
std = []                              # Enable standard library features
serde = ["dep:serde"]                # Enable serialization with serde dependency
parallel = ["dep:rayon"]             # Enable parallel operations
benchmarking = ["dep:criterion"]     # Enable benchmarking utilities
debug-info = []                      # Enable debug information

# Optional dependencies (only included when feature is enabled)
[dependencies]
serde = { version = "1.0", optional = true }
rayon = { version = "1.7", optional = true }

[dev-dependencies]
criterion = { version = "0.5", optional = true }
```

**Feature Naming Conventions:**
- Use `kebab-case` for feature names
- Prefix debug/testing features: `debug-info`, `test-utils`
- Use descriptive names: `parallel` not `par`, `serialization` not `ser`

---

### **2. Conditional Compilation with `#[cfg]`**

The `#[cfg]` attribute lets you conditionally compile code based on features:

```rust
// src/lib.rs

// Always compiled
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
    
    // Only included when 'debug-info' feature is enabled
    #[cfg(feature = "debug-info")]
    operations_count: usize,
    
    #[cfg(feature = "debug-info")]
    merge_history: Vec<(usize, usize)>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
            
            #[cfg(feature = "debug-info")]
            operations_count: 0,
            
            #[cfg(feature = "debug-info")]
            merge_history: Vec::new(),
        }
    }
    
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        #[cfg(feature = "debug-info")]
        {
            self.operations_count += 1;
        }
        
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
        
        #[cfg(feature = "debug-info")]
        {
            self.merge_history.push((x, y));
        }
        
        self.count -= 1;
        true
    }
    
    // This entire method only exists when debug-info is enabled
    #[cfg(feature = "debug-info")]
    pub fn get_operations_count(&self) -> usize {
        self.operations_count
    }
    
    #[cfg(feature = "debug-info")]
    pub fn get_merge_history(&self) -> &[(usize, usize)] {
        &self.merge_history
    }
}

// Feature-specific modules
#[cfg(feature = "serde")]
mod serialization {
    use super::UnionFind;
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    pub struct SerializableUnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
        count: usize,
    }
    
    impl From<&UnionFind> for SerializableUnionFind {
        fn from(uf: &UnionFind) -> Self {
            Self {
                parent: uf.parent.clone(),
                rank: uf.rank.clone(),
                count: uf.count,
            }
        }
    }
}

#[cfg(feature = "serde")]
pub use serialization::*;
```

**Common `#[cfg]` Patterns:**
- `#[cfg(feature = "my-feature")]` - Feature enabled
- `#[cfg(not(feature = "my-feature"))]` - Feature disabled
- `#[cfg(all(feature = "a", feature = "b"))]` - Both features enabled
- `#[cfg(any(feature = "a", feature = "b"))]` - Either feature enabled
- `#[cfg(target_os = "windows")]` - Platform-specific code

---

### **3. Optional Dependencies**

Optional dependencies are only included when their corresponding feature is enabled:

```toml
# Cargo.toml
[dependencies]
# Always included
thiserror = "1.0"

# Optional dependencies
serde = { version = "1.0", optional = true }
rayon = { version = "1.7", optional = true }
tokio = { version = "1.0", features = ["full"], optional = true }

[features]
default = ["std"]
std = []
serialization = ["dep:serde"]
parallel = ["dep:rayon"]  
async = ["dep:tokio"]
all = ["serialization", "parallel", "async"]
```

```rust
// Only available when 'parallel' feature is enabled
#[cfg(feature = "parallel")]
pub fn parallel_union_many(&mut self, pairs: &[(usize, usize)]) {
    use rayon::prelude::*;
    
    // This is a simplified example - real parallel Union-Find is complex
    let operations: Vec<_> = pairs.par_iter()
        .map(|&(x, y)| (self.find(x), self.find(y)))
        .collect();
    
    for (root_x, root_y) in operations {
        if root_x != root_y {
            self.union_roots(root_x, root_y);
        }
    }
}

// Only available when 'async' feature is enabled
#[cfg(feature = "async")]
pub async fn async_large_union(&mut self, pairs: Vec<(usize, usize)>) -> tokio::task::JoinHandle<usize> {
    let mut count = 0;
    tokio::task::spawn(async move {
        for (x, y) in pairs {
            // Simulate expensive async work
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            count += 1;
        }
        count
    })
}
```

---

### **4. Feature Dependencies and Composition**

Features can depend on other features and compose together:

```toml
# Cargo.toml
[features]
default = ["std"]

# Basic features
std = []
alloc = []

# Serialization features
serde = ["dep:serde", "std"]
json = ["serde", "dep:serde_json"]
bincode = ["serde", "dep:bincode"]

# Performance features  
parallel = ["dep:rayon", "std"]
simd = ["dep:wide"]

# Convenience bundles
full = ["serde", "json", "parallel", "debug-info"]
minimal = ["alloc"]  # No std, just alloc

# Development features
debug-info = ["std"]
testing = ["debug-info", "dep:proptest"]
```

```rust
// Feature combination logic
impl UnionFind {
    #[cfg(all(feature = "serde", feature = "std"))]
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let serializable = SerializableUnionFind::from(self);
        let json = serde_json::to_string_pretty(&serializable)?;
        std::fs::write(path, json)
    }
    
    #[cfg(all(feature = "serde", not(feature = "std")))]  
    pub fn serialize_to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        let serializable = SerializableUnionFind::from(self);
        serde_json::to_vec(&serializable)
    }
    
    // Available with either std or alloc
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn with_capacity(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        parent.extend(0..n);
        
        Self {
            parent,
            rank: vec![0; n],
            count: n,
            
            #[cfg(feature = "debug-info")]
            operations_count: 0,
            
            #[cfg(feature = "debug-info")]
            merge_history: Vec::new(),
        }
    }
}
```

---

### **5. Platform and Environment Conditional Compilation**

Beyond features, you can conditionally compile based on target platform:

```rust
impl UnionFind {
    // Windows-specific optimizations
    #[cfg(target_os = "windows")]
    pub fn with_windows_optimization(n: usize) -> Self {
        // Use Windows-specific memory allocation
        let mut uf = Self::new(n);
        // Windows-specific setup code
        uf
    }
    
    // Unix-specific features
    #[cfg(unix)]
    pub fn with_mmap(n: usize) -> std::io::Result<Self> {
        // Use memory mapping on Unix systems
        // This is a simplified example
        Ok(Self::new(n))
    }
    
    // Debug vs release builds
    #[cfg(debug_assertions)]
    fn validate_bounds(&self, x: usize) {
        assert!(x < self.parent.len(), "Index {} out of bounds", x);
    }
    
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn validate_bounds(&self, _x: usize) {
        // No-op in release builds for performance
    }
    
    // Pointer width specific code
    #[cfg(target_pointer_width = "64")]
    pub fn optimized_for_64bit(&self) -> bool {
        true
    }
    
    #[cfg(not(target_pointer_width = "64"))]
    pub fn optimized_for_64bit(&self) -> bool {
        false
    }
}
```

**Common Target Conditions:**
- `target_os`: `"windows"`, `"macos"`, `"linux"`, `"android"`, `"ios"`
- `target_family`: `"unix"`, `"windows"`
- `target_arch`: `"x86"`, `"x86_64"`, `"arm"`, `"aarch64"`
- `target_pointer_width`: `"32"`, `"64"`

---

## Complete Runnable Example

Here's a comprehensive example demonstrating Cargo features in practice:

```rust
// This example shows how features work in practice
// In a real project, this would be spread across multiple files

use std::collections::HashMap;

/// Union-Find data structure with optional features
pub struct FeaturefulUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
    
    // Debug information (only with debug-info feature)
    #[cfg(feature = "debug-info")]
    operations: Vec<String>,
    
    #[cfg(feature = "debug-info")]
    operation_count: HashMap<String, usize>,
}

impl FeaturefulUnionFind {
    pub fn new(n: usize) -> Self {
        #[cfg(feature = "debug-info")]
        let mut operation_count = HashMap::new();
        #[cfg(feature = "debug-info")]
        operation_count.insert("new".to_string(), 1);
        
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
            
            #[cfg(feature = "debug-info")]
            operations: vec![format!("Created UnionFind with {} elements", n)],
            
            #[cfg(feature = "debug-info")]
            operation_count,
        }
    }
    
    pub fn find(&mut self, x: usize) -> usize {
        #[cfg(feature = "debug-info")]
        {
            *self.operation_count.entry("find".to_string()).or_insert(0) += 1;
            self.operations.push(format!("find({})", x));
        }
        
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        #[cfg(feature = "debug-info")]
        {
            *self.operation_count.entry("union".to_string()).or_insert(0) += 1;
            self.operations.push(format!("union({}, {})", x, y));
        }
        
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
    
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        #[cfg(feature = "debug-info")]
        {
            *self.operation_count.entry("connected".to_string()).or_insert(0) += 1;
            self.operations.push(format!("connected({}, {})", x, y));
        }
        
        self.find(x) == self.find(y)
    }
    
    pub fn count(&self) -> usize {
        self.count
    }
    
    // Debug feature methods
    #[cfg(feature = "debug-info")]
    pub fn print_debug_info(&self) {
        println!("=== Union-Find Debug Information ===");
        println!("Current component count: {}", self.count);
        println!("Total operations: {}", self.operations.len());
        
        println!("\nOperation counts:");
        for (op, count) in &self.operation_count {
            println!("  {}: {}", op, count);
        }
        
        println!("\nOperation history (last 5):");
        let start = if self.operations.len() > 5 { self.operations.len() - 5 } else { 0 };
        for op in &self.operations[start..] {
            println!("  {}", op);
        }
    }
    
    #[cfg(not(feature = "debug-info"))]
    pub fn print_debug_info(&self) {
        println!("Debug information not available. Enable 'debug-info' feature.");
    }
    
    // Serialization (only available with serde feature)
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        use serde_json::json;
        let data = json!({
            "parent": self.parent,
            "rank": self.rank,
            "count": self.count,
        });
        serde_json::to_string_pretty(&data)
    }
    
    #[cfg(not(feature = "serde"))]
    pub fn to_json(&self) -> Result<String, &'static str> {
        Err("Serialization not available. Enable 'serde' feature.")
    }
    
    // Parallel operations (only with parallel feature)
    #[cfg(feature = "parallel")]
    pub fn parallel_batch_union(&mut self, pairs: Vec<(usize, usize)>) -> Vec<bool> {
        // In a real implementation, this would be more sophisticated
        // This is a simplified demonstration
        pairs.into_iter()
            .map(|(x, y)| self.union(x, y))
            .collect()
    }
    
    #[cfg(not(feature = "parallel"))]
    pub fn parallel_batch_union(&mut self, pairs: Vec<(usize, usize)>) -> Vec<bool> {
        // Fallback to sequential processing
        pairs.into_iter()
            .map(|(x, y)| self.union(x, y))
            .collect()
    }
}

// Feature-specific trait implementations
#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    struct UnionFindSnapshot {
        parent: Vec<usize>,
        rank: Vec<usize>,
        count: usize,
    }
    
    impl FeaturefulUnionFind {
        pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
            let snapshot: UnionFindSnapshot = serde_json::from_str(json)?;
            Ok(Self {
                parent: snapshot.parent,
                rank: snapshot.rank,
                count: snapshot.count,
                
                #[cfg(feature = "debug-info")]
                operations: vec!["Loaded from JSON".to_string()],
                
                #[cfg(feature = "debug-info")]
                operation_count: {
                    let mut map = HashMap::new();
                    map.insert("from_json".to_string(), 1);
                    map
                },
            })
        }
    }
}

fn main() {
    println!("=== Cargo Features Demo ===\n");
    
    let mut uf = FeaturefulUnionFind::new(8);
    
    // Basic operations
    println!("--- Basic Operations ---");
    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(0, 2);
    
    println!("0 and 3 connected: {}", uf.connected(0, 3));
    println!("0 and 4 connected: {}", uf.connected(0, 4));
    println!("Components: {}", uf.count());
    
    // Debug info (conditional)
    println!("\n--- Debug Information ---");
    uf.print_debug_info();
    
    // Serialization demo (conditional)
    println!("\n--- Serialization ---");
    match uf.to_json() {
        Ok(json) => {
            println!("JSON representation:");
            println!("{}", json);
        }
        Err(e) => println!("Serialization error: {}", e),
    }
    
    // Parallel operations demo (conditional)
    println!("\n--- Parallel Operations ---");
    let pairs = vec![(4, 5), (6, 7), (5, 6)];
    let results = uf.parallel_batch_union(pairs);
    println!("Batch union results: {:?}", results);
    println!("Final components: {}", uf.count());
    
    // Platform-specific info
    println!("\n--- Platform Information ---");
    println!("Running on: {}", std::env::consts::OS);
    println!("Architecture: {}", std::env::consts::ARCH);
    
    #[cfg(debug_assertions)]
    println!("Build type: Debug");
    
    #[cfg(not(debug_assertions))]
    println!("Build type: Release");
    
    println!("\n✅ Features demonstration complete!");
    
    // Show which features are currently enabled
    println!("\n--- Feature Status ---");
    
    #[cfg(feature = "debug-info")]
    println!("✅ debug-info feature enabled");
    #[cfg(not(feature = "debug-info"))]
    println!("❌ debug-info feature disabled");
    
    #[cfg(feature = "serde")]
    println!("✅ serde feature enabled");
    #[cfg(not(feature = "serde"))]
    println!("❌ serde feature disabled");
    
    #[cfg(feature = "parallel")]
    println!("✅ parallel feature enabled");
    #[cfg(not(feature = "parallel"))]
    println!("❌ parallel feature disabled");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        let mut uf = FeaturefulUnionFind::new(5);
        assert_eq!(uf.count(), 5);
        
        assert!(uf.union(0, 1));
        assert_eq!(uf.count(), 4);
        assert!(uf.connected(0, 1));
    }
    
    #[cfg(feature = "debug-info")]
    #[test]
    fn test_debug_features() {
        let mut uf = FeaturefulUnionFind::new(3);
        uf.union(0, 1);
        uf.find(0);
        
        // With debug-info, we should have operation history
        uf.print_debug_info(); // This shouldn't panic
    }
    
    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization() {
        let uf = FeaturefulUnionFind::new(3);
        let json = uf.to_json().expect("Should serialize");
        assert!(json.contains("parent"));
        assert!(json.contains("rank"));
        assert!(json.contains("count"));
    }
}
```

**Output (with debug-info feature enabled):**
```
=== Cargo Features Demo ===

--- Basic Operations ---
0 and 3 connected: true
0 and 4 connected: false
Components: 5

--- Debug Information ---
=== Union-Find Debug Information ===
Current component count: 5
Total operations: 6

Operation counts:
  new: 1
  union: 3
  find: 6
  connected: 2

Operation history (last 5):
  union(0, 2)
  connected(0, 3)
  find(0)
  find(3)
  connected(0, 4)

--- Serialization ---
Serialization not available. Enable 'serde' feature.

--- Parallel Operations ---
Batch union results: [true, true, true]
Final components: 2

--- Platform Information ---
Running on: windows
Architecture: x86_64
Build type: Debug

✅ Features demonstration complete!

--- Feature Status ---
✅ debug-info feature enabled
❌ serde feature disabled
❌ parallel feature disabled
```

---

## 🔍 Deep Dive Analysis

### **Feature Flag Best Practices**

1. **Start with default features that work everywhere**
```toml
[features]
default = ["std"]  # Safe, widely compatible
```

2. **Make expensive dependencies optional**
```toml
[dependencies]
tokio = { version = "1.0", optional = true }  # Heavy async runtime
serde = { version = "1.0", optional = true }  # Only for serialization
```

3. **Use feature bundles for common combinations**
```toml
[features]
full = ["serde", "parallel", "async"]
minimal = []  # Bare minimum
```

4. **Document your features clearly**
```toml
## Features
# - `serde`: Enable serialization support
# - `parallel`: Enable parallel processing with rayon  
# - `debug-info`: Include debug information and tracing
```

### **Testing with Features**

```bash
# Test with default features
cargo test

# Test with no features
cargo test --no-default-features

# Test with specific features
cargo test --features "serde,debug-info"

# Test all feature combinations (use cargo-hack)
cargo install cargo-hack
cargo hack test --feature-powerset
```

### **Feature Composition Patterns**

```rust
// Pattern 1: Layered features
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::collections::BTreeMap as HashMap;

// Pattern 2: Fallback implementations
#[cfg(feature = "parallel")]
fn process_parallel<T>(items: Vec<T>) -> Vec<T> {
    use rayon::prelude::*;
    items.into_par_iter().map(|x| x).collect()
}

#[cfg(not(feature = "parallel"))]
fn process_parallel<T>(items: Vec<T>) -> Vec<T> {
    items.into_iter().map(|x| x).collect()
}

// Pattern 3: Optional trait implementations
#[cfg(feature = "serde")]
impl serde::Serialize for UnionFind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        // Implementation
        todo!()
    }
}
```

---

## 🚨 Common Pitfalls

### **Pitfall 1: Breaking API Changes in Features**

```rust
// ❌ BAD: Different public APIs based on features
impl UnionFind {
    #[cfg(feature = "advanced")]
    pub fn find(&mut self, x: usize) -> Result<usize, Error> {
        // Returns Result when advanced feature is enabled
    }
    
    #[cfg(not(feature = "advanced"))]
    pub fn find(&mut self, x: usize) -> usize {
        // Returns usize when advanced feature is disabled
    }
}
```

**Solution**: Keep public API stable, vary internal implementation

```rust
// ✅ GOOD: Consistent public API
impl UnionFind {
    pub fn find(&mut self, x: usize) -> usize {
        #[cfg(feature = "advanced")]
        {
            self.find_with_advanced_checks(x).unwrap_or_else(|_| {
                panic!("Find operation failed");
            })
        }
        
        #[cfg(not(feature = "advanced"))]
        {
            self.find_simple(x)
        }
    }
}
```

### **Pitfall 2: Feature Explosion**

```rust
// ❌ BAD: Too many granular features
[features]
enable-bounds-checking = []
enable-operation-counting = []
enable-history-tracking = []
enable-json-serialization = []
enable-binary-serialization = []
enable-async-find = []
enable-async-union = []
```

**Solution**: Group related features logically

```rust
// ✅ GOOD: Logical feature groups
[features]
debug = ["bounds-checking", "operation-counting", "history-tracking"]
serialization = ["json", "binary"]
async = ["async-operations"]
```

### **Pitfall 3: Circular Feature Dependencies**

```toml
# ❌ BAD: Circular dependencies
[features]
feature-a = ["feature-b"]
feature-b = ["feature-a"]  # Creates a cycle!
```

**Solution**: Design clear feature hierarchy

```toml
# ✅ GOOD: Clear hierarchy
[features]
base = []
advanced = ["base"]  
full = ["advanced", "serialization"]
```

---

## 🎯 Practice Exercises

### **Exercise 1: Design Feature Flags**

You're building a graph library. Design appropriate feature flags for:
- Serialization support (JSON, binary)
- Parallel algorithms
- Visualization capabilities 
- Different graph representations (adjacency list vs matrix)
- Debug and profiling tools

```toml
# Your solution here
[features]
# Design the feature structure
```

### **Exercise 2: Conditional API Design**

Implement a `Graph` struct that:
- Has basic functionality always available
- Adds serialization methods only when `serde` feature is enabled
- Provides parallel algorithms only when `parallel` feature is enabled
- Includes debug information only when `debug` feature is enabled

```rust
// Your implementation here
pub struct Graph {
    // Design the structure with conditional fields
}
```

### **Exercise 3: Feature Testing Strategy**

Write a test plan for testing all feature combinations of a crate with features:
`default = ["std"]`, `serde`, `parallel`, `debug`, `full = ["serde", "parallel", "debug"]`

```bash
# Your testing commands here
```

---

## 🔗 Integration Points

### **Mission 10 Connection**

Today's learning directly enhances Mission 10:
- **REQ-2**: Add optional optimizations with feature flags
- **REQ-4**: Enable conditional debugging information
- **REQ-6**: Provide serialization as optional feature
- Mission 10 `Cargo.toml` will include features for benchmarking, serialization, and debug tools

### **Real-World Applications**

```toml
# Mission 10 enhanced Cargo.toml
[package]
name = "mission10"
version = "0.1.0"

[features]
default = ["std"]
std = []
serde = ["dep:serde", "dep:serde_json"]
parallel = ["dep:rayon"]
debug-info = ["std"]
benchmarking = ["dep:criterion"]
full = ["serde", "parallel", "debug-info"]

[dependencies]
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }
rayon = { version = "1.7", optional = true }

[dev-dependencies]
criterion = { version = "0.5", optional = true }
```

### **Zettelkasten Connections**

- **[[mission-10]]**: Apply conditional compilation to Union-Find implementation
- **[[Cargo and Package Management]]**: Advanced Cargo configuration patterns
- **[[API Design]]**: How features affect public API design
- **[[Testing Strategies]]**: Testing with different feature combinations

---

## 💡 Key Takeaways

1. **Features enable optional functionality** without breaking core APIs
2. **`#[cfg(feature = "name")]`** conditionally compiles code based on enabled features
3. **Optional dependencies** are only included when their features are enabled
4. **Default features** should work everywhere and be lightweight
5. **Feature composition** allows users to choose exactly what they need
6. **Test all feature combinations** to ensure compatibility
7. **Keep public APIs stable** across feature configurations

---

## 📋 Tomorrow's Preview

Tomorrow (Day 39) we'll explore **Workspace management** for multi-crate projects, learning how to:
- Organize related crates in a workspace
- Share dependencies and configurations
- Manage inter-crate dependencies
- Build and test workspace members efficiently

This will help us understand how the Mission Tutorial system integrates with main missions!

---

*Tags: #daily-study #week-6 #cargo-features #conditional-compilation #feature-flags #mission-10*

*Links: [[zettel-index]] | [[daily-study/Day37]] | [[daily-study/Day39]] | [[Week 6 Overview]] | [[mission-10]]*