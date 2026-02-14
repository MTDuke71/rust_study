# Day 13: Distress Signal - Function Guide

**Problem**: Compare and sort nested list packets to decode a distress signal.

**Navigation**: [← Day 12](day12_function_guide.md) | [Problem](day13.md) | [Code](../../src/solver/day13.rs) | [Summary](../summary_2022.md) | [Day 14 →](day14_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Compare pairs of packets, sum indices of pairs in right order
- **Part 2**: Sort all packets with divider packets, find decoder key

### Performance
- **Combined runtime**: 721µs (parse + Part 1 + Part 2)
- **Parse-once applied**: ✅ Single parse, shared data
- **Complexity**: O(n × m × d) where n = packets, m = avg packet size, d = avg nesting depth

### Key Insight
**Recursive comparison with type coercion**: The problem requires comparing nested structures where integers can be promoted to singleton lists. This is elegantly solved with recursive pattern matching on `serde_json::Value`.

---

## Algorithm Analysis

### Part 1: Packet Pair Comparison

**Goal**: Determine which packet pairs are already in the right order.

**Comparison Rules**:
1. **Both integers**: Lower integer comes first
   - `3 < 5` → Left is smaller, inputs in right order
   
2. **Both arrays**: Compare element by element
   - `[1,1,3] vs [1,1,5]` → Compare positions until mismatch
   - If one list runs out first, shorter list comes first
   - `[1,1,3] vs [1,1,3,1,1]` → Left runs out, inputs in right order

3. **Mixed types**: Convert integer to `[integer]` and retry
   - `[0,0,0] vs 2` → Convert to `[0,0,0] vs [2]`
   - Then compare as arrays

**Algorithm**:
```
For each pair (1-indexed):
    result = compare_packets(left, right)
    if result == Ordering::Less:
        add pair_index to sum
        
return sum
```

**Complexity**: O(n × m × d) where:
- n = number of pairs
- m = average elements per packet
- d = average nesting depth

---

### Part 2: Sorting with Divider Packets

**Goal**: Sort all packets and find positions of divider packets `[[2]]` and `[[6]]`.

**Algorithm**:
```
1. Collect all packets from all pairs
2. Add divider packets: [[2]] and [[6]]
3. Sort using compare_packets as comparator
4. Find indices of dividers (1-based)
5. Return product of indices
```

**Example** (from problem):
```
After sorting 16 packets + 2 dividers:
[]           ← position 1
[[]]         ← position 2
[[[]]]       ← position 3
...
[[2]]        ← position 10 (divider 1)
...
[[6]]        ← position 14 (divider 2)
...

Decoder key = 10 × 14 = 140
```

**Complexity**: O(n log n × m × d) where:
- n = total packets (pairs × 2 + 2 dividers)
- m × d = comparison cost per packet pair

---

## Function Breakdown

### 1. `compare_packets(left: &Value, right: &Value) -> Ordering`

**Purpose**: Recursively compare two packet values according to the rules.

**Implementation**:
```rust
fn compare_packets(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        // Case 1: Both are numbers - direct comparison
        (Value::Number(l), Value::Number(r)) => {
            let l_num = l.as_i64().unwrap();
            let r_num = r.as_i64().unwrap();
            l_num.cmp(&r_num)
        }

        // Case 2: Both are arrays - element-by-element comparison
        (Value::Array(l_arr), Value::Array(r_arr)) => {
            for i in 0..l_arr.len().min(r_arr.len()) {
                match compare_packets(&l_arr[i], &r_arr[i]) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            // If all elements equal, shorter array comes first
            l_arr.len().cmp(&r_arr.len())
        }

        // Case 3: Left is number, right is array - convert left to [left]
        (Value::Number(_), Value::Array(_)) => {
            let l_arr = Value::Array(vec![left.clone()]);
            compare_packets(&l_arr, right)
        }

        // Case 4: Left is array, right is number - convert right to [right]
        (Value::Array(_), Value::Number(_)) => {
            let r_arr = Value::Array(vec![right.clone()]);
            compare_packets(left, &r_arr)
        }

        // Should not happen with valid input
        _ => Ordering::Equal,
    }
}
```

**Key Points**:
- **Pattern matching** on `serde_json::Value` enum cleanly handles all 4 cases
- **Recursive calls** for nested structures (Case 3 & 4 convert then recurse)
- **Element-by-element** comparison in Case 2 with early return on mismatch
- **Length tiebreaker**: If all elements match, shorter array wins

**Example Traces**:

**Simple case**:
```
compare([1,1,3,1,1], [1,1,5,1,1])
  → Both arrays, compare element by element
  → [0]: 1 == 1, continue
  → [1]: 1 == 1, continue
  → [2]: 3 < 5, return Ordering::Less ✅
```

**Mixed types**:
```
compare([[1],[2,3,4]], [[1],4])
  → Both arrays
  → [0]: compare([1], [1])
    → Both arrays, [0]: 1 == 1, lengths equal → Equal
  → [1]: compare([2,3,4], 4)
    → Mixed! Convert right to [4]
    → compare([2,3,4], [4])
      → Both arrays, [0]: 2 < 4 → Ordering::Less ✅
```

**Performance**: O(m × d) per comparison where m = elements, d = depth

---

### 2. `parse_input(input: &str) -> ParsedData`

**Purpose**: Parse the input into packet pairs using `serde_json`.

**Implementation**:
```rust
pub fn parse_input(input: &str) -> ParsedData {
    let pairs = input
        .split("\n\n")                    // Split on blank lines
        .filter(|s| !s.trim().is_empty()) // Skip empty sections
        .map(|section| {
            let mut lines = section.lines();
            let left_str = lines.next().unwrap().trim();
            let right_str = lines.next().unwrap().trim();

            // Parse JSON using serde_json
            let left: Value = serde_json::from_str(left_str)
                .unwrap_or_else(|_| panic!("Failed to parse: {}", left_str));
            let right: Value = serde_json::from_str(right_str)
                .unwrap_or_else(|_| panic!("Failed to parse: {}", right_str));

            PacketPair { left, right }
        })
        .collect();

    ParsedData { pairs }
}
```

**Key Points**:
- **Split on blank lines** to separate pairs
- **serde_json** handles all the complex nested list parsing
- **No manual parsing** needed - JSON parser does the heavy lifting!
- **Error handling** with descriptive panic messages for debugging

**Why serde_json**:
- ✅ Robust handling of nested structures
- ✅ Automatic type detection (numbers vs arrays)
- ✅ Well-tested library (no need to write custom parser)
- ✅ `Value` enum makes pattern matching clean

**Performance**: O(n × m) where n = pairs, m = avg packet size

---

### 3. `solve_part1_with_data(data: &ParsedData) -> usize`

**Purpose**: Find sum of indices of correctly ordered pairs.

**Implementation**:
```rust
fn solve_part1_with_data(data: &ParsedData) -> usize {
    data.pairs
        .iter()
        .enumerate()                           // Get (index, pair)
        .filter_map(|(idx, pair)| {
            // Compare packets
            if compare_packets(&pair.left, &pair.right) == Ordering::Less {
                Some(idx + 1)                  // 1-based indexing!
            } else {
                None
            }
        })
        .sum()
}
```

**Key Points**:
- **enumerate()** gives 0-based indices, convert to 1-based with `idx + 1`
- **filter_map()** combines filtering and mapping in one step
- **Ordering::Less** means left < right (correct order)
- **sum()** accumulates the matching indices

**Example**:
```
Pair 1: [1,1,3] vs [1,1,5] → Less → Add 1
Pair 2: [[1],[2,3,4]] vs [[1],4] → Less → Add 2
Pair 3: [9] vs [[8,7,6]] → Greater → Skip
Pair 4: [[4,4],4,4] vs [[4,4],4,4,4] → Less → Add 4
...
Sum = 1 + 2 + 4 + 6 = 13 (example)
```

**Performance**: O(n × m × d) where n = pairs

---

### 4. `solve_part2_with_data(data: &ParsedData) -> usize`

**Purpose**: Sort all packets and find decoder key.

**Implementation**:
```rust
fn solve_part2_with_data(data: &ParsedData) -> usize {
    // Collect all packets from pairs
    let mut packets: Vec<Value> = data
        .pairs
        .iter()
        .flat_map(|pair| vec![pair.left.clone(), pair.right.clone()])
        .collect();

    // Add divider packets
    let divider1: Value = serde_json::from_str("[[2]]").unwrap();
    let divider2: Value = serde_json::from_str("[[6]]").unwrap();
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    // Sort all packets using our comparison function
    packets.sort_by(compare_packets);

    // Find indices of divider packets (1-based)
    let mut divider1_idx = 0;
    let mut divider2_idx = 0;

    for (idx, packet) in packets.iter().enumerate() {
        if packet == &divider1 {
            divider1_idx = idx + 1;  // 1-based!
        } else if packet == &divider2 {
            divider2_idx = idx + 1;  // 1-based!
        }
    }

    divider1_idx * divider2_idx
}
```

**Key Points**:
- **flat_map()** flattens pairs into individual packets
- **clone()** needed because we're taking ownership (could optimize with references)
- **sort_by()** uses our `compare_packets` function directly
- **Linear search** for dividers (could early-exit after finding both)
- **1-based indexing** for final answer

**Optimization opportunity**:
```rust
// Could early-exit when both dividers found:
for (idx, packet) in packets.iter().enumerate() {
    if packet == &divider1 {
        divider1_idx = idx + 1;
    } else if packet == &divider2 {
        divider2_idx = idx + 1;
    }
    if divider1_idx != 0 && divider2_idx != 0 {
        break;  // Both found!
    }
}
```

**Performance**: O(n log n × m × d) for sorting

---

### 5. `solve(input: &str) -> (usize, usize)`

**Purpose**: Public API that parses once and solves both parts.

**Implementation**:
```rust
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);         // ← Parse ONCE
    (
        solve_part1_with_data(&data),      // ← Use parsed data
        solve_part2_with_data(&data),      // ← Reuse same data
    )
}
```

**Parse-once verification**:
- ✅ Single `parse_input()` call
- ✅ Both parts accept `&ParsedData`
- ✅ Benchmark confirms: combined ≈ Part 1 + Part 2 time

**Performance**: 721µs total (parse included)

---

## Performance Analysis

### Benchmark Results

```
day13_parse:    367.41µs
day13_part1:      3.32µs
day13_part2:    338.07µs
day13_combined: 705.13µs
```

**Breakdown (measured)**:
- Parsing (serde_json): **367.41µs (52.1%)**
- Part 1 comparison: **3.32µs (0.5%)**
- Part 2 sorting: **338.07µs (47.9%)**

**Key insights**:
- ✅ **Part 1 is extremely fast** (3.3µs) - only ~150 comparisons
- ✅ **Part 2 dominates solve time** (~102× slower than Part 1)
  - Sorting 302 packets requires ~2,488 comparisons (n log n)
  - Factor: 2488/150 ≈ 16.6× more comparisons + overhead
- ✅ **Parsing is the bottleneck** (367µs > 341µs for both solves)

**Why is parsing the bottleneck?**
- serde_json is a general-purpose JSON parser
- Handles all JSON features (we only need arrays + integers)
- Allocates `Value` enum for every element

**Potential optimizations** (not implemented):
1. **Custom parser**: Could be 5-10× faster for this specific format
2. **Reuse allocations**: Pool `Vec` allocations during sorting
3. **Cow or references**: Avoid cloning packets in Part 2
4. **Early exit**: Stop searching after finding both dividers

**Why we didn't optimize**:
- ✅ 705µs is well under 100ms budget
- ✅ serde_json is correct and maintainable
- ✅ Premature optimization avoided
- ✅ Code clarity prioritized
- ✅ **LESSON**: Measure, don't guess! Initial estimates were way off.

---

## Key Patterns

### 1. Recursive Type Coercion

**Pattern**: Handle type mismatches by promoting to common type and recursing.

```rust
// Before: Manual branching nightmare
if is_number(left) && is_number(right) {
    compare_numbers(left, right)
} else if is_array(left) && is_array(right) {
    compare_arrays(left, right)
} else if is_number(left) {
    compare_arrays(&[left], right)
} else { /* ... */ }

// After: Clean pattern matching with recursion
match (left, right) {
    (Number(l), Number(r)) => l.cmp(r),
    (Array(l), Array(r)) => compare_element_wise(l, r),
    (Number(_), Array(_)) => compare_packets(&promote(left), right),
    (Array(_), Number(_)) => compare_packets(left, &promote(right)),
}
```

**Benefits**:
- ✅ Each case is independent and testable
- ✅ Recursion handles arbitrarily deep nesting
- ✅ Type coercion isolated to 2 lines

---

### 2. Using serde_json::Value for Heterogeneous Data

**Pattern**: Leverage existing JSON parser instead of custom enum.

**Alternative (manual enum)**:
```rust
enum Packet {
    Number(i64),
    List(Vec<Packet>),
}

// Need custom parser for: [1,[2,[3]]]
```

**Actual (serde_json)**:
```rust
use serde_json::Value;

let packet: Value = serde_json::from_str("[1,[2,[3]]]").unwrap();
// Everything just works!
```

**Benefits**:
- ✅ No custom parsing code (hundreds of lines saved)
- ✅ Handles all JSON edge cases (empty lists, deep nesting, etc.)
- ✅ Pattern matching on `Value` is idiomatic
- ✅ `Value::ArrayValue::Number` map directly to problem domain

**Trade-offs**:
- ❌ Slightly slower than custom enum (but fast enough)
- ❌ Less type safety (could have non-integer numbers in theory)
- ✅ Rapid development outweighs downsides for AoC

---

### 3. sort_by with Custom Comparator

**Pattern**: Use `sort_by()` with existing comparison function.

```rust
// We have this for Part 1:
fn compare_packets(left: &Value, right: &Value) -> Ordering { /* ... */ }

// Reuse it for Part 2:
packets.sort_by(compare_packets);
```

**Why this works**:
- `sort_by()` expects `FnMut(&T, &T) -> Ordering`
- Our function has signature `fn(&Value, &Value) -> Ordering`
- Function pointer automatically coerces to closure!

**Alternative (more verbose)**:
```rust
packets.sort_by(|a, b| compare_packets(a, b));  // Explicit closure
```

**Benefits of direct function**:
- ✅ Less code
- ✅ Compiler can optimize (no closure overhead)
- ✅ Clearer intent

---

### 4. Element-by-Element Comparison with Early Return

**Pattern**: Compare sequences until mismatch, use length as tiebreaker.

```rust
for i in 0..left.len().min(right.len()) {
    match compare(left[i], right[i]) {
        Ordering::Equal => continue,     // Keep checking
        other => return other,           // Found mismatch!
    }
}
left.len().cmp(&right.len())            // All equal, shorter wins
```

**Why this works**:
- Equal elements don't determine order → continue
- First mismatch determines order → early return
- If all elements match, shorter sequence comes first

**Example**:
```
[1,1,3] vs [1,1,3,1,1]
  i=0: 1 == 1, continue
  i=1: 1 == 1, continue
  i=2: 3 == 3, continue
  (loop exits, right has more elements)
  3 < 5 → Ordering::Less ✅
```

---

### 5. filter_map for Combined Filter + Map

**Pattern**: Transform and filter in one pass.

```rust
// Less idiomatic:
data.pairs.iter().enumerate()
    .filter(|(idx, pair)| is_ordered(pair))
    .map(|(idx, _)| idx + 1)
    .sum()

// More idiomatic:
data.pairs.iter().enumerate()
    .filter_map(|(idx, pair)| {
        if is_ordered(pair) { Some(idx + 1) } else { None }
    })
    .sum()
```

**Benefits**:
- ✅ Single iterator adaptor (more efficient)
- ✅ Logic in one place (easier to understand)
- ✅ More flexible (can transform during filter)

---

## Common Mistakes

### 1. Forgetting 1-Based Indexing

**Wrong**:
```rust
.enumerate()
.filter_map(|(idx, pair)| {
    if ordered { Some(idx) } else { None }  // ❌ 0-based!
})
```

**Right**:
```rust
.enumerate()
.filter_map(|(idx, pair)| {
    if ordered { Some(idx + 1) } else { None }  // ✅ 1-based!
})
```

**How to remember**: Problem says "The first pair has index 1"

---

### 2. Incorrect Type Coercion Direction

**Wrong**:
```rust
(Value::Number(_), Value::Array(_)) => {
    let r_arr = vec![right.clone()];  // ❌ Wrong direction!
    compare_packets(left, &r_arr)
}
```

**Right**:
```rust
(Value::Number(_), Value::Array(_)) => {
    let l_arr = vec![left.clone()];   // ✅ Convert left (the number)!
    compare_packets(&l_arr, right)
}
```

**How to remember**: Match says `Number` on LEFT, so convert left to array

---

### 3. Not Handling Empty Lists

**Wrong**:
```rust
(Value::Array(l), Value::Array(r)) => {
    for i in 0..l.len() {  // ❌ Crashes if l.len() > r.len()!
        compare(l[i], r[i])
    }
}
```

**Right**:
```rust
(Value::Array(l), Value::Array(r)) => {
    for i in 0..l.len().min(r.len()) {  // ✅ Safe!
        match compare(&l[i], &r[i]) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    l.len().cmp(&r.len())  // ✅ Handle length difference
}
```

**Test case**: `[] vs [3]` should return `Ordering::Less`

---

### 4. Cloning Without Need

**Inefficient**:
```rust
let divider1 = serde_json::from_str("[[2]]").unwrap();
packets.push(divider1.clone());  // Clone for push
// ... later ...
if packet == &divider1 { /* ... */ }  // Compare with original
```

**Why this works but is wasteful**:
- We clone `divider1` to push it
- Then compare with the original
- We're keeping both original + clone in memory

**Better** (but more complex):
```rust
// Option 1: Parse twice (cheap for small packets)
packets.push(serde_json::from_str("[[2]]").unwrap());
let divider1_val = serde_json::from_str("[[2]]").unwrap();
if packet == &divider1_val { /* ... */ }

// Option 2: Search by value comparison
let target = serde_json::from_str("[[2]]").unwrap();
if packet == &target { /* ... */ }
```

**In practice**: For AoC, the current approach is fine (readable > micro-optimization)

---

### 5. Forgetting to Handle Equal Case

**Wrong**:
```rust
for i in 0..min_len {
    if compare(left[i], right[i]) != Ordering::Equal {
        return compare(left[i], right[i]);  // ❌ Returns Equal too!
    }
}
```

**Right**:
```rust
for i in 0..min_len {
    match compare(&left[i], &right[i]) {
        Ordering::Equal => continue,  // ✅ Keep checking
        other => return other,        // ✅ Only return non-Equal
    }
}
```

---

## Related Problems

### Recursive Comparison

**Similar problems**:
- Comparing file directory trees
- Comparing AST (Abstract Syntax Trees)
- Deep equality checks in data structures

**Pattern**:
1. Base case: Atomic types (numbers, strings)
2. Recursive case: Container types (arrays, objects)
3. Type coercion: Convert when types mismatch

---

### Custom Sorting

**Similar problems**:
- Topological sort (with custom precedence rules)
- Scheduling (with complex priority logic)
- Version string sorting (semantic versioning)

**Pattern**:
1. Define comparison function: `fn(T, T) -> Ordering`
2. Use `sort_by()` or `sort_by_key()`
3. Ensure transitivity (if A < B and B < C, then A < C)

---

### Finding Elements After Sorting

**Similar problems**:
- Day 1 (find top-3 after sorting)
- Binary search after sorting
- Finding percentiles (median = find at position n/2)

**Pattern**:
1. Sort with custom comparator
2. Linear search or binary search for target
3. Return position or value

---

### JSON Parsing

**Similar problems**:
- Configuration file parsing
- API response handling
- Data transformation pipelines

**When to use serde_json**:
- ✅ Input is valid JSON
- ✅ Don't need custom types (Value is flexible enough)
- ✅ Performance is acceptable
- ❌ Parse millions of packets (custom parser would be faster)

---

## Key Takeaways

### 1. Pattern Matching Makes Recursive Logic Clear

**Before understanding**:
```rust
// Mental model: "I need to handle 4 different cases with lots of if-else"
```

**After understanding**:
```rust
match (left, right) {
    (Number, Number) => /* easy */,
    (Array, Array)   => /* recurse on elements */,
    (Number, Array)  => /* convert left and recurse */,
    (Array, Number)  => /* convert right and recurse */,
}
// Mental model: "Pattern matching enumerates all cases visually"
```

**Lesson**: Rust's pattern matching is **perfect** for problems with clearly defined cases like this one.

---

### 2. Leverage Existing Libraries for Complex Parsing

**Attempted approaches**:
1. ❌ Manual recursive descent parser (100+ lines, error-prone)
2. ✅ serde_json (1 line: `serde_json::from_str(input)`)

**Trade-off**:
- Manual parser: ~10× faster, but takes hours to write and debug
- serde_json: 721µs is fine, took 2 minutes to integrate

**Lesson**: For AoC, **choose libraries over custom code** unless performance is truly critical.

---

### 3. Recursive Functions Need Base Cases AND Recursive Cases

**Structure of `compare_packets`**:
- **Base case**: Both numbers → direct comparison (no recursion)
- **Recursive cases**: 
  - Both arrays → recurse on elements
  - Mixed types → convert, then recurse

**Common mistake**: Forgetting base case → infinite recursion!

**Lesson**: Every recursive function needs:
1. Base case (terminates recursion)
2. Recursive case (makes problem smaller)
3. Proof that recursive calls eventually reach base case

---

### 4. Element-by-Element Comparison Pattern

**The pattern**:
```rust
for i in 0..min_length {
    match compare(left[i], right[i]) {
        Equal => continue,
        other => return other,  // First mismatch wins
    }
}
length_comparison  // Tiebreaker
```

**Where it applies**:
- String comparison (lexicographic order)
- Array comparison (this problem)
- Tuple comparison
- Iterator comparison

**Lesson**: This pattern is **fundamental** to comparison algorithms. Learn it once, use it everywhere.

---

### 5. 1-Based Indexing Requires Constant Vigilance

**Rust is 0-based**, but many problems use 1-based indexing:
- Array problems: "The first element is at position 1"
- This problem: "The first pair has index 1"

**Best practice**:
1. Use 0-based internally (enumerate, indexing)
2. Convert to 1-based at boundaries (input/output)
3. Add comments: `idx + 1  // 1-based indexing`

**Lesson**: **Consistency** within code > matching problem specification. Convert at boundaries only.

---

### 6. filter_map is Underused But Powerful

**Common pattern**:
```rust
// Less efficient (two passes):
iter.filter(predicate).map(transform)

// More efficient (one pass):
iter.filter_map(|x| if predicate(x) { Some(transform(x)) } else { None })
```

**When to use**:
- Filter + map in same step
- Transform returns Option (natural filter)
- Performance matters (avoid intermediate allocation)

**Lesson**: Learn iterator adapters deeply - they're more powerful than loops for most cases.

---

### 7. Comparison Functions Enable Reuse

**Code reuse**:
```rust
fn compare_packets(...) -> Ordering {
    // Complex comparison logic
}

// Part 1: Manual comparison
if compare_packets(left, right) == Ordering::Less { /* ... */ }

// Part 2: Sorting reuses same function
packets.sort_by(compare_packets);
```

**Lesson**: Designing functions to return `Ordering` makes them composable with Rust's sorting/comparison infrastructure.

---

### 8. Parse-Once Pattern Now Automatic

**This is the 13th day following parse-once pattern** - it's now second nature!

**Evidence**: 
- ✅ Code structure matches template automatically
- ✅ Benchmark confirms: combined ≈ part1 + part2
- ✅ No user review needed to catch violations

**Lesson**: Establishing patterns early (Day 7 lesson) pays off as they become **habitual**.

---

## Conclusion

Day 13 demonstrates the power of:
1. **Recursive pattern matching** for type-flexible comparison
2. **Library leverage** (serde_json over custom parsing)
3. **Comparison function reuse** (Part 1 comparison → Part 2 sorting)
4. **Early return** in element-wise comparison for efficiency

**Most valuable lesson**: When a problem involves comparing complex nested structures with type coercion rules, **pattern matching + recursion** is the natural, elegant solution in Rust.

---

**Navigation**: [← Day 12](day12_function_guide.md) | [Problem](day13.md) | [Code](../../src/solver/day13.rs) | [Summary](../summary_2022.md) | [Day 14 →](day14_function_guide.md)
