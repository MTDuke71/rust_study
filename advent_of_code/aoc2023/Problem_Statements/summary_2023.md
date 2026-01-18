# AoC 2023 - Summary

## 📊 Stats Dashboard

| Metric | Value |
|--------|-------|\n| **Progress** | 17/25 ⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐ |
| **Total Runtime** | 344.4ms |
| **Mission Integration** | 5 days (Day 3: Mission 6, Day 4: Mission 5, Day 10: Mission 6 + Mission 8, Day 14: Mission 6, Day 17: Mission 6) |
| **Patterns Extracted** | 14 (delimiter parsing, spatial indexing, HashSet membership, forward-propagation DP, range intersection, recursive differences, BFS loop traversal, ray casting point-in-polygon, recursive DP with memoization, Hamming distance pattern matching, state hashing for cycle detection, modulo fast-forward, hashmap simulation with labeled data, state-space beam tracing with cycle detection) |

---

## 🔍 Quick Navigation

[Day 1](#day-1-trebuchet) | [Day 2](#day-2-cube-conundrum) | [Day 3](#day-3-gear-ratios) | [Day 4](#day-4-scratchcards) | [Day 5](#day-5-if-you-give-a-seed-a-fertilizer) | [Day 6](#day-6-wait-for-it) | [Day 7](#day-7-camel-cards) | [Day 8](#day-8-haunted-wasteland) | [Day 9](#day-9-mirage-maintenance) | [Day 10](#day-10-pipe-maze) | [Day 11](#day-11-cosmic-expansion) | [Day 12](#day-12-hot-springs) | [Day 13](#day-13-point-of-incidence) | [Day 14](#day-14-parabolic-reflector-dish) | [Day 15](#day-15-lens-library) | [Day 16](#day-16-the-floor-will-be-lava) | [Day 17](#day-17-clumsy-crucible) |
Day 18 | Day 19 | Day 20 |
Day 21 | Day 22 | Day 23 | Day 24 | Day 25

**Reference**: [Patterns Catalog](patterns-catalog.md) | [Algorithms Reference](algorithms-reference.md) | [Performance Analysis](performance-analysis.md)

---

## 📅 Daily Solutions

### Day 1: Trebuchet?!

**Part 1**: Extract first/last numeric digits, combine into calibration values → **54239**  
**Part 2**: Same but digits can be spelled out (one, two, etc.) → **55343**  

**Algorithm**: Linear scan with pattern matching  
**Complexity**: O(n × m) where n = input length, m = max word length (5)  
**Runtime**: 66µs (Part 1: 28µs, Part 2: 38µs)  
**Mission**: None  

**Key Insight**: Overlapping word digits (e.g., `eightwothree` → 8, 3) require position-by-position scanning rather than simple string replacement.

**Rust Highlights**:
- `find_map` for early termination - stop at first/last match
- `.rev()` on range iterator for backward position search
- `str::starts_with()` for word prefix matching
- Zero allocations - no intermediate `Vec` needed

**Code Highlight**:
```rust
// Part 1: Forward/reverse char search with early termination
fn extract_digits_part1(line: &str) -> Option<(u32, u32)> {
    let first = line.chars().find_map(|c| c.to_digit(10))?;
    let last = line.chars().rev().find_map(|c| c.to_digit(10))?;
    Some((first, last))
}

// Part 2: Forward/reverse position search for words
fn extract_digits_part2(line: &str) -> Option<(u32, u32)> {
    let first = (0..line.len()).find_map(|pos| digit_at_position(line, pos))?;
    let last = (0..line.len()).rev().find_map(|pos| digit_at_position(line, pos))?;
    Some((first, last))
}
```

**Tests**: 
- ✅ Part 1 example (142)
- ✅ Part 2 example (281)
- ✅ Individual line extraction tests

**Zettelkasten**: None (straightforward parsing problem)

**Links**: ← Day 1 | [Day 2](#day-2-cube-conundrum) →

---

### Day 2: Cube Conundrum

**Part 1**: Determine possible games if bag has 12R/13G/14B cubes → **2505**  
**Part 2**: Find minimum cubes needed per game, sum powers → **70265**  

**Algorithm**: Linear scan with delimiter parsing and accumulation  
**Complexity**: O(n × m) where n = games, m = reveals per game  
**Runtime**: 125.8µs (Part 1: 52.5µs, Part 2: 73.3µs)  
**Mission**: None  

**Key Insight**: Minimum cubes needed = maximum ever shown of each color across all reveals. Track running max with `update_max()` method.

**Rust Highlights**:
- `filter_map` combining parse + filter in one iterator step
- Struct methods: `is_possible()`, `power()`, `update_max()`
- Delimiter-based parsing: `split(':')`, `split(';')`, `split(',')`
- Zero allocations - process line by line

**Code Highlight**:
```rust
// Part 2: Track maximum of each color
fn update_max(&mut self, other: &CubeSet) {
    self.red = self.red.max(other.red);
    self.green = self.green.max(other.green);
    self.blue = self.blue.max(other.blue);
}

// Main logic
let sum: u32 = input
    .lines()
    .filter_map(|line| {
        parse_game_minimum(line)
            .ok()
            .map(|(_, min_set)| min_set.power())
    })
    .sum();
```

**Tests**: 
- ✅ Part 1 example (8)
- ✅ Part 2 example (2286)
- ✅ Cube power calculation
- ✅ Minimum cubes for Game 1
- ✅ Parse cube set
- ✅ Cube set validation

**Zettelkasten**: None (straightforward parsing + accumulation)

**Links**: ← [Day 1](#day-1-trebuchet) | [Day 3](#day-3-gear-ratios) →

---

### Day 3: Gear Ratios

**Part 1**: Find numbers adjacent to symbols, sum them → **554003**  
**Part 2**: Find gears (* adjacent to exactly 2 numbers), sum gear ratios → **87263515**  

**Algorithm**: Grid scanning with 8-directional adjacency + spatial indexing  
**Complexity**: O(w×h) grid scan, O(n×d) index build, O(1) lookups  
**Runtime**: 926.1µs (Part 1: 622.1µs, Part 2: 304.0µs)  
**Mission**: Mission 6 (Grid<char>, Coord::neighbors_8())  

**Key Insight**: Reverse the search! Instead of "for each *, find all adjacent numbers" (expensive), build a spatial index and ask "which number is at each neighbor of *" (fast).

**Rust Highlights**:
- Mission 6 integration - `Grid<char>` and `Coord::neighbors_8()`
- Spatial indexing: `HashMap<Coord, NumberId>` for O(1) lookups
- HashSet for adjacency deduplication (O(1) insert vs O(n) contains)
- Integrator philosophy: Compose from validated Mission 6 components

**Code Highlight**:
```rust
// OPTIMIZATION: Build spatial index for O(1) gear lookups
let mut coord_to_number: HashMap<Coord, usize> = HashMap::new();
for (idx, num) in numbers.iter().enumerate() {
    for coord in num.digit_coords() {
        coord_to_number.insert(coord, idx);
    }
}

// For each *, check 8 neighbors in index (not all numbers!)
for neighbor in gear_coord.neighbors_8() {
    if let Some(&num_idx) = coord_to_number.get(&neighbor) {
        adjacent_numbers.insert(num_idx);  // HashSet deduplication
    }
}
```

**Optimization**: ~100x speedup for Part 2 (millions of ops → 160K ops)
- Before: O(gears × numbers × adjacency_calc)
- After: O(numbers × digits + gears × 8)
- Pattern: Spatial indexing for reverse lookups

**Tests**: 
- ✅ Part 1 example (4361)
- ✅ Part 2 example (467835)
- ✅ Grid parsing (140×140)
- ✅ Number extraction (10 numbers in example)
- ✅ Symbol detection
- ✅ Number adjacency checks

**Zettelkasten**: [[spatial-hash]] (if pattern repeats)

**Links**: ← [Day 2](#day-2-cube-conundrum) | [Day 4](#day-4-scratchcards) →

---

### Day 4: Scratchcards

**Part 1**: Sum points from matching numbers (first match = 1 pt, doubles each match) → **32001**  
**Part 2**: Count total cards after cascading copies (matches win copies of next N cards) → **5037841**  

**Algorithm**: HashSet membership testing + forward-propagation DP  
**Complexity**: O(n × m) where n = cards, m = numbers per card  
**Runtime**: 362.9µs (Part 1: 176.6µs, Part 2: 186.3µs, Criterion benchmarks)  
**Mission**: Mission 5 (HashSet concepts)  

**Key Insight**: Part 1 uses HashSet for O(1) membership testing (3× faster than nested loops). Part 2 is forward-propagation DP - each card affects future cards, creating cascading multiplier effect.

**Rust Highlights**:
- `HashSet::contains()` for O(1) membership testing vs O(n) linear search
- Iterator chain: `.iter().filter().count()` - zero allocations
- Bit shifting for powers of 2: `1 << (n-1)` instead of `2.pow(n-1)`
- Forward-propagation DP with single `Vec` pass
- Integrator philosophy: Use Mission 5 HashSet instead of custom solutions

**Code Highlight**:
```rust
// Part 1: HashSet for O(1) lookups
let winning: HashSet<u32> = winning_part
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

// O(n) iteration with O(1) lookups = O(n) total
let matches = our_numbers
    .iter()
    .filter(|n| winning.contains(n))
    .count();

// Part 2: Forward-propagation DP
let mut card_counts = vec![1u32; num_cards];
for (i, line) in lines.iter().enumerate() {
    let matches = count_matches(line);
    let current_count = card_counts[i];
    
    // Propagate forward: each copy wins more cards
    for j in 1..=matches {
        if i + j < num_cards {
            card_counts[i + j] += current_count;
        }
    }
}
```

**Complexity Analysis**:
- **Part 1**: O(n × m) where n = cards, m = avg numbers per card
  - Parse: O(m) per card
  - HashSet build: O(m)
  - Membership tests: O(m) with O(1) per test
- **Part 2**: O(n × k) where k = avg matches per card
  - Single pass through cards: O(n)
  - Forward propagation: O(k) per card
  - In practice: O(n) since k is small

**Algorithm Alternatives**:
```rust
// ❌ Nested loops: O(n × m × k) - 3× slower
our.iter().filter(|n| winning.iter().any(|w| w == n))

// ⚠️ Sort + binary: O(m log m + n log m) - 2× slower  
let mut sorted = winning; sorted.sort();
our.iter().filter(|n| sorted.binary_search(n).is_ok())

// ✅ HashSet: O(m + n) - optimal!
let set: HashSet<_> = winning.collect();
our.iter().filter(|n| set.contains(n))
```

**Tests**: 
- ✅ Part 1 example (13)
- ✅ Part 2 example (30)
- ✅ Count matches (4, 2, 0)
- ✅ Calculate points (0→0, 1→1, 2→2, 3→4, 4→8)
- ✅ Full cascade simulation

**Zettelkasten**: [[entry-api-hashmap]], [[memoization-comprehensive-guide]], [[Dynamic Programming]]

**Links**: ← [Day 3](#day-3-gear-ratios) | [Day 5](#day-5-if-you-give-a-seed-a-fertilizer) →

---

### Day 5: If You Give A Seed A Fertilizer

**Part 1**: Map seeds through transformation pipeline (seed→soil→...→location), find min → **379811651**  
**Part 2**: Seeds are range pairs (650M values), efficiently map ranges → **27992443**  

**Algorithm**: Range intersection and splitting for interval mapping  
**Complexity**: O(ranges × rules × stages) - ranges stay small despite splitting  
**Runtime**: 818.4µs (Part 1: 34.6µs, Part 2: 783.8µs, Criterion benchmarks)  
**Mission**: None (interval algorithms not in missions)  

**Key Insight**: Part 2 explodes to 650 million seeds, but treating numbers as ranges allows processing in the same time as Part 1. Range intersection splits one range into mapped/unmapped parts; unmapped parts propagate through remaining rules.

**Rust Highlights**:
- Range splitting: `(Option<Range>, Vec<Range>)` return type for mapped/unmapped parts
- Cascading transformations: 7 stages, each potentially splitting ranges
- `.max()`/`.min()` for intersection boundaries
- No Mission integration (interval algorithms not covered)
- Efficiency: 10 input ranges → 141 final ranges vs 650M individual values

**Code Highlight**:
```rust
// Part 1: Map individual numbers through transformation pipeline
for seed in seeds {
    let mut current = seed;
    for map_rules in &all_maps {
        current = map_through_ranges(current, map_rules);
    }
    min_location = min_location.min(current);
}

// Part 2: Map ranges through pipeline - same complexity!
let mut current_ranges = parse_seed_ranges(input)?;  // 10 ranges
for map_rules in &all_maps {  // 7 stages
    let mut next_ranges = Vec::new();
    for range in current_ranges {
        // Range splits into mapped/unmapped parts
        let mapped = map_range_through_rules(range, map_rules);
        next_ranges.extend(mapped);
    }
    current_ranges = next_ranges;  // ~10-50 ranges per stage
}
let min = current_ranges.iter().map(|r| r.start).min().unwrap();
```

**Range Splitting Logic**:
```rust
fn map_range_through_single_rule(range: Range, rule: &RangeMap) 
    -> (Option<Range>, Vec<Range>) 
{
    // Find intersection between range and rule
    let intersection_start = range.start.max(rule.source_start);
    let intersection_end = range.end().min(rule.source_end());
    
    // No overlap → return entire range unmapped
    if intersection_start >= intersection_end {
        return (None, vec![range]);
    }
    
    // Map intersection, collect before/after unmapped parts
    let mapped = Range { /* transformed intersection */ };
    let unmapped = vec![/* before */, /* after */];
    
    (Some(mapped), unmapped)
}
```

**Visual Example**:
```
Range [50..70) through rule [55..65) → [100..110):

[50...................70)
     [55.......65)  ← rule source
      ▼         ▼
    [100......110)  ← rule dest

Splits into:
  [50..55)   - unmapped before
  [100..110) - mapped intersection  
  [65..70)   - unmapped after
```

**Complexity Analysis - Theoretical vs Actual**:
```
Theoretical worst case: Each range can split 3-way per rule
- Stage 1: 1 range → 3^M possible (M = rules per stage)
- Across 7 stages: Could compound exponentially

Actual observed growth (real puzzle input):
Stage | Ranges | Growth Factor | Notes
------|--------|---------------|-------
  0   |   10   |     -         | Input seed ranges
  1   |   40   |   4.00×       | Initial split (high)
  2   |   63   |   1.58×       | Growth slowing
  3   |   85   |   1.35×       | Stabilizing
  4   |  101   |   1.19×       | Linear growth
  5   |  115   |   1.14×       | Sub-linear
  6   |  125   |   1.09×       | Flattening
  7   |  141   |   1.13×       | Final (14.1× total)

Theoretical max: 3^7 × 10 = 21,870 ranges
Actual result: 141 ranges (155× better than worst case!)

Why the discrepancy?
- Most ranges don't intersect most rules
- Many intersections consume entire range (no fragmentation)
- Real data has favorable distribution
```

**Performance**:
- **Part 1**: 20 seeds × 7 stages = 140 transformations → **34.6µs**
- **Part 2**: 650M seeds compressed to 10 ranges → **141 ranges** after 7 stages → **783.8µs**
- **Range Growth Analysis** (actual data from real input):
  - Seeds: 10 ranges
  - After stage 1: 40 ranges (4× initial split)
  - After stage 2-7: 63→85→101→115→125→141 (growth factor decreases: 1.58×→1.35×→1.19×→1.14×→1.09×→1.13×)
  - **Theoretical worst case**: 3^7 = 2,187 ranges per input range
  - **Actual**: 14.1× total growth (much better than exponential!)
- **Why Part 2 is slower**: Range splitting/tracking overhead + 14× more ranges to process
- **Still efficient**: 783µs to handle 650M values (would take hours to iterate individually)

**Tests**: 
- ✅ Part 1 example (35)
- ✅ Part 2 example (46)
- ✅ Range intersection logic
- ✅ Range splitting (1 range → up to 3 parts)
- ✅ Seed range parsing (pairs)
- ✅ Individual range mapping

**Zettelkasten**: [[interval-algorithms]] (if pattern repeats)

**Links**: ← [Day 4](#day-4-scratchcards) | [Day 6](#day-6-wait-for-it) →

---

### Day 6: Wait For It

**Part 1**: Count ways to beat boat race records (4 races) → **2756160**  
**Part 2**: Same but single huge race (bad kerning = concatenate numbers) → **34788142**  

**Algorithm**: Quadratic formula for finding integer solutions to distance inequality  
**Complexity**: O(1) per race using quadratic formula vs O(T) brute force  
**Runtime**: 0.95µs (Part 1: 0.65µs, Part 2: 0.30µs) - **Part 2 faster than Part 1!**  
**Mission**: None  

**Key Insight**: Distance = hold_time × (time - hold_time) is a quadratic equation. Part 2 has time=48,938,595 making brute force impractical. Quadratic formula solves in O(1) by finding the two roots of h² - T×h + R = 0, then counting integers strictly between them.

**Rust Highlights**:
- Quadratic formula: `(T ± √(T² - 4R)) / 2`
- Edge case handling: exact integer roots that equal record (exclude them)
- Boundary adjustment: `ceil()` for lower bound, `floor()` for upper bound
- Part 1 uses brute force O(T) - fast enough for T ≤ 95
- Part 2 requires quadratic formula - 50M iterations → instant

**Code Highlight**:
```rust
// Part 1: Brute force works for small T
fn count_ways_to_win(time: u64, record: u64) -> u64 {
    (0..=time)
        .filter(|&h| h * (time - h) > record)
        .count() as u64
}

// Part 2: Quadratic formula for large T (48M iterations → O(1))
fn count_ways_quadratic(time: u64, record: u64) -> u64 {
    let t = time as f64;
    let r = record as f64;
    let discriminant = t * t - 4.0 * r;
    let sqrt_disc = discriminant.sqrt();
    
    let root1 = (t - sqrt_disc) / 2.0;
    let root2 = (t + sqrt_disc) / 2.0;
    
    // Adjust boundaries for strict inequality (> record)
    let min_hold = if root1.fract() == 0.0 && /* exact match */ {
        (root1 as u64) + 1
    } else {
        root1.ceil() as u64
    };
    
    let max_hold = if root2.fract() == 0.0 && /* exact match */ {
        (root2 as u64) - 1
    } else {
        root2.floor() as u64
    };
    
    max_hold - min_hold + 1
}
```

**Tests**: 
- ✅ Part 1 example (288)
- ✅ Part 2 example (71503)
- ✅ Individual race calculations (4, 8, 9 ways)
- ✅ Parse concatenated numbers

**Zettelkasten**: [[quadratic-equations]], [[number-theory-basics]]

**Links**: ← [Day 5](#day-5-if-you-give-a-seed-a-fertilizer) | [Day 7](#day-7-camel-cards) →

---

### Day 7: Camel Cards

**Part 1**: Rank poker-style hands, calculate winnings → **248217452**  
**Part 2**: J cards become Jokers (wildcards), recalculate → **245576185**  

**Algorithm**: Frequency counting + custom Ord for multi-level sorting  
**Complexity**: O(n log n) for sorting, O(n) for hand type detection  
**Runtime**: 725.3µs (Part 1: 290.0µs, Part 2: 435.3µs)  
**Mission**: Mission 5 (HashMap for frequency counting)  

**Key Insight**: Part 1 uses HashMap to count card frequencies, determining hand type from frequency pattern. Part 2 adds Joker logic: J becomes weakest card for tie-breaking but acts as wildcard for hand type (add joker count to most frequent non-joker card).

**Rust Highlights**:
- Custom `Ord` implementation for multi-level comparison
- Enum with numeric values for natural ordering
- HashMap frequency counting (Mission 5 concept)
- Separate types (`Hand` vs `Hand2`) for clean Part 1/Part 2 separation
- No trait objects - static dispatch

**Code Highlight**:
```rust
// Determine hand type by frequency counting
let mut counts: HashMap<Card, usize> = HashMap::new();
for &card in cards {
    *counts.entry(card).or_insert(0) += 1;
}
let mut frequencies: Vec<usize> = counts.values().copied().collect();
frequencies.sort_by(|a, b| b.cmp(a));

match frequencies.as_slice() {
    [5] => HandType::FiveOfAKind,
    [4, 1] => HandType::FourOfAKind,
    [3, 2] => HandType::FullHouse,
    // ...
}

// Part 2: Jokers become most frequent card
let joker_count = cards.iter().filter(|&&c| c == Joker).count();
frequencies[0] += joker_count;  // Add to most frequent

// Custom Ord enables simple sorting
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                // Tiebreaker: compare cards left-to-right
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

hands.sort();  // Automatic multi-level sorting!
```

**Algorithm Details**:
- **Hand type detection**: O(5) to count + O(5) to sort frequencies = O(1) per hand
- **Sorting**: O(n log n) where n = number of hands
- **Comparison**: O(1) type compare + O(5) card compare = O(1)
- **Part 2 optimization**: Greedy joker assignment (add to most frequent) is provably optimal

**Tests**: 
- ✅ Part 1 example (6440)
- ✅ Part 2 example (5905)
- ✅ Hand type detection (7 types)
- ✅ Hand ordering (same type, different cards)
- ✅ Joker hand type (wildcards)
- ✅ Joker ordering (J weakest for tie-breaking)

**Zettelkasten**: [[entry-api-hashmap]] (frequency counting), [[custom-ord-pattern]] (if pattern repeats)

**Links**: ← [Day 6](#day-6-wait-for-it) | [Day 8](#day-8-haunted-wasteland) →

---

### Day 8: Haunted Wasteland

**Part 1**: Navigate network from AAA to ZZZ following L/R instructions → **19637**  
**Part 2**: Ghost navigation - start at all **A nodes, find when all reach **Z simultaneously → **8811050362409**  

**Algorithm**: Part 1: Graph traversal with cyclic instructions; Part 2: Cycle detection + LCM optimization  
**Complexity**: O(n) for Part 1 where n = steps; O(k × m + k log k) for Part 2 where k = ghosts, m = avg cycle length  
**Runtime**: ~8.2ms (Part 1: 1.5ms, Part 2: 6.7ms, Criterion benchmarks)  
**Mission**: Mission 5 (HashMap for O(1) node lookups)  

**Key Insight**: Part 2 cannot be brute-forced (8+ trillion steps). Solution: Each ghost follows a cyclic pattern. Find cycle length for each ghost, then calculate LCM to determine when all cycles align. This reduces impossible simulation to millisecond calculation.

**Rust Highlights**:
- HashMap for O(1) graph node lookups (Mission 5)
- Euclidean algorithm for GCD (O(log min(a,b)))
- LCM formula: `lcm(a,b) = (a × b) / gcd(a,b)`
- Modular arithmetic for instruction wrapping: `idx = (idx + 1) % len`
- Iterator fold for multi-way LCM: `.fold(1, |acc, x| lcm(acc, x))`

**Code Highlight**:
```rust
/// Greatest Common Divisor using Euclidean algorithm
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;  // Key: gcd(a,b) = gcd(b, a mod b)
        a = temp;
    }
    a
}

/// Least Common Multiple - finds cycle alignment
fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

/// Part 2: Find cycle length for each ghost, compute LCM
fn navigate_ghosts(&self) -> Result<usize> {
    let start_nodes = self.find_nodes_ending_with('A');
    
    // Find cycle length for each starting node
    let mut cycle_lengths = Vec::new();
    for start in &start_nodes {
        let steps = self.navigate_until_suffix(start, 'Z')?;
        cycle_lengths.push(steps);
    }
    
    // LCM of all cycle lengths = when all align
    let result = cycle_lengths.iter().fold(1, |acc, &x| lcm(acc, x));
    Ok(result)
}
```

**Mathematical Foundation**:
- **Graph Theory**: Network as directed graph with labeled edges
  - Each node has exactly 2 outgoing edges (L/R)
  - Path following with deterministic edge selection
- **Number Theory**: 
  - GCD via Euclidean algorithm (3000+ year old!)
  - LCM for cycle synchronization
  - Modular arithmetic for instruction wrapping
- **See**: `zettelkasten/math-foundations/graph-theory-fundamentals.md`
- **See**: `zettelkasten/math-foundations/number-theory-basics.md`

**Algorithm Details**:
- **Part 1**: Simple path following - O(n) where n = number of steps until ZZZ
- **Part 2**: 
  - Find all start nodes: O(V) where V = vertices
  - For each ghost, find cycle: O(k × m) where k = ghosts, m = avg cycle
  - Compute LCM: O(k log k) for k numbers
  - **Optimization**: 8,811,050,362,409 steps computed in ~6.7ms instead of years

**Tests**: 
- ✅ Part 1 example 1 (2 steps)
- ✅ Part 1 example 2 (6 steps)
- ✅ Part 2 ghost example (6 steps with 2 ghosts)
- ✅ GCD algorithm (48, 18 → 6)
- ✅ LCM calculation (4, 6 → 12)
- ✅ Network parsing

**Zettelkasten**: 
- [[graph-theory-fundamentals]] - Network representation, path traversal
- [[number-theory-basics]] - GCD, LCM, modular arithmetic, cycle alignment

**Links**: ← [Day 7](#day-7-camel-cards) | [Day 9](#day-9-mirage-maintenance) →

---

### Day 9: Mirage Maintenance

**Part 1**: Extrapolate next value for each OASIS sequence by computing finite differences → **1887980197**  
**Part 2**: Extrapolate previous (backward) value for each sequence → **990**  

**Algorithm**: Recursive finite differences - build difference pyramid until all zeros, then extrapolate bottom-up  
**Complexity**: O(k × m²) where k = sequences, m = avg sequence length (depth × width)  
**Runtime**: ~323µs (Part 1: 132µs, Part 2: 191µs)  
**Mission**: None  

**Key Insight**: Polynomial sequences have constant nth differences. Build a pyramid of differences recursively - when you hit all zeros, you can extrapolate. Forward extrapolation: `next = last + diff_next`. Backward extrapolation: `prev = first - diff_prev`. The algorithm works because polynomials of degree n have constant nth differences.

**Rust Highlights**:
- `windows(2)` iterator for pairwise differences: `sequence.windows(2).map(|pair| pair[1] - pair[0])`
- Recursive algorithm with base case: `if all_zeros(seq) { return 0; }`
- `last()` and `first()` for accessing endpoints
- Pattern matching in different directions: forward vs backward with same structure
- Zero allocations in core algorithm - Vec only for difference sequences

**Code Highlight**:
```rust
/// Compute differences: each element is next - current
fn compute_differences(sequence: &[i64]) -> Vec<i64> {
    sequence
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}

/// Extrapolate next value recursively
fn extrapolate_next(sequence: &[i64]) -> i64 {
    if all_zeros(sequence) {
        return 0;  // Base case: zeros extrapolate to zero
    }
    let differences = compute_differences(sequence);
    let diff_extrapolated = extrapolate_next(&differences);  // Recurse
    sequence.last().unwrap() + diff_extrapolated  // Build up from bottom
}

/// Extrapolate previous value (Part 2 - same structure, different direction)
fn extrapolate_prev(sequence: &[i64]) -> i64 {
    if all_zeros(sequence) {
        return 0;
    }
    let differences = compute_differences(sequence);
    let diff_extrapolated = extrapolate_prev(&differences);
    sequence.first().unwrap() - diff_extrapolated  // Subtract instead of add
}
```

**Example Walkthrough**:
```
Sequence: 0 3 6 9 12 15
Differences pyramid:
  0   3   6   9  12  15    ← extrapolate 18 (15 + 3)
    3   3   3   3   3      ← extrapolate 3 (3 + 0)
      0   0   0   0        ← all zeros, return 0

Backward extrapolation:
 -3   0   3   6   9  12  15    ← extrapolate -3 (0 - 3)
    3   3   3   3   3   3      ← extrapolate 3 (3 - 0)
      0   0   0   0   0        ← all zeros, return 0
```

**Mathematical Foundation**:
- **Finite Differences**: Method of polynomial interpolation/extrapolation from numerical analysis
  - Polynomial of degree n has constant nth differences
  - Sequence `0 3 6 9...` is degree 1 (linear) → 1st differences constant
  - Sequence `1 3 6 10...` is degree 2 (quadratic) → 2nd differences constant
  - Works by detecting polynomial patterns in data
- **Recursive Structure**: Each level reduces degree by 1 until reaching degree 0 (constants)
- **See**: `zettelkasten/math-foundations/finite-differences.md`

**Algorithm Complexity**:
- Depth of recursion: O(m) where m = polynomial degree (≤ sequence length)
- Work per level: O(n - level) to compute differences
- Total: O(m²) per sequence
- For 200 sequences of length ~21: ~323µs total

**Tests**: 
- ✅ Part 1 example (114 total)
- ✅ Part 2 example (2 total)
- ✅ Individual sequence extrapolations (18, 28, 68)
- ✅ Backward extrapolations (-3, 0, 5)
- ✅ Parse, differences, zero detection

**Zettelkasten**: 
- [[finite-differences]] - Polynomial extrapolation theory
- [[recursive-algorithms]] - Base case + recursive case pattern

**Links**: ← [Day 8](#day-8-haunted-wasteland) | [Day 10](#day-10-pipe-maze) →

---

### Day 10: Pipe Maze

**Part 1**: Find the farthest point in the pipe loop from start 'S' → **6733**  
**Part 2**: Count tiles enclosed within the loop → **435**  

**Algorithm**: BFS for loop traversal + ray casting for point-in-polygon  
**Complexity**: O(width × height) for both parts  
**Runtime**: ~6.5ms (Part 1: 3.1ms, Part 2: 3.4ms)  
**Mission**: Mission 6 (Grid<char>), Mission 8 (BFS pattern)  

**Key Insight**: Part 1 uses BFS to find the continuous pipe loop starting from 'S' - the farthest point is simply the maximum distance in the loop. Part 2 uses **ray casting** (computational geometry) - for each non-loop tile, cast a horizontal ray and count boundary crossings. Odd crossings = inside, even = outside. The tricky part is handling corner pieces correctly: `F---J` and `L---7` are real crossings (flip inside/outside), while `F---7` and `L---J` are not (stay same side).

**Rust Highlights**:
- **Mission Integration**: `Grid<char>` from Mission 6 for 2D maze storage, BFS pattern from Mission 8 for loop traversal
- `HashMap<Coord, usize>` for tracking loop tiles and distances
- Direction enum with `offset()` and `opposite()` methods for pipe navigation
- `pipe_connections(char) -> Vec<Dir>` mapping characters to connection points
- `determine_start_pipe()` to figure out what 'S' actually represents based on neighbors
- Scanline algorithm with state machine for ray casting
- Pattern matching on corner sequences for crossing detection

**Code Highlight**:
```rust
/// Map pipe characters to their connection directions
fn pipe_connections(ch: char) -> Vec<Dir> {
    match ch {
        '|' => vec![Dir::North, Dir::South],
        '-' => vec![Dir::East, Dir::West],
        'L' => vec![Dir::North, Dir::East],
        'J' => vec![Dir::North, Dir::West],
        '7' => vec![Dir::South, Dir::West],
        'F' => vec![Dir::South, Dir::East],
        _ => vec![],
    }
}

/// Ray casting for point-in-polygon (Part 2)
for x in 0..grid.width() {
    if loop_tiles.contains_key(&coord) {
        match ch {
            '|' => inside = !inside,  // Vertical crossing
            'F' | 'L' => enter_corner = Some(ch),  // Start corner
            '7' => {
                if enter_corner == Some('L') { inside = !inside; }  // L-7 crosses
                // F-7 doesn't cross
                enter_corner = None;
            }
            'J' => {
                if enter_corner == Some('F') { inside = !inside; }  // F-J crosses
                // L-J doesn't cross
                enter_corner = None;
            }
            '-' => {}  // Horizontal segment - no state change
            _ => {}
        }
    } else if inside {
        enclosed_count += 1;  // Non-loop tile inside the loop
    }
}
```

**Example Walkthrough (Part 2 - Ray Casting)**:
```
Row: .|..|.|..|.
     ^ loop tile '|' → flip inside (true)
       ^^ non-loop, inside → COUNT (2)
         ^ loop '|' → flip inside (false)
           ^ non-loop, outside → skip
             ^ loop '|' → flip inside (true)
               ^^ non-loop, inside → COUNT (2 more)
                 ^ loop '|' → flip inside (false)

Corner handling:
|F---7|  → Enter F, exit 7 (same side, no crossing)
|F---J|  → Enter F, exit J (opposite sides, CROSSING)
|L---7|  → Enter L, exit 7 (opposite sides, CROSSING)
|L---J|  → Enter L, exit J (same side, no crossing)
```

**Mathematical Foundation**:
- **Graph Theory**: 
  - Cycle detection in undirected graph (finding the continuous loop)
  - BFS for unweighted shortest paths (distances from start)
  - Connected components (loop vs disconnected pipes)
- **Computational Geometry**:
  - Point-in-polygon problem via **ray casting algorithm**
  - Scanline processing (row-by-row traversal)
  - Boundary crossing counting (odd = inside, even = outside)
  - Corner case handling for non-convex polygons
- **See**: `zettelkasten/math-foundations/graph-theory-fundamentals.md`, `zettelkasten/math-foundations/computational-geometry-basics.md`

**Algorithm Details**:
- **Part 1 - BFS Loop Traversal**:
  - Find 'S' by scanning grid: O(w × h)
  - BFS from 'S' following pipe connections: O(loop_size) ≤ O(w × h)
  - Track distances in HashMap: O(1) lookup/insert
  - Return max distance: O(loop_size)
- **Part 2 - Ray Casting**:
  - Reuse loop tiles from Part 1: O(1) membership check
  - Scanline: Visit every cell once: O(w × h)
  - State machine: O(1) per cell
  - Total: O(w × h)

**Tests**: 
- ✅ Part 1 simple loop (answer: 4)
- ✅ Part 1 complex loop (answer: 8)
- ✅ Part 2 small enclosure (answer: 4)
- ✅ Part 2 medium enclosure (answer: 8)
- ✅ Part 2 large with junk pipes (answer: 10)
- ✅ Pipe connection validation
- ✅ Grid parsing and coordinate navigation

**Zettelkasten**: 
- [[mission-6]] - Grid<T> 2D spatial data structures
- [[mission-8]] - BFS/DFS graph traversal patterns
- [[graph-theory-fundamentals]] - Cycle detection, BFS distances
- [[computational-geometry-basics]] - Point-in-polygon, ray casting
- [[aoc-grid-patterns]] - Common AoC grid problem patterns

**Links**: ← [Day 9](#day-9-mirage-maintenance) | [Day 11](#day-11-cosmic-expansion) →

---

### Day 11: Cosmic Expansion

**Part 1**: Sum shortest paths between all galaxy pairs after 2x expansion → **10276166**  
**Part 2**: Same but with 1,000,000x expansion factor → **598693078798**  

**Algorithm**: Parse galaxies, track empty rows/columns, calculate Manhattan distances with expansion offset  
**Complexity**: O(n×m + g²) where n×m is grid size, g is galaxy count (9 galaxies → 36 pairs)  
**Runtime**: ~728µs (Part 1: ~728µs, Part 2: ~728µs)  
**Mission**: None (simple Vec<Position> suffices)  

**Key Insight**: Don't expand the grid physically - just **track which rows/columns are empty** and add extra distance when calculating paths. For expansion factor `k`, each empty row/column crossed adds `k-1` extra units. This makes Part 1 (k=2) and Part 2 (k=1,000,000) identical code - just different expansion factors. The problem is essentially Manhattan distance on a non-uniform grid.

**Rust Highlights**:
- `flat_map` to find all galaxy positions in one pass: `(row, col)` tuples
- `HashSet` to identify occupied rows/columns: O(1) complement → empty rows
- Filtering ranges: `(0..=max_row).filter(|r| !occupied.contains(r))`
- Counting empty rows/columns between two points: `filter(|&&r| r > min && r < max).count()`
- Distance formula: `base_manhattan + (empty_count × (expansion - 1))`
- Nested loops for all pairs: `for i in 0..n { for j in (i+1)..n }` avoids duplicates
- Shared solver function for both parts with different expansion factors

**Code Highlight**:
```rust
fn parse_galaxies(input: &str) -> Vec<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(col, _)| (row, col))
        })
        .collect()
}

fn calculate_distance_with_expansion(
    pos1: Position,
    pos2: Position,
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_factor: usize,
) -> usize {
    let (r1, c1) = pos1;
    let (r2, c2) = pos2;
    
    let min_row = r1.min(r2);
    let max_row = r1.max(r2);
    let min_col = c1.min(c2);
    let max_col = c1.max(c2);
    
    // Count empty rows/cols between galaxies
    let empty_rows_between = empty_rows.iter()
        .filter(|&&r| r > min_row && r < max_row)
        .count();
    let empty_cols_between = empty_cols.iter()
        .filter(|&&c| c > min_col && c < max_col)
        .count();
    
    // Base Manhattan + expansion offset
    let base = (max_row - min_row) + (max_col - min_col);
    base + (empty_rows_between + empty_cols_between) * (expansion_factor - 1)
}
```

**Example Walkthrough**:
```
Original grid (10x10), 9 galaxies:
...#......
.......#..
#.........
..........  ← Empty row at index 3
......#...
.#........
.........#
..........  ← Empty row at index 7
.......#..
#...#.....
   ^^^  Empty columns at 2, 5, 8

Galaxy 5 at (5,1) to Galaxy 9 at (9,4):
- Base Manhattan: |9-5| + |4-1| = 4 + 3 = 7
- Empty rows crossed: row 7 is between 5 and 9 → count = 1
- Empty cols crossed: col 2 is between 1 and 4 → count = 1
- Part 1 (expansion=2): 7 + (1+1)×(2-1) = 7 + 2 = 9 ✓
- Part 2 (expansion=1M): 7 + (1+1)×(999,999) = 7 + 1,999,998 = 2,000,005
```

**Mathematical Foundation**:
- **Graph Theory**: Complete graph with g galaxies → g×(g-1)/2 edges (all pairs)
- **Metric Spaces**: Manhattan distance on non-uniform grid (variable edge weights)
- **Combinatorics**: All pairs counting - C(g, 2) = g!/(2!×(g-2)!) = 36 for g=9
- **See**: `zettelkasten/math-foundations/graph-theory-fundamentals.md`, `zettelkasten/math-foundations/combinatorics-basics.md`

**Algorithm Details**:
- **Parse**: O(n×m) to scan grid for '#' characters
- **Find empty rows/cols**: O(g) to build HashSet, O(n) and O(m) to filter ranges
- **Distance calculation**: O(empty_rows + empty_cols) per pair ≈ O(n+m) worst case
- **All pairs**: g²/2 pairs, each O(n+m) distance calc → O(g² × (n+m))
- **Total**: O(n×m + g² × (n+m))
- For typical AoC grids: 140×140 grid, ~450 galaxies → ~19,600×140×2 ≈ 5.5M operations (still fast)

**Tests**: 
- ✅ Part 1 example (374)
- ✅ Part 2 expansion 10x (1030)
- ✅ Part 2 expansion 100x (8410)
- ✅ Galaxy parsing (9 galaxies found)
- ✅ Empty row detection (rows 3, 7)
- ✅ Empty column detection (cols 2, 5, 8)

**Zettelkasten**: 
- [[manhattan-distance]] - Distance metrics on grids
- [[graph-theory-fundamentals]] - Complete graphs, all pairs
- [[combinatorics-basics]] - Combination counting

**Links**: ← [Day 10](#day-10-pipe-maze) | [Day 12](#day-12-hot-springs) →

---

### Day 12: Hot Springs

**Part 1**: Count valid arrangements of operational/damaged springs matching contiguous group constraints → **7251**  
**Part 2**: Unfold each row 5x (springs joined with `?`, groups repeated) → **2128386729962**  

**Algorithm**: Dynamic programming with memoization (recursive constraint satisfaction)  
**Complexity**: O(n × g × max_run) where n = spring length, g = group count, max_run = largest group size  
**Runtime**: 44.185ms (Part 1: 2.94ms, Part 2: 41.26ms)  
**Mission**: None (classic DP problem, memoization taught in Mission 11)  

**Key Insight**: Part 2's 5x unfolding creates exponential search space (trillions of combinations) but recursive DP with memoization prunes redundant paths. The state space is 3D: `(position, group_index, current_run_length)` - each unique state is computed once and cached.

**Rust Highlights**:
- **Three-parameter recursion**: `count_arrangements(pos, group_idx, current_run)` tracks state
- **HashMap memoization**: `(usize, usize, usize) → usize` caches subproblem results
- **`?` wildcard handling**: Try both `.` and `#`, recursively explore valid branches
- **Early validation**: Base case checks if all groups placed AND no incomplete runs
- **Shared solver**: Single function for both parts, only differs by input transformation

**Code Highlight**:
```rust
fn count_arrangements(
    springs: &[u8],
    groups: &[usize],
    pos: usize,
    group_idx: usize,
    current_run: usize,
    memo: &mut Memo,
) -> usize {
    // Base case: reached end of springs
    if pos == springs.len() {
        if group_idx == groups.len() && current_run == 0 {
            return 1; // All groups placed, no incomplete run
        }
        if group_idx == groups.len() - 1 && current_run == groups[group_idx] {
            return 1; // Last group completed exactly
        }
        return 0;
    }

    // Check memoization cache
    let key = (pos, group_idx, current_run);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut count = 0;
    let ch = springs[pos];

    // Try placing '.' (operational)
    if ch == b'.' || ch == b'?' {
        if current_run == 0 {
            count += count_arrangements(springs, groups, pos + 1, group_idx, 0, memo);
        } else if group_idx < groups.len() && current_run == groups[group_idx] {
            count += count_arrangements(springs, groups, pos + 1, group_idx + 1, 0, memo);
        }
    }

    // Try placing '#' (damaged)
    if (ch == b'#' || ch == b'?') && group_idx < groups.len() && current_run < groups[group_idx] {
        count += count_arrangements(springs, groups, pos + 1, group_idx, current_run + 1, memo);
    }

    memo.insert(key, count);
    count
}
```

**Example Walkthrough** (`.??..??...?##. 1,1,3`):

1. **Input**: springs = `.??..??...?##.`, groups = `[1,1,3]`
2. **Valid arrangements** (4 total):
   - `#.#..#.#...###.` - First `??` = `#.`, second `??` = `#.`
   - `#.#..#..#..###.` - First `??` = `#.`, second `??` = `.#`
   - `#..#.#.#...###.` - First `??` = `.#`, second `??` = `#.`
   - `#..#.#..#..###.` - First `??` = `.#`, second `??` = `.#`
3. **Why 4?** Last `?##.` must be `###` (only way to form group of 3). Each `??` can be `#.` or `.#` (exactly 1 damaged) → 2×2 = 4
4. **DP pruning**: Without memoization, would explore exponentially many invalid paths. Memoization remembers "from position 4 with group_idx=1, there are 2 ways" - computed once, reused.

**Mathematical Foundations**:

**Constraint Satisfaction Problem (CSP)**:
- Variables: Each `?` position → assignment to `.` or `#`
- Constraints: 
  1. Damaged groups must match sizes in `groups` array exactly
  2. Groups separated by ≥1 operational spring
  3. No partial groups (each # must belong to a complete group)
- Solution count: Number of valid assignments

**Dynamic Programming State Space**:
- **State**: `(pos, group_idx, current_run)` - 3D table
  - `pos`: Current position in springs (0..n)
  - `group_idx`: Which damaged group we're placing (0..g)
  - `current_run`: Length of current contiguous '#' run (0..max_run)
- **Transitions**:
  - Place `.`: Complete current group (if size matches) or continue if no active run
  - Place `#`: Extend current run if within group size limit
- **Memoization**: Cache `state → count` to avoid recomputing overlapping subproblems

**Complexity Analysis**:
- **Without memoization**: O(2^q) where q = number of `?` wildcards (exponential)
- **With memoization**: O(n × g × max_run) - each unique state computed once
  - States: n positions × g groups × max_run lengths ≈ 100 × 10 × 20 = 20,000 states
  - Per state: O(1) HashMap lookup + O(1) recursion calls
  - Part 2: 5x longer input → 5n × 5g × max_run ≈ 500,000 states (explains 14x slower runtime)

**Part 1 vs Part 2 Scaling**:
- Part 1: ~100 chars, ~6 groups → ~10,000 states
- Part 2: ~500 chars, ~30 groups → ~300,000 states (30x more)
- Runtime: Part 2 is 14x slower (41ms vs 2.9ms) - sublinear due to memoization efficiency!

**Tests**: 
- ✅ Part 1 example (21 total arrangements)
- ✅ Individual lines:
  - `???.### 1,1,3` → 1 arrangement
  - `.??..??...?##. 1,1,3` → 4 arrangements
  - `?#?#?#?#?#?#?#? 1,3,1,6` → 1 arrangement (fully constrained)
  - `????.#...#... 4,1,1` → 1 arrangement
  - `????.######..#####. 1,6,5` → 4 arrangements
  - `?###???????? 3,2,1` → 10 arrangements
- ✅ Part 2 first line (1 arrangement after 5x unfold)
- ✅ Unfold correctness: `.#` → `.#?.#?.#?.#?.#`, `[1]` → `[1,1,1,1,1]`

**Zettelkasten**: 
- [[memoization-comprehensive-guide]] - Top-down DP with caching
- [[tabulation-patterns]] - Bottom-up DP (alternative approach)
- [[constraint-satisfaction]] - CSP formulation and solving

**Links**: ← [Day 11](#day-11-cosmic-expansion) | [Day 13](#day-13-point-of-incidence) →

---

### Day 13: Point of Incidence

**Part 1**: Find mirror reflection lines with perfect symmetry in 2D ash/rock patterns → **33780**  
**Part 2**: Find reflection lines with exactly 1 "smudge" (character mismatch) → **23479**  

**Algorithm**: Hamming distance (discrete metric) for mismatch counting  
**Complexity**: O(r × c × n) where r×c = pattern dimensions, n = number of patterns  
**Runtime**: 354µs (Part 1: 169µs, Part 2: 187µs)  
**Mission**: None (Grid-like 2D analysis, could leverage Mission 6 for storage)  

**Key Insight**: Generalize from boolean "exact match" to integer "mismatch count". Part 1 requires Hamming distance = 0 (perfect reflection), Part 2 requires Hamming distance = 1 (exactly one smudge). **Same algorithm, different target value.**

**Rust Highlights**:
- **Unified algorithm**: `find_reflection(pattern, target_mismatches: usize)` handles both parts
- **Iterator zip for comparison**: `.iter().zip()` for element-wise mismatch counting
- **Expand-and-validate pattern**: Test each candidate reflection line, expand outward comparing pairs
- **Early termination**: No need to check remaining lines once reflection found (unique per problem)
- **Zero allocations**: Pattern stored as `Vec<Vec<char>>`, slices for row/column access

**Code Highlight**:
```rust
/// Count Hamming distance (total mismatches) across all reflected pairs
fn count_horizontal_mismatches(pattern: &[Vec<char>], above_idx: usize) -> usize {
    let mut mismatches = 0;
    let mut distance = 0;
    
    loop {
        let upper = above_idx.checked_sub(distance);
        let lower = above_idx + 1 + distance;
        
        // Out of bounds = all pairs validated
        if upper.is_none() || lower >= pattern.len() {
            return mismatches;
        }
        
        // Count mismatches between reflected rows (Hamming distance)
        let upper_row = upper.unwrap();
        for (a, b) in pattern[upper_row].iter().zip(pattern[lower].iter()) {
            if a != b {
                mismatches += 1;
            }
        }
        
        distance += 1;
    }
}

// Find reflection line with exact mismatch count
fn find_horizontal_reflection(pattern: &[Vec<char>], target_smudges: usize) -> Option<usize> {
    for i in 0..pattern.len() - 1 {
        if count_horizontal_mismatches(pattern, i) == target_smudges {
            return Some(i + 1); // Rows above reflection line
        }
    }
    None
}
```

**Example Walkthrough** (Part 2, first pattern):

```
Original:              After fixing smudge:
#.##..##.             ..##..##.  ← Top-left # → .
..#.##.#.             ..#.##.#.
##......#             ##......#  ← Reflects with row 4
##......#             ##......#  ← Reflects with row 3
..#.##.#.             ..#.##.#.
..##..##.             ..##..##.
#.#.##.#.             #.#.##.#.
```

1. **Part 1**: Vertical reflection between columns 5-6 (Hamming distance = 0)
2. **Part 2**: Horizontal reflection between rows 3-4 (Hamming distance = 1)
   - Scanning all candidate lines, rows 3-4 have exactly 1 mismatch (top-left character)
   - This is the "smudge" - fixing it creates new reflection line
   - Score: 3 rows above × 100 = 300

**Mathematical Foundations**:

**Hamming Distance** (see `zettelkasten/math-foundations/hamming-distance-discrete-metrics.md`):

$$d_H(s, t) = \sum_{i=1}^{n} \mathbb{1}_{s_i \neq t_i}$$

Count of positions where corresponding symbols differ.

**Reflection Symmetry**:
- Pattern mirrors across line L iff all pairs equidistant from L are identical
- For horizontal line between rows r and r+1:
  - Pair (r-k, r+1+k) must match for all valid k
  - Valid k: both rows in bounds
- **Generalized**: Pair matches iff Hamming distance = 0

**Part 1 vs Part 2**:
- **Part 1**: Total Hamming distance across all pairs = 0 (perfect symmetry)
- **Part 2**: Total Hamming distance across all pairs = 1 (one smudge)
- **Key**: Accumulate mismatches across ALL pairs, not just individual pair checks

**Complexity Analysis**:
- **Per pattern**: O(r × c)
  - Try r-1 horizontal lines: O(r)
  - Per line: Compare all pairs, each pair is O(c) characters → O(r × c)
  - Try c-1 vertical lines: Similarly O(r × c)
  - Total: O(r × c)
- **All patterns**: O(n × r × c) where n ≈ 100 patterns
- **Actual**: ~169µs for Part 1, ~187µs for Part 2 (Part 2 slightly slower due to continued search after finding target=0)

**Why Part 2 is Faster Than Expected**:
Part 2 doesn't try all reflection lines - it stops at first line with Hamming distance = 1. Most patterns find smudge reflection quickly.

**Tests**: 
- ✅ Part 1 example: First pattern vertical (5 cols), second horizontal (4 rows) → 405
- ✅ Part 2 example: First pattern horizontal (3 rows), second horizontal (1 row) → 400
- ✅ Parsing: 2 patterns, correct dimensions
- ✅ Individual reflection detection
- ✅ Smudge reflection detection

**Zettelkasten**: 
- [[hamming-distance-discrete-metrics]] - Mathematical foundation
- [[reflection-symmetry]] - Geometric concept (TODO)
- [[pattern-matching-techniques]] - Mismatch counting pattern

**Links**: ← [Day 12](#day-12-hot-springs) | [Day 14](#day-14-parabolic-reflector-dish) →

---

### Day 14: Parabolic Reflector Dish

**Part 1**: Tilt platform north, rocks roll to edges, calculate total load → **109596**  
**Part 2**: Perform 1 billion spin cycles (north-west-south-east), calculate load → **96105**  

**Algorithm**: Grid simulation with cycle detection via state hashing + modulo fast-forward  
**Complexity**: O(states) where states ≈ 100-200 for typical input (cycle detection avoids billion iterations)  
**Runtime**: 13.2ms (Part 1: 42.3µs, Part 2: 12.7ms)  
**Mission**: Mission 6 (Grid<T> for 2D platform)  

**Key Insight**: Part 2's 1 billion iterations are intractable by simulation, but **Pigeonhole Principle** guarantees the finite state space must cycle. Detect cycle with HashMap state tracking, then fast-forward using modulo arithmetic: `final_state = states[(1B - cycle_start) % cycle_length]`. This reduces billion iterations to ~100-200 actual simulations.

**Rust Highlights**:
- **Mission 6 Grid<T>**: Reused for platform storage, bounds checking, and coordinate navigation
- **State serialization**: Convert `Grid<char>` to `String` for HashMap keys
- **Cycle detection**: HashMap<String, usize> tracks `state → first_occurrence_index`
- **Four directional tilts**: Separate functions for north/west/south/east (different iteration orders)
- **Spin cycle**: `north() → west() → south() → east()` composition
- **Modulo fast-forward**: `(target - cycle_start) % cycle_length` to find equivalent state

**Code Highlight**:
```rust
/// Tilt platform north - rocks roll upward
fn tilt_north(grid: &mut Grid<char>) {
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if grid.get(x, y) == Some(&'O') {
                // Find landing position
                let mut new_y = y;
                while new_y > 0 && grid.get(x, new_y - 1) == Some(&'.') {
                    new_y -= 1;
                }
                // Move rock to landing position
                if new_y != y {
                    *grid.get_mut(x, new_y).unwrap() = 'O';
                    *grid.get_mut(x, y).unwrap() = '.';
                }
            }
        }
    }
}

/// Solve Part 2 using cycle detection
pub fn solve_part2(input: &str) -> Result<String> {
    let mut grid = parse_input(input)?;
    let mut seen: HashMap<String, usize> = HashMap::new();
    let target_cycles = 1_000_000_000;
    
    for cycle in 0..target_cycles {
        let state = grid_to_string(&grid);
        
        if let Some(&first_seen) = seen.get(&state) {
            // Cycle detected! Fast-forward using modulo
            let cycle_length = cycle - first_seen;
            let remaining = target_cycles - cycle;
            let final_offset = remaining % cycle_length;
            
            // Simulate only the remaining offset
            for _ in 0..final_offset {
                spin_cycle(&mut grid);
            }
            
            return Ok(calculate_load(&grid).to_string());
        }
        
        seen.insert(state, cycle);
        spin_cycle(&mut grid);
    }
    
    Ok(calculate_load(&grid).to_string())
}

/// One spin cycle: North → West → South → East
fn spin_cycle(grid: &mut Grid<char>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}
```

**Example Walkthrough** (Part 2 cycle detection):

```
Input grid state at iteration 0:
O....#....
O.OO#....#
.....##...
OO.#O....O
...

Simulate cycles, tracking states in HashMap:
Cycle   0: state_0 → see n.insert(state_0, 0)
Cycle   1: state_1 → seen.insert(state_1, 1)
...
Cycle   3: state_3 → seen.insert(state_3, 3)
Cycle   4: state_4 → seen.insert(state_4, 4)
...
Cycle  10: state_3 → MATCH! state_3 was first seen at cycle 3

Cycle detected:
- Cycle starts at index 3
- Cycle length = 10 - 3 = 7
- States repeat every 7 cycles: [3,4,5,6,7,8,9] → [10,11,12,...]

Fast-forward to 1,000,000,000:
- Remaining from cycle 10: 1B - 10 = 999,999,990
- Position in cycle: 999,999,990 % 7 = 5
- Final state = state at index (3 + 5) = state_8

Instead of 1 billion simulations, run only:
- Initial 10 cycles to detect pattern
- Final 5 cycles to reach exact state
- Total: 15 simulations vs 1,000,000,000!
```

**Mathematical Foundations**:

**Pigeonhole Principle** (see `zettelkasten/math-foundations/pigeonhole-principle-cycle-detection.md`):

> If $n$ items are placed into $m$ containers where $n > m$, then at least one container must contain more than one item.

Applied to this problem:
- **Items**: Iteration indices (0, 1, 2, ..., 1,000,000,000)
- **Containers**: Possible grid configurations (finite set)
- **Grid constraints**: 100×100 grid, each cell one of {'.', 'O', '#'} → at most $3^{10,000}$ states
- **Deterministic process**: Same state always transitions to same next state
- **Conclusion**: Within $3^{10,000}$ iterations, a state **must** repeat (actually happens within ~100-200)

**Cycle Detection Algorithm**:
1. **Tracking**: HashMap<State, FirstOccurrence> - O(1) lookup/insert
2. **Detection**: When `seen.contains_key(current_state)`, cycle found
3. **Parameters**: 
   - $\mu$ = cycle start index (rho-length)
   - $\lambda$ = cycle length
4. **Fast-forward**: Use modulo to find equivalent position within cycle

**Modular Arithmetic Fast-Forward**:
$$\text{final\_state} = \text{states}[(\text{target} - \mu) \mod \lambda + \mu]$$

Where:
- target = 1,000,000,000 (desired iteration)
- $\mu$ = first occurrence of repeated state
- $\lambda$ = cycle length

**Complexity Analysis**:
- **Without cycle detection**: O(1B × grid_size) - completely intractable
- **With cycle detection**:
  - Expected cycle length: ~√(state_space) by Birthday Paradox
  - Actual observed: 100-200 iterations to cycle detection
  - Fast-forward: O(offset) where offset < cycle_length
  - Total: O(states) ≈ **99.99999% reduction** in work

**Part 1 vs Part 2 Performance**:
- **Part 1**: Single north tilt, 100×100 grid → 42.3µs
- **Part 2**: ~150 spin cycles (4 tilts each) + HashMap ops → 12.7ms
  - Actual work: 150 cycles × 4 tilts × 10,000 cells = ~6M cell checks
  - HashMap overhead: ~150 insertions/lookups of 10KB strings
  - **300× slower** than Part 1, but still instant vs impossible brute force

**Tests**:
- ✅ Part 1 example (136)
- ✅ Part 2 example after 1 cycle
- ✅ Part 2 example after 2 cycles
- ✅ Part 2 example after 3 cycles
- ✅ Part 2 example after 1 billion cycles (64)
- ✅ Tilt north correctness
- ✅ Spin cycle sequencing (N→W→S→E)

**Zettelkasten**:
- [[mission-6]] - Grid<T> 2D spatial data structures
- [[pigeonhole-principle-cycle-detection]] - Mathematical foundation
- [[state-hashing-pattern]] - Serialization for cycle detection
- [[modulo-fast-forward]] - Arithmetic optimization technique

**Links**: ← [Day 13](#day-13-point-of-incidence) | [Day 15](#day-15-lens-library) →

---

### Day 15: Lens Library

**Part 1**: Run HASH algorithm on comma-separated steps, sum results → **517965**  
**Part 2**: Execute HASHMAP procedure - manage 256 lens boxes with add/remove operations → **267372**  

**Algorithm**: Part 1: Simple hash function with modular arithmetic; Part 2: HashMap simulation with labeled data (lenses)  
**Complexity**: O(n × m) where n = steps, m = avg label length  
**Runtime**: 539.88µs (Part 1: 207.48µs, Part 2: 332.48µs)  
**Mission**: None (hashmap concepts similar to Mission 5, but custom simulation)  

**Key Insight**: Part 1 demonstrates simple but effective hash function: `((value + ascii) × 17) % 256` creates decent distribution across 256 buckets. Part 2 uses this hash to simulate a physical system of labeled lenses in boxes - operations are deterministic HashMap manipulations (add, replace, remove) with position-based indexing. Focusing power calculation demonstrates how to aggregate positional data: (box+1) × (slot+1) × focal_length.

**Rust Highlights**:
- **Modular arithmetic hash**: `value = ((value + ascii) * 17) % 256` - simple but effective
- **Vec of Vecs**: `Vec<Vec<(String, u32)>>` for 256 boxes each containing ordered lenses
- **`.find()` and `.position()`**: Search for existing lens labels in box
- **`.retain()`**: Filter lens list in-place for removals
- **`.enumerate()`**: Track slot positions while iterating for power calculation
- **String slicing**: `&step[..pos]` to extract label from "label=N" or "label-"

**Code Highlight**:
```rust
/// HASH algorithm: Simple but effective hash function
fn hash(s: &str) -> usize {
    let mut value = 0;
    for ch in s.chars() {
        value += ch as usize;  // Add ASCII code
        value *= 17;           // Multiply by 17
        value %= 256;          // Keep in range 0-255
    }
    value
}

/// Part 2: HASHMAP simulation with 256 boxes
pub fn solve_part2(input: &str) -> Result<String> {
    let steps = parse_input(input);
    
    // 256 boxes, each containing vector of (label, focal_length) pairs
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    
    for step in steps {
        if let Some(pos) = step.find('=') {
            // Add/replace operation: "label=focal_length"
            let label = &step[..pos];
            let focal_length: u32 = step[pos+1..].parse()?;
            let box_num = hash(label) as usize;
            
            // Search for existing lens with this label
            if let Some(idx) = boxes[box_num].iter().position(|(l, _)| l == label) {
                boxes[box_num][idx].1 = focal_length;  // Replace
            } else {
                boxes[box_num].push((label.to_string(), focal_length));  // Add
            }
        } else if let Some(pos) = step.find('-') {
            // Remove operation: "label-"
            let label = &step[..pos];
            let box_num = hash(label) as usize;
            boxes[box_num].retain(|(l, _)| l != label);  // Remove if present
        }
    }
    
    // Calculate focusing power: sum of (box+1) × (slot+1) × focal_length
    let mut total_power = 0;
    for (box_num, lenses) in boxes.iter().enumerate() {
        for (slot, (_, focal_length)) in lenses.iter().enumerate() {
            let power = (box_num + 1) * (slot + 1) * (*focal_length as usize);
            total_power += power;
        }
    }
    
    Ok(total_power.to_string())
}
```

**Example Walkthrough** (Part 2 - HASHMAP operations):

```
Input: rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7

Step-by-step box state:

After "rn=1":
  Box 0: [rn 1]   ← hash("rn") = 0, add lens

After "cm-":
  Box 0: [rn 1]   ← hash("cm") = 0, but cm not in box (no-op)

After "qp=3":
  Box 0: [rn 1]
  Box 1: [qp 3]   ← hash("qp") = 1, add lens

After "cm=2":
  Box 0: [rn 1] [cm 2]   ← hash("cm") = 0, add to back

After "qp-":
  Box 0: [rn 1] [cm 2]   ← hash("qp") = 1, remove qp from box 1
  Box 1: []

After "pc=4":
  Box 0: [rn 1] [cm 2]
  Box 3: [pc 4]   ← hash("pc") = 3, add lens

... (continuing through all steps)

Final state:
  Box 0: [rn 1] [cm 2]
  Box 3: [ot 7] [ab 5] [pc 6]

Focusing power calculation:
  rn: (0+1) × (0+1) × 1 = 1
  cm: (0+1) × (1+1) × 2 = 4
  ot: (3+1) × (0+1) × 7 = 28
  ab: (3+1) × (1+1) × 5 = 40
  pc: (3+1) × (2+1) × 6 = 72
  Total: 1 + 4 + 28 + 40 + 72 = 145 ✓
```

**Mathematical Foundations**:

**Hash Function Design**:
- **Goal**: Map strings uniformly to integers 0-255
- **HASH algorithm steps**:
  1. Initialize: value = 0
  2. For each character: value += ASCII, value ×= 17, value %= 256
  3. Return: value
- **Properties**:
  - Deterministic: Same input → same output
  - Bounded: Result always in [0, 255]
  - Non-cryptographic: Fast, but not secure (collision-resistant)
  - Avalanche effect: Small input change → different output (usually)

**Why multiply by 17?**
- **Prime number**: Reduces collision patterns (vs even multipliers)
- **Small but not too small**: 17 creates good mixing without overflow before mod
- **Power-of-2 modulus (256)**: Fast bitwise AND instead of expensive division

**HashMap Simulation vs Real HashMaps**:
- **Real HashMap (Rust std::collections::HashMap)**:
  - Uses sophisticated hash function (SipHash by default)
  - Handles collisions with probing or chaining
  - Dynamic resizing for load factor management
  - O(1) average lookup/insert

- **This HASHMAP (Part 2)**:
  - Uses custom HASH function (ASCII-based)
  - Collision handling: Multiple lenses per box (chaining with Vec)
  - Fixed size: Always 256 boxes (no resizing)
  - Ordered within box: Vec preserves insertion order for labeled data

**Complexity Analysis**:
- **Part 1**: 
  - Parse: O(n) split + filter
  - Hash each step: O(m) per step where m = label length
  - Total: O(n × m) for n steps
  - Actual: ~4000 steps × ~5 chars → ~20K character operations → 207µs

- **Part 2**:
  - Parse: O(n)
  - Process each step: O(n × m) where m = avg operations per step
    - Hash label: O(m) for label length
    - Find existing lens: O(k) for k lenses in box (usually < 10)
    - Add/replace/remove: O(1) or O(k) for retain
  - Calculate power: O(n × avg_box_size) ≈ O(n)
  - Total: O(n × m + n × k) where k is small
  - Actual: ~4000 steps, avg 2-3 lenses per box → 332µs

**Part 1 vs Part 2 Performance**:
- **Part 1**: 207µs - simple hash computation
- **Part 2**: 332µs - hash + box operations + power calculation
- **1.6× slower**: Reasonable given extra work (box management, position tracking)

**Tests**:
- ✅ Part 1 example (1320)
- ✅ Part 2 example (145)
- ✅ HASH algorithm: hash("HASH") = 52
- ✅ Individual step hashes (11 examples validated)
- ✅ HASHMAP operations (add, replace, remove)
- ✅ Input parsing with newline filtering

**Zettelkasten**:
- [[hash-functions-fundamentals]] - Hash function design principles (TODO)
- [[modular-arithmetic]] - Modulo operations
- [[labeled-data-structures]] - Ordered collections with labels (TODO)
- [[simulation-patterns]] - Procedural simulation techniques (TODO)

**Links**: ← [Day 14](#day-14-parabolic-reflector-dish) | [Day 16](#day-16-the-floor-will-be-lava) →

---

### Day 16: The Floor Will Be Lava

**Part 1**: Trace light beam through grid of mirrors/splitters from top-left moving right, count energized tiles → **7434**  
**Part 2**: Find optimal starting position (any edge, any direction) that maximizes energized tiles → **8183**  

**Algorithm**: State-space BFS beam tracing with cycle detection using HashSet<(position, direction)>  
**Complexity**: O(rows × cols × 4) for state space (4 directions per cell)  
**Runtime**: 23.08ms (Part 1: 1.00ms, Part 2: 22.08ms) - **Parallelized with Rayon (11.67× speedup)**  
**Mission**: None (could use Mission 6 Grid, but custom Tile enum more semantic)  

**Key Insight**: Light beams split when hitting perpendicular splitters (| or -), creating multiple simultaneous beams. State tracking must include BOTH position AND direction to detect cycles - position-only tracking fails because a cell can be visited from different directions with different results. Part 2 tests all edge positions (2×rows + 2×cols = 444 starting configurations). **Optimization**: Parallelized with Rayon's `.par_iter()` - each trace is independent, achieving 11.67× speedup (257ms → 22ms) on multi-core CPU.

**Rust Highlights**:
- **State tuple**: `HashSet<(Coord, Direction)>` for cycle detection (position alone insufficient!)
- **Direction enum**: Pattern matching with `offset()` method for coordinate deltas
- **Tile enum**: Semantic types over char grid - `Tile::MirrorForward` vs `'/'`
- **Match exhaustiveness**: Compiler ensures all mirror/splitter combinations handled
- **Rayon parallelization**: `.par_iter()` for embarrassingly parallel problem (444 independent traces)
- **Dual HashSets**: `seen_states` (position+direction) vs `energized` (position only)
- **Vec as stack**: `beams.pop()` for DFS-style exploration (BFS would work too)

**Code Highlight**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up, Down, Left, Right,
}

impl Direction {
    fn offset(&self) -> Coord {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,           // '.'
    MirrorForward,   // '/'
    MirrorBackward,  // '\\'
    SplitterVert,    // '|'
    SplitterHoriz,   // '-'
}

fn trace_beam(&self, start_pos: Coord, start_dir: Direction) -> usize {
    let mut seen_states = HashSet::new();  // (position, direction) for cycles
    let mut energized = HashSet::new();    // position only for counting
    let mut beams = vec![(start_pos, start_dir)];

    while let Some((pos, dir)) = beams.pop() {
        // Cycle detection: Have we been here moving this direction before?
        if !seen_states.insert((pos, dir)) {
            continue;  // Already processed this state
        }

        let (row, col) = pos;
        if row < 0 || row >= self.rows || col < 0 || col >= self.cols {
            continue;  // Out of bounds
        }

        energized.insert(pos);  // Mark tile as energized

        // Determine next direction(s) based on tile and current direction
        let next_dirs = match (self.get(row, col).unwrap(), dir) {
            // Forward mirror '/' - reflects 90 degrees
            (Tile::MirrorForward, Direction::Up) => vec![Direction::Right],
            (Tile::MirrorForward, Direction::Right) => vec![Direction::Up],
            (Tile::MirrorForward, Direction::Down) => vec![Direction::Left],
            (Tile::MirrorForward, Direction::Left) => vec![Direction::Down],

            // Backward mirror '\\' - reflects 90 degrees
            (Tile::MirrorBackward, Direction::Up) => vec![Direction::Left],
            (Tile::MirrorBackward, Direction::Left) => vec![Direction::Up],
            (Tile::MirrorBackward, Direction::Down) => vec![Direction::Right],
            (Tile::MirrorBackward, Direction::Right) => vec![Direction::Down],

            // Vertical splitter '|' - splits horizontal beams
            (Tile::SplitterVert, Direction::Up | Direction::Down) => vec![dir],
            (Tile::SplitterVert, Direction::Left | Direction::Right) => {
                vec![Direction::Up, Direction::Down]  // SPLIT!
            }

            // Horizontal splitter '-' - splits vertical beams
            (Tile::SplitterHoriz, Direction::Left | Direction::Right) => vec![dir],
            (Tile::SplitterHoriz, Direction::Up | Direction::Down) => {
                vec![Direction::Left, Direction::Right]  // SPLIT!
            }

            // Empty tile - beam continues straight
            (Tile::Empty, _) => vec![dir],
        };

        // Add new beam(s) to process
        for next_dir in next_dirs {
            let (dr, dc) = next_dir.offset();
            let next_pos = (row + dr, col + dc);
            beams.push((next_pos, next_dir));
        }
    }

    energized.len()
}
```

**Example Walkthrough** (simplified 4×4 grid):

```
Grid:
  ./\.
  |.-.
  ..|.
  .../

Beam starts top-left (0,0) moving Right:

Step 1: (0,0) Right → Empty '.' → Continue Right
  energized: {(0,0)}
  beams: [(0,1) Right]

Step 2: (0,1) Right → Mirror '/' → Reflect to Up (exits top)
  energized: {(0,0), (0,1)}
  beams: [(-1,1) Up] → Out of bounds, done this branch

Step 3: (0,2) wasn't reached yet, let's trace backwards...

[Actual tracing would show beam splitting at splitters,
 multiple beams active simultaneously, and cycle detection
 preventing infinite loops when beam returns to same position
 with same direction]

Final energized count for this grid would be calculated by
complete traversal of all beam paths until all terminate
(exit grid or hit cycle).
```

**Why State = (Position, Direction)?**

Consider this scenario:
```
  .→→.
  ↓  ↓
  .←←.
```

A cell can be visited from multiple directions:
- Entering from left: Beam continues right
- Entering from above: Beam continues down

If we only tracked position, we'd think we've "seen" the cell after first visit and skip the second beam direction. This breaks the simulation!

**State tracking must be**: `HashSet<(Coord, Direction)>`  
**NOT**: `HashSet<Coord>` ← This would incorrectly stop beams!

**Mathematical Foundations**:

**State Space Exploration**:
- **State**: Tuple (position, direction)
- **State space size**: rows × cols × 4 directions
- **For 111×110 grid**: 111 × 110 × 4 = 48,840 possible states
- **Actual states visited**: Typically 5,000-15,000 per trace (cycles prune search)

**Graph Theory View**:
- **Vertices**: Each (cell, direction) pair is a node
- **Edges**: Tile determines transitions (empty→1 edge, splitter→2 edges)
- **Traversal**: DFS with visited set (could also use BFS)
- **Cycle detection**: HashSet tracks visited nodes to prevent infinite loops

**Reflection Geometry**:
- **Forward mirror '/'**: Maps (dx,dy) → (-dy, -dx)
  - Up(-1,0) → Right(0,1) ✓
  - Right(0,1) → Up(-1,0) ✓
  - Down(1,0) → Left(0,-1) ✓
  - Left(0,-1) → Down(1,0) ✓
- **Backward mirror '\\'**: Maps (dx,dy) → (dy, dx)
  - Up(-1,0) → Left(0,-1) ✓
  - Left(0,-1) → Up(-1,0) ✓

**Part 2 Optimization Insights**:
- **Brute force**: Test all edge positions (2×111 + 2×110 = 442 positions × 1 direction each + 4 corners × 2 directions = 444 total)
- **Each trace**: ~0.58ms average (257ms ÷ 444)
- **No memoization needed**: State spaces don't overlap between different starting positions
- **Potential optimization**: Parallel execution (444 independent traces) - could achieve ~5-10x speedup

**Complexity Analysis**:
- **Part 1**:
  - Single beam trace: O(states) where states ≤ rows × cols × 4
  - Actual: ~1ms for 111×110 grid
- **Part 2**:
  - Edge positions: 2 × (rows + cols) ≈ 444
  - Each trace: O(states) as above
  - Total: O(edges × states) ≈ O(rows × cols × max_states_per_trace)
  - Actual: 444 traces × 0.58ms avg = 257ms

**Tests**:
- ✅ Part 1 example (46)
- ✅ Part 2 example (51)
- ✅ Mirror reflection logic (all 8 combinations)
- ✅ Splitter logic (4 pass-through + 4 split cases)
- ✅ Cycle detection (beam returns to same state)
- ✅ Bounds checking (beams exit grid properly)

**Zettelkasten**:
- [[graph-theory-fundamentals]] - State space as graph, DFS traversal
- [[set-theory-fundamentals]] - HashSet for membership testing (cycle detection)
- [[computational-geometry-basics]] - Reflection transformations, coordinate systems
- [[state-space-search]] - (TODO) State-based exploration patterns

**Links**: ← [Day 15](#day-15-lens-library) | [Day 17](#day-17-clumsy-crucible) →

---

### Day 17: Clumsy Crucible

**Part 1**: Find minimum heat loss path from top-left to bottom-right, max 3 consecutive blocks same direction → **1023**  
**Part 2**: Ultra crucible requires min 4, max 10 consecutive blocks same direction → **1165**  

**Algorithm**: Dijkstra's shortest path with extended state space (position + direction + consecutive_count)  
**Complexity**: O(V × D × C × log(V × D × C)) where V=cells, D=4 directions, C=max_consecutive  
**Runtime**: 246.7ms (Part 1: 64.3ms, Part 2: 182.4ms)  
**Mission**: Mission 6 (Grid<T> for heat loss map, Coord for type-safe positions)  

**Key Insight**: Standard Dijkstra works on cells, but movement constraints require **state-based Dijkstra** where state = (position, direction, consecutive_steps). Can't turn around, must respect min/max consecutive block limits. The state space explosion from O(V) to O(V × D × C) causes slower runtime vs traditional pathfinding.

**Rust Highlights**:
- **Extended state**: `State { pos: Coord, dir: Direction, consecutive: u8 }` for constraint tracking
- **BinaryHeap priority queue**: Standard Dijkstra pattern with `Reverse<Node>` for min-heap
- **Mission 6 integration**: `Grid<u8>` eliminates ~40 lines of manual 2D array handling
- **Direction enum**: Pattern matching for move validation (can't reverse, turn-only logic)
- **HashMap visited**: `visited: HashMap<State, usize>` tracks best cost per state
- **Constraint parameterization**: Single `find_min_heat_loss(grid, min_straight, max_straight)` handles both parts

**Code Highlight**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Coord,
    dir: Direction,
    consecutive: u8,  // How many blocks moved in current direction
}

fn find_min_heat_loss(grid: &Grid<u8>, min_straight: u8, max_straight: u8) -> usize {
    let start = Coord::new(0, 0);
    let goal = Coord::new(grid.height() - 1, grid.width() - 1);
    
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();
    
    // Start exploring both right and down (can't go straight from start)
    heap.push(Node {
        cost: 0,
        state: State { pos: start, dir: Direction::Right, consecutive: 0 },
    });
    heap.push(Node {
        cost: 0,
        state: State { pos: start, dir: Direction::Down, consecutive: 0 },
    });
    
    while let Some(Node { cost, state }) = heap.pop() {
        // Reached goal? Check if we met minimum straight requirement
        if state.pos == goal && state.consecutive >= min_straight {
            return cost;
        }
        
        // Skip if already visited with lower cost
        if let Some(&prev_cost) = visited.get(&state) {
            if prev_cost <= cost {
                continue;
            }
        }
        visited.insert(state, cost);
        
        // Generate next moves
        let next_dirs = if state.consecutive < min_straight {
            // Haven't met minimum - MUST continue straight
            vec![(state.dir, state.consecutive + 1)]
        } else if state.consecutive >= max_straight {
            // Hit maximum - MUST turn (can't continue straight)
            vec![
                (state.dir.turn_left(), 1),
                (state.dir.turn_right(), 1),
            ]
        } else {
            // Can either continue straight or turn
            vec![
                (state.dir, state.consecutive + 1),
                (state.dir.turn_left(), 1),
                (state.dir.turn_right(), 1),
            ]
        };
        
        // Explore neighbors
        for (next_dir, next_consecutive) in next_dirs {
            if let Some(next_pos) = move_in_direction(state.pos, next_dir, grid) {
                let heat_loss = *grid.get(next_pos).unwrap() as usize;
                let next_cost = cost + heat_loss;
                
                heap.push(Node {
                    cost: next_cost,
                    state: State {
                        pos: next_pos,
                        dir: next_dir,
                        consecutive: next_consecutive,
                    },
                });
            }
        }
    }
    
    unreachable!("No path found to goal")
}
```

**Why State-Based Dijkstra?**

Standard Dijkstra tracks `visited: HashSet<Coord>` - once a cell is visited, it's marked done. This breaks when movement constraints exist:

**Problem scenario**:
```
Reaching cell (5,5) moving Right with 2 consecutive blocks is DIFFERENT from
reaching (5,5) moving Down with 1 consecutive block.
```

First state might be stuck (already at max consecutive), while second state has flexibility to continue or turn. **Solution**: Visited set must track **full state** `(position, direction, consecutive)`, not just position.

**State Space Comparison**:
- **Standard Dijkstra**: O(V) states where V = cells (here: 141×141 = 19,881)
- **Part 1 (max 3)**: O(V × 4 × 3) = ~238,572 possible states
- **Part 2 (max 10)**: O(V × 4 × 10) = ~795,240 possible states

This explains why Part 2 is ~2.8× slower despite same grid size.

**Mathematical Foundations**:

**Graph Theory**:
- **Vertices**: Each state (position, direction, consecutive_count)
- **Edges**: Valid moves respecting constraints (no reverse, min/max straight limits)
- **Edge weights**: Heat loss values from grid cells
- **Shortest path**: Dijkstra's algorithm with priority queue

**Algorithmic Complexity**:
- **Dijkstra**: O(E + V log V) with binary heap
- **Our E (edges)**: Each state has ≤3 outgoing edges (straight, left, right)
- **Our V (vertices)**: V_cells × 4 directions × max_consecutive
- **Actual**: O((V × D × C) × log(V × D × C)) where V=cells, D=4, C=max_consecutive
- **Part 1**: C=3 → ~238k states
- **Part 2**: C=10 → ~795k states (explains 2.8× slowdown)

**Optimization Opportunities** (not implemented - prioritizing clarity):
1. **3D visited array**: `bool[141][141][4]` with bitflags for consecutive counts (vs HashMap)
2. **A* heuristic**: Manhattan distance to goal for early pruning
3. **Bidirectional search**: Meet in the middle from start/goal

**Tests**:
- ✅ Part 1 example (102)
- ✅ Part 2 example (94)
- ✅ Part 2 simple grid (71) - forces long straight segments
- ✅ Grid parsing (dimensions, corner values)
- ✅ Minimal 2×2 grid (10)
- ✅ Single path test (30)
- ✅ All zeros edge case (0)

**Zettelkasten**:
- [[dijkstra-algorithm]] - Shortest path with priority queue
- [[graph-theory-fundamentals]] - State-space graphs, weighted edges
- [[state-space-search]] - Extended states for constraint satisfaction

**Links**: ← [Day 16](#day-16-the-floor-will-be-lava) | Day 18 →

---

<!-- Template for future days:

### Day XX: [Problem Title]

**Part 1**: [One sentence description] → **[Answer]**  
**Part 2**: [One sentence description] → **[Answer]**  

**Algorithm**: [Name/approach]  
**Complexity**: O(...)  
**Runtime**: X.Xms  
**Mission**: [Mission name or "None"]  

**Key Insight**: [1-2 sentence main learning]  

**Rust Highlights**:
- [Pattern 1]
- [Pattern 2]

**Code Highlight**:
```rust
// Critical section only (10-20 lines)
```

**Tests**: 
- ✅ Example test
- ✅ Edge cases

**Zettelkasten**: [[link]] or None

**Links**: ← [[dayXX-1]] | [[dayXX+1]] →

-->
