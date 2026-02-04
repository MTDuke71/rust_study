# Day 4: Camp Cleanup - Function Guide

**Quick Links**: [Problem Statement](day04.md) | [← Summary](../summary_2022.md) | [All Days](README.md) | [Code](../../../aoc2022/src/solver/day04.rs)

**Problem**: AoC 2022 Day 4 - Find overlapping section assignment ranges

**Solution**: `advent_of_code/aoc2022/src/solver/day04.rs`

**Performance**: Parse: 26.4µs | Combined: 27.7µs

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Type Definitions](#type-definitions)
3. [Core Implementation](#core-implementation)
4. [Algorithm Analysis](#algorithm-analysis)
5. [Public API](#public-api)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Testing Strategy](#testing-strategy)
9. [Common Pitfalls](#common-pitfalls)
10. [Key Takeaways](#key-takeaways)

---

## 🎯 Overview

### Problem Summary
Elves are assigned section ranges for camp cleanup. Find pairs where assignments overlap:
- **Part 1**: Count pairs where one range fully contains the other
- **Part 2**: Count pairs where ranges have any overlap at all

### Example
```
Input: 2-4,6-8
       2-8,3-7

Part 1: Full containment
2-8 fully contains 3-7 → Count this pair
(2 ≤ 3 AND 8 ≥ 7) ✓

Part 2: Any overlap
2-4,6-8 → No overlap (4 < 6)
5-7,7-9 → Overlap at 7
2-8,3-7 → Overlap at 3,4,5,6,7
```

### Key Insights
1. **Range containment**: One range is subset of another
2. **Range intersection**: Ranges share at least one element
3. **Efficient checking**: No need to enumerate elements - just compare endpoints
4. **Inclusive ranges**: Both start and end are included [start, end]
5. **Parse-once pattern**: Single parse supports both parts

### Complexity Analysis
- **Time**: O(n) where n = number of assignment pairs
  - Parsing: O(n) - single pass through input
  - Part 1: O(n) - check containment for each pair
  - Part 2: O(n) - check overlap for each pair
- **Space**: O(n) - store all pairs
- **Optimal**: Cannot do better than O(n) - must examine each pair

---

## 📦 Type Definitions

### `Range`

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}
```

**Purpose**: Represent an inclusive range of section IDs [start, end]

**Fields**:
- `start: u32` - First section ID in the range (inclusive)
- `end: u32` - Last section ID in the range (inclusive)

**Design Decision**: Why `u32` not `usize`?
1. **Explicit size**: Section IDs are always positive integers
2. **Copy semantics**: u32 is Copy (cheap to pass around)
3. **Sufficient range**: u32::MAX = 4,294,967,295 (more than enough for section IDs)

**Memory layout**: 8 bytes total (2 × u32)
- Compact: Can fit 125,000 ranges in 1MB
- Copy: No heap allocation, fast to clone

**Methods**:

#### `Range::new(start: u32, end: u32) -> Self`
Create a new range.

**Signature**:
```rust
pub fn new(start: u32, end: u32) -> Self
```

**Example**:
```rust
let range = Range::new(2, 8);
assert_eq!(range.start, 2);
assert_eq!(range.end, 8);
```

**Design Note**: No validation that start ≤ end. Input is trusted (from puzzle).

---

#### `Range::contains(&self, other: &Range) -> bool`
Check if this range fully contains another range.

**Signature**:
```rust
pub fn contains(&self, other: &Range) -> bool
```

**Algorithm**:
```
self contains other ⟺ 
    self.start ≤ other.start AND 
    self.end ≥ other.end

Visual:
self:  [--------]
other:   [----]   ✓ contained

self:  [-----]
other:   [------] ✗ not contained
```

**Implementation**:
```rust
pub fn contains(&self, other: &Range) -> bool {
    self.start <= other.start && self.end >= other.end
}
```

**Example**:
```rust
let r1 = Range::new(2, 8);
let r2 = Range::new(3, 7);
assert!(r1.contains(&r2));  // 2 ≤ 3 AND 8 ≥ 7 ✓
assert!(!r2.contains(&r1)); // 3 ≤ 2 ✗
```

**Complexity**: O(1) - two comparisons

**Edge Cases**:
- Self-containment: `r.contains(&r)` → `true` (range contains itself)
- Single element: `Range::new(5, 5).contains(&Range::new(5, 5))` → `true`

---

#### `Range::overlaps(&self, other: &Range) -> bool`
Check if ranges have any overlap (share at least one element).

**Signature**:
```rust
pub fn overlaps(&self, other: &Range) -> bool
```

**Algorithm**:
```
Ranges overlap ⟺ 
    self.start ≤ other.end AND 
    other.start ≤ self.end

Visual examples:
self:  [-----]
other:    [-----]  ✓ overlap

self:  [-----]
other:        [--] ✓ touch at boundary

self:  [--]
other:       [--]  ✗ gap between
```

**Mathematical Foundation**:
This is the **interval intersection** test from set theory. Two intervals overlap if and only if:
- The start of each interval is at or before the end of the other

**Implementation**:
```rust
pub fn overlaps(&self, other: &Range) -> bool {
    self.start <= other.end && other.start <= self.end
}
```

**Example**:
```rust
// No overlap (gap)
assert!(!Range::new(2, 4).overlaps(&Range::new(6, 8)));

// Overlap at boundary
assert!(Range::new(5, 7).overlaps(&Range::new(7, 9)));

// Full overlap
assert!(Range::new(2, 8).overlaps(&Range::new(3, 7)));
```

**Complexity**: O(1) - two comparisons

**Edge Cases**:
- Touching at boundary: `[2,4]` and `[4,6]` → `true` (share element 4)
- Identical ranges: `r.overlaps(&r)` → `true`
- Containment: If `r1.contains(&r2)` then `r1.overlaps(&r2)` (containment implies overlap)

**Design Note**: Overlap is symmetric: `a.overlaps(&b) ⟺ b.overlaps(&a)`

---

### `RangePair`

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangePair {
    pub first: Range,
    pub second: Range,
}
```

**Purpose**: Represent a pair of elf assignments to check for overlap

**Fields**:
- `first: Range` - First elf's section assignment
- `second: Range` - Second elf's section assignment

**Memory layout**: 16 bytes (2 × 8-byte Range)
- Compact: 1000 pairs = 16KB
- Copy: Entire struct can be copied efficiently

**Methods**:

#### `RangePair::has_full_containment(&self) -> bool`
Check if one range fully contains the other (Part 1).

**Signature**:
```rust
pub fn has_full_containment(&self) -> bool
```

**Algorithm**:
```
Pair has full containment ⟺ 
    first contains second OR 
    second contains first

Visual:
first:  [--------]
second:   [----]     ✓ first contains second

first:    [----]
second: [--------]   ✓ second contains first
```

**Implementation**:
```rust
pub fn has_full_containment(&self) -> bool {
    self.first.contains(&self.second) || 
    self.second.contains(&self.first)
}
```

**Example**:
```rust
let pair = RangePair {
    first: Range::new(2, 8),
    second: Range::new(3, 7),
};
assert!(pair.has_full_containment()); // 2-8 contains 3-7
```

**Complexity**: O(1) - two containment checks

---

#### `RangePair::has_overlap(&self) -> bool`
Check if ranges have any overlap (Part 2).

**Signature**:
```rust
pub fn has_overlap(&self) -> bool
```

**Algorithm**: Delegate to `Range::overlaps()`

**Implementation**:
```rust
pub fn has_overlap(&self) -> bool {
    self.first.overlaps(&self.second)
}
```

**Example**:
```rust
let pair = RangePair {
    first: Range::new(5, 7),
    second: Range::new(7, 9),
};
assert!(pair.has_overlap()); // Overlap at 7
```

**Complexity**: O(1)

**Note**: If `has_full_containment()` returns true, then `has_overlap()` also returns true (containment is a special case of overlap).

---

### `AssignmentPairs`

**Definition**:
```rust
#[derive(Debug, Clone)]
pub struct AssignmentPairs {
    pub pairs: Vec<RangePair>,
}
```

**Purpose**: Store all parsed assignment pairs from input

**Fields**:
- `pairs: Vec<RangePair>` - All range pairs to check

**Memory**: For 1000 pairs = 16KB (very compact)

---

## 🔧 Core Implementation

### Parsing Functions

#### `parse_range(s: &str) -> Option<Range>`
Parse a range string like "2-4" into a Range.

**Signature**:
```rust
fn parse_range(s: &str) -> Option<Range>
```

**Algorithm**:
```
Input: "2-4"
1. Split on '-' → ("2", "4")
2. Parse each to u32 → (2, 4)
3. Create Range::new(2, 4)
```

**Implementation**:
```rust
fn parse_range(s: &str) -> Option<Range> {
    let (start, end) = s.split_once('-')?;  // ❶
    Some(Range::new(
        start.parse().ok()?,                 // ❷
        end.parse().ok()?                    // ❸
    ))
}
```

**Key Points**:
1. ❶ `split_once('-')` - More efficient than `split()` when expecting exactly 2 parts
2. ❷ `parse().ok()?` - Convert parse error to `None` and propagate with `?`
3. ❸ Returns `None` if any step fails (missing '-', invalid number)

**Example**:
```rust
assert_eq!(parse_range("2-4"), Some(Range::new(2, 4)));
assert_eq!(parse_range("10-99"), Some(Range::new(10, 99)));
assert_eq!(parse_range("invalid"), None);
```

**Complexity**: O(m) where m = string length (parse two numbers)

**Error Handling**: Returns `None` instead of panicking on invalid input

---

#### `parse_line(line: &str) -> Option<RangePair>`
Parse a line like "2-4,6-8" into a RangePair.

**Signature**:
```rust
fn parse_line(line: &str) -> Option<RangePair>
```

**Algorithm**:
```
Input: "2-4,6-8"
1. Split on ',' → ("2-4", "6-8")
2. Parse each with parse_range()
3. Create RangePair
```

**Implementation**:
```rust
fn parse_line(line: &str) -> Option<RangePair> {
    let (first, second) = line.split_once(',')?;
    Some(RangePair {
        first: parse_range(first)?,
        second: parse_range(second)?,
    })
}
```

**Example**:
```rust
let pair = parse_line("2-4,6-8").unwrap();
assert_eq!(pair.first, Range::new(2, 4));
assert_eq!(pair.second, Range::new(6, 8));
```

**Complexity**: O(m) where m = line length

---

#### `parse_input(input: &str) -> AssignmentPairs`
Parse entire input into AssignmentPairs.

**Signature**:
```rust
pub fn parse_input(input: &str) -> AssignmentPairs
```

**Algorithm**:
```
Input: Multi-line string
For each line:
    1. Trim whitespace
    2. Parse with parse_line()
    3. Filter out None (invalid lines)
    4. Collect into Vec
```

**Implementation**:
```rust
pub fn parse_input(input: &str) -> AssignmentPairs {
    let pairs = input
        .lines()
        .filter_map(|line| parse_line(line.trim()))
        .collect();

    AssignmentPairs { pairs }
}
```

**Example**:
```rust
let input = "2-4,6-8\n2-3,4-5\n";
let data = parse_input(input);
assert_eq!(data.pairs.len(), 2);
```

**Complexity**: O(n × m) where n = lines, m = avg line length

**Error Handling**: Silently filters invalid lines (safe for AoC input)

---

### Solution Functions

#### `solve_part1_impl(data: &AssignmentPairs) -> usize`
Count pairs where one range fully contains the other.

**Signature**:
```rust
fn solve_part1_impl(data: &AssignmentPairs) -> usize
```

**Algorithm**:
```
For each pair:
    If has_full_containment():
        count += 1
Return count
```

**Implementation**:
```rust
fn solve_part1_impl(data: &AssignmentPairs) -> usize {
    data.pairs
        .iter()
        .filter(|pair| pair.has_full_containment())
        .count()
}
```

**Complexity**: O(n) - single pass through pairs

**Answer**: 448 pairs have full containment

---

#### `solve_part2_impl(data: &AssignmentPairs) -> usize`
Count pairs where ranges have any overlap.

**Signature**:
```rust
fn solve_part2_impl(data: &AssignmentPairs) -> usize
```

**Algorithm**:
```
For each pair:
    If has_overlap():
        count += 1
Return count
```

**Implementation**:
```rust
fn solve_part2_impl(data: &AssignmentPairs) -> usize {
    data.pairs
        .iter()
        .filter(|pair| pair.has_overlap())
        .count()
}
```

**Complexity**: O(n) - single pass through pairs

**Answer**: 794 pairs have any overlap

**Note**: Part 2 count ≥ Part 1 count (full containment implies overlap)

---

## 🧮 Algorithm Analysis

### Range Containment Algorithm

**Problem**: Given two ranges [a₁, a₂] and [b₁, b₂], does a contain b?

**Solution**: Check if `a₁ ≤ b₁ AND a₂ ≥ b₂`

**Mathematical Foundation**: Set theory subset relation
- Range [a, b] represents set {a, a+1, ..., b}
- Containment ⟺ subset relation: B ⊆ A

**Why This Works**:
```
If a₁ ≤ b₁ AND a₂ ≥ b₂:
    Then A starts before or at B's start
    And A ends after or at B's end
    Therefore A covers all of B's elements
```

**Visual Proof**:
```
Case 1: A contains B
A: [--------]
B:   [----]
   a₁≤b₁ ✓  a₂≥b₂ ✓

Case 2: A doesn't contain B
A: [-----]
B:   [------]
   a₁≤b₁ ✓  a₂≥b₂ ✗
```

**Complexity**: O(1) - two comparisons
**Correctness**: Proven by set theory subset definition

---

### Range Intersection Algorithm

**Problem**: Given two ranges [a₁, a₂] and [b₁, b₂], do they share any elements?

**Solution**: Check if `a₁ ≤ b₂ AND b₁ ≤ a₂`

**Mathematical Foundation**: Interval intersection from real analysis

**Why This Works**:
```
Ranges DON'T overlap when:
    a₂ < b₁  (A ends before B starts)
    OR
    b₂ < a₁  (B ends before A starts)

By De Morgan's law, ranges DO overlap when:
    NOT(a₂ < b₁ OR b₂ < a₁)
    = NOT(a₂ < b₁) AND NOT(b₂ < a₁)
    = a₂ ≥ b₁ AND b₂ ≥ a₁
    = a₁ ≤ b₂ AND b₁ ≤ a₂  (reordered)
```

**Visual Proof**:
```
Overlap cases:
A: [-----]        A:  [-----]      A: [--------]
B:    [-----]     B:    [--]       B:   [----]
   a₁≤b₂ ✓           a₁≤b₂ ✓           a₁≤b₂ ✓
   b₁≤a₂ ✓           b₁≤a₂ ✓           b₁≤a₂ ✓

No overlap:
A: [--]           A:       [--]
B:       [--]     B: [--]
   a₁≤b₂ ✗           b₁≤a₂ ✗
```

**Complexity**: O(1) - two comparisons
**Correctness**: Proven by interval intersection theorem

---

## 🚀 Performance Analysis

### Benchmark Results
```
day04_parse:     26.4µs ± 0.05µs
day04_combined:  27.7µs ± 0.09µs
```

**Breakdown**:
- **Parsing**: 26.4µs (95.3% of total)
- **Logic**: 1.3µs (4.7% of total)

**Insights**:
1. **Parsing dominates**: String operations are the bottleneck
2. **Logic is trivial**: Range comparisons are extremely fast
3. **No optimizations needed**: 27.7µs is already excellent

### Scaling Analysis

**Input size**: 1000 pairs

**Per-operation cost**:
- Parse one range: 26.4µs / 2000 ranges ≈ 13ns per range
- Check containment: ~0.65ns per pair
- Check overlap: ~0.65ns per pair

**Theoretical limits**:
- **Best case**: O(n) - must read all input
- **Current**: O(n) - single pass parsing + single pass solving
- **Achieved**: Optimal algorithmic complexity

**Why so fast?**:
1. **No allocations in hot path**: All range operations use stack-allocated structs
2. **Copy semantics**: `Range` is Copy (8 bytes) - no heap indirection
3. **Simple comparisons**: Just integer comparisons, highly cache-friendly
4. **Iterator fusion**: Rust optimizes iterator chains into tight loops

---

## 🎨 Design Patterns

### 1. Parse-Once Pattern
Parse input once, solve both parts with same data.

**Implementation**:
```rust
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);  // Parse once
    (
        solve_part1_impl(&data),     // Reuse parsed data
        solve_part2_impl(&data)
    )
}
```

**Benefits**:
- Eliminates redundant parsing
- Ensures consistency between parts
- Clear separation: parsing vs. solving

---

### 2. Type-Driven Design
Use types to encode domain concepts.

**Implementation**:
```rust
struct Range { start: u32, end: u32 }
struct RangePair { first: Range, second: Range }
```

**Benefits**:
- Self-documenting code
- Type system prevents errors (can't mix up ranges)
- Methods on types encapsulate logic

---

### 3. Iterator Combinators
Use iterator chains for declarative filtering/counting.

**Implementation**:
```rust
data.pairs
    .iter()
    .filter(|pair| pair.has_full_containment())
    .count()
```

**Benefits**:
- Readable: "count pairs that have full containment"
- Efficient: Compiled to tight loop
- Composable: Easy to add more filters

---

### 4. Symmetry in Overlap Check
Overlap is commutative: `a.overlaps(b) ⟺ b.overlaps(a)`

**Design Decision**: Make it explicit in containment check
```rust
first.contains(&second) || second.contains(&first)
```

**Benefits**:
- Clear intent
- No bias toward first/second
- Handles both directions

---

## 🧪 Testing Strategy

### Unit Tests

#### Test Range Operations
```rust
#[test]
fn test_range_contains() {
    let r1 = Range::new(2, 8);
    let r2 = Range::new(3, 7);
    assert!(r1.contains(&r2));
    assert!(!r2.contains(&r1));
}

#[test]
fn test_range_overlaps() {
    // No overlap
    assert!(!Range::new(2, 4).overlaps(&Range::new(6, 8)));
    
    // Overlap
    assert!(Range::new(5, 7).overlaps(&Range::new(7, 9)));
}
```

#### Test Parsing
```rust
#[test]
fn test_parse_range() {
    assert_eq!(parse_range("2-4"), Some(Range::new(2, 4)));
}

#[test]
fn test_parse_line() {
    let pair = parse_line("2-4,6-8").unwrap();
    assert_eq!(pair.first, Range::new(2, 4));
}
```

### Integration Tests
```rust
#[test]
fn test_part1_example() {
    const EXAMPLE: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    assert_eq!(solve_part1(EXAMPLE), 2);
}

#[test]
fn test_part2_example() {
    const EXAMPLE: &str = "...";
    assert_eq!(solve_part2(EXAMPLE), 4);
}
```

**Coverage**:
- ✅ Range containment edge cases
- ✅ Range overlap boundary conditions
- ✅ Parsing valid/invalid input
- ✅ Example data from problem statement
- ✅ Both parts with known answers

---

## ⚠️ Common Pitfalls

### 1. Off-by-One Errors
**Mistake**: Treating ranges as exclusive instead of inclusive
```rust
// ❌ WRONG: Exclusive end
fn overlaps_wrong(&self, other: &Range) -> bool {
    self.start < other.end && other.start < self.end
}

// ✅ CORRECT: Inclusive ranges
fn overlaps(&self, other: &Range) -> bool {
    self.start <= other.end && other.start <= self.end
}
```

**Why it matters**: `[4,4]` and `[4,6]` should overlap (share element 4)

---

### 2. Asymmetric Containment Check
**Mistake**: Only checking one direction
```rust
// ❌ WRONG: Misses reverse containment
fn has_full_containment_wrong(&self) -> bool {
    self.first.contains(&self.second)
}

// ✅ CORRECT: Check both directions
fn has_full_containment(&self) -> bool {
    self.first.contains(&self.second) || 
    self.second.contains(&self.first)
}
```

**Why it matters**: Part 1 answer would be wrong

---

### 3. Parsing Without Error Handling
**Mistake**: Using `unwrap()` instead of `Option`
```rust
// ❌ WRONG: Panics on invalid input
fn parse_range_wrong(s: &str) -> Range {
    let parts: Vec<_> = s.split('-').collect();
    Range::new(
        parts[0].parse().unwrap(),
        parts[1].parse().unwrap()
    )
}

// ✅ CORRECT: Graceful error handling
fn parse_range(s: &str) -> Option<Range> {
    let (start, end) = s.split_once('-')?;
    Some(Range::new(start.parse().ok()?, end.parse().ok()?))
}
```

---

## 💡 Key Takeaways

### Implementation Insights
1. **Endpoint comparisons suffice**: No need to enumerate range elements
2. **Copy semantics win**: Small structs (Range = 8 bytes) should be Copy
3. **Type safety helps**: `Range` and `RangePair` prevent logic errors
4. **Iterator chains are fast**: Rust optimizes them into tight loops

### Algorithmic Insights
1. **Set theory foundation**: Containment = subset, overlap = intersection
2. **O(1) range operations**: Just compare endpoints
3. **Optimal complexity**: Can't beat O(n) when processing n pairs
4. **Symmetry matters**: Overlap and containment are symmetric relations

### Rust Patterns
1. **Parse-once pattern**: Avoid redundant work
2. **Type-driven design**: Let types guide implementation
3. **Error handling with `Option`**: Use `?` for clean propagation
4. **Iterator combinators**: Declarative, efficient, composable

---

## 🔗 Related Concepts

### Mathematical Foundations
- **Set theory**: Subset (containment), intersection (overlap)
- **Interval arithmetic**: Range operations
- **De Morgan's laws**: Negating overlap condition

### Rust Concepts
- **Copy trait**: Small structs can be copied efficiently
- **Iterator combinators**: `filter()`, `count()`
- **Error handling**: `Option`, `?` operator
- **Zero-cost abstractions**: Range methods compile to simple comparisons

### AoC Patterns
- **Range operations**: Common in scheduling/interval problems
- **Parse-once**: Standard pattern for multi-part problems
- **Type-driven design**: Create types that match domain

---

**Navigation**: [← Day 3](day03_function_guide.md) | [Summary](../summary_2022.md) | [Day 5 →](day05_function_guide.md)
