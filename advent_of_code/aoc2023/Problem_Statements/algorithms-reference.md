# AoC 2023 - Algorithms Reference

Links to zettelkasten deep dives and implementation details for complex algorithms encountered in AoC 2023.

---

## 📊 Algorithm Usage Summary

| Algorithm | Day(s) | Complexity | Zettelkasten |
|-----------|--------|------------|--------------|
| Linear Scan | Day 1, Day 2 | O(n) | - |
| Delimiter Parsing | Day 2, Day 4, Day 5 | O(n × m) | - |
| Running Maximum | Day 2 | O(n) | - |
| Spatial Indexing | Day 3 | O(n) build, O(1) lookup | [[spatial-indexing-pattern]] |
| Grid Scanning | Day 3 | O(w × h) | - |
| HashSet Membership | Day 4 | O(1) per lookup | - |
| Forward-Propagation DP | Day 4 | O(n × m) | [[memoization-comprehensive-guide]] |
| Range Intersection | Day 5 | O(n × m) | [[interval-algorithms]] |
| Range Mapping | Day 5 | O(ranges × rules × stages) | - |
| Quadratic Formula | Day 6 | O(1) per equation | - |
| Frequency Counting | Day 7 | O(n) | [[entry-api-hashmap]] |
| Custom Ord Multi-Level Sort | Day 7 | O(n log n) | [[custom-ord-pattern]] |
| Graph Traversal (Directed) | Day 8 | O(n) | [[graph-theory-fundamentals]] |
| GCD (Euclidean Algorithm) | Day 8 | O(log min(a,b)) | [[number-theory-basics]] |
| LCM (Cycle Alignment) | Day 8 | O(1) | [[number-theory-basics]] |
| Modular Arithmetic | Day 8 | O(1) | [[number-theory-basics]] |

---

## � Set Operations

### HashSet Membership Testing (Day 4)
**Implementation**: `src/solver/day04.rs::count_matches()`  
**Complexity**: O(m) to build set, O(1) per lookup, O(n) total for n lookups  
**Key Concept**: Use HashSet for O(1) membership testing instead of O(n) linear search  

**When to use**: 
- Need to check "is this element in the collection" repeatedly
- Membership testing is more important than order
- Collection won't change after initial build

**Pattern**:
```rust
// Build HashSet once: O(m)
let winning: HashSet<u32> = winning_numbers
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

// O(1) lookups instead of O(m) linear search
let matches = our_numbers
    .iter()
    .filter(|n| winning.contains(n))  // O(1) per lookup
    .count();
```

**Alternatives**:
- Nested loops: O(n × m) - avoid for large datasets
- Sort + binary search: O(m log m + n log m) - good but HashSet is simpler
- HashSet: O(m + n) - optimal!

**Mission**: Mission 5 (HashSet concepts)

**Zettelkasten**: [[entry-api-hashmap]] (HashSet is similar data structure)

---

## �🔤 String Algorithms

### Position-Based Pattern Matching (Day 1)
**Implementation**: `src/solver/day01.rs`  
**Complexity**: O(n × m) where n = string length, m = pattern count  
**Key Concept**: Scan each position for all possible patterns  

**When to use**: 
- Overlapping patterns need detection
- Simple regex would miss overlaps
- Pattern set is small and fixed

**Zettelkasten**: None (too simple for deep dive)

### Delimiter-Based Parsing (Day 2)
**Implementation**: `src/solver/day02.rs`  
**Complexity**: O(n × m) where n = lines, m = avg delimited segments  
**Key Concept**: Hierarchical split on multiple delimiters (`:`, `;`, `,`)  

**When to use**: 
- Input has structured format with consistent separators
- Need to parse nested data (records → fields → values)
- Simple regex would be overkill

**Pattern**:
```rust
line.split(':')      // Primary structure
    .split(';')      // Secondary sequences  
    .split(',')      // Tertiary elements
    .split_whitespace() // Final values
```

**Zettelkasten**: None (common parsing pattern)

### Running Maximum Tracking (Day 2)
**Implementation**: `src/solver/day02.rs::update_max()`  
**Complexity**: O(n) single pass  
**Key Concept**: Track maximum value seen so far across stream  

**When to use**:
- Need minimum resources to satisfy all observations
- Finding envelope/bounds of variable data
- One-pass streaming aggregation

