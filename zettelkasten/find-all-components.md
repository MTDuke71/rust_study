# find_all_components - Connected Component Detection

> **Core Concept**: Systematic algorithm for discovering all distinct connected regions in a grid by sweeping and exploring each unvisited target cell.

---

## 🎯 **What It Does**

`find_all_components` is a grid analysis algorithm that:
- **Scans** every cell in a grid systematically (left-to-right, top-to-bottom)
- **Identifies** all separate connected regions matching a target value
- **Groups** cells by connectivity using 4-directional adjacency
- **Returns** a vector of components, where each component is a vector of coordinates

**Key Property**: Guarantees that no cell appears in multiple components (mutually exclusive regions).

---

## 📝 **Function Signature**

```rust
pub fn find_all_components<T: PartialEq + Clone>(
    grid: &Grid<T>,
    target: &T,
) -> Vec<Vec<Coord>>
```

**Generic Type `<T: PartialEq + Clone>`:**
- `T`: Any type the grid can hold (usually `char`, `i32`, etc.)
- `PartialEq`: Required for `==` comparison with target
- `Clone`: Required for internal operations

**Parameters:**
- `grid: &Grid<T>` - Immutable reference to grid (preserves original data)
- `target: &T` - Value to search for (e.g., `'#'` for walls, `'.'` for floors)

**Returns:**
- `Vec<Vec<Coord>>` - List of components (each component = list of coordinates)
- Example: `[[(0,0), (0,1)], [(5,5), (5,6)]]` = 2 separate regions

---

## 🔧 **Algorithm Structure**

### **Two-Phase Approach**

#### **Phase 1: Systematic Grid Sweep**
```rust
for row in 0..grid.height() {
    for col in 0..grid.width() {
        let pos = Coord::new(row as isize, col as isize);
        
        // Skip if already processed or not target
        if visited.contains(&pos) { continue; }
        if grid.get_coord(pos) != Some(target) { continue; }
        
        // Found new component - explore it completely
        let component = explore_component(grid, pos, target, &mut visited);
        components.push(component);
    }
}
```

**Key Features:**
- **Exhaustive**: Checks every cell exactly once
- **Order**: Left-to-right, top-to-bottom (like reading a book)
- **Early skip**: Uses `visited` set to avoid reprocessing

#### **Phase 2: Component Exploration (DFS)**
```rust
fn explore_component<T: PartialEq + Clone>(
    grid: &Grid<T>,
    start: Coord,
    target: &T,
    visited: &mut HashSet<Coord>,
) -> Vec<Coord> {
    let mut stack = vec![start];
    let mut component = Vec::new();
    
    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) { continue; }
        if !grid.contains(pos) { continue; }
        if grid.get_coord(pos) != Some(target) { continue; }
        
        visited.insert(pos);
        component.push(pos);
        
        // Add unvisited 4-connected neighbors to stack
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
    
    component
}
```

**Key Features:**
- **Depth-First Search**: Uses explicit stack (no recursion limits)
- **4-Connectivity**: Only horizontal/vertical neighbors (not diagonal)
- **Visited tracking**: Global set prevents revisiting cells
- **Complete exploration**: Finds ALL reachable cells from starting point

---

## 🎨 **Visual Example**

### **Input Grid:**
```
##....##
##....##
........
..###...
..###...
```

### **Execution Trace:**

**Step 1**: Sweep encounters `(0,0)` = `#`
- Not visited ✓, Matches target ✓
- Explore component from `(0,0)` using DFS
- **Result**: Component 1 = `[(0,0), (0,1), (1,0), (1,1)]`
- Mark all 4 cells as visited

**Step 2**: Sweep continues to `(0,1)`
- Already visited ✗ (skip)

**Step 3**: Sweep continues to `(0,2)` through `(0,5)`
- All are `.` (not target) → skip

**Step 4**: Sweep encounters `(0,6)` = `#`
- Not visited ✓, Matches target ✓
- Explore component from `(0,6)` using DFS
- **Result**: Component 2 = `[(0,6), (0,7), (1,6), (1,7)]`
- Mark all 4 cells as visited

**Step 5**: Continue sweep through rows 1-2 (all visited or non-target)

**Step 6**: Sweep encounters `(3,2)` = `#`
- Not visited ✓, Matches target ✓
- Explore component from `(3,2)` using DFS
- **Result**: Component 3 = `[(3,2), (3,3), (3,4), (4,2), (4,3), (4,4)]`

**Final Output:**
```rust
[
    [(0,0), (0,1), (1,0), (1,1)],     // Top-left island
    [(0,6), (0,7), (1,6), (1,7)],     // Top-right island
    [(3,2), (3,3), (3,4), (4,2), (4,3), (4,4)]  // Bottom island
]
```

---

## 🔑 **Key Design Decisions**

