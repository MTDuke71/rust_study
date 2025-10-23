//! Grid data structure for 2D spatial operations
//!
//! Provides a generic `Grid<T>` with safe bounds checking, efficient access patterns,
//! and iterator support optimized for competitive programming.

use crate::coord::Coord;
use std::fmt;
use std::ops::{Index, IndexMut};

/// Generic 2D grid with bounds checking and efficient access patterns
///
/// # Requirements Satisfied: REQ-1
///
/// The Grid uses row-major storage for cache-friendly access patterns.
/// All indexing operations include bounds checking for safety.
///
/// # Examples
///
/// ```rust
/// use mission6::{Grid, Coord};
///
/// let mut grid = Grid::new(3, 3, 0);
/// grid[(1, 1)] = 42;
///
/// let coord = Coord::new(1, 1);
/// assert_eq!(grid[coord], 42);
///
/// // Iterate over all cells
/// for (coord, value) in grid.enumerate() {
///     println!("Cell at {:?}: {}", coord, value);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

// Manual Hash implementation to handle generic T
impl<T: std::hash::Hash> std::hash::Hash for Grid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

impl<T: Clone> Grid<T> {
    /// Create a new grid with the given dimensions, filled with the default value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission6::Grid;
    ///
    /// let grid = Grid::new(5, 3, '.');
    /// assert_eq!(grid.width(), 5);
    /// assert_eq!(grid.height(), 3);
    /// ```
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        let data = vec![default_value; width * height];
        Grid {
            data,
            width,
            height,
        }
    }

    /// Create a grid from a 2D vector
    ///
    /// # Panics
    ///
    /// Panics if the input is empty or rows have different lengths
    pub fn from_vec2d(data: Vec<Vec<T>>) -> Self {
        if data.is_empty() || data[0].is_empty() {
            panic!("Cannot create grid from empty data");
        }

        let height = data.len();
        let width = data[0].len();

        // Verify all rows have the same length
        if !data.iter().all(|row| row.len() == width) {
            panic!("All rows must have the same length");
        }

        let flat_data: Vec<T> = data.into_iter().flatten().collect();

        Grid {
            data: flat_data,
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    /// Get the width of the grid
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the grid
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the total number of cells in the grid
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the grid is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Check if a coordinate is within bounds
    pub fn in_bounds(&self, coord: Coord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    /// Convert (x, y) coordinates to a linear index
    fn coord_to_index(&self, coord: Coord) -> usize {
        coord.y * self.width + coord.x
    }

    /// Get a reference to the cell at the given coordinate
    /// Returns None if out of bounds
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if self.in_bounds(coord) {
            let index = self.coord_to_index(coord);
            Some(&self.data[index])
        } else {
            None
        }
    }

    /// Get a mutable reference to the cell at the given coordinate
    /// Returns None if out of bounds
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if self.in_bounds(coord) {
            let index = self.coord_to_index(coord);
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    /// Iterator over all values in the grid
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    /// Mutable iterator over all values in the grid
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    /// Iterator over all coordinates in the grid
    pub fn coordinates(&self) -> CoordinateIterator {
        CoordinateIterator {
            width: self.width,
            height: self.height,
            current: 0,
        }
    }

    /// Iterator over (coordinate, value) pairs
    pub fn enumerate(&self) -> EnumerateIterator<'_, T> {
        EnumerateIterator {
            grid: self,
            coords: self.coordinates(),
        }
    }

    /// Iterator over (coordinate, mutable value) pairs
    pub fn enumerate_mut(&mut self) -> EnumerateMutIterator<'_, T> {
        EnumerateMutIterator {
            width: self.width,
            height: self.height,
            data: &mut self.data,
            current: 0,
        }
    }

    /// Get an iterator over a specific row
    pub fn row(&self, y: usize) -> Option<RowIterator<'_, T>> {
        if y < self.height {
            Some(RowIterator {
                grid: self,
                y,
                current_x: 0,
            })
        } else {
            None
        }
    }

    /// Get an iterator over a specific column
    pub fn column(&self, x: usize) -> Option<ColumnIterator<'_, T>> {
        if x < self.width {
            Some(ColumnIterator {
                grid: self,
                x,
                current_y: 0,
            })
        } else {
            None
        }
    }
}

