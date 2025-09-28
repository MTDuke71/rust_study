//! Grid data structure for 2D spatial operations
//! 
//! Provides a generic Grid<T> with safe bounds checking, efficient access patterns,
//! and iterator support optimized for competitive programming.

use crate::coord::Coord;
use std::ops::{Index, IndexMut};
use std::fmt;

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
            panic!("Grid index ({}, {}) out of bounds ({}x{})", x, y, self.width, self.height);
        }
        let index = self.coord_to_index(coord);
        &self.data[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let coord = Coord::new(x, y);
        if !self.in_bounds(coord) {
            panic!("Grid index ({}, {}) out of bounds ({}x{})", x, y, self.width, self.height);
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
            panic!("Grid coord {:?} out of bounds ({}x{})", coord, self.width, self.height);
        }
        let index = self.coord_to_index(coord);
        &self.data[index]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        if !self.in_bounds(coord) {
            panic!("Grid coord {:?} out of bounds ({}x{})", coord, self.width, self.height);
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
    fn test_bounds_checking() {
        let grid = Grid::new(2, 2, 0);
        assert!(grid.in_bounds(Coord::new(0, 0)));
        assert!(grid.in_bounds(Coord::new(1, 1)));
        assert!(!grid.in_bounds(Coord::new(2, 1)));
        assert!(!grid.in_bounds(Coord::new(1, 2)));
    }

    #[test]
    fn test_indexing() {
        let mut grid = Grid::new(3, 3, 0);
        grid[(1, 1)] = 42;
        assert_eq!(grid[(1, 1)], 42);
        
        let coord = Coord::new(2, 0);
        grid[coord] = 99;
        assert_eq!(grid[coord], 99);
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
    fn test_row_column_iteration() {
        let mut grid = Grid::new(3, 2, 0);
        grid[(0, 0)] = 1; grid[(1, 0)] = 2; grid[(2, 0)] = 3;
        grid[(0, 1)] = 4; grid[(1, 1)] = 5; grid[(2, 1)] = 6;

        let row0: Vec<_> = grid.row(0).unwrap().cloned().collect();
        assert_eq!(row0, vec![1, 2, 3]);

        let col1: Vec<_> = grid.column(1).unwrap().cloned().collect();
        assert_eq!(col1, vec![2, 5]);
    }

    #[test]
    fn test_from_vec2d() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        
        let grid = Grid::from_vec2d(data);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(2, 1)], 6);
    }
}