# 🗺️ Mission 6: 2D Grids & Navigation - Spatial Data Structures

**V-Cycle implementation of production-grade spatial data structures for competitive programming and real-world applications**

---

## 🎯 Mission Focus

Mission 6 implements **2D grid data structures and navigation algorithms**, providing foundational spatial problem-solving capabilities:

- **Generic Grid<T>** with row-major memory layout and safe bounds checking
- **Coordinate navigation** with 2D coordinate systems and direction-based movement
- **Pathfinding algorithms** (BFS, A*, Dijkstra) for unweighted and weighted grids
- **AoC utilities** including flood fill, connected components, and grid parsing
- **Performance optimization** with cache-friendly access patterns
- **Mission integration** with HashMap/HashSet for coordinate storage

Sixth mission implementing grids with navigation algorithms, coordinate systems, and spatial data management.

---

## 📖 Mission Resources

### **Main Implementation**
- **[[../../missions/Mission6/README.md|Mission 6 README]]** - Complete V-Cycle documentation
- **[[../../missions/Mission6/src/lib.rs|Grid Implementation]]** - 2D grid, coordinates, and navigation algorithms
- **[[../../missions/Mission6/src/lib.rs|Implementation & Tests]]** - Coordinate, boundary validation, and algorithm tests

### **Tutorial Integration**
- **[[../../tutorials/Mission6_tut/README.md|Mission 6 Tutorial]]** - Grid learning progression
- **Tutorial Focus**: Coordinate systems → Navigation algorithms → Pathfinding
- **Step 3**: Coordinates and direction-based movement
- **Step 5**: AoC utilities (flood fill, connected components)

### **Examples**
- `missions/Mission6/examples/` - Grid usage demonstrations
- Pathfinding algorithm examples
- Flood fill and connected component analysis

### **Coverage Reports**
- **[[../../missions/Mission6/COVERAGE_IMPROVEMENT_LOG|Coverage Improvement Log]]** - Test coverage improvements and strategies

---

## 🎯 Mission Requirements

### **REQ-1: Grid Structure**
- Generic `Grid<T>` with row-major memory layout
- Fixed-size rectangular grids (width × height)
- Safe bounds checking, O(1) element access
- **Pattern**: Generic programming with flexible type support
- **Connected to**: [[../ownership-fundamentals|Ownership Fundamentals]]

### **REQ-2: Coordinate Navigation**
- 2D coordinate system with `Coord(x, y)` representation
- Direction-based movement (8-directional support)
- Distance calculations:
  - **[[../Chebyshev Distance|Chebyshev Distance]]** - Chessboard/8-connected distance
  - **[[../Manhattan Distance|Manhattan Distance]]** - 4-connected grid distance
  - **[[../Euclidean Distance|Euclidean Distance]]** - Continuous space distance
- Neighbor finding (4-connected and 8-connected)
- **Tutorial**: Mission6_tut Step 3 - Coordinates

### **REQ-3: Pathfinding Algorithms**
- BFS for unweighted shortest paths
- A* algorithm with customizable heuristics
- Dijkstra's algorithm for weighted graphs
- **Connected to**: [[../daily-study/rust_learning_week4_notes/Day22|Day 22 - Pathfinding]]
- **Integration**: Graph algorithms on spatial data

### **REQ-4: AoC Utilities**
- Grid parsing from text input
- Flood fill operations (4-connected and 8-connected)
- Connected component analysis
- Pattern recognition and cellular automaton simulation
- **Tutorial**: Mission6_tut Step 5 - AoC Utilities
- **Applications**: Real-world Advent of Code patterns

### **REQ-5: Performance Optimization**
- Memory-efficient representation with minimal overhead
- Cache-friendly access patterns
- Iterator-based operations for zero-cost abstractions
- **Benchmarking**: Performance analysis and optimization

### **REQ-6: Mission Integration**
- HashMap/HashSet compatibility for coordinate storage
- Stack/Queue integration for pathfinding algorithms
- Consistent error handling and API patterns
- **Integration with**: [[mission-5|Mission 5]] (HashMap for coordinates)

---

## 🔗 Cross-Track Integration

