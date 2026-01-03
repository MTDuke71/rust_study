# AoC 2023 - Summary

## 📊 Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 3/25 ⭐⭐⭐⭐⭐⭐ |
| **Total Runtime** | 716.9µs |
| **Mission Integration** | 1 day (Day 3: Mission 6) |
| **Patterns Extracted** | 2 (delimiter parsing, spatial indexing) |

---

## 🔍 Quick Navigation

[Day 1](#day-1-trebuchet) | [Day 2](#day-2-cube-conundrum) | [Day 3](#day-3-gear-ratios) | Day 4 | Day 5 | Day 6 | Day 7 | Day 8 | Day 9 | Day 10 |
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

**Links**: ← [Day 2](#day-2-cube-conundrum) | Day 4 →

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