**Pattern**:
```rust
fn update_max(&mut self, other: &T) {
    self.field = self.field.max(other.field);
}
```

**Zettelkasten**: None (standard accumulation pattern)

---

## 🗺️ Graph Algorithms

### Directed Graph Traversal (Day 8)
**Implementation**: `src/solver/day08.rs::Network::navigate()`  
**Complexity**: O(n) where n = number of steps until destination  
**Key Concept**: Follow edges deterministically through directed graph  

**When to use**: 
- Graph has deterministic edge selection (not search-based)
- Following a path with specific navigation rules
- Directed graph with labeled edges
- Cycle detection needed

**Pattern**:
```rust
struct Network {
    nodes: HashMap<String, (String, String)>,  // node -> (left, right)
}

fn navigate(&self, start: &str, end: &str) -> Result<usize> {
    let mut current = start.to_string();
    let mut steps = 0;
    
    while current != end {
        let (left, right) = self.nodes.get(&current)?;
        current = match instruction {
            'L' => left.clone(),
            'R' => right.clone(),
        };
        steps += 1;
    }
    Ok(steps)
}
```

**Day 8 Application**: Network navigation with L/R instructions
- Each node has exactly 2 outgoing edges (left, right)
- Instructions repeat cyclically
- Part 1: Simple path AAA → ZZZ (19,637 steps)
- Part 2: Multiple paths with cycle detection (8.8 trillion steps via LCM)

**Mission**: Mission 5 (HashMap for O(1) node lookups)

**Zettelkasten**: [[graph-theory-fundamentals]]

### Breadth-First Search (BFS)
**Day(s)**: TBD  
**Zettelkasten**: [[bfs-patterns]]  
**Mission**: Mission 8 (Graph)

### Dijkstra's Algorithm
**Day(s)**: TBD  
**Zettelkasten**: [[dijkstra-algorithm]]  
**Mission**: Mission 8 (Graph)

### A* Search
**Day(s)**: TBD  
**Zettelkasten**: [[a-star-algorithm]]

---

## 🔢 Mathematical Algorithms

### Quadratic Formula (Day 6)
**Implementation**: `src/solver/day06.rs::count_ways_quadratic()`  
**Complexity**: O(1) per equation  
**Key Concept**: Solve quadratic inequalities to find integer solutions without iteration  

**When to use**: 
- Need to find range of values satisfying quadratic inequality
- Brute force iteration would be too slow (millions of values)
- Equation has form ax² + bx + c > 0 (or < 0)
- Precision matters - need exact boundaries

**Pattern**:
```rust
// Problem: Find how many integers h satisfy: h × (T - h) > R
// Rearrange: -h² + T×h - R > 0  or  h² - T×h + R < 0
// Solve h² - T×h + R = 0 using quadratic formula

fn count_ways_quadratic(time: u64, record: u64) -> u64 {
    let t = time as f64;
    let r = record as f64;
    
    // Discriminant: b² - 4ac where a=1, b=-T, c=R
    let discriminant = t * t - 4.0 * r;
    if discriminant < 0.0 { return 0; }  // No real solutions
    
    let sqrt_disc = discriminant.sqrt();
    
    // Two roots: h = (T ± √(T² - 4R)) / 2
    let root1 = (t - sqrt_disc) / 2.0;  // Lower bound
    let root2 = (t + sqrt_disc) / 2.0;  // Upper bound
    
    // Need STRICTLY between roots (> record, not >= record)
    // Handle edge case: if root is exact integer equal to record
    let min_hold = if root1.fract() == 0.0 && 
                      (root1 as u64 * (time - root1 as u64) == record) {
        (root1 as u64) + 1  // Exact match, exclude it
    } else {
        root1.ceil() as u64  // Round up to next integer
    };
    
    let max_hold = if root2.fract() == 0.0 && 
                      (root2 as u64 * (time - root2 as u64) == record) {
        (root2 as u64) - 1  // Exact match, exclude it
    } else {
        root2.floor() as u64  // Round down to prev integer
    };
    
    if max_hold >= min_hold {
        max_hold - min_hold + 1  // Count of integers in range
    } else {
        0
    }
}
```

**Day 6 Application**: Boat race mechanics
- Hold button for h milliseconds → speed = h mm/ms
- Remaining time = (T - h) ms
- Distance = h × (T - h) mm
- Need: distance > record
- Part 1: 4 races with T ≤ 95 → brute force O(T) works
- Part 2: Single race with T = 48,938,595 → quadratic formula O(1) required

