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
| Iterator zip() for Element-wise Comparison | Day 13 | `day13.rs` |
| Hamming Distance for Pattern Matching | Day 13 | `day13.rs` |
| Target-Value Generalization | Day 13 | `day13.rs` |
| HashMap State Hashing for Cycle Detection | Day 14 | `day14.rs` |
| Modulo Fast-Forward Optimization | Day 14 | `day14.rs` |
| Grid State Serialization | Day 14 | `day14.rs` |
| String Slicing for Label Extraction | Day 15 | `day15.rs` |
| Vec of Vecs for Fixed Buckets | Day 15 | `day15.rs` |
| .retain() for In-Place Filtering | Day 15 | `day15.rs` |
| State Tuple (Position, Direction) | Day 16 | `day16.rs` |
| Direction Enum with offset() Method | Day 16 | `day16.rs` |
| Dual HashSets (State vs Result) | Day 16 | `day16.rs` |
| Match Exhaustiveness for Tile Logic | Day 16 | `day16.rs` |
| State Tuple (Pos + Direction + Counter) | Day 17 | `day17.rs` |
| BinaryHeap Priority Queue (Min-Heap) | Day 17 | `day17.rs` |
| HashMap for Visited State Tracking | Day 17 | `day17.rs` |
| Constraint-Based Move Generation | Day 17 | `day17.rs` |
| Vertex-Only Polygon Tracing | Day 18 | `day18.rs` |
| Cross-Product Accumulation (Shoelace) | Day 18 | `day18.rs` |
| Mathematical Formula Composition | Day 18 | `day18.rs` |
| Hex String Parsing with Radix | Day 18 | `day18.rs` |
| Enum-Based State Machine Destinations | Day 19 | `day19.rs` |
| HashMap Workflow Lookup | Day 19 | `day19.rs` |
| Conditional Rule Evaluation (First Match Wins) | Day 19 | `day19.rs` |
| Range Splitting by Operator | Day 19 | `day19.rs` |
| DFS with State-Space Range Propagation | Day 19 | `day19.rs` |
| Mathematical Combination Counting | Day 19 | `day19.rs` |

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

### Pattern: Cycle Detection via State Hashing
**Used**: Day 14  
**When to use**: Finite state space, deterministic transitions, large iteration count, need to detect repeating patterns  
**Code**: `src/solver/day14.rs::solve_part2()`

```rust
/// Detect cycles in deterministic state space using HashMap
fn detect_cycle_and_fast_forward<State: Hash + Eq + Clone>(
    initial_state: State,
    next_state: impl Fn(&State) -> State,
    target_iterations: usize,
) -> State {
    let mut seen: HashMap<State, usize> = HashMap::new();
    let mut state = initial_state;
    let mut iteration = 0;
    
    while iteration < target_iterations {
        // Check if we've seen this state before
        if let Some(&first_occurrence) = seen.get(&state) {
            // Cycle detected!
            let cycle_length = iteration - first_occurrence;
            
            // Fast-forward using modulo arithmetic
            let remaining = target_iterations - iteration;
            let offset_in_cycle = remaining % cycle_length;
            
            // Simulate only the offset instead of all remaining iterations
            for _ in 0..offset_in_cycle {
                state = next_state(&state);
            }
            
            return state;
        }
        
        // Track state's first occurrence
        seen.insert(state.clone(), iteration);
        
        // Advance to next state
        state = next_state(&state);
        iteration += 1;
    }
    
    state
}
```

**Day 14 Application**: 1 billion spin cycles on rock platform
- **State**: Grid<char> serialized to String for HashMap key
- **Transition**: spin_cycle (deterministic N→W→S→E tilts)
- **Cycle detection**: At iteration ~100-200, grid state repeats
- **Fast-forward**: `(1B - 100) % cycle_length` gives final state position
- **Performance**: 12.7ms instead of impossible brute force

**Pattern Breakdown**:

1. **State Tracking**: HashMap<State, usize> maps state → first occurrence index
2. **Cycle Detection**: When `seen.contains_key(current_state)`, cycle found
3. **Cycle Parameters**:
   - `cycle_start = seen[state]` - where cycle begins
   - `cycle_length = current_iteration - cycle_start`
4. **Modulo Fast-Forward**:
   ```rust
   let final_position = cycle_start + ((target - cycle_start) % cycle_length);
   ```
5. **Simulate Offset**: Run only `offset_in_cycle` more iterations

**Mathematical Foundation** (Pigeonhole Principle):
- Finite state space (m possible states)
- Deterministic process (same state → same next state)
- After m+1 iterations, **must** repeat (pigeonhole principle)
- Expected cycle length: ~√m (birthday paradox)

**State Serialization Pattern**:
```rust
// Convert complex state to hashable representation
fn grid_to_string(grid: &Grid<char>) -> String {
    (0..grid.height())
        .map(|y| {
            (0..grid.width())
                .map(|x| grid.get(x, y).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
```

**Complexity**:
- **Space**: O(unique_states) for HashMap
- **Time**: O(μ + λ) where μ = cycle start, λ = cycle length
- **Expected**: μ + λ ≈ √(state_space) << target_iterations

**Alternatives**:
```rust
// ❌ Brute force - intractable for large iterations
for _ in 0..1_000_000_000 {
    state = next_state(state);
}

// ❌ Floyd's Tortoise-Hare - O(1) space but more complex
let (mu, lambda) = floyd_detect_cycle(initial, next_state);

// ✅ HashMap state tracking - simple, single-pass, exact parameters
let final = detect_cycle_and_fast_forward(initial, next_state, 1_000_000_000);
```

**Usage Summary**:
- ✅ Grid simulations with large iteration counts
- ✅ Graph cycles (Day 8 ghost paths)
- ✅ State machines with finite configurations
- ✅ Any deterministic process on finite state space

**Mission Integration**: Mission 6 (Grid<T> for state storage)

**Zettelkasten**: [[pigeonhole-principle-cycle-detection]], [[modular-arithmetic]]

### Pattern: Modulo Fast-Forward After Cycle Detection
**Used**: Day 14  
**When to use**: After detecting cycle, need to find state at very large iteration number  
**Code**: `src/solver/day14.rs::solve_part2()`

```rust
// After detecting cycle at iteration `cycle_start` with length `cycle_length`:
let remaining = target_iterations - current_iteration;
let offset_in_cycle = remaining % cycle_length;

// Simulate only `offset` more iterations instead of `remaining`
for _ in 0..offset_in_cycle {
    state = next_state(state);
}
```

**Mathematical Justification**:
If state repeats every `cycle_length` iterations starting from `cycle_start`:
- `state[cycle_start]` = `state[cycle_start + cycle_length]` = `state[cycle_start + 2×cycle_length]` = ...
- `state[n]` = `state[cycle_start + ((n - cycle_start) % cycle_length)]` for all n ≥ cycle_start

**Example**:
```rust
// Cycle detected: state[100] == state[107] (cycle_length = 7)
// Want: state[1,000,000,000]
// Equivalent: state[100 + ((1B - 100) % 7)]
//           = state[100 + (999,999,900 % 7)]
//           = state[100 + 5]
//           = state[105]
// 
// Instead of 999,999,900 iterations, run only 5!
```

**Day 14 Numbers**:
- Cycle detected at iteration ~100-200 (varies by input)
- Cycle length ~7-20 (varies by input)
- Target: 1,000,000,000 iterations
- **Reduction**: ~99.99999% fewer iterations needed

**Zettelkasten**: [[modular-arithmetic]]

### Pattern: State-Space Extension for Constrained Pathfinding (Day 17)
**Used**: Day 17  
**When to use**: Dijkstra/BFS with movement constraints beyond position (direction, momentum, resources, keys)  
**Code**: `src/solver/day17.rs::find_min_heat_loss()`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Coord,           // Position in grid
    dir: Direction,       // Current direction of movement
    consecutive: u8,      // Constraint tracker (e.g., consecutive moves)
}

