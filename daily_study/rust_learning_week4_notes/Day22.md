# Day 22 · Grid Fundamentals (2D Arrays & Coordinates)

> **Learning Context**: Day 22 begins Week 4's focus on spatial algorithms and grid-based problem solving, building on Week 3's trait system mastery to enable Mission 6's 2D grid utilities and Mission 7's graph representation.

**Cross-Track Integration:**
- **Mission Focus**: Grid fundamentals power Mission 6's pathfinding and Mission 7's spatial graph algorithms - see [[Mission6 Overview]] and [[Mission7 Overview]]
- **Daily Study**: Week 4 opener - transitions from abstract types to concrete spatial structures
- **Rust Book**: Builds on Chapter 8.1 (Vectors) for 2D array storage

**Related Zettelkasten Notes:**
- [[../missions/Mission6/README|Mission6 - 2D Grids & Navigation]] - Real-world grid implementation
- [[Collections MOC]] - Grid as specialized collection type
- [[zettel-index]] - Main learning hub

---

## 🎯 Core Concepts

### What Are Grids?

**Grids** are 2D data structures that represent spatial relationships in a rectangular coordinate system. They're fundamental to:
- **Game Development**: Tile-based games, chess boards, roguelikes
- **Algorithms**: Pathfinding (A*, Dijkstra), flood fill, maze generation
- **Advent of Code**: Many AoC problems use grid-based puzzles
- **Image Processing**: Pixel grids, convolution filters

**Key Properties:**
- **Fixed dimensions**: Width and height define the grid size
- **Coordinate access**: Access elements using (row, column) or (x, y) pairs
- **Contiguous memory**: Often backed by a single `Vec<T>` for cache efficiency
- **Bounded iteration**: Can iterate over all cells or specific ranges

---

## 📐 Coordinate Systems

### Understanding Grid Coordinates

There are multiple ways to think about grid positions:

```rust
// Array-style indexing (row, column)
grid[row][column]  // Row-major order (common in programming)

// Cartesian coordinates (x, y)
grid[(x, y)]       // X = column, Y = row (matches math/graphics)

// Linear indexing (1D offset)
let index = row * width + column;
grid[index]
```

**Critical Understanding:**
- **Row-major order**: Data stored row by row in memory
- **Column vs X-coordinate**: X increases to the right (columns)
- **Row vs Y-coordinate**: Y can increase down (array-style) or up (Cartesian)

### Coordinate Struct Design

```rust
/// Represents a position in a 2D grid
/// 
/// This struct uses isize (signed integers) to allow for:
/// - Negative coordinates during intermediate calculations
/// - Direction offsets that can be negative
/// - Boundary checking without overflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: isize,  // Y-coordinate (vertical position)
    pub col: isize,  // X-coordinate (horizontal position)
}

impl Coord {
    /// Creates a new coordinate
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
    
    /// Creates from (x, y) Cartesian coordinates
    /// Note: x maps to column, y maps to row
    pub fn from_xy(x: isize, y: isize) -> Self {
        Self { row: y, col: x }
    }
    
    /// Converts to (x, y) tuple for graphics/math contexts
    pub fn to_xy(&self) -> (isize, isize) {
        (self.col, self.row)
    }
}
```

**Why Use `isize` Instead of `usize`?**

Using signed integers (`isize`) for coordinates provides several advantages:

1. **Direction Calculations**: Can represent negative offsets
   ```rust
   let up = Coord::new(-1, 0);      // Moving up decreases row
   let left = Coord::new(0, -1);    // Moving left decreases column
   ```

2. **Boundary Checking**: Can detect out-of-bounds without panic
   ```rust
   let pos = Coord::new(0, 0);
   let new_pos = Coord::new(pos.row - 1, pos.col);  // -1, 0 (negative!)
   // Easy to check: if new_pos.row < 0 || new_pos.col < 0 { /* out of bounds */ }
   ```

3. **Math Operations**: Arithmetic works naturally
   ```rust
   let start = Coord::new(5, 5);
   let offset = Coord::new(-2, 3);
   let end = Coord::new(start.row + offset.row, start.col + offset.col);
   // Result: (3, 8) - no overflow concerns
   ```

---

## 🗂️ Grid Storage Strategies

### Strategy 1: Flat Vector (Recommended for Performance)

**Idea**: Store all grid data in a single `Vec<T>`, convert 2D coordinates to 1D index.

