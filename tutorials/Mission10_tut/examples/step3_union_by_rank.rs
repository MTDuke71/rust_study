// Mission 10 Tutorial - Step 3: Union by Rank Optimization
// Building on Step 1 to add the second major optimization

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Mission 10 Tutorial - Step 3: Union by Rank             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("=== What is Union by Rank? ===");
    println!("In previous steps, we saw that naive union creates tall trees.");
    println!("Union by rank always makes the smaller tree a subtree of the larger,");
    println!("keeping trees balanced and find() operations fast.\n");

    // Part 1: Review the Problem
    println!("=== Part 1: The Problem with Naive Union ===\n");
    demonstrate_problem();

    // Part 2: Union by Rank Solution
    println!("\n=== Part 2: Union by Rank Solution ===\n");
    demonstrate_union_by_rank();

    // Part 3: Rank Tracking Explained
    println!("\n=== Part 3: Understanding Rank ===\n");
    explain_rank_concept();

    // Part 4: Side-by-Side Comparison
    println!("\n=== Part 4: Side-by-Side Comparison ===\n");
    side_by_side_comparison();

    // Part 5: Performance Impact
    println!("\n=== Part 5: Performance Impact ===\n");
    performance_analysis();

    // Part 6: Interactive Demo
    println!("\n=== Part 6: Interactive Tree Building ===\n");
    interactive_demo();

    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║              Key Takeaways                                ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ • Union by rank keeps trees balanced                      ║");
    println!("║ • Always attach smaller tree to larger tree              ║");
    println!("║ • Rank represents upper bound on tree height             ║");
    println!("║ • Guarantees tree height ≤ log₂(n)                      ║");
    println!("║ • Changes worst case from O(n) to O(log n)              ║");
    println!("║                                                           ║");
    println!("║ Next: Step 4 - Combining Both Optimizations              ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
}

/// Basic Union-Find from Step 1 (no optimizations)
/// Used for comparison to show the problem
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

        // NAIVE: Always make root_x point to root_y
        // This can create very unbalanced trees!
        self.parent[root_x] = root_y;
        self.count -= 1;
        true
    }

    fn get_tree_height(&self, root: usize) -> usize {
        let mut max_height = 0;
        for i in 0..self.parent.len() {
            if self.find(i) == root {
                let mut height = 0;
                let mut current = i;
                while current != self.parent[current] {
                    height += 1;
                    current = self.parent[current];
                }
                max_height = max_height.max(height);
            }
        }
        max_height
    }
}

/// Union-Find WITH Union by Rank optimization
/// Key addition: Track rank (upper bound on tree height)
struct UnionByRankUF {
    parent: Vec<usize>,
    rank: Vec<usize>, // NEW: Rank of each tree (upper bound on height)
    count: usize,
}

impl UnionByRankUF {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n], // Initially all have rank 0 (single nodes)
            count: n,
        }
    }

    /// Find WITHOUT path compression (for pure union-by-rank demo)
    fn find(&self, x: usize) -> usize {
        let mut root = x;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        root
    }

    /// Union WITH rank optimization
    /// Always make smaller rank tree a subtree of larger rank tree
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already connected
        }

        // Compare ranks and attach smaller to larger
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                // root_x has smaller rank, make it child of root_y
                self.parent[root_x] = root_y;
                // root_y's rank doesn't change (it got taller but within bound)
            }
            std::cmp::Ordering::Greater => {
                // root_y has smaller rank, make it child of root_x
                self.parent[root_y] = root_x;
                // root_x's rank doesn't change
            }
            std::cmp::Ordering::Equal => {
                // Same rank, either choice works, but rank increases
                self.parent[root_x] = root_y;
                self.rank[root_y] += 1; // IMPORTANT: Rank increases only when equal!
            }
        }

        self.count -= 1;
        true
    }

    #[allow(dead_code)]
    fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    #[allow(dead_code)]
    fn count(&self) -> usize {
        self.count
    }

    fn get_rank(&self, x: usize) -> usize {
        let root = self.find(x);
        self.rank[root]
    }

    fn get_tree_height(&self, root: usize) -> usize {
        let mut max_height = 0;
        for i in 0..self.parent.len() {
            if self.find(i) == root {
                let mut height = 0;
                let mut current = i;
                while current != self.parent[current] {
                    height += 1;
                    current = self.parent[current];
                }
                max_height = max_height.max(height);
            }
        }
        max_height
    }

    fn print_tree_structure(&self, elements: &[usize]) {
        println!("Tree structure (parent pointers):");
        for &elem in elements {
            let parent = self.parent[elem];
            let rank = self.rank[elem];
            if elem == parent {
                println!("  {}: ROOT (rank {})", elem, rank);
            } else {
                println!("  {}: → {} (rank {})", elem, parent, rank);
            }
        }
    }

    fn print_rank_summary(&self) {
        println!("Rank summary:");
        for (i, &rank) in self.rank.iter().enumerate() {
            if i == self.parent[i] && rank > 0 {
                println!("  Root {}: rank {} (≤ {} nodes)", i, rank, 1 << rank);
            }
        }
    }
}

