# Advent of Code 2015 – Highlights Summary

A distilled overview of the main patterns, algorithms, and Rust takeaways that recur across the AoC 2015 solutions in this repo.

---

## Core Problem Patterns

- Parsing & String Processing
  - Line-by-line parsing, regex-based validation, escape handling (Day 5, 8, 12, 16).
  - Byte vs UTF-8 pitfalls (Day 8) — treat input as bytes when required by the puzzle.
- Simulation on Grids and State Machines
  - 2D grids (lights, Game of Life) and step-wise updates with double-buffering (Day 6, 18).
  - Timed effects/state transitions in turn-based systems (Day 22).
- Combinatorial Optimization & Search
  - Subset sum and constrained combinations (Day 17, 24).
  - Traveling Salesman variants (Day 9) and seatings (Day 13) with symmetry/pruning.
  - A* search on implicit state graphs (Day 22) with admissible heuristics.
- Number Theory & Arithmetic Tricks
  - Divisor sums / sieve-like marking (Day 20).
  - Modular exponentiation by squaring (Day 25), diagonal indexing via triangular numbers.
- Brute Force with Pruning
  - Early exit when minimal constraints satisfied (e.g., cardinality before QE in Day 24).
  - Sort descending to prune sooner; use indices to avoid duplicate-value ambiguity.

---

## Key Algorithmic Takeaways

- Use Problem Structure Aggressively
  - Symmetry, cycles, and invariants can cut work by orders of magnitude (Day 13, 14, 19).
  - Reverse the problem when greedy becomes optimal (Day 19 reverse synthesis).
- Choose the Right Search Strategy
  - Exhaustive enumeration for small n (TSP up to ~10 nodes, subsets up to ~25 items).
  - Informed search (A*) when branching explodes and a heuristic exists (Day 22).
- Optimize Where It Matters
  - Sort for pruning, precompute bounds, early-stop when primary criterion is met.
  - Replace linear recurrences with closed forms and log-time exponentiation (Day 25).
- Validate Correctness Before Micro-optimizing
  - Build small tests and example traces; confirm logic then refine performance.

---

## Common Pitfalls (and Fixes)

- UTF-8 vs Byte Semantics (Day 8)
  - Counting code points vs bytes changes results; implement byte-oriented logic for escapes.
- Incorrect Remainder Construction (Day 24)
  - Don’t slice by ranges after choosing a subset; remove by chosen indices to form the true remainder.
- Greedy Isn’t Always Optimal (Day 13)
  - Local “weakest link” heuristics can fail; prefer full search with symmetry pruning.
- Memoization Doesn’t Always Help (Day 10)
  - When subproblems don’t repeat, caching adds overhead; simple iteration wins.

---

## Rust-Specific Lessons

- Ergonomics & Safety
  - Use `anyhow::Result` for concise error propagation; prefer explicit return types for clarity.
  - HashMap/HashSet for visited states and uniqueness; `VecDeque` for BFS when needed.
- Performance
  - Prune early with sorted inputs; avoid cloning in hot loops; prefer indices over values.
  - Use `u128` intermediates for safe modular multiplications; cast back to `u64`.
- Testing & Tooling
  - Unit tests co-located with modules; example binaries for execution traces and walkthroughs.
  - Criterion benches (e.g., Day 10) to validate optimization choices.

---

## Reusable Building Blocks in This Repo

- Subset Generators
  - Exact-size combinations summing to a target, returning indices for deduplication-safe removal (Day 24).
- Partition Validators
  - K-way equal-sum partition with recursion and base-case short-circuit (Day 24).
- Search Frameworks
  - A* abstractions via Mission9; priority queues for weighted graph exploration (Day 22).
- Parsing Helpers
  - Regex-based field extraction; robust line parsers with error messages (Day 5, 12, 16, 25).

---

## Strategy Playbook (What Worked Repeatedly)

1. Parse and Model the State Clearly
   - Keep state minimal; separate parsing from solving.
2. Start with a Correct Baseline
   - Brute force or straightforward simulation first; add tests on provided examples.
3. Add Pruning/Heuristics Guided by Constraints
   - Sort, bound, and early-exit according to primary objectives.
4. Exploit Mathematical Structure
   - Triangular numbers, modular arithmetic, divisors/multiples, cycle math.
5. Instrument and Validate
   - Log small traces; add example binaries for insight; bench when performance matters.

---

## Representative Highlights by Day

- Day 8: Bytes vs UTF‑8 — write byte-aware parsers for escape sequences.
- Day 9/13: TSP/Seating — symmetry exploitation and pruning; exact search over permutations.
- Day 14: Cycle math vs simulation — both viable; math gives faster insight.
- Day 19: Reverse greedy — structure-aware approach beats naive BFS.
- Day 22: A* with admissible heuristics — model state carefully; manage effect timers.
- Day 24: Subset sum + k‑partition — cardinality-first search, correct remainder handling, QE tie-break.
- Day 25: Diagonal indexing + modexp — closed-form index, O(log n) exponentiation.

---

## Final Thoughts

AoC 2015 rewards recognizing structure early, pairing a correct baseline with targeted pruning, and applying the right math (triangular numbers, modular arithmetic) where possible. In Rust, lean on strong types, small helpers, and example-driven exploration to iterate confidently and efficiently.

---

## Related Resources

- [[../../zettelkasten/AoC Patterns MOC]] - Problem-solving patterns catalog
- [[../examples/day22_implementation_walkthrough]] - A* search deep dive
- [[../examples/day19_README]] - Greedy algorithm analysis
- [[../../zettelkasten/Missions Overview]] - Data structure implementations (Stack, Queue, HashMap, Graph)
- [[../../daily_study/README]] - Daily learning materials aligned with AoC concepts

*Tags: #aoc #aoc2015 #algorithms #optimization #problem-solving #rust #patterns*
