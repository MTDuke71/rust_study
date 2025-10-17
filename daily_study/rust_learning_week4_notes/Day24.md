# Day 24 · Grid Algorithms (Flood Fill & Connected Components)

> **Learning Context**: Day 24 introduces fundamental grid algorithms that power region detection, maze solving, and area counting - essential for Mission 6's pathfinding utilities and countless AoC problems.

**Cross-Track Integration:**
- **Mission Focus**: Flood fill enables Mission 6's region analysis and Mission 7's component detection
- **Daily Study**: Week 4, Day 3 - Core spatial algorithms building on navigation foundation
- **Rust Book**: Applies Chapter 8 (Collections) with HashSet for visited tracking

**Related Zettelkasten Notes:**
- [[Mission6 Overview]]
- [[BFS Patterns]] - Breadth-first search applications
- [[DFS Patterns]] - Depth-first search applications
- [[zettel-index]] - Main learning hub

---

## 🎯 Core Concepts

### What is Flood Fill?

**Flood Fill** is an algorithm that determines and colors a connected region in a multi-dimensional space. Think of it as the "paint bucket" tool in graphics programs.

**Real-World Applications:**
- **Image Editing**: Paint bucket tool, magic wand selection
- **Game Development**: Room detection, line-of-sight calculations
- **Maze Solving**: Finding all reachable areas from a starting point
- **AoC Problems**: Counting islands, finding regions, measuring areas

**Key Characteristics:**
- **Starting point**: Begins at a seed position
- **Spreading pattern**: Expands to adjacent cells
- **Stopping condition**: Stops at boundaries or different values
- **Result**: All connected cells matching criteria

### Connected Components

**Connected Components** are maximal sets of mutually reachable cells. Finding all components answers questions like:
- How many separate islands exist?
- What are the distinct regions in this map?
- Are two points in the same connected area?

---

## 🌊 Flood Fill Algorithms

### Recursive Flood Fill (DFS-based)

The **recursive approach** is the most intuitive: visit a cell, mark it, then recursively visit neighbors.

```rust
/// Recursive flood fill using depth-first search
/// 
/// This is the simplest implementation but has limitations:
/// - Stack overflow on large regions (recursion depth limit)
/// - Harder to control/pause mid-execution
/// - Natural for problems requiring backtracking
/// 
/// # Arguments
/// * `grid` - The grid to fill
/// * `start` - Starting coordinate
/// * `target` - Value to match (cells to fill)
/// * `replacement` - Value to replace with
/// 
/// # Example
/// ```
/// let mut grid = Grid::from_str("#####\n#...#\n#.#.#\n#...#\n#####");
/// flood_fill_recursive(&mut grid, Coord::new(1, 1), '.', 'X');
/// // All '.' cells connected to (1,1) become 'X'
/// ```
pub fn flood_fill_recursive<T: PartialEq + Clone>(
    grid: &mut Grid<T>,
    pos: Coord,
    target: &T,
    replacement: T,
) {
    // Base case 1: Out of bounds
    if !grid.contains(pos) {
        return;
    }
    
    // Base case 2: Cell doesn't match target
    if grid.get_coord(pos) != Some(target) {
        return;
    }
    
    // Base case 3: Already replaced (prevents infinite recursion)
    if grid.get_coord(pos) == Some(&replacement) {
        return;
    }
    
    // Recursive case: Replace this cell and recurse to neighbors
    let row = pos.row as usize;
    let col = pos.col as usize;
    grid[(row, col)] = replacement.clone();
    
    // Recurse to all 4-connected neighbors
    for neighbor in grid.neighbors_4(pos) {
        flood_fill_recursive(grid, neighbor, target, replacement.clone());
    }
}
```

**How Recursive Flood Fill Works:**

```
Step 1: Start at seed      Step 2: Fill & recurse up   Step 3: Fill & recurse right
#####                       #####                        #####
#S..#                       #X..#                        #XX.#
#.#.#                       #.#.#                        #.#.#
#...#                       #...#                        #...#
#####                       #####                        #####

Step 4: Continue...         Step 5: Backtrack           Final: All connected filled
#####                       #####                        #####
#XXX#                       #XXX#                        #XXX#
#.#.#                       #X#.#                        #X#X#
#...#                       #X..#                        #XXX#
#####                       #####                        #####
```

**Pros:**
- ✅ Simple to understand and implement
- ✅ Natural fit for divide-and-conquer problems
- ✅ Minimal memory overhead (just call stack)

**Cons:**
- ❌ Stack overflow on large regions (typically ~1000-10000 cells)
- ❌ Hard to control execution flow
- ❌ Difficult to collect metadata during traversal

### Iterative Flood Fill (Explicit Stack)

The **iterative approach** uses an explicit stack to avoid recursion limits.

```rust
use std::collections::HashSet;

