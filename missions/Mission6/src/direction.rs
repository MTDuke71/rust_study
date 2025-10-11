//! Direction enumeration for 2D navigation
//!
//! Provides the Direction enum for representing movement directions
//! with rotation operations and vector calculations.

use std::fmt;

/// Cardinal and diagonal directions for 2D navigation
///
/// Uses standard compass directions with the following coordinate system:
/// - North: decreases y (up on screen/grid)
/// - South: increases y (down on screen/grid)  
/// - East: increases x (right)
/// - West: decreases x (left)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    /// Get all 4 cardinal directions (North, South, East, West)
    pub fn cardinals() -> [Direction; 4] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    /// Get all 8 directions (cardinals + diagonals)
    pub fn all() -> [Direction; 8] {
        [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }

    /// Check if this is a cardinal direction (not diagonal)
    pub fn is_cardinal(self) -> bool {
        matches!(
            self,
            Direction::North | Direction::South | Direction::East | Direction::West
        )
    }

    /// Check if this is a diagonal direction
    pub fn is_diagonal(self) -> bool {
        !self.is_cardinal()
    }

    /// Get the opposite direction
    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    /// Rotate clockwise by 45 degrees
    pub fn rotate_clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        }
    }

    /// Rotate counterclockwise by 45 degrees
    pub fn rotate_counterclockwise(self) -> Direction {
        match self {
            Direction::North => Direction::NorthWest,
            Direction::NorthWest => Direction::West,
            Direction::West => Direction::SouthWest,
            Direction::SouthWest => Direction::South,
            Direction::South => Direction::SouthEast,
            Direction::SouthEast => Direction::East,
            Direction::East => Direction::NorthEast,
            Direction::NorthEast => Direction::North,
        }
    }

    /// Rotate clockwise by 90 degrees (for cardinal directions)
    pub fn rotate_90_clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            // For diagonal directions, rotate by 90 degrees
            Direction::NorthEast => Direction::SouthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::NorthWest => Direction::NorthEast,
        }
    }

    /// Rotate counterclockwise by 90 degrees (for cardinal directions)
    pub fn rotate_90_counterclockwise(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            // For diagonal directions, rotate by 90 degrees
            Direction::NorthEast => Direction::NorthWest,
            Direction::NorthWest => Direction::SouthWest,
            Direction::SouthWest => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthEast,
        }
    }

    /// Get the unit vector (dx, dy) for this direction
    /// Returns (x_delta, y_delta) where positive x is East, positive y is South
    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }

    /// Convert from a unit vector to direction
    /// Returns None if the vector doesn't correspond to a valid direction
    pub fn from_delta(dx: isize, dy: isize) -> Option<Direction> {
        match (dx.signum(), dy.signum()) {
            (0, -1) => Some(Direction::North),
            (1, -1) => Some(Direction::NorthEast),
            (1, 0) => Some(Direction::East),
            (1, 1) => Some(Direction::SouthEast),
            (0, 1) => Some(Direction::South),
            (-1, 1) => Some(Direction::SouthWest),
            (-1, 0) => Some(Direction::West),
            (-1, -1) => Some(Direction::NorthWest),
            _ => None,
        }
    }

    /// Get the direction from one coordinate to another
    /// Returns the primary direction (may not be exact for non-aligned coordinates)
    pub fn from_coords(
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Option<Direction> {
        let dx = to_x as isize - from_x as isize;
        let dy = to_y as isize - from_y as isize;

        if dx == 0 && dy == 0 {
            return None; // Same coordinate
        }

        Direction::from_delta(dx.signum(), dy.signum())
    }

    /// Get the angle in degrees (0 = North, clockwise)
    pub fn angle_degrees(self) -> f64 {
        match self {
            Direction::North => 0.0,
            Direction::NorthEast => 45.0,
            Direction::East => 90.0,
            Direction::SouthEast => 135.0,
            Direction::South => 180.0,
            Direction::SouthWest => 225.0,
            Direction::West => 270.0,
            Direction::NorthWest => 315.0,
        }
    }

    /// Create direction from angle in degrees (0 = North, clockwise)
    /// Rounds to the nearest 45-degree direction
    pub fn from_angle_degrees(degrees: f64) -> Direction {
        let normalized = ((degrees % 360.0) + 360.0) % 360.0; // Normalize to [0, 360)
        let octant = ((normalized + 22.5) / 45.0) as usize % 8;

        match octant {
            0 => Direction::North,
            1 => Direction::NorthEast,
            2 => Direction::East,
            3 => Direction::SouthEast,
            4 => Direction::South,
            5 => Direction::SouthWest,
            6 => Direction::West,
            7 => Direction::NorthWest,
            _ => unreachable!(),
        }
    }

    /// Get a single-character representation
    pub fn to_char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::NorthEast => '╱',
            Direction::East => '>',
            Direction::SouthEast => '╲',
            Direction::South => 'v',
            Direction::SouthWest => '╱',
            Direction::West => '<',
            Direction::NorthWest => '╲',
        }
    }

    /// Parse from a character
    pub fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' | 'N' | 'n' => Some(Direction::North),
            '>' | 'E' | 'e' => Some(Direction::East),
            'v' | 'S' | 's' => Some(Direction::South),
            '<' | 'W' | 'w' => Some(Direction::West),
            _ => None,
        }
    }
}

