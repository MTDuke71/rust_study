// Mission 10 Tutorial - Step 2: Path Compression Optimization
// Building on Step 1 to add the first major optimization

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Mission 10 Tutorial - Step 2: Path Compression          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("=== What is Path Compression? ===");
    println!("In Step 1, we saw that trees can become chains, making find() O(n).");
    println!("Path compression is a simple optimization that flattens the tree");
    println!("during find(), making subsequent finds much faster.\n");

    // Part 1: Review the Problem
    println!("=== Part 1: The Problem with Basic Union-Find ===\n");
    demonstrate_problem();

    // Part 2: Path Compression Solution
    println!("\n=== Part 2: Path Compression Solution ===\n");
    demonstrate_path_compression();

    // Part 3: Side-by-Side Comparison
    println!("\n=== Part 3: Side-by-Side Comparison ===\n");
    side_by_side_comparison();

    // Part 4: Visualizing the Compression
    println!("\n=== Part 4: Visualizing Path Compression ===\n");
    visualize_compression();

    // Part 5: Performance Impact
    println!("\n=== Part 5: Performance Impact ===\n");
    performance_analysis();

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║              Key Takeaways                                ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ • Path compression flattens trees during find()          ║");
    println!("║ • Makes all nodes on path point directly to root         ║");
    println!("║ • Dramatically improves subsequent find() calls          ║");
    println!("║ • Changes worst case from O(n) to O(log n)              ║");
    println!("║ • Requires &mut self (modifies structure)                ║");
    println!("║                                                           ║");
    println!("║ Next: Step 3 - Union by Rank Optimization                ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
}

/// Basic Union-Find from Step 1 (no optimizations)
struct BasicUnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl BasicUnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            count: n,
        }
    }

    /// Naive find - just follows parent pointers
    fn find(&self, mut x: usize) -> usize {
        while x != self.parent[x] {
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        self.parent[root_x] = root_y;
        self.count -= 1;
        true
    }

    #[allow(dead_code)]
    fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

/// Path Compression Union-Find - First optimization!
struct PathCompressionUF {
    parent: Vec<usize>,
    count: usize,
}

impl PathCompressionUF {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            count: n,
        }
    }

    /// Find WITH path compression
    /// Key difference: Makes every node on the path point directly to root
    fn find(&mut self, x: usize) -> usize {
        // Two-pass approach:
        // Pass 1: Find the root
        let mut root = x;
        while root != self.parent[root] {
            root = self.parent[root];
        }

        // Pass 2: Make every node on path point to root (compression!)
        let mut current = x;
        while current != root {
            let next = self.parent[current];
            self.parent[current] = root;  // Direct connection to root!
            current = next;
        }

        root
    }

    /// Recursive version (cleaner but uses stack)
    #[allow(dead_code)]
    fn find_recursive(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            // Compress path by making parent point directly to root
            self.parent[x] = self.find_recursive(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        self.parent[root_x] = root_y;
        self.count -= 1;
        true
    }

    #[allow(dead_code)]
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    #[allow(dead_code)]
    fn count(&self) -> usize {
        self.count
    }
}

fn demonstrate_problem() {
    let mut uf = BasicUnionFind::new(10);

    println!("Creating a chain: 0 → 1 → 2 → 3 → 4 → 5");
    for i in 0..5 {
        uf.union(i, i + 1);
    }

    println!("\nTree structure (worst case chain):");
    println!("       5");
    println!("      ↗");
    println!("     4");
    println!("    ↗");
    println!("   3");
    println!("  ↗");
    println!(" 2");
    println!("↗");
    println!("1");
    println!("↗");
    println!("0");

    println!("\nTo find root of 0, we must traverse:");
    println!("  0 → 1 → 2 → 3 → 4 → 5  (5 steps!)");
    println!("  This is O(n) - very slow for long chains");
}

fn demonstrate_path_compression() {
    let mut uf = PathCompressionUF::new(10);

    println!("Creating the same chain: 0 → 1 → 2 → 3 → 4 → 5");
    for i in 0..5 {
        uf.union(i, i + 1);
    }

    println!("\nBEFORE first find(0), tree structure:");
    println!("       5");
    println!("      ↗");
    println!("     4");
    println!("    ↗");
    println!("   3");
    println!("  ↗");
    println!(" 2");
    println!("↗");
    println!("1");
    println!("↗");
    println!("0");

    println!("\n>>> Calling find(0) <<<");
    let root = uf.find(0);
    println!("Root of 0: {}", root);

    println!("\nAFTER find(0), tree structure (compressed!):");
    println!("       5");
    println!("     ↗ ↑ ↖ ↑ ↖");
    println!("   0   1   2   3   4");
    println!("\nAll nodes now point directly to root 5!");
    println!("Next find(0) will be O(1) instead of O(5)");
}

fn side_by_side_comparison() {
    // Create identical initial structures
    let mut basic_uf = BasicUnionFind::new(8);
    let mut compressed_uf = PathCompressionUF::new(8);

    // Build same chain in both
    println!("Building chain: 0 → 1 → 2 → 3 → 4 → 5 → 6 → 7");
    for i in 0..7 {
        basic_uf.union(i, i + 1);
        compressed_uf.union(i, i + 1);
    }

    println!("\n--- BASIC UNION-FIND ---");
    println!("Find root of 0:");
    let _root1 = basic_uf.find(0);
    println!("  Steps required: ~7 (follows chain)");
    
    println!("\nFind root of 0 AGAIN:");
    let _root2 = basic_uf.find(0);
    println!("  Steps required: ~7 (no improvement!)");

    println!("\n--- PATH COMPRESSION UNION-FIND ---");
    println!("Find root of 0:");
    let _root3 = compressed_uf.find(0);
    println!("  Steps required: ~7 (follows chain)");
    println!("  BUT: Tree is now compressed!");
    
    println!("\nFind root of 0 AGAIN:");
    let _root4 = compressed_uf.find(0);
    println!("  Steps required: 1 (direct to root!)");
    println!("  Improvement: 7x faster!");

    println!("\nFind ANY node on the compressed path:");
    let _r1 = compressed_uf.find(1);
    let _r2 = compressed_uf.find(2);
    let _r3 = compressed_uf.find(3);
    println!("  All find operations now O(1)!");
}