/// Extended state Dijkstra pattern
fn find_shortest_path(grid: &Grid<u8>, constraints: Constraints) -> usize {
    let mut heap = BinaryHeap::new();  // Priority queue
    let mut visited: HashMap<State, usize> = HashMap::new();  // Best cost per STATE
    
    // Start with initial states
    heap.push(Node {
        cost: 0,
        state: State { pos: start, dir: initial_dir, consecutive: 0 },
    });
    
    while let Some(Node { cost, state }) = heap.pop() {
        // Goal check with constraint validation
        if state.pos == goal && meets_constraints(&state) {
            return cost;
        }
        
        // Skip if we've seen this STATE with lower/equal cost
        if let Some(&prev_cost) = visited.get(&state) {
            if prev_cost <= cost {
                continue;
            }
        }
        visited.insert(state, cost);
        
        // Generate next moves based on constraints
        for next_state in generate_valid_moves(&state, &constraints) {
            if is_valid_position(next_state.pos, grid) {
                let edge_cost = get_edge_cost(state.pos, next_state.pos, grid);
                heap.push(Node {
                    cost: cost + edge_cost,
                    state: next_state,
                });
            }
        }
    }
    
    unreachable!("No path found")
}
```

**Day 17 Application**: Crucible pathfinding with straight-line limits
- **State**: `(position, direction, consecutive_blocks)`
- **Constraints**:
  - Part 1: Max 3 consecutive blocks same direction
  - Part 2: Min 4, max 10 consecutive blocks
  - Cannot reverse (180° turns forbidden)
- **State space**: cells × 4 directions × max_consecutive
- **Performance**: Part 1 ~239k states (64ms), Part 2 ~795k states (182ms)

**Key Insight - Why Position-Only Fails**:
```rust
// ❌ WRONG - Only tracks position
visited: HashSet<Coord>

// Problem: Reaching (5,5) moving Right with 3 consecutive is DIFFERENT 
// from (5,5) moving Down with 1 consecutive!
// First state can't go straight (constraint), second can.

// ✅ CORRECT - Tracks full state
visited: HashMap<State, usize>  // State = (pos, dir, consecutive)
```

**Constraint-Based Move Generation Pattern**:
```rust
fn generate_valid_moves(state: &State, constraints: &Constraints) -> Vec<State> {
    if state.consecutive < constraints.min_straight {
        // MUST continue straight (haven't met minimum)
        vec![State {
            pos: state.pos.move_dir(state.dir),
            dir: state.dir,
            consecutive: state.consecutive + 1,
        }]
    } else if state.consecutive >= constraints.max_straight {
        // MUST turn (hit maximum)
        vec![
            State { pos: turn_left(...), dir: state.dir.turn_left(), consecutive: 1 },
            State { pos: turn_right(...), dir: state.dir.turn_right(), consecutive: 1 },
        ]
    } else {
        // Flexible - can continue or turn
        vec![
            continue_straight(...),
            turn_left(...),
            turn_right(...),
        ]
    }
}
```

**State Space Complexity**:
- **Standard Dijkstra**: O(V) states where V = positions
- **Extended Dijkstra**: O(V × D × C) states where:
  - V = positions (grid cells)
  - D = directions (typically 4)
  - C = constraint values (consecutive count, keys collected, etc.)
- **Example**: 141×141 grid × 4 dir × 10 max_consecutive = ~795,000 states

**Applications**:
- Movement constraints (max turns, straight limits, momentum)
- Resource tracking (fuel, inventory, keys)
- Direction-dependent costs (wind, slopes)
- Multi-agent coordination (position + which agent)

**Alternatives**:
```rust
// ❌ Position-only Dijkstra - breaks with constraints
let visited: HashSet<Coord> = HashSet::new();

// ✅ State-space Dijkstra - handles constraints exactly
let visited: HashMap<State, usize> = HashMap::new();

