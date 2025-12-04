# Mission Composition Patterns

**Leveraging validated mission components to accelerate problem solving**

> **Key Insight**: The integrator approach - compose from proven, tested components rather than reimplementing from scratch. This reduces code, eliminates bugs, and lets you focus on problem logic.

---

## 🎯 **Core Philosophy**

### **The Integrator Mindset**

Like an integration engineer who takes validated components from experts and assembles them into final products, Rust learning follows the same pattern:

| **Traditional Approach** | **Integrator Approach** |
|--------------------------|-------------------------|
| Implement grid from scratch | Use Mission 6 `Grid<T>` |
| Write BFS/DFS manually | Use Mission 8 graph algorithms |
| Build custom hash tables | Use Mission 5 `HashMap` patterns |
| Handle bounds checking everywhere | Let mission abstractions handle it |

### **Benefits**

- ✅ **Proven correctness** - Mission implementations are V-Cycle validated with comprehensive tests
- ✅ **Performance optimized** - Missions meet Big-O requirements and are benchmarked
- ✅ **Time efficiency** - Focus on problem-solving logic, not infrastructure
- ✅ **Learning reinforcement** - Practical application of mission concepts
- ✅ **Code clarity** - Domain logic visible, plumbing abstracted away

---

## 🗺️ **Mission-to-Problem Mapping**

### **Quick Reference**

| **Problem Pattern** | **Mission** | **Key Components** |
|---------------------|-------------|-------------------|
| Grid problems (pathfinding, regions) | [[mission-6]] | `Grid<T>`, `Coord`, `neighbors_*` |
| Graph traversal (BFS/DFS) | [[mission-8]] | `Graph` trait, `bfs()`, `dfs()` |
| Connected components, grouping | [[mission-10]] | `UnionFind` |
| Key-value storage, caching | [[mission-5]] | `HashMap`, `HashSet` patterns |
| Stack-based parsing | [[mission-1]] | `Stack<T>` |
| Queue-based processing | [[mission-2]] | `Queue<T>`, `RingBuffer` |
| Search algorithms | [[mission-3]] | Binary search variants |
| Linked structures | [[mission-4]] | Smart pointer patterns |
| Pathfinding (A*, Dijkstra) | [[mission-9]] | Advanced pathfinding algorithms |

---

## 📦 **Case Study: AoC 2025 Day 4**

### **The Problem**
Count paper rolls (`@`) accessible by forklifts (< 4 adjacent rolls in 8-connected neighborhood), then iteratively remove accessible rolls.

### **Without Mission Composition**

```rust
// Manual implementation requires:
// - Grid parsing with line splitting
// - 8-direction offset array: [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]
// - Bounds checking for every neighbor access
// - Edge/corner special cases
// - ~50+ lines of infrastructure code
```

### **With Mission 6 Composition**

```rust
use mission6::{AocGridParser, Coord, Grid};

fn count_adjacent_rolls(grid: &Grid<char>, coord: Coord) -> usize {
    coord
        .neighbors_8_bounded(grid.width(), grid.height())
        .filter(|&neighbor| grid[neighbor] == '@')
        .count()
}

fn is_accessible(grid: &Grid<char>, coord: Coord) -> bool {
    grid[coord] == '@' && count_adjacent_rolls(grid, coord) < 4
}
```

### **Results**
- **Lines of code**: ~30 vs ~80+ (manual)
- **Bugs**: Zero (mission handles bounds)
- **Focus**: Pure problem logic (accessibility rule, erosion loop)
- **Time to solution**: Minutes, not hours

---

## 🔧 **Common Composition Patterns**

### **Pattern 1: Grid + Graph**

Combine Mission 6 Grid storage with Mission 8 Graph algorithms:

