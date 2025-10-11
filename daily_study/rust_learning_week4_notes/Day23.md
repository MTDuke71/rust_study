# Day 23 · Grid Navigation (Directions & Bounds Checking)

> **Learning Context**: Day 23 builds on Day 22's grid fundamentals by adding movement and navigation capabilities, essential for Mission 6's pathfinding algorithms and AoC-style maze problems.

**Cross-Track Integration:**
- **Mission Focus**: Direction handling enables Mission 6's BFS/DFS pathfinding and Mission 7's graph traversal
- **Daily Study**: Week 4, Day 2 - Building navigation layer on top of spatial structures
- **Rust Book**: Applies Chapter 6 (Enums) to represent directions and Chapter 8 (Vectors) for neighbor lists

**Related Zettelkasten Notes:**
- [[../missions/Mission6/README|Mission6 - Pathfinding & Navigation]]
- [[Direction Enum Patterns]] - Direction representation strategies
- [[zettel-index]] - Main learning hub

---

## 🎯 Core Concepts

### Why Navigation Matters

Grid navigation is fundamental to countless algorithms:
- **Pathfinding**: A*, Dijkstra's, BFS/DFS require exploring neighbors
- **Flood Fill**: Spreading from a cell to connected cells
- **Game AI**: Character movement, line-of-sight calculations
- **Map Analysis**: Finding reachable areas, counting regions

**Key Navigation Challenges:**
1. **Direction Representation**: How do we encode UP, DOWN, LEFT, RIGHT?
2. **Neighbor Generation**: Given a cell, what cells are adjacent?
3. **Bounds Checking**: Is a position valid within the grid?
4. **Movement Validation**: Can we move to a cell (walls, obstacles)?

---

## 🧭 Direction Representation

### Cardinal Directions (4-connected)

**Cardinal directions** are the four main compass directions: North, South, East, West.

```rust
/// Four cardinal directions for grid movement
/// 
/// Cardinal directions form a 4-connected grid where each cell
/// has at most 4 neighbors (no diagonal movement).
/// 
/// Common uses:
/// - Chess rook movement
/// - Manhattan distance calculations  
/// - Simple maze navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,  // Up (decrease row)
    South,  // Down (increase row)
    East,   // Right (increase column)
    West,   // Left (decrease column)
}

impl Direction {
    /// Returns all four cardinal directions
    /// 
    /// Useful for iterating over all possible moves from a cell
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }
    
    /// Converts direction to row/column offset
    /// 
    /// This is the key to applying directions to coordinates:
    /// new_coord = old_coord + direction.offset()
    /// 
    /// # Returns
    /// (row_delta, col_delta) where:
    /// - Negative row_delta = moving up
    /// - Positive row_delta = moving down
    /// - Negative col_delta = moving left
    /// - Positive col_delta = moving right
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),   // Move up: row - 1
            Direction::South => (1, 0),    // Move down: row + 1
            Direction::East => (0, 1),     // Move right: col + 1
            Direction::West => (0, -1),    // Move left: col - 1
        }
    }
    
    /// Returns the opposite direction
    /// 
    /// Useful for backtracking, reversing paths
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    
    /// Rotates direction 90 degrees clockwise
    pub fn rotate_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    
    /// Rotates direction 90 degrees counter-clockwise
    pub fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}
```

### Intercardinal Directions (8-connected)

**Intercardinal directions** include diagonals, forming an 8-connected grid.