**Visual Example**:
```
Time T=7, Record R=9
Distance function: d(h) = h × (7 - h) = -h² + 7h

Parabola (upside-down):
    12 •       
       |   •  •    
    10 | •      •  ← Need d(h) > 9
     9 |----------  ← Record line
       •           •
     0 •••••••••••••
       0 1 2 3 4 5 6 7
       
Roots of -h² + 7h - 9 = 0:
  h = (7 ± √(49-36)) / 2 = (7 ± √13) / 2
  h ≈ 1.697 or 5.303
  
Integers in (1.697, 5.303): {2, 3, 4, 5} → 4 ways
```

**Alternatives**:
```rust
// ❌ Brute force: O(T) - too slow for T=48M
for hold in 0..=time {
    if hold * (time - hold) > record {
        count += 1;
    }
}

// ✅ Quadratic formula: O(1) - instant!
let (min, max) = solve_quadratic_bounds(time, record);
let count = max - min + 1;
```

**Performance Impact**: Day 6 Part 2
- Brute force: ~50 million iterations (unacceptable)
- Quadratic formula: 2 square roots + boundary adjustment (< 1µs)
- Speedup: >50 million × faster

**Key Insights**:
- **Boundary precision**: Must handle case where root equals integer exactly
- **Strict inequality**: Need `> record`, not `>= record` (exclude exact ties)
- **Integer conversion**: `ceil()`/`floor()` for boundaries, adjust for exact roots
- **Discriminant check**: Negative discriminant means no solutions

**Mission**: None (quadratic equations not in current missions)

**Zettelkasten**: [[quadratic-equations]], [[number-theory-basics]]

### Greatest Common Divisor - Euclidean Algorithm (Day 8)
**Implementation**: `src/solver/day08.rs::gcd()`  
**Complexity**: O(log min(a, b))  
**Key Concept**: Recursive division to find largest common divisor  

**When to use**: 
- Need GCD for LCM calculation
- Simplifying fractions
- Finding cycle periods
- Number theory problems

**Pattern**:
```rust
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;  // Key: gcd(a,b) = gcd(b, a mod b)
        a = temp;
    }
    a
}
```

**Mathematical Foundation**:
- **Euclidean algorithm**: 3000+ year old algorithm
- **Key insight**: gcd(a, b) = gcd(b, a mod b)
- **Termination**: When b = 0, gcd = a
- **Efficiency**: Each iteration reduces problem size logarithmically

**Day 8 Application**: Required for LCM calculation in Part 2

**Zettelkasten**: [[number-theory-basics]]

### Least Common Multiple - Cycle Alignment (Day 8)
**Implementation**: `src/solver/day08.rs::lcm()`  
**Complexity**: O(1) given GCD  
**Key Concept**: Find smallest number divisible by both inputs  

**When to use**: 
- Multiple cycles that need to align
- Finding when periodic events synchronize
- Combining frequencies
- Ghost/parallel path problems

**Pattern**:
```rust
fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

// Multi-way LCM using fold
let result = values.iter().fold(1, |acc, &x| lcm(acc, x));
```

**Mathematical Foundation**:
- **Formula**: lcm(a, b) = (a × b) / gcd(a, b)
- **Relationship**: a × b = gcd(a, b) × lcm(a, b)
- **Properties**: 
  - lcm(a, b) ≥ max(a, b)
  - lcm(a, b) ≤ a × b
  - lcm(a, b) = a × b only when gcd(a, b) = 1 (coprime)

**Day 8 Application**: Ghost navigation synchronization
- Each ghost has cycle length (time to reach **Z)
- Find when ALL cycles align simultaneously
- Alternative would require simulating 8+ trillion steps
- LCM reduces to millisecond calculation

**Example**:
```rust
// Ghost 1: reaches goal every 4 steps
// Ghost 2: reaches goal every 6 steps
// When do both reach goal together?
lcm(4, 6) = 24 / gcd(4,6) = 24 / 2 = 12 steps
```

**Performance Impact**: 
- Brute force: Would need to simulate 8,811,050,362,409 steps (impossible)
- LCM approach: ~6.7ms total for Part 2
- **Optimization**: Infinite speedup (brute force intractable)

