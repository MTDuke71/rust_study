//! Day 5 Exercise Solutions: Advanced Maze Solving Techniques
//! 
//! This file contains solutions to the Step 5 exercises, demonstrating
//! advanced applications of BFS/DFS algorithms in maze solving contexts.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

/// A cell in a maze can be one of these types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Path,
    Start,
    End,
    Obstacle(u32), // Exercise 4: Removable obstacles with cost
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Cell::Wall => '#',
            Cell::Path => '.',
            Cell::Start => 'S',
            Cell::End => 'E',
            Cell::Obstacle(cost) => {
                if *cost <= 9 {
                    char::from_digit(*cost, 10).unwrap_or('O')
                } else {
                    'O'
                }
            }
        };
        write!(f, "{}", ch)
    }
}

/// Graph trait that our algorithms expect
pub trait Graph<T> {
    fn neighbors(&self, node: &T) -> Vec<T>;
}

/// Enhanced maze structure with support for multiple starts and weighted obstacles
#[derive(Debug)]
pub struct EnhancedMaze {
    grid: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
    starts: Vec<(usize, usize)>,
    ends: Vec<(usize, usize)>,
}

impl EnhancedMaze {
    /// Create a new maze from string representation
    pub fn from_string(maze_str: &str) -> Self {
        let lines: Vec<&str> = maze_str.trim().split('\n').collect();
        let rows = lines.len();
        let cols = if rows > 0 { lines[0].len() } else { 0 };
        
        let mut grid = Vec::with_capacity(rows);
        let mut starts = Vec::new();
        let mut ends = Vec::new();
        
        for (row, line) in lines.iter().enumerate() {
            let mut row_cells = Vec::with_capacity(cols);
            for (col, ch) in line.chars().enumerate() {
                let cell = match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Path,
                    'S' => {
                        starts.push((row, col));
                        Cell::Start
                    },
                    'E' => {
                        ends.push((row, col));
                        Cell::End
                    },
                    '1'..='9' => {
                        if let Some(cost) = ch.to_digit(10) {
                            Cell::Obstacle(cost)
                        } else {
                            Cell::Path
                        }
                    },
                    _ => Cell::Path,
                };
                row_cells.push(cell);
            }
            grid.push(row_cells);
        }
        
        Self { grid, rows, cols, starts, ends }
    }
    
    /// Get all start positions
    pub fn starts(&self) -> &[(usize, usize)] {
        &self.starts
    }
    
    /// Get all end positions
    pub fn ends(&self) -> &[(usize, usize)] {
        &self.ends
    }
    
    /// Check if a position is valid and walkable (considering obstacle removal cost)
    fn is_walkable(&self, row: usize, col: usize, max_obstacle_cost: Option<u32>) -> bool {
        if row >= self.rows || col >= self.cols {
            return false;
        }
        
        match self.grid[row][col] {
            Cell::Wall => false,
            Cell::Path | Cell::Start | Cell::End => true,
            Cell::Obstacle(cost) => {
                if let Some(max_cost) = max_obstacle_cost {
                    cost <= max_cost
                } else {
                    false // Obstacles block path if no removal budget
                }
            }
        }
    }
    
    /// Display the maze with optional solution paths
    pub fn display_with_paths(&self, paths: Option<&[Vec<(usize, usize)>]>) {
        let mut display_grid = self.grid.clone();
        
        // Mark paths with different symbols
        if let Some(path_list) = paths {
            for path in path_list.iter() {
                for &(row, col) in path {
                    if self.grid[row][col] == Cell::Path {
                        // Use a simple character replacement for visualization
                        display_grid[row][col] = Cell::Path; // Keep as path, we'll print symbol
                    }
                }
            }
        }
        
        println!("Enhanced Maze ({}×{}):", self.rows, self.cols);
        for (row_idx, row) in display_grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                let ch = if let Some(path_list) = paths {
                    let mut found_path = false;
                    let mut symbol = '.';
                    
                    for (i, path) in path_list.iter().enumerate() {
                        if path.contains(&(row_idx, col_idx)) && 
                           cell != Cell::Start && cell != Cell::End {
                            let path_symbols = ['*', '+', 'x', 'o', '~', '%', '&', '@'];
                            symbol = if i < path_symbols.len() { 
                                path_symbols[i] 
                            } else { 
                                '*' 
                            };
                            found_path = true;
                            break;
                        }
                    }
                    
                    if found_path {
                        symbol
                    } else {
                        format!("{}", cell).chars().next().unwrap_or('.')
                    }
                } else {
                    format!("{}", cell).chars().next().unwrap_or('.')
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

/// Exercise 4: Enhanced Graph implementation with obstacle costs
impl Graph<(usize, usize)> for EnhancedMaze {
    fn neighbors(&self, &(row, col): &(usize, usize)) -> Vec<(usize, usize)> {
        self.neighbors_with_cost((row, col), None)
    }
}

impl EnhancedMaze {
    /// Get neighbors considering obstacle removal cost
    pub fn neighbors_with_cost(&self, (row, col): (usize, usize), max_obstacle_cost: Option<u32>) -> Vec<(usize, usize)> {
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
            
            if new_row >= 0 && new_col >= 0 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                if self.is_walkable(new_row, new_col, max_obstacle_cost) {
                    neighbors.push((new_row, new_col));
                }
            }
        }
        
        neighbors
    }
}

