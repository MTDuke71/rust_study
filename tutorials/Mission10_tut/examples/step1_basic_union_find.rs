// Mission 10 Tutorial - Step 1: Basic Union-Find
// Understanding the fundamentals without optimizations

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║  Mission 10 Tutorial - Step 1: Basic Union-Find  ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    println!("=== What is Union-Find? ===");
    println!("Union-Find (Disjoint Set Union) maintains a collection of");
    println!("non-overlapping sets and supports two key operations:");
    println!("  1. FIND: Determine which set an element belongs to");
    println!("  2. UNION: Merge two sets into one\n");

    // Part 1: Basic Implementation
    println!("=== Part 1: Basic Implementation (No Optimizations) ===\n");
    basic_union_find_demo();

    // Part 2: Understanding the Tree Structure
    println!("\n=== Part 2: Tree Structure Visualization ===\n");
    tree_structure_demo();

    // Part 3: Applications Preview
    println!("\n=== Part 3: Why Union-Find is Useful ===\n");
    applications_preview();

    // Part 4: Performance Analysis
    println!("\n=== Part 4: Performance Without Optimizations ===\n");
    performance_analysis();

    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║              Key Takeaways                        ║");
    println!("╠═══════════════════════════════════════════════════╣");
    println!("║ • Union-Find tracks disjoint sets efficiently     ║");
    println!("║ • Each set is represented as a tree              ║");
    println!("║ • Find follows parent pointers to root          ║");
    println!("║ • Union connects two trees                       ║");
    println!("║ • Without optimizations: O(n) worst case        ║");
    println!("║                                                   ║");
    println!("║ Next: Step 2 - Path Compression Optimization     ║");
    println!("╚═══════════════════════════════════════════════════╝");
}

/// Basic Union-Find implementation without optimizations
struct BasicUnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl BasicUnionFind {
    /// Create a new Union-Find with n elements
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // Each element is its own parent initially
            count: n,
        }
    }

    /// Find the root of element x (naive implementation)
    fn find(&self, mut x: usize) -> usize {
        // Follow parent pointers until we reach a root (where parent[x] == x)
        while x != self.parent[x] {
            x = self.parent[x];
        }
        x
    }

    /// Union the sets containing x and y (naive implementation)
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Arbitrarily make root_x point to root_y
        self.parent[root_x] = root_y;
        self.count -= 1;
        true
    }

    /// Check if x and y are in the same set
    fn connected(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Get number of disjoint sets
    fn count(&self) -> usize {
        self.count
    }

    /// Visualize the parent array
    fn visualize(&self) {
        println!("  Parent array:");
        print!("    Index:  ");
        for i in 0..self.parent.len() {
            print!("{:3} ", i);
        }
        println!();
        print!("    Parent: ");
        for i in 0..self.parent.len() {
            print!("{:3} ", self.parent[i]);
        }
        println!();
    }
}

fn basic_union_find_demo() {
    let mut uf = BasicUnionFind::new(10);

    println!("Initial state: 10 elements, each in its own set");
    uf.visualize();
    println!("  Number of sets: {}", uf.count());

    println!("\nPerforming union(0, 1):");
    uf.union(0, 1);
    uf.visualize();
    println!("  Sets: {}", uf.count());
    println!("  Connected(0, 1)? {}", uf.connected(0, 1));

    println!("\nPerforming union(2, 3):");
    uf.union(2, 3);
    uf.visualize();
    println!("  Sets: {}", uf.count());

    println!("\nPerforming union(0, 2) - merges {{0,1}} and {{2,3}}:");
    uf.union(0, 2);
    uf.visualize();
    println!("  Sets: {}", uf.count());
    println!("  Connected(0, 3)? {}", uf.connected(0, 3));
    println!("  Connected(0, 4)? {}", uf.connected(0, 4));

    println!("\nPerforming union(4, 5) and union(5, 6):");
    uf.union(4, 5);
    uf.union(5, 6);
    uf.visualize();
    println!("  Sets: {}", uf.count());
    println!("  Connected(4, 6)? {}", uf.connected(4, 6));
}

