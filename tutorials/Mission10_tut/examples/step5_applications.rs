//! Step 5: Real-World Applications
//!
//! This tutorial demonstrates practical applications of Union-Find data structure
//! in solving real-world problems across different domains.
//!
//! **Learning Objectives**:
//! - Apply Union-Find to graph algorithms (Kruskal's MST)
//! - Solve connectivity problems (connected components)
//! - Implement cycle detection
//! - Model social networks
//! - Perform image segmentation
//!
//! **Prerequisites**: Complete Steps 1-4 (understand optimized Union-Find)
//!
//! Run with: `cargo run --example step5_applications`

use std::collections::HashMap;

/// Optimized Union-Find with path compression and union by rank
/// (Copied from Step 4 for self-contained example)
#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
    operations: u64,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
            operations: 0,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        self.operations += 1;
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already connected
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => self.parent[root_x] = root_y,
            std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        self.count -= 1;
        true
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn count(&self) -> usize {
        self.count
    }

    fn get_components(&mut self) -> Vec<Vec<usize>> {
        let mut components: HashMap<usize, Vec<usize>> = HashMap::new();
        
        for i in 0..self.parent.len() {
            let root = self.find(i);
            components.entry(root).or_insert_with(Vec::new).push(i);
        }
        
        components.into_values().collect()
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  Step 5: Real-World Applications of Union-Find          ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Application 1: Kruskal's Minimum Spanning Tree
    println!("═══════════════════════════════════════════════════════════");
    println!("Application 1: Kruskal's Minimum Spanning Tree Algorithm");
    println!("═══════════════════════════════════════════════════════════\n");
    demonstrate_kruskal_mst();

    println!("\n");

    // Application 2: Connected Components
    println!("═══════════════════════════════════════════════════════════");
    println!("Application 2: Finding Connected Components in Graphs");
    println!("═══════════════════════════════════════════════════════════\n");
    demonstrate_connected_components();

    println!("\n");

    // Application 3: Cycle Detection
    println!("═══════════════════════════════════════════════════════════");
    println!("Application 3: Cycle Detection in Undirected Graphs");
    println!("═══════════════════════════════════════════════════════════\n");
    demonstrate_cycle_detection();

    println!("\n");

    // Application 4: Social Network Friend Circles
    println!("═══════════════════════════════════════════════════════════");
    println!("Application 4: Social Network Friend Circles");
    println!("═══════════════════════════════════════════════════════════\n");
    demonstrate_social_network();

    println!("\n");

    // Application 5: Image Segmentation
    println!("═══════════════════════════════════════════════════════════");
    println!("Application 5: Image Segmentation by Color Similarity");
    println!("═══════════════════════════════════════════════════════════\n");
    demonstrate_image_segmentation();

    println!("\n");

    // Performance Comparison
    println!("═══════════════════════════════════════════════════════════");
    println!("Performance Comparison: Union-Find vs Alternatives");
    println!("═══════════════════════════════════════════════════════════\n");
    performance_comparison();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  Exercises                                               ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    print_exercises();
}

// ============================================================================
// Application 1: Kruskal's Minimum Spanning Tree
// ============================================================================

/// Edge in a weighted graph
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    u: usize,
    v: usize,
    weight: i32,
}

