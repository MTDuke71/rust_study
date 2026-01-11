//! # Day 10: Pipe Maze
//!
//! ## Part 1
//! Find the giant loop of pipes starting at 'S'. Calculate how many steps along the loop
//! it takes to get from the starting position to the farthest point.
//!
//! ## Algorithm
//! - Approach: Use Mission 6 Grid<char> for storage + BFS traversal to find loop distances
//! - Parse grid, find 'S', determine pipe connections, BFS to find all distances
//! - Complexity: O(width × height) for grid operations
//!
//! ## Mission Integration
//! - Mission 6 Grid<char>: 2D pipe maze storage with coordinate navigation
//! - Mission 8 BFS pattern: Traverse the continuous loop, track distances from start
//!
//! ## Mathematical Foundation
//!
//! **Part 1 - Graph Theory**:
//! - BFS for shortest paths in unweighted graphs
//! - Cycle detection in undirected graphs
//! - See `zettelkasten/math-foundations/graph-theory-fundamentals.md`
//!
//! **Part 2 - Computational Geometry**:
//! - Ray casting algorithm for point-in-polygon detection
//! - Jordan Curve Theorem (odd crossings = inside)
//! - Corner handling with state machine
//! - See `zettelkasten/math-foundations/computational-geometry-basics.md`

use anyhow::Result;
use mission6::{Coord, Grid};
use std::collections::{HashMap, VecDeque};

/// Represents the four cardinal directions for pipe connections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    /// Get the offset (dx, dy) for moving in this direction
    fn offset(&self) -> (i32, i32) {
        match self {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
        }
    }

    /// Get the opposite direction
    fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

/// Get the directions that a pipe character connects to
fn pipe_connections(ch: char) -> Vec<Dir> {
    match ch {
        '|' => vec![Dir::North, Dir::South],
        '-' => vec![Dir::East, Dir::West],
        'L' => vec![Dir::North, Dir::East],
        'J' => vec![Dir::North, Dir::West],
        '7' => vec![Dir::South, Dir::West],
        'F' => vec![Dir::South, Dir::East],
        '.' => vec![],
        'S' => vec![Dir::North, Dir::South, Dir::East, Dir::West], // Placeholder, will be determined
        _ => vec![],
    }
}

/// Move from a coordinate in a direction, returning the new coordinate if valid
fn move_coord(grid: &Grid<char>, coord: Coord, dir: Dir) -> Option<Coord> {
    let (dx, dy) = dir.offset();
    let new_x = coord.x as i32 + dx;
    let new_y = coord.y as i32 + dy;

    if new_x >= 0 && new_y >= 0 {
        let new_coord = Coord::new(new_x as usize, new_y as usize);
        if grid.in_bounds(new_coord) {
            return Some(new_coord);
        }
    }
    None
}

/// Check if two pipes connect in the given direction
fn pipes_connect(grid: &Grid<char>, from: Coord, dir: Dir) -> bool {
    let from_char = grid[from];
    let from_connections = pipe_connections(from_char);

    // Check if 'from' pipe has an opening in this direction
    if !from_connections.contains(&dir) && from_char != 'S' {
        return false;
    }

    // Get the neighbor coordinate
    let Some(to) = move_coord(grid, from, dir) else {
        return false;
    };

    let to_char = grid[to];
    let to_connections = pipe_connections(to_char);

    // Check if 'to' pipe has an opening in the opposite direction
    to_connections.contains(&dir.opposite())
}

/// Find the starting position 'S' in the grid
fn find_start(grid: &Grid<char>) -> Option<Coord> {
    for (coord, &ch) in grid.enumerate() {
        if ch == 'S' {
            return Some(coord);
        }
    }
    None
}

/// Use BFS to traverse the pipe loop and find distances from start
fn find_loop_distances(grid: &Grid<char>, start: Coord) -> HashMap<Coord, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let current_dist = distances[&current];

        // Try all four directions
        for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            if pipes_connect(grid, current, dir) {
                if let Some(neighbor) = move_coord(grid, current, dir) {
                    // Only visit if we haven't seen it yet
                    if let std::collections::hash_map::Entry::Vacant(e) = distances.entry(neighbor)
                    {
                        e.insert(current_dist + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    distances
}

/// Parse the input into a Grid<char>
fn parse_grid(input: &str) -> Grid<char> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut grid = Grid::new(width, height, '.');

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
        }
    }

    grid
}