**Zettelkasten**: [[number-theory-basics]]

### Modular Arithmetic - Cyclic Wrapping (Day 8)
**Implementation**: Used throughout `src/solver/day08.rs`  
**Complexity**: O(1) per operation  
**Key Concept**: Wrap indices/values cyclically using modulo  

**When to use**: 
- Repeating sequences (instructions, patterns)
- Circular buffers
- Cyclic iteration
- Clock arithmetic

**Pattern**:
```rust
// Wrap index to array bounds
let idx = (idx + 1) % array.len();

// Repeating instruction sequence
let instruction = instructions[step_count % instructions.len()];
```

**Day 8 Application**: Instruction wrapping
- Instructions "LLR" repeat: L, L, R, L, L, R, L, L, R, ...
- After reaching end, wrap to beginning
- `instruction_idx = (instruction_idx + 1) % instructions.len()`

**Mathematical Properties**:
- `(a + b) mod n = ((a mod n) + (b mod n)) mod n`
- `(a × b) mod n = ((a mod n) × (b mod n)) mod n`
- `a mod n` gives remainder in range [0, n-1]

**Zettelkasten**: [[number-theory-basics]]

### Frequency Counting with HashMap (Day 7)
**Implementation**: `src/solver/day07.rs::determine_type()`  
**Complexity**: O(n) to build, O(1) per lookup  
**Key Concept**: Count occurrences of elements to determine patterns  

**When to use**: 
- Need to know "how many times does X appear"
- Determining patterns from frequency distribution
- Grouping elements by count
- Finding duplicates or unique elements

**Pattern**:
```rust
// Build frequency map
let mut counts: HashMap<T, usize> = HashMap::new();
for &item in items {
    *counts.entry(item).or_insert(0) += 1;
}

// Extract frequency distribution
let mut frequencies: Vec<usize> = counts.values().copied().collect();
frequencies.sort_by(|a, b| b.cmp(a));  // Descending

// Pattern matching on frequencies
match frequencies.as_slice() {
    [5] => "All same",
    [4, 1] => "Four of one, one different",
    [3, 2] => "Three and two",
    // ...
}
```

**Day 7 Application**: Poker hand classification
- Count how many of each card appears
- Pattern [5] = five of a kind, [4,1] = four of a kind, [3,2] = full house, etc.
- Frequencies array provides canonical representation of hand type

**Entry API Pattern**:
```rust
// ❌ Multiple HashMap lookups
if let Some(count) = map.get_mut(&key) {
    *count += 1;
} else {
    map.insert(key, 1);
}

// ✅ Single lookup with Entry API
*map.entry(key).or_insert(0) += 1;
```

**Alternatives**:
```rust
// ❌ Manual counting: O(n²)
for item in items {
    let count = items.iter().filter(|&x| x == item).count();
}

// ✅ HashMap: O(n)
let counts: HashMap<_, _> = items.iter()
    .fold(HashMap::new(), |mut acc, &item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });
```

**Mission**: Mission 5 (HashMap entry API)

**Zettelkasten**: [[entry-api-hashmap]]

### Custom Ord for Multi-Level Sorting (Day 7)
**Implementation**: `src/solver/day07.rs::impl Ord for Hand`  
**Complexity**: O(n log n) for sort, O(k) per comparison where k = comparison levels  
**Key Concept**: Define custom ordering with multiple comparison criteria  

**When to use**: 
- Need to sort complex types with multiple fields
- Primary/secondary/tertiary sorting criteria
- Natural ordering doesn't match default derive
- Want to use `.sort()` instead of `.sort_by()`