// ⚠️ A* with heuristic - faster but needs admissible heuristic
let heuristic = manhattan_distance(pos, goal);  // Must not overestimate!
```

**Mission Integration**: Mission 6 (Grid<T> for maps)

**Zettelkasten**: [[graph-theory-fundamentals]], [[dijkstra-algorithm]], [[state-space-search]]

### Pattern: BinaryHeap Priority Queue for Dijkstra (Day 17)
**Used**: Day 17  
**When to use**: Need min-heap for greedy algorithms (Dijkstra, A*, Prim's MST)  
**Code**: `src/solver/day17.rs`

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Node {
    cost: usize,   // Priority field
    state: State,  // Payload
}

// Manual Ord implementation for min-heap behavior
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparison for min-heap (BinaryHeap is max-heap by default)
        other.cost.cmp(&self.cost)
            .then_with(|| self.state.cmp(&other.state))  // Tiebreaker
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra() {
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    
    heap.push(Node { cost: 0, state: initial_state });
    
    while let Some(Node { cost, state }) = heap.pop() {
        // Always gets MINIMUM cost node (due to reversed Ord)
        // ...
    }
}
```

**Alternative - Using Reverse wrapper**:
```rust
use std::cmp::Reverse;

// Simpler but requires wrapping/unwrapping
let mut heap: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

heap.push(Reverse((0, initial_state)));

while let Some(Reverse((cost, state))) = heap.pop() {
    // Unwrap Reverse to get values
}
```

**Day 17 Application**:
- Priority queue for Dijkstra's algorithm
- Always process lowest-cost state next (greedy choice)
- O(log n) push/pop operations
- Crucial for optimal pathfinding

**Pattern Comparison**:

| Approach | Pros | Cons |
|----------|------|------|
| Custom Ord | Clean pop(), clear semantics | More boilerplate |
| Reverse wrapper | Less code | Constant wrapping/unwrapping |
| External library (priority-queue) | Feature-rich | External dependency |

**Complexity**:
- **push**: O(log n)
- **pop**: O(log n)
- **peek**: O(1)

**Zettelkasten**: [[binary-heap-patterns]]

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

## 🎯 Pattern Matching & Comparison

### Pattern: Target-Value Generalization
**Used**: Day 13  
**When to use**: Multiple problem parts differ only in a target value/threshold  
**Code**: `src/solver/day13.rs::find_reflection()`

```rust
// Generalized function with target parameter
fn find_reflection(pattern: &[Vec<char>], target_mismatches: usize) -> Option<usize> {
    for candidate_line in reflection_lines {
        if count_mismatches(pattern, candidate_line) == target_mismatches {
            return Some(candidate_line);
        }
    }
    None
}

// Part 1: Perfect match (0 mismatches)
find_reflection(pattern, 0)

// Part 2: Exactly 1 mismatch
find_reflection(pattern, 1)
```

**Key Pattern**:
1. **Identify varying parameter**: What changes between parts? (threshold, target, mode)
2. **Generalize function signature**: Add parameter for varying value
3. **Single implementation**: Same logic handles all cases
4. **Caller specialization**: Pass specific value for each part

**Benefits**:
- Zero code duplication
- Easy to extend (Part 3 with target=2 would be trivial)
- Clear separation: Algorithm vs configuration

**Day 13 Application**: Boolean "exact match" → Integer "mismatch count" generalization

**Zettelkasten**: [[parametric-polymorphism]], [[code-reuse-patterns]]

### Pattern: Iterator zip() for Element-wise Comparison
**Used**: Day 13  
**When to use**: Need to compare two sequences element-by-element  
**Code**: `src/solver/day13.rs::count_horizontal_mismatches()`

```rust
// Count mismatches using zip + filter + count
fn hamming_distance(a: &[char], b: &[char]) -> usize {
    a.iter()
        .zip(b.iter())
        .filter(|(x, y)| x != y)
        .count()
}

// More explicit version with for loop
fn hamming_distance_explicit(a: &[char], b: &[char]) -> usize {
    let mut count = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        if x != y {
            count += 1;
        }
    }
    count
}
```