use std::str::FromStr;

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "north" | "n" | "up" | "u" => Ok(Direction::North),
            "northeast" | "ne" => Ok(Direction::NorthEast),
            "east" | "e" | "right" | "r" => Ok(Direction::East),
            "southeast" | "se" => Ok(Direction::SouthEast),
            "south" | "s" | "down" | "d" => Ok(Direction::South),
            "southwest" | "sw" => Ok(Direction::SouthWest),
            "west" | "w" | "left" | "l" => Ok(Direction::West),
            "northwest" | "nw" => Ok(Direction::NorthWest),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Direction::North => "North",
            Direction::NorthEast => "NorthEast",
            Direction::East => "East",
            Direction::SouthEast => "SouthEast",
            Direction::South => "South",
            Direction::SouthWest => "SouthWest",
            Direction::West => "West",
            Direction::NorthWest => "NorthWest",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_basics() {
        assert!(Direction::North.is_cardinal());
        assert!(Direction::NorthEast.is_diagonal());

        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::NorthEast.opposite(), Direction::SouthWest);
    }

    #[test]
    fn test_rotation() {
        let dir = Direction::North;

        assert_eq!(dir.rotate_clockwise(), Direction::NorthEast);
        assert_eq!(dir.rotate_90_clockwise(), Direction::East);
        assert_eq!(dir.rotate_counterclockwise(), Direction::NorthWest);
        assert_eq!(dir.rotate_90_counterclockwise(), Direction::West);
    }

    #[test]
    fn test_delta() {
        assert_eq!(Direction::North.delta(), (0, -1));
        assert_eq!(Direction::East.delta(), (1, 0));
        assert_eq!(Direction::SouthWest.delta(), (-1, 1));

        // Test round-trip conversion
        for direction in Direction::all() {
            let (dx, dy) = direction.delta();
            assert_eq!(Direction::from_delta(dx, dy), Some(direction));
        }
    }

    #[test]
    fn test_angles() {
        assert_eq!(Direction::North.angle_degrees(), 0.0);
        assert_eq!(Direction::East.angle_degrees(), 90.0);
        assert_eq!(Direction::South.angle_degrees(), 180.0);
        assert_eq!(Direction::West.angle_degrees(), 270.0);

        // Test round-trip conversion
        for direction in Direction::all() {
            let angle = direction.angle_degrees();
            assert_eq!(Direction::from_angle_degrees(angle), direction);
        }
    }

    #[test]
    fn test_parsing() {
        assert_eq!(Direction::from_char('^'), Some(Direction::North));
        assert_eq!(Direction::from_char('E'), Some(Direction::East));
        assert_eq!(Direction::from_char('x'), None);

        assert_eq!("north".parse(), Ok(Direction::North));
        assert_eq!("NORTHEAST".parse(), Ok(Direction::NorthEast));
        assert_eq!("invalid".parse::<Direction>(), Err(()));
    }

    #[test]
    fn test_from_coords() {
        // Same coordinate
        assert_eq!(Direction::from_coords(5, 5, 5, 5), None);

        // Cardinal directions
        assert_eq!(Direction::from_coords(5, 5, 5, 3), Some(Direction::North));
        assert_eq!(Direction::from_coords(5, 5, 7, 5), Some(Direction::East));
        assert_eq!(Direction::from_coords(5, 5, 5, 8), Some(Direction::South));
        assert_eq!(Direction::from_coords(5, 5, 2, 5), Some(Direction::West));

        // Diagonal directions
        assert_eq!(
            Direction::from_coords(5, 5, 7, 3),
            Some(Direction::NorthEast)
        );
        assert_eq!(
            Direction::from_coords(5, 5, 3, 7),
            Some(Direction::SouthWest)
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(Direction::North.to_string(), "North");
        assert_eq!(Direction::SouthEast.to_string(), "SouthEast");
    }
}