### **Mission Connections**
- **[[mission-5|Mission 5]]** - Previous: HashMap (used for coordinate storage in pathfinding)
- **[[mission-7|Mission 7]]** - Next: Graph algorithms (builds on grid concepts)
- **[[mission-1|Mission 1]]** - Stack/Queue for BFS/DFS traversal
- **[[mission-8|Mission 8]]** - Graph trait implementations applied to grids
- **Spatial algorithms**: Foundation for graph traversal

### **Daily Study Connections**
- **[[../daily-study/rust_learning_week4_notes/Day19|Day 19]]** - Grid navigation patterns
- **[[../daily-study/rust_learning_week4_notes/Day20|Day 20]]** - Coordinate systems
- **[[../daily-study/rust_learning_week4_notes/Day22|Day 22]]** - 2D array foundations and pathfinding
- **[[../daily-study/rust_learning_week4_notes/Day23|Day 23]]** - Direction-based movement
- **[[../daily-study/rust_learning_week4_notes/Day24|Day 24]]** - Flood fill algorithms
- **[[../daily-study/rust_learning_week4_notes/Day25|Day 25]]** - BFS implementation

### **Rust Book Connections**
- **[[../rust_book/rust-book-ch6|Chapter 6]]** - Enums (Direction enum and pattern matching)
- **[[../rust_book/rust-book-ch7|Chapter 7]]** - Modules (organizing grid, coord, pathfinding modules)
- **[[../rust_book/rust-book-ch8|Chapter 8]]** - Collections (Vec-based grid storage strategies)
- **[[../rust_book/rust-book-ch10|Chapter 10]]** - Generics (generic Grid<T> implementation)

### **Algorithm Concepts**
- **[[../flood-fill|Flood Fill]]** - Connected region algorithms
- **[[../find-all-components|Find All Components]]** - Connected component detection
- **[[../4-connectivity|4-Connectivity]]** - Grid neighbor patterns
- **[[../Coordinate Systems|Coordinate Systems]]** - Index/coordinate conversions
- **[[../Pathfinding Algorithms|Pathfinding Algorithms]]** - BFS, A*, Dijkstra

---

## 🔬 API Design

### **Grid<T> - Generic 2D Grid**
```rust
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self
        where T: Clone
    pub fn from_vec(data: Vec<T>, width: usize) -> Option<Self>
    pub fn get(&self, x: usize, y: usize) -> Option<&T>
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T>
    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool
    pub fn in_bounds(&self, x: usize, y: usize) -> bool
    pub fn width(&self) -> usize
    pub fn height(&self) -> usize
    pub fn iter(&self) -> impl Iterator<Item = &T>
    pub fn enumerate(&self) -> impl Iterator<Item = (Coord, &T)>
}
```

### **Coord - 2D Coordinate**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self
    pub fn manhattan_distance(&self, other: &Coord) -> i32
    pub fn euclidean_distance(&self, other: &Coord) -> f64
    pub fn chebyshev_distance(&self, other: &Coord) -> i32
    pub fn neighbors_4(&self) -> Vec<Coord>
    pub fn neighbors_8(&self) -> Vec<Coord>
    pub fn move_dir(&self, dir: Direction) -> Coord
}
```

### **Direction - 8-Directional Movement**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North, South, East, West,
    NorthEast, NorthWest, SouthEast, SouthWest,
}

impl Direction {
    pub fn offset(&self) -> (i32, i32)
    pub fn rotate_cw(&self) -> Direction
    pub fn rotate_ccw(&self) -> Direction
    pub fn opposite(&self) -> Direction
}
```

---

## 📈 Performance Characteristics

### **Grid Operations**
| Operation | Time | Space | Notes |
|-----------|------|-------|-------|
| `get/set` | O(1) | O(1) | Direct index calculation |
| `in_bounds` | O(1) | O(1) | Simple comparison |
| `neighbors` | O(1) | O(k) | k = 4 or 8 neighbors |
| **Storage** | O(w×h) | - | Row-major Vec storage |

### **Pathfinding Algorithms**
| Algorithm | Time | Space | Notes |
|-----------|------|-------|-------|
| **BFS** | O(V+E) | O(V) | V=cells, E=connections |
| **A*** | O(V log V) | O(V) | With binary heap |
| **Dijkstra** | O(V log V) | O(V) | Weighted graphs |
| **Flood Fill** | O(V) | O(V) | Connected components |