fn demonstrate_problem() {
    let mut uf = BasicUnionFind::new(8);

    println!("Let's create a worst-case scenario with naive union:");
    println!("Starting with elements: 0, 1, 2, 3, 4, 5, 6, 7\n");

    // Create worst case: always union smaller number to larger
    println!("Naive unions (always attach left to right):");
    uf.union(0, 1);
    println!("  union(0, 1): 0 → 1");
    uf.union(1, 2);
    println!("  union(1, 2): creates chain 0 → 1 → 2");
    uf.union(2, 3);
    println!("  union(2, 3): creates chain 0 → 1 → 2 → 3");
    uf.union(3, 4);
    println!("  union(3, 4): creates chain 0 → 1 → 2 → 3 → 4");

    println!("\nResulting tree structure:");
    println!("         4");
    println!("        ↗");
    println!("       3");
    println!("      ↗");
    println!("     2");
    println!("    ↗");
    println!("   1");
    println!("  ↗");
    println!(" 0");

    let root = uf.find(0);
    let height = uf.get_tree_height(root);
    println!("\nTree height: {} (very tall!)", height);
    println!("find(0) requires {} steps - this is O(n)!", height);
}

fn demonstrate_union_by_rank() {
    let mut uf = UnionByRankUF::new(8);

    println!("Same sequence with union by rank:");
    println!("Starting with elements: 0, 1, 2, 3, 4, 5, 6, 7");
    println!("All start with rank 0\n");

    println!("Union by rank operations:");

    println!("1. union(0, 1):");
    println!("   - Both have rank 0 (equal)");
    println!("   - Attach 0 → 1, increase rank[1] to 1");
    uf.union(0, 1);
    println!("   - Result: 0 → 1 (rank 1)");

    println!("\n2. union(1, 2):");
    println!("   - rank[1] = 1, rank[2] = 0");
    println!("   - 1 has higher rank, so 2 → 1");
    uf.union(1, 2);
    println!("   - Result: 0 → 1 ← 2 (rank[1] stays 1)");

    println!("\n3. union(3, 4):");
    println!("   - Both have rank 0 (equal)");
    println!("   - Attach 3 → 4, increase rank[4] to 1");
    uf.union(3, 4);
    println!("   - Result: 3 → 4 (rank 1)");

    println!("\n4. union(1, 4): (connecting two trees)");
    println!("   - rank[1] = 1, rank[4] = 1 (equal!)");
    println!("   - Attach 1 → 4, increase rank[4] to 2");
    uf.union(1, 4);
    println!("   - Result: Combined tree with root 4 (rank 2)");

    println!("\nFinal tree structure:");
    println!("      4 (rank 2)");
    println!("     ↙ ↘");
    println!("    1   3");
    println!("   ↙ ↘   ↘");
    println!("  0   2   (empty)");

    uf.print_tree_structure(&[0, 1, 2, 3, 4]);

    let root = uf.find(0);
    let height = uf.get_tree_height(root);
    println!("\nTree height: {} (much better!)", height);
    println!("Maximum possible height: ≤ log₂(5) ≈ 2.3");
}

