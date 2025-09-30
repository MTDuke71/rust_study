//! Flood fill algorithms for 2D grids
//! 
//! Provides efficient flood fill implementations for region detection,
//! area calculation, and grid modifications.

use crate::{Grid, Coord};
use std::collections::{VecDeque, HashSet};

/// Result of a flood fill operation
#[derive(Debug, Clone)]
pub struct FloodFillResult {
    pub filled_coords: Vec<Coord>,
    pub area: usize,
    pub perimeter: usize,
}

/// Flood fill algorithms for 2D grids
/// 
/// # Requirements Satisfied: REQ-4
/// 
/// Provides flood fill algorithms for region detection and modification,
/// supporting both 4-connected and 8-connected neighborhoods.
pub struct FloodFill;

impl FloodFill {
    /// Fill a region starting from a coordinate with 4-connectivity
    /// 
    /// Performs flood fill using 4-directional connectivity (cardinal directions only).
    /// Returns all coordinates that were filled.
    /// 
    /// # Arguments
    /// 
    /// * `grid` - Mutable reference to the grid to modify
    /// * `start` - Starting coordinate for the fill
    /// * `target_value` - Value to match for filling (original value at start)
    /// * `fill_value` - Value to fill matching cells with
    /// 
    /// # Returns
    /// 
    /// Vector of all coordinates that were modified during the fill operation.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission6::{Grid, Coord, FloodFill};
    /// 
    /// let mut grid = Grid::new(3, 3, '.');
    /// let start = Coord::new(1, 1);
    /// 
    /// let filled = FloodFill::fill_4(&mut grid, start, '.', '#');
    /// assert_eq!(filled.len(), 9); // All cells filled
    /// ```
    pub fn fill_4<T: Clone + PartialEq>(
        grid: &mut Grid<T>,
        start: Coord,
        target_value: T,
        fill_value: T,
    ) -> Vec<Coord> {
        if !grid.in_bounds(start) || grid[start] != target_value || target_value == fill_value {
            return Vec::new();
        }

        let mut filled = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if grid[current] == target_value {
                grid[current] = fill_value.clone();
                filled.push(current);

                // Add neighbors to queue
                for neighbor in current.neighbors_4_bounded(grid.width(), grid.height()) {
                    if !visited.contains(&neighbor) && grid[neighbor] == target_value {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        filled
    }

    /// Fill a region starting from a coordinate with 8-connectivity
    /// 
    /// Performs flood fill using 8-directional connectivity (including diagonals).
    pub fn fill_8<T: Clone + PartialEq>(
        grid: &mut Grid<T>,
        start: Coord,
        target_value: T,
        fill_value: T,
    ) -> Vec<Coord> {
        if !grid.in_bounds(start) || grid[start] != target_value || target_value == fill_value {
            return Vec::new();
        }

        let mut filled = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if grid[current] == target_value {
                grid[current] = fill_value.clone();
                filled.push(current);

                // Add neighbors to queue (8-connected)
                for neighbor in current.neighbors_8_bounded(grid.width(), grid.height()) {
                    if !visited.contains(&neighbor) && grid[neighbor] == target_value {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        filled
    }

    /// Find all coordinates in a connected region without modifying the grid
    /// 
    /// Returns all coordinates that would be affected by a flood fill operation
    /// without actually modifying the grid.
    pub fn find_region_4<T: PartialEq>(
        grid: &Grid<T>,
        start: Coord,
        target_value: &T,
    ) -> Vec<Coord> {
        if !grid.in_bounds(start) || &grid[start] != target_value {
            return Vec::new();
        }

        let mut region = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            region.push(current);

            for neighbor in current.neighbors_4_bounded(grid.width(), grid.height()) {
                if !visited.contains(&neighbor) && &grid[neighbor] == target_value {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        region
    }

    /// Find all coordinates in a connected region with 8-connectivity
    pub fn find_region_8<T: PartialEq>(
        grid: &Grid<T>,
        start: Coord,
        target_value: &T,
    ) -> Vec<Coord> {
        if !grid.in_bounds(start) || &grid[start] != target_value {
            return Vec::new();
        }

        let mut region = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            region.push(current);

            for neighbor in current.neighbors_8_bounded(grid.width(), grid.height()) {
                if !visited.contains(&neighbor) && &grid[neighbor] == target_value {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        region
    }

    /// Calculate area and perimeter of a region
    /// 
    /// Returns detailed information about a connected region including
    /// its area (number of cells) and perimeter (number of boundary edges).
    pub fn analyze_region_4<T: PartialEq>(
        grid: &Grid<T>,
        start: Coord,
        target_value: &T,
    ) -> FloodFillResult {
        let region = Self::find_region_4(grid, start, target_value);
        let area = region.len();
        
        // Calculate perimeter by counting boundary edges
        let mut perimeter = 0;
        let region_set: HashSet<_> = region.iter().copied().collect();

        for &coord in &region {
            for neighbor in coord.neighbors_4_bounded(grid.width(), grid.height()) {
                if !region_set.contains(&neighbor) {
                    perimeter += 1;
                }
            }
            
            // Also count edges at grid boundaries
            if coord.x == 0 { perimeter += 1; }
            if coord.y == 0 { perimeter += 1; }
            if coord.x == grid.width() - 1 { perimeter += 1; }
            if coord.y == grid.height() - 1 { perimeter += 1; }
        }

        FloodFillResult {
            filled_coords: region,
            area,
            perimeter,
        }
    }

    /// Find all distinct regions in the grid
    /// 
    /// Returns a vector of all distinct connected regions found in the grid.
    /// Each region is represented as a vector of coordinates.
    pub fn find_all_regions<T: PartialEq + Clone>(
        grid: &Grid<T>,
        target_value: &T,
    ) -> Vec<Vec<Coord>> {
        let mut regions = Vec::new();
        let mut visited = HashSet::new();

        for coord in grid.enumerate() {
            if !visited.contains(&coord.0) && &grid[coord.0] == target_value {
                let region = Self::find_region_4(grid, coord.0, target_value);
                
                // Mark all coordinates in this region as visited
                for &c in &region {
                    visited.insert(c);
                }
                
                if !region.is_empty() {
                    regions.push(region);
                }
            }
        }

        regions
    }

    /// Replace all occurrences of a value in connected regions
    /// 
    /// Finds all connected regions containing the target value and replaces
    /// them with the replacement value. Returns the number of regions modified.
    pub fn replace_regions<T: PartialEq + Clone>(
        grid: &mut Grid<T>,
        target_value: T,
        replacement_value: T,
    ) -> usize {
        if target_value == replacement_value {
            return 0;
        }

        let regions = Self::find_all_regions(grid, &target_value);
        let region_count = regions.len();

        for region in regions {
            for coord in region {
                grid[coord] = replacement_value.clone();
            }
        }

        region_count
    }

    /// Count the number of distinct regions with the specified value
    /// 
    /// Returns the number of separate connected components containing
    /// the target value.
    pub fn count_regions<T: PartialEq + Clone>(grid: &Grid<T>, target_value: &T) -> usize {
        Self::find_all_regions(grid, target_value).len()
    }

    /// Find the largest region with the specified value
    /// 
    /// Returns the coordinates of the largest connected region, or None
    /// if no regions are found.
    pub fn find_largest_region<T: PartialEq + Clone>(
        grid: &Grid<T>,
        target_value: &T,
    ) -> Option<Vec<Coord>> {
        Self::find_all_regions(grid, target_value)
            .into_iter()
            .max_by_key(|region| region.len())
    }

    /// Fill holes in a region (flood fill background that's surrounded)
    /// 
    /// Identifies and fills "holes" in a region - areas of background_value
    /// that are completely surrounded by foreground_value.
    pub fn fill_holes<T: PartialEq + Clone>(
        grid: &mut Grid<T>,
        _foreground_value: &T,
        background_value: T,
        fill_value: T,
    ) -> usize {
        let mut holes_filled = 0;
        let mut visited = HashSet::new();
        
        // Collect coordinates first to avoid borrowing issues
        let coords_to_check: Vec<_> = grid.enumerate().map(|(coord, _)| coord).collect();

        for coord in coords_to_check {
            if !visited.contains(&coord) && grid[coord] == background_value {
                let region = Self::find_region_4(grid, coord, &background_value);
                
                // Check if this region touches the boundary
                let touches_boundary = region.iter().any(|&c| {
                    c.x == 0 
                    || c.y == 0 
                    || c.x == grid.width() - 1
                    || c.y == grid.height() - 1
                });

                // Mark all coordinates as visited
                for &c in &region {
                    visited.insert(c);
                }

                // If doesn't touch boundary, it's a hole - fill it
                if !touches_boundary && !region.is_empty() {
                    for coord in region {
                        grid[coord] = fill_value.clone();
                    }
                    holes_filled += 1;
                }
            }
        }

        holes_filled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_4_simple() {
        let mut grid = Grid::new(3, 3, '.');
        let start = Coord::new(1, 1);

        let filled = FloodFill::fill_4(&mut grid, start, '.', '#');
        
        assert_eq!(filled.len(), 9); // All cells should be filled
        
        // Verify all cells are now '#'
        for coord in grid.enumerate() {
            assert_eq!(grid[coord.0], '#');
        }
    }

    #[test]
    fn test_fill_4_with_obstacles() {
        let mut grid = Grid::new(3, 3, '.');
        grid[(1, 1)] = '#'; // Obstacle in center
        
        let start = Coord::new(0, 0);
        let filled = FloodFill::fill_4(&mut grid, start, '.', 'X');
        
        // Should fill all '.' cells but not the '#'
        assert_eq!(filled.len(), 8);
        assert_eq!(grid[(1, 1)], '#'); // Obstacle unchanged
        
        for coord in filled {
            assert_eq!(grid[coord], 'X');
        }
    }

    #[test]
    fn test_fill_8_vs_fill_4() {
        let mut grid1 = Grid::new(3, 3, '.');
        let mut grid2 = grid1.clone();
        
        // Create a barrier that 4-connected can go around but tests both approaches
        grid1[(1, 1)] = '#'; // Center obstacle
        grid2[(1, 1)] = '#';

        let start = Coord::new(0, 0);
        let filled_4 = FloodFill::fill_4(&mut grid1, start, '.', 'X');
        let filled_8 = FloodFill::fill_8(&mut grid2, start, '.', 'Y');

        // Both should fill the same cells since there's a path around the center obstacle
        assert_eq!(filled_4.len(), filled_8.len());
        assert_eq!(filled_4.len(), 8); // 9 cells - 1 obstacle
    }

    #[test]
    fn test_find_region() {
        let mut grid = Grid::new(3, 3, '.');
        grid[(1, 1)] = '#';
        
        let region = FloodFill::find_region_4(&grid, Coord::new(0, 0), &'.');
        assert_eq!(region.len(), 8); // All '.' cells
        
        // Grid should be unchanged
        for coord in grid.enumerate() {
            let expected = if coord.0 == Coord::new(1, 1) { '#' } else { '.' };
            assert_eq!(grid[coord.0], expected);
        }
    }

    #[test]
    fn test_analyze_region() {
        let mut grid = Grid::new(3, 3, '.');
        grid[(1, 1)] = '#'; // Center obstacle
        
        let result = FloodFill::analyze_region_4(&grid, Coord::new(0, 0), &'.');
        
        assert_eq!(result.area, 8); // 8 '.' cells
        assert!(result.perimeter > 0); // Should have some perimeter
    }

    #[test]
    fn test_find_all_regions() {
        let mut grid = Grid::new(5, 5, '.');
        
        // Create two separate regions of '#'
        grid[(1, 1)] = '#';
        grid[(1, 2)] = '#';
        
        grid[(3, 3)] = '#';
        grid[(3, 4)] = '#';
        
        let regions = FloodFill::find_all_regions(&grid, &'#');
        assert_eq!(regions.len(), 2);
        
        // Each region should have 2 cells
        assert_eq!(regions[0].len(), 2);
        assert_eq!(regions[1].len(), 2);
    }

    #[test]
    fn test_count_regions() {
        let mut grid = Grid::new(5, 5, '.');
        
        // Create isolated regions
        grid[(0, 0)] = '#';
        grid[(2, 2)] = '#';
        grid[(4, 4)] = '#';
        
        assert_eq!(FloodFill::count_regions(&grid, &'#'), 3);
        assert_eq!(FloodFill::count_regions(&grid, &'.'), 1); // Background is connected
    }

    #[test]
    fn test_find_largest_region() {
        let mut grid = Grid::new(5, 5, '.');
        
        // Create regions of different sizes
        grid[(0, 0)] = '#'; // Size 1
        
        grid[(2, 0)] = '#'; // Size 2
        grid[(3, 0)] = '#';
        
        grid[(1, 2)] = '#'; // Size 3
        grid[(1, 3)] = '#';
        grid[(2, 2)] = '#';
        
        let largest = FloodFill::find_largest_region(&grid, &'#').unwrap();
        assert_eq!(largest.len(), 3);
    }

    #[test]
    fn test_replace_regions() {
        let mut grid = Grid::new(3, 3, '.');
        grid[(1, 1)] = '#';
        
        let regions_modified = FloodFill::replace_regions(&mut grid, '.', 'X');
        assert_eq!(regions_modified, 1); // One region of '.' was replaced
        
        // Check that '.' was replaced with 'X'
        for coord in grid.enumerate() {
            let expected = if coord.0 == Coord::new(1, 1) { '#' } else { 'X' };
            assert_eq!(grid[coord.0], expected);
        }
    }

    #[test]
    fn test_fill_holes() {
        let mut grid = Grid::new(5, 5, '#');
        
        // Create a hole in the middle
        grid[(2, 2)] = '.';
        
        let holes_filled = FloodFill::fill_holes(&mut grid, &'#', '.', 'X');
        assert_eq!(holes_filled, 1);
        assert_eq!(grid[(2, 2)], 'X');
        
        // Create a region that touches boundary (not a hole)
        grid[(0, 0)] = '.';
        let holes_filled = FloodFill::fill_holes(&mut grid, &'#', '.', 'Y');
        assert_eq!(holes_filled, 0); // No holes filled
        assert_eq!(grid[(0, 0)], '.'); // Should remain unchanged
    }

    #[test]
    fn test_no_change_when_same_values() {
        let mut grid = Grid::new(3, 3, '.');
        let filled = FloodFill::fill_4(&mut grid, Coord::new(0, 0), '.', '.');
        
        assert!(filled.is_empty());
    }

    #[test]
    fn test_out_of_bounds_start() {
        let mut grid = Grid::new(3, 3, '.');
        let filled = FloodFill::fill_4(&mut grid, Coord::new(5, 5), '.', '#');
        
        assert!(filled.is_empty());
    }
}