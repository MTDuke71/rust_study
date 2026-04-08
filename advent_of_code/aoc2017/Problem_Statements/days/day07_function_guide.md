# Day 7: Recursive Circus — Function Guide

**Problem**: Find the root of a weighted program tree, then identify the single program with the wrong weight and compute the correction needed for tower balance.
**Answers**: Part 1 = **airlri**, Part 2 = **1,206**
**Code**: [day07.rs](../../src/solver/day07.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [find_root](#find_root)
5. [total_weight](#total_weight)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: ~1,400 lines describing programs with weights and optional children:
```
fwft (72) -> ktlj, cntj, xhth
pbga (66)
```

**Part 1**: Find the **root program** — the one that holds up the entire tower (appears in no other program's children list).

**Part 2**: Exactly one program has the wrong weight. Find what its weight **should be** so that every disc in the tower is balanced (all children of each node have equal subtree weights).

**Example** (13 programs):
```
tknk (41) -> ugml, padx, fwft
ugml (68) -> gyxo, ebii, jptl    subtree = 68+61+61+61 = 251  ← too heavy
padx (45) -> pbga, havc, qoyq    subtree = 45+66+66+66 = 243
fwft (72) -> ktlj, cntj, xhth    subtree = 72+57+57+57 = 243
```
- Part 1: **tknk** (root)
- Part 2: **60** (ugml weighs 68, needs to be 68−8=60 to make subtree 243)

---

## Data Structures

### Program struct
```rust
struct Program<'a> {
    weight: u32,
    children: Vec<&'a str>,
}
```

Borrows program names from the input string (`&'a str`) — zero allocation for names. Children stored as `Vec` since each program has 0–N children.

### Program map
```rust
HashMap<&'a str, Program<'a>>
```

Maps program name → its weight and children. O(1) lookup by name, essential for tree traversal in `total_weight`.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> HashMap<&str, Program<'_>>
```

Parses each line in a single pass using `split_once`:
1. `split_once(' ')` → name, rest
2. `split_once(')')` → weight string (trim leading `(`), children string
3. `strip_prefix(" -> ")` → if present, split children by `", "`

**Zero-copy**: Program names and the input string share the same memory. Only `Vec<&str>` allocations for children lists.

---

## `find_root`

```rust
fn find_root<'a>(programs: &HashMap<&'a str, Program<'a>>) -> &'a str
```

Collects all names that appear as children into a `HashSet`, then finds the one program name NOT in that set.

**Complexity**: O(n) where n = total programs. Single pass to build child set + single pass to find the missing name.

**Alternative approaches**:
- **Indegree counting**: Count how many times each name appears as a child; root has indegree 0
- **Set difference**: `all_names − all_children` = root

All are O(n); the HashSet approach is simplest.

---

## `total_weight`

```rust
fn total_weight(name: &str, programs: &HashMap<&str, Program<'_>>) -> Result<u32, u32>
```

Recursive bottom-up computation. Returns:
- `Ok(total)` — balanced subtree with this total weight
- `Err(corrected_weight)` — found the wrong weight; propagates up immediately

**Algorithm**:
1. Leaf node → return `Ok(own_weight)`
2. Recurse into all children (short-circuit on `Err`)
3. If all children have equal subtree weights → return `Ok(own + sum_of_children)`
4. Otherwise, find the **minority** weight (the odd one out):
   - Group children by subtree weight into a frequency map
   - The group with exactly 1 member is wrong
   - Compute correction: `wrong_own_weight + (correct_total − wrong_total)`
   - Return `Err(corrected_weight)`

**Key insight**: The minority detection uses `HashMap<u32, Vec<&str>>`. With 2+ children, the wrong weight always appears exactly once (puzzle guarantee). The correction adjusts the program's **own weight** by the difference in subtree totals.

**Why bottom-up works**: The recursive call naturally finds the deepest imbalance first. If a child's subtree is internally balanced but its parent's disc is not, the error is at the parent level. The `Err` short-circuit ensures only the first (deepest) imbalance is reported.

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 288.71µs |

**Performance breakdown**: ~1,400 programs parsed, one tree traversal for root finding (O(n) with HashSet), one recursive tree traversal for weight balancing (O(n) visiting each node once).

**Dominant cost**: HashMap operations during parsing (~1,400 insertions) and HashSet operations during root finding (~1,400 insertions + lookups).

**Why no optimization needed**: 289µs is well under the 100ms budget.

---

## Key Patterns

### Tree from flat input (implicit parent-child)

The input describes a tree without explicit parent pointers — each node lists its children. Finding the root requires computing which node has no parent (not in any child list). This "orphan detection" pattern appears whenever you need to find a source node in a DAG.

### Bottom-up recursion with early exit

The `Result<u32, u32>` return type elegantly combines two purposes:
- `Ok` = normal computation (subtree weight)
- `Err` = answer found, propagate immediately

This avoids maintaining a mutable "answer found" variable or returning `Option` with separate "found" logic. The `?` operator on recursive calls provides natural short-circuiting.

### Minority detection with frequency grouping

Finding the "odd one out" among N values where exactly one differs:
```rust
let mut freq: HashMap<u32, Vec<&str>> = HashMap::new();
// Group with len==1 is the wrong one
```

This generalizes to any "find the outlier" problem. Alternative: if you know N≥3, you can compare the first three values to determine which group is the majority — but the HashMap approach is cleaner and handles edge cases.

### Zero-copy parsing with lifetimes

Using `&'a str` borrows from the input string avoids allocating `String` for each program name. The `Program<'a>` struct's lifetime ties it to the input, making the borrow relationship explicit. This is idiomatic for parsers that don't need to outlive their input.

---

## Q&A — DFS Traversal Analysis

### Is the recursive search depth-first or breadth-first?

**Depth-first.** The recursion in `total_weight` goes all the way down to leaves before checking balance at each level on the way back up. The `?` operator propagates the first `Err` found at the deepest imbalanced node back up the call stack immediately, skipping remaining siblings.

A breadth-first approach would use a queue and process level-by-level, which would be more complex here since subtree totals require knowing children's totals first — naturally a bottom-up/DFS computation.

### How does the recursion flow?

Hit a leaf → return its weight → back up one level → now you have all sibling weights to compare. If balanced, sum them up and return to the next level up. The imbalance is only detectable when you're back at the parent with all children's totals in hand.

### How deep is the tree, and where was the error?

Depth is measured from the root (root = depth 0):

```
Depth 0: airlri (root)          ← 7 children
Depth 1: tylelk, ...
Depth 2: tylelk's disc          ← IMBALANCE FOUND HERE
Depth 3: ...
Depth 4: ...
Depth 5: leaves (deepest)
```

The tree is **6 levels deep** (depth 0–5). The imbalance was near the **top** at depth 2.

### Does the `?` short-circuit actually help?

With debug instrumentation, the actual puzzle visited **141 of 1,418 nodes (9.9%)** — skipping over 90% of the tree. The DFS happened to explore the branch containing the imbalance first, and the `?` bailed out before touching the other branches.

| | Example | Actual puzzle |
|---|---|---|
| **Imbalance depth** | 0 (root's disc) | 2 (two levels deep) |
| **Nodes visited** | 13/13 (100%) | 141/1,418 (9.9%) |
| **Short-circuit savings** | None — root level, needed all children | 90% skipped |

### Was the 9.9% traversal guaranteed?

No — pure luck of iteration order. `HashMap` doesn't guarantee key ordering, so which branch DFS explores first is essentially random. The root has 7 children — if the branch containing the imbalance had been iterated last instead of first, we'd have visited ~90% of the tree before finding it.

- **Worst case**: All 1,418 nodes visited (100%), same as the example
- **Best case**: Just the path from root to the imbalanced node plus its children (what happened here)

The algorithm's **correctness** doesn't depend on the order, only **performance**. A breadth-first approach would have the opposite luck profile — shallow imbalances found fast (check all of depth 2 before depth 3), but always visiting every node at shallower depths first regardless of where the error is.