**Pattern**:
```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Ord for ComplexType {
    fn cmp(&self, other: &Self) -> Ordering {
        // Primary comparison
        match self.priority.cmp(&other.priority) {
            Ordering::Equal => {
                // Secondary comparison (tiebreaker)
                match self.name.cmp(&other.name) {
                    Ordering::Equal => {
                        // Tertiary comparison
                        self.id.cmp(&other.id)
                    }
                    other => other,
                }
            }
            other => other,
        }
    }
}

impl PartialOrd for ComplexType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

**Day 7 Application**: Poker hand ranking
- Primary: Hand type (five of a kind > four of a kind > ... > high card)
- Secondary: Card-by-card comparison left-to-right (A > K > Q > ...)
- Enables `hands.sort()` for automatic multi-level sorting

**Simplified Pattern** (for iterator comparisons):
```rust
// Compare element-by-element with early termination
for i in 0..n {
    match self.items[i].cmp(&other.items[i]) {
        Ordering::Equal => continue,  // Try next item
        other => return other,         // Found difference
    }
}
Ordering::Equal  // All items equal
```

**Benefits**:
- Type-safe comparison
- Automatic `.sort()` integration
- No need for custom comparator closures
- Self-documenting ordering logic

**Mission**: None (Ord trait is standard library)

**Zettelkasten**: [[custom-ord-pattern]] (if pattern repeats in future days)

### Greatest Common Divisor (GCD) / Least Common Multiple (LCM)
**Day(s)**: TBD  
**Zettelkasten**: [[number-theory-basics]]

### Modular Arithmetic
**Day(s)**: TBD  
**Zettelkasten**: [[modular-arithmetic]]

---

## 🧩 Dynamic Programming

### Forward-Propagation DP (Day 4)
**Implementation**: `src/solver/day04.rs::solve_part2()`  
**Complexity**: O(n × m) where n = items, m = average propagation width  
**Key Concept**: Process items sequentially, each affecting future items based on current state  

**When to use**: 
- State changes propagate forward (no backwards dependencies)
- Future values depend on accumulated state from earlier items
- One-pass solution possible (no recursion needed)
- Cascading effects (copies winning more copies)

**Pattern**:
```rust
// Initialize state array
let mut state = vec![initial_value; n];

// Process each item, updating future items
for (i, item) in items.iter().enumerate() {
    let current_state = state[i];
    let effect_range = calculate_effect(item);
    
    // Propagate current state forward
    for j in 1..=effect_range {
        if i + j < n {
            state[i + j] += current_state;  // Accumulate
        }
    }
}

let result: u32 = state.iter().sum();
```

**Day 4 Application**: Scratchcard copies cascade forward
- Each card with N matches wins copies of next N cards
- Copies also win more copies (multiplier effect)
- Track count per card, propagate forward

**Zettelkasten**: [[memoization-comprehensive-guide]], [[Dynamic Programming]]

### Memoization Patterns
**Day(s)**: TBD  
**Zettelkasten**: [[memoization-aoc2024-patterns]]

---

## 🔢 Interval/Range Algorithms

### Range Intersection and Splitting (Day 5)
**Implementation**: `src/solver/day05.rs::map_range_through_single_rule()`  
**Complexity**: O(1) per range-rule pair, O(n × m) for n ranges through m rules  
**Key Concept**: Split input ranges into mapped and unmapped portions based on intersection with mapping rules  

**When to use**: 
- Need to transform large ranges of values without iterating individual elements
- Interval mapping problems (coordinates, time ranges, value transformations)
- Dataset too large to process element-by-element (billions of values)
- Transformations can be expressed as range-to-range mappings

**Pattern**:
```rust
fn map_range_through_rule(range: Range, rule: &RangeMap) 
    -> (Option<Range>, Vec<Range>) 
{
    // Check for intersection
    let intersection_start = range.start.max(rule.source_start);
    let intersection_end = range.end().min(rule.source_end());
    
    if intersection_start >= intersection_end {
        return (None, vec![range]);  // No overlap
    }
    
    // Map the intersection
    let offset = intersection_start - rule.source_start;
    let mapped = Range {
        start: rule.dest_start + offset,
        length: intersection_end - intersection_start,
    };
    
    // Collect unmapped parts (before/after intersection)
    let mut unmapped = Vec::new();
    if range.start < intersection_start {
        unmapped.push(Range {
            start: range.start,
            length: intersection_start - range.start,
        });
    }
    if range.end() > intersection_end {
        unmapped.push(Range {
            start: intersection_end,
            length: range.end() - intersection_end,
        });
    }
    
    (Some(mapped), unmapped)
}
```

**Visual Example**:
```
Input range:  [50...................70)
Rule source:       [55.......65)
Rule dest:        [100......110)

Result:
  Before:     [50..55)           ← unmapped
  Intersection:    [55..65) → [100..110) ← mapped
  After:               [65..70)  ← unmapped
