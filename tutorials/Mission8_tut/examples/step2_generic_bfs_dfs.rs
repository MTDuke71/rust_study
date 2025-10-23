/// # Mission 8 Tutorial - Step 2: Generic BFS & DFS Implementation
/// 
/// This tutorial teaches you how to implement breadth-first search (BFS) and
/// depth-first search (DFS) algorithms that work on any graph type through
/// trait abstraction.
/// 
/// ## Learning Objectives
/// 
/// By the end of this tutorial, you will:
/// - Implement BFS using VecDeque (FIFO queue)
/// - Implement DFS using explicit Vec stack (LIFO)
/// - Understand visited tracking patterns
/// - Compare BFS vs DFS traversal orders
/// - Use trait abstraction effectively
/// 
/// ## Prerequisites
/// 
/// - Complete Step 1 (Algorithm Trait Design)
/// - Understand basic data structures (Vec, VecDeque, HashSet)
/// - Familiar with trait concepts in Rust
/// 
/// ## Tutorial Structure
/// 
/// 1. Understanding BFS - Level-order traversal concept
/// 2. Implementing BFS - Queue-based algorithm
/// 3. Understanding DFS - Depth-first exploration
/// 4. Implementing DFS - Stack-based algorithm
/// 5. Comparing BFS vs DFS - Different traversal orders
/// 6. Testing with Multiple Graph Types - Trait abstraction
/// 7. Common Mistakes - Error patterns to avoid
///
use std::collections::{HashMap, HashSet, VecDeque};
use mission8_tut::*;

fn main() {
    println!("=== Mission 8 Tutorial - Step 2: Generic BFS & DFS Implementation ===\n");
    
    // Section 1: Understanding BFS
    section_1_understanding_bfs();
    
    // Section 2: Implementing BFS
    section_2_implementing_bfs();
    
    // Section 3: Understanding DFS
    section_3_understanding_dfs();
    
    // Section 4: Implementing DFS
    section_4_implementing_dfs();
    
    // Section 5: Comparing BFS vs DFS
    section_5_comparing_algorithms();
    
    // Section 6: Testing with Multiple Graph Types
    section_6_multiple_graph_types();
    
    // Section 7: Common Mistakes
    section_7_common_mistakes();
    
    println!("\n🎉 Step 2 Complete! You now understand:");
    println!("  ✅ BFS level-order traversal with queue");
    println!("  ✅ DFS depth-first exploration with stack");
    println!("  ✅ Visited tracking to prevent cycles");
    println!("  ✅ Algorithm comparison and selection");
    println!("  ✅ Generic implementation with traits");
    println!("\nNext: Step 3 - Algorithm Composition (shortest path, cycle detection)");
}

/// Section 1: Understanding BFS - Level-order traversal concept
fn section_1_understanding_bfs() {
    println!("📚 Section 1: Understanding BFS");
    println!("==============================\n");
    
    println!("🎯 BFS (Breadth-First Search) Concept:");
    println!("  • Explores nodes level by level");
    println!("  • Uses a FIFO queue (first in, first out)");
    println!("  • Guarantees shortest path in unweighted graphs");
    println!("  • Memory usage: O(width) of the graph\n");
    
    println!("📊 BFS on our sample graph:");
    println!("  Graph structure:");
    println!("      0");
    println!("     / \\");
    println!("    1   2"); 
    println!("     \\ /");
    println!("      3");
    println!();
    
    println!("  BFS Traversal Order:");
    println!("  Level 0: [0]     <- Start here");
    println!("  Level 1: [1, 2]  <- All neighbors of level 0");
    println!("  Level 2: [3]     <- All neighbors of level 1");
    println!("  Result: [0, 1, 2, 3] (level-order)");
    println!();
    
    explain_bfs_shortest_path();
    println!();
}

