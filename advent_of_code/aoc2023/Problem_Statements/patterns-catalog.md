# AoC 2023 - Patterns Catalog

Reusable patterns extracted from daily solutions. Patterns are added when used in 3+ days.

---

## 📊 Pattern Usage Summary

| Pattern | Days Used | Location |
|---------|-----------|----------|
| Forward/Reverse Search | Day 1 | `day01.rs` |
| Delimiter-Based Parsing | Day 1, Day 2, Day 4, Day 5, Day 7 | `day01.rs`, `day02.rs`, `day04.rs`, `day05.rs`, `day07.rs` |
| Spatial Indexing | Day 3 | `day03.rs` |
| HashSet Membership | Day 4 | `day04.rs` |
| Forward-Propagation DP | Day 4 | `day04.rs` |
| Range Intersection/Splitting | Day 5 | `day05.rs` |
| Custom Ord for Sorting | Day 7 | `day07.rs` |
| Modular Arithmetic (Cyclic Wrapping) | Day 8 | `day08.rs` |
| HashMap for Graph Adjacency | Day 8 | `day08.rs` |
| Recursive Difference Computation | Day 9 | `day09.rs` |
| windows(2) for Pairwise Operations | Day 9 | `day09.rs` |
| BFS with HashMap Distance Tracking | Day 10 | `day10.rs` |
| Ray Casting State Machine | Day 10 | `day10.rs` |
| Direction Enum with Offsets | Day 10 | `day10.rs` |
| flat_map for Position Extraction | Day 11 | `day11.rs` |
| HashSet Complement for Empty Detection | Day 11 | `day11.rs` |
| Nested Loop for All Pairs | Day 11 | `day11.rs` |
| Recursive DP with Memoization | Day 12 | `day12.rs` |
| Three-Dimensional State Tuple | Day 12 | `day12.rs` |
| Branching Recursion with Constraints | Day 12 | `day12.rs` |

---

## 🔤 Parsing Patterns

### Pattern: Position-by-Position Scanning
**Used**: Day 1  
**When to use**: When overlapping patterns need to be detected (e.g., `eightwothree`)  
**Code**: `src/solver/day01.rs::digit_at_position()`

```rust
fn digit_at_position(s: &str, pos: usize) -> Option<u32> {
    let remainder = &s[pos..];
    // Check patterns at this position
    for (pattern, value) in PATTERNS {
        if remainder.starts_with(pattern) {
            return Some(value);
        }
    }
    None
}
```

**Note**: Will promote to `src/patterns/parsing.rs` if used 3+ times.

### Pattern: Delimiter-Based Hierarchical Parsing
**Used**: Day 1 (lines), Day 2 (structured data)  
**When to use**: Input has nested structure with consistent delimiters  
**Code**: `src/solver/day02.rs::parse_game()`

```rust
// Hierarchical split: lines → records → fields → values
let parts: Vec<&str> = line.split(':').collect();  // "Game 1: data"
let game_id = parts[0].strip_prefix("Game ")?.parse()?;

for reveal in parts[1].split(';') {  // Multiple reveals per game
    for cube in reveal.split(',') {   // Multiple cubes per reveal
        let parts: Vec<&str> = cube.trim().split_whitespace().collect();
        let count: u32 = parts[0].parse()?;
        let color = parts[1];
    }
}
```

**Pattern**: 
1. Split on primary delimiter (`:` for record structure)
2. Split on secondary delimiter (`;` for sequences)
3. Split on tertiary delimiter (`,` for elements)
4. Parse individual values

**Error handling**: Use `?` operator with `anyhow::Result` for clean propagation.

**Note**: Pattern used 3+ times, extracted to common pattern.

### Pattern: HashSet for O(1) Membership Testing
**Used**: Day 4  
**When to use**: Need to check if elements exist in a collection repeatedly  
**Code**: `src/solver/day04.rs::count_matches()`

```rust
// Build HashSet once from source data
let collection: HashSet<T> = source
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

// O(1) membership tests instead of O(n) linear search
let matches = items
    .iter()
    .filter(|item| collection.contains(item))
    .count();
```

**Complexity**: 
- Build HashSet: O(m) where m = collection size
- Each lookup: O(1) average case
- Total for n lookups: O(m + n)