/// **Day 10: Pipe Maze**
///
/// Find the farthest point in the pipe loop from the starting position.
pub fn solve_part1(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");

    let distances = find_loop_distances(&grid, start);

    // The farthest point is the maximum distance
    let max_distance = distances.values().max().unwrap_or(&0);

    Ok(max_distance.to_string())
}

/// Determine what pipe type 'S' actually represents based on its connections
fn determine_start_pipe(grid: &Grid<char>, start: Coord) -> char {
    let connects_north = pipes_connect(grid, start, Dir::North);
    let connects_south = pipes_connect(grid, start, Dir::South);
    let connects_east = pipes_connect(grid, start, Dir::East);
    let connects_west = pipes_connect(grid, start, Dir::West);

    match (connects_north, connects_south, connects_east, connects_west) {
        (true, true, false, false) => '|',
        (false, false, true, true) => '-',
        (true, false, true, false) => 'L',
        (true, false, false, true) => 'J',
        (false, true, false, true) => '7',
        (false, true, true, false) => 'F',
        _ => 'S', // Shouldn't happen in valid input
    }
}

/// Count how many tiles are enclosed by the loop
///
/// Uses ray casting algorithm: for each non-loop tile, count crossings
/// of the loop boundary. Odd crossings = inside, even = outside.
pub fn solve_part2(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");

    // First, find all tiles that are part of the main loop
    let loop_tiles = find_loop_distances(&grid, start);

    // Determine what 'S' actually represents
    let start_pipe = determine_start_pipe(&grid, start);

    // For each row, scan left to right using ray casting
    let mut enclosed_count = 0;

    for y in 0..grid.height() {
        let mut inside = false;
        let mut enter_corner: Option<char> = None;

        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            let mut ch = grid[coord];

            // Replace 'S' with its actual pipe type
            if ch == 'S' {
                ch = start_pipe;
            }

            // Skip if this tile is part of the loop
            if loop_tiles.contains_key(&coord) {
                // Check if we need to flip inside/outside state
                match ch {
                    '|' => {
                        // Vertical pipe always crosses
                        inside = !inside;
                    }
                    'F' | 'L' => {
                        // Entering a corner section
                        enter_corner = Some(ch);
                    }
                    '7' => {
                        // Exiting corner - check if it's a real crossing
                        if enter_corner == Some('L') {
                            // L---7 is a crossing (went up then down)
                            inside = !inside;
                        }
                        // F---7 is NOT a crossing (stayed on bottom)
                        enter_corner = None;
                    }
                    'J' => {
                        // Exiting corner - check if it's a real crossing
                        if enter_corner == Some('F') {
                            // F---J is a crossing (went down then up)
                            inside = !inside;
                        }
                        // L---J is NOT a crossing (stayed on top)
                        enter_corner = None;
                    }
                    '-' => {
                        // Horizontal pipe - no state change
                    }
                    _ => {}
                }
            } else if inside {
                // This tile is not part of the loop and we're inside
                enclosed_count += 1;
            }
        }
    }

    Ok(enclosed_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const EXAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const EXAMPLE5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1_example1() {
        let result = solve_part1(EXAMPLE1).unwrap();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part1_example2() {
        let result = solve_part1(EXAMPLE2).unwrap();
        assert_eq!(result, "8");
    }

    #[test]
    fn test_pipe_connections() {
        assert_eq!(pipe_connections('|'), vec![Dir::North, Dir::South]);
        assert_eq!(pipe_connections('-'), vec![Dir::East, Dir::West]);
        assert_eq!(pipe_connections('L'), vec![Dir::North, Dir::East]);
        assert_eq!(pipe_connections('J'), vec![Dir::North, Dir::West]);
        assert_eq!(pipe_connections('7'), vec![Dir::South, Dir::West]);
        assert_eq!(pipe_connections('F'), vec![Dir::South, Dir::East]);
        assert_eq!(pipe_connections('.'), vec![]);
    }

    #[test]
    fn test_part2_example3() {
        let result = solve_part2(EXAMPLE3).unwrap();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part2_example4() {
        let result = solve_part2(EXAMPLE4).unwrap();
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part2_example5() {
        let result = solve_part2(EXAMPLE5).unwrap();
        assert_eq!(result, "10");
    }
}