```rust
/// High-performance grid using flat vector storage
/// 
/// Memory layout: [row0_col0, row0_col1, ..., row1_col0, row1_col1, ...]
/// This provides excellent cache locality for row-wise iteration
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    /// Converts 2D coordinates to 1D vector index
    /// 
    /// # Formula
    /// index = row * width + column
    /// 
    /// This is "row-major order" - we store complete rows sequentially
    /// 
    /// # Panics
    /// Panics if coordinates are out of bounds
    fn to_index(&self, row: usize, col: usize) -> usize {
        assert!(row < self.height, "Row {} out of bounds (height: {})", row, self.height);
        assert!(col < self.width, "Column {} out of bounds (width: {})", col, self.width);
        row * self.width + col
    }
    
    /// Gets a reference to the cell at the given position
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.height && col < self.width {
            Some(&self.data[self.to_index(row, col)])
        } else {
            None  // Out of bounds - return None instead of panicking
        }
    }
    
    /// Gets a mutable reference to the cell at the given position
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.height && col < self.width {
            let idx = self.to_index(row, col);
            Some(&mut self.data[idx])
        } else {
            None
        }
    }
}

impl<T: Clone> Grid<T> {
    /// Creates a new grid filled with the given value
    /// 
    /// # Arguments
    /// * `width` - Number of columns
    /// * `height` - Number of rows
    /// * `default` - Value to fill all cells with
    /// 
    /// # Example
    /// ```
    /// let grid = Grid::new(10, 10, '.');
    /// // Creates a 10x10 grid filled with '.' characters
    /// ```
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }
}

// Index operator support for convenient access
use std::ops::{Index, IndexMut};

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[self.to_index(row, col)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let idx = self.to_index(row, col);
        &mut self.data[idx]
    }
}
```

**Advantages of Flat Storage:**
- ✅ **Cache-friendly**: All data is contiguous in memory
- ✅ **Simple indexing**: Single calculation to find any cell
- ✅ **Memory efficient**: No extra vector overhead per row
- ✅ **Fast iteration**: Can iterate over entire grid linearly

**Disadvantages:**
- ❌ **Manual indexing**: Must convert 2D → 1D yourself
- ❌ **Overflow risk**: Large grids may overflow on index calculation (use `checked_mul`)

### Strategy 2: Vector of Vectors (Intuitive but Slower)

```rust
/// Grid using nested vectors (simpler but less efficient)
pub struct Grid2D<T> {
    rows: Vec<Vec<T>>,
}

impl<T: Clone> Grid2D<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            rows: vec![vec![default; width]; height],
        }
    }
    
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.rows.get(row)?.get(col)
    }
}

// Direct indexing: grid[row][col]
impl<T> Index<usize> for Grid2D<T> {
    type Output = Vec<T>;
    
    fn index(&self, row: usize) -> &Self::Output {
        &self.rows[row]
    }
}
```

**Advantages:**
- ✅ **Intuitive**: `grid[row][col]` matches mental model
- ✅ **Simple bounds**: Each row knows its own length

**Disadvantages:**
- ❌ **Memory fragmentation**: Each row is a separate allocation
- ❌ **Cache misses**: Rows may not be adjacent in memory
- ❌ **Extra indirection**: Two pointer dereferences per access

---

## 🚀 Complete Runnable Example

```rust
use std::ops::{Index, IndexMut};
use std::fmt;