**Key Pattern**:
1. **Pair elements**: `.zip()` creates iterator of tuples
2. **Compare elements**: Filter or count based on predicate
3. **Aggregate result**: `.count()`, `.sum()`, `.all()`, etc.

**Benefits**:
- Concise and idiomatic Rust
- Automatically handles length mismatches (stops at shorter sequence)
- Composable with other iterator adapters

**Common Variations**:
```rust
// Check if all pairs equal
a.iter().zip(b.iter()).all(|(x, y)| x == y)

// Check if any pair equal
a.iter().zip(b.iter()).any(|(x, y)| x == y)

// Find first mismatch position
a.iter().zip(b.iter()).position(|(x, y)| x != y)

// Collect differences
a.iter().zip(b.iter())
    .filter_map(|(x, y)| if x != y { Some((x, y)) } else { None })
    .collect()
```

**Zettelkasten**: [[iterator-patterns]], [[zip-iterator]]

---

## � Data Structure Patterns

### Pattern: String Slicing for Label Extraction
**Used**: Day 15  
**When to use**: Parsing strings with known delimiters to extract substrings  
**Code**: `src/solver/day15.rs`

```rust
// Extract label from "label=value" or "label-"
if let Some(pos) = step.find('=') {
    let label = &step[..pos];        // Substring before '='
    let value = &step[pos+1..];      // Substring after '='
    let focal_length: u32 = value.parse()?;
}

if let Some(pos) = step.find('-') {
    let label = &step[..pos];        // Substring before '-'
    // No value part for removal operation
}
```

**Key Concepts**:
- **String slicing**: `&str[start..end]` creates view without allocation
- **Exclusive end**: `[..pos]` excludes character at `pos`
- **Inclusive start**: `[pos+1..]` starts after delimiter
- **Zero-copy**: No String allocation, just borrows from original

**Pattern Structure**:
1. Find delimiter position with `.find()`
2. Slice before delimiter: `&str[..pos]`
3. Slice after delimiter: `&str[pos+1..]`
4. Parse as needed

**Alternatives**:
```rust
// Less efficient: split creates iterator and allocates
let parts: Vec<&str> = step.split('=').collect();
let label = parts[0];
let value = parts.get(1).unwrap_or("");

// More complex: split_once returns Option<(&str, &str)>
if let Some((label, value)) = step.split_once('=') {
    // Use label and value
}
```

**Benefits of slicing**:
- No allocations (just borrows)
- O(1) operation (just pointer arithmetic)
- Direct access without intermediate collections
- Clear intent when position is already known

### Pattern: Vec of Vecs for Fixed Buckets
**Used**: Day 15  
**When to use**: HashMap-like structure with known bucket count and ordering requirements  
**Code**: `src/solver/day15.rs::solve_part2()`

```rust
// Create 256 empty buckets
let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];

// Hash determines bucket
let box_num = hash(label) as usize;

// Operations on bucket
if let Some(idx) = boxes[box_num].iter().position(|(l, _)| l == label) {
    boxes[box_num][idx].1 = new_value;     // Replace
} else {
    boxes[box_num].push((label, value));   // Add to end
}

boxes[box_num].retain(|(l, _)| l != label);  // Remove
```

**Structure**:
```
Vec<Vec<T>>:
  Index 0: [item1, item2, item3]     ← Box 0
  Index 1: []                          ← Box 1 (empty)
  Index 2: [item4]                     ← Box 2
  ...
  Index 255: [item5, item6]            ← Box 255
```

**When to use**:
- **Fixed bucket count**: Outer Vec size known upfront (e.g., 256 boxes)
- **Variable items per bucket**: Inner Vecs can grow/shrink independently
- **Ordered within bucket**: Vec maintains insertion order (unlike HashMap)
- **Direct indexing**: Bucket number is integer (not arbitrary hash)