/// Iterative flood fill using explicit stack (DFS)
/// 
/// This avoids stack overflow by using heap-allocated stack.
/// Can handle arbitrarily large regions limited only by memory.
/// 
/// # Returns
/// Vector of all coordinates that were filled
pub fn flood_fill_iterative<T: PartialEq + Clone>(
    grid: &mut Grid<T>,
    start: Coord,
    target: &T,
    replacement: T,
) -> Vec<Coord> {
    // Early return if starting position is invalid
    if grid.get_coord(start) != Some(target) {
        return Vec::new();
    }
    
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let mut filled = Vec::new();
    
    while let Some(pos) = stack.pop() {
        // Skip if already visited
        if visited.contains(&pos) {
            continue;
        }
        
        // Skip if out of bounds
        if !grid.contains(pos) {
            continue;
        }
        
        // Skip if doesn't match target
        if grid.get_coord(pos) != Some(target) {
            continue;
        }
        
        // Mark as visited
        visited.insert(pos);
        
        // Replace the cell
        let row = pos.row as usize;
        let col = pos.col as usize;
        grid[(row, col)] = replacement.clone();
        filled.push(pos);
        
        // Add neighbors to stack
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    
    filled
}
```

**Key Differences from Recursive:**
- Uses `Vec` as explicit stack instead of call stack
- Uses `HashSet` to track visited cells
- Can return metadata (list of filled cells)
- No stack overflow risk

### Queue-Based Flood Fill (BFS)

The **breadth-first approach** fills outward in concentric layers.

```rust
use std::collections::{HashSet, VecDeque};

/// Queue-based flood fill using breadth-first search
/// 
/// Fills outward in layers from the starting point.
/// Useful when you need to know distance from origin.
/// 
/// # Returns
/// Vector of (coordinate, distance) pairs showing fill order
pub fn flood_fill_bfs<T: PartialEq + Clone>(
    grid: &mut Grid<T>,
    start: Coord,
    target: &T,
    replacement: T,
) -> Vec<(Coord, usize)> {
    if grid.get_coord(start) != Some(target) {
        return Vec::new();
    }
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut filled = Vec::new();
    
    queue.push_back((start, 0));
    visited.insert(start);
    
    while let Some((pos, distance)) = queue.pop_front() {
        // Skip if out of bounds or doesn't match
        if !grid.contains(pos) || grid.get_coord(pos) != Some(target) {
            continue;
        }
        
        // Replace the cell
        let row = pos.row as usize;
        let col = pos.col as usize;
        grid[(row, col)] = replacement.clone();
        filled.push((pos, distance));
        
        // Add neighbors to queue with increased distance
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
    
    filled
}
```

**BFS Fill Pattern:**
```
Distance 0: S          Distance 1: X          Distance 2: X
#####                  #####                  #####
#S..#                  #X..#                  #XX.#
#.#.#                  #X#.#                  #XX#X
#...#                  #...#                  #XX.#
#####                  #####                  #####
```

**Advantages of BFS:**
- ✅ Fills in distance layers
- ✅ Can calculate distances from origin
- ✅ Useful for shortest path problems

**Disadvantages:**
- ❌ Higher memory usage (queue + visited set)
- ❌ Slower than stack-based for simple fills

---

## 🏝️ Connected Components

### Finding All Connected Components

**Problem**: Given a grid, find all distinct connected regions.

```rust
/// Finds all connected components in a grid
/// 
/// Returns a vector where each element is a vector of coordinates
/// forming one connected component.
/// 
/// # Arguments
/// * `grid` - Grid to analyze
/// * `target` - Value to consider as "connected"
/// 
/// # Example
/// ```
/// // Grid:  ###.###
/// //        #.#.#.#
/// //        ###.###
/// // Returns 2 components (left and right groups of #)
/// ```
pub fn find_all_components<T: PartialEq + Clone>(
    grid: &Grid<T>,
    target: &T,
) -> Vec<Vec<Coord>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    
    // Check every cell in the grid
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let pos = Coord::new(row as isize, col as isize);
            
            // Skip if already visited or not target value
            if visited.contains(&pos) {
                continue;
            }
            if grid.get_coord(pos) != Some(target) {
                continue;
            }
            
            // Found a new component - explore it completely
            let component = explore_component(grid, pos, target, &mut visited);
            if !component.is_empty() {
                components.push(component);
            }
        }
    }
    
    components
}

