use mission6::{Coord, Grid};
use std::collections::{HashMap, VecDeque};

/// Day 20: Race Condition - Cheat Detection in Maze Racing
/// 
/// Problem: Find cheats (phasing through walls) that save time on a racetrack
/// Part 1: Cheats last exactly 2 picoseconds
/// Part 2: Cheats last up to 20 picoseconds
/// 
/// Key Insight: Use bidirectional distance maps (BFS from S and E) to calculate
/// cheat savings efficiently without pathfinding each cheat.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    
    /// Convert Pos to Mission 6 Coord for grid access
    /// Used in tests for validation
    #[allow(dead_code)]
    pub fn to_coord(&self) -> Coord {
        Coord::new(self.col, self.row)  // Coord is (x=col, y=row)
    }
    
    /// Manhattan distance between two positions
    pub fn manhattan_distance(&self, other: &Pos) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
    
    /// Get all positions within Manhattan distance `max_dist`
    pub fn positions_within_distance(&self, max_dist: usize, rows: usize, cols: usize) -> Vec<(Pos, usize)> {
        let mut positions = Vec::new();
        
        for dr in -(max_dist as isize)..=(max_dist as isize) {
            for dc in -(max_dist as isize)..=(max_dist as isize) {
                let new_row = self.row as isize + dr;
                let new_col = self.col as isize + dc;
                
                if new_row >= 0 && new_row < rows as isize 
                    && new_col >= 0 && new_col < cols as isize {
                    let target = Pos::new(new_row as usize, new_col as usize);
                    let dist = self.manhattan_distance(&target);
                    
                    if dist > 0 && dist <= max_dist {
                        positions.push((target, dist));
                    }
                }
            }
        }
        
        positions
    }
}

/// Parse the grid and find start and end positions
pub fn parse_input(input: &str) -> (Grid<char>, Pos, Pos) {
    let lines: Vec<&str> = input.trim().lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    
    let mut grid = Grid::new(width, height, '#');
    let mut start = Pos::new(0, 0);
    let mut end = Pos::new(0, 0);
    
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let coord = Coord::new(c, r);
            *grid.get_mut(coord).unwrap() = ch;
            match ch {
                'S' => start = Pos::new(r, c),
                'E' => end = Pos::new(r, c),
                _ => {}
            }
        }
    }
    
    (grid, start, end)
}

/// BFS from a starting position to calculate distances to all reachable positions
/// This is one half of our "bidirectional" distance calculation
pub fn bfs_distances(grid: &Grid<char>, start: Pos) -> HashMap<Pos, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    
    queue.push_back((start, 0));
    distances.insert(start, 0);
    
    while let Some((pos, dist)) = queue.pop_front() {
        // Try all 4 directions
        let neighbors = [
            (pos.row.wrapping_sub(1), pos.col),     // Up
            (pos.row + 1, pos.col),                  // Down
            (pos.row, pos.col.wrapping_sub(1)),     // Left
            (pos.row, pos.col + 1),                  // Right
        ];
        
        for (new_row, new_col) in neighbors {
            let coord = Coord::new(new_col, new_row);
            if let Some(&cell) = grid.get(coord) {
                if cell != '#' {  // Can walk on '.', 'S', or 'E'
                    let next_pos = Pos::new(new_row, new_col);
                    
                    if let std::collections::hash_map::Entry::Vacant(e) = distances.entry(next_pos) {
                        e.insert(dist + 1);
                        queue.push_back((next_pos, dist + 1));
                    }
                }
            }
        }
    }
    
    distances
}

/// Count cheats that save at least `min_savings` picoseconds
/// max_cheat_length: 2 for Part 1, 20 for Part 2
fn count_cheats(
    grid: &Grid<char>,
    start: Pos,
    end: Pos,
    max_cheat_length: usize,
    min_savings: usize,
) -> usize {
    // Run BFS from start and end to get bidirectional distance maps
    let dist_from_start = bfs_distances(grid, start);
    let dist_to_end = bfs_distances(grid, end);
    
    // Normal path length (no cheating)
    let normal_path_length = *dist_from_start.get(&end).unwrap();
    
    let mut cheat_count = 0;
    
    let (rows, cols) = (grid.height(), grid.width());
    
    // For each position on the track, try all possible cheats
    for (&cheat_start, &dist_to_cheat_start) in &dist_from_start {
        // Try all positions within Manhattan distance `max_cheat_length`
        for (cheat_end, cheat_length) in cheat_start.positions_within_distance(max_cheat_length, rows, cols) {
            // Check if cheat_end is on the track (has a distance to end)
            if let Some(&dist_from_cheat_end) = dist_to_end.get(&cheat_end) {
                // Calculate total distance with this cheat
                let distance_with_cheat = dist_to_cheat_start + cheat_length + dist_from_cheat_end;
                
                // Calculate savings
                if distance_with_cheat < normal_path_length {
                    let savings = normal_path_length - distance_with_cheat;
                    
                    if savings >= min_savings {
                        cheat_count += 1;
                    }
                }
            }
        }
    }
    
    cheat_count
}

pub fn solve_part1(input: &str) -> String {
    let (grid, start, end) = parse_input(input);
    let result = count_cheats(&grid, start, end, 2, 100);
    result.to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (grid, start, end) = parse_input(input);
    let result = count_cheats(&grid, start, end, 20, 100);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_parse_input() {
        let (grid, start, end) = parse_input(EXAMPLE);
        assert_eq!(*grid.get(start.to_coord()).unwrap(), 'S');
        assert_eq!(*grid.get(end.to_coord()).unwrap(), 'E');
    }

    #[test]
    fn test_bfs_distances() {
        let (grid, start, end) = parse_input(EXAMPLE);
        let distances = bfs_distances(&grid, start);
        
        // Should find a path to the end
        assert!(distances.contains_key(&end));
        
        // The example states the fastest time is 84 picoseconds
        assert_eq!(*distances.get(&end).unwrap(), 84);
    }

    #[test]
    fn test_part1_example() {
        let (grid, start, end) = parse_input(EXAMPLE);
        
        // Test with lower threshold to see all cheats
        // Example says there are 44 cheats total (various savings amounts)
        let all_cheats = count_cheats(&grid, start, end, 2, 2);
        assert_eq!(all_cheats, 44);
    }

    #[test]
    fn test_part2_example() {
        let (grid, start, end) = parse_input(EXAMPLE);
        
        // Part 2: with 20 picosecond cheats and 50+ savings
        let cheats_50plus = count_cheats(&grid, start, end, 20, 50);
        
        // Example states: 32+31+29+39+25+23+20+19+12+14+12+22+4+3 = 285
        assert_eq!(cheats_50plus, 285);
    }
}