### **Memory Layout**
- **Row-major storage**: `data[y * width + x]` for cache efficiency
- **Coordinate struct**: 8 bytes (two i32 fields)
- **Grid overhead**: 24 bytes (Vec + width + height)

---

## 🎓 Key Concepts & Patterns

### **Row-Major vs Column-Major**

**Row-Major (Mission 6)**:
```rust
// Contiguous rows in memory
data[y * width + x]  // Good cache locality for row iteration
```

**Benefits**:
- ✅ **Cache-friendly** for horizontal traversal
- ✅ **Natural for text parsing** (line-by-line input)
- ✅ **Standard in most languages**

### **Coordinate System Design**

**Index-based (internal)**:
```rust
let index = y * width + x;  // Vec indexing
```

**Coordinate-based (public API)**:
```rust
let coord = Coord::new(x, y);  // User-facing
grid.get(coord.x, coord.y)
```

**Hash compatibility**:
```rust
// Coord implements Hash, Eq for HashMap/HashSet
let mut visited: HashSet<Coord> = HashSet::new();
visited.insert(Coord::new(5, 10));
```

### **Distance Metrics**

**Manhattan (4-connected)**:
```rust
fn manhattan_distance(a: Coord, b: Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
```

**Chebyshev (8-connected)**:
```rust
fn chebyshev_distance(a: Coord, b: Coord) -> i32 {
    max((a.x - b.x).abs(), (a.y - b.y).abs())
}
```

**Euclidean (continuous)**:
```rust
fn euclidean_distance(a: Coord, b: Coord) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    (dx * dx + dy * dy).sqrt()
}
```

### **Flood Fill Pattern**

**4-connected flood fill**:
```rust
fn flood_fill(grid: &mut Grid<char>, start: Coord, fill: char) {
    let target = grid.get(start.x, start.y).cloned();
    let mut stack = vec![start];
    
    while let Some(coord) = stack.pop() {
        if grid.get(coord.x, coord.y) == Some(&target) {
            grid.set(coord.x, coord.y, fill);
            for neighbor in coord.neighbors_4() {
                if grid.in_bounds(neighbor.x, neighbor.y) {
                    stack.push(neighbor);
                }
            }
        }
    }
}
```

---

## 🚀 Real-World Applications

### **Game Development**
- **Tile-based worlds** - 2D games, roguelikes
- **Collision detection** - Grid-based physics
- **Pathfinding** - AI navigation around obstacles
- **Field of view** - Line-of-sight calculations

### **Image Processing**
- **Pixel manipulation** - Filters, transformations
- **Connected components** - Object detection
- **Flood fill** - Selection tools, region growing
- **Convolution** - Edge detection, blurring

### **Robotics**
- **Path planning** - Obstacle avoidance
- **Occupancy grids** - Environment mapping
- **Navigation** - Shortest path to goal
- **Spatial queries** - Nearest obstacle detection

### **Geographic Systems**
- **Map data** - Tile-based rendering
- **Spatial queries** - Region finding
- **Elevation maps** - Terrain analysis
- **Distance calculations** - Multi-source queries

### **Advent of Code Patterns (REQ-4)**
```rust
// Grid parsing from text
fn parse_grid(input: &str) -> Grid<char> {
    let lines: Vec<_> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let data: Vec<char> = lines.iter()
        .flat_map(|line| line.chars())
        .collect();
    Grid::from_vec(data, width).unwrap()
}

// Connected component counting
fn count_regions(grid: &Grid<char>, target: char) -> usize {
    let mut visited = HashSet::new();
    let mut count = 0;
    
    for (coord, &cell) in grid.enumerate() {
        if cell == target && !visited.contains(&coord) {
            flood_fill_set(grid, coord, &mut visited);
            count += 1;
        }
    }
    count
}
```

---

## 🎄 AoC Applications

### **Supported Problem Categories**
- **Grid Traversal**: BFS/DFS on 2D maps with obstacles
- **Flood Fill**: Connected component analysis, region finding
- **Pathfinding**: Shortest path with various constraints
- **Pattern Recognition**: Cellular automata, shape detection
- **Distance Calculations**: Multi-source BFS, Voronoi diagrams

### **Historical AoC Problems**
- **2015 Day 18**: Conway's Game of Life simulation
- **2021 Day 9**: Heightmap analysis with flood fill
- **2022 Day 12**: Hill climbing with elevation constraints
- **2023 Day 18**: Lavaduct lagoon area calculation

