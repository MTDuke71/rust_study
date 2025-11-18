// Mission 10 Tutorial - Step 4: Combined Optimizations
// Bringing together path compression and union by rank for optimal performance

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Mission 10 Tutorial - Step 4: Combined Optimizations    ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("=== The Power of Synergy ===");
    println!("We've seen path compression (Step 2) and union by rank (Step 3).");
    println!("Now we combine them for the ultimate Union-Find performance:");
    println!("O(α(n)) - practically constant time!\n");

    // Part 1: Why Combine?
    println!("=== Part 1: Why Combine Both Optimizations? ===\n");
    explain_synergy();

    // Part 2: Full Implementation
    println!("\n=== Part 2: Optimized Union-Find Implementation ===\n");
    demonstrate_optimized_implementation();

    // Part 3: Progressive Comparison
    println!("\n=== Part 3: Progressive Performance Comparison ===\n");
    progressive_comparison();

    // Part 4: The Inverse Ackermann Function
    println!("\n=== Part 4: Understanding O(α(n)) Complexity ===\n");
    explain_inverse_ackermann();

    // Part 5: Real-World Performance
    println!("\n=== Part 5: Real-World Performance Analysis ===\n");
    real_world_performance();

    // Part 6: Mission 10 Connection
    println!("\n=== Part 6: Connection to Mission 10 ===\n");
    mission_connection();

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║              Key Takeaways                                ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ • Combined optimizations achieve O(α(n)) complexity       ║");
    println!("║ • α(n) ≤ 4 for all practical values of n                 ║");
    println!("║ • Path compression + union by rank work synergistically   ║");
    println!("║ • Performance is virtually constant in practice           ║");
    println!("║ • This is the industry-standard Union-Find               ║");
    println!("║                                                           ║");
    println!("║ Next: Step 5 - Real-World Applications                   ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
}

/// Version 1: Basic Union-Find (no optimizations)
#[allow(dead_code)]
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

    fn find(&self, x: usize) -> usize {
        let mut root = x;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        root
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
}

/// Version 2: Path Compression Only
#[allow(dead_code)]
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

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        self.parent[root_x] = root_y; // Naive attachment
        self.count -= 1;
        true
    }
}

/// Version 3: Union by Rank Only
#[allow(dead_code)]
struct UnionByRankUF {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionByRankUF {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    fn find(&self, x: usize) -> usize {
        let mut root = x;
        while root != self.parent[root] {
            root = self.parent[root]; // No path compression
        }
        root
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => self.parent[root_x] = root_y,
            std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
            std::cmp::Ordering::Equal => {
                self.parent[root_x] = root_y;
                self.rank[root_y] += 1;
            }
        }
        self.count -= 1;
        true
    }
}

/// Version 4: OPTIMIZED - Both Path Compression AND Union by Rank
/// This is the industry-standard implementation!
struct OptimizedUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
    operations: u64, // Track operations for analysis
}

impl OptimizedUnionFind {
    /// Create new Union-Find with n elements
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // Each element is its own parent initially
            rank: vec![0; n],         // All start with rank 0
            count: n,                 // n separate components initially
            operations: 0,
        }
    }

    /// Find root with PATH COMPRESSION
    /// Makes every node on path point directly to root
    fn find(&mut self, x: usize) -> usize {
        // First, find the root without path compression
        let mut root = x;
        while root != self.parent[root] {
            self.operations += 1; // Count each step to find root
            root = self.parent[root];
        }

        // Then, compress the path by making all nodes point to root
        let mut current = x;
        while current != root {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }

        root
    }

    /// Union with UNION BY RANK
    /// Always attach smaller rank tree to larger rank tree
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Union by rank: attach smaller tree to larger tree
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                // root_x smaller, attach to root_y
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                // root_y smaller, attach to root_x
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                // Equal rank, attach root_x to root_y and increase rank
                self.parent[root_x] = root_y;
                self.rank[root_y] += 1;
            }
        }

        self.count -= 1;
        true
    }

    /// Check if two elements are connected
    #[allow(dead_code)]
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Get number of connected components
    #[allow(dead_code)]
    fn count(&self) -> usize {
        self.count
    }

    /// Get statistics for analysis
    fn get_stats(&mut self) -> UnionFindStats {
        let mut components = std::collections::HashMap::new();
        let size = self.parent.len();

        // Group elements by their root
        for i in 0..size {
            let root = self.find(i);
            components.entry(root).or_insert_with(Vec::new).push(i);
        }

        let max_component_size = components.values().map(|v| v.len()).max().unwrap_or(0);
        let avg_component_size = if components.is_empty() {
            0.0
        } else {
            size as f64 / components.len() as f64
        };

        // Calculate average path length
        let mut total_path_length = 0;
        for i in 0..size {
            let mut current = i;
            let mut path_length = 0;
            while current != self.parent[current] {
                path_length += 1;
                current = self.parent[current];
            }
            total_path_length += path_length;
        }
        let avg_path_length = total_path_length as f64 / size as f64;

        UnionFindStats {
            total_elements: size,
            num_components: components.len(),
            max_component_size,
            avg_component_size,
            avg_path_length,
            total_operations: self.operations,
        }
    }

    /// Reset operation counter
    fn reset_operations(&mut self) {
        self.operations = 0;
    }
}