/// Explores a single connected component from a starting point
fn explore_component<T: PartialEq + Clone>(
    grid: &Grid<T>,
    start: Coord,
    target: &T,
    visited: &mut HashSet<Coord>,
) -> Vec<Coord> {
    let mut stack = vec![start];
    let mut component = Vec::new();
    
    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }
        if !grid.contains(pos) {
            continue;
        }
        if grid.get_coord(pos) != Some(target) {
            continue;
        }
        
        visited.insert(pos);
        component.push(pos);
        
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    
    component
}
```

### Component Analysis

Once you have components, you can analyze them:

```rust
/// Analyzes properties of a connected component
#[derive(Debug)]
pub struct ComponentInfo {
    pub size: usize,           // Number of cells
    pub perimeter: usize,      // External edges
    pub bounds: (Coord, Coord), // Top-left and bottom-right corners
}

impl ComponentInfo {
    /// Analyzes a component and returns its properties
    pub fn analyze<T: Clone>(grid: &Grid<T>, component: &[Coord]) -> Self {
        let mut min_row = isize::MAX;
        let mut max_row = isize::MIN;
        let mut min_col = isize::MAX;
        let mut max_col = isize::MIN;
        let mut perimeter = 0;
        
        let component_set: HashSet<_> = component.iter().copied().collect();
        
        for &pos in component {
            // Update bounds
            min_row = min_row.min(pos.row);
            max_row = max_row.max(pos.row);
            min_col = min_col.min(pos.col);
            max_col = max_col.max(pos.col);
            
            // Count perimeter (edges touching non-component cells)
            for neighbor in grid.neighbors_4(pos) {
                if !component_set.contains(&neighbor) {
                    perimeter += 1;
                }
            }
            
            // Also count grid boundary edges
            if pos.row == 0 { perimeter += 1; }
            if pos.row == (grid.height() - 1) as isize { perimeter += 1; }
            if pos.col == 0 { perimeter += 1; }
            if pos.col == (grid.width() - 1) as isize { perimeter += 1; }
        }
        
        Self {
            size: component.len(),
            perimeter,
            bounds: (
                Coord::new(min_row, min_col),
                Coord::new(max_row, max_col),
            ),
        }
    }
}
```

---

## 🚀 Complete Runnable Example

```rust
use std::collections::{HashSet, VecDeque};
use std::fmt;