### **1. Global Visited Set**
```rust
let mut visited = HashSet::new();  // Shared across ALL components
```

**Why global?**
- Once explored as part of Component 1, never need to check again
- Prevents redundant DFS explorations
- O(1) lookup prevents performance degradation

**Alternative (bad):**
- Create new visited set per component → re-explores cells multiple times
- Would increase complexity from O(W×H) to O(W²×H²)

### **2. Immutable Grid Reference**
```rust
grid: &Grid<T>  // Borrow, don't modify
```

**Why immutable?**
- Only reading grid data, not changing it
- Allows multiple analyses on same grid
- Original data preserved for further operations

### **3. Mutable Visited Reference**
```rust
&mut visited  // Passed to explore_component()
```

**Why mutable?**
- Each exploration updates shared visited set
- Outer loop sees updates from inner explorations
- Prevents duplicate component detection

### **4. 4-Connectivity Rule**
```rust
pub fn neighbors_4(&self, coord: Coord) -> Vec<Coord> {
    let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    //              ↑ up    ↓ down   ← left   → right
```

**Why only 4 directions?**
- Standard for grid-based problems (AoC, competitive programming)
- Diagonal cells are NOT connected
- Example:
  ```
  #.#  ← These are TWO components (not connected diagonally)
  ...
  #.#  ← These are also separate
  ```

---

## ⚡ **Performance Analysis**

### **Time Complexity: O(W × H)**

**Outer Loop:**
- Checks every cell once: W × H iterations
- Each check: O(1) visited lookup

**Component Exploration (all DFS combined):**
- Each target cell explored exactly once across ALL components
- Each cell: O(1) visited check + up to 4 neighbor checks
- Total: N cells × 5 operations (where N ≤ W×H)

**Total:** O(W×H) + O(N×5) = **O(W×H)** ✅

### **Space Complexity: O(W × H)**

**Visited Set:**
- Worst case: all cells are target → O(W×H)

**Components Vector:**
- Worst case: every cell is its own component → O(W×H)

**DFS Stack:**
- Worst case: entire grid is one component → O(W×H) stack depth

**Total:** **O(W×H)** ✅

### **Practical Performance (20×8 dungeon)**

**Grid Size:** 160 cells
**Operation Counts:**
- Outer loop checks: 160
- DFS pop checks: ~156 (only target cells)
- Neighbor checks: ~620 (156 × 4)
- **Total operations:** ~936

**HashSet Lookups:** Each is O(1), so total is still O(160) = O(W×H) ✅

---

## 🎯 **Common Use Cases**

### **1. Island Counting**
```rust
let islands = find_all_components(&ocean, &'#');
println!("Total islands: {}", islands.len());
```

**Example:**
```
~~~#####~~~
~~~##~##~~~  → 1 island (all connected)
~~~#####~~~
```

### **2. Room Detection**
```rust
let rooms = find_all_components(&dungeon, &'.');
println!("Detected {} rooms", rooms.len());
```

**Example:**
```
################
#....#.....#...#  → 1 large corridor component
#.##.#.###.#.#.#
#.##.#.###.#.#.#  → 3 separate box interior components
################
```

### **3. Region Classification**
```rust
let components = find_all_components(&map, &'.');
for (i, region) in components.iter().enumerate() {
    let info = ComponentInfo::analyze(&map, region);
    println!("Region {}: {} cells, perimeter {}", i, info.size, info.perimeter);
}
```

### **4. Connectivity Testing**
```rust
fn are_connected(grid: &Grid<char>, pos1: Coord, pos2: Coord) -> bool {
    let component = explore_component(grid, pos1, &'.', &mut HashSet::new());
    component.contains(&pos2)
}
```

---

## 🚨 **Common Pitfalls & Solutions**

### **Pitfall 1: Diagonal Connectivity Confusion**
```rust
// WRONG: Assuming these are connected
#.#
.#.  ← Middle # connects them!
#.#

// CORRECT: These are separate
#...#
.....  ← No connection path
#...#
```

**Solution:** Remember only 4-directional (up/down/left/right) neighbors count.

### **Pitfall 2: Forgetting to Pass Mutable Visited**
```rust
// WRONG: Creates new visited set
let component = explore_component(grid, pos, target, &mut HashSet::new());

// CORRECT: Uses shared visited set
let component = explore_component(grid, pos, target, &mut visited);
```

**Solution:** Always pass the outer loop's `visited` set by mutable reference.

### **Pitfall 3: Modifying Grid During Search**
```rust
// WRONG: Mutates grid while searching
let components = find_all_components(&mut grid, &'#');  // Takes &mut
// This prevents further analysis on original data!

// CORRECT: Keep grid immutable
let components = find_all_components(&grid, &'#');  // Takes &
// Original grid preserved for further operations
```

**Solution:** Use immutable reference unless you specifically need to modify the grid.

