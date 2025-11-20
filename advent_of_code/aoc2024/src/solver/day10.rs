use anyhow::{Context, Result};
use mission6::{Coord, Grid};
use mission8::{bfs, Graph};

/// Topographic map using Mission 6's Grid for storage and Mission 8's Graph for algorithms.
#[derive(Debug, Clone)]
struct TopoMap {
    heights: Grid<Option<u32>>,
}

impl TopoMap {
    /// Parse the hiking trail map from the input string.
    /// Digits 0-9 represent heights, '.' represents impassable positions.
    /// Leverages Mission 6's grid parsing as a foundation.
    fn from_str(input: &str) -> Result<Self> {
        let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() {
            anyhow::bail!("Empty input");
        }

        let height = lines.len();
        let width = lines[0].len();

        // Verify rectangular grid
        for (idx, line) in lines.iter().enumerate() {
            if line.len() != width {
                anyhow::bail!(
                    "Non-rectangular grid at line {}: got {} columns, expected {}",
                    idx,
                    line.len(),
                    width
                );
            }
        }

        // Create grid and parse heights
        let mut heights = Grid::new(width, height, None);

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let coord = Coord::new(x, y);
                heights[coord] = if ch == '.' {
                    None
                } else {
                    Some(
                        ch.to_digit(10)
                            .with_context(|| format!("Invalid character '{ch}' in map"))?,
                    )
                };
            }
        }

        Ok(TopoMap { heights })
    }

    /// Get the height at a coordinate, or None if impassable or out of bounds.
    /// Uses Mission 6's safe access via Grid::get().
    fn height_at(&self, coord: Coord) -> Option<u32> {
        self.heights.get(coord).and_then(|&h| h)
    }

    /// Find all trailheads (positions with height 0).
    /// Uses Mission 6's grid enumeration.
    fn find_trailheads(&self) -> Vec<Coord> {
        self.heights
            .enumerate()
            .filter_map(|(coord, &height)| {
                if height == Some(0) {
                    Some(coord)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get valid neighbors (4-directional, height increases by exactly 1).
    /// Uses Mission 6's Coord::neighbors_4() for direction handling.
    fn valid_neighbors(&self, coord: Coord) -> Vec<Coord> {
        let current_height = match self.height_at(coord) {
            Some(h) => h,
            None => return Vec::new(), // Impassable position
        };

        // Mission 6 provides 4-directional neighbors with automatic bounds handling
        coord
            .neighbors_4()
            .filter(|&next_coord| {
                // Must be in bounds
                if !self.heights.in_bounds(next_coord) {
                    return false;
                }

                // Must have height = current + 1
                if let Some(next_height) = self.height_at(next_coord) {
                    next_height == current_height + 1
                } else {
                    false
                }
            })
            .collect()
    }

    /// Part 1: Calculate trailhead score (number of reachable 9s).
    /// Uses Mission 8's generic BFS algorithm.
    fn trailhead_score(&self, start: Coord) -> usize {
        let visited = bfs(self, start);

        // Count how many visited positions have height 9
        visited
            .iter()
            .filter(|&&coord| self.height_at(coord) == Some(9))
            .count()
    }

    /// Part 2: Calculate trailhead rating (number of distinct paths to 9s).
    fn trailhead_rating(&self, start: Coord) -> usize {
        self.count_paths_dfs(start)
    }

    /// DFS to count all distinct paths from current position to height 9.
    fn count_paths_dfs(&self, coord: Coord) -> usize {
        // If we reached height 9, we found one complete path
        if self.height_at(coord) == Some(9) {
            return 1;
        }

        // Count paths through all valid neighbors
        // valid_neighbors already checks height increases by exactly 1
        let mut total_paths = 0;
        for neighbor in self.valid_neighbors(coord) {
            total_paths += self.count_paths_dfs(neighbor);
        }

        total_paths
    }
}

/// Implement Mission 8's Graph trait for TopoMap.
/// This allows using generic graph algorithms like BFS on the topographic map.
impl Graph for TopoMap {
    type Node = Coord;

    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        self.valid_neighbors(node)
    }

    fn contains(&self, node: Self::Node) -> bool {
        self.heights.in_bounds(node) && self.height_at(node).is_some()
    }

    fn nodes(&self) -> Vec<Self::Node> {
        self.heights
            .enumerate()
            .filter_map(|(coord, &height)| height.map(|_| coord))
            .collect()
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
        assert_eq!(map.heights.height(), 7);
        assert_eq!(map.heights.width(), 7);
        assert_eq!(map.height_at(Coord::new(3, 0)), Some(0));
        assert_eq!(map.height_at(Coord::new(0, 0)), None); // '.' is impassable
    }

    #[test]
    fn test_find_trailheads() {
        let map = TopoMap::from_str(SMALL_EXAMPLE).unwrap();
        let trailheads = map.find_trailheads();
        assert_eq!(trailheads.len(), 1);
        assert_eq!(trailheads[0], Coord::new(3, 0));
    }

    #[test]
    fn test_valid_neighbors() {
        let map = TopoMap::from_str(SMALL_EXAMPLE).unwrap();
        let neighbors = map.valid_neighbors(Coord::new(3, 0)); // height 0
        assert_eq!(neighbors.len(), 1); // Only one neighbor with height 1
        assert_eq!(neighbors[0], Coord::new(3, 1));
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
        assert!(map.contains(Coord::new(3, 0))); // height 0
        assert!(!map.contains(Coord::new(0, 0))); // '.' is impassable
        assert!(!map.contains(Coord::new(100, 100))); // out of bounds

        let all_nodes = map.nodes();
        assert!(all_nodes.contains(&Coord::new(3, 0)));
        assert!(!all_nodes.contains(&Coord::new(0, 0))); // '.' excluded
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
