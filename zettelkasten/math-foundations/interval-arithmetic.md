# Interval Arithmetic

**Field**: Discrete Mathematics / Real Analysis

**Prerequisites**: [[set-theory-fundamentals]] (for theoretical foundation)

---

## 📐 Definition

**Interval**: A continuous range of values between two endpoints, typically represented as $[a, b]$ where $a \leq b$.

**Formal Notation**:
- **Closed interval**: $[a, b] = \{x \mid a \leq x \leq b\}$ (includes both endpoints)
- **Open interval**: $(a, b) = \{x \mid a < x < b\}$ (excludes both endpoints)
- **Half-open**: $[a, b)$ or $(a, b]$ (includes one endpoint)
- **Interval endpoints**: $a$ = start/lower bound, $b$ = end/upper bound

**Intuition**: An interval represents all values between two boundaries. In AoC and competitive programming, we typically use **closed integer intervals** like $[2, 8] = \{2, 3, 4, 5, 6, 7, 8\}$.

**Set Theory Connection**: An interval is a special type of set with a **continuous range** of elements. While general sets can have arbitrary elements like $\{2, 5, 9\}$, intervals contain *all* values between endpoints.

---

## 🔑 Key Properties/Theorems

### **Property 1**: Endpoint Representation
- **Statement**: An interval is completely characterized by its two endpoints
- **Significance**: No need to enumerate elements—just compare boundaries
- **Rust**: Store as `struct Range { start: u32, end: u32 }` (8 bytes vs potentially thousands of elements)

### **Property 2**: Order Invariant
- **Statement**: For valid intervals, $a \leq b$ always holds
- **Significance**: Start is always ≤ end
- **Rust**: Can assume this property (puzzle inputs are well-formed)

---

## 🎯 Core Algorithms

### **Algorithm 1: Interval Containment**

**Problem**: Given intervals $A = [a_1, a_2]$ and $B = [b_1, b_2]$, does $A$ fully contain $B$?

**Mathematical Definition**:
$$A \supseteq B \iff B \subseteq A \iff a_1 \leq b_1 \land a_2 \geq b_2$$

**Set Theory Foundation**: 
- Containment = subset relation: $B \subseteq A$ means every element of $B$ is in $A$
- For intervals: All elements $\{b_1, b_1+1, ..., b_2\}$ are in $\{a_1, a_1+1, ..., a_2\}$

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

Case 2: A doesn't contain B (B extends past A's end)
A: [-----]
B:   [------]
   a₁≤b₁ ✓  a₂≥b₂ ✗

Case 3: A doesn't contain B (B starts before A)
A:   [-----]
B: [------]
   a₁≤b₁ ✗
```

**Implementation**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}
```

**Complexity**: $O(1)$ - exactly two comparisons

**Edge Cases**:
- Self-containment: $[a, b]$ contains itself ✓
- Single element: $[5, 5]$ contains $[5, 5]$ ✓
- Empty intervals: Not applicable (puzzle inputs are valid)

---

### **Algorithm 2: Interval Intersection (Overlap)**

**Problem**: Given intervals $A = [a_1, a_2]$ and $B = [b_1, b_2]$, do they share any elements?

**Mathematical Definition**:
$$A \cap B \neq \emptyset \iff a_1 \leq b_2 \land b_1 \leq a_2$$