### **Mission 6 Integration Example**
- **[[../aoc2024-day4-mission6-example|AoC 2024 Day 4]]** - Word search with 43% code reduction
- Real-world validation of grid utility design
- Competitive programming pattern demonstration

---

## 📊 Current Progress

- 🔄 **REQ-1**: Grid structure (generic Grid<T> implementation)
- ⏳ **REQ-2**: Coordinate navigation (Coord, Direction, distances)
- ⏳ **REQ-3**: Pathfinding algorithms (BFS, A*, Dijkstra)
- ⏳ **REQ-4**: AoC utilities (flood fill, parsing, connected components)
- ⏳ **REQ-5**: Performance optimization (cache patterns, benchmarking)
- ⏳ **REQ-6**: Mission integration (HashMap/HashSet compatibility)

---

## 🧪 Testing Philosophy

Mission 6 maintains comprehensive requirement tracing:

```rust
#[test] // REQ-1: Grid structure
fn req1_generic_grid_creation() { ... }

#[test] // REQ-2: Coordinate navigation
fn req2_coordinate_distance_calculations() { ... }

#[test] // REQ-3: Pathfinding
fn req3_bfs_shortest_path() { ... }

#[test] // REQ-4: AoC utilities
fn req4_flood_fill_connected_components() { ... }

#[test] // REQ-5: Performance
fn req5_cache_friendly_access() { ... }

#[test] // REQ-6: Integration
fn req6_hashmap_coordinate_storage() { ... }
```

---

## 🏆 Key Learning Outcomes

### **Technical Skills**
- **Grid memory layout** - Row-major storage and indexing
- **Coordinate systems** - Index/coordinate conversions
- **Pathfinding algorithms** - BFS, A*, Dijkstra implementations
- **Flood fill operations** - Connected component analysis
- **Distance metrics** - Manhattan, Euclidean, Chebyshev
- **Iterator patterns** - Efficient grid traversal

### **Engineering Skills**
- **V-Cycle methodology** - Requirements-driven development
- **Spatial algorithm design** - Efficient 2D problem solving
- **Performance benchmarking** - Measuring and optimizing grid operations
- **Generic type design** - Flexible, reusable spatial structures
- **Cache-friendly patterns** - Optimizing memory access

### **Advanced Patterns**
- **Row-major layout** - Cache-efficient storage
- **Coordinate abstraction** - Clean public API design
- **Direction enum** - Type-safe movement
- **Hash compatibility** - Coord as HashMap key
- **Iterator composition** - Zero-cost traversal

---

## 💡 Key Takeaways

1. **Row-major is cache-friendly** - Contiguous row storage
2. **Coordinate abstraction is clean** - Hide index calculations
3. **Distance metrics matter** - Choose based on connectivity
4. **Flood fill is powerful** - Connected component workhorse
5. **Integration is key** - HashMap for visited sets, Stack for DFS
6. **Generics enable reuse** - Grid<T> works for any type
7. **AoC validates design** - Real problems prove utility

---

## 🔮 Next Steps

1. **Complete REQ-1 Grid Structure** - Finish generic Grid<T>
2. **Implement REQ-2 Coordinates** - Coord, Direction, distances
3. **Add REQ-3 Pathfinding** - BFS, A*, Dijkstra algorithms
4. **Create REQ-4 AoC Utilities** - Flood fill, parsing helpers
5. **Optimize REQ-5 Performance** - Cache patterns, benchmarking
6. **Integrate REQ-6 Missions** - HashMap/HashSet compatibility
7. **[[mission-7|Mission 7]]** - Graph algorithms building on grids

---

*This mission provides foundational spatial data structures essential for competitive programming, game development, and real-world applications requiring 2D spatial reasoning.*

---

*Tags: #mission6 #2d-grids #navigation #spatial-algorithms #pathfinding #coordinates #flood-fill #v-cycle*

*Links: [[../zettel-index|Zettel Index]] | [[mission-5|Mission 5]] | [[mission-7|Mission 7]] | [[../flood-fill|Flood Fill]] | [[../daily-study/rust_learning_week4_notes/Day24|Day 24]] | [[../aoc2024-day4-mission6-example|AoC Day 4 Example]] | [[../Missions Overview|Missions Overview]]*