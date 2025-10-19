use mission8::*;
use std::fmt;

/// Maze solver using BFS for shortest path finding
/// 
/// This example demonstrates:
/// - Converting 2D grid problem to graph representation
/// - Implementing Graph trait for custom types
/// - Using shortest_path algorithm from Mission8
/// - Visualizing solutions in 2D space
fn main() {
    println!("=== Mission 8: Maze Solver Application ===\n");
    
    // Solve multiple mazes of increasing complexity
    solve_maze("Simple 5×5 Maze", create_simple_maze());
    solve_maze("Medium 8×8 Maze", create_medium_maze());
    solve_maze("Large 12×12 Maze", create_large_maze());
    solve_maze("Unsolvable Maze", create_unsolvable_maze());
    
    println!("🎉 All maze solving demonstrations complete!");
    println!("\n💡 Key learnings:");
    println!("  • 2D grids can be modeled as graphs (cells = nodes, adjacency = edges)");
    println!("  • BFS guarantees shortest path in unweighted graphs");
    println!("  • Custom types can implement Graph trait for algorithm reuse");
    println!("  • Coordinate systems need careful bounds checking");
}

/// Solve a single maze and display results
fn solve_maze(name: &str, maze: Maze) {
    println!("🧩 Solving: {}", name);
    println!("📐 Size: {}×{}", maze.height, maze.width);
    
    // Show original maze
    println!("🗺️  Original maze:");
    println!("{}", maze);
    
    // Find start and end positions
    let start = maze.find_start().expect("Maze must have a start position");
    let end = maze.find_end().expect("Maze must have an end position");
    
    println!("🚀 Start: {:?}, End: {:?}", start, end);
    
    // Solve using BFS shortest path
    match shortest_path(&maze, start, end) {
        Ok(path) => {
            println!("✅ Solution found! Path length: {}", path.len() - 1);
            println!("📍 Path coordinates: {:?}", path);
            
            // Create and display solved maze
            let solved_maze = maze.with_solution_path(&path);
            println!("🎯 Maze with solution path (marked with *):");
            println!("{}", solved_maze);
            
            // Performance info
            let total_cells = maze.width * maze.height;
            let walkable_cells = maze.count_walkable_cells();
            println!("📊 Maze statistics:");
            println!("  • Total cells: {}", total_cells);
            println!("  • Walkable cells: {} ({:.1}%)", 
                     walkable_cells, 
                     walkable_cells as f32 / total_cells as f32 * 100.0);
            println!("  • Path efficiency: {:.1}% of walkable area", 
                     (path.len() - 1) as f32 / walkable_cells as f32 * 100.0);
        }
        Err(e) => {
            println!("❌ No solution found: {:?}", e);
            println!("💡 This maze has no path from start to end");
        }
    }
    
    println!("{}\n", "=".repeat(50));
}

/// Represents a cell in the maze
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,        // # - Impassable wall
    Path,        // . - Walkable path
    Start,       // S - Starting position  
    End,         // E - End/goal position
    Solution,    // * - Part of solution path (for visualization)
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Wall => '#',
            Cell::Path => '.',
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Solution => '*',
        };
        write!(f, "{}", symbol)
    }
}

/// 2D maze structure
#[derive(Debug, Clone)]
pub struct Maze {
    pub grid: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    /// Create new maze from 2D grid
    pub fn new(grid: Vec<Vec<Cell>>) -> Self {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        
        Self { grid, width, height }
    }
    