**Trade-offs**:
```
Vec<Vec<T>> vs HashMap<usize, Vec<T>>:

Vec<Vec<T>>:
✅ Faster indexing: O(1) array access vs O(1) hash lookup
✅ Simpler: No hash function overhead
✅ Predictable memory: Contiguous outer vec
✅ Works when bucket count is fixed and small
❌ Wastes space if many buckets empty
❌ Not suitable if bucket indices are sparse (e.g., 0, 1000000, 2000000)

HashMap<usize, Vec<T>>:
✅ Sparse-friendly: Only allocates used buckets
✅ Dynamic keys: Can use any hashable type
❌ Slower: Hash computation + potential collision resolution
❌ More complex: Requires hasher, load factor, resizing
```

**Day 15 Rationale**:
- 256 buckets = small enough to allocate all upfront
- Dense keys: All buckets 0-255 potentially used
- Fast access: Direct array indexing critical for Part 2 performance
- Ordering matters: Lens positions within box determine focusing power

### Pattern: .retain() for In-Place Filtering
**Used**: Day 15  
**When to use**: Removing elements from Vec matching a predicate without allocation  
**Code**: `src/solver/day15.rs::solve_part2()`

```rust
// Remove all lenses with specific label
boxes[box_num].retain(|(l, _)| l != label);
```

**Key Concepts**:
- **In-place modification**: Mutates Vec without creating new one
- **Predicate**: Keep elements where predicate returns true
- **Efficient**: O(n) single pass, no allocations
- **Preserves order**: Remaining elements keep relative positions

**Pattern Structure**:
```rust
vec.retain(|item| predicate(item));  // Keep if predicate true

// Equivalent manual loop (more verbose):
let mut i = 0;
while i < vec.len() {
    if !predicate(&vec[i]) {
        vec.remove(i);  // Shift all following elements
    } else {
        i += 1;
    }
}
```

**Common Use Cases**:
```rust
// Remove all zeros
numbers.retain(|&x| x != 0);

// Keep only even numbers
numbers.retain(|&x| x % 2 == 0);

// Remove items matching complex criteria
items.retain(|item| item.is_valid() && item.score > threshold);

// Day 15: Remove lens by label
lenses.retain(|(label, _)| label != "cm");
```

**Alternatives**:
```rust
// ❌ Creating new Vec (allocates, not in-place)
let filtered: Vec<_> = vec.iter()
    .filter(|item| predicate(item))
    .cloned()
    .collect();

// ❌ Manual loop with remove() (O(n²) due to shifts)
for i in (0..vec.len()).rev() {  // Reverse to avoid index issues
    if !predicate(&vec[i]) {
        vec.remove(i);  // O(n) shift per removal
    }
}

// ✅ .retain() (O(n) single pass, in-place)
vec.retain(|item| predicate(item));
```

**Performance**:
- **Time**: O(n) single pass through vec
- **Space**: O(1) no allocations (modifies in-place)
- **Mechanism**: Compacts array by moving kept items forward, then truncates

**When NOT to use**:
- Need both kept and removed elements → use `.partition()` instead
- Vec is immutable → use `.iter().filter().collect()` to create new vec
- Removing by index not predicate → use `.remove(idx)` or `.drain(range)`

**Zettelkasten**: [[vec-operations]], [[in-place-algorithms]]

---

## �📝 Pattern Extraction Criteria

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
## 🔄 Workflow State Machines (Day 19)

### Pattern: Enum-Based Destination Type Safety
**Used**: Day 19  
**When to use**: State machines with terminal states and transitions
**Code**: `src/solver/day19.rs::Destination`

```rust
enum Destination {
    Accept,           // Terminal success
    Reject,           // Terminal failure
    Workflow(String), // Transition to named workflow
}

// Exhaustive matching enforced by compiler
match destination {
    Destination::Accept => return true,
    Destination::Reject => return false,
    Destination::Workflow(name) => current = name,
}
```

**Benefits**: Compiler prevents missing cases, self-documenting code, impossible invalid states