#[derive(Debug)]
struct UnionFindStats {
    total_elements: usize,
    num_components: usize,
    max_component_size: usize,
    avg_component_size: f64,
    avg_path_length: f64,
    total_operations: u64,
}

impl std::fmt::Display for UnionFindStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UnionFind Stats:\n\
             ├─ Elements: {}\n\
             ├─ Components: {}\n\
             ├─ Max component size: {}\n\
             ├─ Avg component size: {:.2}\n\
             ├─ Avg path length: {:.2}\n\
             └─ Total operations: {}",
            self.total_elements,
            self.num_components,
            self.max_component_size,
            self.avg_component_size,
            self.avg_path_length,
            self.total_operations
        )
    }
}

fn explain_synergy() {
    println!("Each optimization helps the other:");

    println!("\n🔄 Path Compression helps Union by Rank:");
    println!("  • Flattens trees, making rank bounds tighter");
    println!("  • Reduces tree height below theoretical maximum");
    println!("  • Makes subsequent operations faster");

    println!("\n🏗️  Union by Rank helps Path Compression:");
    println!("  • Keeps initial tree height low");
    println!("  • Reduces work for path compression");
    println!("  • Prevents pathological worst cases");

    println!("\n🚀 Together they achieve:");
    println!("  • O(α(n)) amortized complexity");
    println!("  • α(n) ≤ 4 for all practical n");
    println!("  • Virtually constant time operations");

    println!("\n💡 Think of it like this:");
    println!("  • Union by rank: Good architect (plans balanced structures)");
    println!("  • Path compression: Efficient maintenance (optimizes during use)");
    println!("  • Together: Perfect building that improves with age!");
}

fn demonstrate_optimized_implementation() {
    let mut uf = OptimizedUnionFind::new(10);

    println!("Creating the same problematic sequence from earlier steps:");
    println!("Elements: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9\n");

    let operations = [
        (0, 1),
        (2, 3),
        (4, 5),
        (6, 7),
        (8, 9), // Pairs
        (1, 3),
        (5, 7),
        (3, 7),
        (1, 9), // Connections
    ];

    for (i, &(x, y)) in operations.iter().enumerate() {
        println!("Step {}: union({}, {})", i + 1, x, y);

        let before_stats = uf.get_stats();
        uf.reset_operations();

        let result = uf.union(x, y);

        let after_stats = uf.get_stats();

        if result {
            println!("  ✓ Connected {} and {}", x, y);
            println!(
                "  📊 Components: {} → {}",
                before_stats.num_components, after_stats.num_components
            );
            println!(
                "  🎯 Path length: {:.2} → {:.2}",
                before_stats.avg_path_length, after_stats.avg_path_length
            );
        } else {
            println!("  ≡ Already connected");
        }
        println!();
    }

    println!("Final state:");
    let final_stats = uf.get_stats();
    println!("{}", final_stats);

    println!("\nNotice how average path length stays very low!");
    println!("This is the power of combined optimizations. 🚀");
}

