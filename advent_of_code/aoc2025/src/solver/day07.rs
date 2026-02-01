use anyhow::{Context, Result};
use mission6::{Coord, Direction, Grid};
use mission8::Graph;
use std::collections::{HashMap, HashSet, VecDeque};

/// Cell types in the tachyon manifold
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,    // '.' - tachyon passes through
    Splitter, // '^' - beam splits left and right
    Start,    // 'S' - starting position
}

/// Wrapper for Grid that implements Mission 8's Graph trait
///
/// Demonstrates Mission integration: Mission 6 Grid + Mission 8 Graph trait
#[allow(dead_code)]
struct ManifoldGraph<'a> {
    grid: &'a Grid<Cell>,
}

#[allow(dead_code)]
impl<'a> ManifoldGraph<'a> {
    fn new(grid: &'a Grid<Cell>) -> Self {
        ManifoldGraph { grid }
    }
}

impl<'a> Graph for ManifoldGraph<'a> {
    type Node = Coord;

    /// Get neighbors for tachyon beam simulation
    ///
    /// For a position, returns possible next positions based on cell type:
    /// - At splitter: Returns left and right positions (quantum split)
    /// - At empty/start: Returns position below (beams move South)
    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        let mut neighbors = Vec::new();

        if let Some(cell) = self.grid.get(node) {
            match cell {
                Cell::Splitter => {
                    // Quantum split: both left and right positions
                    if let Some(left) = node.try_move_in_direction(Direction::West, 1) {
                        if self.grid.get(left).is_some() {
                            neighbors.push(left);
                        }
                    }
                    if let Some(right) = node.try_move_in_direction(Direction::East, 1) {
                        if self.grid.get(right).is_some() {
                            neighbors.push(right);
                        }
                    }
                }
                Cell::Empty | Cell::Start => {
                    // Continue moving South
                    if let Some(south) = node.try_move_in_direction(Direction::South, 1) {
                        if self.grid.get(south).is_some() {
                            neighbors.push(south);
                        }
                    }
                }
            }
        }

        neighbors
    }

    fn contains(&self, node: Self::Node) -> bool {
        self.grid.get(node).is_some()
    }

    /// Returns all grid cells as nodes
    ///
    /// Note: For this problem, we don't need to enumerate all nodes (beam simulation
    /// starts from a specific position), but we implement this for trait completeness.
    fn nodes(&self) -> Vec<Self::Node> {
        let mut all_nodes = Vec::new();
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                all_nodes.push(Coord { x, y });
            }
        }
        all_nodes
    }
}

/// Parse the manifold grid from input
///
/// Returns a Grid<Cell> and the starting position as Coord
fn parse_manifold(input: &str) -> Result<(Grid<Cell>, Coord)> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

    let height = lines.len();
    let width = lines.first().context("Empty input")?.len();

    // Mission 6 Grid: new(width, height, default)
    let mut grid = Grid::new(width, height, Cell::Empty);
    let mut start_pos = None;

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let cell = match ch {
                '.' => Cell::Empty,
                '^' => Cell::Splitter,
                'S' => {
                    // Mission 6 Coord: x=col, y=row
                    start_pos = Some(Coord::new(col, row));
                    Cell::Start
                }
                _ => anyhow::bail!("Unknown character '{}' at ({}, {})", ch, row, col),
            };
            // Mission 6 Grid supports indexing with (col, row) tuples
            grid[(col, row)] = cell;
        }
    }

    let start = start_pos.context("No starting position 'S' found")?;
    Ok((grid, start))
}

pub fn solve_part1(input: &str) -> Result<String> {
    solve_part1_with_debug(input, false)
}