### Pattern: HashMap Workflow Storage
**Used**: Day 19  
**When to use**: Need O(1) workflow lookup by name
**Code**: `src/solver/day19.rs::parse_workflows()`

```rust
let workflows: HashMap<String, Workflow> = input
    .lines()
    .take_while(|line| !line.is_empty())
    .map(parse_workflow)
    .map(|w| (w.name.clone(), w))
    .collect();

// O(1) lookup
let workflow = workflows.get(current).unwrap();
```

### Pattern: First-Match-Wins Rule Evaluation
**Used**: Day 19  
**When to use**: Priority-based decision trees, conditional chains
**Code**: `src/solver/day19.rs::Workflow::process()`

```rust
for rule in &self.rules {
    if rule.matches(part) {
        return rule.destination();  // First match wins!
    }
}
```

**Note**: Order matters! Subsequent rules only checked if prior ones don't match.

## 🎯 Range Propagation (Day 19 Part 2)

### Pattern: Range Splitting by Conditional Operator
**Used**: Day 19 Part 2  
**When to use**: Partition search space by constraints, constraint propagation
**Code**: `src/solver/day19.rs::split_range()`

```rust
fn split_range(range: Range, op: Op, value: u64) -> (Range, Range) {
    match op {
        Op::LessThan => {
            let matching = Range {
                min: range.min,
                max: range.max.min(value - 1),  // Cap at value-1
            };
            let non_matching = Range {
                min: range.min.max(value),      // Start at value
                max: range.max,
            };
            (matching, non_matching)
        }
        Op::GreaterThan => {
            let matching = Range {
                min: range.min.max(value + 1),  // Start at value+1
                max: range.max,
            };
            let non_matching = Range {
                min: range.min,
                max: range.max.min(value),      // Cap at value
            };
            (matching, non_matching)
        }
    }
}
```

**Key Insight**: Split returns BOTH matching and non-matching ranges - matching goes to destination, non-matching continues to next rule!

### Pattern: DFS with State-Space Range Constraints
**Used**: Day 19 Part 2  
**When to use**: Exponential enumeration impossible, constraints form ranges
**Code**: `src/solver/day19.rs::count_accepted()`

```rust
fn count_accepted(
    workflow_name: &str,
    mut ranges: PartRange,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    // Terminal cases
    if workflow_name == "A" {
        return ranges.combinations();  // Mathematical counting!
    }
    if workflow_name == "R" {
        return 0;
    }
    
    let workflow = workflows.get(workflow_name).unwrap();
    let mut total = 0;
    
    for rule in &workflow.rules {
        match rule {
            Rule::Conditional { attr, op, value, dest } => {
                let (matching, non_matching) = split_range(ranges.get(*attr), *op, *value);
                
                // Recurse with matching range
                if !matching.is_empty() {
                    let mut matching_ranges = ranges;
                    matching_ranges.set(*attr, matching);
                    total += count_accepted(dest_name, matching_ranges, workflows);
                }
                
                // Continue with non-matching (KEY: progressive narrowing!)
                ranges.set(*attr, non_matching);
                
                if ranges.is_empty() {
                    break;
                }
            }
            Rule::Unconditional { dest } => {
                total += count_accepted(dest_name, ranges, workflows);
                break;
            }
        }
    }
    
    total
}
```

**Brilliant Optimization**: Avoids 4000^4 = 256 trillion enumeration! Instead propagates ranges through ~30 workflows.

### Pattern: Mathematical Combination Counting
**Used**: Day 19 Part 2  
**When to use**: Count without enumeration
**Code**: `src/solver/day19.rs::PartRange::combinations()`

```rust
impl PartRange {
    fn combinations(&self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
}

impl Range {
    fn size(&self) -> u64 {
        if self.max >= self.min {
            self.max - self.min + 1
        } else {
            0  // Empty range
        }
    }
}
```

**Result**: Count 123,972,546,935,551 combinations in 190µs without creating a single Part instance!