**Alternatives comparison**:
```rust
// ❌ Nested loops: O(n × m)
items.iter().filter(|item| 
    collection.iter().any(|c| c == item)
).count()

// ⚠️ Sort + binary search: O(m log m + n log m)
let mut sorted = collection.to_vec();
sorted.sort();
items.iter().filter(|item|
    sorted.binary_search(item).is_ok()
).count()

// ✅ HashSet: O(m + n) - optimal!
let set: HashSet<_> = collection.collect();
items.iter().filter(|item| set.contains(item)).count()
```

**Mission Integration**: Mission 5 HashSet concepts

**Note**: Pattern used once, monitor for extraction if used 3+ times.

### Pattern: Forward/Reverse Search with Early Termination
**Used**: Day 1  
**When to use**: Finding first and last occurrence of something in a sequence  
**Code**: `src/solver/day01.rs`

```rust
// For simple char matching:
let first = line.chars().find_map(|c| c.to_digit(10))?;
let last = line.chars().rev().find_map(|c| c.to_digit(10))?;

// For position-based matching (when patterns look forward):
let first = (0..line.len()).find_map(|pos| check_at_position(line, pos))?;
let last = (0..line.len()).rev().find_map(|pos| check_at_position(line, pos))?;
```

**Key insight**: For Part 2, we iterate *positions* in reverse, not the string itself. The check function still looks forward from each position.

---

## 🗺️ Grid Processing Patterns

### Pattern: Spatial Index for Reverse Lookups
**Used**: Day 3  
**When to use**: Need to repeatedly find "what entity is at this coordinate"  
**Code**: `src/solver/day03.rs::solve_part2()`

```rust
// Build spatial index: HashMap<Coord, EntityId>
let mut coord_to_entity: HashMap<Coord, usize> = HashMap::new();
for (id, entity) in entities.iter().enumerate() {
    for coord in entity.occupied_coords() {
        coord_to_entity.insert(coord, id);
    }
}

// O(1) lookup instead of iterating all entities
for target_coord in points_of_interest {
    if let Some(&entity_id) = coord_to_entity.get(&target_coord) {
        // Process entity at this coordinate
    }
}
```

**Performance**: Converts O(grid_size × entity_count) → O(grid_size + entities × coords_per_entity)

**Key insight**: Reverse the search - instead of "which coordinates does this entity touch", ask "which entity is at this coordinate".

**Note**: Pattern used once, monitor for extraction if used 3+ times.

### Pattern: 8-Directional Grid Neighbor Iteration
**Used**: Day 3  
**When to use**: Need to check all adjacent cells including diagonals  
**Code**: Mission 6 `Coord::neighbors_8()`

```rust
// Mission 6 provides this iterator
for neighbor in coord.neighbors_8() {
    if grid.in_bounds(neighbor) {
        // Process adjacent cell
    }
}
```

**Mission Integration**: Use Mission 6's built-in iterator rather than manual offset calculation.

### Pattern: BFS Neighbor Exploration
**Used**: TBD  
**Code**: `src/patterns/grid_search.rs` (when created)

---

## 🧩 Dynamic Programming Patterns

### Pattern: Forward-Propagation State Accumulation
**Used**: Day 4  
**When to use**: Items affect future items, no backwards dependencies  
**Code**: `src/solver/day04.rs::solve_part2()`

```rust
// Initialize state vector (e.g., counts, values, etc.)
let mut state = vec![initial_value; n];

// Process each item in sequence
for (i, item) in items.iter().enumerate() {
    let current_value = state[i];
    let propagation_range = compute_effect(item);
    
    // Current state affects future positions
    for offset in 1..=propagation_range {
        if i + offset < n {
            state[i + offset] += current_value;
        }
    }
}

// Final answer: aggregate accumulated state
let result: T = state.iter().sum();
```

**Key characteristics**:
- **One-pass**: Process items left-to-right, no recursion needed
- **Forward-only**: Position i only affects positions > i
- **Accumulation**: Future values are sum/product of influences
- **No memoization**: Don't revisit positions

**Day 4 Example**: Scratchcard cascading
```rust
let mut card_counts = vec![1u32; num_cards];  // 1 original each

for (i, card) in cards.iter().enumerate() {
    let matches = count_matches(card);
    let copies = card_counts[i];  // How many of THIS card
    
    // Each copy wins more cards forward
    for j in 1..=matches {
        if i + j < num_cards {
            card_counts[i + j] += copies;  // Cascade!
        }
    }
}
```