impl Edge {
    fn new(u: usize, v: usize, weight: i32) -> Self {
        Edge { u, v, weight }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn demonstrate_kruskal_mst() {
    println!("Problem: Find the Minimum Spanning Tree (MST) of a weighted graph");
    println!("A spanning tree connects all vertices with minimum total edge weight.\n");

    println!("Why Union-Find?");
    println!("  • Efficiently detects cycles when adding edges");
    println!("  • O(E log E) overall complexity with sorting");
    println!("  • Perfect for Kruskal's greedy algorithm\n");

    // Example graph (vertices 0-5)
    println!("Example Graph (6 vertices):");
    println!("    0 ----4---- 1");
    println!("    |\\          |");
    println!("    | \\         |");
    println!("   2|  3\\       |1");
    println!("    |    \\      |");
    println!("    |     \\     |");
    println!("    2      3----5");
    println!("    |     /      |");
    println!("    |    /       |");
    println!("   3|  1/        |2");
    println!("    |  /         |");
    println!("    | /          |");
    println!("    4 -----------5\n");

    let mut edges = vec![
        Edge::new(0, 1, 4),
        Edge::new(0, 2, 2),
        Edge::new(0, 3, 3),
        Edge::new(1, 5, 1),
        Edge::new(2, 4, 3),
        Edge::new(3, 4, 1),
        Edge::new(3, 5, 3),
        Edge::new(4, 5, 2),
    ];

    println!("Edges (unsorted):");
    for edge in &edges {
        println!("  ({}, {}) weight: {}", edge.u, edge.v, edge.weight);
    }

    // Kruskal's Algorithm
    println!("\n--- Running Kruskal's Algorithm ---\n");

    // Step 1: Sort edges by weight
    edges.sort();
    println!("Step 1: Sort edges by weight");
    for edge in &edges {
        println!("  ({}, {}) weight: {}", edge.u, edge.v, edge.weight);
    }

    // Step 2: Initialize Union-Find
    let num_vertices = 6;
    let mut uf = UnionFind::new(num_vertices);
    let mut mst_edges = Vec::new();
    let mut total_weight = 0;

    println!("\nStep 2: Process edges in order, skip if creates cycle\n");

    // Step 3: Add edges if they don't create cycles
    for edge in &edges {
        if !uf.connected(edge.u, edge.v) {
            uf.union(edge.u, edge.v);
            mst_edges.push(*edge);
            total_weight += edge.weight;
            println!("  ✓ Add edge ({}, {}) weight: {} [Components: {}]",
                     edge.u, edge.v, edge.weight, uf.count());
        } else {
            println!("  ✗ Skip edge ({}, {}) weight: {} [Would create cycle]",
                     edge.u, edge.v, edge.weight);
        }
    }

    println!("\n--- MST Result ---");
    println!("MST Edges:");
    for edge in &mst_edges {
        println!("  ({}, {}) weight: {}", edge.u, edge.v, edge.weight);
    }
    println!("\nTotal MST Weight: {}", total_weight);
    println!("MST Edges Count: {} (should be V-1 = {})", mst_edges.len(), num_vertices - 1);

    println!("\nVisual MST:");
    println!("    0           1");
    println!("    |\\          |");
    println!("    | \\         |");
    println!("   2|  3\\       |1");
    println!("    |    \\      |");
    println!("    |     \\     |");
    println!("    2      3    5");
    println!("    |     /       ");
    println!("    |    /        ");
    println!("   3|  1/         ");
    println!("    |  /          ");
    println!("    | /           ");
    println!("    4             \n");

    println!("Complexity Analysis:");
    println!("  • Sorting edges: O(E log E)");
    println!("  • Union-Find operations: O(E α(V))");
    println!("  • Overall: O(E log E) dominated by sorting");
}

// ============================================================================
// Application 2: Connected Components
// ============================================================================

fn demonstrate_connected_components() {
    println!("Problem: Find all connected components in an undirected graph");
    println!("A connected component is a maximal set of vertices where every");
    println!("pair is connected by a path.\n");

    println!("Why Union-Find?");
    println!("  • Natural representation: each component is a set");
    println!("  • Dynamic: can add edges and update components efficiently");
    println!("  • O(E α(V)) for processing all edges\n");

    // Example graph
    println!("Example Graph (10 vertices, 8 edges):");
    println!("    0---1   2---3---4");
    println!("        |           ");
    println!("        5   6---7   ");
    println!("                    ");
    println!("        8---9       \n");

    let num_vertices = 10;
    let edges = vec![
        (0, 1), (1, 5),  // Component 1: {0, 1, 5}
        (2, 3), (3, 4),  // Component 2: {2, 3, 4}
        (6, 7),          // Component 3: {6, 7}
        (8, 9),          // Component 4: {8, 9}
    ];

    println!("Edges:");
    for (u, v) in &edges {
        println!("  {} -- {}", u, v);
    }

    let mut uf = UnionFind::new(num_vertices);

    println!("\nProcessing edges:\n");
    for (u, v) in &edges {
        uf.union(*u, *v);
        println!("  Union({}, {}) → {} components remain", u, v, uf.count());
    }

    // Get all components
    let components = uf.get_components();

    println!("\n--- Connected Components ---");
    println!("Total components: {}\n", components.len());

    for (i, component) in components.iter().enumerate() {
        println!("Component {}: {:?} (size: {})", i + 1, component, component.len());
    }

    println!("\nQueries:");
    let queries = vec![(0, 5), (2, 4), (0, 2), (6, 7), (8, 0)];
    for (u, v) in queries {
        let connected = uf.connected(u, v);
        println!("  Are {} and {} connected? {}", u, v, if connected { "✓ Yes" } else { "✗ No" });
    }

    println!("\nComparison with DFS/BFS:");
    println!("  Union-Find:");
    println!("    ✓ Dynamic (efficient edge additions)");
    println!("    ✓ O(α(V)) per query");
    println!("    ✓ Simple implementation");
    println!("    ✗ Only connectivity, not paths");
    println!("\n  DFS/BFS:");
    println!("    ✓ Can find actual paths");
    println!("    ✓ Can compute other properties");
    println!("    ✗ O(V + E) per query (unless preprocessed)");
    println!("    ✗ Not efficient for dynamic updates");
}

// ============================================================================
// Application 3: Cycle Detection
// ============================================================================

fn demonstrate_cycle_detection() {
    println!("Problem: Detect if adding an edge creates a cycle in undirected graph");
    println!("Used in: graph validation, network design, dependency checking\n");

    println!("Why Union-Find?");
    println!("  • If two vertices are already connected, adding edge creates cycle");
    println!("  • O(α(V)) per edge check");
    println!("  • Simple and efficient\n");

    println!("Example: Building a network incrementally");
    println!("Vertices: {{0, 1, 2, 3, 4}}\n");

    let num_vertices = 5;
    let edges = vec![
        (0, 1, "Connect nodes 0-1"),
        (1, 2, "Connect nodes 1-2"),
        (2, 3, "Connect nodes 2-3"),
        (3, 4, "Connect nodes 3-4"),
        (1, 3, "Shortcut: Connect nodes 1-3"),
        (0, 4, "Try to connect nodes 0-4"),
    ];

    let mut uf = UnionFind::new(num_vertices);

    println!("Adding edges one by one:\n");

    for (i, (u, v, description)) in edges.iter().enumerate() {
        println!("Step {}: {} (edge {}-{})", i + 1, description, u, v);

        if uf.connected(*u, *v) {
            println!("  ⚠️  CYCLE DETECTED! Vertices {} and {} already connected", u, v);
            println!("  ✗ Rejecting edge to maintain acyclic structure\n");
        } else {
            uf.union(*u, *v);
            println!("  ✓ Edge added successfully");
            println!("  Current tree structure:");
            print_tree_structure(*u, *v);
            println!();
        }
    }

    println!("--- Final Graph State ---");
    println!("Components: {}", uf.count());
    if uf.count() == 1 {
        println!("Graph is connected (single component)");
    } else {
        println!("Graph has {} separate components", uf.count());
    }

    println!("\nUse Cases:");
    println!("  • Network topology validation");
    println!("  • Dependency graph checking (build systems)");
    println!("  • Circuit design (avoiding short circuits)");
    println!("  • Game development (path validation)");
}

fn print_tree_structure(u: usize, v: usize) {
    println!("      {} --- {}", u, v);
}

// ============================================================================
// Application 4: Social Network Friend Circles
// ============================================================================

#[derive(Debug)]
struct SocialNetwork {
    uf: UnionFind,
    names: Vec<String>,
}

impl SocialNetwork {
    fn new(people: Vec<String>) -> Self {
        let n = people.len();
        SocialNetwork {
            uf: UnionFind::new(n),
            names: people,
        }
    }

    fn add_friendship(&mut self, person1: usize, person2: usize) {
        if self.uf.union(person1, person2) {
            println!("  ✓ {} and {} are now friends!",
                     self.names[person1], self.names[person2]);
        } else {
            println!("  → {} and {} are already in the same friend circle",
                     self.names[person1], self.names[person2]);
        }
    }

    fn are_friends(&mut self, person1: usize, person2: usize) -> bool {
        self.uf.connected(person1, person2)
    }

    fn get_friend_circles(&mut self) -> Vec<Vec<String>> {
        let components = self.uf.get_components();
        components.iter()
            .map(|comp| comp.iter().map(|&i| self.names[i].clone()).collect())
            .collect()
    }

    fn count_circles(&self) -> usize {
        self.uf.count()
    }
}

fn demonstrate_social_network() {
    println!("Problem: Identify friend circles in a social network");
    println!("Friend circles are groups where members are directly or indirectly connected\n");

    println!("Why Union-Find?");
    println!("  • Friends of friends form transitive groups");
    println!("  • Efficiently track mutually connected groups");
    println!("  • Dynamic updates as friendships form\n");

    let people = vec![
        "Alice", "Bob", "Carol", "Dave", "Eve", "Frank", "Grace", "Henry"
    ].iter().map(|s| s.to_string()).collect();

    let mut network = SocialNetwork::new(people);

    println!("Initial: {} people, {} circles (everyone isolated)\n",
             network.names.len(), network.count_circles());

    println!("Adding friendships:\n");

    // Form friend circles
    network.add_friendship(0, 1); // Alice - Bob
    network.add_friendship(1, 2); // Bob - Carol
    network.add_friendship(3, 4); // Dave - Eve
    network.add_friendship(5, 6); // Frank - Grace
    network.add_friendship(0, 2); // Alice - Carol (already in same circle)

    println!("\nCurrent friend circles: {}\n", network.count_circles());

    // More connections
    println!("Adding more connections:\n");
    network.add_friendship(1, 3); // Bob - Dave (merges two circles)
    network.add_friendship(6, 7); // Grace - Henry

    println!("\n--- Friend Circle Analysis ---\n");
    let circles = network.get_friend_circles();

    for (i, circle) in circles.iter().enumerate() {
        println!("Circle {}: {} (size: {})", i + 1, circle.join(", "), circle.len());
    }

    println!("\nQueries:\n");
    let queries = vec![
        (0, 2, "Alice", "Carol"),
        (0, 4, "Alice", "Eve"),
        (5, 7, "Frank", "Henry"),
        (0, 7, "Alice", "Henry"),
    ];

    for (p1, p2, name1, name2) in queries {
        let connected = network.are_friends(p1, p2);
        println!("  Are {} and {} in same friend circle? {}",
                 name1, name2, if connected { "✓ Yes" } else { "✗ No" });
    }

    println!("\nPractical Applications:");
    println!("  • Social media (suggest mutual friends)");
    println!("  • Community detection");
    println!("  • Group recommendation");
    println!("  • Network clustering");
}

// ============================================================================
// Application 5: Image Segmentation
// ============================================================================

#[derive(Debug, Clone)]
struct Image {
    pixels: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(data: Vec<Vec<char>>) -> Self {
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };
        Image {
            pixels: data,
            width,
            height,
        }
    }

    fn get_pixel(&self, row: usize, col: usize) -> char {
        self.pixels[row][col]
    }

    fn pixel_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    fn display(&self) {
        for row in &self.pixels {
            for &pixel in row {
                print!("{} ", pixel);
            }
            println!();
        }
    }

    fn display_segmented(&mut self, uf: &mut UnionFind) {
        let components = uf.get_components();
        let mut region_map = HashMap::new();

        // Assign region numbers
        for (region_id, component) in components.iter().enumerate() {
            for &pixel_idx in component {
                region_map.insert(pixel_idx, region_id);
            }
        }

        println!("\nSegmented Image (region numbers):");
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.pixel_index(row, col);
                let region = region_map.get(&idx).unwrap_or(&0);
                print!("{} ", region);
            }
            println!();
        }
    }
}

