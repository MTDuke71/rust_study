# AoC 2023 - Patterns Catalog

Reusable patterns extracted from daily solutions. Patterns are added when used in 3+ days.

---

## 📊 Pattern Usage Summary

| Pattern | Days Used | Location |
|---------|-----------|----------|
| Forward/Reverse Search | Day 1 | `day01.rs` |
| Delimiter-Based Parsing | Day 1, Day 2, Day 4 | `day01.rs`, `day02.rs`, `day04.rs` |
| Spatial Indexing | Day 3 | `day03.rs` |
| HashSet Membership | Day 4 | `day04.rs` |
| Forward-Propagation DP | Day 4 | `day04.rs` |

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

---

## 🔢 Mathematical Patterns

*Patterns to be extracted as math-heavy problems are solved.*

### Pattern: GCD/LCM for Cycle Detection
**Used**: TBD  
**Code**: `src/patterns/number_theory.rs` (when created)

---

## 🌐 Graph Patterns

*Patterns to be extracted as graph problems are solved.*

### Pattern: Dijkstra with Priority Queue
**Used**: TBD  
**Code**: `src/patterns/pathfinding.rs` (when created)

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
