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
pub fn find_start(grid: &Grid<char>) -> Option<Coord> {
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
pub fn parse_grid(input: &str) -> Grid<char> {
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

/// Solve both parts efficiently by sharing grid parsing and loop distance calculation
pub fn solve_both_parts(input: &str) -> Result<(String, String)> {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");
    let loop_tiles = find_loop_distances(&grid, start);
    let start_pipe = determine_start_pipe(&grid, start);

    // Part 1: Find maximum distance
    let max_distance = loop_tiles.values().max().unwrap_or(&0);
    let part1 = max_distance.to_string();

    // Part 2: Count enclosed tiles using ray casting
    let mut enclosed_count = 0;

    for y in 0..grid.height() {
        let mut inside = false;
        let mut enter_corner: Option<char> = None;

        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            let mut ch = grid[coord];

            if ch == 'S' {
                ch = start_pipe;
            }

            if loop_tiles.contains_key(&coord) {
                match ch {
                    '|' => inside = !inside,
                    'F' | 'L' => enter_corner = Some(ch),
                    '7' => {
                        if enter_corner == Some('L') {
                            inside = !inside;
                        }
                        enter_corner = None;
                    }
                    'J' => {
                        if enter_corner == Some('F') {
                            inside = !inside;
                        }
                        enter_corner = None;
                    }
                    '-' => {}
                    _ => {}
                }
            } else if inside {
                enclosed_count += 1;
            }
        }
    }

    let part2 = enclosed_count.to_string();

    Ok((part1, part2))
}

/// **Day 10: Pipe Maze**
///
/// Find the farthest point in the pipe loop from the starting position.
pub fn solve_part1(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");

    let distances = find_loop_distances(&grid, start);

    // The farthest point in a circular loop is exactly halfway around
    // For even-length loops: n/2, for odd-length: (n+1)/2 (ceiling division)
    // This works for any loop length without scanning for max
    let max_distance = distances.len().div_ceil(2);

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

/// Trace the loop and return ordered vertices as (x, y) coordinates
///
/// This converts the pipe loop into a polygon for use with Shoelace formula.
/// Starting from 'S', follows the pipes in order to build the vertex list.
pub fn find_loop_vertices(grid: &Grid<char>, start: Coord) -> Vec<(i64, i64)> {
    let mut vertices = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut current = start;
    
    // Add starting vertex
    vertices.push((start.x as i64, start.y as i64));
    visited.insert(start);
    
    // Find first connected neighbor
    let mut prev_dir = None;
    for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
        if pipes_connect(grid, current, dir) {
            if let Some(next) = move_coord(grid, current, dir) {
                current = next;
                prev_dir = Some(dir);
                vertices.push((current.x as i64, current.y as i64));
                visited.insert(current);
                break;
            }
        }
    }
    
    // Follow the loop until we return to start
    while current != start {
        let mut found_next = false;
        
        // Try all directions except where we came from
        for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            // Don't go back the way we came
            if Some(dir.opposite()) == prev_dir {
                continue;
            }
            
            if pipes_connect(grid, current, dir) {
                if let Some(next) = move_coord(grid, current, dir) {
                    if next == start {
                        // Completed the loop
                        return vertices;
                    }
                    
                    if !visited.contains(&next) {
                        current = next;
                        prev_dir = Some(dir);
                        vertices.push((current.x as i64, current.y as i64));
                        visited.insert(current);
                        found_next = true;
                        break;
                    }
                }
            }
        }
        
        if !found_next {
            // Shouldn't happen with valid input
            break;
        }
    }
    
    vertices
}

/// Count how many tiles are enclosed by the loop
///
/// # Algorithm: Jordan Curve Theorem (Horizontal Ray Casting)
///
/// Scans each row left-to-right, counting how many loop boundaries are crossed.
/// Odd crossings = inside, even crossings = outside.
///
/// # Related Problems
/// - **AoC 2025 Day 9**: Same algorithm for continuous polygon (see `aoc2025/examples/animate_ray_casting.py`)
/// - **Pattern**: Works for both discrete grids (this) and continuous spaces (Day 9)
///
/// # Complexity
/// - Time: O(width × height) - single pass through grid
/// - Space: O(loop_size) - store loop tile positions
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

// ============================================================================
// Visualization Functions
// ============================================================================

/// Visualize the pipe maze with loop highlighting
pub fn visualize_loop(input: &str) -> String {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");
    let loop_tiles = find_loop_distances(&grid, start);

    let mut output = String::new();
    output.push_str("Pipe Maze - Loop Highlighted:\n");
    output.push_str(&"─".repeat(grid.width() + 2));
    output.push('\n');

    for y in 0..grid.height() {
        output.push('│');
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            let ch = grid[coord];

            if loop_tiles.contains_key(&coord) {
                // Highlight loop tiles with colored Unicode box drawing
                let display = match ch {
                    '|' => '│',
                    '-' => '─',
                    'L' => '└',
                    'J' => '┘',
                    '7' => '┐',
                    'F' => '┌',
                    'S' => '●',
                    _ => ch,
                };
                output.push(display);
            } else {
                output.push('·'); // Non-loop tiles
            }
        }
        output.push('│');
        output.push('\n');
    }

    output.push_str(&"─".repeat(grid.width() + 2));
    output.push('\n');
    output
}

