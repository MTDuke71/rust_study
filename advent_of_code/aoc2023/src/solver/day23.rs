//! # Day 23: A Long Walk
//!
//! **Longest Path with Forced Movement**
//!
//! ## Part 1
//! Find the longest hike through hiking trails with slopes that force movement:
//! - `.` = path (can move in any cardinal direction)
//! - `#` = forest (impassable)
//! - `^`, `>`, `v`, `<` = slopes (must move in indicated direction)
//! - Cannot revisit tiles
//! - Start: top row single path
//! - Goal: bottom row single path
//!
//! ## Part 2
//! TBD (likely: ignore slope restrictions)
//!
//! ## Algorithm
//! - Approach: DFS with backtracking to find longest path
//! - Complexity: O(4^n) worst case (exponential due to backtracking)
//! - Mission Integration: Mission 6 Grid, Mission 8 graph traversal concepts
//!
//! ## Optimizations
//! - Early pruning of impossible paths
//! - Visited set for cycle prevention
//! - Direction-based movement validation

use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    SlopeNorth,
    SlopeEast,
    SlopeSouth,
    SlopeWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Get neighbors in cardinal directions (N, E, S, W)
    fn neighbors(&self, rows: usize, cols: usize) -> Vec<(Coord, Direction)> {
        let mut result = Vec::new();

        // North
        if self.row > 0 {
            result.push((Coord::new(self.row - 1, self.col), Direction::North));
        }
        // South
        if self.row < rows - 1 {
            result.push((Coord::new(self.row + 1, self.col), Direction::South));
        }
        // West
        if self.col > 0 {
            result.push((Coord::new(self.row, self.col - 1), Direction::West));
        }
        // East
        if self.col < cols - 1 {
            result.push((Coord::new(self.row, self.col + 1), Direction::East));
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct HikingMap {
    grid: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
    start: Coord,
    goal: Coord,
}

impl HikingMap {
    fn parse(input: &str) -> Result<Self> {
        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();
        let cols = lines[0].len();

        let mut grid = Vec::with_capacity(rows);

        for line in lines {
            let row: Vec<Tile> = line
                .chars()
                .map(|c| match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '^' => Tile::SlopeNorth,
                    '>' => Tile::SlopeEast,
                    'v' => Tile::SlopeSouth,
                    '<' => Tile::SlopeWest,
                    _ => panic!("Unknown tile: {}", c),
                })
                .collect();
            grid.push(row);
        }

        // Find start (top row)
        let start_col = grid[0]
            .iter()
            .position(|&t| t == Tile::Path)
            .expect("No start found");
        let start = Coord::new(0, start_col);

        // Find goal (bottom row)
        let goal_col = grid[rows - 1]
            .iter()
            .position(|&t| t == Tile::Path)
            .expect("No goal found");
        let goal = Coord::new(rows - 1, goal_col);

        Ok(HikingMap {
            grid,
            rows,
            cols,
            start,
            goal,
        })
    }

    fn get(&self, coord: Coord) -> Tile {
        self.grid[coord.row][coord.col]
    }

    /// Check if we can move from current position to next position
    /// considering slope restrictions (if enabled)
    fn can_move(
        &self,
        _current: Coord,
        next: Coord,
        direction: Direction,
        ignore_slopes: bool,
    ) -> bool {
        let tile = self.get(next);

        match tile {
            Tile::Forest => false,
            Tile::Path => true,
            // Slopes either force movement in specific direction or are treated as paths
            Tile::SlopeNorth => ignore_slopes || direction == Direction::North,
            Tile::SlopeEast => ignore_slopes || direction == Direction::East,
            Tile::SlopeSouth => ignore_slopes || direction == Direction::South,
            Tile::SlopeWest => ignore_slopes || direction == Direction::West,
        }
    }

    /// DFS to find longest path
    fn find_longest_path(&self, ignore_slopes: bool) -> usize {
        let mut visited = HashSet::new();
        visited.insert(self.start);
        self.dfs(self.start, &mut visited, ignore_slopes)
    }

    fn dfs(&self, current: Coord, visited: &mut HashSet<Coord>, ignore_slopes: bool) -> usize {
        // Base case: reached goal
        if current == self.goal {
            return 0;
        }

        let mut max_length = 0;

        // Try all neighbors
        for (next, direction) in current.neighbors(self.rows, self.cols) {
            // Skip if already visited or can't move there
            if visited.contains(&next) || !self.can_move(current, next, direction, ignore_slopes) {
                continue;
            }

            // Recursively explore this path
            visited.insert(next);
            let length = self.dfs(next, visited, ignore_slopes);

            // Only count valid paths (those that reach the goal)
            if length > 0 || next == self.goal {
                max_length = max_length.max(length + 1);
            }

            visited.remove(&next);
        }

        max_length
    }

    /// Build a contracted graph of junctions and corridors
    /// This reduces the search space by collapsing long hallways
    fn build_graph(&self, ignore_slopes: bool) -> HashMap<Coord, Vec<(Coord, usize)>> {
        let mut graph: HashMap<Coord, Vec<(Coord, usize)>> = HashMap::new();

        // Find all junctions (points with > 2 neighbors) plus start and goal
        let mut junctions = HashSet::new();
        junctions.insert(self.start);
        junctions.insert(self.goal);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let coord = Coord::new(row, col);
                if self.get(coord) == Tile::Forest {
                    continue;
                }

                // Count accessible neighbors
                let neighbor_count = coord
                    .neighbors(self.rows, self.cols)
                    .into_iter()
                    .filter(|(next, dir)| self.can_move(coord, *next, *dir, ignore_slopes))
                    .count();

                if neighbor_count > 2 {
                    junctions.insert(coord);
                }
            }
        }

        println!("Found {} junctions", junctions.len());

        // For each junction, find paths to other junctions
        for &junction in &junctions {
            let mut edges = Vec::new();

            // BFS from this junction to find other junctions
            for (start_neighbor, start_dir) in junction.neighbors(self.rows, self.cols) {
                if !self.can_move(junction, start_neighbor, start_dir, ignore_slopes) {
                    continue;
                }

                let mut visited = HashSet::new();
                visited.insert(junction);
                visited.insert(start_neighbor);

                let mut current = start_neighbor;
                let mut distance = 1;

                // Follow the corridor until we hit another junction
                loop {
                    if junctions.contains(&current) {
                        edges.push((current, distance));
                        break;
                    }

                    // Find the next unvisited neighbor
                    let next_coords: Vec<_> = current
                        .neighbors(self.rows, self.cols)
                        .into_iter()
                        .filter(|(next, dir)| {
                            !visited.contains(next)
                                && self.can_move(current, *next, *dir, ignore_slopes)
                        })
                        .collect();

                    if next_coords.is_empty() {
                        // Dead end
                        break;
                    } else if next_coords.len() == 1 {
                        // Continue along corridor
                        let (next, _) = next_coords[0];
                        visited.insert(next);
                        current = next;
                        distance += 1;
                    } else {
                        // Multiple paths - this should be a junction
                        break;
                    }
                }
            }

            graph.insert(junction, edges);
        }

        graph
    }

    /// DFS on contracted graph
    fn dfs_graph(
        &self,
        current: Coord,
        visited: &mut HashSet<Coord>,
        graph: &HashMap<Coord, Vec<(Coord, usize)>>,
    ) -> usize {
        if current == self.goal {
            return 0;
        }

        let mut max_length = 0;

        if let Some(neighbors) = graph.get(&current) {
            for &(next, distance) in neighbors {
                if visited.contains(&next) {
                    continue;
                }

                visited.insert(next);
                let length = self.dfs_graph(next, visited, graph);

                if length > 0 || next == self.goal {
                    max_length = max_length.max(length + distance);
                }

                visited.remove(&next);
            }
        }

        max_length
    }
}