**Complexity**: O(n × m) where n = items, m = avg propagation width

**When NOT to use**: 
- Need to look backwards (use full DP table)
- Recursive subproblems (use memoization)
- Need to track intermediate states (use different DP pattern)

**Note**: Pattern used once, monitor for extraction if used 3+ times.

### Pattern: Custom Ord for Multi-Criteria Sorting
**Used**: Day 7  
**When to use**: Need to sort complex types with multiple comparison levels  
**Code**: `src/solver/day07.rs::impl Ord for Hand`

```rust
// Define natural ordering for enums
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    // ... ordered from weakest to strongest
    FiveOfAKind = 7,
}

// Custom Ord for multi-level comparison
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                // Tiebreaker: compare element by element
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }
}
```

**Pattern**: 
1. Primary comparison (hand type)
2. If equal, secondary comparison (card-by-card)
3. Chain comparisons with early return
4. Use derived `PartialOrd` to get `partial_cmp` automatically

**Day 7 Application**: Poker hand ranking
- First compare hand type (five of a kind > four of a kind, etc.)
- If same type, compare cards left-to-right
- Enables simple `.sort()` call on `Vec<Hand>`

**Rust Benefits**:
- Automatic sorting with `.sort()`
- Type-safe comparison (can't compare incompatible types)
- No need for custom comparator functions
- Derived traits reduce boilerplate

**Note**: Pattern used once, monitor for extraction if used 3+ times.

---

## 🔢 Mathematical Patterns

### Pattern: Modular Arithmetic for Cyclic Wrapping
**Used**: Day 8  
**When to use**: Repeating sequences, circular buffers, cyclic iteration  
**Code**: `src/solver/day08.rs`

```rust
// Wrap index to stay within array bounds
let idx = (idx + 1) % array.len();

// Access repeating sequence
let instruction = instructions[step_count % instructions.len()];
```

**Day 8 Application**: 
- Instructions "LLR" repeat indefinitely
- After index 2, wrap back to 0
- `instruction_idx = (instruction_idx + 1) % instructions.len()`

**Alternatives**:
```rust
// ❌ Manual wrapping with conditional
idx += 1;
if idx >= array.len() {
    idx = 0;
}

// ✅ Modular arithmetic - cleaner and more efficient
idx = (idx + 1) % array.len();
```

**Zettelkasten**: [[number-theory-basics]]

### Pattern: GCD/LCM for Cycle Alignment
**Used**: Day 8  
**When to use**: Multiple cycles that need to synchronize  
**Code**: `src/solver/day08.rs::gcd()`, `src/solver/day08.rs::lcm()`

```rust
/// Euclidean algorithm for GCD
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// LCM using GCD
fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

// Multi-way LCM
let result = cycle_lengths.iter().fold(1, |acc, &x| lcm(acc, x));
```

**Day 8 Application**: Ghost synchronization
- Each ghost has different cycle length to reach goal
- Find when ALL cycles align: LCM of all cycle lengths
- Avoids simulating 8+ trillion individual steps

**Performance**: 
- Brute force: Intractable (trillions of steps)
- LCM: ~6.7ms (instant calculation)

**Zettelkasten**: [[number-theory-basics]]

---

## 🌐 Graph Patterns

### Pattern: HashMap for Graph Adjacency Lists
**Used**: Day 8  
**When to use**: Sparse graphs, string-based node IDs, O(1) neighbor lookup  
**Code**: `src/solver/day08.rs::Network`

```rust
struct Network {
    nodes: HashMap<String, (String, String)>,  // node -> (left, right)
}

impl Network {
    fn navigate(&self, start: &str, end: &str) -> Result<usize> {
        let mut current = start.to_string();
        let mut steps = 0;
        
        while current != end {
            let (left, right) = self.nodes
                .get(&current)
                .context(format!("Node {} not found", current))?;
            
            current = match instruction {
                'L' => left.clone(),
                'R' => right.clone(),
            };
            steps += 1;
        }
        Ok(steps)
    }
}
```

**Day 8 Application**: Directed graph with labeled edges
- Each node has exactly 2 outgoing edges (left/right)
- HashMap allows O(1) node lookup by string ID
- Better than Vec for sparse graphs or non-integer node IDs

**Alternatives**:
```rust
// ❌ Vec<Vec<usize>> - only works for dense graphs with integer IDs
let adjacency: Vec<Vec<usize>> = vec![vec![]; num_nodes];

// ❌ Vec of structs - O(n) search to find node
let nodes: Vec<Node> = vec![...];

// ✅ HashMap - O(1) lookup, flexible node IDs
let nodes: HashMap<String, (String, String)> = HashMap::new();
```

**Mission Integration**: Mission 5 HashMap concepts

**Zettelkasten**: [[graph-theory-fundamentals]]

---

## 🔁 Recursive Patterns

### Pattern: Recursive Difference Computation with Base Case
**Used**: Day 9  
**When to use**: Building difference pyramids, detecting polynomial patterns, recursive extrapolation  
**Code**: `src/solver/day09.rs`

```rust
/// Compute pairwise differences using windows(2)
fn compute_differences(sequence: &[i64]) -> Vec<i64> {
    sequence
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}

/// Recursive extrapolation with base case
fn extrapolate_next(sequence: &[i64]) -> i64 {
    // Base case: all zeros
    if sequence.iter().all(|&x| x == 0) {
        return 0;
    }
    
    // Recursive case: compute differences and recurse
    let differences = compute_differences(sequence);
    let diff_extrapolated = extrapolate_next(&differences);
    
    // Build up answer from recursion result
    sequence.last().unwrap() + diff_extrapolated
}
```

**Key Pattern Elements**:
1. **Base case**: Termination condition (all zeros)
2. **Recursive transformation**: Create smaller subproblem (differences)
3. **Recursive call**: Solve subproblem
4. **Build up**: Combine subproblem solution with current level

**Day 9 Application**: Polynomial extrapolation
- Build difference pyramid recursively
- Base case: All zeros → return 0
- Recursive case: Next value = last + extrapolated_difference
- Backward version: Previous value = first - extrapolated_difference

**Complexity**: O(n²) where n = sequence length (depth × width of pyramid)

**windows(2) Pattern**:
```rust
// Process consecutive pairs efficiently
let differences: Vec<_> = sequence
    .windows(2)
    .map(|pair| pair[1] - pair[0])  // next - current
    .collect();

// Alternative to manual iteration:
// ❌ Manual indexing
for i in 0..sequence.len()-1 {
    let diff = sequence[i+1] - sequence[i];
}

// ✅ windows(2) - cleaner and safer
for pair in sequence.windows(2) {
    let diff = pair[1] - pair[0];
}
```

**Rust Highlights**:
- `.windows(2)` for pairwise operations
- `.all()` for base case check  
- Recursion for pyramid structure
- Symmetry: forward/backward differ only in sign

**Mathematical Foundation**: Finite differences from numerical analysis
- Polynomial of degree n has constant nth differences
- Recursion automatically detects degree
- Works bidirectionally (forward/backward extrapolation)

**Note**: Pattern used once, monitor for extraction if used 3+ times.

---

## 📝 Notes

- Patterns are added when first used, promoted to reusable modules if used 3+ times
- Focus on Rust-idiomatic approaches (iterators, Option/Result, traits)
- Cross-reference to Mission implementations when applicable
- Link to zettelkasten for deep mathematical foundations


let node = nodes.iter().find(|n| n.id == "AAA")?;  // Slow!

// ✅ HashMap - O(1) lookup with any key type
let nodes: HashMap<String, Neighbors> = HashMap::new();
let neighbors = nodes.get("AAA")?;  // Fast!
```

**Mission Integration**: Mission 5 (HashMap), Mission 8 (Graph trait for algorithms)

**Zettelkasten**: [[graph-theory-fundamentals]]

### Pattern: BFS with HashMap Distance Tracking
**Used**: Day 10  
**When to use**: Need to track distances/costs from a starting point in a graph  
**Code**: `src/solver/day10.rs::find_loop_distances()`

```rust
fn find_distances(start: Node) -> HashMap<Node, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    
    distances.insert(start, 0);
    queue.push_back(start);
    
    while let Some(current) = queue.pop_front() {
        let current_dist = distances[&current];
        
        for neighbor in get_neighbors(current) {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, current_dist + 1);
                queue.push_back(neighbor);
            }
        }
    }
    
    distances
}
```

**Key Points**:
- HashMap serves dual purpose: visited set + distance storage
- `contains_key` prevents revisiting nodes
- VecDeque for FIFO queue (BFS guarantee)
- Returns all reachable nodes with their distances

**Complexity**: O(V + E) where V = vertices, E = edges

**Mission Integration**: Mission 8 (BFS pattern)

### Pattern: Ray Casting with State Machine (Day 10)
**Used**: Day 10 (point-in-polygon)  
**When to use**: Determining if points are inside/outside a boundary  
**Code**: `src/solver/day10.rs::solve_part2()`

```rust
// Scanline with state machine for boundary crossings
let mut inside = false;
let mut enter_corner: Option<char> = None;