/// Visualize distances from start
pub fn visualize_distances(input: &str) -> String {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");
    let loop_tiles = find_loop_distances(&grid, start);
    let max_dist = loop_tiles.values().max().unwrap_or(&0);

    let mut output = String::new();
    output.push_str(&format!("Distance Map (max distance: {}):\n", max_dist));
    output.push_str(&"─".repeat(grid.width() * 3 + 2));
    output.push('\n');

    for y in 0..grid.height() {
        output.push('│');
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);

            if let Some(&dist) = loop_tiles.get(&coord) {
                output.push_str(&format!("{:2} ", dist));
            } else {
                output.push_str(" · ");
            }
        }
        output.push('│');
        output.push('\n');
    }

    output.push_str(&"─".repeat(grid.width() * 3 + 2));
    output.push('\n');
    output
}

/// Visualize inside/outside tiles (Part 2)
///
/// Uses ray casting to show which tiles are inside vs outside the loop.
///
/// # See Also
/// - `solve_part2()`: Uses same algorithm to count inside tiles
/// - AoC 2025 Day 9: Same ray casting pattern for continuous polygons
///   (`aoc2025/examples/animate_ray_casting.py` shows animated version)
pub fn visualize_inside_outside(input: &str) -> String {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");
    let loop_tiles = find_loop_distances(&grid, start);
    let start_pipe = determine_start_pipe(&grid, start);

    let mut output = String::new();
    output.push_str("Inside/Outside Map:\n");
    output.push_str("  ● = Loop tile\n");
    output.push_str("  █ = Inside loop\n");
    output.push_str("  · = Outside loop\n");
    output.push_str(&"─".repeat(grid.width() + 2));
    output.push('\n');

    for y in 0..grid.height() {
        output.push('│');
        let mut inside = false;
        let mut enter_corner: Option<char> = None;

        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            let mut ch = grid[coord];

            if ch == 'S' {
                ch = start_pipe;
            }

            if loop_tiles.contains_key(&coord) {
                output.push('●'); // Loop tile

                // Update inside/outside state
                match ch {
                    '|' => inside = !inside,
                    'F' | 'L' => enter_corner = Some(ch),
                    '7' => {
                        if enter_corner == Some('L') {
                            inside = !inside;
                        }
                        enter_corner = None;
                    }
                    'J' => {
                        if enter_corner == Some('F') {
                            inside = !inside;
                        }
                        enter_corner = None;
                    }
                    _ => {}
                }
            } else if inside {
                output.push('█'); // Inside
            } else {
                output.push('·'); // Outside
            }
        }
        output.push('│');
        output.push('\n');
    }

    output.push_str(&"─".repeat(grid.width() + 2));
    output.push('\n');
    output
}

/// Comprehensive visualization showing all three views
pub fn visualize_all(input: &str) -> String {
    let mut output = String::new();
    output.push_str("═".repeat(60).as_str());
    output.push_str("\n DAY 10: PIPE MAZE VISUALIZATION\n");
    output.push_str("═".repeat(60).as_str());
    output.push_str("\n\n");

    output.push_str(&visualize_loop(input));
    output.push('\n');
    output.push_str(&visualize_distances(input));
    output.push('\n');
    output.push_str(&visualize_inside_outside(input));

    output
}

