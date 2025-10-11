use competitive_linked_tree::{NodeId, TreeDiameter};

fn main() {
    println!("🔗 Competitive Programming: Tree Diameter with LinkedQueue");
    println!("===========================================================");
    println!();

    // Example 1: Line graph (simple case)
    demo_line_graph();
    println!();

    // Example 2: Star graph
    demo_star_graph();
    println!();

    // Example 3: Complex binary tree
    demo_binary_tree();
    println!();

    // Example 4: Large tree performance
    demo_large_tree();
    println!();

    // Example 5: Queue growth analysis
    demo_queue_analysis();
}

fn demo_line_graph() {
    println!("📍 Example 1: Line Graph");
    println!("-------------------------");

    // Create line graph: 0-1-2-3-4
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    let n = 5;

    println!("Tree structure: 0-1-2-3-4 (line graph)");
    println!("Nodes: {}", n);
    println!("Edges: {:?}", edges);
    println!();

    let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");

    match diameter_finder.find_diameter() {
        Some((diameter, path)) => {
            println!("✅ Diameter found: {}", diameter);
            println!("📍 Diameter path: {:?}", path);
            println!(
                "💡 Explanation: Longest path from {} to {}",
                path[0],
                path[path.len() - 1]
            );

            if let Some(centers) = diameter_finder.find_center() {
                println!("🎯 Tree center(s): {:?}", centers);
            }
        }
        None => {
            println!("❌ No diameter found (empty tree)");
        }
    }
}

fn demo_star_graph() {
    println!("📍 Example 2: Star Graph");
    println!("-------------------------");

    // Create star graph: center=0, leaves=1,2,3,4
    let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
    let n = 5;

    println!("Tree structure:");
    println!("    1   2");
    println!("     \\ /");
    println!("      0");
    println!("     / \\");
    println!("    3   4");
    println!("Nodes: {}", n);
    println!("Edges: {:?}", edges);
    println!();

    let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");

    match diameter_finder.find_diameter() {
        Some((diameter, path)) => {
            println!("✅ Diameter found: {}", diameter);
            println!("📍 Diameter path: {:?}", path);
            println!("💡 Explanation: Any leaf to any other leaf through center");

            if let Some(centers) = diameter_finder.find_center() {
                println!("🎯 Tree center(s): {:?}", centers);
            }
        }
        None => {
            println!("❌ No diameter found");
        }
    }
}

fn demo_binary_tree() {
    println!("📍 Example 3: Binary Tree");
    println!("--------------------------");

    // Create binary tree:
    //       4
    //      / \\
    //     1   5
    //    / \\   \\
    //   0   2   6
    //       \\
    //        3
    let edges = vec![
        (4, 1),
        (4, 5), // Root connections
        (1, 0),
        (1, 2), // Left subtree
        (5, 6), // Right subtree
        (2, 3), // Deep left path
    ];
    let n = 7;

    println!("Tree structure:");
    println!("       4");
    println!("      / \\");
    println!("     1   5");
    println!("    / \\   \\");
    println!("   0   2   6");
    println!("       \\");
    println!("        3");
    println!("Nodes: {}", n);
    println!("Edges: {:?}", edges);
    println!();

    let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");

    match diameter_finder.find_diameter() {
        Some((diameter, path)) => {
            println!("✅ Diameter found: {}", diameter);
            println!("📍 Diameter path: {:?}", path);
            println!("💡 Two-pass BFS algorithm:");
            println!("   1st BFS: From node 0 → found farthest node");
            println!("   2nd BFS: From that node → found diameter");

            visualize_path_analysis(&diameter_finder, &path);
        }
        None => {
            println!("❌ No diameter found");
        }
    }
}