```rust
/// Eight directions including diagonals (8-connected grid)
/// 
/// 8-connected grids allow diagonal movement, where each cell
/// has at most 8 neighbors.
/// 
/// Common uses:
/// - Chess king/queen movement
/// - Chebyshev distance calculations
/// - More natural pathfinding (humans move diagonally)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction8 {
    /// Returns all eight directions
    pub fn all() -> [Direction8; 8] {
        [
            Direction8::North,
            Direction8::NorthEast,
            Direction8::East,
            Direction8::SouthEast,
            Direction8::South,
            Direction8::SouthWest,
            Direction8::West,
            Direction8::NorthWest,
        ]
    }
    
    /// Converts direction to row/column offset
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction8::North     => (-1, 0),
            Direction8::NorthEast => (-1, 1),   // Up and right
            Direction8::East      => (0, 1),
            Direction8::SouthEast => (1, 1),    // Down and right
            Direction8::South     => (1, 0),
            Direction8::SouthWest => (1, -1),   // Down and left
            Direction8::West      => (0, -1),
            Direction8::NorthWest => (-1, -1),  // Up and left
        }
    }
    
    /// Checks if this is a diagonal direction
    pub fn is_diagonal(&self) -> bool {
        matches!(self, 
            Direction8::NorthEast | Direction8::SouthEast | 
            Direction8::SouthWest | Direction8::NorthWest
        )
    }
    
    /// Checks if this is a cardinal direction
    pub fn is_cardinal(&self) -> bool {
        !self.is_diagonal()
    }
}
```

---

## 🚶 Neighbor Generation

### Generating 4-Connected Neighbors

```rust
impl Coord {
    /// Returns all valid 4-connected neighbors within grid bounds
    /// 
    /// # Arguments
    /// * `width` - Grid width (number of columns)
    /// * `height` - Grid height (number of rows)
    /// 
    /// # Example
    /// ```
    /// let center = Coord::new(5, 5);
    /// let neighbors = center.neighbors_4(10, 10);
    /// // Returns: [(4,5), (6,5), (5,6), (5,4)] - up, down, right, left
    /// ```
    pub fn neighbors_4(&self, width: usize, height: usize) -> Vec<Coord> {
        Direction::all()
            .iter()
            .filter_map(|dir| {
                let (dr, dc) = dir.offset();
                let new_row = self.row + dr;
                let new_col = self.col + dc;
                
                // Check bounds before creating coordinate
                if new_row >= 0 && new_row < height as isize &&
                   new_col >= 0 && new_col < width as isize {
                    Some(Coord::new(new_row, new_col))
                } else {
                    None  // Out of bounds - skip this neighbor
                }
            })
            .collect()
    }
    
    /// Returns neighbor in a specific direction (if valid)
    /// 
    /// # Returns
    /// `Some(coord)` if neighbor is within bounds, `None` otherwise
    pub fn neighbor_in_direction(
        &self,
        dir: Direction,
        width: usize,
        height: usize
    ) -> Option<Coord> {
        let (dr, dc) = dir.offset();
        let new_row = self.row + dr;
        let new_col = self.col + dc;
        
        if new_row >= 0 && new_row < height as isize &&
           new_col >= 0 && new_col < width as isize {
            Some(Coord::new(new_row, new_col))
        } else {
            None
        }
    }
}
```

### Generating 8-Connected Neighbors

```rust
impl Coord {
    /// Returns all valid 8-connected neighbors (including diagonals)
    /// 
    /// # Example
    /// ```
    /// let center = Coord::new(5, 5);
    /// let neighbors = center.neighbors_8(10, 10);
    /// // Returns 8 coordinates surrounding (5,5)
    /// ```
    pub fn neighbors_8(&self, width: usize, height: usize) -> Vec<Coord> {
        Direction8::all()
            .iter()
            .filter_map(|dir| {
                let (dr, dc) = dir.offset();
                let new_row = self.row + dr;
                let new_col = self.col + dc;
                
                if new_row >= 0 && new_row < height as isize &&
                   new_col >= 0 && new_col < width as isize {
                    Some(Coord::new(new_row, new_col))
                } else {
                    None
                }
            })
            .collect()
    }
}
```

---

## 🛡️ Bounds Checking Strategies

### Strategy 1: Check Before Creating Coordinate

```rust
/// Safe approach: Validate before creating coordinate
fn move_if_valid(
    pos: Coord,
    dir: Direction,
    width: usize,
    height: usize
) -> Option<Coord> {
    let (dr, dc) = dir.offset();
    let new_row = pos.row + dr;
    let new_col = pos.col + dc;
    
    // Check bounds BEFORE creating coordinate
    if new_row >= 0 && new_row < height as isize &&
       new_col >= 0 && new_col < width as isize {
        Some(Coord::new(new_row, new_col))
    } else {
        None  // Invalid move
    }
}
```

**Advantages:**
- ✅ Never creates invalid coordinates
- ✅ Clear intent with `Option` return type
- ✅ No panic risk

**Disadvantages:**
- ❌ Bounds checking logic must be repeated
- ❌ Can't easily compose coordinate operations

### Strategy 2: Create Then Validate

```rust
impl Coord {
    /// Checks if coordinate is within grid bounds
    pub fn is_valid(&self, width: usize, height: usize) -> bool {
        self.row >= 0 && self.row < height as isize &&
        self.col >= 0 && self.col < width as isize
    }
}

