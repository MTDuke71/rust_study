//! Day 18: RAM Run - Memory Space Pathfinding
//!
//! **Mission Integration**: Perfect example of composing validated components!
//! - **Mission 6 (Grid)**: Memory space representation with efficient coordinate access
//! - **Mission 8 (Graph + BFS)**: Shortest path algorithm for navigating safe coordinates
//!
//! ## Problem Summary
//! - **Part 1**: Find shortest path through 71x71 memory grid after 1024 bytes have corrupted it
//! - **Part 2**: Binary search to find first byte that blocks all paths to exit
//!
//! ## Solution Approach
//! 1. Parse byte coordinates from input
//! 2. Use Mission 6's `Grid<bool>` to track corrupted positions
//! 3. Implement Mission 8's `Graph` trait for the memory space
//! 4. Part 1: Use `shortest_path()` after simulating 1024 bytes
//! 5. Part 2: Binary search over byte count until path becomes impossible
//!
//! ## Integrator Philosophy in Action
//! Instead of implementing grid + BFS from scratch:
//! - ✅ Reused validated Mission 6 Grid component
//! - ✅ Reused validated Mission 8 shortest_path algorithm
//! - ✅ Focused on problem-specific logic (corruption simulation, binary search)
//! - ✅ Clean, maintainable code leveraging proven components

use anyhow::{Context, Result};
use mission6::{Coord, Grid};
use mission8::{shortest_path, Graph};

/// Memory space that tracks corrupted coordinates
#[derive(Debug, Clone)]
struct MemorySpace {
    grid: Grid<bool>, // true = corrupted, false = safe
    width: usize,
    height: usize,
}

impl MemorySpace {
    /// Create a new memory space with given dimensions
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height, false),
            width,
            height,
        }
    }

    /// Mark a coordinate as corrupted
    fn corrupt(&mut self, coord: Coord) {
        if coord.x < self.width && coord.y < self.height {
            self.grid[coord] = true;
        }
    }

    /// Check if a coordinate is safe (not corrupted and within bounds)
    fn is_safe(&self, coord: Coord) -> bool {
        coord.x < self.width && coord.y < self.height && !self.grid[coord]
    }
}

/// Implement Graph trait for pathfinding using Mission 8's shortest_path
impl Graph for MemorySpace {
    type Node = Coord;

    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        let mut neighbors = Vec::new();

        // Try all four directions: up, down, left, right
        let directions = [
            (0, -1), // up
            (0, 1),  // down
            (-1, 0), // left
            (1, 0),  // right
        ];

        for (dx, dy) in directions {
            if let (Some(new_x), Some(new_y)) =
                (node.x.checked_add_signed(dx), node.y.checked_add_signed(dy))
            {
                let new_coord = Coord::new(new_x, new_y);
                if self.is_safe(new_coord) {
                    neighbors.push(new_coord);
                }
            }
        }

        neighbors
    }

    fn contains(&self, node: Self::Node) -> bool {
        node.x < self.width && node.y < self.height
    }

    fn nodes(&self) -> Vec<Self::Node> {
        // Return all coordinates in the grid
        let mut all_nodes = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                all_nodes.push(Coord::new(x, y));
            }
        }
        all_nodes
    }
}

/// Parse falling byte coordinates from input
fn parse_bytes(input: &str) -> Result<Vec<Coord>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid coordinate format: {}", line);
            }
            let x = parts[0]
                .parse::<usize>()
                .with_context(|| format!("Invalid X coordinate: {}", parts[0]))?;
            let y = parts[1]
                .parse::<usize>()
                .with_context(|| format!("Invalid Y coordinate: {}", parts[1]))?;
            Ok(Coord::new(x, y))
        })
        .collect()
}

/// Part 1: Find shortest path after first N bytes have fallen
pub fn part1(input: &str) -> Result<String> {
    let bytes = parse_bytes(input)?;

    // For the real input, grid is 71x71 (0-70), simulate first 1024 bytes
    let (width, height, num_bytes) = if bytes.len() < 30 {
        // Example: 7x7 grid (0-6), 12 bytes
        (7, 7, 12)
    } else {
        // Real input: 71x71 grid (0-70), 1024 bytes
        (71, 71, 1024)
    };

    // Create memory space and simulate falling bytes
    let mut memory = MemorySpace::new(width, height);
    for coord in bytes.iter().take(num_bytes) {
        memory.corrupt(*coord);
    }

    // Find shortest path from (0,0) to (width-1, height-1)
    let start = Coord::new(0, 0);
    let goal = Coord::new(width - 1, height - 1);

    match shortest_path(&memory, start, goal) {
        Ok(path) => {
            // Path length is number of steps (edges), not number of nodes
            let steps = path.len() - 1;
            Ok(steps.to_string())
        }
        Err(_) => anyhow::bail!("No path found to exit"),
    }
}

/// Part 2: Find first byte that blocks the path
pub fn part2(input: &str) -> Result<String> {
    let bytes = parse_bytes(input)?;

    // Determine grid size
    let (width, height) = if bytes.len() < 30 { (7, 7) } else { (71, 71) };

    let start = Coord::new(0, 0);
    let goal = Coord::new(width - 1, height - 1);

    // Binary search for the first blocking byte
    let mut left = 0;
    let mut right = bytes.len();

    while left < right {
        let mid = (left + right) / 2;

        // Create memory space with first 'mid' bytes fallen
        let mut memory = MemorySpace::new(width, height);
        for coord in bytes.iter().take(mid) {
            memory.corrupt(*coord);
        }

        // Check if path still exists
        if shortest_path(&memory, start, goal).is_ok() {
            // Path exists, try more bytes
            left = mid + 1;
        } else {
            // Path blocked, try fewer bytes
            right = mid;
        }
    }

    // left-1 is the last byte where path existed, left is first blocking byte
    if left > 0 && left <= bytes.len() {
        let blocking_byte = bytes[left - 1];
        Ok(format!("{},{}", blocking_byte.x, blocking_byte.y))
    } else {
        anyhow::bail!("Could not find blocking byte");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE).unwrap();
        assert_eq!(result, "22");
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE).unwrap();
        assert_eq!(result, "6,1");
    }
}