### **Pitfall 4: Not Handling Empty Components**
```rust
// Handle case where no target cells exist
let components = find_all_components(&grid, &'X');
if components.is_empty() {
    println!("No 'X' cells found in grid");
}
```

---

## 🔗 **Related Algorithms**

### **Flood Fill (Single Component)**
```rust
// Find ONE component from a starting point
flood_fill_iterative(&mut grid, start, &'.', 'X');
```

**Difference:** Flood fill modifies the grid and finds one region. `find_all_components` finds ALL regions without modification.

### **BFS Component Detection**
```rust
// Same result, but explores in layers instead of depth
fn explore_component_bfs(...) -> Vec<Coord> {
    let mut queue = VecDeque::new();
    // ... BFS instead of DFS
}
```

**Difference:** BFS explores breadth-first (layers), DFS explores depth-first (exhaustive paths). Result is the same for component detection.

### **Union-Find (Disjoint Set)**
```rust
// Alternative approach for static grids
let mut uf = UnionFind::new(w * h);
// Union adjacent cells...
```

**Difference:** Union-Find is better for dynamic connectivity queries. `find_all_components` is simpler and sufficient for most grid problems.

---

## 📚 **Real-World Applications**

### **Advent of Code Patterns**
- **2023 Day 21**: Garden plot connectivity
- **2022 Day 12**: Hill climbing regions
- **2021 Day 9**: Basin detection (low point flooding)
- **2020 Day 17**: Active cube region tracking

### **Game Development**
- **Pathfinding**: Separate reachable areas
- **Fog of War**: Visible regions from player position
- **Level Generation**: Room connectivity validation
- **AI Navigation**: Valid movement zones

### **Image Processing**
- **Object Detection**: Connected pixel regions
- **Segmentation**: Separate color/intensity regions
- **Blob Analysis**: Shape detection and classification

---

## 💡 **Optimization Opportunities**

### **1. Early Exit for Single Component**
```rust
if components.len() == 1 {
    return components;  // Found what we need
}
```

### **2. Component Size Filtering**
```rust
let large_components: Vec<_> = components.into_iter()
    .filter(|comp| comp.len() >= min_size)
    .collect();
```

### **3. Parallel Component Analysis**
```rust
use rayon::prelude::*;
let stats: Vec<_> = components.par_iter()
    .map(|comp| ComponentInfo::analyze(grid, comp))
    .collect();
```

### **4. 8-Connectivity Variant**
```rust
pub fn neighbors_8(&self, coord: Coord) -> Vec<Coord> {
    let offsets = [
        (-1,-1), (-1,0), (-1,1),  // Top row
        ( 0,-1),         ( 0,1),  // Middle row (skip center)
        ( 1,-1), ( 1,0), ( 1,1),  // Bottom row
    ];
    // ... same pattern
}
```

---

## 🧪 **Testing Strategies**

### **Unit Tests**
```rust
#[test]
fn empty_grid_returns_empty_components() {
    let grid = Grid::from_pattern(vec!["..."]);
    let components = find_all_components(&grid, &'#');
    assert_eq!(components.len(), 0);
}

#[test]
fn single_cell_component() {
    let grid = Grid::from_pattern(vec!["#"]);
    let components = find_all_components(&grid, &'#');
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].len(), 1);
}

#[test]
fn diagonal_cells_are_separate() {
    let grid = Grid::from_pattern(vec!["#.#", "...", "#.#"]);
    let components = find_all_components(&grid, &'#');
    assert_eq!(components.len(), 4);  // Four separate cells
}
```

### **Property-Based Tests**
```rust
#[quickcheck]
fn total_cells_equals_sum_of_components(grid: Grid<char>) -> bool {
    let components = find_all_components(&grid, &'#');
    let total: usize = components.iter().map(|c| c.len()).sum();
    total == grid.count_matching(&'#')
}
```

---

## 📖 **Further Reading**

### **Related Zettelkasten Notes**
- [[explore-component]] - DFS helper function details
- [[flood-fill]] - Single-component modification algorithm
- [[component-info]] - Component analysis and metrics
- [[4-connectivity]] - Grid neighbor patterns
- [[hashset-visited]] - Visited tracking implementation
- [[grid-traversal-patterns]] - Common grid scanning approaches

### **External Resources**
- [[mission-6]] (or [[m6]]): Complete flood fill implementation with multiple algorithms
- [[daily-study/Day24]] (or [[ds-day24]]): Visual demonstrations and complete runnable examples
- **Competitive Programming**: Graph theory and connectivity problems

---

*Tags: #connected-components #dfs #grid-algorithms #mission6 #concept #implementation #aoc-patterns #daily-study*

*Links: [[zettel-index]] | [[daily-study/Day24]] | [[mission-6]] | [[flood-fill]] | [[explore-component]]*
