//! Coordinate system for 2D grid navigation
//! 
//! Provides the Coord type for representing positions in 2D space with
//! arithmetic operations, distance calculations, and direction-based movement.

use crate::direction::Direction;
use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::fmt;
use std::hash::{Hash, Hasher};

/// A coordinate in 2D space with x (column) and y (row) components
/// 
/// # Requirements Satisfied: REQ-2
/// 
/// Coordinates use a standard (x, y) system where:
/// - x increases moving right (East)
/// - y increases moving down (South)
/// 
/// This matches common screen/grid conventions and AoC problem formats.
/// 
/// # Examples
/// 
/// ```rust
/// use mission6::{Coord, Direction};
/// 
/// let start = Coord::new(5, 10);
/// let end = start.move_in_direction(Direction::North, 3);
/// 
/// assert_eq!(end, Coord::new(5, 7)); // North decreases y
/// assert_eq!(start.manhattan_distance(end), 3);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    /// Create a new coordinate
    pub fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    /// Create a coordinate at the origin (0, 0)
    pub fn origin() -> Self {
        Coord::new(0, 0)
    }

    /// Move in the given direction by the specified distance
    /// 
    /// Returns None if the movement would result in underflow (negative coordinates)
    pub fn try_move_in_direction(self, direction: Direction, distance: usize) -> Option<Self> {
        match direction {
            Direction::North => {
                if self.y >= distance {
                    Some(Coord::new(self.x, self.y - distance))
                } else {
                    None
                }
            }
            Direction::South => Some(Coord::new(self.x, self.y + distance)),
            Direction::East => Some(Coord::new(self.x + distance, self.y)),
            Direction::West => {
                if self.x >= distance {
                    Some(Coord::new(self.x - distance, self.y))
                } else {
                    None
                }
            }
            Direction::NorthEast => {
                if self.y >= distance {
                    Some(Coord::new(self.x + distance, self.y - distance))
                } else {
                    None
                }
            }
            Direction::NorthWest => {
                if self.x >= distance && self.y >= distance {
                    Some(Coord::new(self.x - distance, self.y - distance))
                } else {
                    None
                }
            }
            Direction::SouthEast => Some(Coord::new(self.x + distance, self.y + distance)),
            Direction::SouthWest => {
                if self.x >= distance {
                    Some(Coord::new(self.x - distance, self.y + distance))
                } else {
                    None
                }
            }
        }
    }

    /// Move in the given direction by the specified distance
    /// 
    /// # Panics
    /// 
    /// Panics if the movement would result in underflow (negative coordinates)
    pub fn move_in_direction(self, direction: Direction, distance: usize) -> Self {
        self.try_move_in_direction(direction, distance)
            .unwrap_or_else(|| panic!("Movement in direction {:?} by {} would underflow from {:?}", 
                                    direction, distance, self))
    }

    /// Move one step in the given direction
    pub fn step(self, direction: Direction) -> Option<Self> {
        self.try_move_in_direction(direction, 1)
    }

    /// Calculate Manhattan distance (sum of absolute differences) to another coordinate
    pub fn manhattan_distance(self, other: Coord) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx + dy
    }

    /// Calculate Chebyshev distance (max of absolute differences) to another coordinate
    /// Also known as chessboard distance or L∞ metric
    pub fn chebyshev_distance(self, other: Coord) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx.max(dy)
    }

    /// Calculate squared Euclidean distance (avoids floating point)
    pub fn euclidean_distance_squared(self, other: Coord) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx * dx + dy * dy
    }

    /// Get all 4 orthogonal neighbors (North, South, East, West)
    pub fn neighbors_4(self) -> Neighbors4 {
        Neighbors4 {
            coord: self,
            index: 0,
        }
    }

    /// Get all 8 neighbors (orthogonal + diagonal)
    pub fn neighbors_8(self) -> Neighbors8 {
        Neighbors8 {
            coord: self,
            index: 0,
        }
    }

    /// Get all 4 orthogonal neighbors that are within the given bounds
    pub fn neighbors_4_bounded(self, width: usize, height: usize) -> impl Iterator<Item = Coord> {
        self.neighbors_4()
            .filter(move |coord| coord.x < width && coord.y < height)
    }

    /// Get all 8 neighbors that are within the given bounds
    pub fn neighbors_8_bounded(self, width: usize, height: usize) -> impl Iterator<Item = Coord> {
        self.neighbors_8()
            .filter(move |coord| coord.x < width && coord.y < height)
    }

    /// Convert to signed coordinates (useful for arithmetic)
    pub fn to_signed(self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }

    /// Create from signed coordinates if they're non-negative
    pub fn from_signed(x: isize, y: isize) -> Option<Self> {
        if x >= 0 && y >= 0 {
            Some(Coord::new(x as usize, y as usize))
        } else {
            None
        }
    }
}

// Iterator for 4-directional neighbors
pub struct Neighbors4 {
    coord: Coord,
    index: usize,
}

impl Iterator for Neighbors4 {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= 4 {
                return None;
            }

            let direction = match self.index {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                3 => Direction::West,
                _ => unreachable!(),
            };

            self.index += 1;

            if let Some(neighbor) = self.coord.step(direction) {
                return Some(neighbor);
            }
            // Continue loop if step failed (underflow)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = 4 - self.index;
        (0, Some(remaining)) // Could be 0 if all moves underflow
    }
}