fn tree_structure_demo() {
    println!("Understanding the Tree Representation:");
    println!("  • Each element points to its parent");
    println!("  • Root elements point to themselves");
    println!("  • Find operation follows parent pointers to root");
    println!("  • Union operation connects two roots\n");

    let mut uf = BasicUnionFind::new(8);

    println!("Example: Building a tree structure");
    println!("\nInitial: Each element is its own tree (forest of 8 trees)");
    println!("  0  1  2  3  4  5  6  7");
    println!("  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓");
    println!("  0  1  2  3  4  5  6  7  (each points to itself)");

    uf.union(0, 1);
    println!("\nAfter union(0, 1): Element 0 now points to 1");
    println!("     1           ");
    println!("    ↗          ");
    println!("   0            ");
    println!("  (0's parent is now 1)");

    uf.union(2, 3);
    uf.union(1, 2);
    println!("\nAfter union(2, 3) and union(1, 2):");
    println!("       3        ");
    println!("      ↗         ");
    println!("     2          ");
    println!("    ↗           ");
    println!("   1            ");
    println!("  ↗             ");
    println!(" 0              ");
    println!("  (Tree gets taller - this is a problem!)");

    println!("\nProblem: Without optimizations, trees can become chains");
    println!("  • Find operation becomes O(n) in worst case");
    println!("  • Union operation also O(n) due to find");
    println!("  • Solution: Path compression & union by rank (later steps)");
}

fn applications_preview() {
    println!("Common Use Cases:");
    println!("  1. Network Connectivity");
    println!("     - Are two computers connected in a network?");
    println!("     - Can data flow between nodes?");
    println!();
    println!("  2. Social Networks");
    println!("     - Friend circles / connected groups");
    println!("     - Mutual connections");
    println!();
    println!("  3. Image Processing");
    println!("     - Connected components in images");
    println!("     - Flood fill algorithms");
    println!();
    println!("  4. Graph Algorithms");
    println!("     - Cycle detection");
    println!("     - Kruskal's Minimum Spanning Tree");
    println!("     - Connected components");
    println!();

    // Simple network example
    println!("Example: Network Connectivity");
    let mut network = BasicUnionFind::new(6);

    println!("  6 computers: [0, 1, 2, 3, 4, 5]");
    println!("  Connections: 0-1, 1-2, 3-4");

    network.union(0, 1);
    network.union(1, 2);
    network.union(3, 4);

    println!(
        "\n  Can computer 0 reach computer 2? {}",
        network.connected(0, 2)
    );
    println!(
        "  Can computer 0 reach computer 3? {}",
        network.connected(0, 3)
    );
    println!("  Number of separate networks: {}", network.count());

    println!("\n  Adding connection: 2-3");
    network.union(2, 3);
    println!(
        "  Can computer 0 reach computer 3? {}",
        network.connected(0, 3)
    );
    println!("  Number of separate networks: {}", network.count());
}

fn performance_analysis() {
    println!("Time Complexity Analysis (Basic Implementation):");
    println!();
    println!("  Operation  │ Best Case │ Worst Case │ Average");
    println!("  ───────────┼───────────┼────────────┼──────────");
    println!("  new(n)     │   O(n)    │    O(n)    │   O(n)");
    println!("  find(x)    │   O(1)    │    O(n)    │   O(log n)");
    println!("  union(x,y) │   O(1)    │    O(n)    │   O(log n)");
    println!("  connected  │   O(1)    │    O(n)    │   O(log n)");
    println!();
    println!("  Worst Case: Chain of n elements");
    println!("    0 → 1 → 2 → 3 → ... → n-1");
    println!("    Finding n-1 requires n steps!");
    println!();
    println!("  Space Complexity: O(n)");
    println!("    - parent array: n elements");
    println!();

    // Demonstrate worst case
    let mut uf = BasicUnionFind::new(10);
    println!("Creating worst-case scenario (chain):");

    for i in 0..9 {
        uf.union(i, i + 1);
    }

    uf.visualize();
    println!("\n  To find the root of element 0:");
    println!(
        "    0 → {} → {} → {} → ... → 9",
        uf.parent[0], uf.parent[uf.parent[0]], uf.parent[uf.parent[uf.parent[0]]]
    );
    println!("    Requires 9 steps (O(n) where n=10)");
    println!();
    println!("  Solutions (coming in next steps):");
    println!("    • Path Compression: Flatten tree during find");
    println!("    • Union by Rank: Keep trees balanced");
    println!("    • Combined: O(α(n)) ≈ O(1) for practical n");
}

// Exercises for the reader
#[allow(dead_code)]
fn exercises() {
    println!("\n=== Exercises ===");
    println!("1. Modify BasicUnionFind to track the size of each set");
    println!("2. Implement a method to return all elements in a set");
    println!("3. Add a method to return all roots (representatives)");
    println!("4. Implement cycle detection using Union-Find");
    println!("5. Measure actual performance on large inputs");
}