// Indexing by (usize, usize)
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let coord = Coord::new(x, y);
        if !self.in_bounds(coord) {
            panic!(
                "Grid index ({}, {}) out of bounds ({}x{})",
                x, y, self.width, self.height
            );
        }
        let index = self.coord_to_index(coord);
        &self.data[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let coord = Coord::new(x, y);
        if !self.in_bounds(coord) {
            panic!(
                "Grid index ({}, {}) out of bounds ({}x{})",
                x, y, self.width, self.height
            );
        }
        let index = self.coord_to_index(coord);
        &mut self.data[index]
    }
}

// Indexing by Coord
impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, coord: Coord) -> &Self::Output {
        if !self.in_bounds(coord) {
            panic!(
                "Grid coord {:?} out of bounds ({}x{})",
                coord, self.width, self.height
            );
        }
        let index = self.coord_to_index(coord);
        &self.data[index]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        if !self.in_bounds(coord) {
            panic!(
                "Grid coord {:?} out of bounds ({}x{})",
                coord, self.width, self.height
            );
        }
        let index = self.coord_to_index(coord);
        &mut self.data[index]
    }
}

// Iterator implementations
pub struct CoordinateIterator {
    width: usize,
    height: usize,
    current: usize,
}

impl Iterator for CoordinateIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.width * self.height {
            let x = self.current % self.width;
            let y = self.current / self.width;
            self.current += 1;
            Some(Coord::new(x, y))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.width * self.height - self.current;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for CoordinateIterator {}

pub struct EnumerateIterator<'a, T> {
    grid: &'a Grid<T>,
    coords: CoordinateIterator,
}

impl<'a, T> Iterator for EnumerateIterator<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.coords.next().map(|coord| (coord, &self.grid[coord]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.coords.size_hint()
    }
}

impl<'a, T> ExactSizeIterator for EnumerateIterator<'a, T> {}

pub struct EnumerateMutIterator<'a, T> {
    width: usize,
    height: usize,
    data: &'a mut [T],
    current: usize,
}

impl<'a, T> Iterator for EnumerateMutIterator<'a, T> {
    type Item = (Coord, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.width * self.height {
            let x = self.current % self.width;
            let y = self.current / self.width;
            let coord = Coord::new(x, y);

            // This is safe because we're consuming elements one by one
            let data_ptr = self.data.as_mut_ptr();
            let value = unsafe { &mut *data_ptr.add(self.current) };

            self.current += 1;
            Some((coord, value))
        } else {
            None
        }
    }
}

pub struct RowIterator<'a, T> {
    grid: &'a Grid<T>,
    y: usize,
    current_x: usize,
}

impl<'a, T> Iterator for RowIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x < self.grid.width {
            let coord = Coord::new(self.current_x, self.y);
            self.current_x += 1;
            Some(&self.grid[coord])
        } else {
            None
        }
    }
}

