//! Advent of Code utilities for grid-based problems
//!
//! Provides parsing, visualization, and common algorithms specifically
//! tailored for AoC grid problems.

use crate::{Coord, Grid};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

/// Common patterns for parsing AoC grid inputs
pub struct AocGridParser;

impl AocGridParser {
    /// Parse a multi-line string into a character grid
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission6::AocGridParser;
    ///
    /// let input = "..#\n.#.\n#..";
    /// let grid = AocGridParser::parse_char_grid(input);
    /// assert_eq!(grid.width(), 3);
    /// assert_eq!(grid.height(), 3);
    /// ```
    pub fn parse_char_grid(input: &str) -> Grid<char> {
        let lines: Vec<&str> = input.trim().lines().collect();
        if lines.is_empty() {
            return Grid::new(0, 0, ' ');
        }

        let height = lines.len();
        let width = lines[0].len();
        let mut grid = Grid::new(width, height, ' ');

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if x < width && y < height {
                    grid[(x, y)] = ch;
                }
            }
        }

        grid
    }

    /// Parse a multi-line string into a digit grid (0-9)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission6::AocGridParser;
    ///
    /// let input = "123\n456\n789";
    /// let grid = AocGridParser::parse_digit_grid(input);
    /// assert_eq!(grid[(1, 1)], 5);
    /// ```
    pub fn parse_digit_grid(input: &str) -> Grid<u32> {
        let lines: Vec<&str> = input.trim().lines().collect();
        if lines.is_empty() {
            return Grid::new(0, 0, 0);
        }

        let height = lines.len();
        let width = lines[0].len();
        let mut grid = Grid::new(width, height, 0);

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if let Some(digit) = ch.to_digit(10) {
                    if x < width && y < height {
                        grid[(x, y)] = digit;
                    }
                }
            }
        }

        grid
    }

    /// Parse a grid where each cell is separated by spaces
    ///
    /// Useful for parsing grids where each cell value can be multi-character.
    pub fn parse_spaced_grid<T>(input: &str) -> Grid<T>
    where
        T: FromStr + Default + Clone,
        T::Err: std::fmt::Debug,
    {
        let lines: Vec<&str> = input.trim().lines().collect();
        if lines.is_empty() {
            return Grid::new(0, 0, T::default());
        }

        let first_row: Vec<&str> = lines[0].split_whitespace().collect();
        let width = first_row.len();
        let height = lines.len();

        let mut grid = Grid::new(width, height, T::default());

        for (y, line) in lines.iter().enumerate() {
            let values: Vec<&str> = line.split_whitespace().collect();
            for (x, value_str) in values.iter().enumerate() {
                if let Ok(value) = value_str.parse::<T>() {
                    if x < width && y < height {
                        grid[(x, y)] = value;
                    }
                }
            }
        }

        grid
    }

    /// Find all coordinates containing a specific value
    ///
    /// Useful for finding special markers in AoC grids like start positions,
    /// enemies, treasures, etc.
    pub fn find_all<T: PartialEq>(grid: &Grid<T>, target: &T) -> Vec<Coord> {
        grid.enumerate()
            .filter_map(
                |(coord, value)| {
                    if value == target {
                        Some(coord)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    /// Find the first occurrence of a value
    ///
    /// Returns the coordinate of the first cell containing the target value,
    /// scanning left-to-right, top-to-bottom.
    pub fn find_first<T: PartialEq>(grid: &Grid<T>, target: &T) -> Option<Coord> {
        grid.enumerate().find_map(
            |(coord, value)| {
                if value == target {
                    Some(coord)
                } else {
                    None
                }
            },
        )
    }
}

/// Grid visualization utilities
pub struct GridVisualizer;

impl GridVisualizer {
    /// Convert a character grid to a pretty-printed string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission6::{Grid, GridVisualizer};
    ///
    /// let grid = Grid::new(3, 2, '.');
    /// let display = GridVisualizer::to_string(&grid);
    /// assert!(display.contains("..."));
    /// ```
    pub fn to_string<T: std::fmt::Display>(grid: &Grid<T>) -> String {
        let mut result = String::new();

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                result.push_str(&format!("{}", grid[(x, y)]));
            }
            if y < grid.height() - 1 {
                result.push('\n');
            }
        }

        result
    }

    /// Create a visual representation with coordinates
    ///
    /// Adds row and column numbers to help with debugging.
    pub fn to_string_with_coords<T: std::fmt::Display>(grid: &Grid<T>) -> String {
        let mut result = String::new();

        // Add column headers
        result.push_str("  ");
        for x in 0..grid.width() {
            result.push_str(&format!("{}", x % 10));
        }
        result.push('\n');

        for y in 0..grid.height() {
            // Add row number
            result.push_str(&format!("{} ", y % 10));

            for x in 0..grid.width() {
                result.push_str(&format!("{}", grid[(x, y)]));
            }
            result.push('\n');
        }

        result
    }

    /// Highlight specific coordinates in the grid
    ///
    /// Replaces the values at specified coordinates with a highlight character.
    pub fn highlight_coords<T>(grid: &Grid<T>, coords: &[Coord], highlight_char: char) -> String
    where
        T: Clone + PartialEq + std::fmt::Display,
    {
        let coord_set: HashSet<_> = coords.iter().copied().collect();
        let mut result = String::new();

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let coord = Coord::new(x, y);
                if coord_set.contains(&coord) {
                    result.push(highlight_char);
                } else {
                    result.push_str(&format!("{}", grid[(x, y)]));
                }
            }
            if y < grid.height() - 1 {
                result.push('\n');
            }
        }

        result
    }
}

