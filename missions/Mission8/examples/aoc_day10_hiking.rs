use mission8::{bfs, Graph};

/// Represents a position on the topographic map as (row, col)
type Position = (usize, usize);

/// Topographic map with height information
struct TopoMap {
    heights: Vec<Vec<Option<u32>>>,  // None for impassable positions
    rows: usize,
    cols: usize,
}

impl TopoMap {
    /// Create a new topographic map from input string
    fn from_str(input: &str) -> Self {
        let heights: Vec<Vec<Option<u32>>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            None  // Impassable
                        } else {
                            c.to_digit(10)
                        }
                    })
                    .collect()
            })
            .collect();

        let rows = heights.len();
        let cols = if rows > 0 { heights[0].len() } else { 0 };

        Self { heights, rows, cols }
    }

    /// Get height at a position (None if impassable)
    fn height_at(&self, pos: Position) -> Option<u32> {
        self.heights[pos.0][pos.1]
    }

    /// Check if position is valid (in bounds)
    fn is_valid(&self, pos: Position) -> bool {
        pos.0 < self.rows && pos.1 < self.cols
    }

    /// Find all trailheads (positions with height 0)
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

    /// Get valid neighbors (4-directional, height increases by exactly 1)
    fn valid_neighbors(&self, pos: Position) -> Vec<Position> {
        let current_height = match self.height_at(pos) {
            Some(h) => h,
            None => return Vec::new(),  // Impassable position
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

    /// Part 1: Calculate trailhead score (number of reachable 9s)
    fn trailhead_score(&self, start: Position) -> usize {
        let visited = bfs(self, start);

        // Count how many visited positions have height 9
        visited
            .iter()
            .filter(|&&pos| self.height_at(pos) == Some(9))
            .count()
    }

    /// Part 2: Calculate trailhead rating (number of distinct paths to 9s)
    fn trailhead_rating(&self, start: Position) -> usize {
        self.count_paths_dfs(start)
    }

    /// DFS to count all distinct paths from current position to height 9
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

/// Implement Graph trait for TopoMap to use Mission 8's BFS
impl Graph for TopoMap {
    type Node = Position;

    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        self.valid_neighbors(node)
    }

    fn contains(&self, node: Self::Node) -> bool {
        self.is_valid(node)
    }

    fn nodes(&self) -> Vec<Self::Node> {
        let mut nodes = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                nodes.push((row, col));
            }
        }
        nodes
    }
}

fn main() {
    // Example from problem description
    let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let map = TopoMap::from_str(input);
    let trailheads = map.find_trailheads();

    println!("=== AoC 2024 Day 10: Hoof It ===");
    println!("Using Mission 8 Generic Graph Traversal\n");

    // Part 1: Sum of trailhead scores
    let mut total_score = 0;
    println!("Part 1: Trailhead Scores");
    for (i, &trailhead) in trailheads.iter().enumerate() {
        let score = map.trailhead_score(trailhead);
        println!("  Trailhead {} at {:?}: score = {}", i + 1, trailhead, score);
        total_score += score;
    }
    println!("\nTotal score: {}", total_score);
    assert_eq!(total_score, 36, "Part 1 example should be 36");

    // Part 2: Sum of trailhead ratings
    let mut total_rating = 0;
    println!("\nPart 2: Trailhead Ratings");
    for (i, &trailhead) in trailheads.iter().enumerate() {
        let rating = map.trailhead_rating(trailhead);
        println!(
            "  Trailhead {} at {:?}: rating = {}",
            i + 1,
            trailhead,
            rating
        );
        total_rating += rating;
    }
    println!("\nTotal rating: {}", total_rating);
    assert_eq!(total_rating, 81, "Part 2 example should be 81");

    println!("\n✅ Both parts solved using Mission 8 graph algorithms!");
    println!("   - Part 1: BFS to find reachable destinations");
    println!("   - Part 2: DFS to count all distinct paths");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example_part1() {
        let input = "\
0123
1234
8765
9876";
        let map = TopoMap::from_str(input);
        let trailheads = map.find_trailheads();

        assert_eq!(trailheads.len(), 1);
        assert_eq!(map.trailhead_score(trailheads[0]), 1);
    }

    #[test]
    fn test_small_example_part2() {
        let input = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

        let map = TopoMap::from_str(input);
        let trailheads = map.find_trailheads();

        assert_eq!(trailheads.len(), 1);
        assert_eq!(map.trailhead_rating(trailheads[0]), 3);
    }

    #[test]
    fn test_large_example_part1() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let map = TopoMap::from_str(input);
        let trailheads = map.find_trailheads();

        let total_score: usize = trailheads.iter().map(|&th| map.trailhead_score(th)).sum();
        assert_eq!(total_score, 36);
    }

    #[test]
    fn test_large_example_part2() {
        let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let map = TopoMap::from_str(input);
        let trailheads = map.find_trailheads();

        let total_rating: usize = trailheads.iter().map(|&th| map.trailhead_rating(th)).sum();
        assert_eq!(total_rating, 81);
    }

    #[test]
    fn test_neighbors_constraint() {
        let input = "\
012
123
234";
        let map = TopoMap::from_str(input);

        // From position (0,0) with height 0, should only reach (0,1) and (1,0) with height 1
        let neighbors = map.valid_neighbors((0, 0));
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(1, 0)));
    }

    #[test]
    fn test_graph_trait_implementation() {
        let input = "\
012
123
234";
        let map = TopoMap::from_str(input);

        // Test Graph trait methods
        assert!(map.contains((0, 0)));
        assert!(map.contains((2, 2)));
        assert!(!map.contains((5, 5)));

        let all_nodes = map.nodes();
        assert_eq!(all_nodes.len(), 9); // 3x3 grid

        let neighbors = map.neighbors((1, 1));
        assert!(!neighbors.is_empty());
    }
}