```

**Day 5 Application**: Seed range transformations
- Part 1: 20 individual seeds → 34.6µs
- Part 2: 650 million seeds as 10 ranges → 783.8µs
- **Complexity paradox**:
  - Theoretical worst case: O(3^M) per range where M = rules per stage
  - Could compound to exponential growth across 7 stages
  - Actual observed: 10 → 40 → 63 → 85 → 101 → 115 → 125 → 141 ranges
  - Growth factor: 4× initial, then decreases to ~1.1-1.2× per stage
  - Total: 14.1× growth (not 2,187× theoretical maximum)
- **Why it works**: Real data is well-behaved (most ranges don't intersect most rules)
- **Performance**: O(ranges × rules × stages) with low constant factors

**Alternatives**:
```rust
// ❌ Iterate all values: O(n) where n = billions
for value in range.start..range.end() {
    let mapped = apply_transformation(value);
}

// ✅ Range splitting: O(k) where k = number of ranges (dozens)
let output_ranges = apply_range_transformation(input_range, rules);
```

**Key Insights**:
- **Range compression**: Handle billions of values as dozens of ranges
- **Cascading splits**: Ranges fragment through multiple stages but remain manageable
- **Unmapped passthrough**: Values not covered by rules map to themselves
- **Order matters**: Process rules sequentially to avoid double-mapping

**Mission**: None (interval algorithms not in current missions)

**Zettelkasten**: [[interval-algorithms]] (if pattern repeats)

---

## 🔍 Search Algorithms

*To be populated as search problems are solved.*

### Binary Search Variants
**Day(s)**: TBD  
**Zettelkasten**: [[binary-search-patterns]]  
**Mission**: Mission 3 (Binary Search)

### Backtracking
**Day(s)**: TBD  
**Zettelkasten**: [[backtracking-patterns]]

---

## 📐 Geometric Algorithms

### Grid Scanning with Adjacency Checks (Day 3)
**Implementation**: `src/solver/day03.rs`  
**Complexity**: O(w × h) for grid scan, O(8) for neighbor checks  
**Key Concept**: Scan grid for patterns, check 8-directional neighbors  
**Mission**: Mission 6 (Grid, Coord)

**When to use**: 
- 2D grid problems with adjacency requirements
- Symbol/pattern detection in grids
- Local neighborhood analysis

**Pattern**:
```rust
for y in 0..grid.height() {
    for x in 0..grid.width() {
        let coord = Coord::new(x, y);
        for neighbor in coord.neighbors_8() {
            // Check adjacent cells
        }
    }
}
```

**Zettelkasten**: None (standard grid traversal)

### Spatial Indexing for Grid Lookups (Day 3 Optimization)
**Implementation**: `src/solver/day03.rs::solve_part2()`  
**Complexity**: O(n × d) to build index, O(1) per lookup  
**Key Concept**: HashMap mapping coordinates to entities for instant lookups  

**When to use**: 
- Need to find "what's at this coordinate" repeatedly
- Reverse lookups (coordinate → entity instead of entity → coordinates)
- O(1) spatial queries instead of O(n) linear search

**Pattern**:
```rust
// Build spatial index: coord → entity_id
let mut coord_to_entity: HashMap<Coord, usize> = HashMap::new();
for (id, entity) in entities.iter().enumerate() {
    for coord in entity.occupied_coords() {
        coord_to_entity.insert(coord, id);
    }
}

// O(1) lookup instead of O(n) linear search
if let Some(&entity_id) = coord_to_entity.get(&target_coord) {
    // Found entity at coordinate
}
```

**Optimization Impact**: Day 3 Part 2 speedup: ~100x (millions of ops → 160K ops)

**Zettelkasten**: [[spatial-hash]] (if pattern repeats)

### Flood Fill
**Day(s)**: TBD  
**Mission**: Mission 6 (Grid) + Mission 8 (Graph)

### Shoelace Formula / Pick's Theorem
**Day(s)**: TBD  
**Zettelkasten**: [[computational-geometry]]

---

## 🔗 Mission Integration Map

| Mission | Algorithms | Days Used |
|---------|------------|-----------|
| Mission 3 (Binary Search) | Binary search variants | TBD || Mission 5 (HashMap) | HashSet membership testing, O(1) lookups | Day 4 || Mission 6 (Grid) | Grid traversal, 8-directional neighbors, spatial indexing | Day 3 |
## 📝 Notes

- Algorithm entries are created when first encountered
- Deep dives go to zettelkasten (linked from here)
- Implementation details stay in solver files
- This file serves as navigation hub
