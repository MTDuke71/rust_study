# Day 21: Monkey Math - Function Guide

**Problem**: Evaluate a tree of monkey expressions and solve for a single unknown variable.

**Navigation**: [← Day 20](day20_function_guide.md) | [Problem](day21.md) | [Code](../../src/solver/day21.rs) | [Summary](../summary_2022.md) | [Day 22 →](day22_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Each monkey either yells a number or computes `left op right`. Evaluate the tree rooted at `root`.
- **Part 2**: `root` becomes an equality check (`=`), and `humn` is unknown. Find the value `humn` must yell.

### Performance
- **Parse**: 156µs (HashMap construction from ~2,800 monkeys)
- **Part 1**: 32µs (recursive tree evaluation)
- **Part 2**: 217µs (tree walk + operation inversion)
- **Combined**: ~499µs

### Key Insight
**The expression tree has exactly one unknown.** Since `humn` appears in exactly one branch of every node along the path from `root` to `humn`, Part 2 reduces to walking that path and inverting each operation to propagate a target value downward.

---

## Algorithm Analysis

### Part 1: Recursive Tree Evaluation

Standard recursive evaluation — each monkey's value depends on at most two others:

```
root: pppw + sjmn
  → eval("pppw") + eval("sjmn")
    → eval("cczh") / eval("lfqf") + eval("drzm") * eval("dbpl")
      → ... (recurse until leaf numbers)
```

No memoization needed: the dependency graph is a tree (no shared subexpressions), so each monkey is evaluated exactly once. With ~2,800 monkeys, this completes in ~32µs.

### Part 2: Inversion Walk

The tree has one unknown (`humn`). For `root`'s equality check:

1. **Determine which branch contains `humn`** — recursive `contains_humn()` search
2. **Evaluate the known branch** — standard `eval()` gives the target value
3. **Walk down the `humn` branch**, inverting each operation:

```
If humn is on the LEFT:
  left + k = target  →  left = target - k
  left - k = target  →  left = target + k
  left * k = target  →  left = target / k
  left / k = target  →  left = target * k

If humn is on the RIGHT:
  k + right = target  →  right = target - k
  k - right = target  →  right = k - target    ← NOT target - k!
  k * right = target  →  right = target / k
  k / right = target  →  right = k / target    ← NOT target / k!
```

The asymmetry for subtraction and division is the primary trap. When `humn` is on the right side of a non-commutative operation, the inversion is different.

---

## Implementation Details

### Data Flow
```
Input → parse_input() → HashMap<&str, Monkey>
                          ↓
                    solve_part1() → eval("root") → i64
                          ↓
                    solve_part2() → contains_humn() → find humn branch
                                  → eval(known_side) → target value
                                  → solve_for_humn() → walk + invert → i64
```

### Function Signatures

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `parse_input` | `&str` | `HashMap<&str, Monkey>` | Parse monkey definitions |
| `eval` | `&ParsedData, &str` | `i64` | Recursively evaluate a monkey |
| `contains_humn` | `&ParsedData, &str` | `bool` | Check if subtree contains `humn` |
| `solve_for_humn` | `&ParsedData, &str, i64` | `i64` | Walk toward humn inverting ops |

### Key Code Patterns

**Zero-copy parsing with `&str` references**:
```rust
pub enum Monkey<'a> {
    Num(i64),
    Op { left: &'a str, op: char, right: &'a str },
}
```
Monkey names borrow directly from the input string — no String allocations. The lifetime `'a` ties parsed data to the input's lifetime.

**HashMap for O(1) monkey lookup**:
```rust
pub type ParsedData<'a> = HashMap<&'a str, Monkey<'a>>;
```
Each `eval()` call does a constant-time lookup by name, making tree traversal O(n) total.

---

## Performance Analysis

### Benchmark Results
```
Parse:    156µs  (HashMap from ~2,800 lines)
Part 1:    32µs  (single tree traversal)
Part 2:   217µs  (contains_humn + eval + solve_for_humn)
Combined: 499µs
```

Part 2 is ~7× slower than Part 1 because `contains_humn()` traverses the tree to determine which branch holds `humn` at each level. A potential optimization would be to cache `contains_humn` results, but at 217µs total it's well within budget.

### Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| Parse | O(n) | One pass through input |
| `eval` | O(n) | Each monkey visited once (tree, no sharing) |
| `contains_humn` | O(n) per call, O(n × d) total | Called at each level of depth d |
| `solve_for_humn` | O(d × n) | d levels, contains_humn + eval at each |
| Overall Part 2 | O(n × d) | d = tree depth (~log n for balanced) |

---

## Edge Cases

- **Non-commutative inversion**: `k - right = target → right = k - target` (not `target - k`)
- **Division inversion when right**: `k / right = target → right = k / target`
- **Integer division**: All divisions in the input are exact (no truncation issues)
- **Large numbers**: Part 1 answer exceeds 32-bit range (160 trillion) — `i64` required

---

## Key Takeaways

1. **Expression tree evaluation is naturally recursive** — HashMap lookup + pattern matching makes it clean and fast
2. **Single unknown → algebraic inversion** — no need for symbolic math, just invert operations while walking the tree
3. **Non-commutative operations require left/right awareness** — subtraction and division invert differently depending on which side is unknown
4. **Zero-copy parsing with lifetimes** — `&'a str` references avoid String allocations while keeping the HashMap ergonomic
5. **No memoization needed** — the dependency graph is a tree (no DAG sharing), so naive recursion visits each node exactly once

---

**Answer**: Part 1: `160274622817992` | Part 2: `3087390115721`

**Related patterns**: [[expression-tree]] | [[algebraic-inversion]] | [[modular-arithmetic]]