// Iterator for 8-directional neighbors
pub struct Neighbors8 {
    coord: Coord,
    index: usize,
}

impl Iterator for Neighbors8 {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        const DIRECTIONS: [Direction; 8] = [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ];

        loop {
            if self.index >= 8 {
                return None;
            }

            let direction = DIRECTIONS[self.index];
            self.index += 1;

            if let Some(neighbor) = self.coord.step(direction) {
                return Some(neighbor);
            }
            // Continue loop if step failed (underflow)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = 8 - self.index;
        (0, Some(remaining)) // Could be 0 if all moves underflow
    }
}

// Arithmetic operations
impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Coord::new(
            self.x.saturating_sub(other.x),
            self.y.saturating_sub(other.y),
        )
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x.saturating_sub(other.x);
        self.y = self.y.saturating_sub(other.y);
    }
}

// Hash implementation for use in HashMap/HashSet
impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Convert from tuples
impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord::new(x, y)
    }
}

impl From<Coord> for (usize, usize) {
    fn from(coord: Coord) -> Self {
        (coord.x, coord.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_coordinate_creation() {
        let coord = Coord::new(5, 10);
        assert_eq!(coord.x, 5);
        assert_eq!(coord.y, 10);

        let origin = Coord::origin();
        assert_eq!(origin, Coord::new(0, 0));
    }

    #[test]
    fn test_movement() {
        let start = Coord::new(5, 5);

        assert_eq!(start.move_in_direction(Direction::North, 2), Coord::new(5, 3));
        assert_eq!(start.move_in_direction(Direction::South, 1), Coord::new(5, 6));
        assert_eq!(start.move_in_direction(Direction::East, 3), Coord::new(8, 5));
        assert_eq!(start.move_in_direction(Direction::West, 1), Coord::new(4, 5));
    }

    #[test]
    fn test_movement_bounds() {
        let coord = Coord::new(2, 3);
        
        // These should work
        assert!(coord.try_move_in_direction(Direction::North, 3).is_some());
        assert!(coord.try_move_in_direction(Direction::West, 2).is_some());
        
        // These should fail (underflow)
        assert!(coord.try_move_in_direction(Direction::North, 4).is_none());
        assert!(coord.try_move_in_direction(Direction::West, 3).is_none());
    }

    #[test]
    fn test_distances() {
        let a = Coord::new(1, 1);
        let b = Coord::new(4, 5);

        assert_eq!(a.manhattan_distance(b), 7); // |4-1| + |5-1| = 3 + 4 = 7
        assert_eq!(a.chebyshev_distance(b), 4); // max(3, 4) = 4
        assert_eq!(a.euclidean_distance_squared(b), 25); // 3² + 4² = 9 + 16 = 25
    }

    #[test]
    fn test_neighbors() {
        let center = Coord::new(5, 5);
        
        let neighbors4: Vec<_> = center.neighbors_4().collect();
        assert!(neighbors4.contains(&Coord::new(5, 4))); // North
        assert!(neighbors4.contains(&Coord::new(5, 6))); // South  
        assert!(neighbors4.contains(&Coord::new(6, 5))); // East
        assert!(neighbors4.contains(&Coord::new(4, 5))); // West
        
        let neighbors8: Vec<_> = center.neighbors_8().collect();
        assert!(neighbors8.len() == 8);
        assert!(neighbors8.contains(&Coord::new(4, 4))); // NorthWest
        assert!(neighbors8.contains(&Coord::new(6, 6))); // SouthEast
    }

    #[test]
    fn test_neighbors_boundary() {
        let corner = Coord::new(0, 0);
        
        // At corner, some neighbors will be invalid due to underflow
        let neighbors4: Vec<_> = corner.neighbors_4().collect();
        assert!(neighbors4.len() < 4);
        assert!(neighbors4.contains(&Coord::new(1, 0))); // East
        assert!(neighbors4.contains(&Coord::new(0, 1))); // South
    }

    #[test]
    fn test_arithmetic() {
        let a = Coord::new(3, 4);
        let b = Coord::new(1, 2);

        assert_eq!(a + b, Coord::new(4, 6));
        assert_eq!(a - b, Coord::new(2, 2));
        
        let mut c = a;
        c += b;
        assert_eq!(c, Coord::new(4, 6));
    }

    #[test]
    fn test_hash_map_compatibility() {
        let mut map: HashMap<Coord, String> = HashMap::new();
        
        let coord1 = Coord::new(5, 10);
        let coord2 = Coord::new(5, 10); // Same coordinates
        
        map.insert(coord1, "first".to_string());
        assert_eq!(map.get(&coord2), Some(&"first".to_string()));
    }

    #[test]
    fn test_conversions() {
        let coord = Coord::new(3, 7);
        let tuple: (usize, usize) = coord.into();
        assert_eq!(tuple, (3, 7));
        
        let coord2: Coord = (5, 2).into();
        assert_eq!(coord2, Coord::new(5, 2));
        
        let (sx, sy) = coord.to_signed();
        assert_eq!((sx, sy), (3, 7));
        
        let coord3 = Coord::from_signed(sx, sy).unwrap();
        assert_eq!(coord3, coord);
    }
}