fn progressive_comparison() {
    println!("=== All Four Versions Side by Side ===\n");

    // Create all four versions
    let mut basic = BasicUnionFind::new(16);
    let mut path_only = PathCompressionUF::new(16);
    let mut rank_only = UnionByRankUF::new(16);
    let mut optimized = OptimizedUnionFind::new(16);

    let operations = [
        (0, 1),
        (2, 3),
        (4, 5),
        (6, 7),
        (8, 9),
        (10, 11),
        (12, 13),
        (14, 15),
        (1, 3),
        (5, 7),
        (9, 11),
        (13, 15),
        (3, 7),
        (11, 15),
        (7, 15),
    ];

    println!(
        "Performing {} operations on 16 elements:\n",
        operations.len()
    );

    // Apply same operations to all versions
    for &(x, y) in &operations {
        basic.union(x, y);
        path_only.union(x, y);
        rank_only.union(x, y);
        optimized.union(x, y);
    }

    // Measure performance with multiple finds
    println!("Performance test: 1000 random find operations");

    let test_elements: Vec<usize> = (0..1000).map(|i| i % 16).collect();

    // Count steps for each version
    let mut basic_steps = 0;
    for &elem in &test_elements {
        let mut current = elem;
        while current != basic.parent[current] {
            basic_steps += 1;
            current = basic.parent[current];
        }
    }

    let mut path_steps = 0;
    for &elem in &test_elements {
        let mut current = elem;
        while current != path_only.parent[current] {
            path_steps += 1;
            current = path_only.parent[current];
        }
    }

    let mut rank_steps = 0;
    for &elem in &test_elements {
        let mut current = elem;
        while current != rank_only.parent[current] {
            rank_steps += 1;
            current = rank_only.parent[current];
        }
    }

    // Optimized version - count actual operations
    optimized.reset_operations();
    for &elem in &test_elements {
        optimized.find(elem);
    }

    println!("\nResults (steps for 1000 finds):");
    println!("┌─────────────────────┬───────────┬──────────────┐");
    println!("│ Version             │ Steps     │ Improvement  │");
    println!("├─────────────────────┼───────────┼──────────────┤");
    println!(
        "│ 1. Basic            │ {:>8}  │ baseline     │",
        basic_steps
    );
    println!(
        "│ 2. Path Compression │ {:>8}  │ ~{:.1}x better  │",
        path_steps,
        basic_steps as f64 / path_steps.max(1) as f64
    );
    println!(
        "│ 3. Union by Rank    │ {:>8}  │ ~{:.1}x better  │",
        rank_steps,
        basic_steps as f64 / rank_steps.max(1) as f64
    );
    println!(
        "│ 4. Both Combined    │ {:>8}  │ ~{:.1}x better  │",
        optimized.operations,
        basic_steps as f64 / optimized.operations.max(1) as f64
    );
    println!("└─────────────────────┴───────────┴──────────────┘\n");

    println!("🎯 The optimized version uses almost constant time!");
    println!(
        "   Each find() averages ~{:.1} steps",
        optimized.operations as f64 / test_elements.len() as f64
    );

    println!("\n💡 Why aren't the improvements more dramatic?");
    println!("   1. Small dataset (16 elements) - benefits show more with thousands");
    println!("   2. Path compression works progressively - improves with use");
    println!("   3. Union by rank prevents worst-case scenarios from happening");
    println!("   4. The first few finds build compressed paths for future finds");

    println!("\n=== Tree Structure Analysis ===");
    let final_stats = optimized.get_stats();
    println!("Average path length: {:.2}", final_stats.avg_path_length);
    println!(
        "This means find() touches ~{:.0} nodes on average",
        final_stats.avg_path_length + 1.0
    );
    println!("Compare to theoretical worst case: log₂(16) = 4 nodes");
    println!("Path compression keeps us well below even the improved bound!");
}

fn explain_inverse_ackermann() {
    println!("=== The Mysterious α(n) Function ===\n");

    println!("α(n) is the inverse Ackermann function:");
    println!("  • It grows EXTREMELY slowly");
    println!("  • For all practical purposes, α(n) ≤ 4");
    println!("  • It's the theoretical complexity of optimized Union-Find\n");

    println!("How slow does α(n) grow?");

    let values = [
        (10u64, 2),
        (100u64, 3),
        (1000u64, 3),
        (10000u64, 4),
        (100000u64, 4),
        (1000000u64, 4),
        (1000000000u64, 4),          // 10^9
        (1000000000000000000u64, 4), // 10^18
    ];

    println!("┌─────────────────┬─────────┐");
    println!("│ n               │ α(n)    │");
    println!("├─────────────────┼─────────┤");
    for (n, alpha_n) in values {
        if n < 1000000 {
            println!("│ {:>15} │ {:>7} │", format_number(n), alpha_n);
        } else if n < 10000000000u64 {
            // 10^10
            println!("│ {:>15} │ {:>7} │", format_large_number(n), alpha_n);
        } else {
            println!(
                "│ {:>15} │ {:>7} │",
                format!("10^{}", (n as f64).log10() as u32),
                alpha_n
            );
        }
    }

    // Add the theoretical astronomically large example
    println!("│ {:>15} │ {:>7} │", "10^100", 5);
    println!("└─────────────────┴─────────┘\n");

    println!("💡 What this means:");
    println!("  • For any realistic problem size, α(n) ≤ 4");
    println!("  • Your Union-Find operations are essentially O(1)");
    println!("  • The difference between α(n) and 1 is negligible in practice");

    println!("\n🤔 Why is it so slow?");
    println!("  The Ackermann function A(m,n) grows faster than any primitive recursive function.");
    println!("  Its inverse α(n) asks: 'What's the smallest m such that A(m,4) ≥ n?'");
    println!("  Since A grows so fast, its inverse grows incredibly slowly!");

    println!("\n🎯 Practical takeaway:");
    println!("  Don't worry about α(n) vs O(1) - they're the same in practice!");
    println!("  Focus on the algorithm correctness and implementation quality.");
}