    /// Find the start position (S) in the maze
    pub fn find_start(&self) -> Option<(usize, usize)> {
        for (row, cells) in self.grid.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if cell == Cell::Start {
                    return Some((row, col));
                }
            }
        }
        None
    }
    
    /// Find the end position (E) in the maze
    pub fn find_end(&self) -> Option<(usize, usize)> {
        for (row, cells) in self.grid.iter().enumerate() {
            for (col, &cell) in cells.iter().enumerate() {
                if cell == Cell::End {
                    return Some((row, col));
                }
            }
        }
        None
    }
    
    /// Count walkable cells (Path, Start, End)
    pub fn count_walkable_cells(&self) -> usize {
        self.grid.iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| matches!(cell, Cell::Path | Cell::Start | Cell::End))
            .count()
    }
    
    /// Create new maze with solution path marked
    pub fn with_solution_path(&self, path: &[(usize, usize)]) -> Self {
        let mut new_maze = self.clone();
        
        // Mark path cells (but preserve Start and End markers)
        for &(row, col) in path {
            if new_maze.grid[row][col] == Cell::Path {
                new_maze.grid[row][col] = Cell::Solution;
                // Keep Start and End markers as-is
            }
        }
        
        new_maze
    }
    
    /// Get valid neighbors for a position (4-connectivity: up, down, left, right)
    fn get_neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (row, col) = pos;
        let mut neighbors = Vec::new();
        
        // Check all 4 directions (up, down, left, right)
        let directions = [
            (-1, 0), // up
            (1, 0),  // down
            (0, -1), // left
            (0, 1),  // right
        ];
        
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            
            // Check bounds
            if new_row >= 0 && new_row < self.height as i32 &&
               new_col >= 0 && new_col < self.width as i32 {
                let new_pos = (new_row as usize, new_col as usize);
                
                // Check if cell is walkable
                if self.is_walkable(new_pos) {
                    neighbors.push(new_pos);
                }
            }
        }
        
        neighbors
    }
    
    /// Check if a position is walkable (not a wall)
    fn is_walkable(&self, (row, col): (usize, usize)) -> bool {
        matches!(self.grid[row][col], Cell::Path | Cell::Start | Cell::End)
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Implement Graph trait for Maze to enable BFS/DFS algorithms
impl Graph for Maze {
    type Node = (usize, usize);
    
    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        self.get_neighbors(node)
    }
    
    fn contains(&self, (row, col): Self::Node) -> bool {
        row < self.height && col < self.width
    }
    
    fn nodes(&self) -> Vec<Self::Node> {
        let mut nodes = Vec::new();
        for row in 0..self.height {
            for col in 0..self.width {
                if self.is_walkable((row, col)) {
                    nodes.push((row, col));
                }
            }
        }
        nodes
    }
}

// Maze creation functions

fn create_simple_maze() -> Maze {
    use Cell::*;
    
    let grid = vec![
        vec![Wall, Wall, Wall, Wall, Wall],
        vec![Wall, Start, Path, Path, Wall],  
        vec![Wall, Wall, Wall, Path, Wall],
        vec![Wall, Path, Path, Path, Wall],
        vec![Wall, Wall, Wall, End, Wall],
    ];
    
    Maze::new(grid)
}

fn create_medium_maze() -> Maze {
    use Cell::*;
    
    let grid = vec![
        vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
        vec![Wall, Start, Path, Wall, Path, Path, Path, Wall],
        vec![Wall, Path, Path, Wall, Path, Wall, Path, Wall],
        vec![Wall, Path, Wall, Wall, Path, Wall, Path, Wall],
        vec![Wall, Path, Path, Path, Path, Wall, Path, Wall],
        vec![Wall, Wall, Wall, Path, Wall, Wall, Path, Wall],
        vec![Wall, Path, Path, Path, Path, Path, Path, Wall],
        vec![Wall, Wall, Wall, Wall, Wall, Wall, End, Wall],
    ];
    
    Maze::new(grid)
}

fn create_large_maze() -> Maze {
    use Cell::*;
    
    let grid = vec![
        vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
        vec![Wall, Start, Path, Path, Wall, Path, Path, Path, Path, Path, Path, Wall],
        vec![Wall, Wall, Wall, Path, Wall, Path, Wall, Wall, Wall, Wall, Path, Wall],
        vec![Wall, Path, Path, Path, Wall, Path, Wall, Path, Path, Path, Path, Wall],
        vec![Wall, Path, Wall, Wall, Wall, Path, Wall, Path, Wall, Wall, Wall, Wall],
        vec![Wall, Path, Path, Path, Path, Path, Wall, Path, Path, Path, Path, Wall],
        vec![Wall, Wall, Wall, Wall, Wall, Path, Wall, Wall, Wall, Path, Wall, Wall],
        vec![Wall, Path, Path, Path, Path, Path, Path, Path, Path, Path, Path, Wall],
        vec![Wall, Path, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Path, Wall],
        vec![Wall, Path, Path, Path, Path, Path, Path, Path, Path, Wall, Path, Wall],
        vec![Wall, Wall, Wall, Wall, Wall, Path, Wall, Wall, Path, Path, End, Wall],
        vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
    ];
    
    Maze::new(grid)
}

fn create_unsolvable_maze() -> Maze {
    use Cell::*;
    
    let grid = vec![
        vec![Wall, Wall, Wall, Wall, Wall],
        vec![Wall, Start, Path, Wall, Wall],
        vec![Wall, Path, Wall, Wall, Wall], 
        vec![Wall, Wall, Wall, End, Wall],   // End is isolated!
        vec![Wall, Wall, Wall, Wall, Wall],
    ];
    
    Maze::new(grid)
}