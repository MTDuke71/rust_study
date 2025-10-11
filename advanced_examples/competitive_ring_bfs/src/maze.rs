/// Position in a 2D grid maze
///
/// # Requirements Satisfied: REQ-G1, REQ-G3
/// Represents a coordinate pair with row and column indices.
/// Used throughout BFS to track current position and path reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    /// Create a new position
    ///
    /// # Requirements Satisfied: REQ-G1
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Get neighboring positions (4-directional movement)
    ///
    /// # Requirements Satisfied: REQ-B2
    /// Returns up to 4 neighbors: up, down, left, right
    /// Handles boundary checking to prevent underflow/overflow
    pub fn neighbors(&self, max_row: usize, max_col: usize) -> Vec<Position> {
        let mut neighbors = Vec::with_capacity(4);

        // Up
        if self.row > 0 {
            neighbors.push(Position::new(self.row - 1, self.col));
        }

        // Down
        if self.row + 1 < max_row {
            neighbors.push(Position::new(self.row + 1, self.col));
        }

        // Left
        if self.col > 0 {
            neighbors.push(Position::new(self.row, self.col - 1));
        }

        // Right
        if self.col + 1 < max_col {
            neighbors.push(Position::new(self.row, self.col + 1));
        }

        neighbors
    }
}

/// Cell type in the maze
///
/// # Requirements Satisfied: REQ-G3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Open path that can be traversed
    Open,
    /// Wall that blocks movement
    Wall,
    /// Starting position
    Start,
    /// Goal position
    Goal,
}

impl Cell {
    /// Check if this cell can be traversed during BFS
    ///
    /// # Requirements Satisfied: REQ-G3, REQ-B1
    pub fn is_traversable(&self) -> bool {
        match self {
            Cell::Open | Cell::Start | Cell::Goal => true,
            Cell::Wall => false,
        }
    }
}

/// 2D grid maze representation
///
/// # Requirements Satisfied: REQ-G1, REQ-G2, REQ-G4
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
    start: Option<Position>,
    goal: Option<Position>,
}

impl Grid {
    /// Create a new grid from string representation
    ///
    /// # Format
    /// - 'S' = Start position
    /// - 'G' = Goal position  
    /// - '.' = Open path
    /// - '#' = Wall
    ///
    /// # Requirements Satisfied: REQ-G1, REQ-G2, REQ-G4
    ///
    /// # Errors
    /// Returns None if:
    /// - Grid is empty
    /// - No start or goal found
    /// - Multiple starts or goals found
    /// - Rows have different lengths
    pub fn from_strings(lines: Vec<String>) -> Option<Self> {
        if lines.is_empty() {
            return None;
        }

        let rows = lines.len();
        let cols = lines[0].len();

        // Validate all rows have same length (REQ-G4)
        if !lines.iter().all(|line| line.len() == cols) {
            return None;
        }

        let mut cells = Vec::with_capacity(rows);
        let mut start = None;
        let mut goal = None;

        for (row_idx, line) in lines.iter().enumerate() {
            let mut row = Vec::with_capacity(cols);

            for (col_idx, ch) in line.chars().enumerate() {
                let cell = match ch {
                    'S' => {
                        // REQ-G2: Exactly one start
                        if start.is_some() {
                            return None; // Multiple starts
                        }
                        start = Some(Position::new(row_idx, col_idx));
                        Cell::Start
                    }
                    'G' => {
                        // REQ-G2: Exactly one goal
                        if goal.is_some() {
                            return None; // Multiple goals
                        }
                        goal = Some(Position::new(row_idx, col_idx));
                        Cell::Goal
                    }
                    '.' => Cell::Open,
                    '#' => Cell::Wall,
                    _ => return None, // Invalid character
                };
                row.push(cell);
            }
            cells.push(row);
        }

        // REQ-G2: Must have both start and goal
        if start.is_none() || goal.is_none() {
            return None;
        }

        Some(Grid {
            cells,
            rows,
            cols,
            start,
            goal,
        })
    }