fn main() {
    println!("=== Day 24: Grid Algorithms Demo ===\n");
    
    // 1. Basic Flood Fill
    println!("🔷 1. Recursive Flood Fill");
    println!("==========================");
    
    let mut grid1 = Grid::from_pattern(vec![
        "#####",
        "#...#",
        "#.#.#",
        "#...#",
        "#####",
    ]);
    
    println!("Original grid:");
    println!("{}", grid1);
    
    let start = Coord::new(1, 1);
    flood_fill_recursive(&mut grid1, start, &'.', 'X');
    
    println!("After flood fill from (1,1):");
    println!("{}", grid1);
    
    // 2. Iterative Flood Fill with Metadata
    println!("\n🔷 2. Iterative Flood Fill (with cell count)");
    println!("============================================");
    
    let mut grid2 = Grid::from_pattern(vec![
        "......",
        ".####.",
        ".#..#.",
        ".#..#.",
        ".####.",
        "......",
    ]);
    
    println!("Original grid:");
    println!("{}", grid2);
    
    let filled = flood_fill_iterative(&mut grid2, Coord::new(2, 2), &'.', 'O');
    
    println!("After filling interior (filled {} cells):", filled.len());
    println!("{}", grid2);
    
    // 3. BFS Flood Fill with Distances
    println!("\n🔷 3. BFS Flood Fill (showing distances)");
    println!("========================================");
    
    let mut grid3 = Grid::from_pattern(vec![
        "........",
        "..####..",
        ".#....#.",
        ".#.##.#.",
        ".#....#.",
        "..####..",
        "........",
    ]);
    
    println!("Original grid:");
    println!("{}", grid3);
    
    let filled_with_distance = flood_fill_bfs(&mut grid3, Coord::new(3, 2), &'.', 'X');
    
    println!("\nFill progression (coord, distance):");
    for (pos, dist) in filled_with_distance.iter().take(10) {
        println!("  {:?} at distance {}", pos, dist);
    }
    println!("  ... ({} cells total)", filled_with_distance.len());
    
    // 4. Finding Connected Components
    println!("\n🔷 4. Finding Connected Components");
    println!("==================================");
    
    let grid4 = Grid::from_pattern(vec![
        "##.....##",
        "##.....##",
        ".........",
        "...###...",
        "...###...",
        ".........",
        "#........",
        "##.......",
    ]);
    
    println!("Grid with multiple regions:");
    println!("{}", grid4);
    
    let components = find_all_components(&grid4, &'#');
    
    println!("\nFound {} connected components:", components.len());
    for (i, component) in components.iter().enumerate() {
        let info = ComponentInfo::analyze(&grid4, component);
        println!("  Component {}: {} cells, perimeter {}, bounds {:?}",
            i + 1, info.size, info.perimeter, info.bounds);
    }
    
    // 5. Island Counting (AoC Pattern)
    println!("\n🔷 5. Island Counting Problem");
    println!("=============================");
    
    let ocean = Grid::from_pattern(vec![
        "~~~~~#####~~~~~",
        "~~~~##~~~##~~~~",
        "~~~##~~~~~##~~~",
        "~~##~~~~~~~##~~",
        "~#####~#####~~~",
        "~~~~~~~#~~~~~~~",
        "~~~~~~###~~~~~~",
        "~~~~~#####~~~~~",
        "~~~~###~###~~~~",
        "~~~~~~#~#~~~~~~",
    ]);
    
    println!("Ocean map (~ = water, # = land):");
    println!("{}", ocean);
    
    let islands = find_all_components(&ocean, &'#');
    
    println!("\nIsland analysis:");
    println!("  Total islands: {}", islands.len());
    for (i, island) in islands.iter().enumerate() {
        let info = ComponentInfo::analyze(&ocean, island);
        println!("  Island {}: {} cells, perimeter {}",
            i + 1, info.size, info.perimeter);
    }
    
    // Visualize each island separately
    println!("\nVisualized islands (marked as 'O'):");
    for (i, island) in islands.iter().enumerate() {
        let mut highlighted = ocean.clone();
        // Mark this island with 'O'
        for &pos in island {
            let row = pos.row as usize;
            let col = pos.col as usize;
            highlighted[(row, col)] = 'O';
        }
        println!("\n  Island {}:", i + 1);
        println!("{}", highlighted);
    }
    
    // Visualize perimeter cells with edge counts
    println!("\nPerimeter edge contribution per cell:");
    println!("(Shows how many edges each cell contributes to total perimeter)");
    for (i, island) in islands.iter().enumerate() {
        let mut perimeter_vis = ocean.clone();
        let island_set: HashSet<_> = island.iter().copied().collect();
        
        // Calculate edge contribution for each cell
        for &pos in island {
            let mut exposed_edges = 0;
            for neighbor in ocean.neighbors_4(pos) {
                if !island_set.contains(&neighbor) {
                    exposed_edges += 1;
                }
            }
            
            let row = pos.row as usize;
            let col = pos.col as usize;
            
            // Display edge count: 0=interior, 1-4=perimeter with count
            perimeter_vis[(row, col)] = match exposed_edges {
                0 => '.',  // Interior cell (no exposed edges)
                1 => '1',  // 1 edge exposed
                2 => '2',  // 2 edges exposed
                3 => '3',  // 3 edges exposed
                4 => '4',  // 4 edges exposed (isolated cell)
                _ => '?',  // Should never happen
            };
        }
        
        println!("\n  Island {} - Edge contributions per cell:", i + 1);
        println!("{}", perimeter_vis);
        
        // Calculate and show statistics
        let mut edge_counts = vec![0; 5]; // Index = number of exposed edges
        for &pos in island {
            let mut exposed_edges = 0;
            for neighbor in ocean.neighbors_4(pos) {
                if !island_set.contains(&neighbor) {
                    exposed_edges += 1;
                }
            }
            edge_counts[exposed_edges] += 1;
        }
        
        println!("\n  Edge contribution breakdown:");
        if edge_counts[0] > 0 {
            println!("    . (0 edges): {} interior cells", edge_counts[0]);
        }
        for edges in 1..=4 {
            if edge_counts[edges] > 0 {
                println!("    {} ({} edges): {} cells contributing {} total edges",
                    edges, edges, edge_counts[edges], edge_counts[edges] * edges);
            }
        }
        
        let total_perimeter: usize = (1..=4)
            .map(|edges| edge_counts[edges] * edges)
            .sum();
        println!("  Total perimeter: {} edges", total_perimeter);
    }
    
    // 6. Practical Application: Room Detection
    println!("\n🔷 6. Room Detection in Dungeon");
    println!("===============================");
    
    let dungeon = Grid::from_pattern(vec![
        "####################",
        "#..................#",
        "#.####.#####.#####.#",
        "#.#..#.#...#.#...#.#",
        "#.#..#.#...#.#...#.#",
        "#.####.#####.#####.#",
        "#..................#",
        "####################",
    ]);
    
    println!("Dungeon layout:");
    println!("{}", dungeon);
    
    let rooms = find_all_components(&dungeon, &'.');
    
    println!("\nDetected {} room(s):", rooms.len());
    for (i, room) in rooms.iter().enumerate() {
        let info = ComponentInfo::analyze(&dungeon, room);
        let (min_bound, max_bound) = info.bounds;
        let width = (max_bound.col - min_bound.col + 1) as usize;
        let height = (max_bound.row - min_bound.row + 1) as usize;
        
        println!("  Room {}: {} tiles, {}x{} bounding box",
            i + 1, info.size, width, height);
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

impl Grid<char> {
    pub fn from_pattern(lines: Vec<&str>) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);
        
        for line in lines {
            for ch in line.chars() {
                data.push(ch);
            }
        }
        
        Self { data, width, height }
    }
}

