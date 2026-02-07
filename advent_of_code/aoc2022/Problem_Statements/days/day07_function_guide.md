# Day 7: No Space Left On Device - Function Guide

**Quick Links**: [← Day 6](day06_function_guide.md) | [Problem Statement](day07.md) | [Summary](../summary_2022.md) | [Day 8 →](day08_function_guide.md)

---

## Problem Overview
Parse terminal output (`cd` / `ls` commands and their results) to reconstruct a filesystem. Compute the total size of every directory (including nested contents), then:
- **Part 1**: Sum sizes of all directories with total ≤ 100,000
- **Part 2**: Find the smallest directory to delete to free enough disk space (70M total, 30M needed)

**Answer**: Part 1: `1086293` | Part 2: `366028`

## Performance Benchmarks
- **Combined**: 19.6µs
- **Parse**: Integrated (single-pass — parsing and solving happen together)

---

## Algorithm: Stack-Based Size Accumulation

### The Key Insight — No Tree Needed

The naive approach builds an explicit directory tree (nodes with children), then walks it to compute sizes. But we only need **sizes**, not structure or names.

**Observation**: The `cd` / `cd ..` commands form a natural stack. When we pop a directory (cd ..), its accumulated size rolls up to the parent. We never need to revisit a directory.

### How It Works

```
Input:              Stack (running totals):     Completed sizes:
$ cd /              [0]                         []
$ ls
14848514 b.txt      [14848514]                  []
8504156 c.dat       [23352670]                  []
dir a
$ cd a              [23352670, 0]               []
$ ls
29116 f             [23352670, 29116]           []
2557 g              [23352670, 31673]           []
62596 h.lst         [23352670, 94269]           []
dir e
$ cd e              [23352670, 94269, 0]        []
$ ls
584 i               [23352670, 94269, 584]      []
$ cd ..             [23352670, 94853]           [584]       ← e popped, added to a
$ cd ..             [23447523]                  [584, 94853] ← a popped, added to /
$ cd d              [23447523, 0]               [584, 94853]
$ ls
4060174 j           [23447523, 4060174]         [584, 94853]
...
$ cd ..             [48381165]                  [584, 94853, 24933642]
(end)               []                          [584, 94853, 24933642, 48381165]
```

**Result**: A flat `Vec<u64>` of all directory sizes. No HashMap, no String keys, no tree nodes.

---

## Function Reference

### `compute_dir_sizes(input: &str) -> Vec<u64>`
**Purpose**: Single-pass extraction of all directory sizes  
**Strategy**: Stack of running totals + pop-and-accumulate  

| Input Line | Action |
|------------|--------|
| `$ cd /` | Clear stack, push 0 (root) |
| `$ cd ..` | Pop top → add to parent → record in results |
| `$ cd <dir>` | Push 0 (new directory accumulator) |
| `$ ls` | Ignore |
| `dir <name>` | Ignore (sizes come from files) |
| `<size> <name>` | Add size to stack top (current directory) |

**End of input**: Unwind remaining stack (directories never explicitly exited).

**Complexity**: O(n) where n = number of input lines. Each line does constant work.

### `solve_part1_impl(input: &str) -> u64`
Filter sizes ≤ 100,000 and sum.

### `solve_part2_impl(input: &str) -> u64`
Calculate `need_to_free = root_size - 40,000,000`, then find smallest dir ≥ that threshold.

---

## Optimization: HashMap → Stack (23× speedup)

| Version | Technique | Combined | Key Difference |
|---------|-----------|----------|----------------|
| **v1** | HashMap<String, u64> with path keys | 455µs | String format! per file × depth |
| **v2** | Vec<u64> stack accumulation | **19.6µs** | Zero allocations beyond stack growth |

**Why v1 was slow**: For each file encountered, v1 built `format!("/{}", segments.join("/"))` path strings for every ancestor directory. With ~1,000 input lines and average depth ~5, that's ~5,000 String allocations per solve call. HashMap hashing added further overhead.

**Why v2 is fast**: A file at depth 5 does one `parse::<u64>()` and one `+= size`. No strings, no hashing, no allocation. The stack itself is a `Vec<u64>` that grows to max depth (~10-15) and stays there.

**What was preserved**: Both versions produce identical results. The insight is the same — sizes roll up from children to parents. v2 just exploits the fact that `cd ..` happens in LIFO order (it's a stack!), so we don't need to name directories at all.

---

## Conceptual Analysis

### Why This Problem Maps to a Stack

The terminal commands follow a strict **depth-first traversal** of the filesystem:
- `cd <dir>` = go deeper (push)
- `cd ..` = backtrack (pop)
- `ls` + output = process current node

This is exactly a pre-order DFS over a tree. Since DFS is inherently stack-based, a stack is the natural data structure. The HashMap approach fights this by maintaining random-access keys; the stack approach embraces it.

### The Integrator Perspective

Think of each directory as a **component under test**. When you `cd` into it, you're opening that component's test fixture. File entries are test results being accumulated. When you `cd ..`, you're closing the fixture and rolling the results up to the integration level. The stack mirrors your integration workflow: drill into a subsystem, collect data, roll up to the parent integration layer.

---

## Rust Patterns Used

| Pattern | Usage | Why |
|---------|-------|-----|
| **`Vec` as stack** | `push(0)` / `pop()` for directory entry/exit | Natural LIFO matches cd/cd.. semantics |
| **`last_mut().unwrap()`** | Accumulate file sizes to current directory | O(1) mutable access to stack top |
| **`starts_with()`** | Line classification | Fast prefix check avoids full parsing |
| **`split_ascii_whitespace()`** | Extract file size | Lazy iterator, stops after first token |
| **`saturating_sub()`** | `need_to_free` calculation | Prevents underflow if disk isn't full |
| **Iterator chains** | `.filter().copied().min()` for Part 2 | Functional style, zero allocations |

---

## Test Coverage

| Test | Validates | Expected |
|------|-----------|----------|
| `test_dir_sizes` | All 4 directory sizes from example | e=584, a=94853, d=24933642, /=48381165 |
| `test_part1_example` | Sum of dirs ≤ 100,000 | 95,437 (e + a) |
| `test_part2_example` | Smallest dir to free 8,381,165 | 24,933,642 (dir d) |

---

## Zettelkasten Connections
- Problem involves **filesystem tree traversal** — but solved without building a tree
- Stack-based accumulation pattern relates to [[DFS Patterns]]
- HashMap → stack optimization is an example of [[aoc-optimization-strategies]] Level 2 (data structure selection)
- Vec-as-stack pattern also used in Day 5 ([[sliding-window-patterns]] for rolling state concepts)

---

**Navigation**: [← Day 6](day06_function_guide.md) | [Problem Statement](day07.md) | [Summary](../summary_2022.md) | [Day 8 →](day08_function_guide.md)