fn demonstrate_image_segmentation() {
    println!("Problem: Segment an image into regions of similar colors");
    println!("Group adjacent pixels with the same color into segments\n");

    println!("Why Union-Find?");
    println!("  • Efficiently merge adjacent similar regions");
    println!("  • O(pixels × α(pixels)) complexity");
    println!("  • Simple and effective for region growing\n");

    // Create sample image (8x8 ASCII art)
    let image_data = vec![
        vec!['A', 'A', 'A', 'B', 'B', 'C', 'C', 'C'],
        vec!['A', 'A', 'A', 'B', 'B', 'C', 'C', 'C'],
        vec!['A', 'A', 'B', 'B', 'B', 'B', 'C', 'C'],
        vec!['D', 'D', 'B', 'B', 'B', 'B', 'C', 'C'],
        vec!['D', 'D', 'D', 'B', 'B', 'C', 'C', 'C'],
        vec!['D', 'D', 'D', 'E', 'E', 'E', 'C', 'C'],
        vec!['D', 'D', 'E', 'E', 'E', 'E', 'E', 'C'],
        vec!['D', 'D', 'E', 'E', 'E', 'E', 'E', 'E'],
    ];

    let mut image = Image::new(image_data);

    println!("Original Image ({} x {}):\n", image.height, image.width);
    image.display();

    // Initialize Union-Find for all pixels
    let total_pixels = image.width * image.height;
    let mut uf = UnionFind::new(total_pixels);

    println!("\nSegmentation Process: Union adjacent pixels with same color\n");

    let mut unions = 0;
    // Process each pixel and union with similar neighbors
    for row in 0..image.height {
        for col in 0..image.width {
            let current_pixel = image.get_pixel(row, col);
            let current_idx = image.pixel_index(row, col);

            // Check right neighbor
            if col + 1 < image.width {
                let right_pixel = image.get_pixel(row, col + 1);
                if current_pixel == right_pixel {
                    let right_idx = image.pixel_index(row, col + 1);
                    if uf.union(current_idx, right_idx) {
                        unions += 1;
                    }
                }
            }

            // Check bottom neighbor
            if row + 1 < image.height {
                let bottom_pixel = image.get_pixel(row + 1, col);
                if current_pixel == bottom_pixel {
                    let bottom_idx = image.pixel_index(row + 1, col);
                    if uf.union(current_idx, bottom_idx) {
                        unions += 1;
                    }
                }
            }
        }
    }

    println!("Performed {} union operations", unions);
    println!("Found {} distinct regions\n", uf.count());

    image.display_segmented(&mut uf);

    // Analyze regions
    let components = uf.get_components();
    println!("\n--- Region Analysis ---\n");

    for (i, component) in components.iter().enumerate() {
        let (row, col) = image.index_to_coords(component[0]);
        let color = image.get_pixel(row, col);
        println!("Region {}: Color '{}', Size {} pixels",
                 i, color, component.len());
    }

    println!("\nApplications:");
    println!("  • Medical imaging (tumor detection)");
    println!("  • Computer vision (object recognition)");
    println!("  • Photo editing (magic wand tool)");
    println!("  • Geographic information systems");
}