impl<T: Clone> Grid<T> {
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    
    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        }
    }
    
    #[allow(dead_code)]
    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
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
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.iter()
            .map(|(dr, dc)| Coord::new(coord.row + dr, coord.col + dc))
            .filter(|&pos| self.contains(pos))
            .collect()
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let idx = row * self.width + col;
        &self.data[idx]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let idx = row * self.width + col;
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

// Flood fill implementations
fn flood_fill_recursive<T: PartialEq + Clone>(
    grid: &mut Grid<T>,
    pos: Coord,
    target: &T,
    replacement: T,
) {
    if !grid.contains(pos) {
        return;
    }
    if grid.get_coord(pos) != Some(target) {
        return;
    }
    if grid.get_coord(pos) == Some(&replacement) {
        return;
    }
    
    let row = pos.row as usize;
    let col = pos.col as usize;
    grid[(row, col)] = replacement.clone();
    
    for neighbor in grid.neighbors_4(pos) {
        flood_fill_recursive(grid, neighbor, target, replacement.clone());
    }
}

fn flood_fill_iterative<T: PartialEq + Clone>(
    grid: &mut Grid<T>,
    start: Coord,
    target: &T,
    replacement: T,
) -> Vec<Coord> {
    if grid.get_coord(start) != Some(target) {
        return Vec::new();
    }
    
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let mut filled = Vec::new();
    
    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }
        if !grid.contains(pos) {
            continue;
        }
        if grid.get_coord(pos) != Some(target) {
            continue;
        }
        
        visited.insert(pos);
        
        let row = pos.row as usize;
        let col = pos.col as usize;
        grid[(row, col)] = replacement.clone();
        filled.push(pos);
        
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    
    filled
}

fn flood_fill_bfs<T: PartialEq + Clone>(
    grid: &mut Grid<T>,
    start: Coord,
    target: &T,
    replacement: T,
) -> Vec<(Coord, usize)> {
    if grid.get_coord(start) != Some(target) {
        return Vec::new();
    }
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut filled = Vec::new();
    
    queue.push_back((start, 0));
    visited.insert(start);
    
    while let Some((pos, distance)) = queue.pop_front() {
        // Skip if out of bounds
        if !grid.contains(pos) {
            continue;
        }
        
        // Skip if doesn't match target (already replaced or wrong value)
        if grid.get_coord(pos) != Some(target) {
            continue;
        }
        
        // Replace the cell
        let row = pos.row as usize;
        let col = pos.col as usize;
        grid[(row, col)] = replacement.clone();
        filled.push((pos, distance));
        
        // Add unvisited neighbors to queue
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
    
    filled
}

fn find_all_components<T: PartialEq + Clone>(
    grid: &Grid<T>,
    target: &T,
) -> Vec<Vec<Coord>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let pos = Coord::new(row as isize, col as isize);
            
            if visited.contains(&pos) {
                continue;
            }
            if grid.get_coord(pos) != Some(target) {
                continue;
            }
            
            let component = explore_component(grid, pos, target, &mut visited);
            if !component.is_empty() {
                components.push(component);
            }
        }
    }
    
    components
}

