# AoC 2023 - Summary

## 📊 Stats Dashboard

| Metric | Value |
|--------|-------|\n| **Progress** | 10/25 ⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐ |
| **Total Runtime** | 9.472ms |
| **Mission Integration** | 3 days (Day 3: Mission 6, Day 4: Mission 5, Day 10: Mission 6 + Mission 8) |
| **Patterns Extracted** | 8 (delimiter parsing, spatial indexing, HashSet membership, forward-propagation DP, range intersection, recursive differences, BFS loop traversal, ray casting point-in-polygon) |

---

## 🔍 Quick Navigation

[Day 1](#day-1-trebuchet) | [Day 2](#day-2-cube-conundrum) | [Day 3](#day-3-gear-ratios) | [Day 4](#day-4-scratchcards) | [Day 5](#day-5-if-you-give-a-seed-a-fertilizer) | [Day 6](#day-6-wait-for-it) | [Day 7](#day-7-camel-cards) | [Day 8](#day-8-haunted-wasteland) | [Day 9](#day-9-mirage-maintenance) | [Day 10](#day-10-pipe-maze) |
Day 11 | Day 12 | Day 13 | Day 14 | Day 15 | Day 16 | Day 17 | Day 18 | Day 19 | Day 20 |
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

**Links**: ← [Day 9](#day-9-mirage-maintenance) | Day 11 →

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
