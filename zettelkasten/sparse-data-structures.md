# Sparse Data Structures

**Tags:** #data-structures #optimization #memory-efficiency #sparse-representation #performance #spatial-data #aoc  
**Created:** 2025-12-09  
**Related:** [[computational-geometry]], [[aabb-sampling-optimization]], [[hashmap-fundamentals]], [[mission-5]]

---

## 🎯 Core Concept

**Sparse Data Structures** are optimized for datasets where most values are "empty" or default, storing **only the non-default values** rather than materializing the entire space.

**Key Principle**: When data occupies a tiny fraction of the total possible space, use a structure that stores only the **occupied positions** rather than allocating memory for every possible position.

## 🧠 Dense vs Sparse: The Trade-off

### **Dense Representation**

```rust
// ❌ Dense: Allocate entire 2D grid
let width = 100_000;
let height = 100_000;
let grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];
// Memory: 100,000 × 100,000 = 10 billion bytes = 10 GB

// Set one cell
grid[50_000][50_000] = '#';
// Used: 1 cell out of 10 billion = 0.00000001% occupancy
```

**Characteristics**:
- ✅ **O(1) access** by index
- ✅ **Simple indexing** - direct array access
- ✅ **Cache-friendly** for dense iteration
- ❌ **Memory explosion** for large, sparse spaces
- ❌ **Initialization cost** proportional to total size

### **Sparse Representation**

```rust
// ✅ Sparse: Store only occupied cells
let mut grid: HashSet<(i64, i64)> = HashSet::new();
// Memory: 0 bytes initially

// Set one cell
grid.insert((50_000, 50_000));
// Memory: ~24 bytes (one entry)
// Used: 100% of allocated memory
```

**Characteristics**:
- ✅ **Memory proportional to occupied cells**
- ✅ **No initialization cost**
- ✅ **Unbounded coordinates** (no predefined bounds)
- ❌ **O(1) average access** but with hash overhead
- ❌ **No cache locality** for iteration

## 📊 When to Use Sparse Structures

### **Occupancy Threshold Rule of Thumb**

```
Sparse if: occupied_cells / total_possible_cells < 0.01 (1%)

Dense if:  occupied_cells / total_possible_cells > 0.10 (10%)

Hybrid if: 0.01 < occupancy < 0.10
```

### **Real-World Examples**

| **Problem** | **Total Space** | **Occupied** | **Occupancy** | **Choice** |
|-------------|----------------|--------------|---------------|------------|
| Chess board | 8 × 8 = 64 | ~32 pieces | 50% | **Dense** |
| Small maze | 20 × 20 = 400 | ~300 walls | 75% | **Dense** |
| City map | 10K × 10K = 100M | ~5M buildings | 5% | **Hybrid** |
| AoC Day 9 boundary | 96K × 96K = 9.36B | 589K edges | 0.006% | **Sparse** |
| AoC Day 9 interior | 9.36B cells | 7.4B filled | 79% | **Dense** ❌ (but too big!) |

## 🛠️ Sparse Structure Options in Rust

### **1. HashSet for Presence/Absence**

```rust
use std::collections::HashSet;

// Store occupied coordinates
let mut occupied: HashSet<(i64, i64)> = HashSet::new();

// Add position
occupied.insert((100, 200));

// Check position
if occupied.contains(&(100, 200)) {
    println!("Occupied");
}

// Memory: ~24 bytes per entry (16 for tuple + 8 overhead)
```

