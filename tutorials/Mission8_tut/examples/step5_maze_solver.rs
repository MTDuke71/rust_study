//! Step 5: Maze Solver Application
//! 
//! This tutorial demonstrates how to build a complete real-world application
//! using BFS/DFS algorithms by solving mazes as graph problems.

/// Graph trait that our algorithms expect
pub trait Graph<T> {
    fn neighbors(&self, node: &T) -> Vec<T>;
}

/// A cell in a maze can be one of these types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Path,
    Start,
    End,
}

/// Maze structure representing a 2D grid
#[derive(Debug)]
pub struct Maze {
    grid: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
}

impl Maze {
    /// Create a new maze from string representation
    pub fn from_string(maze_str: &str) -> Self {
        let lines: Vec<&str> = maze_str.trim().split('\n').collect();
        let rows = lines.len();
        let cols = lines[0].len();
        
        let mut grid = Vec::with_capacity(rows);
        let mut start = None;
        let mut end = None;
        
        for (row, line) in lines.iter().enumerate() {
            let mut row_cells = Vec::with_capacity(cols);
            for (col, ch) in line.chars().enumerate() {
                let cell = match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Path,
                    'S' => {
                        start = Some((row, col));
                        Cell::Start
                    },
                    'E' => {
                        end = Some((row, col));
                        Cell::End
                    },
                    _ => Cell::Path, // Default to path for unknown characters
                };
                row_cells.push(cell);
            }
            grid.push(row_cells);
        }
        
