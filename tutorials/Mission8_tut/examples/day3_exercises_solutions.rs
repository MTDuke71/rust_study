//! Day 3 Exercise Solutions: Algorithm Composition
//!
//! This file contains solutions to the Day 3 exercises from the Mission 8 tutorial.
//! These exercises build upon the basic algorithm composition concepts to solve
//! more advanced graph problems.
//!
//! Run with: cargo run --example day3_exercises_solutions

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

/// Exercise 1: Find all cycles in a graph
///
/// This extends basic cycle detection to find ALL cycles in a graph.
/// Uses DFS with back-edge detection and path tracking.

#[derive(Debug, Clone, PartialEq)]
enum NodeColor {
    White, // Unvisited
    Gray,  // Currently visiting
    Black, // Finished visiting
}

fn find_all_cycles<N>(graph: &HashMap<N, Vec<N>>) -> Vec<Vec<N>>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let mut all_cycles = Vec::new();
    let mut color: HashMap<N, NodeColor> = HashMap::new();
    let mut path = Vec::new();

    // Initialize all nodes as white
    for &node in graph.keys() {
        color.insert(node, NodeColor::White);
    }

    for &node in graph.keys() {
        if color[&node] == NodeColor::White {
            find_all_cycles_dfs(graph, node, &mut color, &mut path, &mut all_cycles);
        }
    }

    all_cycles
}

fn find_all_cycles_dfs<N>(
    graph: &HashMap<N, Vec<N>>,
    node: N,
    color: &mut HashMap<N, NodeColor>,
    path: &mut Vec<N>,
    all_cycles: &mut Vec<Vec<N>>,
) where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    color.insert(node, NodeColor::Gray);
    path.push(node);

    for &neighbor in graph.get(&node).unwrap_or(&Vec::new()) {
        match color.get(&neighbor).unwrap_or(&NodeColor::White) {
            NodeColor::Gray => {
                // Back edge found - extract cycle
                if let Some(pos) = path.iter().position(|&n| n == neighbor) {
                    let cycle = path[pos..].to_vec();
                    if cycle.len() > 1 {
                        // Avoid self-loops as trivial cycles
                        all_cycles.push(cycle);
                    }
                }
            }
            NodeColor::White => {
                find_all_cycles_dfs(graph, neighbor, color, path, all_cycles);
            }
            NodeColor::Black => {
                // Already processed, skip
            }
        }
    }

    path.pop();
    color.insert(node, NodeColor::Black);
}

/// Exercise 2: Find longest path in DAG
///
/// This finds the longest path in a Directed Acyclic Graph (DAG).
/// Uses dynamic programming with topological sorting.
///
fn find_longest_path_in_dag<N>(graph: &HashMap<N, Vec<N>>, start: N, end: N) -> Option<Vec<N>>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    // First, check if the graph is actually a DAG
    if has_cycle(graph) {
        return None; // Not a DAG
    }

    // Get topological order
    let topo_order = topological_sort(graph);
    if topo_order.is_empty() {
        return None;
    }

    // Dynamic programming: longest distance to each node
    let mut distances: HashMap<N, i32> = HashMap::new();
    let mut parent: HashMap<N, N> = HashMap::new();

    // Initialize distances
    for &node in graph.keys() {
        distances.insert(node, if node == start { 0 } else { i32::MIN });
    }

    // Process nodes in topological order
    for &node in &topo_order {
        if distances[&node] == i32::MIN {
            continue; // Node not reachable from start
        }

        for &neighbor in graph.get(&node).unwrap_or(&Vec::new()) {
            let new_distance = distances[&node] + 1;
            if new_distance > distances[&neighbor] {
                distances.insert(neighbor, new_distance);
                parent.insert(neighbor, node);
            }
        }
    }

    // Check if end is reachable
    if distances[&end] == i32::MIN {
        return None;
    }

    // Reconstruct path
    let mut path = Vec::new();
    let mut current = end;

    while current != start {
        path.push(current);
        current = parent[&current];
    }
    path.push(start);
    path.reverse();

    Some(path)
}

fn has_cycle<N>(graph: &HashMap<N, Vec<N>>) -> bool
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let mut color: HashMap<N, NodeColor> = HashMap::new();

    for &node in graph.keys() {
        color.insert(node, NodeColor::White);
    }

    for &node in graph.keys() {
        if color[&node] == NodeColor::White && has_cycle_dfs(graph, node, &mut color) {
            return true;
        }
    }

    false
}

fn has_cycle_dfs<N>(graph: &HashMap<N, Vec<N>>, node: N, color: &mut HashMap<N, NodeColor>) -> bool
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    color.insert(node, NodeColor::Gray);

    for &neighbor in graph.get(&node).unwrap_or(&Vec::new()) {
        match color.get(&neighbor).unwrap_or(&NodeColor::White) {
            NodeColor::Gray => return true, // Back edge found
            NodeColor::White => {
                if has_cycle_dfs(graph, neighbor, color) {
                    return true;
                }
            }
            NodeColor::Black => {} // Already processed
        }
    }

    color.insert(node, NodeColor::Black);
    false
}