/// Create coordinate, then check validity
fn move_with_validation(
    pos: Coord,
    dir: Direction,
    width: usize,
    height: usize
) -> Option<Coord> {
    let (dr, dc) = dir.offset();
    let new_pos = Coord::new(pos.row + dr, pos.col + dc);
    
    // Check after creation
    if new_pos.is_valid(width, height) {
        Some(new_pos)
    } else {
        None
    }
}
```

**Advantages:**
- ✅ Separates coordinate math from bounds checking
- ✅ Reusable `is_valid()` method
- ✅ Easier to compose operations

**Disadvantages:**
- ❌ Creates coordinate that might be invalid
- ❌ Requires explicit validation

### Strategy 3: Grid-Based Bounds Checking

```rust
impl<T> Grid<T> {
    /// Checks if coordinate is valid for this specific grid
    pub fn contains(&self, coord: Coord) -> bool {
        coord.row >= 0 && coord.row < self.height as isize &&
        coord.col >= 0 && coord.col < self.width as isize
    }
    
    /// Gets cell if coordinate is valid
    pub fn get_coord(&self, coord: Coord) -> Option<&T> {
        if self.contains(coord) {
            Some(&self[(coord.row as usize, coord.col as usize)])
        } else {
            None
        }
    }
    
    /// Returns all valid 4-connected neighbors for a coordinate
    /// 
    /// This method encapsulates bounds checking within the grid itself
    pub fn neighbors_4(&self, coord: Coord) -> Vec<Coord> {
        Direction::all()
            .iter()
            .filter_map(|dir| {
                let (dr, dc) = dir.offset();
                let neighbor = Coord::new(coord.row + dr, coord.col + dc);
                
                // Use grid's own bounds checking
                if self.contains(neighbor) {
                    Some(neighbor)
                } else {
                    None
                }
            })
            .collect()
    }
}
```

**Advantages:**
- ✅ Grid knows its own bounds
- ✅ No need to pass width/height separately
- ✅ Most ergonomic API

**Disadvantages:**
- ❌ Requires grid reference
- ❌ Can't validate coordinates without grid

---

## 🚀 Complete Runnable Example

```rust
use std::fmt;