fn format_number(n: u64) -> String {
    if n < 1000 {
        n.to_string()
    } else if n < 1000000 {
        format!("{},000", n / 1000)
    } else {
        format!("{},000,000", n / 1000000)
    }
}

fn format_large_number(n: u64) -> String {
    format!("10^{}", (n as f64).log10() as u32)
}

fn real_world_performance() {
    println!("=== Real-World Performance Examples ===\n");

    println!("Let's test on realistic problem sizes:\n");

    // Test different sizes
    let sizes = [100, 1000, 10000, 50000];

    for &size in &sizes {
        println!("📊 Testing with {} elements:", size);

        let mut uf = OptimizedUnionFind::new(size);

        // Create many random connections (about 50% connectivity)
        let num_operations = size / 2;
        uf.reset_operations();

        for i in 0..num_operations {
            let x = (i * 17) % size; // Pseudo-random pattern
            let y = (i * 23 + 7) % size;
            uf.union(x, y);
        }

        let after_unions = uf.operations;
        uf.reset_operations();

        // Test find performance
        for i in 0..1000.min(size) {
            uf.find(i);
        }

        let finds_ops = uf.operations;
        let stats = uf.get_stats();

        println!("  📈 Union operations: {}", after_unions);
        println!("  🔍 1000 finds used: {} operations", finds_ops);
        println!("  📏 Avg path length: {:.2}", stats.avg_path_length);
        println!("  🏗️  Components: {}", stats.num_components);
        println!("  ⚡ Ops per find: {:.2}", finds_ops as f64 / 1000.0);
        println!();
    }

    println!("🎯 Key observations:");
    println!("  • Path length stays very small even for large n");
    println!("  • Operations per find is nearly constant");
    println!("  • Performance scales beautifully with problem size");
    println!("  • This is why Union-Find is used in production systems!");
}