fn main() {
    println!("=== Day 22: Grid Fundamentals Demo ===\n");
    
    // 1. Basic Grid Creation
    println!("🔷 1. Creating and Initializing Grids");
    println!("====================================");
    
    let mut grid = Grid::new(5, 5, '.');
    println!("Created 5x5 grid filled with '.'");
    println!("{}", grid);
    
    // 2. Coordinate Systems
    println!("\n🔷 2. Understanding Coordinates");
    println!("==============================");
    
    // Array-style: (row, column)
    grid[(0, 0)] = 'A';  // Top-left corner
    grid[(0, 4)] = 'B';  // Top-right corner
    grid[(4, 0)] = 'C';  // Bottom-left corner
    grid[(4, 4)] = 'D';  // Bottom-right corner
    println!("Set corners using (row, col) indexing:");
    println!("{}", grid);
    
    // 3. Safe Access with get/get_mut
    println!("\n🔷 3. Safe Bounds Checking");
    println!("=========================");
    
    match grid.get(2, 2) {
        Some(cell) => println!("Cell at (2,2): {}", cell),
        None => println!("Cell at (2,2) is out of bounds"),
    }
    
    // Attempting out-of-bounds access (safe)
    match grid.get(10, 10) {
        Some(cell) => println!("Cell at (10,10): {}", cell),
        None => println!("Cell at (10,10) is out of bounds ✓"),
    }
    
    // 4. Grid Modification
    println!("\n🔷 4. Modifying Grid Contents");
    println!("============================");
    
    if let Some(cell) = grid.get_mut(2, 2) {
        *cell = 'X';  // Place marker in center
        println!("Set center cell to 'X'");
    }
    println!("{}", grid);
    
    // 5. Iteration Patterns
    println!("\n🔷 5. Iterating Over Grid");
    println!("========================");
    
    println!("Finding all non-empty cells:");
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if grid[(row, col)] != '.' {
                println!("  Found '{}' at ({}, {})", grid[(row, col)], row, col);
            }
        }
    }
    
    // 6. Common Grid Patterns
    println!("\n🔷 6. Common Grid Patterns");
    println!("=========================");
    
    // Pattern: Creating a border
    let mut bordered_grid = Grid::new(7, 7, ' ');
    for col in 0..bordered_grid.width() {
        bordered_grid[(0, col)] = '#';  // Top border
        bordered_grid[(6, col)] = '#';  // Bottom border
    }
    for row in 0..bordered_grid.height() {
        bordered_grid[(row, 0)] = '#';  // Left border
        bordered_grid[(row, 6)] = '#';  // Right border
    }
    bordered_grid[(3, 3)] = 'O';  // Center marker
    
    println!("Grid with border and center marker:");
    println!("{}", bordered_grid);
    
    // 7. Coordinate Arithmetic
    println!("\n🔷 7. Coordinate Arithmetic");
    println!("==========================");
    
    let start = Coord::new(3, 3);
    println!("Starting position: {:?}", start);
    
    let directions = [
        ("up", Coord::new(-1, 0)),
        ("right", Coord::new(0, 1)),
        ("down", Coord::new(1, 0)),
        ("left", Coord::new(0, -1)),
    ];
    
    for (name, offset) in &directions {
        let new_pos = Coord::new(start.row + offset.row, start.col + offset.col);
        println!("  Move {}: {:?}", name, new_pos);
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
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
    
    fn to_index(&self, row: usize, col: usize) -> usize {
        assert!(row < self.height && col < self.width);
        row * self.width + col
    }
    
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.height && col < self.width {
            Some(&self.data[self.to_index(row, col)])
        } else {
            None
        }
    }
    
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.height && col < self.width {
            let idx = self.to_index(row, col);
            Some(&mut self.data[idx])
        } else {
            None
        }
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
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[self.to_index(row, col)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
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
2. **Local file**: Save as `day22_demo.rs` and run `rustc day22_demo.rs && ./day22_demo`
3. **In this workspace**: `cargo new day22_grid_demo && cd day22_grid_demo` then paste into `src/main.rs`

---

## 💡 Key Takeaways

### Grid Design Principles

1. **Use Flat Storage for Performance**
   - Single `Vec<T>` with index calculation
   - Better cache locality, fewer allocations

2. **Choose Signed Coordinates**
   - `isize` allows negative offsets
   - Simplifies boundary checking math

3. **Provide Both Safe and Unsafe Access**
   - `get()/get_mut()` return `Option` for safety
   - `Index/IndexMut` traits for convenience (panics on invalid access)

4. **Support Multiple Coordinate Systems**
   - Array-style: `(row, col)`
   - Cartesian: `(x, y)` where x=col, y=row
   - Provide conversion methods

### Common Pitfalls

❌ **Using `usize` for coordinates**
```rust
let pos = (0usize, 0usize);
let new_row = pos.0 - 1;  // Panic! Underflow on unsigned subtraction
```

✅ **Using `isize` for coordinates**
```rust
let pos = (0isize, 0isize);
let new_row = pos.0 - 1;  // -1 (out of bounds, but no panic)
if new_row < 0 { /* handle boundary */ }
```

❌ **Forgetting row-major order**
```rust
let index = col * height + row;  // Wrong! Column-major
```

✅ **Using correct row-major calculation**
```rust
let index = row * width + col;  // Correct! Row-major
```

---

## 🔗 Related Topics

### Tomorrow's Preview: Day 23 - Grid Navigation
- Direction enums (N, S, E, W, NE, NW, SE, SW)
- Neighbor generation algorithms
- Bounds checking strategies
- Movement validation

### Mission Integration
- **Mission 6**: Implements full `Grid<T>` with iteration, parsing, and display
- **Mission 7**: Uses grids to represent graph connectivity matrices

### AoC Applications
- **Pathfinding**: Find shortest path through maze
- **Flood Fill**: Count connected regions
- **Game of Life**: Cellular automaton simulation
- **Map Parsing**: Convert text input to grid representation

---

*Tags: #grids #2d-arrays #coordinates #spatial-algorithms #mission6 #data-structures #aoc-patterns*
*Links: [[../missions/Mission6/README|Mission6]] | [[zettel-index]] | [[Collections MOC]] | [[daily-study/Day23]] →*
