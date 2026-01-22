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

pub fn part2(_input: &str) -> usize {
    // Part 2 TBD
    0
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
}
