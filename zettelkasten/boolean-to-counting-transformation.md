# Boolean-to-Counting Transformation Pattern

**Type**: Algorithm Pattern  
**Domain**: Dynamic Programming, Competitive Programming  
**Discovered**: AoC 2024 Day 19 (Linen Layout)  
**Complexity**: Same underlying algorithm, different reduction operator

---

## 🎯 The Pattern

A common AoC escalation: **Part 1** asks "Is it possible?" (boolean), **Part 2** asks "How many ways?" (counting). The beautiful insight: **identical algorithmic structure**, only the reduction operator changes.

### Code Example (AoC 2024 Day 19)

```rust
// Part 1: Filter + Count (Boolean existence check)
pub fn solve_part1(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    
    designs
        .iter()
        .filter(|design| can_make_design(&patterns, design))  // ← Boolean predicate
        .count()  // ← Count how many are TRUE
}

// Part 2: Map + Sum (Counting all ways)
pub fn solve_part2(input: &str) -> u64 {
    let (patterns, designs) = parse_input(input);
    
    designs
        .iter()
        .map(|design| count_ways_to_make(&patterns, design))  // ← Returns count
        .sum()  // ← Sum all counts
}
```

**Almost identical**:
- Same `parse_input()`
- Same `designs.iter()`
- Same per-item processing
- Different only in: `filter` → `map` and `count` → `sum`

---

## 🔍 The Underlying Transformation

### Part 1: Existence Check (OR Logic)

```rust
fn can_make_recursive<'a>(
    patterns: &[&str],
    remaining: &'a str,
    memo: &mut HashMap<&'a str, bool>,  // ← Cache booleans
) -> bool {
    if remaining.is_empty() { return true; }  // ← Success: found one way
    if let Some(&result) = memo.get(remaining) { return result; }
    
    for pattern in patterns {
        if let Some(rest) = remaining.strip_prefix(pattern) {
            if can_make_recursive(patterns, rest, memo) {  // ← Check if possible
                memo.insert(remaining, true);
                return true;  // ← Early exit: found ANY solution
            }
        }
    }
    memo.insert(remaining, false);
    false
}
```

**Logic**: `result = subproblem1 OR subproblem2 OR ... OR subproblemN`

### Part 2: Exhaustive Counting (SUM Logic)

```rust
fn count_ways_recursive<'a>(
    patterns: &[&str],
    remaining: &'a str,
    memo: &mut HashMap<&'a str, u64>,  // ← Cache counts
) -> u64 {
    if remaining.is_empty() { return 1; }  // ← Base: one way (do nothing)
    if let Some(&count) = memo.get(remaining) { return count; }
    
    let mut total_ways = 0;
    for pattern in patterns {
        if let Some(rest) = remaining.strip_prefix(pattern) {
            total_ways += count_ways_recursive(patterns, rest, memo);  // ← Accumulate
        }
    }
    memo.insert(remaining, total_ways);
    total_ways  // ← Return sum of all paths
}
```

**Logic**: `result = subproblem1 + subproblem2 + ... + subproblemN`

---

## 📊 The Four Key Changes

| Aspect | Part 1 (Boolean) | Part 2 (Counting) |
|--------|-----------------|-------------------|
| **Cache Type** | `HashMap<&str, bool>` | `HashMap<&str, u64>` |
| **Base Case** | `return true` (found one) | `return 1` (one way) |
| **Reduction** | Early-exit OR (`if found { return true }`) | Accumulative SUM (`total += count()`) |
| **Result** | `bool` (possible/impossible) | `u64` (number of ways) |

**Everything else stays identical**:
- ✅ Same recursive structure
- ✅ Same state space (remaining string)
- ✅ Same transitions (strip_prefix patterns)
- ✅ Same memoization pattern (check → compute → cache)
- ✅ Same complexity (O(P × L) with memo)

---

## 🎓 Why This Pattern Matters

### 1. **Minimal Code Delta**
Changing 4 lines transforms the algorithm from existence to counting. This shows:
- The **structure** is the algorithm (not the types)
- Boolean logic and arithmetic are **dual operations**
- Good DP design is **transformation-friendly**

### 2. **Performance Equivalence**
Both versions have identical complexity:
- **Without memo**: O(P^L) exponential (intractable)
- **With memo**: O(P × L) linear (efficient)

The counting version doesn't need to "work harder" - it just **accumulates instead of short-circuits**.

### 3. **Rust Type System Leverage**
The type system **guides the transformation**:
- Change `HashMap<_, bool>` → `HashMap<_, u64>` 
- Compiler errors show where logic must change
- Type-driven development: let types guide implementation

### 4. **Common AoC Pattern**
This escalation appears repeatedly:
- **Day 19**: Can make design? → How many ways?
- **Grid paths**: Can reach goal? → How many paths?
- **Subset sum**: Is target possible? → How many combinations?
- **Tree traversal**: Is path valid? → Count all valid paths?

**Recognition skill**: When Part 1 asks "possible?", expect Part 2 to ask "how many?"