```rust
use mission6::{Grid, Coord};
use mission8::{Graph, bfs};

// Grid provides storage, Graph trait enables algorithms
impl Graph for MyMap {
    type Node = Coord;
    
    fn neighbors(&self, node: &Coord) -> Vec<Coord> {
        node.neighbors_4_bounded(self.grid.width(), self.grid.height())
            .filter(|&c| self.is_passable(c))
            .collect()
    }
}

// Now use Mission 8 BFS on your grid!
let path = bfs(&my_map, start, end);
```

### **Pattern 2: Union-Find + Grid**

Connected component analysis on grids:

```rust
use mission6::{Grid, Coord};
use mission10::UnionFind;

fn count_regions(grid: &Grid<char>) -> usize {
    let mut uf = UnionFind::new(grid.width() * grid.height());
    
    for coord in grid.coordinates() {
        if grid[coord] == '#' {
            for neighbor in coord.neighbors_4_bounded(grid.width(), grid.height()) {
                if grid[neighbor] == '#' {
                    let idx1 = coord.y * grid.width() + coord.x;
                    let idx2 = neighbor.y * grid.width() + neighbor.x;
                    uf.union(idx1, idx2);
                }
            }
        }
    }
    
    uf.count_sets()
}
```

### **Pattern 3: HashMap + Algorithms**

Memoization using Mission 5 patterns:

```rust
use std::collections::HashMap;

fn solve_with_memo(input: &str) -> usize {
    let mut memo: HashMap<State, usize> = HashMap::new();
    
    fn recursive(state: State, memo: &mut HashMap<State, usize>) -> usize {
        if let Some(&cached) = memo.get(&state) {
            return cached;
        }
        
        let result = /* expensive computation */;
        memo.insert(state, result);
        result
    }
    
    recursive(initial_state, &mut memo)
}
```

---

## 📊 **AoC Mission Integration Examples**

| **AoC Problem** | **Missions Used** | **Code Reduction** |
|-----------------|-------------------|-------------------|
| [[../../advent_of_code/aoc2025/Problem_Statements/day04\|Day 4 2025]] | Mission 6 Grid | ~40% |
| [[aoc2024-day5-mission-integration\|Day 5 2024]] | Mission 7+8 Graph | ~40% |
| [[aoc2024-day4-mission6-example\|Day 4 2024]] | Mission 6 Grid | ~43% |
| AoC 2015 Day 18 | Mission 6 Grid | ~35% |

---

## 🎯 **When to Compose vs Implement**

### **Use Mission Components When:**

- Problem maps to existing mission functionality
- Infrastructure (grids, graphs, collections) is standard
- Time/correctness matters more than learning internals
- Solving AoC or competitive programming

### **Implement From Scratch When:**

- Learning the data structure itself is the goal
- Problem requires truly novel structure not covered
- Need specialized optimizations missions don't provide
- Building the mission implementation itself!

---

## 🔗 **Integration Checklist**

Before implementing any AoC or practice problem:

- [ ] **Scan missions/** for applicable data structures
- [ ] **Check Mission READMEs** for feature compatibility
- [ ] **Review mission tests** for performance characteristics
- [ ] **Prefer composition** over custom implementation
- [ ] **Document integration** in solution comments

---

## 🔗 **Related Concepts**

### **Missions**
- [[mission-6]] - Grid infrastructure (most commonly composed)
- [[mission-8]] - Graph algorithms (BFS/DFS)
- [[mission-10]] - Union-Find for connectivity
- [[Missions Overview]] - Complete mission catalog

### **AoC Integration**
- [[AoC Integration]] - How missions accelerate AoC solving
- [[AoC Patterns MOC]] - Problem patterns and solutions
- [[aoc2024-day5-mission-integration]] - Detailed integration case study

### **Learning Philosophy**
- [[V-Cycle Integration]] - Engineering methodology
- [[../\.github/copilot-instructions]] - Integrator approach documentation

---

*Tags: #composition #missions #integration #aoc #patterns #reuse #integrator*

*Links: [[Missions Overview]] | [[AoC Patterns MOC]] | [[AoC Integration]] | [[mission-6]] | [[mission-8]] | [[mission-10]] | [[V-Cycle Integration]]*