pub struct ColumnIterator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    current_y: usize,
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y < self.grid.height {
            let coord = Coord::new(self.x, self.current_y);
            self.current_y += 1;
            Some(&self.grid[coord])
        } else {
            None
        }
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;
            }
            if y < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(3, 2, 'X');
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.len(), 6);
        assert_eq!(grid[(0, 0)], 'X');
        assert_eq!(grid[(2, 1)], 'X');
    }

    #[test]
    fn test_is_empty() {
        // Test empty grid (0x0)
        let empty_grid = Grid::new(0, 0, 0);
        assert!(empty_grid.is_empty());
        assert_eq!(empty_grid.len(), 0);

        // Test non-empty grid
        let non_empty_grid = Grid::new(2, 2, 0);
        assert!(!non_empty_grid.is_empty());
        assert_eq!(non_empty_grid.len(), 4);

        // Test grid with width or height = 0
        let zero_width = Grid::new(0, 5, 0);
        assert!(zero_width.is_empty());

        let zero_height = Grid::new(5, 0, 0);
        assert!(zero_height.is_empty());
    }

    #[test]
    fn test_get() {
        let mut grid = Grid::new(3, 2, 0);
        grid[(0, 0)] = 10;
        grid[(1, 0)] = 20;
        grid[(2, 1)] = 30;

        // Test valid gets
        assert_eq!(grid.get(Coord::new(0, 0)), Some(&10));
        assert_eq!(grid.get(Coord::new(1, 0)), Some(&20));
        assert_eq!(grid.get(Coord::new(2, 1)), Some(&30));
        assert_eq!(grid.get(Coord::new(0, 1)), Some(&0)); // Default value

        // Test out-of-bounds (should return None)
        assert_eq!(grid.get(Coord::new(3, 0)), None); // x out of bounds
        assert_eq!(grid.get(Coord::new(0, 2)), None); // y out of bounds
        assert_eq!(grid.get(Coord::new(3, 2)), None); // Both out of bounds
        assert_eq!(grid.get(Coord::new(100, 100)), None); // Way out of bounds
    }

    #[test]
    fn test_get_mut() {
        let mut grid = Grid::new(2, 2, 0);

        // Test valid get_mut and modify
        if let Some(cell) = grid.get_mut(Coord::new(0, 0)) {
            *cell = 42;
        }
        assert_eq!(grid[(0, 0)], 42);

        // Modify another cell
        if let Some(cell) = grid.get_mut(Coord::new(1, 1)) {
            *cell = 99;
        }
        assert_eq!(grid[(1, 1)], 99);

        // Test that original cells are preserved
        assert_eq!(grid[(1, 0)], 0);
        assert_eq!(grid[(0, 1)], 0);

        // Test out-of-bounds (should return None)
        assert!(grid.get_mut(Coord::new(2, 0)).is_none());
        assert!(grid.get_mut(Coord::new(0, 2)).is_none());
        assert!(grid.get_mut(Coord::new(2, 2)).is_none());

        // Verify we can chain operations
        if let Some(cell) = grid.get_mut(Coord::new(1, 0)) {
            *cell = 55;
        }
        assert_eq!(grid[(1, 0)], 55);
    }

    #[test]
    fn test_bounds_checking() {
        let grid = Grid::new(2, 2, 0);
        assert!(grid.in_bounds(Coord::new(0, 0)));
        assert!(grid.in_bounds(Coord::new(1, 1)));
        assert!(!grid.in_bounds(Coord::new(2, 1)));
        assert!(!grid.in_bounds(Coord::new(1, 2)));
    }

    #[test]
    fn test_indexing() {
        // This test explicitly exercises line 220 (let index = coord_to_index)
        let mut grid = Grid::new(3, 3, 0);

        // Test reading with tuple indexing (exercises line 220)
        grid[(1, 1)] = 42;
        assert_eq!(grid[(1, 1)], 42);

        // Test writing and reading various positions to ensure line 220 executes multiple times
        grid[(0, 0)] = 10;
        grid[(2, 2)] = 30;
        grid[(1, 2)] = 25;

        assert_eq!(grid[(0, 0)], 10); // Line 220 executed
        assert_eq!(grid[(2, 2)], 30); // Line 220 executed
        assert_eq!(grid[(1, 2)], 25); // Line 220 executed

        // Test with Coord indexing
        let coord = Coord::new(2, 0);
        grid[coord] = 99;
        assert_eq!(grid[coord], 99);

        // Test all corners to thoroughly exercise indexing
        grid[(0, 0)] = 1;
        grid[(2, 0)] = 2;
        grid[(0, 2)] = 3;
        grid[(2, 2)] = 4;

        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(2, 0)], 2);
        assert_eq!(grid[(0, 2)], 3);
        assert_eq!(grid[(2, 2)], 4);
    }

    #[test]
    fn test_iterators() {
        let mut grid = Grid::new(2, 2, 0);
        grid[(0, 0)] = 1;
        grid[(1, 0)] = 2;
        grid[(0, 1)] = 3;
        grid[(1, 1)] = 4;

        let coords: Vec<_> = grid.coordinates().collect();
        assert_eq!(coords.len(), 4);

        let values: Vec<_> = grid.iter().cloned().collect();
        assert_eq!(values, vec![1, 2, 3, 4]);

        let enumerated: Vec<_> = grid.enumerate().map(|(c, &v)| (c, v)).collect();
        assert_eq!(enumerated[0], (Coord::new(0, 0), 1));
        assert_eq!(enumerated[3], (Coord::new(1, 1), 4));
    }

    #[test]
    fn test_iter_mut() {
        let mut grid = Grid::new(2, 2, 0);

        // Modify all values using iter_mut
        for value in grid.iter_mut() {
            *value = 42;
        }

        // Verify all values were changed
        assert_eq!(grid[(0, 0)], 42);
        assert_eq!(grid[(1, 0)], 42);
        assert_eq!(grid[(0, 1)], 42);
        assert_eq!(grid[(1, 1)], 42);

        // Test with enumeration
        for (i, value) in grid.iter_mut().enumerate() {
            *value = i as i32;
        }

        assert_eq!(grid[(0, 0)], 0);
        assert_eq!(grid[(1, 0)], 1);
        assert_eq!(grid[(0, 1)], 2);
        assert_eq!(grid[(1, 1)], 3);
    }

    #[test]
    fn test_enumerate_mut() {
        let mut grid = Grid::new(2, 2, 0);

        // Modify values based on coordinates
        for (coord, value) in grid.enumerate_mut() {
            *value = (coord.x + coord.y * 10) as i32;
        }

        assert_eq!(grid[(0, 0)], 0); // 0 + 0*10 = 0
        assert_eq!(grid[(1, 0)], 1); // 1 + 0*10 = 1
        assert_eq!(grid[(0, 1)], 10); // 0 + 1*10 = 10
        assert_eq!(grid[(1, 1)], 11); // 1 + 1*10 = 11

        // Test that we can iterate twice
        let mut sum = 0;
        for (_, value) in grid.enumerate_mut() {
            sum += *value;
        }
        assert_eq!(sum, 1 + 10 + 11);
    }

    #[test]
    fn test_row_column_iteration() {
        let mut grid = Grid::new(3, 2, 0);
        grid[(0, 0)] = 1;
        grid[(1, 0)] = 2;
        grid[(2, 0)] = 3;
        grid[(0, 1)] = 4;
        grid[(1, 1)] = 5;
        grid[(2, 1)] = 6;

        // Test valid rows
        let row0: Vec<_> = grid.row(0).unwrap().cloned().collect();
        assert_eq!(row0, vec![1, 2, 3]);

        let row1: Vec<_> = grid.row(1).unwrap().cloned().collect();
        assert_eq!(row1, vec![4, 5, 6]);

        // Test valid columns
        let col0: Vec<_> = grid.column(0).unwrap().cloned().collect();
        assert_eq!(col0, vec![1, 4]);

        let col1: Vec<_> = grid.column(1).unwrap().cloned().collect();
        assert_eq!(col1, vec![2, 5]);

        let col2: Vec<_> = grid.column(2).unwrap().cloned().collect();
        assert_eq!(col2, vec![3, 6]);

        // Test out-of-bounds (these should return None)
        assert!(grid.row(2).is_none()); // Only 2 rows (0, 1)
        assert!(grid.row(100).is_none());

        assert!(grid.column(3).is_none()); // Only 3 columns (0, 1, 2)
        assert!(grid.column(100).is_none());
    }

    #[test]
    fn test_from_vec2d() {
        // This test explicitly exercises lines 80-81 (let height/width statements)
        let data = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let grid = Grid::from_vec2d(data);

        // Verify the let statements on lines 80-81 worked correctly
        assert_eq!(grid.width(), 3); // width = data[0].len() from line 81
        assert_eq!(grid.height(), 2); // height = data.len() from line 80

        // Verify data was flattened correctly
        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(2, 1)], 6);

        // Test multiple sizes to ensure let statements work for various dimensions
        let large_data = vec![
            vec![10, 20, 30, 40],
            vec![50, 60, 70, 80],
            vec![90, 100, 110, 120],
        ];
        let large_grid = Grid::from_vec2d(large_data);
        assert_eq!(large_grid.width(), 4); // Tests line 81 again
        assert_eq!(large_grid.height(), 3); // Tests line 80 again
    }

    #[test]
    fn test_from_vec2d_dimensions() {
        // Explicitly test lines 80-81 with various sizes
        // Line 80: let height = data.len();
        // Line 81: let width = data[0].len();

        // 1x1 grid
        let tiny = Grid::from_vec2d(vec![vec![42]]);
        assert_eq!(tiny.height(), 1); // Line 80
        assert_eq!(tiny.width(), 1); // Line 81
        assert_eq!(tiny[(0, 0)], 42);

        // Wide grid (1x5)
        let wide = Grid::from_vec2d(vec![vec![1, 2, 3, 4, 5]]);
        assert_eq!(wide.height(), 1); // Line 80: data.len() = 1
        assert_eq!(wide.width(), 5); // Line 81: data[0].len() = 5

        // Tall grid (5x1)
        let tall = Grid::from_vec2d(vec![vec![1], vec![2], vec![3], vec![4], vec![5]]);
        assert_eq!(tall.height(), 5); // Line 80: data.len() = 5
        assert_eq!(tall.width(), 1); // Line 81: data[0].len() = 1

        // Square grid (4x4)
        let square = Grid::from_vec2d(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]);
        assert_eq!(square.height(), 4); // Line 80: data.len() = 4
        assert_eq!(square.width(), 4); // Line 81: data[0].len() = 4
    }

    #[test]
    #[should_panic(expected = "Cannot create grid from empty data")]
    fn test_from_vec2d_panics_on_empty_outer() {
        // Test panic on line 77 - empty outer vector
        let empty_data: Vec<Vec<i32>> = vec![];
        let _grid: Grid<i32> = Grid::from_vec2d(empty_data);
    }

    #[test]
    #[should_panic(expected = "Cannot create grid from empty data")]
    fn test_from_vec2d_panics_on_empty_inner() {
        // Test panic on line 77 - empty inner vector
        let empty_data: Vec<Vec<i32>> = vec![vec![]];
        let _grid: Grid<i32> = Grid::from_vec2d(empty_data);
    }

    #[test]
    #[should_panic(expected = "All rows must have the same length")]
    fn test_from_vec2d_panics_on_unequal_rows() {
        // Test panic on line 85 - unequal row lengths
        let bad_data = vec![
            vec![1, 2, 3],
            vec![4, 5], // Different length!
            vec![6, 7, 8],
        ];
        let _grid = Grid::from_vec2d(bad_data);
    }

    #[test]
    #[should_panic(expected = "Grid index (5, 0) out of bounds")]
    fn test_index_tuple_panics_on_out_of_bounds() {
        // Test panic on line 222 - Index<(usize, usize)> out of bounds
        let grid = Grid::new(3, 3, 0);
        let _value = grid[(5, 0)]; // x out of bounds
    }

    #[test]
    #[should_panic(expected = "Grid index (0, 5) out of bounds")]
    fn test_index_mut_tuple_panics_on_out_of_bounds() {
        // Test panic on line 233 - IndexMut<(usize, usize)> out of bounds
        let mut grid = Grid::new(3, 3, 0);
        grid[(0, 5)] = 42; // y out of bounds
    }

    #[test]
    fn test_display_formatting() {
        // Test line 380 - Display implementation with formatting
        let mut grid = Grid::new(3, 2, 0);
        grid[(0, 0)] = 1;
        grid[(1, 0)] = 2;
        grid[(2, 0)] = 3;
        grid[(0, 1)] = 4;
        grid[(1, 1)] = 5;
        grid[(2, 1)] = 6;

        let display = format!("{}", grid);
        assert_eq!(display, "123\n456");
    }

    #[test]
    fn test_display_single_row() {
        // Test Display with single row (no newline)
        let mut grid = Grid::new(3, 1, 0);
        grid[(0, 0)] = 7;
        grid[(1, 0)] = 8;
        grid[(2, 0)] = 9;

        let display = format!("{}", grid);
        assert_eq!(display, "789");
    }
}