pub fn solve_part1(input: &str) -> Result<String> {
    let map = HikingMap::parse(input)?;
    // Part 1: Simple DFS works fine with slope restrictions
    let graph = map.build_graph(false); // Respect slopes
    println!("Part 1: {} junctions (with slope constraints)", graph.len());
    let longest = map.find_longest_path(false); // Respect slopes
    Ok(longest.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let map = HikingMap::parse(input)?;
    // Part 2: Use graph contraction to avoid stack overflow
    // (ignoring slopes creates many more paths)
    let graph = map.build_graph(true); // Ignore slopes
    println!("Part 2: {} junctions (ignoring slopes)", graph.len());
    let mut visited = HashSet::new();
    visited.insert(map.start);
    let longest = map.dfs_graph(map.start, &mut visited, &graph);
    Ok(longest.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "94");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "154");
    }

    #[test]
    fn test_parse_map() {
        let map = HikingMap::parse(EXAMPLE).unwrap();
        assert_eq!(map.rows, 23);
        assert_eq!(map.cols, 23);
        assert_eq!(map.start, Coord::new(0, 1));
        assert_eq!(map.goal, Coord::new(22, 21));
    }

    #[test]
    fn test_tile_parsing() {
        let map = HikingMap::parse(EXAMPLE).unwrap();
        assert_eq!(map.get(Coord::new(0, 0)), Tile::Forest);
        assert_eq!(map.get(Coord::new(0, 1)), Tile::Path);
        // Row 3: "###.....#.>.>.###.#.###"
        assert_eq!(map.get(Coord::new(3, 10)), Tile::SlopeEast);
        assert_eq!(map.get(Coord::new(3, 12)), Tile::SlopeEast);
        // Row 4: "###v#####.#v#.###.#.###"
        assert_eq!(map.get(Coord::new(4, 3)), Tile::SlopeSouth);
        assert_eq!(map.get(Coord::new(4, 11)), Tile::SlopeSouth);
    }
}