fn explain_rank_concept() {
    println!("=== Understanding Rank ===\n");

    println!("Rank is NOT the actual height - it's an upper bound!");
    println!("Key properties:");
    println!("  • Rank starts at 0 for single nodes");
    println!("  • Rank increases only when unioning equal-rank trees");
    println!("  • Rank never decreases");
    println!("  • Tree height ≤ rank ≤ log₂(tree_size)");

    println!("\n=== Why Rank Works ===");

    let mut uf = UnionByRankUF::new(16);

    println!("Building trees to show rank progression:\n");

    // Build perfectly balanced binary tree
    println!("Step 1: Create pairs (rank 0 → 1)");
    for i in 0..8 {
        let j = i * 2;
        if j + 1 < 16 {
            uf.union(j, j + 1);
            println!("  union({}, {}): rank increases 0 → 1", j, j + 1);
        }
    }

    println!("\nStep 2: Combine pairs (rank 1 → 2)");
    uf.union(1, 3); // Tree roots with rank 1
    println!("  union(1, 3): both rank 1, result rank 2");

    uf.union(5, 7);
    println!("  union(5, 7): both rank 1, result rank 2");

    println!("\nStep 3: Combine larger trees (rank 2 → 3)");
    uf.union(3, 7); // Tree roots with rank 2
    println!("  union(3, 7): both rank 2, result rank 3");

    println!("\nRank progression shows perfect balance:");
    println!("  Rank 0: 1 node    (2⁰ = 1)");
    println!("  Rank 1: ≥ 2 nodes  (2¹ = 2)");
    println!("  Rank 2: ≥ 4 nodes  (2² = 4)");
    println!("  Rank 3: ≥ 8 nodes  (2³ = 8)");
    println!("  ...");
    println!("  Rank k: ≥ 2ᵏ nodes");

    uf.print_rank_summary();

    println!("\nThis guarantees: tree_height ≤ rank ≤ log₂(n)");
    println!("So find() is always O(log n) in worst case!");
}

fn side_by_side_comparison() {
    println!("=== Comparing Naive vs Union by Rank ===\n");

    let mut basic = BasicUnionFind::new(10);
    let mut ranked = UnionByRankUF::new(10);

    let operations = [
        (0, 1),
        (2, 3),
        (4, 5),
        (6, 7),
        (8, 9),
        (1, 3),
        (5, 7),
        (3, 7),
        (1, 9),
    ];

    println!("Performing same operations on both structures:\n");

    for (i, &(x, y)) in operations.iter().enumerate() {
        println!("Operation {}: union({}, {})", i + 1, x, y);

        basic.union(x, y);
        ranked.union(x, y);

        // Find a common root to analyze
        let basic_root = basic.find(0);
        let ranked_root = ranked.find(0);

        let basic_height = basic.get_tree_height(basic_root);
        let ranked_height = ranked.get_tree_height(ranked_root);
        let ranked_rank = ranked.get_rank(0);

        println!("  Naive:  tree height = {}", basic_height);
        println!(
            "  Ranked: tree height = {}, rank = {}",
            ranked_height, ranked_rank
        );

        if ranked_height < basic_height {
            println!(
                "  → Rank optimization wins! {} < {}",
                ranked_height, basic_height
            );
        }
        println!();
    }

    println!("=== Final Comparison ===");
    let basic_root = basic.find(0);
    let ranked_root = ranked.find(0);

    println!(
        "Naive union final height:  {}",
        basic.get_tree_height(basic_root)
    );
    println!(
        "Union by rank final height: {}",
        ranked.get_tree_height(ranked_root)
    );
    println!("Union by rank final rank:   {}", ranked.get_rank(0));
}

fn performance_analysis() {
    println!("=== Performance Analysis ===\n");

    println!("Time Complexity Comparison:");
    println!("┌────────────────────┬─────────────┬─────────────────┐");
    println!("│ Operation          │ Naive Union │ Union by Rank   │");
    println!("├────────────────────┼─────────────┼─────────────────┤");
    println!("│ find(x)            │ O(n)        │ O(log n)        │");
    println!("│ union(x, y)        │ O(n)        │ O(log n)        │");
    println!("│ connected(x, y)    │ O(n)        │ O(log n)        │");
    println!("└────────────────────┴─────────────┴─────────────────┘\n");

    println!("Space Complexity:");
    println!("  Naive:         O(n) - parent array");
    println!("  Union by Rank: O(n) - parent array + rank array");
    println!("  Extra space:   O(n) - one integer per element\n");

    println!("Why the improvement?");
    println!("  • Naive union can create chains of length n");
    println!("  • Union by rank guarantees tree height ≤ log₂(n)");
    println!("  • Each find() traversal is at most log₂(n) steps");

    // Demonstrate with larger example
    println!("\n=== Scaling Demonstration ===");

    for size in [16, 64, 256, 1024] {
        let naive_worst = size - 1; // Chain length
        let ranked_worst = (size as f32).log2().ceil() as usize;

        println!(
            "Size {}: Naive O({}) vs Ranked O({})",
            size, naive_worst, ranked_worst
        );
    }

    println!("\nFor n = 1,000,000:");
    println!("  Naive worst case:  999,999 steps");
    println!("  Ranked worst case: ~20 steps");
    println!("  That's 50,000x faster! 🚀");
}