for cell in row {
    if is_boundary(cell) {
        match cell {
            '|' => inside = !inside,  // Simple crossing
            'F' | 'L' => enter_corner = Some(cell),  // Enter corner
            '7' => {
                if enter_corner == Some('L') { inside = !inside; }
                enter_corner = None;
            }
            'J' => {
                if enter_corner == Some('F') { inside = !inside; }
                enter_corner = None;
            }
            '-' => {}  // Continuation - no state change
            _ => {}
        }
    } else if inside {
        count_inside += 1;
    }
}
```

**Corner Logic**:
- Vertical boundaries (`|`) always flip inside/outside
- Horizontal segments (`-`) continue the boundary
- Corners require pairing: `F-J` or `L-7` flip, `F-7` or `L-J` don't

**Zettelkasten**: [[computational-geometry-basics]], [[ray-casting-algorithm]]

### Pattern: Direction Enum with Offsets (Day 10)
**Used**: Day 10 (grid navigation)  
**When to use**: Grid-based problems needing directional movement  
**Code**: `src/solver/day10.rs::Dir`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North, South, East, West,
}

impl Dir {
    fn offset(&self) -> (i32, i32) {
        match self {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
        }
    }
    
    fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

// Usage
fn move_in_direction(coord: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
    let (dx, dy) = dir.offset();
    let new_x = coord.0 as i32 + dx;
    let new_y = coord.1 as i32 + dy;
    
    if new_x >= 0 && new_y >= 0 {
        Some((new_x as usize, new_y as usize))
    } else {
        None
    }
}
```