fn topological_sort<N>(graph: &HashMap<N, Vec<N>>) -> Vec<N>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    for &node in graph.keys() {
        if !visited.contains(&node) {
            topological_sort_dfs(graph, node, &mut visited, &mut stack);
        }
    }

    stack.reverse();
    stack
}

fn topological_sort_dfs<N>(
    graph: &HashMap<N, Vec<N>>,
    node: N,
    visited: &mut HashSet<N>,
    stack: &mut Vec<N>,
) where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    visited.insert(node);

    for &neighbor in graph.get(&node).unwrap_or(&Vec::new()) {
        if !visited.contains(&neighbor) {
            topological_sort_dfs(graph, neighbor, visited, stack);
        }
    }

    stack.push(node);
}

/// Exercise 3: Implement bidirectional BFS
///
/// This is an optimization for finding shortest paths by searching
/// from both start and end simultaneously.
fn bidirectional_bfs<N>(graph: &HashMap<N, Vec<N>>, start: N, end: N) -> Option<Vec<N>>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    if start == end {
        return Some(vec![start]);
    }

    let mut visited_start: HashMap<N, N> = HashMap::new();
    let mut visited_end: HashMap<N, N> = HashMap::new();
    let mut queue_start = VecDeque::new();
    let mut queue_end = VecDeque::new();

    queue_start.push_back(start);
    queue_end.push_back(end);
    visited_start.insert(start, start);
    visited_end.insert(end, end);

    while !queue_start.is_empty() && !queue_end.is_empty() {
        // Expand from start side
        if let Some(path) =
            expand_bidirectional_side(graph, &mut queue_start, &mut visited_start, &visited_end)
        {
            return Some(path);
        }

        // Expand from end side
        if let Some(path) =
            expand_bidirectional_side(graph, &mut queue_end, &mut visited_end, &visited_start)
        {
            return Some(path);
        }
    }

    None
}

fn expand_bidirectional_side<N>(
    graph: &HashMap<N, Vec<N>>,
    queue: &mut VecDeque<N>,
    visited_this: &mut HashMap<N, N>,
    visited_other: &HashMap<N, N>,
) -> Option<Vec<N>>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let current = queue.pop_front()?;

    // Check if we've met the other search
    if visited_other.contains_key(&current) {
        return reconstruct_bidirectional_path(current, visited_this, visited_other);
    }

    for &neighbor in graph.get(&current).unwrap_or(&Vec::new()) {
        if let std::collections::hash_map::Entry::Vacant(e) = visited_this.entry(neighbor) {
            e.insert(current);
            queue.push_back(neighbor);
        }
    }

    None
}

fn reconstruct_bidirectional_path<N>(
    meeting_point: N,
    visited_start: &HashMap<N, N>,
    visited_end: &HashMap<N, N>,
) -> Option<Vec<N>>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let mut path = Vec::new();

    // Build path from start to meeting point
    let mut current = meeting_point;
    while let Some(&parent) = visited_start.get(&current) {
        path.push(current);
        current = parent;
        if current == parent {
            break;
        }
    }
    path.reverse();

    // Build path from meeting point to end
    current = visited_end[&meeting_point];
    while let Some(&parent) = visited_end.get(&current) {
        if current != meeting_point {
            path.push(current);
        }
        current = parent;
        if current == parent {
            break;
        }
    }

    Some(path)
}

/// Exercise 4: Find bridges in graph
///
/// A bridge is an edge whose removal increases the number of connected components.
/// Uses DFS with discovery times and low values.
fn find_bridges<N>(graph: &HashMap<N, Vec<N>>) -> Vec<(N, N)>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let mut bridges = Vec::new();
    let mut discovery_time = HashMap::new();
    let mut low_time = HashMap::new();
    let mut time = 0;
    let mut visited = HashSet::new();

    for &node in graph.keys() {
        if !visited.contains(&node) {
            find_bridges_dfs(
                graph,
                node,
                node, // parent
                &mut visited,
                &mut discovery_time,
                &mut low_time,
                &mut time,
                &mut bridges,
            );
        }
    }

    bridges
}