/// Common AoC grid algorithms
pub struct AocAlgorithms;

impl AocAlgorithms {
    /// Count adjacent cells matching a condition
    ///
    /// Counts how many of the 8 surrounding cells (or 4 for cardinal-only)
    /// satisfy the given predicate.
    pub fn count_adjacent<T, F>(
        grid: &Grid<T>,
        coord: Coord,
        use_diagonals: bool,
        predicate: F,
    ) -> usize
    where
        F: Fn(&T) -> bool,
    {
        if use_diagonals {
            coord
                .neighbors_8_bounded(grid.width(), grid.height())
                .filter(|&neighbor| predicate(&grid[neighbor]))
                .count()
        } else {
            coord
                .neighbors_4_bounded(grid.width(), grid.height())
                .filter(|&neighbor| predicate(&grid[neighbor]))
                .count()
        }
    }

    /// Apply Conway's Game of Life rules (or similar cellular automaton)
    ///
    /// Creates a new grid based on cellular automaton rules applied to the current state.
    pub fn apply_cellular_automaton<T, F>(grid: &Grid<T>, rule: F) -> Grid<T>
    where
        T: Clone,
        F: Fn(&Grid<T>, Coord, &T) -> T,
    {
        let mut new_grid = grid.clone();

        for (coord, value) in grid.enumerate() {
            new_grid[coord] = rule(grid, coord, value);
        }

        new_grid
    }

    /// Find shortest distances from multiple sources simultaneously
    ///
    /// Useful for AoC problems where you need distances from multiple points.
    pub fn multi_source_bfs<T, F>(
        grid: &Grid<T>,
        sources: &[Coord],
        is_passable: F,
    ) -> HashMap<Coord, (usize, Coord)>
    // (distance, closest_source)
    where
        F: Fn(&T) -> bool,
    {
        let mut distances = HashMap::new();
        let mut queue = std::collections::VecDeque::new();

        // Initialize with all sources
        for &source in sources {
            if grid.in_bounds(source) && is_passable(&grid[source]) {
                distances.insert(source, (0, source));
                queue.push_back(source);
            }
        }

        while let Some(current) = queue.pop_front() {
            let (current_dist, closest_source) = distances[&current];

            for neighbor in current.neighbors_4_bounded(grid.width(), grid.height()) {
                if distances.contains_key(&neighbor) {
                    continue;
                }

                if !is_passable(&grid[neighbor]) {
                    continue;
                }

                distances.insert(neighbor, (current_dist + 1, closest_source));
                queue.push_back(neighbor);
            }
        }

        distances
    }