**Benefits**:
- Type-safe directions (can't mix up x/y)
- Easy to add diagonal directions
- Clean opposite() for connection checking
- Integrates with Mission 6 Grid `Coord` type

**Mission Integration**: Mission 6 (Grid, Direction enum)

### Pattern: Dijkstra with Priority Queue
**Used**: TBD  
**Code**: `src/patterns/pathfinding.rs` (when created)

---

## 🧠 Dynamic Programming Patterns

### Pattern: Recursive DP with Memoization
**Used**: Day 12  
**When to use**: Recursive problem with overlapping subproblems and multi-dimensional state  
**Code**: `src/solver/day12.rs::count_arrangements()`

```rust
use std::collections::HashMap;

type Memo = HashMap<(usize, usize, usize), usize>;

fn count_arrangements(
    input: &[u8],
    constraints: &[usize],
    pos: usize,
    state1: usize,
    state2: usize,
    memo: &mut Memo,
) -> usize {
    // 1. Base case: validate final state
    if pos == input.len() {
        return if all_constraints_satisfied(state1, state2, constraints) {
            1
        } else {
            0
        };
    }

    // 2. Check memoization cache
    let key = (pos, state1, state2);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    // 3. Recursive exploration with pruning
    let mut count = 0;
    for choice in possible_choices(input[pos]) {
        let (new_state1, new_state2) = apply_choice(choice, state1, state2);
        
        // Early pruning: skip invalid branches
        if is_valid_state(new_state1, new_state2, constraints) {
            count += count_arrangements(
                input,
                constraints,
                pos + 1,
                new_state1,
                new_state2,
                memo,
            );
        }
    }

    // 4. Cache and return
    memo.insert(key, count);
    count
}

// Wrapper function to initialize memo
fn solve(input: &[u8], constraints: &[usize]) -> usize {
    let mut memo = HashMap::new();
    count_arrangements(input, constraints, 0, 0, 0, &mut memo)
}
```

**Key Elements**:
1. **State tuple**: `(pos, state1, state2, ...)` captures all information needed for subproblems
2. **Base case**: When recursion bottoms out, validate if solution is complete
3. **Memoization**: `HashMap<State, Result>` caches computed subproblems
4. **Branching**: Try multiple choices at each step, sum valid paths
5. **Early pruning**: Skip invalid branches before recursing (constraint propagation)

**Day 12 Application**:
- **State**: `(position, group_index, current_run_length)`
- **Branches**: For `?` wildcard, try both `.` and `#`
- **Pruning**: Skip if `current_run > groups[group_index]` (run too long)
- **Result**: Sum of valid arrangements from all branches

**Benefits**:
- Naturally models recursive problem structure
- Automatic pruning via memoization (don't recompute same state)
- Reduces exponential to polynomial complexity
- Clean code compared to bottom-up DP table

**When NOT to use**:
- Bottom-up order is obvious and simple (use tabulation instead)
- State space is very sparse (memoization overhead > savings)
- Need to reconstruct solution path (tabulation easier)

**Zettelkasten**: [[memoization-comprehensive-guide]], [[tabulation-patterns]]

### Pattern: Three-Dimensional State Space
**Used**: Day 12  
**When to use**: Need to track multiple independent state variables in DP  
**Code**: `src/solver/day12.rs`

```rust
type Memo = HashMap<(usize, usize, usize), usize>;
//                   │      │      │
//                   │      │      └─ State dimension 3
//                   │      └─────── State dimension 2
//                   └───────────── State dimension 1

// Day 12 specific:
// (position, group_index, current_run_length)
let state = (pos, group_idx, current_run);
```

**When to add dimensions**:
- Need to track progress through input: Add `position` dimension
- Need to track which subproblem: Add `index` dimension (e.g., group_index, item_index)
- Need to track current state: Add `accumulator` dimension (e.g., current_run, current_weight)

**Examples**:
- **1D**: Fibonacci - `memo[n]` (only depends on n)
- **2D**: Coin Change - `memo[(amount, coin_index)]` (unbounded knapsack)
- **3D**: Day 12 - `memo[(pos, group_idx, current_run)]` (constraint satisfaction)
- **4D**: Complex knapsack with multiple constraints

**Trade-offs**:
- More dimensions = larger state space = more memory
- But: Prunes exponential search to polynomial
- Day 12: 3D state (500×30×20) = 300K states vs 2^500 brute force!

**Zettelkasten**: [[multidimensional-dp]]

### Pattern: Branching Recursion with Constraint Validation
**Used**: Day 12  
**When to use**: Multiple choices per step, need to validate constraints before recursing  
**Code**: `src/solver/day12.rs::count_arrangements()`

```rust
let mut count = 0;
let ch = input[pos];

// Branch 1: Try placing '.'
if ch == b'.' || ch == b'?' {
    if constraint_allows_dot(state1, state2) {
        count += recurse_with_dot(pos + 1, new_state1, new_state2, memo);
    }
}

// Branch 2: Try placing '#'
if ch == b'#' || ch == b'?' {
    if constraint_allows_hash(state1, state2) {
        count += recurse_with_hash(pos + 1, new_state1, new_state2, memo);
    }
}

count  // Sum of all valid branches
```

**Key Pattern**:
1. **Enumerate choices**: What are possible actions at this step?
2. **Validate constraints**: Is this choice legal given current state?
3. **Update state**: How does choice change state for next step?
4. **Recurse**: Explore subtree with updated state
5. **Aggregate**: Sum/max/combine results from all branches

**Day 12 Constraints**:
- Placing `.`: Can only end a group if `current_run == groups[group_idx]`
- Placing `#`: Can only extend if `current_run < groups[group_idx]`

**Benefits**:
- Clear separation of choice enumeration vs validation
- Easy to add more constraints (just add validation checks)
- Natural pruning (don't recurse on invalid branches)

**Zettelkasten**: [[backtracking-patterns]], [[constraint-satisfaction]]

---

## 📝 Pattern Extraction Criteria

A pattern is extracted to `src/patterns/` when:
1. Used in **3+ different days**
2. Has **consistent interface** across uses
3. Is **generic enough** to be reusable
4. Provides **significant code reduction**

---

## 🔗 Related Resources

- [Algorithms Reference](algorithms-reference.md) - Deep dives on complex algorithms
- [Performance Analysis](performance-analysis.md) - Optimization techniques
- [[aoc-parsing-patterns]] - Zettelkasten note on parsing
- [[iterator-patterns]] - Zettelkasten note on iterators