fn solve_part1_with_debug(input: &str, debug: bool) -> Result<String> {
    let (grid, start_pos) = parse_manifold(input)?;

    // Simulate beam splitting using BFS
    // Track beams as (position, direction) pairs to avoid loops
    let mut queue: VecDeque<(Coord, Direction)> = VecDeque::new();
    let mut visited: HashSet<(Coord, Direction)> = HashSet::new();
    let mut split_count = 0;

    // Start beam moving South from S
    queue.push_back((start_pos, Direction::South));

    while let Some((pos, dir)) = queue.pop_front() {
        // Avoid infinite loops - skip if we've seen this (position, direction) before
        if !visited.insert((pos, dir)) {
            continue;
        }

        if debug {
            println!("Processing beam at ({}, {}) moving {:?}", pos.x, pos.y, dir);
        }

        // Check current cell
        if let Some(cell) = grid.get(pos) {
            if *cell == Cell::Splitter {
                // We're on a splitter! Count this split event
                split_count += 1;
                if debug {
                    println!("  Hit splitter #{} at ({}, {})", split_count, pos.x, pos.y);
                }

                // Create beams MOVING DOWNWARD (South) from cells to left and right
                // "a new tachyon beam continues from the immediate left and from the immediate right"
                // and "tachyon beams always move downward"
                if let Some(left_pos) = pos.try_move_in_direction(Direction::West, 1) {
                    if grid.get(left_pos).is_some() {
                        queue.push_back((left_pos, Direction::South));
                        if debug {
                            println!(
                                "    Spawned beam at ({}, {}) moving South",
                                left_pos.x, left_pos.y
                            );
                        }
                    }
                }
                if let Some(right_pos) = pos.try_move_in_direction(Direction::East, 1) {
                    if grid.get(right_pos).is_some() {
                        queue.push_back((right_pos, Direction::South));
                        if debug {
                            println!(
                                "    Spawned beam at ({}, {}) moving South",
                                right_pos.x, right_pos.y
                            );
                        }
                    }
                }
                continue; // Don't continue in original direction
            }
        }

        // Try to move in current direction
        if let Some(next_pos) = pos.try_move_in_direction(dir, 1) {
            // Check if still in bounds
            if grid.get(next_pos).is_some() {
                // Continue moving - next iteration will check if it's a splitter
                queue.push_back((next_pos, dir));
            }
            // If out of bounds, beam exits - do nothing
        }
        // If can't move (underflow), beam exits - do nothing
    }

    Ok(split_count.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let (grid, start_pos) = parse_manifold(input)?;

    // For Part 2: Count total number of timeline paths
    // Use DFS to count all paths from start to exit
    count_timelines(&grid, start_pos)
}

/// Count all possible quantum timelines from a position
///
/// **Mission Integration Note**: While we implement Graph trait above for the manifold,
/// we can't directly use Mission 8's generic `dfs()` for this problem because:
/// 1. Standard DFS counts nodes, not paths
/// 2. This problem requires counting ALL unique paths from start to any exit
/// 3. Memoization is critical - without it, the solution times out (O(2^n) paths)
///
/// This demonstrates an important integrator principle: Use mission components as
/// building blocks (Grid, Graph trait), but add problem-specific optimizations
/// (memoization, path counting) where the generic implementations don't fit.
///
/// # Pattern Matching Applied (Ch19.1)
/// - `if let Some(...)` - Option handling for cache lookups and grid bounds
/// - `match cell` - Cell type branching for beam physics
fn count_timelines(grid: &Grid<Cell>, start: Coord) -> Result<String> {
    // Memoized DFS to count all possible paths efficiently
    fn count_paths_memo(
        grid: &Grid<Cell>,
        pos: Coord,
        visited: &mut HashSet<Coord>,
        memo: &mut HashMap<Coord, usize>,
    ) -> usize {
        // Check memo first
        if let Some(&cached) = memo.get(&pos) {
            return cached;
        }

        // Check if we've visited this position in current path (cycle detection)
        if visited.contains(&pos) {
            return 0;
        }

        visited.insert(pos);

        let result = {
            // Check current cell
            if let Some(cell) = grid.get(pos) {
                if *cell == Cell::Splitter {
                    // Split into two paths - quantum particle takes BOTH
                    let mut total = 0;

                    if let Some(left_pos) = pos.try_move_in_direction(Direction::West, 1) {
                        if grid.get(left_pos).is_some() {
                            total += count_paths_memo(grid, left_pos, visited, memo);
                        }
                    }

                    if let Some(right_pos) = pos.try_move_in_direction(Direction::East, 1) {
                        if grid.get(right_pos).is_some() {
                            total += count_paths_memo(grid, right_pos, visited, memo);
                        }
                    }

                    total
                } else {
                    // Empty or Start - continue moving South
                    if let Some(next_pos) = pos.try_move_in_direction(Direction::South, 1) {
                        if grid.get(next_pos).is_some() {
                            count_paths_memo(grid, next_pos, visited, memo)
                        } else {
                            // Exiting grid - one timeline
                            1
                        }
                    } else {
                        // Can't move (underflow) - one timeline
                        1
                    }
                }
            } else {
                // Outside grid - should not happen
                0
            }
        };

        visited.remove(&pos);
        memo.insert(pos, result);
        result
    }

    let mut visited = HashSet::new();
    let mut memo = HashMap::new();
    let timeline_count = count_paths_memo(grid, start, &mut visited, &mut memo);
    Ok(timeline_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_parse_manifold() {
        let (grid, start) = parse_manifold(EXAMPLE).unwrap();

        // Verify start position (Mission 6 Coord: x=col, y=row)
        assert_eq!(start, Coord::new(7, 0));
        assert_eq!(grid[start], Cell::Start);

        // Verify some splitters
        assert_eq!(grid[Coord::new(7, 2)], Cell::Splitter);

        // Verify grid dimensions (width, height)
        assert_eq!(grid.width(), 15);
        assert_eq!(grid.height(), 16);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "40");
    }
}