**Derivation via Negation** (using De Morgan's Laws):
- Intervals **don't** overlap when they're disjoint:
  - $a_2 < b_1$ (A ends before B starts), OR
  - $b_2 < a_1$ (B ends before A starts)
- By De Morgan's law, intervals **do** overlap when:
  - $\neg(a_2 < b_1 \lor b_2 < a_1)$
  - $= \neg(a_2 < b_1) \land \neg(b_2 < a_1)$
  - $= a_2 \geq b_1 \land b_2 \geq a_1$
  - $= a_1 \leq b_2 \land b_1 \leq a_2$ (reordered for symmetry)

**Visual Proof**:
```
Overlap cases:
A: [-----]        
B:    [-----]     ✓ Partial overlap
   a₁≤b₂ ✓ (2≤6)
   b₁≤a₂ ✓ (4≤5)

A:  [-----]      
B:    [--]        ✓ B contained in A
   a₁≤b₂ ✓
   b₁≤a₂ ✓

A: [--------]
B:   [----]       ✓ Full containment is special case of overlap
   a₁≤b₂ ✓
   b₁≤a₂ ✓

No overlap:
A: [--]           
B:       [--]     ✗ Gap between ranges
   a₁≤b₂ ✗ (4≤6) but b₁≤a₂ ✗ (6≤4)

Boundary touching:
A: [--]
B:    [--]        ✓ Touching at single point counts as overlap
   [2,4] ∩ [4,6] = {4}
```

**Implementation**:
```rust
impl Range {
    pub fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}
```

**Complexity**: $O(1)$ - exactly two comparisons

**Properties**:
- **Symmetric**: $A$ overlaps $B \iff B$ overlaps $A$
- **Containment implies overlap**: If $A \supseteq B$ then $A \cap B \neq \emptyset$
- **Transitive for containment**: $A \supseteq B \land B \supseteq C \implies A \supseteq C$
- **NOT transitive for overlap**: $A \cap B \neq \emptyset \land B \cap C \neq \emptyset \not\Rightarrow A \cap C \neq \emptyset$

---

### **Algorithm 3: Sweep Line Merge (Sort + Linear Scan)**

**Problem**: Given $n$ intervals, merge all overlapping and adjacent intervals into a minimal non-overlapping set, then query the gaps.

**Mathematical Definition**:
Given intervals $\{I_1, I_2, ..., I_n\}$, produce merged set $\{M_1, M_2, ..., M_k\}$ where:
- $k \leq n$ (merged set is smaller or equal)
- $\bigcup M_i = \bigcup I_i$ (same coverage)
- $\forall i \neq j: M_i \cap M_j = \emptyset$ (non-overlapping)
- No two merged intervals are adjacent: $M_i.end + 1 < M_{i+1}.start$

**Algorithm**:
1. **Sort** all intervals by start position: $O(n \log n)$
2. **Sweep** left to right, maintaining current merged interval: $O(n)$
3. For each interval $(lo, hi)$:
   - If $lo \leq \text{current.end} + 1$: **extend** current (overlap or adjacent)
   - Otherwise: **emit** current, start new interval

**Why adjacency matters**: For gap-finding problems, $[0, 5]$ and $[6, 10]$ should merge to $[0, 10]$ even though they don't overlap. The gap test is $lo \leq end + 1$, not $lo \leq end$.

**Visual Proof**:
```
Input (unsorted):
  [5,8] [0,2] [4,7]

After sort by start:
  [0,2] [4,7] [5,8]

Sweep:
  Start: current = [0,2]
  [4,7]: 4 > 2+1=3 → GAP! Emit [0,2], current = [4,7]
  [5,8]: 5 ≤ 7+1=8 → Extend to [4,8]
  Done: Emit [4,8]

Result: {[0,2], [4,8]}
Gap:    {3}              ← lowest allowed value
```

**Overflow Trap**: When `end == u32::MAX`, computing `end + 1` overflows. Promote to `u64`:
```rust
// WRONG: overflows when last.1 == u32::MAX
if lo <= last.1 + 1 { ... }

// CORRECT: u64 arithmetic avoids overflow
if (lo as u64) <= (last.1 as u64) + 1 { ... }
```

**Implementation** (from AoC 2016 Day 20):
```rust
type Range = (u32, u32);

fn merge_ranges(ranges: &mut [Range]) -> Vec<Range> {
    ranges.sort_unstable();
    let mut merged: Vec<Range> = Vec::new();
    for &(lo, hi) in ranges.iter() {
        if let Some(last) = merged.last_mut() {
            if (lo as u64) <= (last.1 as u64) + 1 {
                last.1 = last.1.max(hi);
                continue;
            }
        }
        merged.push((lo, hi));
    }
    merged
}
```

**Querying the merged result**:
- **First gap** (lowest allowed value): `merged[0].1 + 1` (if first range starts at 0)
- **Count gaps** (total allowed): `total_space - sum(hi - lo + 1 for each merged range)`

**Complexity**:
- **Time**: $O(n \log n)$ for sort + $O(n)$ for sweep = $O(n \log n)$
- **Space**: $O(n)$ for merged output (often $k \ll n$)

---

### **Theorem 1**: Interval Intersection Test
- **Statement**: Two intervals $[a_1, a_2]$ and $[b_1, b_2]$ overlap if and only if $a_1 \leq b_2 \land b_1 \leq a_2$
- **Proof**: By contrapositive—they're disjoint iff one ends before the other starts
- **Applications**: Range overlap queries, scheduling conflicts, collision detection

### **Theorem 2**: Containment Subset
- **Statement**: $[a_1, a_2] \supseteq [b_1, b_2] \iff a_1 \leq b_1 \land a_2 \geq b_2$
- **Proof**: Subset relation in set theory applied to continuous ranges
- **Applications**: Range validation, hierarchical scheduling, resource allocation

---

## 💻 Rust Implementations

### **AoC 2022 Day 4**: Camp Cleanup - Range Overlap Detection
- **Problem**: Elves assigned section ranges. Find pairs where assignments overlap.
  - Part 1: Count pairs where one range fully contains the other
  - Part 2: Count pairs where ranges have any overlap
- **How it uses interval arithmetic**:
  - Parse ranges: `"2-4"` → `Range { start: 2, end: 4 }`
  - Check containment: Does `[2,8]` contain `[3,7]`? → Yes ($2 \leq 3 \land 8 \geq 7$)
  - Check overlap: Do `[5,7]` and `[7,9]` overlap? → Yes ($5 \leq 9 \land 7 \leq 7$)
- **Results**:
  - Part 1: 448 pairs with full containment
  - Part 2: 794 pairs with any overlap
- **Performance**: 27.7µs (O(1) per pair, 1000 pairs total)
- **Link**: `advent_of_code/aoc2022/src/solver/day04.rs` | [[2026-02-04]]

### **AoC 2016 Day 20**: Firewall Rules - Sweep Line Merge + Gap Finding
- **Problem**: Firewall blocks IP ranges (0 to 4,294,967,295). Find lowest unblocked IP and count all allowed IPs.
  - Part 1: Lowest allowed IP address
  - Part 2: Total count of allowed IPs
- **How it uses interval arithmetic**:
  - Parse 1005 blocked ranges: `"3382920125-3384842620"` → `(u32, u32)` tuple
  - Sort + merge overlapping/adjacent ranges (Algorithm 3 above)
  - Part 1: First gap after merged block starting at 0
  - Part 2: Total IP space (2^32) minus sum of blocked ranges
- **Key insight**: Adjacent ranges must merge ($[0,5]$ + $[6,10]$ → $[0,10]$), requiring `lo <= end + 1` check
- **Overflow safety**: `u32::MAX + 1` overflows — promote adjacency check to `u64`
- **Results**:
  - Part 1: 22,887,907 (lowest allowed IP)
  - Part 2: 109 (total allowed IPs out of 4.3 billion!)
- **Performance**: 42.5us combined (sort dominates at 1005 ranges)
- **Link**: `advent_of_code/aoc2016/src/solver/day20.rs` | [[2026-03-20]]

**Complete Implementation from Day 4**:
```rust
/// A range of section IDs [start, end] (inclusive)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

impl Range {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Check if this range fully contains another range
    /// Mathematical: self ⊇ other ⟺ self.start ≤ other.start ∧ self.end ≥ other.end
    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Check if this range overlaps with another range at all
    /// Mathematical: self ∩ other ≠ ∅ ⟺ self.start ≤ other.end ∧ other.start ≤ self.end
    pub fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

/// A pair of section assignment ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangePair {
    pub first: Range,
    pub second: Range,
}

impl RangePair {
    /// Check if one range fully contains the other (symmetric)
    pub fn has_full_containment(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    /// Check if the ranges have any overlap
    pub fn has_overlap(&self) -> bool {
        self.first.overlaps(&self.second)
    }
}
```

**Usage Example**:
```rust
// Part 1: Full containment
let pair1 = RangePair {
    first: Range::new(2, 8),
    second: Range::new(3, 7),
};
assert!(pair1.has_full_containment());  // [2,8] ⊇ [3,7]

// Part 2: Partial overlap
let pair2 = RangePair {
    first: Range::new(5, 7),
    second: Range::new(7, 9),
};
assert!(pair2.has_overlap());  // [5,7] ∩ [7,9] = {7}

// No overlap
let pair3 = RangePair {
    first: Range::new(2, 4),
    second: Range::new(6, 8),
};
assert!(!pair3.has_overlap());  // [2,4] ∩ [6,8] = ∅
```

**Mathematical Validation**:
```rust
#[test]
fn validate_containment_algorithm() {
    // Test cases from mathematical definition
    let r1 = Range::new(2, 8);
    let r2 = Range::new(3, 7);
    
    // r1 ⊇ r2 because 2 ≤ 3 AND 8 ≥ 7
    assert!(r1.contains(&r2));
    
    // r2 ⊉ r1 because 3 ≤ 2 is false
    assert!(!r2.contains(&r1));
    
    // Self-containment (reflexive property)
    assert!(r1.contains(&r1));
}

#[test]
fn validate_overlap_algorithm() {
    // Overlap cases
    assert!(Range::new(5, 7).overlaps(&Range::new(7, 9)));  // Touch at boundary
    assert!(Range::new(2, 8).overlaps(&Range::new(3, 7)));  // Full containment
    assert!(Range::new(1, 5).overlaps(&Range::new(3, 7)));  // Partial overlap
    
    // Disjoint (no overlap)
    assert!(!Range::new(2, 4).overlaps(&Range::new(6, 8))); // Gap between
    
    // Symmetry property
    let a = Range::new(5, 7);
    let b = Range::new(7, 9);
    assert_eq!(a.overlaps(&b), b.overlaps(&a));
}
```

---

## 📚 Code Examples

### **Parsing Intervals from Strings**

```rust
/// Parse a range string like "2-4" into Range { start: 2, end: 4 }
fn parse_range(s: &str) -> Option<Range> {
    let (start, end) = s.split_once('-')?;
    Some(Range::new(
        start.parse().ok()?,
        end.parse().ok()?
    ))
}

/// Parse a line like "2-4,6-8" into a RangePair
fn parse_line(line: &str) -> Option<RangePair> {
    let (first, second) = line.split_once(',')?;
    Some(RangePair {
        first: parse_range(first)?,
        second: parse_range(second)?,
    })
}

// Usage
let pair = parse_line("2-4,6-8").unwrap();
assert_eq!(pair.first, Range::new(2, 4));
assert_eq!(pair.second, Range::new(6, 8));
```

### **Counting Overlaps Efficiently**

```rust
/// Count pairs where one range fully contains the other (Part 1)
fn count_full_containments(pairs: &[RangePair]) -> usize {
    pairs.iter()
        .filter(|pair| pair.has_full_containment())
        .count()
}

/// Count pairs where ranges have any overlap (Part 2)
fn count_overlaps(pairs: &[RangePair]) -> usize {
    pairs.iter()
        .filter(|pair| pair.has_overlap())
        .count()
}

// Note: Part 2 count ≥ Part 1 count (containment implies overlap)
// Day 4: 448 containments, 794 overlaps → 346 partial overlaps
```

### **Advanced: Multiple Interval Operations**

```rust
/// Find the intersection interval (if it exists)
fn intersection(a: &Range, b: &Range) -> Option<Range> {
    if !a.overlaps(b) {
        return None;
    }
    
    Some(Range::new(
        a.start.max(b.start),  // Intersection starts at later start
        a.end.min(b.end)       // Intersection ends at earlier end
    ))
}

/// Merge overlapping intervals
fn merge(a: &Range, b: &Range) -> Option<Range> {
    if !a.overlaps(b) {
        return None;
    }
    
    Some(Range::new(
        a.start.min(b.start),  // Union starts at earlier start
        a.end.max(b.end)       // Union ends at later end
    ))
}

// Example
let a = Range::new(2, 5);
let b = Range::new(4, 8);

let inter = intersection(&a, &b).unwrap();
assert_eq!(inter, Range::new(4, 5));  // [2,5] ∩ [4,8] = [4,5]

let merged = merge(&a, &b).unwrap();
assert_eq!(merged, Range::new(2, 8));  // [2,5] ∪ [4,8] = [2,8]
```

---

## 🌳 Related Concepts

### **Mathematical Prerequisites**:
- [[set-theory-fundamentals]] - Theoretical foundation (subset, intersection)
- [[number-theory-basics]] - Integer ranges and properties

### **Related Mathematical Concepts**:
- [[computational-geometry-basics]] - Line segment intersection (1D intervals generalize to 2D/3D)
- [[graph-theory-fundamentals]] - Interval graphs (vertices = intervals, edges = overlaps)
- [[combinatorics-fundamentals]] - Counting interval arrangements

### **Related Rust Concepts**:
- [[common-traits-pattern]] - PartialEq, Eq, Copy for Range types
- [[complexity-theory]] - O(1) interval operations vs O(n) element enumeration

### **Applications**:
- **Scheduling**: Find conflicting time slots
- **Resource allocation**: Check if ranges overlap
- **Collision detection**: Bounding box overlap in games
- **Date ranges**: Calendar scheduling, availability checking
- **IP address ranges**: Network CIDR overlap detection

---

## 🎯 Common Patterns in AoC/Competitive Programming

### **Pattern 1: Range Validation**
```rust
/// Check if all ranges in a collection are non-overlapping
fn are_disjoint(ranges: &[Range]) -> bool {
    for i in 0..ranges.len() {
        for j in (i+1)..ranges.len() {
            if ranges[i].overlaps(&ranges[j]) {
                return false;
            }
        }
    }
    true
}
```

### **Pattern 2: Interval Merging**
```rust
/// Merge all overlapping intervals into non-overlapping set
fn merge_intervals(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }
    
    // Sort by start position
    ranges.sort_by_key(|r| r.start);
    
    let mut merged = vec![ranges[0]];
    
    for range in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        
        if last.overlaps(&range) {
            // Extend current interval
            last.end = last.end.max(range.end);
        } else {
            // Start new interval
            merged.push(range);
        }
    }
    
    merged
}

// Example: [1,3], [2,5], [7,9], [8,10] → [1,5], [7,10]
```

### **Pattern 3: Point Containment**
```rust
/// Check if a point lies within an interval
fn contains_point(range: &Range, point: u32) -> bool {
    range.start <= point && point <= range.end
}

// Useful for: "Is this coordinate in bounds?"
assert!(contains_point(&Range::new(0, 100), 50));
```

### **Pattern 4: Range Comparison**
```rust
/// Compare two intervals by various criteria
#[derive(Debug, PartialEq)]
enum RangeRelation {
    Disjoint,           // No overlap
    AContainsB,         // A ⊇ B
    BContainsA,         // B ⊇ A
    PartialOverlap,     // Some overlap but neither contains the other
    Equal,              // Same interval
}

fn classify_relation(a: &Range, b: &Range) -> RangeRelation {
    if !a.overlaps(b) {
        RangeRelation::Disjoint
    } else if a == b {
        RangeRelation::Equal
    } else if a.contains(b) {
        RangeRelation::AContainsB
    } else if b.contains(a) {
        RangeRelation::BContainsA
    } else {
        RangeRelation::PartialOverlap
    }
}
```

---

## 📊 Complexity Reference

| Operation | Interval Method | Naive (enumerate) | Speedup |
|-----------|----------------|-------------------|---------|
| Containment | O(1) | O(n) | ~1000× for [0,1000] |
| Overlap | O(1) | O(n×m) | ~1,000,000× |
| Intersection | O(1) | O(min(n,m)) | ~1000× |
| Merge | O(1) | O(n+m) | ~1000× |

**Memory**:
- Interval representation: O(1) - just two integers
- Element enumeration: O(n) - store all values in range

**From Day 4 Analysis**:
- **Parsing**: 26.4µs for 1000 range pairs
- **Logic**: 1.3µs for 2000 range comparisons
- **Per comparison**: ~0.65 nanoseconds (essentially free)

**Why So Fast?**:
1. **No allocations**: Range is Copy (8 bytes), stack-allocated
2. **Branch-free**: Modern CPUs execute simple comparisons in parallel
3. **Cache-friendly**: Entire Range fits in single cache line
4. **Optimal algorithm**: Can't beat O(1) for these operations

---

## 💡 Key Insights

### **Mathematical Elegance**:
- Interval operations reduce to endpoint comparisons (O(1) vs O(n))
- Set theory provides rigorous foundation (containment = subset)
- De Morgan's laws derive overlap from non-disjointness

### **Algorithmic Power**:
- **Containment**: Single condition checks subset relation
- **Overlap**: Negation of "one ends before other starts"
- **Symmetry**: Overlap is commutative, containment is not

### **Rust Implementation**:
- Small Copy structs (8 bytes) avoid heap allocations
- Type safety prevents mixing ranges with individual values
- Methods encapsulate mathematical invariants

### **Problem-Solving Insights**:
- Transform "find overlapping ranges" → interval intersection test
- Transform "check if range contains another" → endpoint comparison
- Transform "merge ranges" → sort + linear scan with overlap check

### **From AoC 2022 Day 4**:
> "Interval arithmetic is all about **not** enumerating elements. A range [1,1000000] is represented by two u32 values (8 bytes), not a million-element vector. The algorithms exploit this compact representation for O(1) operations."

### **Common Mistakes to Avoid**:
- ❌ Treating intervals as exclusive when they're inclusive
- ❌ Checking only one containment direction (need both A⊇B and B⊇A)
- ❌ Using `<` instead of `≤` in overlap test (misses boundary touching)
- ❌ Enumerating range elements instead of comparing endpoints

---

## 📖 Resources

### **Mathematical Foundations**:
- [Interval (mathematics) - Wikipedia](https://en.wikipedia.org/wiki/Interval_(mathematics))
- [Interval Arithmetic](https://en.wikipedia.org/wiki/Interval_arithmetic)
- [Set Theory for Intervals](https://en.wikibooks.org/wiki/Real_Analysis/Continuity#Intervals)

### **Rust Implementation**:
- [`std::ops::Range`](https://doc.rust-lang.org/std/ops/struct.Range.html) - Built-in range type (for iteration, not arithmetic)
- [`std::ops::RangeInclusive`](https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html) - Inclusive range `a..=b`

### **Applications**:
- Computational Geometry: Line segment intersection
- Scheduling Theory: Resource conflict detection
- Database Systems: Range queries and indexing

---

## 🎓 Learning Progression

**Beginner** (understand):
- Intervals represent continuous ranges
- Containment = "Is B inside A?"
- Overlap = "Do A and B share any values?"

**Intermediate** (implement):
- Endpoint comparison algorithms (O(1))
- Symmetric vs asymmetric operations
- Edge cases (boundary touching, self-containment)

**Advanced** (optimize):
- Interval merging algorithms (sort + linear sweep)
- Sweep line techniques for multiple intervals (see Algorithm 3 below)
- Interval trees for efficient range queries
- Adjacency merging: $[a, b]$ and $[b+1, c]$ merge to $[a, c]$ (critical for gap-finding)
- Overflow safety: promote to wider type when endpoints are at type boundary (e.g., `u32::MAX + 1`)

---

*Tags: #mathematics #interval-arithmetic #real-analysis #discrete-math #ranges #aoc2022-day4 #aoc2016-day20 #sweep-line #computational-geometry*

*Created*: 2026-02-04
*Last Updated*: 2026-03-20
*Implementations*: 2 (AoC 2022 Day 4, AoC 2016 Day 20)
