//! Day 21: Step Counter - BFS Reachability on Grid
//!
//! **Problem**: Count garden plots reachable in exactly N steps from starting position.
//!
//! **Approach**:
//! - Part 1: BFS flood-fill tracking step parity (64 steps)
//! - Part 2: TBD (likely infinite grid pattern detection)
//!
//! **Key Insight**: We can reach a position in exactly N steps if:
//! 1. Shortest path ≤ N steps
//! 2. (N - shortest_path) is even (can go back-and-forth)
//!
//! **Mission Integration**:
//! - Mission 6: Grid<char> for map representation
//! - Mission 8: BFS for shortest paths
//!
//! **Mathematical Foundation**:
//! - Graph Theory: BFS finds shortest paths in unweighted graphs
//! - Parity: Positions reachable in N steps have same parity as N
//!
//! **See**: `zettelkasten/math-foundations/graph-theory-fundamentals.md`

use std::collections::{HashSet, VecDeque};

/// Parse input into grid and find starting position
fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Find starting position 'S'
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .position(|&c| c == 'S')
                .map(|col| (row, col))
        })
        .expect("Starting position 'S' not found");

    (grid, start)
}

/// Count reachable garden plots in exactly `steps` steps
fn count_reachable(grid: &[Vec<char>], start: (usize, usize), steps: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    
    // BFS to find all positions reachable within `steps` steps
    // Track (row, col, step_count)
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    
    queue.push_back((start.0, start.1, 0));
    visited.insert((start.0, start.1, 0));
    
    // Track positions at target step count
    let mut reachable_at_target = HashSet::new();
    
    while let Some((row, col, step)) = queue.pop_front() {
        // If we've reached target steps, mark this position
        if step == steps {
            reachable_at_target.insert((row, col));
            continue; // Don't explore further from here
        }
        
        // Explore neighbors (up, down, left, right)
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            
            // Bounds check
            if new_row < 0 || new_row >= rows as isize || new_col < 0 || new_col >= cols as isize {
                continue;
            }
            
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            
            // Check if it's a valid garden plot
            if grid[new_row][new_col] == '#' {
                continue;
            }
            
            let new_state = (new_row, new_col, step + 1);
            
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back(new_state);
            }
        }
    }
    
    reachable_at_target.len()
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    count_reachable(&grid, start, 64)
}

/// Count reachable plots on INFINITE grid (tiles repeat)
/// Uses modulo arithmetic to map infinite coordinates to grid
fn count_reachable_infinite(grid: &[Vec<char>], start: (isize, isize), steps: usize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    
    // BFS on infinite grid - use (row, col) as infinite coordinates
    let mut queue: VecDeque<(isize, isize, usize)> = VecDeque::new();
    let mut visited: HashSet<(isize, isize, usize)> = HashSet::new();
    
    queue.push_back((start.0, start.1, 0));
    visited.insert((start.0, start.1, 0));
    
    let mut reachable_at_target = HashSet::new();
    
    while let Some((row, col, step)) = queue.pop_front() {
        if step == steps {
            reachable_at_target.insert((row, col));
            continue;
        }
        
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_row = row + dr;
            let new_col = col + dc;
            
            // Map to grid using modulo (handle negative with Euclidean mod)
            let grid_row = new_row.rem_euclid(rows) as usize;
            let grid_col = new_col.rem_euclid(cols) as usize;
            
            // Check if it's a valid garden plot
            if grid[grid_row][grid_col] == '#' {
                continue;
            }
            
            let new_state = (new_row, new_col, step + 1);
            
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back(new_state);
            }
        }
    }
    
    reachable_at_target.len()
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let grid_size = grid.len(); // Assuming square grid
    let start_infinite = (start.0 as isize, start.1 as isize);
    
    // Key insight: 26501365 = 65 + 131*202300
    // Where 65 is distance to edge, 131 is grid size
    // This means we can find a quadratic pattern!
    
    // Calculate for 3 data points: 65, 65+131, 65+262
    let edge_dist = grid_size / 2; // 65 for 131x131 grid
    
    let n0 = count_reachable_infinite(&grid, start_infinite, edge_dist);
    let n1 = count_reachable_infinite(&grid, start_infinite, edge_dist + grid_size);
    let n2 = count_reachable_infinite(&grid, start_infinite, edge_dist + 2 * grid_size);
    
    // Fit quadratic: f(x) = ax² + bx + c
    // We have f(0)=n0, f(1)=n1, f(2)=n2
    // Using Lagrange interpolation / finite differences
    
    let n0 = n0 as i64;
    let n1 = n1 as i64;
    let n2 = n2 as i64;
    
    let a = (n0 - 2 * n1 + n2) / 2;
    let b = (-3 * n0 + 4 * n1 - n2) / 2;
    let c = n0;
    
    // Target: (26501365 - 65) / 131 = 202300
    let target_n = ((26501365 - edge_dist) / grid_size) as i64;
    
    // Evaluate quadratic at target_n
    (a * target_n * target_n + b * target_n + c) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_example_6_steps() {
        let (grid, start) = parse_input(EXAMPLE);
        assert_eq!(count_reachable(&grid, start, 6), 16);
    }
    
    #[test]
    fn test_infinite_grid_example() {
        let (grid, start) = parse_input(EXAMPLE);
        let start_inf = (start.0 as isize, start.1 as isize);
        
        // Test example data points
        assert_eq!(count_reachable_infinite(&grid, start_inf, 6), 16);
        assert_eq!(count_reachable_infinite(&grid, start_inf, 10), 50);
        assert_eq!(count_reachable_infinite(&grid, start_inf, 50), 1594);
        assert_eq!(count_reachable_infinite(&grid, start_inf, 100), 6536);
    }
}