        Self { grid, rows, cols, start, end }
    }
    
    /// Get the start position
    pub fn start(&self) -> Option<(usize, usize)> {
        self.start
    }
    
    /// Get the end position
    pub fn end(&self) -> Option<(usize, usize)> {
        self.end
    }
    
    /// Check if a position is valid and walkable
    fn is_walkable(&self, row: usize, col: usize) -> bool {
        if row >= self.rows || col >= self.cols {
            return false;
        }
        
        !matches!(self.grid[row][col], Cell::Wall)
    }
    
    /// Display the maze with optional solution path
    pub fn display_with_path(&self, path: Option<&Vec<(usize, usize)>>) {
        let mut display_grid = self.grid.clone();
        
        // Mark the path if provided
        if let Some(path_vec) = path {
            for &(row, col) in path_vec {
                if self.grid[row][col] == Cell::Path {
                    display_grid[row][col] = Cell::Path; // We'll use * to show path
                }
            }
        }
        
        println!("Maze ({}×{}):", self.rows, self.cols);
        for (row_idx, row) in display_grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                let ch = if let Some(path_vec) = path {
                    if path_vec.contains(&(row_idx, col_idx)) && 
                       cell != Cell::Start && cell != Cell::End {
                        '*'
                    } else {
                        match cell {
                            Cell::Wall => '#',
                            Cell::Path => '.',
                            Cell::Start => 'S',
                            Cell::End => 'E',
                        }
                    }
                } else {
                    match cell {
                        Cell::Wall => '#',
                        Cell::Path => '.',
                        Cell::Start => 'S',
                        Cell::End => 'E',
                    }
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

/// Implement Graph trait for Maze using 4-connectivity
impl Graph<(usize, usize)> for Maze {
    fn neighbors(&self, &(row, col): &(usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        
        // Check all 4 directions: up, down, left, right
        let directions = [
            (-1i32, 0i32),  // up
            (1, 0),         // down
            (0, -1),        // left
            (0, 1),         // right
        ];
        
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            
            // Check bounds
            if new_row >= 0 && new_col >= 0 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                if self.is_walkable(new_row, new_col) {
                    neighbors.push((new_row, new_col));
                }
            }
        }
        
        neighbors
    }
}

/// BFS shortest path implementation
fn shortest_path<T, G>(
    graph: &G,
    start: T,
    end: T,
) -> Option<Vec<T>>
where
    T: Clone + std::cmp::Eq + std::hash::Hash,
    G: Graph<T>,
{
    use std::collections::{HashMap, VecDeque};
    
    if start == end {
        return Some(vec![start]);
    }
    
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    let mut parents: HashMap<T, T> = HashMap::new();
    
    queue.push_back(start.clone());
    visited.insert(start.clone());
    
    while let Some(current) = queue.pop_front() {
        for neighbor in graph.neighbors(&current) {
            if neighbor == end {
                // Found the target! Reconstruct path
                let mut path = vec![neighbor.clone()];
                let mut current_node = current;
                
                while let Some(parent) = parents.get(&current_node) {
                    path.push(current_node.clone());
                    current_node = parent.clone();
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }
            
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                parents.insert(neighbor.clone(), current.clone());
                queue.push_back(neighbor);
            }
        }
    }
    
    None // No path found
}

fn main() {
    println!("=== Mission 8 Tutorial: Step 5 - Maze Solver Application ===\n");
    
    // Section 1: Understanding the Problem
    println!("📋 Section 1: Understanding the Problem");
    println!("----------------------------------------");
    println!("A maze is a perfect example of a graph problem:");
    println!("• Each walkable cell is a node");
    println!("• Connections between adjacent cells are edges");
    println!("• We want to find the shortest path from Start to End");
    println!("• BFS guarantees the shortest path in unweighted graphs\n");
    
    // Section 2: Simple 5×5 Maze
    println!("🏗️ Section 2: Solving a Simple 5×5 Maze");
    println!("-----------------------------------------");
    
    let simple_maze_str = r#"
#####
#S..#
#.#.#
#...#
###E#"#;
    
    let simple_maze = Maze::from_string(simple_maze_str);
    println!("Original maze:");
    simple_maze.display_with_path(None);
    
    if let (Some(start), Some(end)) = (simple_maze.start(), simple_maze.end()) {
        println!("\nStart: {:?}, End: {:?}", start, end);
        
        match shortest_path(&simple_maze, start, end) {
            Some(path) => {
                println!("✅ Path found! Length: {}", path.len() - 1);
                println!("Path coordinates: {:?}", path);
                println!("\nMaze with solution path (marked with *):");
                simple_maze.display_with_path(Some(&path));
            }
            None => {
                println!("❌ No path exists!");
                simple_maze.display_with_path(None);
            }
        }
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Section 3: Medium 8×8 Maze
    println!("🎯 Section 3: Solving a Medium 8×8 Maze");
    println!("----------------------------------------");
    
    let medium_maze_str = r#"
########
#S.....#
#.####.#
#....#.#
####.#.#
#....#.#
#.####.#
#.....E#
########"#;
    
    let medium_maze = Maze::from_string(medium_maze_str);
    println!("Medium maze:");
    medium_maze.display_with_path(None);
    
    if let (Some(start), Some(end)) = (medium_maze.start(), medium_maze.end()) {
        println!("\nSolving medium maze...");
        let start_time = std::time::Instant::now();
        
        match shortest_path(&medium_maze, start, end) {
            Some(path) => {
                let elapsed = start_time.elapsed();
                println!("✅ Path found! Length: {}, Time: {:?}", path.len() - 1, elapsed);
                println!("\nMaze with solution path:");
                medium_maze.display_with_path(Some(&path));
            }
            None => {
                println!("❌ No path exists!");
            }
        }
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Section 4: Unsolvable Maze
    println!("❌ Section 4: Handling Unsolvable Mazes");
    println!("---------------------------------------");
    
    let unsolvable_maze_str = r#"
#######
#S....#
#.####.
#.....#
#######
#.....#
#....E#
#######"#;
    
    let unsolvable_maze = Maze::from_string(unsolvable_maze_str);
    println!("Unsolvable maze (wall blocks the path):");
    unsolvable_maze.display_with_path(None);
    
    if let (Some(start), Some(end)) = (unsolvable_maze.start(), unsolvable_maze.end()) {
        println!("\nAttempting to solve...");
        
        match shortest_path(&unsolvable_maze, start, end) {
            Some(path) => {
                println!("✅ Path found! Length: {}", path.len() - 1);
                unsolvable_maze.display_with_path(Some(&path));
            }
            None => {
                println!("❌ No path exists! The maze is unsolvable.");
                println!("This demonstrates proper error handling in real applications.");
            }
        }
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Section 5: Larger Maze Performance
    println!("⚡ Section 5: Performance on Larger Mazes");
    println!("-----------------------------------------");
    
    let large_maze_str = r#"
################
#S............##
#.############.#
#..........#...#
##########.###.#
#..........#...#
#.##########.###
#............#.#
#############..#
#..............#
#.############.#
#..........#...#
##########.#.#.#
#..........#...#
#.###########.##
#.............E#
################"#;
    
    let large_maze = Maze::from_string(large_maze_str);
    println!("Large maze ({}×{}):", 
             large_maze_str.lines().count() - 1,
             large_maze_str.lines().nth(1).unwrap().len());
    
    if let (Some(start), Some(end)) = (large_maze.start(), large_maze.end()) {
        println!("Solving large maze...");
        let start_time = std::time::Instant::now();
        
        match shortest_path(&large_maze, start, end) {
            Some(path) => {
                let elapsed = start_time.elapsed();
                println!("✅ Path found! Length: {}, Time: {:?}", path.len() - 1, elapsed);
                println!("Path efficiency: {:.1}% of theoretical minimum", 
                         (path.len() as f64 / ((end.0.abs_diff(start.0) + end.1.abs_diff(start.1)) as f64 + 1.0)) * 100.0);
                
                // Don't display the full maze due to size, but show performance
                println!("✨ BFS efficiently found the shortest path in a complex maze!");
            }
            None => {
                let elapsed = start_time.elapsed();
                println!("❌ No path exists! Determined in {:?}", elapsed);
            }
        }
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Section 6: Multiple Mazes Batch Processing
    println!("🔄 Section 6: Batch Processing Multiple Mazes");
    println!("---------------------------------------------");
    
    let test_mazes = vec![
        ("Tiny", "#S#\n#.#\n#E#"),
        ("Simple", "SE"),
        ("Blocked", "S#E"),
        ("Corridor", "S...E"),
    ];
    
    for (name, maze_str) in test_mazes {
        println!("Testing {} maze: '{}'", name, maze_str);
        let maze = Maze::from_string(maze_str);
        
        if let (Some(start), Some(end)) = (maze.start(), maze.end()) {
            match shortest_path(&maze, start, end) {
                Some(path) => {
                    println!("  ✅ Solvable! Path length: {}", path.len() - 1);
                }
                None => {
                    println!("  ❌ Unsolvable!");
                }
            }
        } else {
            println!("  ⚠️  Invalid maze (missing start or end)!");
        }
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Section 7: Real-World Applications
    println!("🌍 Section 7: Real-World Applications");
    println!("------------------------------------");
    println!("This maze solver demonstrates patterns used in:");
    println!("• 🎮 Game pathfinding (NPCs finding players)");
    println!("• 🗺️  GPS navigation (shortest route finding)"); 
    println!("• 🤖 Robot navigation (autonomous movement)");
    println!("• 🧩 Puzzle solving (escape room games)");
    println!("• 📱 App navigation (UI flow optimization)");
    println!();
    println!("Key techniques learned:");
    println!("✓ Converting real problems to graph representation");
    println!("✓ Implementing Graph trait for custom data structures");
    println!("✓ Using BFS for guaranteed shortest paths");
    println!("✓ Handling edge cases (no path exists)");
    println!("✓ Performance measurement and optimization");
    println!("✓ Batch processing and error handling");
    
    println!("\n🎓 Tutorial Complete! You've built a production-ready maze solver!");
    println!("Next: Step 6 - Integration Testing strategies");
}