fn main() {
    println!("=== Day 23: Grid Navigation Demo ===\n");
    
    // 1. Direction Basics
    println!("🔷 1. Understanding Directions");
    println!("==============================");
    
    for dir in Direction::all() {
        let (dr, dc) = dir.offset();
        println!("{:?}: offset ({:2}, {:2}) - opposite: {:?}", 
            dir, dr, dc, dir.opposite());
    }
    
    // 2. Direction Rotation
    println!("\n🔷 2. Direction Rotation");
    println!("=======================");
    
    let mut dir = Direction::North;
    println!("Starting direction: {:?}", dir);
    println!("Rotating clockwise:");
    for _ in 0..4 {
        dir = dir.rotate_cw();
        println!("  → {:?}", dir);
    }
    
    // 3. Neighbor Generation
    println!("\n🔷 3. Finding Neighbors");
    println!("======================");
    
    let center = Coord::new(5, 5);
    let grid_size = (10, 10);
    
    println!("Center position: {:?}", center);
    println!("4-connected neighbors:");
    for neighbor in center.neighbors_4(grid_size.0, grid_size.1) {
        println!("  {:?}", neighbor);
    }
    
    // 4. Corner Handling
    println!("\n🔷 4. Handling Grid Corners");
    println!("==========================");
    
    let corner = Coord::new(0, 0);  // Top-left corner
    println!("Corner position: {:?}", corner);
    let corner_neighbors = corner.neighbors_4(grid_size.0, grid_size.1);
    println!("Neighbors (only 2 valid): {:?}", corner_neighbors);
    
    // 5. 8-Connected Movement
    println!("\n🔷 5. Diagonal Movement (8-connected)");
    println!("====================================");
    
    println!("Center position: {:?}", center);
    println!("8-connected neighbors (including diagonals):");
    for neighbor in center.neighbors_8(grid_size.0, grid_size.1) {
        println!("  {:?}", neighbor);
    }
    
    // 6. Pathfinding Simulation
    println!("\n🔷 6. Simple Pathfinding Simulation");
    println!("===================================");
    
    let mut grid = Grid::new(10, 10, '.');
    
    // Create a simple maze
    for row in 2..8 {
        grid[(row, 5)] = '#';  // Vertical wall
    }
    grid[(4, 5)] = '.';  // Gap in wall
    
    // Start and goal
    let start = Coord::new(5, 2);
    let goal = Coord::new(5, 8);
    grid[(start.row as usize, start.col as usize)] = 'S';
    grid[(goal.row as usize, goal.col as usize)] = 'G';
    
    println!("Maze layout:");
    println!("{}", grid);
    
    // Find navigable neighbors from start
    println!("From start position {:?}, can move to:", start);
    for neighbor in grid.neighbors_4(start) {
        let cell = grid.get_coord(neighbor).unwrap();
        if *cell != '#' {
            println!("  {:?} (cell: '{}')", neighbor, cell);
        }
    }
    
    // 7. Movement Validation
    println!("\n🔷 7. Movement Validation");
    println!("========================");
    
    let test_pos = Coord::new(5, 4);
    println!("Current position: {:?}", test_pos);
    
    for dir in Direction::all() {
        let (dr, dc) = dir.offset();
        let new_pos = Coord::new(test_pos.row + dr, test_pos.col + dc);
        
        if grid.contains(new_pos) {
            let cell = grid.get_coord(new_pos).unwrap();
            let can_move = *cell != '#';
            println!("  {:?} → {:?}: {} (cell: '{}')",
                dir, new_pos,
                if can_move { "✓ Can move" } else { "✗ Blocked" },
                cell
            );
        } else {
            println!("  {:?} → Out of bounds", dir);
        }
    }
}