    /// Rotate the entire grid 90 degrees clockwise
    ///
    /// Creates a new grid rotated 90 degrees clockwise from the original.
    pub fn rotate_clockwise<T: Clone>(grid: &Grid<T>) -> Grid<T> {
        let new_width = grid.height();
        let new_height = grid.width();
        let mut new_grid = Grid::new(new_width, new_height, grid[(0, 0)].clone());

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let new_x = y;
                let new_y = grid.width() - 1 - x;
                new_grid[(new_x, new_y)] = grid[(x, y)].clone();
            }
        }

        new_grid
    }

    /// Rotate the entire grid 90 degrees counter-clockwise  
    pub fn rotate_counter_clockwise<T: Clone>(grid: &Grid<T>) -> Grid<T> {
        let new_width = grid.height();
        let new_height = grid.width();
        let mut new_grid = Grid::new(new_width, new_height, grid[(0, 0)].clone());

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let new_x = grid.height() - 1 - y;
                let new_y = x;
                new_grid[(new_x, new_y)] = grid[(x, y)].clone();
            }
        }

        new_grid
    }

    /// Mirror the grid horizontally
    pub fn mirror_horizontal<T: Clone>(grid: &Grid<T>) -> Grid<T> {
        let mut new_grid = grid.clone();

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let mirror_x = grid.width() - 1 - x;
                new_grid[(x, y)] = grid[(mirror_x, y)].clone();
            }
        }

        new_grid
    }

    /// Find cycles in grid states (useful for detecting patterns)
    ///
    /// Returns the cycle length and offset if a cycle is detected.
    pub fn detect_cycle<T: Clone + PartialEq + std::hash::Hash + Eq>(
        initial_grid: Grid<T>,
        step_function: fn(&Grid<T>) -> Grid<T>,
        max_steps: usize,
    ) -> Option<(usize, usize)> {
        // (cycle_start, cycle_length)
        let mut seen_states = HashMap::new();
        let mut current_grid = initial_grid;

        for step in 0..max_steps {
            if let Some(&first_seen) = seen_states.get(&current_grid) {
                // Found cycle
                let cycle_length = step - first_seen;
                return Some((first_seen, cycle_length));
            }

            seen_states.insert(current_grid.clone(), step);
            current_grid = step_function(&current_grid);
        }

        None
    }

    /// Calculate Manhattan distance between all pairs of specified coordinates
    ///
    /// Returns the sum of Manhattan distances between all pairs.
    pub fn total_manhattan_distance(coords: &[Coord]) -> usize {
        let mut total = 0;
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                total += coords[i].manhattan_distance(coords[j]);
            }
        }
        total
    }

    /// Find the coordinate that minimizes total Manhattan distance to all others
    ///
    /// Returns the coordinate that has minimum sum of Manhattan distances
    /// to all coordinates in the input slice.
    pub fn find_centroid(coords: &[Coord]) -> Option<Coord> {
        if coords.is_empty() {
            return None;
        }

        let mut best_coord = coords[0];
        let mut best_distance = usize::MAX;

        // Get bounds for search space
        let min_x = coords.iter().map(|c| c.x).min().unwrap();
        let max_x = coords.iter().map(|c| c.x).max().unwrap();
        let min_y = coords.iter().map(|c| c.y).min().unwrap();
        let max_y = coords.iter().map(|c| c.y).max().unwrap();

        // Check all points in bounding rectangle
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let candidate = Coord::new(x, y);
                let total_distance: usize = coords
                    .iter()
                    .map(|&coord| candidate.manhattan_distance(coord))
                    .sum();

                if total_distance < best_distance {
                    best_distance = total_distance;
                    best_coord = candidate;
                }
            }
        }

        Some(best_coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_char_grid() {
        let input = "..#\n.#.\n#..";
        let grid = AocGridParser::parse_char_grid(input);

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid[(2, 0)], '#');
        assert_eq!(grid[(1, 1)], '#');
        assert_eq!(grid[(0, 2)], '#');
    }

    #[test]
    fn test_parse_digit_grid() {
        let input = "123\n456\n789";
        let grid = AocGridParser::parse_digit_grid(input);

        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(1, 1)], 5);
        assert_eq!(grid[(2, 2)], 9);
    }

    #[test]
    fn test_parse_spaced_grid() {
        let input = "10 20 30\n40 50 60";
        let grid: Grid<i32> = AocGridParser::parse_spaced_grid(input);

        assert_eq!(grid[(1, 0)], 20);
        assert_eq!(grid[(2, 1)], 60);
    }

    #[test]
    fn test_find_all() {
        let mut grid = Grid::new(3, 3, '.');
        grid[(1, 1)] = '#';
        grid[(2, 2)] = '#';

        let positions = AocGridParser::find_all(&grid, &'#');
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&Coord::new(1, 1)));
        assert!(positions.contains(&Coord::new(2, 2)));
    }

    #[test]
    fn test_find_first() {
        let mut grid = Grid::new(3, 3, '.');
        grid[(2, 1)] = '#';
        grid[(1, 2)] = '#';

        let first = AocGridParser::find_first(&grid, &'#');
        assert_eq!(first, Some(Coord::new(2, 1))); // Should find top-left first
    }

    #[test]
    fn test_grid_to_string() {
        let mut grid = Grid::new(2, 2, '.');
        grid[(1, 0)] = '#';

        let display = GridVisualizer::to_string(&grid);
        assert_eq!(display, ".#\n..");
    }

    #[test]
    fn test_count_adjacent() {
        let mut grid = Grid::new(3, 3, '.');
        grid[(1, 0)] = '#';
        grid[(0, 1)] = '#';
        grid[(2, 1)] = '#';
        grid[(1, 2)] = '#';

        let count = AocAlgorithms::count_adjacent(&grid, Coord::new(1, 1), false, |&ch| ch == '#');
        assert_eq!(count, 4); // 4 cardinal neighbors
    }

    #[test]
    fn test_multi_source_bfs() {
        let grid = Grid::new(5, 5, '.');
        let sources = vec![Coord::new(0, 0), Coord::new(4, 4)];

        let distances = AocAlgorithms::multi_source_bfs(&grid, &sources, |&ch| ch == '.');

        // Center should be distance 4 from both corners
        assert_eq!(distances[&Coord::new(2, 2)].0, 4);
    }

    #[test]
    fn test_rotate_clockwise() {
        let mut grid = Grid::new(2, 3, '.');
        grid[(0, 0)] = 'A'; // Top-left
        grid[(1, 0)] = 'B'; // Top-right
        grid[(0, 1)] = 'C'; // Middle-left
        grid[(1, 1)] = 'D'; // Middle-right
        grid[(0, 2)] = 'E'; // Bottom-left
        grid[(1, 2)] = 'F'; // Bottom-right

        let rotated = AocAlgorithms::rotate_clockwise(&grid);

        // After 90° clockwise rotation: width=3, height=2
        assert_eq!(rotated.width(), 3);
        assert_eq!(rotated.height(), 2);

        // Original 2x3:        Rotated 3x2:
        // A B                  B D F
        // C D                  A C E
        // E F

        assert_eq!(rotated[(0, 0)], 'B'); // Top-right becomes top-left
        assert_eq!(rotated[(1, 0)], 'D'); // Middle-right becomes top-center
        assert_eq!(rotated[(2, 0)], 'F'); // Bottom-right becomes top-right
        assert_eq!(rotated[(0, 1)], 'A'); // Top-left becomes bottom-left
        assert_eq!(rotated[(1, 1)], 'C'); // Middle-left becomes bottom-center
        assert_eq!(rotated[(2, 1)], 'E'); // Bottom-left becomes bottom-right
    }

    #[test]
    fn test_total_manhattan_distance() {
        let coords = vec![Coord::new(0, 0), Coord::new(3, 0), Coord::new(0, 3)];

        let total = AocAlgorithms::total_manhattan_distance(&coords);
        // Distance (0,0)-(3,0) = 3, (0,0)-(0,3) = 3, (3,0)-(0,3) = 6
        assert_eq!(total, 12);
    }

    #[test]
    fn test_find_centroid() {
        let coords = vec![Coord::new(0, 0), Coord::new(4, 0), Coord::new(2, 3)];

        let centroid = AocAlgorithms::find_centroid(&coords).unwrap();

        // Should be somewhere around (2, 1) to minimize total distance
        assert!(centroid.x >= 1 && centroid.x <= 3);
        assert!(centroid.y <= 2); // y is usize, so always >= 0
    }

    #[test]
    fn test_detect_cycle() {
        // Simple cycle: toggle between two states
        fn toggle_grid(grid: &Grid<bool>) -> Grid<bool> {
            let mut new_grid = grid.clone();
            for (coord, &value) in grid.enumerate() {
                new_grid[coord] = !value;
            }
            new_grid
        }

        let initial = Grid::new(2, 2, true);
        let cycle = AocAlgorithms::detect_cycle(initial, toggle_grid, 10);

        assert!(cycle.is_some());
        let (start, length) = cycle.unwrap();
        assert_eq!(start, 0);
        assert_eq!(length, 2); // Toggles every 2 steps
    }
}