fn mission_connection() {
    println!("=== Connection to Mission 10 ===\n");

    println!("This tutorial teaches the same Union-Find used in Mission 10:");

    println!("\n📝 Mission 10 Requirements (from REQ-IDs):");
    println!("  • REQ-1: Generic Union-Find with path compression");
    println!("  • REQ-2: Union by rank optimization");
    println!("  • REQ-3: O(α(n)) amortized complexity");
    println!("  • REQ-4: Thread-safe operations");

    println!("\n🎓 Tutorial Learning Path:");
    println!("  • Step 1: Basic Union-Find (understand the problem)");
    println!("  • Step 2: Path compression (REQ-1)");
    println!("  • Step 3: Union by rank (REQ-2)");
    println!("  • Step 4: Combined optimizations (REQ-3) ← We are here!");
    println!("  • Step 5+: Real-world applications");

    println!("\n🔗 Mission 10 Implementation Highlights:");
    println!("  ```rust");
    println!("  pub struct UnionFind<T = usize> {{");
    println!("      parent: Vec<usize>,");
    println!("      rank: Vec<usize>,");
    println!("      elements: Vec<T>,");
    println!("  }}");
    println!("  ```");
    println!("  • Generic over element type T");
    println!("  • Includes both optimizations from this tutorial");
    println!("  • Production-ready with comprehensive testing");

    println!("\n🧪 Tutorial vs Mission Testing:");
    println!("  • Tutorial: Educational examples and explanations");
    println!("  • Mission: Formal V-Cycle testing with REQ traceability");
    println!("  • Both: Same underlying algorithm and complexity");

    println!("\n📚 Next Steps:");
    println!("  1. Complete remaining tutorial steps (5-7)");
    println!("  2. Study Mission 10 production implementation");
    println!("  3. Work through Mission 10 formal requirements");
    println!("  4. Apply Union-Find to real problems in your projects!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_union_find_basic_operations() {
        let mut uf = OptimizedUnionFind::new(5);

        // Test initial state
        assert_eq!(uf.count(), 5);
        assert!(!uf.connected(0, 1));

        // Test union
        assert!(uf.union(0, 1));
        assert!(uf.connected(0, 1));
        assert_eq!(uf.count(), 4);

        // Test duplicate union
        assert!(!uf.union(0, 1));
        assert_eq!(uf.count(), 4);

        // Test transitivity
        assert!(uf.union(1, 2));
        assert!(uf.connected(0, 2));
    }

    #[test]
    fn test_path_compression_effect() {
        let mut uf = OptimizedUnionFind::new(10);

        // Create a chain: 0-1-2-3-4
        for i in 0..4 {
            uf.union(i, i + 1);
        }

        // First find will compress the path
        uf.reset_operations();
        let root1 = uf.find(0);
        let ops1 = uf.operations;

        // Second find should be much faster due to compression
        uf.reset_operations();
        let root2 = uf.find(0);
        let ops2 = uf.operations;

        assert_eq!(root1, root2);
        assert!(
            ops2 <= ops1,
            "Second find should be faster: {} vs {}",
            ops2,
            ops1
        );
    }

    #[test]
    fn test_union_by_rank_maintains_balance() {
        let mut uf = OptimizedUnionFind::new(16);

        // Build perfectly balanced tree
        for i in 0..8 {
            uf.union(i * 2, i * 2 + 1);
        }
        for i in 0..4 {
            uf.union(i * 4 + 1, i * 4 + 3);
        }
        for i in 0..2 {
            uf.union(i * 8 + 3, i * 8 + 7);
        }
        uf.union(7, 15);

        // All elements should be connected
        for i in 1..16 {
            assert!(uf.connected(0, i));
        }

        // Tree should be well-balanced
        let stats = uf.get_stats();
        assert!(
            stats.avg_path_length < 4.0,
            "Average path length {} should be less than 4",
            stats.avg_path_length
        );
    }

    #[test]
    fn test_performance_consistency() {
        let mut uf = OptimizedUnionFind::new(1000);

        // Perform many operations
        for i in 0..500 {
            uf.union(i, (i + 1) % 1000);
        }

        // Test find performance consistency
        uf.reset_operations();
        for i in 0..100 {
            uf.find(i * 10);
        }

        let ops = uf.operations;
        let avg_ops_per_find = ops as f64 / 100.0;

        // Should be very efficient (practically constant)
        assert!(
            avg_ops_per_find < 10.0,
            "Average operations per find {} should be less than 10",
            avg_ops_per_find
        );
    }

    #[test]
    fn test_stats_accuracy() {
        let mut uf = OptimizedUnionFind::new(10);

        // Create two components: {0,1,2,3,4} and {5,6,7,8,9}
        for i in 0..4 {
            uf.union(i, i + 1);
        }
        for i in 5..9 {
            uf.union(i, i + 1);
        }

        let stats = uf.get_stats();

        assert_eq!(stats.total_elements, 10);
        assert_eq!(stats.num_components, 2);
        assert_eq!(stats.max_component_size, 5);
        assert_eq!(stats.avg_component_size, 5.0);
    }
}

// Exercises for Step 4:
//
// 1. **Complexity Verification**: Create a test that verifies O(α(n)) complexity by
//    measuring actual operation counts for different values of n. Plot the results.
//
// 2. **Optimization Analysis**: Implement a version that can disable either optimization
//    independently. Compare performance with various workloads.
//
// 3. **Memory Profiling**: Measure memory usage of the optimized Union-Find compared
//    to basic version. Is the extra memory for ranks worth it?
//
// 4. **Worst-Case Construction**: Try to construct inputs that make the optimized
//    version perform as poorly as possible. How bad can you make it?
//
// 5. **Alternative Path Compression**: Implement "path splitting" or "path halving"
//    variants of path compression. Compare their performance with full compression.
//
// 6. **Benchmarking Suite**: Create a comprehensive benchmark that tests Union-Find
//    on various graph types: random, grid, tree-like, complete, etc.
//
// 7. **Production Integration**: Implement a version suitable for production use with
//    error handling, comprehensive documentation, and extensive test coverage.
