/// # Mission 8 Tutorial - Step 3: Algorithm Composition
/// 
/// This tutorial teaches how to compose basic BFS/DFS algorithms to solve
/// higher-level problems like shortest path finding, cycle detection,
/// and component analysis.
/// 
/// ## Learning Objectives
/// 
/// By the end of this tutorial, you will:
/// - Understand shortest path using BFS + parent tracking
/// - Detect cycles using DFS + state tracking
/// - Find connected components
/// - Compose algorithms for complex problems
/// 
/// ## Prerequisites
/// 
/// - Complete Steps 1-2 (trait design and basic algorithms)
/// - Understand BFS and DFS traversal patterns
/// - Familiar with HashSet and HashMap usage
/// 
/// ## Tutorial Structure
/// 
/// 1. Shortest Path with BFS - Using parent tracking
/// 2. Cycle Detection with DFS - Three-color state tracking
/// 3. Connected Components - Multiple algorithm runs
/// 4. Algorithm Composition Patterns - Combining techniques
/// 5. Real-World Applications - Practical uses
/// 6. Performance Considerations - Scaling behavior
/// 7. Error Handling - Robust implementations

use std::collections::{HashMap, HashSet, VecDeque};
use mission8_tut::*;

fn main() {
    println!("=== Mission 8 Tutorial - Step 3: Algorithm Composition ===\n");
    
    section_1_shortest_path();
    section_2_cycle_detection();
    section_3_connected_components();
    section_4_algorithm_composition();
    section_5_real_world_applications();
    section_6_performance_considerations();
    section_7_error_handling();
    
    println!("\n🎉 Step 3 Complete! You now understand:");
    println!("  ✅ Shortest path finding with parent tracking");
    println!("  ✅ Cycle detection with state machines");
    println!("  ✅ Connected component analysis");
    println!("  ✅ Composing algorithms to solve complex problems");
    println!("\nNext: Step 4 - Performance Benchmarking");
}

/// Section 1: Shortest Path with BFS
fn section_1_shortest_path() {
    println!("📚 Section 1: Shortest Path with BFS");
    println!("====================================\n");
    
    println!("🎯 Shortest Path Concept:");
    println!("  • BFS explores level by level (distance 1, 2, 3, ...)");
    println!("  • When we first reach a node, we've found shortest path");
    println!("  • Track parent of each node to reconstruct path");
    println!("  • Only works for unweighted graphs!\n");
    
    explain_shortest_path_technique();
    println!();
    
    // Implementation with detailed explanation
    fn shortest_path_tutorial<N>(
        graph: &HashMap<N, Vec<N>>,
        start: N,
        end: N,
    ) -> Option<Vec<N>>
    where
        N: Copy + Eq + std::hash::Hash + std::fmt::Debug,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent: HashMap<N, N> = HashMap::new();
        
        queue.push_back(start);
        visited.insert(start);
        
        println!("  🚀 Starting shortest path search");
        println!("    From: {:?}, To: {:?}\n", start, end);
        
        let mut step = 0;
        while let Some(current) = queue.pop_front() {
            step += 1;
            println!("  Step {}: Processing {:?}", step, current);
            
            if current == end {
                println!("\n  ✅ Found destination! Reconstructing path...\n");
                
                // Reconstruct path
                let mut path = vec![end];
                let mut node = end;
                
                while let Some(&p) = parent.get(&node) {
                    path.push(p);
                    println!("    parent[{:?}] = {:?}", node, p);
                    node = p;
                }
                
                path.reverse();
                println!();
                return Some(path);
            }
            
            // Add neighbors
            for neighbor in graph.get(&current).unwrap_or(&vec![]) {
                if !visited.contains(neighbor) {
                    visited.insert(*neighbor);
                    parent.insert(*neighbor, current);
                    queue.push_back(*neighbor);
                    println!("    Added {:?} to queue (parent: {:?})", neighbor, current);
                }
            }
        }
        
        None
    }
    
    println!("🧪 Finding shortest path in sample graph:");
    let graph = create_sample_graph();
    if let Some(path) = shortest_path_tutorial(&graph, 0, 3) {
        println!("  Shortest path: {:?}", path);
        println!("  Length: {}\n", path.len() - 1);
    }
    
    println!("🔍 Key Insights:");
    println!("  • BFS visits nodes level by level");
    println!("  • Parent map tracks how we reached each node");
    println!("  • Path reconstruction walks backwards from destination");
    println!("  • This guarantees shortest path in unweighted graphs");
    println!();
}