/// Section 2: Implementing BFS - Queue-based algorithm
fn section_2_implementing_bfs() {
    println!("🔧 Section 2: Implementing BFS");
    println!("==============================\n");
    
    println!("💡 BFS Implementation Pattern:");
    println!("  1. Initialize visited set and queue");
    println!("  2. Add start node to both visited and queue");
    println!("  3. While queue not empty:");
    println!("     a. Remove front node from queue");
    println!("     b. Process the node (add to result)");
    println!("     c. For each unvisited neighbor:");
    println!("        - Mark as visited");
    println!("        - Add to queue");
    println!();
    
    /// Tutorial BFS implementation with detailed tracing
    fn tutorial_bfs<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N> 
    where
        N: Copy + Eq + std::hash::Hash + std::fmt::Debug,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        let mut step = 0;
        
        // Step 0: Initialize with start node
        queue.push_back(start);
        visited.insert(start);
        
        println!("  🚀 Starting BFS from node {:?}", start);
        visualize_bfs_state(step, start, &queue, &visited);
        
        while let Some(current) = queue.pop_front() {
            step += 1;
            result.push(current);
            
            println!("  📍 Processing node {:?}", current);
            
            // Get neighbors from the graph
            let neighbors = match graph.get(&current) {
                Some(neighbors) => neighbors,
                None => {
                    println!("    ⚠️  Node {:?} not found in graph", current);
                    continue;
                }
            };
            
            println!("    Neighbors: {:?}", neighbors);
            
            // Add unvisited neighbors to queue
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    println!("    ➕ Added {:?} to queue", neighbor);
                } else {
                    println!("    ⏭️  {:?} already visited, skipping", neighbor);
                }
            }
            
            visualize_bfs_state(step, current, &queue, &visited);
        }
        
        println!("  ✅ BFS Complete!");
        result
    }
    
    println!("🧪 Running BFS on Sample Graph:");
    let graph = create_sample_graph();
    let bfs_result = tutorial_bfs(&graph, 0);
    
    println!();
    print_traversal("Final BFS Result", &bfs_result);
    println!();
    
    println!("🔍 Key BFS Observations:");
    println!("  • Node 0 processed first (start node)");
    println!("  • Nodes 1 and 2 processed next (level 1)");
    println!("  • Node 3 processed last (level 2)");
    println!("  • Queue ensures level-order processing");
    println!("  • Visited set prevents revisiting nodes");
    println!();
}

/// Section 3: Understanding DFS - Depth-first exploration  
fn section_3_understanding_dfs() {
    println!("📚 Section 3: Understanding DFS");
    println!("==============================\n");
    
    println!("🎯 DFS (Depth-First Search) Concept:");
    println!("  • Explores as far as possible along each branch");
    println!("  • Uses a LIFO stack (last in, first out)");
    println!("  • Better for cycle detection and topological sort");
    println!("  • Memory usage: O(depth) of the graph\n");
    
    println!("📊 DFS on our sample graph:");
    println!("  Graph structure:");
    println!("      0");
    println!("     / \\");
    println!("    1   2");
    println!("     \\ /");
    println!("      3");
    println!();
    
    println!("  DFS Traversal Path (depends on neighbor order):");
    println!("  Path 1: 0 -> 1 -> 3, backtrack to 0 -> 2");
    println!("  Path 2: 0 -> 2 -> 3, backtrack to 0 -> 1");
    println!("  Result: Explores one branch completely before others");
    println!();
    
    explain_dfs_advantages();
    println!();
}

/// Section 4: Implementing DFS - Stack-based algorithm
fn section_4_implementing_dfs() {
    println!("🔧 Section 4: Implementing DFS");
    println!("==============================\n");
    
    println!("💡 DFS Implementation Pattern:");
    println!("  1. Initialize visited set and stack");
    println!("  2. Add start node to stack (not visited yet!)");
    println!("  3. While stack not empty:");
    println!("     a. Remove top node from stack");
    println!("     b. If not visited:");
    println!("        - Mark as visited");
    println!("        - Process the node");
    println!("        - Add all unvisited neighbors to stack");
    println!();
    
    println!("⚠️  Important DFS Pattern:");
    println!("  • Don't mark visited when adding to stack");
    println!("  • Mark visited when popping from stack");
    println!("  • This allows multiple paths to same node in stack");
    println!("  • First path to reach node wins");
    println!();
    
    /// Tutorial DFS implementation with detailed tracing
    fn tutorial_dfs<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N>
    where
        N: Copy + Eq + std::hash::Hash + std::fmt::Debug,
    {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut result = Vec::new();
        let mut step = 0;
        
        // Step 0: Initialize with start node
        stack.push(start);
        
        println!("  🚀 Starting DFS from node {:?}", start);
        visualize_dfs_state(step, start, &stack, &visited);
        
        while let Some(current) = stack.pop() {
            if !visited.contains(&current) {
                step += 1;
                visited.insert(current);
                result.push(current);
                
                println!("  📍 Processing node {:?}", current);
                
                // Get neighbors from the graph
                let neighbors = match graph.get(&current) {
                    Some(neighbors) => neighbors,
                    None => {
                        println!("    ⚠️  Node {:?} not found in graph", current);
                        continue;
                    }
                };
                
                println!("    Neighbors: {:?}", neighbors);
                
                // Add unvisited neighbors to stack (reverse order for consistency)
                let mut unvisited_neighbors = Vec::new();
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        unvisited_neighbors.push(neighbor);
                    }
                }
                
                // Reverse to maintain consistent ordering
                unvisited_neighbors.reverse();
                
                for neighbor in unvisited_neighbors {
                    stack.push(neighbor);
                    println!("    ➕ Added {:?} to stack", neighbor);
                }
                
                visualize_dfs_state(step, current, &stack, &visited);
            } else {
                println!("  ⏭️  Node {:?} already visited, skipping", current);
            }
        }
        
        println!("  ✅ DFS Complete!");
        result
    }
    
    println!("🧪 Running DFS on Sample Graph:");
    let graph = create_sample_graph();
    let dfs_result = tutorial_dfs(&graph, 0);
    
    println!();
    print_traversal("Final DFS Result", &dfs_result);
    println!();
    
    println!("🔍 Key DFS Observations:");
    println!("  • Explores one branch completely before backtracking");
    println!("  • Stack ensures depth-first processing");
    println!("  • Visited check happens when popping (not pushing)");
    println!("  • Order depends on neighbor traversal order");
    println!();
}

