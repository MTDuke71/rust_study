//! Day 2 Exercise Solutions: Generic BFS/DFS Implementation
//!
//! This file contains solutions to the Day 2 exercises from the Mission 8 tutorial.
//! Each exercise builds upon the basic BFS/DFS concepts to solve more complex problems.
//!
//! Run with: cargo run --example day2_exercises_solutions

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

/// Exercise 1: Modify BFS to track levels
/// 
/// Extend the basic BFS to also track which level each node is at.
/// This is useful for applications like finding nodes at specific distances.

fn bfs_with_levels<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<(N, usize)>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    // Queue stores (node, level)
    queue.push_back((start, 0));
    visited.insert(start);
    
    while let Some((current, level)) = queue.pop_front() {
        result.push((current, level));
        
        for &neighbor in graph.get(&current).unwrap_or(&Vec::new()) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, level + 1));
            }
        }
    }
    
    result
}

/// Exercise 2: Implement recursive DFS and compare with iterative
/// 
/// This shows the difference between recursive and iterative DFS implementations.
/// Recursive DFS is more natural but can cause stack overflow on deep graphs.

fn dfs_recursive<N>(
    graph: &HashMap<N, Vec<N>>,
    node: N,
    visited: &mut HashSet<N>,
    result: &mut Vec<N>,
) where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    if visited.contains(&node) {
        return;
    }
    
    visited.insert(node);
    result.push(node);
    
    for &neighbor in graph.get(&node).unwrap_or(&Vec::new()) {
        dfs_recursive(graph, neighbor, visited, result);
    }
}

fn dfs_recursive_wrapper<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    
    dfs_recursive(graph, start, &mut visited, &mut result);
    result
}

/// Exercise 3: Find all paths between two nodes
/// 
/// This is more complex than shortest path - we need to find ALL possible paths.
/// Uses DFS with backtracking to explore all routes.

fn find_all_paths<N>(
    graph: &HashMap<N, Vec<N>>,
    start: N,
    end: N,
) -> Vec<Vec<N>>
where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    let mut all_paths = Vec::new();
    let mut current_path = Vec::new();
    let mut visited = HashSet::new();
    
    find_all_paths_helper(graph, start, end, &mut visited, &mut current_path, &mut all_paths);
    all_paths
}

fn find_all_paths_helper<N>(
    graph: &HashMap<N, Vec<N>>,
    current: N,
    end: N,
    visited: &mut HashSet<N>,
    current_path: &mut Vec<N>,
    all_paths: &mut Vec<Vec<N>>,
) where
    N: Copy + Eq + std::hash::Hash + Debug,
{
    current_path.push(current);
    
    if current == end {
        all_paths.push(current_path.clone());
    } else {
        visited.insert(current);
        
        for &neighbor in graph.get(&current).unwrap_or(&Vec::new()) {
            if !visited.contains(&neighbor) {
                find_all_paths_helper(graph, neighbor, end, visited, current_path, all_paths);
            }
        }
        
        visited.remove(&current);
    }
    
    current_path.pop();
}

/// Exercise 4: Implement BFS on grid (2D array as graph)
/// 
/// This shows how to represent a 2D grid as a graph and run BFS on it.
/// Useful for problems like finding shortest path in a maze.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPosition {
    row: usize,
    col: usize,
}

impl GridPosition {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    
    fn neighbors(&self, rows: usize, cols: usize) -> Vec<GridPosition> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up
        
        for (dr, dc) in directions {
            let new_row = self.row as i32 + dr;
            let new_col = self.col as i32 + dc;
            
            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                neighbors.push(GridPosition::new(new_row as usize, new_col as usize));
            }
        }
        
        neighbors
    }
}

fn bfs_on_grid(
    grid: &Vec<Vec<bool>>, // true = walkable, false = wall
    start: GridPosition,
    end: GridPosition,
) -> Option<Vec<GridPosition>> {
    let rows = grid.len();
    let cols = grid[0].len();
    
    if !grid[start.row][start.col] || !grid[end.row][end.col] {
        return None; // Start or end is a wall
    }
    
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent = HashMap::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        if current == end {
            // Reconstruct path
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
        
        for neighbor in current.neighbors(rows, cols) {
            if !visited.contains(&neighbor) && grid[neighbor.row][neighbor.col] {
                visited.insert(neighbor);
                parent.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }
    
    None // No path found
}

fn main() {
    println!("=== Day 2 Exercise Solutions ===");
    
    // Create a sample graph for testing
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3, 4]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![5]);
    graph.insert(4, vec![5]);
    graph.insert(5, vec![]);
    
    println!("Sample graph structure:");
    for (node, neighbors) in &graph {
        println!("  {} -> {:?}", node, neighbors);
    }
    
    // Exercise 1: BFS with levels
    println!("\n=== Exercise 1: BFS with Levels ===");
    let bfs_with_levels_result = bfs_with_levels(&graph, 0);
    println!("BFS with levels from node 0:");
    for (node, level) in bfs_with_levels_result {
        println!("  Node {} at level {}", node, level);
    }
    
    // Exercise 2: Recursive vs Iterative DFS
    println!("\n=== Exercise 2: Recursive vs Iterative DFS ===");
    
    // First, let's implement iterative DFS for comparison
    fn dfs_iterative<N>(graph: &HashMap<N, Vec<N>>, start: N) -> Vec<N>
    where
        N: Copy + Eq + std::hash::Hash + Debug,
    {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        
        while let Some(current) = stack.pop() {
            if !visited.contains(&current) {
                visited.insert(current);
                result.push(current);
                
                for &neighbor in graph.get(&current).unwrap_or(&Vec::new()) {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
        result
    }
    
    let recursive_result = dfs_recursive_wrapper(&graph, 0);
    let iterative_result = dfs_iterative(&graph, 0);
    
    println!("Recursive DFS: {:?}", recursive_result);
    println!("Iterative DFS: {:?}", iterative_result);
    println!("Results match: {}", recursive_result == iterative_result);
    
    // Exercise 3: Find all paths
    println!("\n=== Exercise 3: Find All Paths ===");
    let all_paths = find_all_paths(&graph, 0, 5);
    println!("All paths from 0 to 5:");
    for (i, path) in all_paths.iter().enumerate() {
        println!("  Path {}: {:?}", i + 1, path);
    }
    
    // Exercise 4: BFS on grid
    println!("\n=== Exercise 4: BFS on Grid ===");
    
    // Create a simple 3x3 grid with some walls
    let grid = vec![
        vec![true,  true,  false], // Row 0: start at (0,0), wall at (0,2)
        vec![false, true,  true ], // Row 1: wall at (1,0), walkable path
        vec![true,  true,  true ], // Row 2: all walkable, end at (2,2)
    ];
    
    let start = GridPosition::new(0, 0);
    let end = GridPosition::new(2, 2);
    
    println!("Grid layout (T=walkable, F=wall):");
    for (i, row) in grid.iter().enumerate() {
        print!("  Row {}: ", i);
        for &cell in row {
            print!("{} ", if cell { "T" } else { "F" });
        }
        println!();
    }
    
    println!("Start: {:?}, End: {:?}", start, end);
    
    if let Some(path) = bfs_on_grid(&grid, start, end) {
        println!("Shortest path found with {} steps:", path.len() - 1);
        for (i, pos) in path.iter().enumerate() {
            println!("  Step {}: {:?}", i, pos);
        }
    } else {
        println!("No path found from start to end!");
    }
    
    println!("\n🎉 All Day 2 exercises completed successfully!");
}