fn explore_component<T: PartialEq + Clone>(
    grid: &Grid<T>,
    start: Coord,
    target: &T,
    visited: &mut HashSet<Coord>,
) -> Vec<Coord> {
    let mut stack = vec![start];
    let mut component = Vec::new();
    
    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }
        if !grid.contains(pos) {
            continue;
        }
        if grid.get_coord(pos) != Some(target) {
            continue;
        }
        
        visited.insert(pos);
        component.push(pos);
        
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    
    component
}

#[derive(Debug)]
pub struct ComponentInfo {
    pub size: usize,
    pub perimeter: usize,
    pub bounds: (Coord, Coord),
}

impl ComponentInfo {
    pub fn analyze<T: Clone>(grid: &Grid<T>, component: &[Coord]) -> Self {
        let mut min_row = isize::MAX;
        let mut max_row = isize::MIN;
        let mut min_col = isize::MAX;
        let mut max_col = isize::MIN;
        let mut perimeter = 0;
        
        let component_set: HashSet<_> = component.iter().copied().collect();
        
        for &pos in component {
            min_row = min_row.min(pos.row);
            max_row = max_row.max(pos.row);
            min_col = min_col.min(pos.col);
            max_col = max_col.max(pos.col);
            
            for neighbor in grid.neighbors_4(pos) {
                if !component_set.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }
        
        Self {
            size: component.len(),
            perimeter,
            bounds: (
                Coord::new(min_row, min_col),
                Coord::new(max_row, max_col),
            ),
        }
    }
}
```

### 🛠️ How to Run This Code:

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day24_demo.rs` and run `rustc day24_demo.rs && ./day24_demo`
3. **In this workspace**: Create a new cargo project and paste into `src/main.rs`

---

## 💡 Key Takeaways

### Algorithm Selection Guide

**Use Recursive Flood Fill when:**
- ✅ Region size is small (< 1000 cells)
- ✅ Code simplicity is priority
- ✅ No metadata collection needed

**Use Iterative Flood Fill when:**
- ✅ Regions might be large
- ✅ Need to collect filled cells
- ✅ Want predictable memory usage

**Use BFS Flood Fill when:**
- ✅ Need distance information
- ✅ Want layer-by-layer filling
- ✅ Implementing shortest path variants

### Performance Characteristics

| Algorithm | Time Complexity | Space Complexity | Stack Safe? |
|-----------|----------------|------------------|-------------|
| Recursive DFS | O(W × H) | O(W × H) call stack | ❌ No |
| Iterative DFS | O(W × H) | O(W × H) heap | ✅ Yes |
| BFS | O(W × H) | O(W × H) heap | ✅ Yes |

### Common Patterns

**Pattern: Counting Islands**
```rust
let islands = find_all_components(&grid, &'#');
println!("Number of islands: {}", islands.len());
```

**Pattern: Finding Largest Region**
```rust
let components = find_all_components(&grid, &'.');
let largest = components.iter()
    .max_by_key(|comp| comp.len())
    .unwrap();
println!("Largest region: {} cells", largest.len());
```

**Pattern: Checking Connectivity**
```rust
fn are_connected(grid: &Grid<char>, pos1: Coord, pos2: Coord) -> bool {
    let component = explore_component(grid, pos1, &'.', &mut HashSet::new());
    component.contains(&pos2)
}
```

---

## 🔗 Related Topics

### Tomorrow's Preview: Day 25 - Queue Applications
- BFS for shortest paths
- Level-order traversal
- Queue-based problem patterns
- Efficient queue implementations

### Mission Integration
- **Mission 6**: Complete flood fill implementation with multiple algorithms
- **Mission 7**: Component detection for graph analysis

### AoC Applications
- **Island Counting**: Count disconnected land masses
- **Region Coloring**: Fill enclosed areas
- **Maze Analysis**: Find all reachable cells
- **Perimeter Calculation**: Measure region boundaries

---

*Tags: #flood-fill #connected-components #dfs #bfs #grid-algorithms #mission6 #graph-theory #aoc-patterns*
*Links: [[daily-study/Day23]] ← | [[Mission6 Overview]] | [[zettel-index]] | [[daily-study/Day25]] →*
