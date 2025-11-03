// Day 37 - Crate Organization (lib vs bin, module trees)
// Run with: cargo run --bin Day37

// ============================================
// Simulating src/lib.rs structure
// ============================================

/// Error types for the crate
#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    IndexOutOfBounds { index: usize, max: usize },
    InvalidOperation(String),
}

type Result<T> = std::result::Result<T, Error>;

// ============================================
// Simulating src/algorithms/union_find.rs
// ============================================

/// Optimized Union-Find with path compression and union by rank
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }
    
    fn find(&mut self, x: usize) -> Result<usize> {
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
    
    fn union(&mut self, x: usize, y: usize) -> Result<bool> {
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
    
    fn connected(&mut self, x: usize, y: usize) -> Result<bool> {
        Ok(self.find(x)? == self.find(y)?)
    }
    
    fn count(&self) -> usize {
        self.count
    }
}

// ============================================
// Main demonstration (simulating main.rs)
// ============================================

fn main() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║  Day 37: Crate Organization (lib vs bin, modules)    ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");
    
    demonstrate_library_crate();
    demonstrate_error_handling();
    demonstrate_api_design();
    
    println!("\n✅ All demonstrations complete!");
}

fn demonstrate_library_crate() {
    println!("=== Library Crate Demo: Union-Find ===\n");
    
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
}

fn demonstrate_error_handling() {
    println!("\n=== Error Handling Demo ===\n");
    
    let mut uf = UnionFind::new(5);
    
    // Valid operations
    println!("--- Valid Operations ---");
    match uf.find(0) {
        Ok(root) => println!("Root of 0: {}", root),
        Err(e) => println!("Error: {:?}", e),
    }
    
    match uf.find(4) {
        Ok(root) => println!("Root of 4: {}", root),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Invalid operations
    println!("\n--- Error Cases ---");
    match uf.find(100) {
        Ok(root) => println!("Root of 100: {}", root),
        Err(e) => println!("✓ Caught error: {:?}", e),
    }
    
    match uf.union(0, 100) {
        Ok(changed) => println!("union(0, 100): {}", changed),
        Err(e) => println!("✓ Caught error: {:?}", e),
    }
}

fn demonstrate_api_design() {
    println!("\n=== API Design Principles Demo ===\n");
    
    // Demonstrate encapsulation
    let mut uf = UnionFind::new(8);
    
    println!("Public API provides:");
    println!("  - new(n) - Creates structure");
    println!("  - find(x) - Finds root with error handling");
    println!("  - union(x, y) - Merges sets");
    println!("  - connected(x, y) - Checks connectivity");
    println!("  - count() - Returns number of sets");
    
    println!("\nInternal implementation hidden:");
    println!("  - parent: Vec<usize> (private)");
    println!("  - rank: Vec<usize> (private)");
    println!("  - find_unchecked() (private helper)");
    
    // Use the clean API
    println!("\n--- Using Clean API ---");
    let _ = uf.union(0, 1);
    let _ = uf.union(2, 3);
    let _ = uf.union(4, 5);
    let _ = uf.union(6, 7);
    
    println!("After 4 unions:");
    println!("  Component count: {}", uf.count());
    
    // Merge two components
    let _ = uf.union(1, 2);
    println!("After merging (0,1) and (2,3):");
    println!("  Component count: {}", uf.count());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.count(), 5);
        
        uf.union(0, 1).unwrap();
        assert_eq!(uf.count(), 4);
        
        assert!(uf.connected(0, 1).unwrap());
        assert!(!uf.connected(0, 2).unwrap());
    }
    
    #[test]
    fn test_error_handling() {
        let mut uf = UnionFind::new(5);
        
        // Valid index
        assert!(uf.find(0).is_ok());
        
        // Invalid index
        assert!(uf.find(10).is_err());
    }
    
    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new(10);
        
        // Create a chain: 0 -> 1 -> 2 -> 3
        uf.union(0, 1).unwrap();
        uf.union(1, 2).unwrap();
        uf.union(2, 3).unwrap();
        
        // Find should compress the path
        let root = uf.find(0).unwrap();
        assert_eq!(root, uf.find(3).unwrap());
    }
}