/// Export visualization to HTML file with proper Unicode support
///
/// Creates a beautiful HTML visualization with:
/// - UTF-8 encoding for proper Unicode box-drawing characters
/// - Color-coded sections (loop in blue, inside tiles in red)
/// - Responsive layout that works in any browser
/// - All three visualization modes in one page
///
/// # See Also
/// - `aoc2025/examples/animate_ray_casting.py`: Animated visualization of same algorithm for polygon
/// - `aoc2025/examples/ray_casting_animation.gif`: Shows odd/even crossing pattern
pub fn export_visualization_html(input: &str, output_path: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AoC 2023 Day 10: Pipe Maze Visualization</title>
    <style>
        body {{
            background: #1e1e1e;
            color: #d4d4d4;
            font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
            padding: 20px;
            line-height: 1.2;
        }}
        .container {{
            max-width: 100%;
            margin: 0 auto;
        }}
        h1 {{
            color: #4ec9b0;
            text-align: center;
            font-size: 2em;
            margin-bottom: 10px;
        }}
        h2 {{
            color: #569cd6;
            margin-top: 30px;
            margin-bottom: 15px;
            border-bottom: 2px solid #569cd6;
            padding-bottom: 5px;
        }}
        .visualization {{
            background: #252526;
            padding: 15px;
            border-radius: 5px;
            overflow-x: auto;
            margin-bottom: 20px;
            white-space: pre;
            font-size: 14px;
        }}
        .loop-char {{ color: #4ec9b0; font-weight: bold; }}
        .inside-char {{ color: #f48771; font-weight: bold; }}
        .legend {{
            background: #2d2d30;
            padding: 10px;
            border-radius: 5px;
            margin-bottom: 20px;
        }}
        .legend-item {{
            display: inline-block;
            margin-right: 20px;
            margin-bottom: 5px;
        }}
        .stats {{
            background: #2d2d30;
            padding: 15px;
            border-radius: 5px;
            margin-bottom: 20px;
        }}
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 10px;
        }}
        .stat-item {{
            background: #252526;
            padding: 10px;
            border-radius: 3px;
        }}
        .stat-value {{
            color: #4ec9b0;
            font-size: 1.5em;
            font-weight: bold;
        }}
        .stat-label {{
            color: #808080;
            font-size: 0.9em;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🎄 Advent of Code 2023 - Day 10: Pipe Maze 🎄</h1>
        
        <div class="stats">
            <h2>📊 Statistics</h2>
            <div class="stats-grid">
                <div class="stat-item">
                    <div class="stat-label">Grid Size</div>
                    <div class="stat-value">{grid_size}</div>
                </div>
                <div class="stat-item">
                    <div class="stat-label">Loop Length</div>
                    <div class="stat-value">{loop_length}</div>
                </div>
                <div class="stat-item">
                    <div class="stat-label">Farthest Point</div>
                    <div class="stat-value">{max_distance}</div>
                </div>
                <div class="stat-item">
                    <div class="stat-label">Inside Tiles</div>
                    <div class="stat-value">{inside_count}</div>
                </div>
            </div>
        </div>
        
        <h2>🔄 Loop Structure</h2>
        <div class="legend">
            <div class="legend-item"><span class="loop-char">●</span> = Loop tile</div>
            <div class="legend-item"><span style="color: #d4d4d4;">·</span> = Non-loop tile</div>
        </div>
        <div class="visualization">{loop_viz}</div>
        
        <h2>📏 Distance Map</h2>
        <div class="legend">
            <div class="legend-item">Shows BFS distances from start position</div>
            <div class="legend-item"><span class="loop-char">Numbers</span> = Steps from start</div>
        </div>
        <div class="visualization">{distance_viz}</div>
        
        <h2>🎯 Inside/Outside Map</h2>
        <div class="legend">
            <div class="legend-item"><span class="loop-char">●</span> = Loop tile</div>
            <div class="legend-item"><span class="inside-char">█</span> = Inside loop</div>
            <div class="legend-item"><span style="color: #d4d4d4;">·</span> = Outside loop</div>
        </div>
        <div class="visualization">{inside_viz}</div>
    </div>
</body>
</html>"#,
        grid_size = {
            let lines: Vec<&str> = input.lines().collect();
            format!("{} × {}", lines.len(), lines.first().map_or(0, |l| l.len()))
        },
        loop_length = {
            let (part1, _) = solve_both_parts(input).expect("Valid input");
            part1.parse::<usize>().unwrap() * 2
        },
        max_distance = solve_part1(input)
            .expect("Valid input")
            .parse::<usize>()
            .unwrap(),
        inside_count = solve_part2(input).expect("Valid input"),
        loop_viz = visualize_loop(input).replace("●", "<span class=\"loop-char\">●</span>"),
        distance_viz = {
            let dist_viz = visualize_distances(input);
            // Color the numbers
            let mut result = String::new();
            for line in dist_viz.lines() {
                let mut colored_line = String::new();
                let mut i = 0;
                let chars: Vec<char> = line.chars().collect();
                while i < chars.len() {
                    if chars[i].is_ascii_digit() {
                        colored_line.push_str("<span class=\"loop-char\">");
                        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == ' ') {
                            colored_line.push(chars[i]);
                            i += 1;
                            if i < chars.len() && !chars[i].is_ascii_digit() {
                                break;
                            }
                        }
                        colored_line.push_str("</span>");
                    } else {
                        colored_line.push(chars[i]);
                        i += 1;
                    }
                }
                result.push_str(&colored_line);
                result.push('\n');
            }
            result
        },
        inside_viz = visualize_inside_outside(input)
            .replace("●", "<span class=\"loop-char\">●</span>")
            .replace("█", "<span class=\"inside-char\">█</span>"),
    );

    let mut file = File::create(output_path)?;
    file.write_all(html.as_bytes())?;

    Ok(())
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

    #[test]
    fn test_solve_both_parts() {
        let (part1, part2) = solve_both_parts(EXAMPLE1).unwrap();
        assert_eq!(part1, "4");
        assert_eq!(part2, "1");

        let (part1, part2) = solve_both_parts(EXAMPLE3).unwrap();
        assert_eq!(part1, "23");
        assert_eq!(part2, "4");
    }
}
