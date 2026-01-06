# AoC 2023 - Summary

## 📊 Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 5/25 ⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐ |
| **Total Runtime** | 1.899ms |
| **Mission Integration** | 2 days (Day 3: Mission 6, Day 4: Mission 5) |
| **Patterns Extracted** | 5 (delimiter parsing, spatial indexing, HashSet membership, forward-propagation DP, range intersection) |

---

## 🔍 Quick Navigation

[Day 1](#day-1-trebuchet) | [Day 2](#day-2-cube-conundrum) | [Day 3](#day-3-gear-ratios) | [Day 4](#day-4-scratchcards) | [Day 5](#day-5-if-you-give-a-seed-a-fertilizer) | Day 6 | Day 7 | Day 8 | Day 9 | Day 10 |
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
**Runtime**: 526.7µs (Part 1: 50.0µs, Part 2: 476.7µs)  
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

**Links**: ← [Day 4](#day-4-scratchcards) | Day 6 →

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