/// Type alias for multi-start pathfinding results
type MultiPathResult = HashMap<((usize, usize), (usize, usize)), Vec<(usize, usize)>>;

/// Exercise 1: Multi-start BFS pathfinding
pub fn multi_start_shortest_path(
    maze: &EnhancedMaze,
    targets: &[(usize, usize)],
) -> MultiPathResult {
    let mut results = HashMap::new();
    
    for &start in maze.starts() {
        for &target in targets {
            if let Some(path) = shortest_path_basic(maze, start, target) {
                results.insert((start, target), path);
            }
        }
    }
    
    results
}

/// Exercise 2: Find all shortest paths between two points
pub fn find_all_shortest_paths(
    maze: &EnhancedMaze,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<(usize, usize)>> {
    if start == end {
        return vec![vec![start]];
    }
    
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut shortest_length = None;
    
    queue.push_back(vec![start]);
    visited.insert(start, 0);
    
    while let Some(current_path) = queue.pop_front() {
        let current_pos = *current_path.last().unwrap();
        let current_length = current_path.len();
        
        // If we found a longer path than the shortest, stop exploring
        if let Some(min_len) = shortest_length {
            if current_length > min_len {
                continue;
            }
        }
        
        if current_pos == end {
            match shortest_length {
                None => {
                    shortest_length = Some(current_length);
                    paths.push(current_path);
                }
                Some(len) if current_length == len => {
                    paths.push(current_path);
                }
                _ => {}
            }
            continue;
        }
        
        for neighbor in maze.neighbors(&current_pos) {
            // Only continue if we haven't visited this node or we're at the same distance
            if let Some(&prev_dist) = visited.get(&neighbor) {
                if current_length + 1 > prev_dist {
                    continue;
                }
            }
            
            if !current_path.contains(&neighbor) {
                let mut new_path = current_path.clone();
                new_path.push(neighbor);
                queue.push_back(new_path);
                visited.insert(neighbor, current_length + 1);
            }
        }
    }
    
    paths
}

/// Exercise 3: Generate random mazes
pub fn generate_random_maze(
    rows: usize, 
    cols: usize, 
    wall_probability: f64,
    num_starts: usize,
    num_ends: usize,
) -> EnhancedMaze {
    use std::collections::HashSet;
    
    // Simple random maze generation
    let mut grid = vec![vec![Cell::Wall; cols]; rows];
    let mut rng_state = 12345u64; // Simple PRNG
    
    // Simple linear congruential generator
    let mut next_random = || {
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        (rng_state / 65536) % 32768
    };
    
    // Generate random paths
    #[allow(clippy::needless_range_loop)]
    for row in 0..rows {
        for col in 0..cols {
            let random_val = next_random() as f64 / 32768.0;
            if random_val > wall_probability {
                grid[row][col] = Cell::Path;
            }
        }
    }
    
    // Ensure borders are walls (except for designated openings)
    for col in 0..cols {
        grid[0][col] = Cell::Wall;
        grid[rows - 1][col] = Cell::Wall;
    }
    for row_data in grid.iter_mut().take(rows) {
        row_data[0] = Cell::Wall;
        row_data[cols - 1] = Cell::Wall;
    }
    
    // Place starts and ends randomly
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    let mut placed_positions = HashSet::new();
    
    // Place starts
    let mut attempts = 0;
    while starts.len() < num_starts && attempts < 100 {
        let row = (next_random() as usize % (rows - 2)) + 1;
        let col = (next_random() as usize % (cols - 2)) + 1;
        
        if !placed_positions.contains(&(row, col)) {
            grid[row][col] = Cell::Start;
            starts.push((row, col));
            placed_positions.insert((row, col));
        }
        attempts += 1;
    }
    
    // Place ends
    attempts = 0;
    while ends.len() < num_ends && attempts < 100 {
        let row = (next_random() as usize % (rows - 2)) + 1;
        let col = (next_random() as usize % (cols - 2)) + 1;
        
        if !placed_positions.contains(&(row, col)) {
            grid[row][col] = Cell::End;
            ends.push((row, col));
            placed_positions.insert((row, col));
        }
        attempts += 1;
    }
    
    EnhancedMaze { grid, rows, cols, starts, ends }
}

/// Exercise 4: Pathfinding with removable obstacles
pub fn shortest_path_with_obstacles(
    maze: &EnhancedMaze,
    start: (usize, usize),
    end: (usize, usize),
    max_removal_cost: u32,
) -> Option<(Vec<(usize, usize)>, u32)> {
    #[derive(Debug, Clone)]
    struct State {
        position: (usize, usize),
        path: Vec<(usize, usize)>,
        cost_used: u32,
    }
    
    if start == end {
        return Some((vec![start], 0));
    }
    
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    
    queue.push_back(State {
        position: start,
        path: vec![start],
        cost_used: 0,
    });
    visited.insert((start, 0), true);
    
    while let Some(current) = queue.pop_front() {
        if current.position == end {
            return Some((current.path, current.cost_used));
        }
        
        // Get all possible neighbors (including through obstacles)
        let directions = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
        
        for (dr, dc) in directions {
            let new_row = current.position.0 as i32 + dr;
            let new_col = current.position.1 as i32 + dc;
            
            if new_row >= 0 && new_col >= 0 {
                let new_pos = (new_row as usize, new_col as usize);
                
                if new_pos.0 < maze.rows && new_pos.1 < maze.cols {
                    let cell = maze.grid[new_pos.0][new_pos.1];
                    let mut new_cost = current.cost_used;
                    let mut can_move = false;
                    
                    match cell {
                        Cell::Wall => continue, // Can't move through walls
                        Cell::Path | Cell::Start | Cell::End => can_move = true,
                        Cell::Obstacle(cost) => {
                            if current.cost_used + cost <= max_removal_cost {
                                new_cost = current.cost_used + cost;
                                can_move = true;
                            }
                        }
                    }
                    
                    if can_move && !current.path.contains(&new_pos) {
                        let state_key = (new_pos, new_cost);
                        if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(state_key) {
                            e.insert(true);
                            
                            let mut new_path = current.path.clone();
                            new_path.push(new_pos);
                            
                            queue.push_back(State {
                                position: new_pos,
                                path: new_path,
                                cost_used: new_cost,
                            });
                        }
                    }
                }
            }
        }
    }
    
    None
}

/// Basic shortest path implementation for Exercise 1
fn shortest_path_basic(
    maze: &EnhancedMaze,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    if start == end {
        return Some(vec![start]);
    }
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        for neighbor in maze.neighbors(&current) {
            if neighbor == end {
                // Found the target! Reconstruct path
                let mut path = vec![neighbor];
                let mut current_node = current;
                
                while let Some(parent) = parents.get(&current_node) {
                    path.push(current_node);
                    current_node = *parent;
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }
            
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parents.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }
    
    None
}

fn main() {
    println!("=== Mission 8 Tutorial: Day 5 Exercise Solutions ===\n");
    
    // Exercise 1: Multiple Start Points
    println!("🚀 Exercise 1: Multiple Start Points");
    println!("------------------------------------");
    
    let multi_start_maze_str = r#"
#########
#S......#
#.######.
#.......#
#.###.#E#
#S....#.#
#######.#
#.....E.#
#########"#;
    
    let multi_maze = EnhancedMaze::from_string(multi_start_maze_str);
    println!("Maze with multiple starts and ends:");
    multi_maze.display_with_paths(None);
    
    let all_paths = multi_start_shortest_path(&multi_maze, multi_maze.ends());
    println!("\n📍 Shortest paths from each start to each end:");
    for ((start, end), path) in &all_paths {
        println!("  Start {:?} → End {:?}: {} steps", start, end, path.len() - 1);
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Exercise 2: All Shortest Paths
    println!("🔍 Exercise 2: Find All Shortest Paths");
    println!("--------------------------------------");
    
    let multi_path_maze_str = r#"
#######
#S....#
#.##.##
#....##
##.#.##
#....E#
#######"#;
    
    let path_maze = EnhancedMaze::from_string(multi_path_maze_str);
    println!("Maze with multiple equivalent shortest paths:");
    path_maze.display_with_paths(None);
    
    if let (Some(&start), Some(&end)) = (path_maze.starts().first(), path_maze.ends().first()) {
        let all_shortest = find_all_shortest_paths(&path_maze, start, end);
        println!("\n🎯 Found {} shortest paths of length {}:", 
                 all_shortest.len(), 
                 all_shortest.first().map(|p| p.len() - 1).unwrap_or(0));
        
        for (i, path) in all_shortest.iter().enumerate() {
            println!("  Path {}: {:?}", i + 1, path);
        }
        
        println!("\nVisualization with different path markers:");
        path_maze.display_with_paths(Some(&all_shortest));
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Exercise 3: Random Maze Generation
    println!("🎲 Exercise 3: Random Maze Generation");
    println!("------------------------------------");
    
    println!("Generating random mazes with different parameters:");
    
    for (i, (size, wall_prob, starts, ends)) in [(8, 0.3, 1, 1), (10, 0.4, 2, 2), (12, 0.35, 1, 3)].iter().enumerate() {
        println!("\n🎮 Random Maze {} ({}×{}, {:.0}% walls, {} starts, {} ends):", 
                 i + 1, size, size, wall_prob * 100.0, starts, ends);
        
        let random_maze = generate_random_maze(*size, *size, *wall_prob, *starts, *ends);
        random_maze.display_with_paths(None);
        
        // Try to solve one path
        if let (Some(&start), Some(&end)) = (random_maze.starts().first(), random_maze.ends().first()) {
            if let Some(path) = shortest_path_basic(&random_maze, start, end) {
                println!("✅ Solvable! Path length: {}", path.len() - 1);
            } else {
                println!("❌ No path found in this random maze");
            }
        }
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Exercise 4: Removable Obstacles
    println!("💣 Exercise 4: Removable Obstacles");
    println!("----------------------------------");
    
    let obstacle_maze_str = r#"
########
#S.....#
#.###3.#
#.1.#1.#
#.2.#2.#
#.....E#
########"#;
    
    let obstacle_maze = EnhancedMaze::from_string(obstacle_maze_str);
    println!("Maze with removable obstacles (numbers show removal cost):");
    obstacle_maze.display_with_paths(None);
    
    if let (Some(&start), Some(&end)) = (obstacle_maze.starts().first(), obstacle_maze.ends().first()) {
        println!("\n🧨 Testing different obstacle removal budgets:");
        
        for budget in [0, 2, 4, 6] {
            match shortest_path_with_obstacles(&obstacle_maze, start, end, budget) {
                Some((path, cost_used)) => {
                    println!("  Budget {}: ✅ Path found! Length: {}, Cost used: {}", 
                             budget, path.len() - 1, cost_used);
                    if budget >= 4 {
                        println!("    Path: {:?}", path);
                    }
                }
                None => {
                    println!("  Budget {}: ❌ No path possible", budget);
                }
            }
        }
        
        // Show optimal path with sufficient budget
        if let Some((optimal_path, cost)) = shortest_path_with_obstacles(&obstacle_maze, start, end, 10) {
            println!("\n🎯 Optimal path with sufficient budget:");
            println!("  Total cost: {}, Path length: {}", cost, optimal_path.len() - 1);
            obstacle_maze.display_with_paths(Some(&[optimal_path]));
        }
    }
    
    println!("\n{}\n", "=".repeat(50));
    
    // Summary
    println!("🎓 Exercise Solutions Summary");
    println!("============================");
    println!("✅ Exercise 1: Multi-start pathfinding - Connect multiple starts to multiple goals");
    println!("✅ Exercise 2: All shortest paths - Find multiple equivalent optimal routes");
    println!("✅ Exercise 3: Random maze generation - Procedural maze creation with parameters");
    println!("✅ Exercise 4: Obstacle removal - Pathfinding with resource-limited obstacle clearing");
    println!();
    println!("💡 Key Learning Outcomes:");
    println!("  • BFS naturally extends to multi-source/multi-target scenarios");
    println!("  • Multiple shortest paths require modified BFS with path tracking");
    println!("  • Randomization techniques can create diverse problem instances");
    println!("  • Weighted obstacles transform pathfinding into a resource optimization problem");
    println!("  • Graph algorithms adapt well to problem variations and constraints");
    
    println!("\n🎉 All Day 5 exercise solutions completed successfully!");
}