---

## 🔗 Related Patterns

### Dynamic Programming Core
This transformation works because:
1. **Optimal substructure**: Solution built from subproblems
2. **Overlapping subproblems**: Same states reached multiple ways
3. **Memoization**: Cache prevents recomputation

### Reduction Operators
Mathematical duality:
- **Boolean OR** (`||`): Short-circuit on first `true`
- **Integer SUM** (`+`): Accumulate all contributions
- **Both**: Commutative, associative (order-independent)

### Iterator Patterns
Rust iterators express the pattern elegantly:
- `filter(predicate).count()` ↔ Boolean aggregation
- `map(transform).sum()` ↔ Numeric aggregation

---

## 💡 When to Apply

### Recognize the Pattern When:
- ✅ Part 1 returns `bool` or `Option<T>`
- ✅ Problem involves exploring multiple paths/choices
- ✅ Overlapping subproblems suggest DP
- ✅ Part 2 asks for count/sum instead of existence

### Implementation Steps:
1. **Implement Part 1** with boolean memoization
2. **Verify correctness** (don't optimize prematurely)
3. **Transform for Part 2**:
   - Change cache type: `bool` → `u64`
   - Change base case: `true` → `1`
   - Change loop: `if found { return }` → `total += count()`
   - Change return: `false` → `total`
4. **Reuse parsing/setup** from Part 1

### Common Mistakes:
- ❌ Implementing Part 2 from scratch (unnecessary work)
- ❌ Forgetting to change base case (`true` → `1`)
- ❌ Using `i32` instead of `u64` (overflow for large counts)
- ❌ Not memoizing in Part 1 (Part 2 will be impossible)

---

## 📚 Real-World Examples

### AoC 2024 Day 19 Results
**Part 1**: 360 designs possible (boolean check)  
**Part 2**: 577,474,410,989,846 total arrangements (counting)

**Key insight**: Part 2's answer is **577 trillion** - impossible to compute without memoization. The transformation from Part 1 to Part 2 changed 4 lines but required zero algorithm restructuring.

### AoC 2024 Day 11 (Stone Multiplication)
Similar pattern:
- **Part 1**: Count stones after 25 blinks (could brute force)
- **Part 2**: Count stones after 75 blinks (requires memoization)
- **Implementation**: Same DP structure, different time horizon

### Classic Examples
- **Fibonacci**: Is N reachable? → How many ways to reach N with steps [1,2]?
- **Grid paths**: Can reach (m,n)? → How many paths to (m,n)?
- **Coin change**: Can make amount? → How many ways to make amount?

---

## 🧪 Testing Strategy

### Part 1 Tests Guide Part 2
```rust
#[test]
fn test_can_make_design() {
    // brwrr is possible
    assert!(can_make_design(&patterns, "brwrr"));
    
    // ubwu is impossible
    assert!(!can_make_design(&patterns, "ubwu"));
}

#[test]
fn test_count_ways() {
    // brwrr has 2 ways
    assert_eq!(count_ways_to_make(&patterns, "brwrr"), 2);
    
    // ubwu has 0 ways (impossible)
    assert_eq!(count_ways_to_make(&patterns, "ubwu"), 0);
}
```

**Validation**: 
- Possible designs have count ≥ 1
- Impossible designs have count = 0
- Known counts validate algorithm correctness

---

## 🎯 Mission 11 Integration

This pattern will be **REQ-4** in Mission 11 (Dynamic Programming):
- **Requirement**: Boolean → Counting transformation template
- **Test cases**: Demonstrate transformation on 3+ problems
- **Tutorial**: Step 5 walks through conversion process
- **Documentation**: Explain why structure remains identical

**Learning objective**: Recognize when DP problems have this duality, implement efficiently.

---

## 🔗 Links

**Related Concepts**:
- [[dynamic-programming-memoization]] - Core DP patterns
- [[aoc-part1-to-part2-escalations]] - Common AoC transformations
- [[iterator-reduction-patterns]] - Rust iterator aggregation
- [[lifetime-parametric-recursion]] - Cache implementation details

**Problem Examples**:
- [[aoc2024-day19]] - Linen Layout (pattern composition)
- [[aoc2024-day11]] - Stone Multiplication (numeric DP)
- [[mission-11]] - Planned DP mission using this pattern

**Implementation**:
- [Day 19 Solution](../advent_of_code/aoc2024/src/solver/day19.rs) - Reference implementation
- [Mission 11 README](../missions/Mission11/README.md) - Requirements specification
- [Tutorial 11 Step 5](../tutorials/Mission11_tut/README.md) - Learning progression

---

**Created**: December 18, 2025  
**Source**: AoC 2024 Day 19 analysis  
**Status**: Validated pattern for Mission 11

*Tags: #dynamic-programming #aoc-pattern #transformation #memoization #boolean-to-counting*

*Links: [[dynamic-programming-memoization]] | [[aoc2024-day19]] | [[mission-11]] | [[iterator-reduction-patterns]]*