/// Section 5: Comparing BFS vs DFS - Different traversal orders
fn section_5_comparing_algorithms() {
    println!("📊 Section 5: Comparing BFS vs DFS");
    println!("===================================\n");
    
    /// Production BFS implementation (without tracing)
    fn bfs<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N>
    where
        N: Copy + Eq + std::hash::Hash,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        queue.push_back(start);
        visited.insert(start);
        
        while let Some(current) = queue.pop_front() {
            result.push(current);
            
            if let Some(neighbors) = graph.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        
        result
    }
    
    /// Production DFS implementation (without tracing)
    fn dfs<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N>
    where
        N: Copy + Eq + std::hash::Hash,
    {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut result = Vec::new();
        
        stack.push(start);
        
        while let Some(current) = stack.pop() {
            if !visited.contains(&current) {
                visited.insert(current);
                result.push(current);
                
                if let Some(neighbors) = graph.get(&current) {
                    let mut neighbors = neighbors.clone();
                    neighbors.reverse();
                    
                    for neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }
        
        result
    }
    
    // Test on different graph types
    println!("🧪 Testing on Sample Graph:");
    let graph1 = create_sample_graph();
    let bfs1 = bfs(&graph1, 0);
    let dfs1 = dfs(&graph1, 0);
    compare_algorithms("Sample Graph", &bfs1, &dfs1);
    
    println!("\n🧪 Testing on Tree Graph:");
    let graph2 = create_tree_graph();
    let bfs2 = bfs(&graph2, 0);
    let dfs2 = dfs(&graph2, 0);
    compare_algorithms("Tree Graph", &bfs2, &dfs2);
    
    println!("\n🧪 Testing on Linear Graph:");
    let graph3 = create_linear_graph();
    let bfs3 = bfs(&graph3, 0);
    let dfs3 = dfs(&graph3, 0);
    compare_algorithms("Linear Graph", &bfs3, &dfs3);
    
    println!("\n🎯 When to Use Each Algorithm:");
    println!("  BFS is better for:");
    println!("  • Shortest path in unweighted graphs");
    println!("  • Finding nodes at specific distance");
    println!("  • Level-order processing");
    println!("  • Minimum spanning tree");
    println!();
    println!("  DFS is better for:");
    println!("  • Cycle detection");
    println!("  • Topological sorting");
    println!("  • Finding strongly connected components");
    println!("  • Maze solving (with backtracking)");
    println!();
}

/// Section 6: Testing with Multiple Graph Types - Trait abstraction
fn section_6_multiple_graph_types() {
    println!("🔧 Section 6: Testing with Multiple Graph Types");
    println!("===============================================\n");
    
    println!("🎯 Trait Abstraction Benefit:");
    println!("  Our algorithms work on ANY type implementing our Graph trait!");
    println!("  This includes HashMap, custom structs, matrices, etc.");
    println!();
    
    /// Simple Graph trait for tutorial (matches Mission8 design)
    trait Graph {
        type Node: Copy + Eq + std::hash::Hash + std::fmt::Debug;
        
        fn neighbors(&self, node: Self::Node) -> Vec<Self::Node>;
        #[allow(dead_code)]
        fn contains(&self, node: Self::Node) -> bool;
    }
    
    /// Implement Graph trait for HashMap
    impl<N> Graph for HashMap<N, Vec<N>>
    where
        N: Copy + Eq + std::hash::Hash + std::fmt::Debug,
    {
        type Node = N;
        
        fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
            self.get(&node).cloned().unwrap_or_default()
        }
        
        fn contains(&self, node: Self::Node) -> bool {
            self.contains_key(&node)
        }
    }
    
    /// Generic BFS that works on any Graph
    fn generic_bfs<G: Graph>(graph: &G, start: G::Node) -> Vec<G::Node> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        queue.push_back(start);
        visited.insert(start);
        
        while let Some(current) = queue.pop_front() {
            result.push(current);
            
            for neighbor in graph.neighbors(current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        
        result
    }
    
    println!("🧪 Testing Generic BFS on Different Node Types:");
    
    // Test with u32 nodes
    println!("\n  Test 1: u32 Nodes");
    let int_graph: HashMap<u32, Vec<u32>> = create_sample_graph();
    let int_result = generic_bfs(&int_graph, 0);
    print_traversal("BFS on u32 graph", &int_result);
    
    // Test with string nodes  
    println!("\n  Test 2: String Nodes");
    let mut str_graph: HashMap<&str, Vec<&str>> = HashMap::new();
    str_graph.insert("A", vec!["B", "C"]);
    str_graph.insert("B", vec!["D"]);
    str_graph.insert("C", vec!["D"]);
    str_graph.insert("D", vec![]);
    
    let str_result = generic_bfs(&str_graph, "A");
    print_traversal("BFS on string graph", &str_result);
    
    // Test with char nodes
    println!("\n  Test 3: char Nodes");
    let mut char_graph: HashMap<char, Vec<char>> = HashMap::new();
    char_graph.insert('X', vec!['Y', 'Z']);
    char_graph.insert('Y', vec!['W']);
    char_graph.insert('Z', vec!['W']);
    char_graph.insert('W', vec![]);
    
    let char_result = generic_bfs(&char_graph, 'X');
    print_traversal("BFS on char graph", &char_result);
    
    println!("\n✨ Generic Programming Success!");
    println!("  The same algorithm works on different node types");
    println!("  This is the power of Rust's trait system!");
    println!();
}

/// Section 7: Common Mistakes - Error patterns to avoid
fn section_7_common_mistakes() {
    println!("⚠️  Section 7: Common Mistakes");
    println!("==============================\n");
    
    show_common_mistakes();
    
    println!("\n🧪 Example: Wrong BFS Implementation");
    
    /// ❌ WRONG: This will cause infinite loops or miss nodes
    #[allow(dead_code)]
    fn wrong_bfs_example<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N>
    where
        N: Copy + Eq + std::hash::Hash + std::fmt::Debug,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        queue.push_back(start);
        // ❌ MISTAKE: Not marking start as visited immediately
        
        while let Some(current) = queue.pop_front() {
            // ❌ MISTAKE: Checking visited after popping
            if visited.contains(&current) {
                continue;
            }
            
            visited.insert(current);
            result.push(current);
            
            if let Some(neighbors) = graph.get(&current) {
                for &neighbor in neighbors {
                    // ❌ MISTAKE: Not checking if already visited before adding
                    queue.push_back(neighbor);
                }
            }
        }
        
        result
    }
    
    println!("  ❌ Wrong pattern can cause:");
    println!("     • Infinite loops in cyclic graphs");
    println!("     • Duplicate nodes in queue");
    println!("     • Incorrect traversal order");
    println!("     • Performance degradation");
    println!();
    
    println!("🧪 Testing Wrong Implementation:");
    let cyclic_graph = create_cyclic_graph();
    println!("  Graph: 0 -> 1 -> 2 -> 0 (cycle)");
    
    // This would cause problems with the wrong implementation
    // For safety, we'll just show the correct implementation
    fn correct_bfs<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N>
    where
        N: Copy + Eq + std::hash::Hash,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        queue.push_back(start);
        visited.insert(start);  // ✅ Mark visited when adding to queue
        
        while let Some(current) = queue.pop_front() {
            result.push(current);
            
            if let Some(neighbors) = graph.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {  // ✅ Check before adding
                        visited.insert(neighbor);       // ✅ Mark visited immediately
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        
        result
    }
    
    let correct_result = correct_bfs(&cyclic_graph, 0);
    print_traversal("Correct BFS on cyclic graph", &correct_result);
    
    println!("\n✅ Correct Implementation Patterns:");
    println!("  1. Mark nodes visited when adding to queue/stack");
    println!("  2. Check visited status before adding neighbors");
    println!("  3. Use explicit data structures (avoid recursion)");
    println!("  4. Handle edge cases (empty graph, disconnected)");
    println!("  5. Test with cyclic graphs to verify correctness");
    println!();
}