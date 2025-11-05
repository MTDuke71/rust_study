// Day 39 - Workspace Management (multi-crate projects)
// Demonstrating workspace configuration and multi-crate project organization

use std::collections::HashMap;

/// Core Union-Find implementation for workspace demonstration
#[derive(Debug, Clone)]
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
}

impl<T> Default for UnionFind<T> 
where 
    T: Clone + Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("=== Day 39: Workspace Management Demonstration ===");
    println!();
    
    // Demonstrate workspace concepts through Union-Find example
    demonstrate_workspace_concepts();
    
    println!();
    demonstrate_multi_crate_benefits();
    
    println!();  
    demonstrate_dependency_management();
    
    println!("=== Workspace Management Complete! ===");
}

fn demonstrate_workspace_concepts() {
    println!("📦 Workspace Structure Concepts:");
    println!("
    Typical multi-crate workspace for Union-Find project:
    
    union-find-workspace/
    ├── Cargo.toml                    # Workspace manifest
    ├── Cargo.lock                    # Shared dependency resolution  
    ├── union-find-core/              # Core algorithm library
    │   ├── Cargo.toml
    │   ├── src/lib.rs
    │   └── tests/
    ├── union-find-cli/               # Command-line interface
    │   ├── Cargo.toml  
    │   ├── src/main.rs
    │   └── tests/
    ├── union-find-web/               # REST API server
    │   ├── Cargo.toml
    │   ├── src/main.rs
    │   └── tests/
    ├── union-find-benchmarks/        # Performance testing
    │   ├── Cargo.toml
    │   └── benches/
    └── union-find-examples/          # Usage examples
        ├── Cargo.toml
        └── examples/
    ");
    
    println!("Benefits of this organization:");
    println!("  • Clear separation of concerns");
    println!("  • Shared dependency management");  
    println!("  • Independent compilation and testing");
    println!("  • Modular development and deployment");
}

fn demonstrate_multi_crate_benefits() {
    println!("🎯 Multi-Crate Benefits Demonstration:");
    
    // Simulate core library functionality
    let mut uf = UnionFind::new();
    
    println!("\n1. Core Library (union-find-core):");
    println!("   - Pure algorithm implementation");
    println!("   - No external dependencies beyond std");
    println!("   - Focused, testable, reusable");
    
    // Make sets for social network example
    let users = ["alice", "bob", "charlie", "diana", "eve"];
    for &user in &users {
        uf.make_set(user);
    }
    println!("   Created {} user sets", uf.count());
    
    println!("\n2. CLI Application (union-find-cli):");  
    println!("   - Uses core library as dependency");
    println!("   - Adds clap for argument parsing");
    println!("   - Command-line interface for operations");
    
    // Simulate CLI operations
    uf.union(&"alice", &"bob");
    uf.union(&"charlie", &"diana");
    println!("   Simulated: uf union alice bob");
    println!("   Simulated: uf union charlie diana");
    println!("   Result: {} connected components", uf.count());
    
    println!("\n3. Web API (union-find-web):");
    println!("   - Uses core library as dependency");  
    println!("   - Adds axum/tokio for async web server");
    println!("   - REST endpoints for remote operations");
    
    // Simulate web API operations
    let alice_charlie_connected = uf.connected(&"alice", &"charlie");
    let alice_bob_connected = uf.connected(&"alice", &"bob");
    println!("   GET /connected?x=alice&y=charlie -> {}", alice_charlie_connected);
    println!("   GET /connected?x=alice&y=bob -> {}", alice_bob_connected);
    
    println!("\n4. Benchmarks (union-find-benchmarks):");
    println!("   - Uses core library as dependency");
    println!("   - Adds criterion for performance testing"); 
    println!("   - Isolated performance validation");
    println!("   - No impact on core library size");
}

fn demonstrate_dependency_management() {
    println!("🔧 Workspace Dependency Management:");
    
    println!("\nWorkspace-level dependency coordination:");
    println!("
    # Root Cargo.toml
    [workspace.dependencies]
    serde = {{ version = \"1.0\", features = [\"derive\"] }}
    clap = {{ version = \"4.0\", features = [\"derive\"] }}
    tokio = {{ version = \"1.0\", features = [\"full\"] }}
    criterion = {{ version = \"0.5\", features = [\"html_reports\"] }}
    
    # Individual crate Cargo.toml files inherit:
    [dependencies]
    serde = {{ workspace = true }}          # Uses workspace version
    clap = {{ workspace = true }}           # CLI crate only
    
    # Benefits:
    # ✅ Consistent versions across all crates
    # ✅ Single place to update dependencies  
    # ✅ Reduced Cargo.lock complexity
    # ✅ Better dependency resolution
    ");
    
    println!("Workspace commands demonstration:");
    println!("
    # Build all crates in workspace
    cargo build --workspace
    
    # Test all crates
    cargo test --workspace
    
    # Build specific crate only  
    cargo build -p union-find-core
    
    # Run specific binary
    cargo run -p union-find-cli -- --help
    
    # Generate documentation for all
    cargo doc --workspace --open
    
    # Format all code
    cargo fmt --workspace
    
    # Lint all code
    cargo clippy --workspace -- -D warnings
    ");
    
    println!("Path dependencies for local development:");
    println!("
    # union-find-cli/Cargo.toml
    [dependencies]
    union-find-core = {{ path = \"../union-find-core\" }}
    clap = {{ workspace = true }}
    
    # union-find-web/Cargo.toml  
    [dependencies]
    union-find-core = {{ path = \"../union-find-core\" }}
    axum = {{ workspace = true }}
    tokio = {{ workspace = true }}
    ");
    
    println!("✨ Result: Modular, maintainable, scalable project structure!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_union_find() {
        let mut uf = UnionFind::new();
        
        // Test basic operations
        uf.make_set(1);
        uf.make_set(2);
        uf.make_set(3);
        
        assert_eq!(uf.count(), 3);
        assert!(!uf.connected(&1, &2));
        
        uf.union(&1, &2);
        assert!(uf.connected(&1, &2));
        assert_eq!(uf.count(), 2);
    }

    #[test]  
    fn test_path_compression() {
        let mut uf = UnionFind::new();
        
        // Create chain: 1 -> 2 -> 3 -> 4
        for i in 1..=4 {
            uf.make_set(i);
        }
        
        uf.union(&1, &2);
        uf.union(&2, &3); 
        uf.union(&3, &4);
        
        // All should be connected after path compression
        assert!(uf.connected(&1, &4));
        
        // Path compression should make subsequent finds faster
        let root1 = uf.find(&1).unwrap();
        let root4 = uf.find(&4).unwrap();
        assert_eq!(root1, root4);
    }
}