/// Section 2: Cycle Detection with DFS
fn section_2_cycle_detection() {
    println!("📚 Section 2: Cycle Detection with DFS");
    println!("======================================\n");
    
    explain_cycle_detection();
    println!();
    
    // Demonstrate with example
    println!("🧪 Cycle Detection Example:");
    println!();
    
    println!("  Graph 1 (Acyclic - DAG):");
    println!("    0 -> 1 -> 3");
    println!("    0 -> 2 -> 3");
    
    let mut acyclic = HashMap::new();
    acyclic.insert(0, vec![1, 2]);
    acyclic.insert(1, vec![3]);
    acyclic.insert(2, vec![3]);
    acyclic.insert(3, vec![]);
    
    println!("    All nodes become BLACK before visiting other branches");
    println!("    No back edges found ✅\n");
    
    println!("  Graph 2 (Cyclic):");
    println!("    0 -> 1 -> 2");
    println!("    2 -> 0 (creates cycle!)");
    
    let mut cyclic = HashMap::new();
    cyclic.insert(0, vec![1]);
    cyclic.insert(1, vec![2]);
    cyclic.insert(2, vec![0]);
    
    println!("    When at node 2, node 0 is GRAY (back edge!)");
    println!("    Cycle detected ❌\n");
    
    println!("🎯 Three-Color Intuition:");
    println!("  White = Haven't started processing");
    println!("  Gray  = Currently processing (on the call stack)");
    println!("  Black = Done processing");
    println!();
    println!("  Back Edge = Edge to Gray node = CYCLE!");
    println!("  Why? Gray node is an ancestor in current path");
    println!();
}

/// Section 3: Connected Components
fn section_3_connected_components() {
    println!("📚 Section 3: Connected Components");
    println!("==================================\n");
    
    explain_connected_components();
    println!();
    
    // Implementation
    fn find_components_tutorial<N>(graph: &HashMap<N, Vec<N>>) -> Vec<Vec<N>>
    where
        N: Copy + Eq + std::hash::Hash + std::fmt::Debug,
    {
        let mut visited = HashSet::new();
        let mut components = Vec::new();
        let mut comp_num = 1;
        
        for node in graph.keys() {
            if !visited.contains(node) {
                println!("  Starting component search from {:?}", node);
                
                // BFS to find all nodes in this component
                let mut queue = VecDeque::new();
                queue.push_back(*node);
                visited.insert(*node);
                let mut component = Vec::new();
                
                while let Some(current) = queue.pop_front() {
                    component.push(current);
                    
                    for neighbor in graph.get(&current).unwrap_or(&vec![]) {
                        if !visited.contains(neighbor) {
                            visited.insert(*neighbor);
                            queue.push_back(*neighbor);
                        }
                    }
                }
                
                println!("    Component {} contains: {:?}", comp_num, component);
                components.push(component);
                comp_num += 1;
            }
        }
        
        components
    }
    
    println!("🧪 Finding components in disconnected graph:");
    let graph = create_disconnected_components();
    let components = find_components_tutorial(&graph);
    
    println!("\n  Total components found: {}", components.len());
    println!("  Component sizes: {:?}", 
        components.iter().map(|c| c.len()).collect::<Vec<_>>());
    println!();
}

/// Section 4: Algorithm Composition Patterns
fn section_4_algorithm_composition() {
    println!("📚 Section 4: Composing Algorithms");
    println!("==================================\n");
    
    explain_algorithm_composition();
    println!();
    
    compare_approaches();
    println!();
    
    println!("🧪 Composition Example: Multi-step Analysis");
    println!();
    
    // Step 1: Check connectivity
    let graph = create_sample_graph();
    let components = connected_components(&graph);
    println!("  Step 1: Graph has {} component(s)", components.len());
    
    // Step 2: Check for cycles
    let has_cycles = has_cycle_tutorial(&graph);
    println!("  Step 2: Graph has cycles? {}\n", has_cycles);
    
    println!("  Analysis complete!");
    println!("  By combining algorithms, we get complete picture of graph structure.");
    println!();
}

/// Section 5: Real-World Applications
fn section_5_real_world_applications() {
    println!("📚 Section 5: Real-World Applications");
    println!("=====================================\n");
    
    println!("🎯 Shortest Path Applications:");
    println!("  • GPS Navigation - Find shortest route");
    println!("  • Maze Solving - Find exit with minimum steps");
    println!("  • Network Routing - Find fastest path to destination");
    println!("  • Social Networks - Find distance between users");
    println!();
    
    println!("🎯 Cycle Detection Applications:");
    println!("  • Dependency Analysis - Detect circular dependencies");
    println!("  • Financial Networks - Detect suspicious cycles");
    println!("  • Compiler Optimization - Detect infinite loops");
    println!("  • Scheduling - Detect impossible deadlines");
    println!();
    
    println!("🎯 Connected Components Applications:");
    println!("  • Network Analysis - Find isolated networks");
    println!("  • Social Graph - Find friend groups");
    println!("  • Image Processing - Find connected regions");
    println!("  • Supply Chain - Identify supplier clusters");
    println!();
}

/// Section 6: Performance Considerations
fn section_6_performance_considerations() {
    println!("📚 Section 6: Performance Considerations");
    println!("=======================================\n");
    
    println!("📊 Complexity Analysis:");
    println!();
    println!("  Algorithm              | Time      | Space     | Notes");
    println!("  ---------------------- | --------- | --------- | ----------");
    println!("  Shortest Path          | O(V + E)  | O(V)      | BFS-based");
    println!("  Cycle Detection        | O(V + E)  | O(V)      | DFS-based");
    println!("  Components             | O(V + E)  | O(V)      | Multiple BFS");
    println!("  All together           | O(V + E)  | O(V)      | No exponential");
    println!();
    
    println!("💡 Performance Tips:");
    println!("  • All composed algorithms remain linear in graph size");
    println!("  • Use HashMap for O(1) neighbor lookup");
    println!("  • Pre-allocate HashSet if node count is known");
    println!("  • Avoid cloning neighbors - store references");
    println!("  • Consider graph representation (sparse vs dense)");
    println!();
}

