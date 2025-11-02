# AoC 2015 Day 24 – Detailed Code Walkthrough

This walkthrough explains the Day 24 solver function-by-function, including algorithmic intent, pruning, and common pitfalls avoided. File references point to the exact places in the code for quick navigation.

Related files:
- Solver: `advent_of_code/aoc2015/src/solver/day24.rs`
- Trace runner: `advent_of_code/aoc2015/examples/day24_execution_trace.rs`
- Code walkthrough runner: `advent_of_code/aoc2015/examples/day24_code_walkthrough.rs`

How to run:
- Unit tests (Day 24 only): `cargo test -p aoc2015 day24 -- --nocapture`
- Execution trace: `cargo run -p aoc2015 --example day24_execution_trace`
- Walkthrough (prints candidates and QE): `cargo run -p aoc2015 --example day24_code_walkthrough`

---

## Problem Recap

Split packages into equal-weight groups (3 groups for Part 1, 4 for Part 2). Choose the first group with the fewest packages; if tied, pick the one with minimal quantum entanglement (product of weights). After selecting the first group, the remaining packages must be partitionable into the other groups of the same target weight.

---

## Function-by-Function

### parse_weights
- Location: `advent_of_code/aoc2015/src/solver/day24.rs:33`
- Signature: `pub fn parse_weights(input: &str) -> Result<Vec<u64>>`
- Purpose: Parse newline-separated positive integers into `Vec<u64>`.
- Behavior: Trims lines and parses; returns an error on any invalid number.

### quantum_entanglement
- Location: `advent_of_code/aoc2015/src/solver/day24.rs:45`
- Signature: `pub fn quantum_entanglement(group: &[u64]) -> u64`
- Purpose: Compute the product of weights for tie-breaking.
- Notes: Uses `Iterator::product()`. Inputs are small enough that `u64` is safe for AoC constraints.

### find_subsets (exact-size combination generation)
- Location: `advent_of_code/aoc2015/src/solver/day24.rs:50`
- Signature: `pub fn find_subsets(weights: &[u64], target: u64, exact_size: usize) -> Vec<Vec<usize>>`
- Purpose: Enumerate all index-combinations of size `exact_size` whose elements sum to `target`.
- Important design choices:
  - Returns indices, not values. This avoids ambiguity when values repeat and makes removal by position precise.
  - Pruning: If `current_sum + next_weight > target`, skip; if `current_sum >= target` or `len >= exact_size`, stop.
  - Monotone build: Iterate `i` from `start..len` so combinations are unique and ordered.
- Complexity: Worst-case combinatorial; pruning plus descending input order keeps it tractable for problem sizes.

### can_partition_remaining (k-way partition verification)
- Location: `advent_of_code/aoc2015/src/solver/day24.rs:100`
- Signature: `pub fn can_partition_remaining(packages: &[u64], target: u64, remaining_groups: u64) -> bool`
- Purpose: Given a multiset of weights and `remaining_groups`, determine if they can be split into groups each summing to `target`.
- Algorithm:
  - Base case: When `remaining_groups == 1`, return true (if previous groups formed correctly, remainder must sum to `target`).
  - Sort descending for stronger pruning (big items first).
  - DFS builds one group up to `target` using a `selected: Vec<bool>` bitmap; when a group hits `target`, build the remaining pool from unselected elements and recurse with `remaining_groups - 1`.
- Why this is correct:
  - Always constructs the remaining set from the actual chosen indices, not by slicing, avoiding incorrect omissions or duplicates.
- Common pitfall avoided:
  - “Slice remainder” bug: Taking `packages[start..]` as the remainder ignores what was actually chosen; the fixed version rebuilds via `selected`.

### solve_part1 / solve_part2 (wrappers)
- Location: `advent_of_code/aoc2015/src/solver/day24.rs:159` and `:164`
- Signature: `pub fn solve_part1(input: &str) -> Result<String>` and `pub fn solve_part2(input: &str) -> Result<String>`
- Purpose: Thin wrappers that call `solve_balanced_partition` with `num_groups = 3` or `4`.

### solve_balanced_partition (core search + tie-break)
- Location: `advent_of_code/aoc2015/src/solver/day24.rs:171`
- Signature: `fn solve_balanced_partition(input: &str, num_groups: u64) -> Result<String>`
- Steps:
  1. Parse weights and compute `total`. Validate divisibility by `num_groups`. Set `target = total / num_groups`.
  2. Sort weights descending for better pruning.
  3. For `size` from 1..N:
     - Enumerate all `exact_size = size` combinations via `find_subsets` that sum to `target`.
     - For each combination, build the chosen group and create the remaining pool by index.
     - If `can_partition_remaining(remaining, target, num_groups - 1)` holds, evaluate QE and keep the best for this size.
     - If any candidate worked for this `size`, stop (minimal size established); return minimal QE across them.
  4. Error if no valid partition found.
- Guarantees:
  - Minimal first-group cardinality by construction (outer loop over `size`).
  - Minimal QE tie-break within the minimal cardinality class.

---

## Complexity and Pruning

- Subset generation is combinatorial but pruned by the target sum and the descending order.
- Partition check recurses over groups; base-case short-circuits when only one group remains.
- Early exit when a minimal-size candidate exists prevents exploring larger sizes.

Practical result: Handles AoC input sizes comfortably without additional heavy optimizations.

---

## Pitfalls and Fixes

- Incorrect remainder construction (slicing) can drop or duplicate items. Fixed by tracking and removing chosen indices.
- Returning as soon as one subset is found can miss valid partitions; the current approach backtracks and tries alternatives as needed.

---

## Tracing and Exploration

- Execution trace: prints totals, targets, and final results
  - `advent_of_code/aoc2015/examples/day24_execution_trace.rs`
- Code walkthrough runner: prints up to 10 candidate subsets per size with QE and feasibility
  - `advent_of_code/aoc2015/examples/day24_code_walkthrough.rs`

Suggested sequence:
1. Run the walkthrough to see candidate enumeration and pruning in action.
2. Run the execution trace to confirm final answers on the full input.

