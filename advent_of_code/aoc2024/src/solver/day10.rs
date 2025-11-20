use anyhow::{bail, Context, Result};
use mission8::{bfs, Graph};

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct TopoMap {
    heights: Vec<Vec<Option<u32>>>,
    rows: usize,
    cols: usize,
}

impl TopoMap {
    /// Parse the hiking trail map from the input string.
    /// Digits 0-9 represent heights, '.' represents impassable positions.
    fn from_str(input: &str) -> Result<Self> {
        let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() {
            bail!("Empty input");
        }

        let rows = lines.len();
        let cols = lines[0].len();
        let mut heights = Vec::with_capacity(rows);

        for (row_idx, line) in lines.iter().enumerate() {
            if line.len() != cols {
                bail!(
                    "Non-rectangular grid at line {}: got {} columns, expected {}",
                    row_idx,
                    line.len(),
                    cols
                );
            }

            let mut row = Vec::with_capacity(cols);
            for ch in line.chars() {
                let height = if ch == '.' {
                    None
                } else {
                    Some(
                        ch.to_digit(10)
                            .with_context(|| format!("Invalid character '{ch}' in map"))?,
                    )
                };
                row.push(height);
            }
            heights.push(row);
        }

        Ok(TopoMap { heights, rows, cols })
    }

    /// Check if a position is within grid bounds.
    fn is_valid(&self, pos: Position) -> bool {
        pos.0 < self.rows && pos.1 < self.cols
    }

    /// Get the height at a position, or None if impassable.
    fn height_at(&self, pos: Position) -> Option<u32> {
        if self.is_valid(pos) {
            self.heights[pos.0][pos.1]
        } else {
            None
        }
    }

    /// Find all trailheads (positions with height 0).
    fn find_trailheads(&self) -> Vec<Position> {
        let mut trailheads = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.heights[row][col] == Some(0) {
                    trailheads.push((row, col));
                }
            }
        }
        trailheads
    }

    /// Get valid neighbors (4-directional, height increases by exactly 1).
    fn valid_neighbors(&self, pos: Position) -> Vec<Position> {
        let current_height = match self.height_at(pos) {
            Some(h) => h,
            None => return Vec::new(), // Impassable position
        };
        let (row, col) = pos;
        let mut neighbors = Vec::new();

        // Check all 4 directions
        let directions = [
            (row.wrapping_sub(1), col), // Up
            (row + 1, col),              // Down
            (row, col.wrapping_sub(1)), // Left
            (row, col + 1),              // Right
        ];

        for &next_pos in &directions {
            if self.is_valid(next_pos) {
                if let Some(next_height) = self.height_at(next_pos) {
                    if next_height == current_height + 1 {
                        neighbors.push(next_pos);
                    }
                }
            }
        }

        neighbors
    }

    /// Part 1: Calculate trailhead score (number of reachable 9s).
    /// Uses Mission 8's generic BFS algorithm.
    fn trailhead_score(&self, start: Position) -> usize {
        let visited = bfs(self, start);

        // Count how many visited positions have height 9
        visited
            .iter()
            .filter(|&&pos| self.height_at(pos) == Some(9))
            .count()
    }

    /// Part 2: Calculate trailhead rating (number of distinct paths to 9s).
    fn trailhead_rating(&self, start: Position) -> usize {
        self.count_paths_dfs(start)
    }

    /// DFS to count all distinct paths from current position to height 9.
    fn count_paths_dfs(&self, pos: Position) -> usize {
        // If we reached height 9, we found one complete path
        if self.height_at(pos) == Some(9) {
            return 1;
        }

        // Count paths through all valid neighbors
        // valid_neighbors already checks height increases by exactly 1
        let mut total_paths = 0;
        for neighbor in self.valid_neighbors(pos) {
            total_paths += self.count_paths_dfs(neighbor);
        }

        total_paths
    }
}

/// Implement Mission 8's Graph trait for TopoMap.
/// This allows using generic graph algorithms like BFS.
impl Graph for TopoMap {
    type Node = Position;

    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        self.valid_neighbors(node)
    }

    fn contains(&self, node: Self::Node) -> bool {
        self.is_valid(node) && self.height_at(node).is_some()
    }

    fn nodes(&self) -> Vec<Self::Node> {
        let mut all_nodes = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.heights[row][col].is_some() {
                    all_nodes.push((row, col));
                }
            }
        }
        all_nodes
    }
}

/// Solve Part 1: Sum of all trailhead scores.
/// Each trailhead's score is the number of height-9 positions reachable from it.
pub fn solve_part1(input: &str) -> Result<String> {
    let map = TopoMap::from_str(input).context("Failed to parse topographic map")?;
    let trailheads = map.find_trailheads();

    let total_score: usize = trailheads.iter().map(|&th| map.trailhead_score(th)).sum();

    Ok(total_score.to_string())
}

/// Solve Part 2: Sum of all trailhead ratings.
/// Each trailhead's rating is the number of distinct hiking trails (paths) from it to any height-9.
pub fn solve_part2(input: &str) -> Result<String> {
    let map = TopoMap::from_str(input).context("Failed to parse topographic map")?;
    let trailheads = map.find_trailheads();

    let total_rating: usize = trailheads.iter().map(|&th| map.trailhead_rating(th)).sum();

    Ok(total_rating.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const LARGE_EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_parse_map() {
        let map = TopoMap::from_str(SMALL_EXAMPLE).unwrap();
        assert_eq!(map.rows, 7);
        assert_eq!(map.cols, 7);
        assert_eq!(map.height_at((0, 3)), Some(0));
        assert_eq!(map.height_at((0, 0)), None); // '.' is impassable
    }

    #[test]
    fn test_find_trailheads() {
        let map = TopoMap::from_str(SMALL_EXAMPLE).unwrap();
        let trailheads = map.find_trailheads();
        assert_eq!(trailheads.len(), 1);
        assert_eq!(trailheads[0], (0, 3));
    }

    #[test]
    fn test_valid_neighbors() {
        let map = TopoMap::from_str(SMALL_EXAMPLE).unwrap();
        let neighbors = map.valid_neighbors((0, 3)); // height 0
        assert_eq!(neighbors.len(), 1); // Only one neighbor with height 1
        assert_eq!(neighbors[0], (1, 3));
    }

    #[test]
    fn test_part1_small_example() {
        let result = solve_part1(SMALL_EXAMPLE).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part1_large_example() {
        let result = solve_part1(LARGE_EXAMPLE).unwrap();
        assert_eq!(result, "36");
    }

    #[test]
    fn test_part2_small_rating_3() {
        let input = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part2_small_rating_13() {
        let input = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2_large_example() {
        let result = solve_part2(LARGE_EXAMPLE).unwrap();
        assert_eq!(result, "81");
    }

    #[test]
    fn test_graph_trait() {
        let map = TopoMap::from_str(SMALL_EXAMPLE).unwrap();
        assert!(map.contains((0, 3))); // height 0
        assert!(!map.contains((0, 0))); // '.' is impassable
        assert!(!map.contains((100, 100))); // out of bounds

        let all_nodes = map.nodes();
        assert!(all_nodes.contains(&(0, 3)));
        assert!(!all_nodes.contains(&(0, 0))); // '.' excluded
    }

    #[test]
    fn test_empty_input() {
        let result = TopoMap::from_str("");
        assert!(result.is_err());
    }

    #[test]
    fn test_non_rectangular_grid() {
        let input = "012\n34";
        let result = TopoMap::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_character() {
        let input = "01X\n234\n567";
        let result = TopoMap::from_str(input);
        assert!(result.is_err());
    }
}