**Best for**: Binary occupancy (cell exists or doesn't)

### **2. HashMap for Values**

```rust
use std::collections::HashMap;

// Store coordinates with associated values
let mut grid: HashMap<(i64, i64), char> = HashMap::new();

// Set value
grid.insert((100, 200), '#');

// Get value (with default)
let cell = grid.get(&(100, 200)).copied().unwrap_or('.');

// Memory: ~32 bytes per entry (16 for key + 1 for char + 15 overhead)
```

**Best for**: Cells with different values/properties

### **3. BTreeMap for Sorted Access**

```rust
use std::collections::BTreeMap;

// Store coordinates in sorted order
let mut grid: BTreeMap<(i64, i64), char> = BTreeMap::new();

// Iterate in coordinate order
for (&(x, y), &cell) in &grid {
    println!("({}, {}): {}", x, y, cell);
}

// Memory: ~48 bytes per entry (more overhead than HashMap)
```

**Best for**: Range queries, sorted iteration

### **4. Custom Sparse Matrix**

```rust
struct SparseMatrix<T> {
    rows: HashMap<i64, HashMap<i64, T>>,
}

impl<T> SparseMatrix<T> {
    fn get(&self, x: i64, y: i64) -> Option<&T> {
        self.rows.get(&y)?.get(&x)
    }
    
    fn set(&mut self, x: i64, y: i64, value: T) {
        self.rows.entry(y).or_insert_with(HashMap::new).insert(x, value);
    }
}
```

**Best for**: Row-major or column-major access patterns

## 🎓 AoC 2025 Day 9: Case Study

### **The Problem**

Find largest rectangle within a polygon formed by connecting 496 points.

**Input characteristics**:
- 496 red tile coordinates
- Coordinate range: X ∈ [1,712, 98,484], Y ∈ [1,528, 98,072]
- Bounding box: 96,772 × 96,544 = **9.36 billion cells**
- Polygon boundary: ~589K tiles
- Polygon interior: ~7.4 billion tiles

### **Failed Approach 1: Dense Grid**

```rust
// ❌ FAILED: Out of memory
let width = 98_484 - 1_712 + 1;   // 96,773
let height = 98_072 - 1_528 + 1;  // 96,545
let grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];
// Memory required: 96,773 × 96,545 × 1 byte = 9.36 GB

// Allocation fails with:
// thread 'main' panicked at 'allocation of 9345245985 bytes failed'
```

**Occupancy**: 589K / 9.36B = **0.006%** - screaming for sparse representation!

### **Failed Approach 2: Sparse Interior Fill**

```rust
// ❌ FAILED: Still too big
let mut grid: HashSet<(i64, i64)> = HashSet::new();

// Store boundary (works fine)
for point in boundary {
    grid.insert(point);  // 589K entries × 24 bytes = 14 MB ✅
}

// Flood fill interior (fails)
for point in interior {
    grid.insert(point);  // 7.4B entries × 24 bytes = 178 GB ❌
}

// Even sparse structure can't handle billions of entries!
```

**Lesson**: Sparse doesn't mean infinite - still bounded by available memory.

### **✅ Successful Approach: Boundary-Only + Ray Casting**

```rust
// ✅ SUCCESS: Store only boundary
let boundary: HashSet<(i64, i64)> = build_polygon_boundary(&points);
// Memory: 589K × 24 bytes = 14.1 MB ✅

// Don't materialize interior - compute membership on-demand
fn is_inside(x: i64, y: i64, boundary: &HashSet<(i64, i64)>, 
             polygon: &[(i64, i64)]) -> bool {
    boundary.contains(&(x, y))  // On boundary? ✅
    || point_in_polygon((x, y), polygon)  // In interior? (computed)
}

// Validate rectangles with AABB sampling
// Only check ~2,000 points per rectangle instead of billions
```

**Memory**: 14 MB (sparse boundary) vs 178 GB (sparse interior) vs 9.36 GB (dense grid)

**Key insight**: Sometimes you don't need to store data at all - **compute it on demand**!

## 🔑 Design Patterns

### **Pattern 1: Boundary-Only Storage**

**When to use**: Need to validate containment without storing entire interior

```rust
// Store perimeter, compute interior membership
let boundary = build_boundary();

fn contains(point, boundary, validator) -> bool {
    boundary.contains(point)      // On edge? Fast lookup
    || validator(point)            // Inside? Computed
}
```

**Examples**:
- Polygon containment (AoC Day 9)
- Circle collision (store circumference, compute interior)
- Maze walls (store obstacles, empty is implicit)

### **Pattern 2: Delta Storage**

**When to use**: Most cells match default, few differ

```rust
// Don't store default values
const DEFAULT: char = '.';
let mut non_default: HashMap<(i64, i64), char> = HashMap::new();

fn get(&self, x: i64, y: i64) -> char {
    self.non_default.get(&(x, y)).copied().unwrap_or(DEFAULT)
}
```

**Examples**:
- Cellular automata (active cells only)
- Diff algorithms (changed lines only)
- Compressed representations

### **Pattern 3: Hierarchical Sparse**

**When to use**: Clustering of occupied cells

```rust
// Divide space into chunks, store only non-empty chunks
struct ChunkedGrid<T> {
    chunks: HashMap<(i64, i64), HashMap<(i64, i64), T>>,
    chunk_size: i64,
}

impl<T> ChunkedGrid<T> {
    fn get(&self, x: i64, y: i64) -> Option<&T> {
        let chunk = (x / self.chunk_size, y / self.chunk_size);
        let local = (x % self.chunk_size, y % self.chunk_size);
        self.chunks.get(&chunk)?.get(&local)
    }
}
```

**Examples**:
- Minecraft world storage (chunk-based)
- Sparse matrix operations
- Spatial hashing for collision detection

## 📊 Performance Comparison

### **Memory Usage**

| **Structure** | **100 cells** | **10K cells** | **1M cells** | **1B cells** |
|---------------|---------------|---------------|--------------|--------------|
| Dense 1000×1000 | 1 MB | 1 MB | 1 MB | N/A (too big) |
| HashSet | 2.4 KB | 240 KB | 24 MB | 24 GB |
| HashMap<(i64,i64), char> | 3.2 KB | 320 KB | 32 MB | 32 GB |
| Custom sparse | Varies | Varies | Varies | Varies |

### **Access Time**

| **Operation** | **Dense** | **HashSet** | **HashMap** | **BTreeMap** |
|---------------|-----------|-------------|-------------|--------------|
| Get by coordinate | O(1) | O(1) avg | O(1) avg | O(log n) |
| Set by coordinate | O(1) | O(1) avg | O(1) avg | O(log n) |
| Iterate all cells | O(W×H) | O(n) | O(n) | O(n) |
| Range query | O(W×H) | O(n) | O(n) | O(log n + k) |

Where:
- W, H = grid dimensions
- n = number of occupied cells
- k = number of results in range

## 💡 Choosing the Right Structure

### **Decision Tree**

```
Is occupancy < 1%?
├─ No → Use dense structure (Vec<Vec<T>>)
└─ Yes → Is total space < 1 million cells?
    ├─ No → Use sparse structure
    └─ Yes → Is memory tight?
        ├─ No → Dense is fine (simple)
        └─ Yes → Use sparse
        
For sparse structures:
Do you need cell values?
├─ No → HashSet<(i64, i64)>
└─ Yes → Do you need sorted iteration?
    ├─ No → HashMap<(i64, i64), T>
    └─ Yes → BTreeMap<(i64, i64), T>
```

### **Anti-Patterns to Avoid**

❌ **Premature optimization**: Don't use sparse for 10×10 grid
❌ **Blind materialization**: Don't store computed/derivable values
❌ **Wrong data structure**: Don't use HashMap if occupancy > 50%
❌ **Memory assumptions**: Don't assume infinite memory for sparse structures

## 🏆 AoC Problem Recognition

### **Sparse Structure Red Flags**

🚩 **Check input scale first!**
- Coordinates span 10K+ range but example uses 10
- "Infinite grid" or "unbounded space"
- Number of active cells << total space
- Example has 8 points, real input has 500+

🚩 **Dense approach fails**:
- Out of memory errors
- Slow initialization
- Timeout on large inputs (but not examples)

### **When to Pivot from Dense to Sparse**

1. **Parse input first** - Check actual coordinate ranges
2. **Calculate occupancy** - occupied / total
3. **Estimate memory** - cells × bytes_per_cell
4. **If > 1 GB → use sparse** or reconsider approach

## 📚 Related Concepts

**Data Structures**:
- [[hashmap-fundamentals]] - HashMap implementation details
- [[mission-5]] - HashMap/HashSet V-Cycle implementation
- [[btreemap-ordered-maps]] - Sorted sparse structures

**Optimization Techniques**:
- [[aabb-sampling-optimization]] - Strategic querying for sparse spaces
- [[ray-casting-algorithm]] - Computing membership without storage
- [[computational-geometry]] - Geometric algorithms for spatial queries

**AoC Applications**:
- [[aoc-2025-day09]] - Boundary-only sparse representation
- [[aoc-2024-day08]] - Grid-based problems with Mission 6
- [[infinite-grid-problems]] - Unbounded coordinate spaces

---

## 🎯 Implementation Checklist

When considering sparse structures:

- [ ] Calculate occupancy percentage
- [ ] Estimate memory requirements for dense approach
- [ ] Check if data is computable instead of storable
- [ ] Consider boundary-only representation
- [ ] Profile actual memory usage
- [ ] Test with real input scale (not just examples)
- [ ] Consider hybrid dense/sparse for moderate occupancy
- [ ] Document why sparse was chosen (for future maintainers)

---

*Sparse data structures embody a fundamental trade-off in computer science: **space for time**. By storing only what's necessary and computing the rest on-demand, we can tackle problems with astronomical state spaces using modest memory. The key is recognizing when materialization is wasteful and pivoting to sparse representation before wasting hours implementing a dense solution that will never fit in memory.*

**Links:** [[computational-geometry]] | [[hashmap-fundamentals]] | [[aabb-sampling-optimization]] | [[mission-5]] | [[aoc-2025-day09]] | [[memory-optimization]]