fn visualize_compression() {
    let mut uf = PathCompressionUF::new(10);

    println!("Step-by-Step Visualization:");
    println!("\n1. Initial state: Everyone points to themselves");
    for i in 0..6 {
        print!("{}→{} ", i, i);
    }
    println!();

    println!("\n2. After union(0,1), union(1,2), union(2,3), union(3,4), union(4,5):");
    println!("   0→1→2→3→4→5 (chain structure)");
    for i in 0..5 {
        uf.union(i, i + 1);
    }

    println!("\n3. Calling find(0) - Watch the compression happen!");
    println!("   Pass 1: Find root by following chain");
    println!("     0 → 1 → 2 → 3 → 4 → 5 ✓ (found root: 5)");
    
    println!("\n   Pass 2: Compress path (make everyone point to 5)");
    println!("     0 → 5 ✓");
    println!("     1 → 5 ✓");
    println!("     2 → 5 ✓");
    println!("     3 → 5 ✓");
    println!("     4 → 5 ✓");
    
    let _root = uf.find(0);

    println!("\n4. Result - Flat tree:");
    println!("          5");
    println!("        ↗ ↑ ↖ ↑ ↖");
    println!("      0   1   2   3   4");
}

fn performance_analysis() {
    println!("Time Complexity Comparison:");
    println!();
    println!("  Operation      │ Basic UF  │ Path Compression");
    println!("  ───────────────┼───────────┼─────────────────");
    println!("  new(n)         │   O(n)    │     O(n)");
    println!("  find (first)   │   O(n)    │     O(n) *");
    println!("  find (repeat)  │   O(n)    │     O(1) **");
    println!("  union          │   O(n)    │     O(log n)");
    println!("  connected      │   O(n)    │     O(log n)");
    println!();
    println!("  * First find may be O(n), but it compresses the path");
    println!("  ** Subsequent finds on same path are O(1)!");
    println!();
    println!("  Space Complexity: O(n) for both (just parent array)");
    println!();

    println!("Demonstration with large dataset:");
    let mut basic = BasicUnionFind::new(1000);
    let mut compressed = PathCompressionUF::new(1000);

    // Create worst-case chain
    for i in 0..999 {
        basic.union(i, i + 1);
        compressed.union(i, i + 1);
    }

    println!("\n  Created chain of 1000 elements");
    println!("\n  Basic UF - find(0):");
    let _r1 = basic.find(0);
    println!("    Would require ~999 steps");

    println!("\n  Path Compression UF - find(0):");
    let _r2 = compressed.find(0);
    println!("    First call: ~999 steps, BUT compresses path");
    println!("    Second call: 1 step! (999x improvement)");

    println!("\n  Amortized Complexity:");
    println!("    • Average case becomes O(log n) instead of O(n)");
    println!("    • Best case (compressed paths): O(1)");
    println!("    • Combined with union by rank: O(α(n)) ≈ O(1)");
}

// Helper function to demonstrate tree structure
#[allow(dead_code)]
fn print_parent_array(parent: &[usize], label: &str) {
    println!("{}", label);
    print!("  Index:  ");
    for i in 0..parent.len() {
        print!("{:2} ", i);
    }
    println!();
    print!("  Parent: ");
    for i in 0..parent.len() {
        print!("{:2} ", parent[i]);
    }
    println!("\n");
}

// Exercises for the reader
#[allow(dead_code)]
fn exercises() {
    println!("\n=== Exercises ===");
    println!("1. Implement path halving (simpler variant)");
    println!("2. Measure actual performance improvement with benchmarks");
    println!("3. Visualize tree structure before/after compression");
    println!("4. Compare iterative vs recursive path compression");
    println!("5. Test on worst-case and average-case scenarios");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_compression_correctness() {
        let mut uf = PathCompressionUF::new(10);
        
        // Build chain
        for i in 0..9 {
            uf.union(i, i + 1);
        }

        // All should be in same set
        for i in 0..9 {
            assert!(uf.connected(i, i + 1));
        }

        assert!(uf.connected(0, 9));
        assert_eq!(uf.count(), 1);
    }

    #[test]
    fn test_compression_occurs() {
        let mut uf = PathCompressionUF::new(5);
        
        // Create chain: 0 → 1 → 2 → 3 → 4
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(3, 4);

        // First find compresses path
        let root1 = uf.find(0);
        
        // Parent of 0 should now point directly to root
        // (not to intermediate node 1)
        let root2 = uf.parent[0];
        assert_eq!(root1, root2, "Path should be compressed after find");
    }

    #[test]
    fn test_basic_vs_compressed() {
        let mut basic = BasicUnionFind::new(10);
        let mut compressed = PathCompressionUF::new(10);

        // Same operations on both
        for i in 0..5 {
            basic.union(i, i + 1);
            compressed.union(i, i + 1);
        }

        // Both should give same results
        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(
                    basic.connected(i, j),
                    compressed.connected(i, j),
                    "Results should match"
                );
            }
        }
    }
}