// ============================================================================
// Performance Comparison
// ============================================================================

fn performance_comparison() {
    println!("Comparing Union-Find with alternative approaches\n");

    println!("┌─────────────────────┬──────────────┬──────────────┬──────────────┐");
    println!("│ Operation           │ Union-Find   │ DFS/BFS      │ Adjacency    │");
    println!("├─────────────────────┼──────────────┼──────────────┼──────────────┤");
    println!("│ Initialization      │ O(V)         │ O(V)         │ O(V²)        │");
    println!("│ Add Edge            │ O(α(V))      │ O(1)         │ O(1)         │");
    println!("│ Check Connected     │ O(α(V))      │ O(V+E)       │ O(1)         │");
    println!("│ Count Components    │ O(1)         │ O(V+E)       │ O(V²)        │");
    println!("│ Find All Components │ O(V)         │ O(V+E)       │ O(V²)        │");
    println!("│ Space               │ O(V)         │ O(V+E)       │ O(V²)        │");
    println!("└─────────────────────┴──────────────┴──────────────┴──────────────┘\n");

    println!("When to Use Union-Find:");
    println!("  ✓ Dynamic connectivity queries");
    println!("  ✓ MST algorithms (Kruskal's)");
    println!("  ✓ Cycle detection in undirected graphs");
    println!("  ✓ Grouping/clustering problems");
    println!("  ✓ When you don't need actual paths\n");

    println!("When NOT to Use Union-Find:");
    println!("  ✗ Need actual paths between nodes → Use BFS/DFS");
    println!("  ✗ Directed graphs → Use different approach");
    println!("  ✗ Need to remove edges → Standard UF doesn't support");
    println!("  ✗ Weighted shortest paths → Use Dijkstra/Bellman-Ford\n");

    println!("Real-World Performance:");
    println!("  For V = 1,000,000 elements:");
    println!("    • Union-Find: ~5-10 million operations/second");
    println!("    • Practically constant time per operation");
    println!("    • α(1,000,000) ≈ 4 (inverse Ackermann function)");
}