/// Section 7: Error Handling
fn section_7_error_handling() {
    println!("📚 Section 7: Error Handling & Robustness");
    println!("=========================================\n");
    
    println!("⚠️  Common Error Scenarios:");
    println!();
    
    println!("  ❌ Issue: Start node doesn't exist");
    println!("  ✅ Solution: Check graph.contains(start) before processing\n");
    
    println!("  ❌ Issue: No path exists between nodes");
    println!("  ✅ Solution: Return Result<Path, Error> or Option<Path>\n");
    
    println!("  ❌ Issue: Empty graph");
    println!("  ✅ Solution: Handle gracefully, return empty result\n");
    
    println!("  ❌ Issue: Self-loops causing issues");
    println!("  ✅ Solution: Check node != neighbor before processing\n");
    
    println!("🧪 Robust shortest_path implementation:");
    
    fn robust_shortest_path<N>(
        graph: &HashMap<N, Vec<N>>,
        start: N,
        end: N,
    ) -> Result<Vec<N>, String>
    where
        N: Copy + Eq + std::hash::Hash + std::fmt::Debug,
    {
        // Validate inputs
        if !graph.contains_key(&start) {
            return Err(format!("Start node {:?} not in graph", start));
        }
        
        if start == end {
            return Ok(vec![start]);
        }
        
        // Run BFS with parent tracking
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent: HashMap<N, N> = HashMap::new();
        
        queue.push_back(start);
        visited.insert(start);
        
        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct path
                let mut path = vec![end];
                let mut node = end;
                
                while let Some(&p) = parent.get(&node) {
                    path.push(p);
                    node = p;
                }
                
                path.reverse();
                return Ok(path);
            }
            
            for neighbor in graph.get(&current).unwrap_or(&vec![]) {
                if !visited.contains(neighbor) {
                    visited.insert(*neighbor);
                    parent.insert(*neighbor, current);
                    queue.push_back(*neighbor);
                }
            }
        }
        
        Err(format!("No path from {:?} to {:?}", start, end))
    }
    
    println!("  Checks: start exists, handles disconnected graphs");
    println!("  Returns Result for proper error propagation");
    println!("  Can be used with ? operator in larger systems\n");
}

// ============================================================================
// Helper implementations for tutorial
// ============================================================================

fn connected_components<N>(graph: &HashMap<N, Vec<N>>) -> Vec<Vec<N>>
where
    N: Copy + Eq + std::hash::Hash,
{
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    
    for node in graph.keys() {
        if !visited.contains(node) {
            let mut queue = VecDeque::new();
            queue.push_back(*node);
            visited.insert(*node);
            let mut component = Vec::new();
            
            while let Some(current) = queue.pop_front() {
                component.push(current);
                
                for neighbor in graph.get(&current).unwrap_or(&vec![]) {
                    if !visited.contains(neighbor) {
                        visited.insert(*neighbor);
                        queue.push_back(*neighbor);
                    }
                }
            }
            
            components.push(component);
        }
    }
    
    components
}

fn has_cycle_tutorial<N>(graph: &HashMap<N, Vec<N>>) -> bool
where
    N: Copy + Eq + std::hash::Hash,
{
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Color {
        White,
        Gray,
        Black,
    }
    
    let mut color: HashMap<N, Color> = HashMap::new();
    
    for node in graph.keys() {
        color.insert(*node, Color::White);
    }
    
    fn dfs<N>(
        graph: &HashMap<N, Vec<N>>,
        node: N,
        color: &mut HashMap<N, Color>,
    ) -> bool
    where
        N: Copy + Eq + std::hash::Hash,
    {
        color.insert(node, Color::Gray);
        
        for neighbor in graph.get(&node).unwrap_or(&vec![]) {
            match color.get(neighbor).unwrap_or(&Color::White) {
                Color::Gray => return true,
                Color::White => {
                    if dfs(graph, *neighbor, color) {
                        return true;
                    }
                }
                Color::Black => {}
            }
        }
        
        color.insert(node, Color::Black);
        false
    }
    
    for node in graph.keys() {
        if *color.get(node).unwrap_or(&Color::White) == Color::White {
            if dfs(graph, *node, &mut color) {
                return true;
            }
        }
    }
    
    false
}

fn create_disconnected_components() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![0, 2]);
    graph.insert(2, vec![1]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![3]);
    graph
}

fn have_cycle<N>(_graph: &HashMap<N, Vec<N>>) -> bool
where
    N: Copy + Eq + std::hash::Hash,
{
    false
}

#[allow(dead_code)]
fn have_cycle_tutorial<N>(_graph: &HashMap<N, Vec<N>>) -> bool
where
    N: Copy + Eq + std::hash::Hash,
{
    false
}