#[allow(clippy::too_many_arguments)]
fn find_bridges_dfs<N>(
    graph: &HashMap<N, Vec<N>>,
    node: N,
    parent: N,
    visited: &mut HashSet<N>,
    discovery_time: &mut HashMap<N, usize>,
    low_time: &mut HashMap<N, usize>,
    time: &mut usize,
    bridges: &mut Vec<(N, N)>,
) where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    visited.insert(node);
    discovery_time.insert(node, *time);
    low_time.insert(node, *time);
    *time += 1;

    for &neighbor in graph.get(&node).unwrap_or(&Vec::new()) {
        if neighbor == parent {
            continue; // Skip parent
        }

        if !visited.contains(&neighbor) {
            find_bridges_dfs(
                graph,
                neighbor,
                node,
                visited,
                discovery_time,
                low_time,
                time,
                bridges,
            );

            // Update low time
            let current_low = low_time[&node];
            let neighbor_low = low_time[&neighbor];
            low_time.insert(node, current_low.min(neighbor_low));

            // Check if edge (node, neighbor) is a bridge
            if discovery_time[&node] < neighbor_low {
                bridges.push((node, neighbor));
            }
        } else {
            // Back edge - update low time
            let current_low = low_time[&node];
            let neighbor_discovery = discovery_time[&neighbor];
            low_time.insert(node, current_low.min(neighbor_discovery));
        }
    }
}

fn main() {
    println!("=== Day 3 Exercise Solutions ===");

    // Create sample graphs for testing
    let mut cyclic_graph: HashMap<u32, Vec<u32>> = HashMap::new();
    cyclic_graph.insert(0, vec![1, 2]);
    cyclic_graph.insert(1, vec![2, 3]);
    cyclic_graph.insert(2, vec![0, 3]); // Creates cycle 0->1->2->0
    cyclic_graph.insert(3, vec![4]);
    cyclic_graph.insert(4, vec![1]); // Creates another cycle

    let mut dag_graph: HashMap<u32, Vec<u32>> = HashMap::new();
    dag_graph.insert(0, vec![1, 2]);
    dag_graph.insert(1, vec![3]);
    dag_graph.insert(2, vec![3, 4]);
    dag_graph.insert(3, vec![5]);
    dag_graph.insert(4, vec![5]);
    dag_graph.insert(5, vec![]);

    let mut bridge_graph: HashMap<u32, Vec<u32>> = HashMap::new();
    bridge_graph.insert(0, vec![1]);
    bridge_graph.insert(1, vec![0, 2, 3]);
    bridge_graph.insert(2, vec![1]);
    bridge_graph.insert(3, vec![1, 4]);
    bridge_graph.insert(4, vec![3]);

    // Exercise 1: Find all cycles
    println!("\n=== Exercise 1: Find All Cycles ===");
    println!("Cyclic graph structure:");
    for (node, neighbors) in &cyclic_graph {
        println!("  {} -> {:?}", node, neighbors);
    }

    let all_cycles = find_all_cycles(&cyclic_graph);
    println!("Found {} cycles:", all_cycles.len());
    for (i, cycle) in all_cycles.iter().enumerate() {
        println!("  Cycle {}: {:?}", i + 1, cycle);
    }

    // Exercise 2: Find longest path in DAG
    println!("\n=== Exercise 2: Find Longest Path in DAG ===");
    println!("DAG structure:");
    for (node, neighbors) in &dag_graph {
        println!("  {} -> {:?}", node, neighbors);
    }

    if let Some(longest_path) = find_longest_path_in_dag(&dag_graph, 0, 5) {
        println!("Longest path from 0 to 5: {:?}", longest_path);
        println!("Path length: {}", longest_path.len() - 1);
    } else {
        println!("No path found or graph contains cycles");
    }

    // Exercise 3: Bidirectional BFS
    println!("\n=== Exercise 3: Bidirectional BFS ===");
    if let Some(bidirectional_path) = bidirectional_bfs(&dag_graph, 0, 5) {
        println!(
            "Bidirectional BFS path from 0 to 5: {:?}",
            bidirectional_path
        );
        println!("Path length: {}", bidirectional_path.len() - 1);
    } else {
        println!("No path found");
    }

    // Compare with regular BFS
    fn regular_bfs_path<N>(graph: &HashMap<N, Vec<N>>, start: N, end: N) -> Option<Vec<N>>
    where
        N: Copy + Eq + std::hash::Hash + Debug,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                let mut path = Vec::new();
                let mut node = end;
                while node != start {
                    path.push(node);
                    node = parent[&node];
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            for &neighbor in graph.get(&current).unwrap_or(&Vec::new()) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    parent.insert(neighbor, current);
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    if let Some(regular_path) = regular_bfs_path(&dag_graph, 0, 5) {
        println!("Regular BFS path from 0 to 5: {:?}", regular_path);
        println!("Path length: {}", regular_path.len() - 1);
    }

    // Exercise 4: Find bridges
    println!("\n=== Exercise 4: Find Bridges ===");
    println!("Bridge graph structure:");
    for (node, neighbors) in &bridge_graph {
        println!("  {} -> {:?}", node, neighbors);
    }

    let bridges = find_bridges(&bridge_graph);
    println!("Found {} bridges:", bridges.len());
    for (i, (from, to)) in bridges.iter().enumerate() {
        println!("  Bridge {}: {} -> {}", i + 1, from, to);
    }

    println!("\n🎉 All Day 3 exercises completed successfully!");
}