// ============================================================================
// Exercises
// ============================================================================

fn print_exercises() {
    println!("Exercise 1: Percolation Simulation");
    println!("  Implement percolation system using Union-Find.");
    println!("  Given N×N grid, randomly open sites. System percolates");
    println!("  when top row connects to bottom row.");
    println!("  Hint: Use virtual top and bottom nodes\n");

    println!("Exercise 2: Dynamic Connectivity with Deletions");
    println!("  Extend Union-Find to support edge deletion.");
    println!("  One approach: rebuild from scratch (but track history)");
    println!("  Challenge: Can you do better?\n");

    println!("Exercise 3: Least Common Ancestor (LCA)");
    println!("  Use Union-Find for offline LCA queries in a tree.");
    println!("  Process nodes in post-order, use union to build ancestors.");
    println!("  Hint: Tarjan's offline LCA algorithm\n");

    println!("Exercise 4: Maze Generation");
    println!("  Generate random maze using Union-Find.");
    println!("  Start with all walls, randomly remove walls that connect");
    println!("  different components until maze is connected.\n");

    println!("Exercise 5: Number of Islands (LeetCode 200)");
    println!("  Given 2D grid of '1's (land) and '0's (water),");
    println!("  count number of islands. Island = connected '1's.");
    println!("  Implement using Union-Find.\n");

    println!("Exercise 6: Accounts Merge (LeetCode 721)");
    println!("  Merge accounts with common email addresses.");
    println!("  Use Union-Find to group accounts.\n");

    println!("For solutions and hints, see exercises/step5_exercises.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal_mst() {
        let mut edges = vec![
            Edge::new(0, 1, 4),
            Edge::new(0, 2, 2),
            Edge::new(0, 3, 3),
            Edge::new(1, 5, 1),
            Edge::new(2, 4, 3),
            Edge::new(3, 4, 1),
            Edge::new(3, 5, 3),
            Edge::new(4, 5, 2),
        ];

        edges.sort();
        let mut uf = UnionFind::new(6);
        let mut mst_edges = Vec::new();
        let mut total_weight = 0;

        for edge in &edges {
            if !uf.connected(edge.u, edge.v) {
                uf.union(edge.u, edge.v);
                mst_edges.push(*edge);
                total_weight += edge.weight;
            }
        }

        assert_eq!(mst_edges.len(), 5); // V - 1 edges
        assert_eq!(total_weight, 9); // Minimum total weight
        assert_eq!(uf.count(), 1); // All connected
    }

    #[test]
    fn test_connected_components() {
        let mut uf = UnionFind::new(10);
        let edges = vec![
            (0, 1), (1, 5),
            (2, 3), (3, 4),
            (6, 7),
            (8, 9),
        ];

        for (u, v) in edges {
            uf.union(u, v);
        }

        assert_eq!(uf.count(), 4); // 4 components

        assert!(uf.connected(0, 5));
        assert!(uf.connected(2, 4));
        assert!(!uf.connected(0, 2));
    }

    #[test]
    fn test_cycle_detection() {
        let mut uf = UnionFind::new(4);

        // Add edges forming a tree
        assert!(!uf.connected(0, 1));
        uf.union(0, 1);

        assert!(!uf.connected(1, 2));
        uf.union(1, 2);

        assert!(!uf.connected(2, 3));
        uf.union(2, 3);

        // This edge would create a cycle
        assert!(uf.connected(0, 3));
    }

    #[test]
    fn test_social_network() {
        let people: Vec<String> = vec!["A", "B", "C", "D"]
            .iter().map(|s| s.to_string()).collect();

        let mut network = SocialNetwork::new(people);

        assert_eq!(network.count_circles(), 4);

        network.add_friendship(0, 1);
        assert_eq!(network.count_circles(), 3);

        network.add_friendship(2, 3);
        assert_eq!(network.count_circles(), 2);

        assert!(network.are_friends(0, 1));
        assert!(!network.are_friends(0, 2));
    }

    #[test]
    fn test_image_segmentation() {
        let image_data = vec![
            vec!['A', 'A', 'B'],
            vec!['A', 'B', 'B'],
            vec!['C', 'C', 'C'],
        ];

        let image = Image::new(image_data);
        let mut uf = UnionFind::new(9);

        // Segment the image
        for row in 0..image.height {
            for col in 0..image.width {
                let current_pixel = image.get_pixel(row, col);
                let current_idx = image.pixel_index(row, col);

                if col + 1 < image.width {
                    let right_pixel = image.get_pixel(row, col + 1);
                    if current_pixel == right_pixel {
                        let right_idx = image.pixel_index(row, col + 1);
                        uf.union(current_idx, right_idx);
                    }
                }

                if row + 1 < image.height {
                    let bottom_pixel = image.get_pixel(row + 1, col);
                    if current_pixel == bottom_pixel {
                        let bottom_idx = image.pixel_index(row + 1, col);
                        uf.union(current_idx, bottom_idx);
                    }
                }
            }
        }

        assert_eq!(uf.count(), 3); // 3 distinct regions (A, B, C)
    }
}
