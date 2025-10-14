//! # Grid Pattern Recognition Module
//!
//! Recognizes and solves the most common AoC grid-based problems:
//! - 2D navigation and pathfinding  
//! - Flood fill algorithms
//! - Grid transformations and rotations
//! - Cellular automata patterns
//!
//! ## Requirements Satisfied: REQ-P1
//!
//! ### Common AoC Grid Problem Types:
//! 1. **Shortest Path**: BFS/Dijkstra on grids (AoC 2021 Day 15, 2019 Day 20)
//! 2. **Flood Fill**: Connected regions (AoC 2017 Day 14, 2020 Day 20)
//! 3. **Grid Simulation**: Conway's Game of Life variants (AoC 2020 Day 17, 2015 Day 18)
//! 4. **Grid Parsing**: ASCII art to data structures (AoC 2022 Day 8, 2018 Day 13)

use crate::{AocPattern, PatternComplexity, PatternError, PatternResult};
use std::collections::{HashMap, HashSet, VecDeque};

/// 2D coordinate system for grid navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Manhattan distance between two coordinates
    pub fn manhattan_distance(&self, other: &Coord) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    /// Get all 4-directional neighbors
    pub fn neighbors_4(&self) -> [Coord; 4] {
        [
            Coord::new(self.x + 1, self.y), // Right
            Coord::new(self.x - 1, self.y), // Left
            Coord::new(self.x, self.y + 1), // Down
            Coord::new(self.x, self.y - 1), // Up
        ]
    }

    /// Get all 8-directional neighbors (including diagonals)
    pub fn neighbors_8(&self) -> [Coord; 8] {
        [
            Coord::new(self.x + 1, self.y),     // Right
            Coord::new(self.x - 1, self.y),     // Left
            Coord::new(self.x, self.y + 1),     // Down
            Coord::new(self.x, self.y - 1),     // Up
            Coord::new(self.x + 1, self.y + 1), // Down-Right
            Coord::new(self.x + 1, self.y - 1), // Up-Right
            Coord::new(self.x - 1, self.y + 1), // Down-Left
            Coord::new(self.x - 1, self.y - 1), // Up-Left
        ]
    }
}

/// Generic grid for AoC problems
#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub cells: HashMap<Coord, T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: Clone + PartialEq,
{
    /// Create empty grid with specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: HashMap::new(),
            width,
            height,
        }
    }

    /// Get cell value at coordinate
    pub fn get(&self, coord: &Coord) -> Option<&T> {
        self.cells.get(coord)
    }

    /// Set cell value at coordinate
    pub fn set(&mut self, coord: Coord, value: T) {
        self.cells.insert(coord, value);
    }

    /// Check if coordinate is within grid bounds
    pub fn in_bounds(&self, coord: &Coord) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && (coord.x as usize) < self.width
            && (coord.y as usize) < self.height
    }

    /// Find all coordinates containing a specific value
    pub fn find_all(&self, target: &T) -> Vec<Coord> {
        self.cells
            .iter()
            .filter_map(
                |(coord, value)| {
                    if value == target {
                        Some(*coord)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    /// Count cells matching a value
    pub fn count(&self, target: &T) -> usize {
        self.cells.values().filter(|&v| v == target).count()
    }
}

impl Grid<char> {
    /// Create grid from multiline string (common AoC input format)
    ///
    /// # Requirements Satisfied: REQ-P1
    /// Handles the most common AoC input pattern
    ///
    /// # Example
    /// ```
    /// use aoc_pattern_recognition::grid_patterns::Grid;
    ///
    /// let input = "..#.\n.#..\n..#.";
    /// let grid = Grid::from_string(input);
    /// assert_eq!(grid.width, 4);
    /// assert_eq!(grid.height, 3);
    /// ```
    pub fn from_string(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.first().map(|line| line.len()).unwrap_or(0);

        let mut grid = Grid::new(width, height);

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                grid.set(Coord::new(x as i32, y as i32), ch);
            }
        }

        grid
    }

    /// Convert grid back to string representation
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                let ch = self.get(&coord).unwrap_or(&' ');
                result.push(*ch);
            }
            if y < self.height - 1 {
                result.push('\n');
            }
        }

        result
    }
}

/// Shortest path pattern for grid navigation
///
/// # Requirements Satisfied: REQ-P1, REQ-P4
/// Implements BFS for unweighted grids, ready for Dijkstra extension
pub struct ShortestPathPattern;

impl AocPattern for ShortestPathPattern {
    type Input = (Grid<char>, Coord, Coord, fn(char) -> bool);
    type Output = Option<Vec<Coord>>;

    fn matches(&self, _input: &Self::Input) -> bool {
        // Pattern matching logic would analyze if this is a shortest path problem
        true
    }

