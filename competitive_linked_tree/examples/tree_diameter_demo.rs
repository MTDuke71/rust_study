use competitive_linked_tree::{TreeDiameter, NodeId};

fn main() {
    println!("🔗 LinkedQueue Tree Diameter Demo");
    println!("==================================");
    println!();
    
    // Demonstrate why LinkedQueue is perfect for tree problems
    demo_dynamic_growth();
    demo_memory_efficiency();
    demo_competitive_programming_scenarios();
}

fn demo_dynamic_growth() {
    println!("📈 Dynamic Growth Demonstration");
    println!("-------------------------------");
    
    // Show how LinkedQueue adapts to different tree shapes
    let test_cases = vec![
        ("Deep tree", create_deep_tree(20)),
        ("Wide tree", create_wide_tree(20)),
        ("Balanced tree", create_balanced_tree(15)),
    ];
    
    for (name, (n, edges)) in test_cases {
        let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");
        
        if let Some((diameter, _path)) = diameter_finder.find_diameter() {
            println!("🌳 {}: {} nodes, diameter = {}", name, n, diameter);
            println!("   Queue grows dynamically during BFS traversal");
            println!("   Maximum queue size ≤ {} (adapts to tree shape)", n);
            println!("   LinkedQueue allocates exactly what's needed");
        }
        println!();
    }
}

fn demo_memory_efficiency() {
    println!("💾 Memory Efficiency Analysis");
    println!("------------------------------");
    
    // Compare memory usage patterns
    let sizes = vec![10, 50, 100, 500, 1000];
    
    for size in sizes {
        // Create line graph (worst case for queue size)
        let edges = (0..size-1).map(|i| (i, i + 1)).collect();
        let diameter_finder = TreeDiameter::from_edges(size, edges).expect("Valid tree");
        
        if let Some((diameter, _)) = diameter_finder.find_diameter() {
            println!("📊 Tree size: {} nodes", size);
            println!("   Diameter: {} (end-to-end)", diameter);
            println!("   LinkedQueue: Allocates nodes only when needed");
            println!("   Memory grows: O(queue_size) not O(total_nodes)");
            println!("   Peak queue size ≤ tree width (much less than total size)");
        }
        println!();
    }
    
    println!("💡 Key Insight: LinkedQueue memory ∝ BFS frontier size, not total tree size");
    println!();
}

fn demo_competitive_programming_scenarios() {
    println!("🏆 Competitive Programming Scenarios");
    println!("-------------------------------------");
    
    // Scenario 1: Tree diameter (classic problem)
    demo_diameter_problem();
    
    // Scenario 2: Tree center finding
    demo_tree_center();
    
    // Scenario 3: Unknown tree size from input
    demo_unknown_tree_size();
}

fn demo_diameter_problem() {
    println!("📝 Scenario 1: Classic Tree Diameter Problem");
    println!("   'Given tree edges, find longest simple path'");
    
    // Create interesting tree structure
    let edges = vec![
        (0, 1), (0, 2),           // Root has two children
        (1, 3), (1, 4),           // Left subtree
        (2, 5),                   // Right subtree (shorter)
        (3, 6), (3, 7),           // Extend left
        (6, 8), (6, 9),           // Deep left branch
        (8, 10),                  // Very deep
    ];
    let n = 11;
    
    let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");
    
    println!("   Tree: {} nodes, complex branching structure", n);
    println!("   Algorithm: Two-pass BFS (standard competitive programming approach)");
    
    if let Some((diameter, path)) = diameter_finder.find_diameter() {
        println!("   ✅ Diameter: {} (path: {:?})", diameter, path);
        println!("   🔗 LinkedQueue handled two independent BFS traversals");
        println!("   ⚡ Each BFS got fresh queue, no capacity pre-planning needed");
    }
    println!();
}

fn demo_tree_center() {
    println!("📝 Scenario 2: Tree Center Finding");
    println!("   'Find node(s) minimizing maximum distance to any other node'");
    
    // Tree where center is not obvious
    let edges = vec![
        (4, 3), (3, 2), (2, 1), (1, 0),  // Main chain
        (2, 5), (2, 6),                   // Branch from middle
        (1, 7),                           // Another branch
    ];
    let n = 8;
    
    let diameter_finder = TreeDiameter::from_edges(n, edges).expect("Valid tree");
    
    if let Some((diameter, path)) = diameter_finder.find_diameter() {
        if let Some(centers) = diameter_finder.find_center() {
            println!("   Tree: {} nodes, diameter = {}", n, diameter);
            println!("   Diameter path: {:?}", path);
            println!("   ✅ Center(s): {:?}", centers);
            println!("   💡 Center always lies on diameter path");
            println!("   🔗 LinkedQueue enables multiple BFS explorations for analysis");
        }
    }
    println!();
}

fn demo_unknown_tree_size() {
    println!("📝 Scenario 3: Unknown Tree Size from Input");
    println!("   'Parse tree from input stream, size unknown until end'");
    
    // Simulate parsing edges one by one (competitive programming style)
    let edge_stream = vec![
        (0, 1), (1, 2), (2, 3), (1, 4), (4, 5), (4, 6),
        (6, 7), (6, 8), (8, 9), (8, 10), (10, 11), (10, 12),
    ];
    
    println!("   Parsing edges from input: {:?}", edge_stream);
    
    // In real competitive programming, we'd determine n from the edges
    let max_node = edge_stream.iter()
        .flat_map(|(u, v)| [*u, *v])
        .max()
        .unwrap_or(0);
    let n = max_node + 1;
    
    let diameter_finder = TreeDiameter::from_edges(n, edge_stream).expect("Valid tree");
    
    println!("   Discovered: {} nodes total", n);
    
    if let Some((diameter, path)) = diameter_finder.find_diameter() {
        println!("   ✅ Diameter: {} (path length: {})", diameter, path.len());
        println!("   🔗 LinkedQueue perfect for unknown size:");
        println!("      • No need to pre-calculate capacity");
        println!("      • Grows as tree structure is discovered");
        println!("      • Each BFS adapts to actual tree shape");
        println!("      • Memory efficient for sparse or unbalanced trees");
    }
    
    println!();
    println!("🎯 Summary: LinkedQueue Excels When...");
    println!("   ✓ Tree size unknown until runtime");
    println!("   ✓ Tree shape varies dramatically (line vs balanced vs star)");
    println!("   ✓ Memory efficiency more important than cache performance");
    println!("   ✓ Multiple independent BFS traversals needed");
    println!("   ✓ Queue size varies significantly between runs");
    println!("   ✓ Input size can be very large or very small");
}

// Helper functions to create different tree shapes
fn create_deep_tree(n: usize) -> (usize, Vec<(NodeId, NodeId)>) {
    // Create a line graph (maximum depth)
    let edges = (0..n-1).map(|i| (i, i + 1)).collect();
    (n, edges)
}

fn create_wide_tree(n: usize) -> (usize, Vec<(NodeId, NodeId)>) {
    // Create a star graph with one center
    let mut edges = Vec::new();
    for i in 1..n {
        edges.push((0, i));
    }
    (n, edges)
}

fn create_balanced_tree(n: usize) -> (usize, Vec<(NodeId, NodeId)>) {
    // Create a complete binary tree
    let mut edges = Vec::new();
    for i in 0..n {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        
        if left < n {
            edges.push((i, left));
        }
        if right < n {
            edges.push((i, right));
        }
    }
    (n, edges)
}