// === Supporting Code ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
    
    pub fn neighbors_4(&self, width: usize, height: usize) -> Vec<Coord> {
        Direction::all()
            .iter()
            .filter_map(|dir| {
                let (dr, dc) = dir.offset();
                let new_row = self.row + dr;
                let new_col = self.col + dc;
                
                if new_row >= 0 && new_row < height as isize &&
                   new_col >= 0 && new_col < width as isize {
                    Some(Coord::new(new_row, new_col))
                } else {
                    None
                }
            })
            .collect()
    }
    
    pub fn neighbors_8(&self, width: usize, height: usize) -> Vec<Coord> {
        Direction8::all()
            .iter()
            .filter_map(|dir| {
                let (dr, dc) = dir.offset();
                let new_row = self.row + dr;
                let new_col = self.col + dc;
                
                if new_row >= 0 && new_row < height as isize &&
                   new_col >= 0 && new_col < width as isize {
                    Some(Coord::new(new_row, new_col))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }
    
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
    
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    
    pub fn rotate_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    
    pub fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction8 {
    North, NorthEast, East, SouthEast,
    South, SouthWest, West, NorthWest,
}

impl Direction8 {
    pub fn all() -> [Direction8; 8] {
        [
            Direction8::North, Direction8::NorthEast,
            Direction8::East, Direction8::SouthEast,
            Direction8::South, Direction8::SouthWest,
            Direction8::West, Direction8::NorthWest,
        ]
    }
    
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction8::North     => (-1, 0),
            Direction8::NorthEast => (-1, 1),
            Direction8::East      => (0, 1),
            Direction8::SouthEast => (1, 1),
            Direction8::South     => (1, 0),
            Direction8::SouthWest => (1, -1),
            Direction8::West      => (0, -1),
            Direction8::NorthWest => (-1, -1),
        }
    }
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn to_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }
    
    pub fn contains(&self, coord: Coord) -> bool {
        coord.row >= 0 && coord.row < self.height as isize &&
        coord.col >= 0 && coord.col < self.width as isize
    }
    
    pub fn get_coord(&self, coord: Coord) -> Option<&T> {
        if self.contains(coord) {
            Some(&self[(coord.row as usize, coord.col as usize)])
        } else {
            None
        }
    }
    
    pub fn neighbors_4(&self, coord: Coord) -> Vec<Coord> {
        Direction::all()
            .iter()
            .filter_map(|dir| {
                let (dr, dc) = dir.offset();
                let neighbor = Coord::new(coord.row + dr, coord.col + dc);
                
                if self.contains(neighbor) {
                    Some(neighbor)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[self.to_index(row, col)]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let idx = self.to_index(row, col);
        &mut self.data[idx]
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self[(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
```

### 🛠️ How to Run This Code:

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day23_demo.rs` and run `rustc day23_demo.rs && ./day23_demo`
3. **In this workspace**: `cargo new day23_navigation_demo && cd day23_navigation_demo` then paste into `src/main.rs`

---

## 💡 Key Takeaways

### Navigation Design Principles

1. **Use Enums for Directions**
   - Type-safe representation
   - Exhaustive match checking
   - Easy to extend with methods

2. **Provide offset() Method**
   - Converts direction to numeric offset
   - Enables coordinate arithmetic
   - Separates direction logic from coordinate logic

3. **Generate Neighbors Lazily**
   - Use `filter_map()` to skip invalid positions
   - Return `Vec<Coord>` for easy iteration
   - Consider iterator-based API for large grids

4. **Encapsulate Bounds Checking**
   - Grid knows its own dimensions
   - Coordinate validation is reusable
   - Clear API with `Option` returns

### Common Patterns

**Pattern: Exploring All Neighbors**
```rust
for neighbor in grid.neighbors_4(current_pos) {
    if grid.get_coord(neighbor) == Some(&'.') {
        // Process walkable neighbor
    }
}
```

**Pattern: Moving in Specific Direction**
```rust
let direction = Direction::North;
if let Some(new_pos) = current_pos.neighbor_in_direction(direction, width, height) {
    // Valid move
}
```

**Pattern: Rotation-Based Search**
```rust
let mut dir = Direction::North;
for _ in 0..4 {
    if let Some(neighbor) = pos.neighbor_in_direction(dir, width, height) {
        // Try this direction
    }
    dir = dir.rotate_cw();
}
```

---

## 🔗 Related Topics

### Tomorrow's Preview: Day 24 - Grid Algorithms
- Flood fill algorithm (recursive and iterative)
- Connected component finding
- Region counting and analysis
- BFS/DFS on grids

### Mission Integration
- **Mission 6**: Complete navigation system with pathfinding
- **Mission 7**: Graph traversal using navigation patterns

### AoC Applications
- **Maze Solving**: Navigate through walls to reach goal
- **Region Counting**: Find all connected areas
- **Shortest Path**: BFS with 4-connected movement

---

*Tags: #navigation #directions #bounds-checking #neighbors #grid-algorithms #mission6 #pathfinding*
*Links: [[Day22]] ← | [[../missions/Mission6/README|Mission6]] | [[zettel-index]] | [[Day24]] →*