    /// Find shortest path using BFS
    ///
    /// # Arguments
    /// * `grid` - The grid to navigate
    /// * `start` - Starting coordinate
    /// * `end` - Target coordinate  
    /// * `walkable` - Function that returns true if a cell can be traversed
    fn solve(&self, input: Self::Input) -> PatternResult<Self::Output> {
        let (grid, start, end, walkable) = input;

        if !grid.in_bounds(&start) || !grid.in_bounds(&end) {
            return Err(PatternError::InvalidInput(
                "Start or end position out of bounds".to_string(),
            ));
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = end;

                while let Some(&prev) = parent.get(&node) {
                    path.push(node);
                    node = prev;
                }
                path.push(start);
                path.reverse();

                return Ok(Some(path));
            }

            for neighbor in current.neighbors_4() {
                if !grid.in_bounds(&neighbor) || visited.contains(&neighbor) {
                    continue;
                }

                if let Some(cell) = grid.get(&neighbor) {
                    if walkable(*cell) {
                        visited.insert(neighbor);
                        parent.insert(neighbor, current);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        Ok(None) // No path found
    }

    fn complexity(&self) -> PatternComplexity {
        PatternComplexity::Linear // O(V + E) where V is cells, E is edges
    }

    fn pattern_name(&self) -> &'static str {
        "Grid Shortest Path (BFS)"
    }
}

/// Flood fill pattern for connected regions
///
/// # Requirements Satisfied: REQ-P1
/// Classic connected component analysis
pub struct FloodFillPattern;

impl AocPattern for FloodFillPattern {
    type Input = (Grid<char>, Coord, char);
    type Output = HashSet<Coord>;

    fn matches(&self, _input: &Self::Input) -> bool {
        true
    }

    /// Flood fill starting from a coordinate
    ///
    /// # Requirements Satisfied: REQ-P1
    /// Finds all connected cells of the same type
    fn solve(&self, input: Self::Input) -> PatternResult<Self::Output> {
        let (grid, start, target_char) = input;

        if !grid.in_bounds(&start) {
            return Err(PatternError::InvalidInput(
                "Start position out of bounds".to_string(),
            ));
        }

        let start_char = grid
            .get(&start)
            .ok_or_else(|| PatternError::InvalidInput("No cell at start position".to_string()))?;

        if *start_char != target_char {
            return Ok(HashSet::new());
        }

        let mut filled = HashSet::new();
        let mut stack = Vec::new();

        stack.push(start);

        while let Some(current) = stack.pop() {
            if filled.contains(&current) {
                continue;
            }

            if let Some(cell) = grid.get(&current) {
                if *cell == target_char {
                    filled.insert(current);

                    for neighbor in current.neighbors_4() {
                        if grid.in_bounds(&neighbor) && !filled.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }

        Ok(filled)
    }

    fn complexity(&self) -> PatternComplexity {
        PatternComplexity::Linear // O(n) where n is the size of the filled region
    }

    fn pattern_name(&self) -> &'static str {
        "Flood Fill Connected Region"
    }
}

/// Grid rotation pattern for transformations
///
/// # Requirements Satisfied: REQ-P1
/// Common in AoC puzzles involving piece manipulation
pub struct GridRotationPattern;

impl GridRotationPattern {
    /// Rotate grid 90 degrees clockwise
    ///
    /// # Requirements Satisfied: REQ-P1
    /// Standard grid transformation needed in many AoC problems
    pub fn rotate_clockwise<T: Clone + PartialEq>(&self, grid: &Grid<T>) -> Grid<T> {
        let mut rotated = Grid::new(grid.height, grid.width);

        for (coord, value) in &grid.cells {
            let new_coord = Coord::new((grid.height as i32 - 1) - coord.y, coord.x);
            rotated.set(new_coord, value.clone());
        }

        rotated
    }

    /// Rotate grid 90 degrees counter-clockwise
    pub fn rotate_counter_clockwise<T: Clone + PartialEq>(&self, grid: &Grid<T>) -> Grid<T> {
        let mut rotated = Grid::new(grid.height, grid.width);

        for (coord, value) in &grid.cells {
            let new_coord = Coord::new(coord.y, (grid.width as i32 - 1) - coord.x);
            rotated.set(new_coord, value.clone());
        }

        rotated
    }

    /// Flip grid horizontally
    pub fn flip_horizontal<T: Clone + PartialEq>(&self, grid: &Grid<T>) -> Grid<T> {
        let mut flipped = Grid::new(grid.width, grid.height);

        for (coord, value) in &grid.cells {
            let new_coord = Coord::new((grid.width as i32 - 1) - coord.x, coord.y);
            flipped.set(new_coord, value.clone());
        }

        flipped
    }
}

/// Helper functions for common grid operations
pub mod grid_utils {
    use super::*;

    /// Find shortest distance between two points (Manhattan distance)
    ///
    /// # Requirements Satisfied: REQ-P4
    /// Fast distance calculation for grid heuristics
    pub fn manhattan_distance(a: Coord, b: Coord) -> u32 {
        a.manhattan_distance(&b)
    }

    /// Check if a coordinate is a corner of the grid
    pub fn is_corner(coord: Coord, width: usize, height: usize) -> bool {
        let corners = [
            Coord::new(0, 0),
            Coord::new((width - 1) as i32, 0),
            Coord::new(0, (height - 1) as i32),
            Coord::new((width - 1) as i32, (height - 1) as i32),
        ];
        corners.contains(&coord)
    }

    /// Check if a coordinate is on the edge of the grid
    pub fn is_edge(coord: Coord, width: usize, height: usize) -> bool {
        coord.x == 0
            || coord.y == 0
            || coord.x == (width - 1) as i32
            || coord.y == (height - 1) as i32
    }

    /// Get all coordinates in a grid
    pub fn all_coordinates(width: usize, height: usize) -> Vec<Coord> {
        let mut coords = Vec::new();
        for y in 0..height {
            for x in 0..width {
                coords.push(Coord::new(x as i32, y as i32));
            }
        }
        coords
    }
}