    /// Get cell at position
    ///
    /// # Requirements Satisfied: REQ-G3
    pub fn get(&self, pos: Position) -> Option<Cell> {
        if pos.row < self.rows && pos.col < self.cols {
            Some(self.cells[pos.row][pos.col])
        } else {
            None
        }
    }

    /// Check if position is within grid bounds
    ///
    /// # Requirements Satisfied: REQ-G4
    pub fn is_valid(&self, pos: Position) -> bool {
        pos.row < self.rows && pos.col < self.cols
    }

    /// Get grid dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Get start position
    ///
    /// # Requirements Satisfied: REQ-G2
    pub fn start(&self) -> Position {
        self.start.expect("Grid should have start position")
    }

    /// Get goal position
    ///
    /// # Requirements Satisfied: REQ-G2  
    pub fn goal(&self) -> Position {
        self.goal.expect("Grid should have goal position")
    }

    /// Calculate maximum possible queue size for this grid
    ///
    /// # Requirements Satisfied: REQ-B3
    /// Maximum queue size occurs when BFS visits every open cell.
    /// This provides the capacity for RingBufferQueue initialization.
    pub fn max_queue_size(&self) -> usize {
        // In worst case, all cells could be in queue simultaneously
        // This happens in a grid where BFS spreads like a diamond pattern
        self.rows * self.cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-G1
    fn req_g1_position_creation() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.row, 5);
        assert_eq!(pos.col, 10);
    }

    #[test] // REQ-B2
    fn req_b2_neighbor_generation() {
        let pos = Position::new(1, 1);
        let neighbors = pos.neighbors(3, 3);

        // Should have 4 neighbors in middle of grid
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&Position::new(0, 1))); // up
        assert!(neighbors.contains(&Position::new(2, 1))); // down
        assert!(neighbors.contains(&Position::new(1, 0))); // left
        assert!(neighbors.contains(&Position::new(1, 2))); // right
    }

    #[test] // REQ-B2
    fn req_b2_boundary_neighbors() {
        // Corner position should have only 2 neighbors
        let pos = Position::new(0, 0);
        let neighbors = pos.neighbors(3, 3);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position::new(0, 1))); // right
        assert!(neighbors.contains(&Position::new(1, 0))); // down
    }

    #[test] // REG-G3
    fn req_g3_cell_traversability() {
        assert!(Cell::Open.is_traversable());
        assert!(Cell::Start.is_traversable());
        assert!(Cell::Goal.is_traversable());
        assert!(!Cell::Wall.is_traversable());
    }

    #[test] // REQ-G1, REQ-G2
    fn req_g1_g2_grid_parsing() {
        let maze = vec!["S..".to_string(), ".#.".to_string(), "..G".to_string()];

        let grid = Grid::from_strings(maze).expect("Valid maze");
        assert_eq!(grid.dimensions(), (3, 3));
        assert_eq!(grid.start(), Position::new(0, 0));
        assert_eq!(grid.goal(), Position::new(2, 2));
    }

    #[test] // REQ-G4
    fn req_g4_invalid_grid_rejected() {
        // Different row lengths
        let invalid_maze = vec![
            "S..".to_string(),
            ".#".to_string(), // Too short
            "..G".to_string(),
        ];

        assert!(Grid::from_strings(invalid_maze).is_none());

        // Multiple starts
        let invalid_maze2 = vec!["S.S".to_string(), "...".to_string(), "..G".to_string()];

        assert!(Grid::from_strings(invalid_maze2).is_none());
    }

    #[test] // REQ-B3
    fn req_b3_max_queue_size_calculation() {
        let maze = vec!["S..".to_string(), "...".to_string(), "..G".to_string()];

        let grid = Grid::from_strings(maze).expect("Valid maze");
        assert_eq!(grid.max_queue_size(), 9); // 3x3 = 9
    }
}