fn demo_large_tree() {
    println!("📍 Example 4: Large Tree Performance");
    println!("-------------------------------------");

    // Create larger tree to demonstrate LinkedQueue scaling
    let n = 100;
    let mut edges = Vec::new();

    // Create a deep tree: 0-1-2-...-99
    for i in 0..n - 1 {
        edges.push((i, i + 1));
    }

    println!("Tree structure: Line graph with {} nodes", n);
    println!("Expected diameter: {}", n - 1);
    println!();

    let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");

    match diameter_finder.find_diameter() {
        Some((diameter, path)) => {
            println!("✅ Diameter found: {}", diameter);
            println!("📍 Path length: {}", path.len());
            println!("🎯 Path endpoints: {} → {}", path[0], path[path.len() - 1]);

            // Demonstrate LinkedQueue benefits for large trees
            println!();
            println!("💡 LinkedQueue Benefits:");
            println!("   • Dynamic growth: No pre-allocation needed");
            println!("   • Memory efficient: Allocates exactly what's needed");
            println!("   • Unbounded: Can handle trees of any size");
            println!("   • Two independent BFS runs with fresh queues");
        }
        None => {
            println!("❌ No diameter found");
        }
    }
}

fn demo_queue_analysis() {
    println!("📍 Example 5: Queue Growth Analysis");
    println!("------------------------------------");

    // Test different tree shapes to show queue growth patterns
    let test_cases = vec![
        ("Line (10 nodes)", create_line_tree(10)),
        ("Star (9 nodes)", create_star_tree(9)),
        ("Binary (15 nodes)", create_binary_tree(15)),
        ("Wide tree (13 nodes)", create_wide_tree(13)),
    ];

    for (name, (n, edges)) in test_cases {
        println!("🔍 Testing: {}", name);

        let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");

        if let Some((diameter, _path)) = diameter_finder.find_diameter() {
            println!("   Diameter: {}, Nodes in tree: {}", diameter, n);
            println!("   Max queue size during BFS ≤ {} nodes", n);
            println!("   LinkedQueue grows dynamically as needed");
        }
        println!();
    }

    println!("💡 Key Insights:");
    println!("  • LinkedQueue adapts to actual queue size needed");
    println!("  • Different tree shapes → different queue patterns");
    println!("  • No wasted memory from over-allocation");
    println!("  • Perfect for unknown or variable tree sizes");
}

fn visualize_path_analysis(diameter_finder: &TreeDiameter, path: &[NodeId]) {
    println!();
    println!("🔍 Path Analysis:");

    for (i, &node) in path.iter().enumerate() {
        if i == 0 {
            println!("   Start: Node {}", node);
        } else if i == path.len() - 1 {
            println!("   End:   Node {} (distance: {})", node, i);
        } else {
            println!("   Step {}: Node {}", i, node);
        }
    }

    if let Some(centers) = diameter_finder.find_center() {
        println!("🎯 Tree center(s): {:?}", centers);
        if centers.len() == 1 {
            println!("   (Center minimizes max distance to any node)");
        } else {
            println!("   (Two centers for even-diameter trees)");
        }
    }
}

// Helper functions to create different tree shapes
fn create_line_tree(n: usize) -> (usize, Vec<(NodeId, NodeId)>) {
    let mut edges = Vec::new();
    for i in 0..n - 1 {
        edges.push((i, i + 1));
    }
    (n, edges)
}

fn create_star_tree(n: usize) -> (usize, Vec<(NodeId, NodeId)>) {
    let mut edges = Vec::new();
    for i in 1..n {
        edges.push((0, i)); // All connect to center (node 0)
    }
    (n, edges)
}

fn create_binary_tree(n: usize) -> (usize, Vec<(NodeId, NodeId)>) {
    let mut edges = Vec::new();

    // Create complete binary tree structure
    for i in 0..n {
        let left_child = 2 * i + 1;
        let right_child = 2 * i + 2;

        if left_child < n {
            edges.push((i, left_child));
        }
        if right_child < n {
            edges.push((i, right_child));
        }
    }

    (n, edges)
}

fn create_wide_tree(n: usize) -> (usize, Vec<(NodeId, NodeId)>) {
    let mut edges = Vec::new();

    // Create tree with wide branching: 0 connects to 1,2,3,4
    // then each of those connects to 2-3 leaves
    edges.push((0, 1));
    edges.push((0, 2));
    edges.push((0, 3));

    let mut node_id = 4;
    for parent in 1..=3 {
        if node_id < n {
            edges.push((parent, node_id));
            node_id += 1;
        }
        if node_id < n {
            edges.push((parent, node_id));
            node_id += 1;
        }
        if node_id < n {
            edges.push((parent, node_id));
            node_id += 1;
        }
    }

    (n, edges)
}