fn interactive_demo() {
    println!("=== Interactive Tree Building ===\n");

    let mut uf = UnionByRankUF::new(8);

    println!("Let's build a tree step by step and see ranks:");
    println!("Initial state: 0, 1, 2, 3, 4, 5, 6, 7 (all rank 0)\n");

    let steps = [
        ("union(0, 1)", 0, 1, "Two rank-0 trees → one rank-1 tree"),
        ("union(2, 3)", 2, 3, "Two rank-0 trees → one rank-1 tree"),
        ("union(4, 5)", 4, 5, "Two rank-0 trees → one rank-1 tree"),
        ("union(6, 7)", 6, 7, "Two rank-0 trees → one rank-1 tree"),
        ("union(1, 3)", 1, 3, "Two rank-1 trees → one rank-2 tree"),
        ("union(5, 7)", 5, 7, "Two rank-1 trees → one rank-2 tree"),
        ("union(3, 7)", 3, 7, "Two rank-2 trees → one rank-3 tree"),
    ];

    for (description, x, y, explanation) in steps {
        println!("🔄 {}: {}", description, explanation);

        let root_x = uf.find(x);
        let root_y = uf.find(y);
        let rank_x = uf.rank[root_x];
        let rank_y = uf.rank[root_y];

        println!(
            "   Before: root({}) has rank {}, root({}) has rank {}",
            x, rank_x, y, rank_y
        );

        uf.union(x, y);

        let new_root = uf.find(x);
        let new_rank = uf.rank[new_root];

        println!("   After:  root {} has rank {}", new_root, new_rank);

        // Show tree structure for smaller examples
        if steps.len()
            - steps
                .iter()
                .position(|(d, _, _, _)| *d == description)
                .unwrap()
            > 4
        {
            uf.print_tree_structure(&[0, 1, 2, 3]);
        }
        println!();
    }

    println!("Final tree structure:");
    uf.print_tree_structure(&[0, 1, 2, 3, 4, 5, 6, 7]);
    uf.print_rank_summary();

    println!("\nNotice: Final tree has height 3, which equals its rank!");
    println!("This demonstrates that rank is a tight upper bound.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_by_rank_basic() {
        let mut uf = UnionByRankUF::new(5);

        assert!(uf.union(0, 1));
        assert!(uf.union(2, 3));
        assert!(uf.union(1, 3));

        assert!(uf.connected(0, 2));
        assert!(!uf.connected(0, 4));
    }

    #[test]
    fn test_rank_increases_correctly() {
        let mut uf = UnionByRankUF::new(4);

        // Equal rank union increases rank
        uf.union(0, 1); // Both rank 0 → result rank 1
        assert_eq!(uf.get_rank(0), 1);

        uf.union(2, 3); // Both rank 0 → result rank 1
        assert_eq!(uf.get_rank(2), 1);

        uf.union(1, 3); // Both rank 1 → result rank 2
        assert_eq!(uf.get_rank(0), 2);
    }

    #[test]
    fn test_height_bounded_by_rank() {
        let mut uf = UnionByRankUF::new(16);

        // Build balanced tree
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

        let root = uf.find(0);
        let height = uf.get_tree_height(root);
        let rank = uf.get_rank(0);

        assert!(
            height <= rank,
            "Height {} should be ≤ rank {}",
            height,
            rank
        );
        assert!(rank <= 4, "Rank {} should be ≤ log₂(16) = 4", rank);
    }
}

// Exercises for Step 3:
//
// 1. **Union by Size**: Implement a variant that tracks tree size instead of rank.
//    Compare its performance with union by rank.
//
// 2. **Worst Case Construction**: Try to construct the worst-case tree for union by rank.
//    What's the tallest tree you can make with 16 elements?
//
// 3. **Rank Analysis**: Prove that if a tree has rank k, it must have at least 2^k nodes.
//    (Hint: Use induction on the union operations)
//
// 4. **Mixed Strategies**: What happens if you use union by rank for some operations
//    and naive union for others? Implement and test this hybrid approach.
//
// 5. **Performance Measurement**: Create trees of size 1000, 10000, 100000 and measure
//    actual find() times for naive vs union-by-rank implementations.
