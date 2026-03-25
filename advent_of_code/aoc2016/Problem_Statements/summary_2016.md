# AoC 2016 - Summary

**Status**: COMPLETE (25/25 -- 50 stars!)
**Project**: [README](../README.md)

---

## Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 25/25 (50 stars) |
| **Total Runtime** | 1074ms |
| **Average per Day** | 43.0ms |
| **Fastest Day** | Day 19 (200ns) |
| **Slowest Day** | Day 14 (627ms) |
| **Mission Integration** | Day 8 (Mission 6 Grid) |
| **Patterns Extracted** | 59 |
| **Optimizations Applied** | Parse-once, byte-level hash checks, Rayon parallelization, single-pass, fixed-size freq arrays, sliding window, bracket state machine, screen simulation, slice recursion, event-driven simulation, canonical BFS, register VM, implicit graph BFS, hash caching, generic hash function, CRT, extended GCD, dragon curve expansion, single-pass parity checksum, path-dependent BFS, XOR transition simplification, Josephus closed-form, power-of-3 piecewise formula, interval merging, byte-level string ops, brute-force inverse, sliding puzzle geometry, BFS pathfinding, pairwise BFS + TSP permutations, binary pattern analysis, VM signal simulation |

**1-Second Goal**: 1074ms / 1,000ms (107.4% -- 74ms over budget, dominated by Days 5+14 MD5 mining)

---

## Performance Table

| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [1](days/day01_function_guide.md) | 1.6µs | 13.2µs | 13.5µs | Coordinate walk + HashSet | None | Part 2 walks block-by-block |
| [2](days/day02_function_guide.md) | 6.7µs | 9.8µs | 20.3µs | Keypad walk + sentinel bounds | None | Generic `decode` serves both parts |
| [3](days/day03_function_guide.md) | 119.4µs | 118.9µs | 121.3µs | Triangle inequality + chunks | None | Parse dominates; sort vs direct identical |
| [4](days/day04_function_guide.md) | 306.8us | 335.3us | 331.8us | Frequency sort + Caesar cipher | None | Pre-filter real rooms; zero-copy `&str` parsing |
| [5](days/day05_function_guide.md) | - | - | 262ms | MD5 mining + Rayon | None | Single-pass + parallel batches; 15.5x speedup over naive |
| [6](days/day06_function_guide.md) | 8.4us | 7.9us | 8.1us | Column frequency + `[u32;26]` | None | Reuses Day 4 freq array pattern; zero-filter for min |
| [7](days/day07_function_guide.md) | 255us | 436us | 713us | Sliding window + bracket state machine | None | P1 streams zero-alloc; P2 collects segments for cross-match |
| [8](days/day08_function_guide.md) | 17.7µs | 18.3µs | 18.2µs | Screen simulation + modular rotation | Mission 6 | First mission integration; interactive viz with crossterm |
| [9](days/day09_function_guide.md) | 157ns | 17.3µs | 17.5µs | Pointer walk + slice recursion | None | Length-only: 11.6B chars computed without building string |
| [10](days/day10_function_guide.md) | 81.4µs | 70.7µs | 69.0µs | Event-driven simulation | None | HashMap mailbox pattern; bots fire when holding 2 chips |
| [11](days/day11_function_guide.md) | 4.32ms | 25.98ms | 30.57ms | BFS + canonical state | None | Pair symmetry reduces 1B→400K states |
| [12](days/day12_function_guide.md) | 1.57ms | 44.53ms | 46.17ms | Register VM | None | Assembunny computes fib(28)+306 / fib(35)+306; trace mode for debugging |
| [13](days/day13_function_guide.md) | 18.9µs | 19.0µs | 18.9µs | BFS on implicit graph | None | Procedural maze via popcount; single BFS answers both parts |
| [14](days/day14_function_guide.md) | 25.4ms | 571ms | 593ms | MD5 triplet/quintuplet mining + Rayon | None | Key stretching (2017× MD5); ~10× Rayon speedup |
| [15](days/day15_function_guide.md) | 1.29µs | 1.40µs | 1.50µs | CRT (Chinese Remainder Theorem) | None | 2,043× faster than brute force; extended GCD for modular inverse |
| [16](days/day16_function_guide.md) | 244ns | 12.2ms | 12.2ms | Dragon curve + early-stop fill + parity checksum | None | 35M-bit fill; 4.4× speedup total; O(1) example: 2.1µs |
| [17](days/day17_function_guide.md) | 13.2µs | 22.9ms | 22.9ms | BFS with MD5-based door states | None | No visited set needed; path-dependent hash makes every state unique |
| [18](days/day18_function_guide.md) | 3.9µs | 1.09ms | 1.10ms | 1D cellular automaton + u128 bit-parallel | None | Bitset implementation: 35× faster than vector (38.5ms → 1.09ms); uses popcount for trap counting |
| [19](days/day19_function_guide.md) | ~0ns | ~0ns | 73ns | Josephus closed-form + power-of-3 formula | None | O(1) math — no simulation; fastest day at 73ns |
| [20](days/day20_function_guide.md) | 41.3us | 41.6us | 42.5us | Interval merging | None | Sort + merge 1005 ranges; u64 overflow guard |
| [21](days/day21_function_guide.md) | 11.7us | 16.2us | 16.1us | String scramble/unscramble | None | 6 op types; brute-force rotate-based reversal |
| [22](days/day22_function_guide.md) | 487us | 194us | 517us | Sliding puzzle + BFS | None | O(n^2) viable pairs; geometry for 5-move slide cycle |
| [23](days/day23_function_guide.md) | 35.7us | 30.9us | 63.6us | Self-modifying assembunny VM + mul-loop optimization | Day 12 VM | tgl instruction; computes a! + C; mul-loop detect avoids 479M iterations |
| [24](days/day24_function_guide.md) | — | — | 480us | Pairwise BFS + brute-force TSP | None | 8 points, 7! perms; BFS dominates |
| [25](days/day25_function_guide.md) | — | — | 6.25ms | Assembunny VM + binary pattern search | Day 12/23 VM | Outputs binary digits of `a+2550`; alternating-bit pattern |

---

## Quick Navigation

**By Day**:
- [Day 1](days/day01_function_guide.md) - No Time for a Taxicab | [Code](../src/solver/day01.rs) ✅
- [Day 2](days/day02_function_guide.md) - Bathroom Security | [Code](../src/solver/day02.rs) ✅
- [Day 3](days/day03_function_guide.md) - Squares With Three Sides | [Code](../src/solver/day03.rs) ✅
- [Day 4](days/day04_function_guide.md) - Security Through Obscurity | [Code](../src/solver/day04.rs) ✅
- [Day 5](days/day05_function_guide.md) - How About a Nice Game of Chess? | [Code](../src/solver/day05.rs) ✅
- [Day 6](days/day06_function_guide.md) - Signals and Noise | [Code](../src/solver/day06.rs) ✅
- [Day 7](days/day07_function_guide.md) - Internet Protocol Version 7 | [Code](../src/solver/day07.rs) ✅
- [Day 8](days/day08_function_guide.md) - Two-Factor Authentication | [Code](../src/solver/day08.rs) | [Viz](../examples/day08_viz.rs) ✅
- [Day 9](days/day09_function_guide.md) - Explosives in Cyberspace | [Code](../src/solver/day09.rs) ✅
- [Day 10](days/day10_function_guide.md) - Balance Bots | [Code](../src/solver/day10.rs) ✅
- [Day 11](days/day11_function_guide.md) - Radioisotope Thermoelectric Generators | [Code](../src/solver/day11.rs) ✅
- [Day 12](days/day12_function_guide.md) - Leonardo's Monorail | [Code](../src/solver/day12.rs) ✅
- [Day 13](days/day13_function_guide.md) - A Maze of Twisty Little Cubicles | [Code](../src/solver/day13.rs) ✅
- [Day 14](days/day14_function_guide.md) - One-Time Pad | [Code](../src/solver/day14.rs) ✅
- [Day 15](days/day15_function_guide.md) - Timing is Everything | [Code](../src/solver/day15.rs) ✅
- [Day 16](days/day16_function_guide.md) - Dragon Checksum | [Code](../src/solver/day16.rs) ✅
- [Day 17](days/day17_function_guide.md) - Two Steps Forward | [Code](../src/solver/day17.rs) ✅
- [Day 18](days/day18_function_guide.md) - Like a Rogue | [Code](../src/solver/day18.rs) ✅
- [Day 19](days/day19_function_guide.md) - An Elephant Named Joseph | [Code](../src/solver/day19.rs) ✅
- [Day 20](days/day20_function_guide.md) - Firewall Rules | [Code](../src/solver/day20.rs) ✅
- [Day 21](days/day21_function_guide.md) - Scrambled Letters and Hash | [Code](../src/solver/day21.rs) ✅
- [Day 22](days/day22_function_guide.md) - Grid Computing | [Code](../src/solver/day22.rs) ✅
- [Day 23](days/day23_function_guide.md) - Safe Cracking | [Code](../src/solver/day23.rs) ✅
- [Day 24](days/day24_function_guide.md) - Air Duct Spelunking | [Code](../src/solver/day24.rs) ✅
- [Day 25](days/day25_function_guide.md) - Clock Signal | [Code](../src/solver/day25.rs) ✅

---

## Algorithms Used

| Day | Algorithm | Key Insight |
|-----|-----------|-------------|
| 1 | Coordinate walk + HashSet | Part 1 = jump by steps; Part 2 = step one-at-a-time, detect revisit |
| 2 | Keypad walk + sentinel bounds | Generic `decode` with `b'.'` sentinels handles rectangular and diamond keypads identically |
| 3 | Triangle inequality + chunks(3) | Sort+1 check vs direct 3-check: identical perf because parsing dominates |
| 4 | Frequency sort + Caesar cipher | `[u32;26]` array beats HashMap for 26-letter alphabet; zero-copy `&str` parsing |
| 5 | MD5 mining + Rayon parallel batches | Single-pass dual extraction + parallel batch mining; 15.5x speedup (4.05s -> 262ms) |
| 6 | Column frequency + fixed-size array | Same `[u32;26]` pattern as Day 4; max vs min extraction for Part 1 vs Part 2 |
| 7 | Sliding window + bracket state machine | `windows(4)` for ABBA, `windows(3)` for ABA/BAB; track bracket state to split supernet/hypernet |
| 8 | Screen simulation + modular rotation | Apply rect/rotate to `Grid<bool>`; modular arithmetic for wrap-around; Part 2 reads pixel art letters |
| 9 | Pointer walk + slice recursion | Byte-level scan with manual index jumps; Part 2 recurses on sub-slices to handle nested markers |
| 10 | Event-driven simulation | Bots fire when holding 2 chips; HashMap as mailbox; parameterized target search |
| 11 | BFS + canonical state space | Sort (gen, chip) pairs to exploit element symmetry; reduces 1B states to ~400K |
| 12 | Register VM (assembunny) | Program computes Fibonacci + constant; VM with step/trace capability for debugging |
| 13 | BFS on implicit graph | Maze generated by formula + popcount; no grid stored; single BFS for shortest path + flood fill |
| 14 | MD5 triplet/quintuplet mining + Rayon | Hash salt+index, find triplets confirmed by quintuplets in next 1000; key stretching (2017× MD5) for Part 2 |
| 15 | Chinese Remainder Theorem | System of linear congruences solved via extended GCD; 2,043× faster than brute-force scan |
| 16 | Dragon curve + checksum | Modified dragon curve doubles+1 data; checksum pairs bits until odd length; 35M bits in 51ms |
| 17 | BFS with MD5-based door states | Path-dependent hash means states are `(position, path)`; no global visited set is needed |
| 18 | 1D cellular automaton + XOR transition | Trap rule reduces to `left ^ right`; reusing two row buffers gives O(width × rows) time and O(width) space |
| 19 | Josephus closed-form + power-of-3 formula | Part 1: binary bit rotation (k=2); Part 2: piecewise formula resets at powers of 3 |
| 20 | Interval merging (sort + sweep) | Sort ranges by start, merge overlapping/adjacent, query gaps; u64 for overflow safety |
| 21 | String scramble + reverse operations | 6 op types on Vec<u8>; Part 2 reverses ops; brute-force trial for rotate-based inverse |
| 22 | Sliding puzzle + BFS geometry | BFS empty node around wall; 5-move cycle slides goal left; O(n^2) viable pairs |
| 24 | Pairwise BFS + brute-force TSP | 8× BFS builds distance matrix; 7! permutations find optimal route; Part 2 adds return edge |
| 25 | Assembunny VM + binary pattern search | Program outputs binary digits of `a+2550` LSB-first; find smallest `a` giving `0,1,0,1,...` pattern |

---

## Patterns Catalog

| Pattern | Days | Description |
|---------|------|-------------|
| Parse-once | 1, 2, 3, 4, 6 | Parse instructions once, reuse parsed data for both parts |
| Sentinel padding | 2 | Use `b'.'` to pad irregular shapes into rectangles for uniform bounds checking |
| Array destructuring | 3 | Destructure `&[a, b, c]` in function signature for clean element access |
| chunks(n) regrouping | 3 | Re-read row-major data as column-major using `chunks(3).flat_map()` |
| Fixed-size freq array | 4, 6 | `[u32; 26]` beats HashMap for small known character sets |
| Zero-copy `&str` parsing | 4 | Borrow slices from input instead of allocating `String`s |
| Byte-level hash check | 5 | Check raw bytes instead of hex string conversion for leading-zero detection |
| Nibble extraction | 5 | Bitwise `& 0x0F` and `>> 4` to read individual hex characters from hash bytes |
| Option array filling | 5 | `[Option<char>; 8]` for first-wins positional password assembly |
| Rayon parallel batches | 5 | Partition index space into chunks, `par_iter` each, sort results to restore order |
| Single-pass dual extraction | 5 | Mine once, fill both passwords simultaneously; Part 1 rides free while Part 2 mines |
| Zero-filter for min | 6 | `.filter(\|c\| c > 0)` prevents unused array slots from winning min selection |
| Dual aggregation | 6 | Same parsed data, different reduction (max vs min) for each part |
| Sliding window | 7 | `windows(n)` for fixed-size palindrome detection (ABBA, ABA) |
| Bracket state machine | 7 | Track `in_bracket` + `start` index to segment lines without splitting/allocating |
| Early rejection | 7 | ABBA inside hypernet → immediate `false` (skip remaining segments) |
| Screen simulation | 8 | Apply instructions sequentially to mutable grid — common in AoC "pixel art" problems |
| Modular rotation | 8 | `(index + amount) % dimension` for circular shift with natural wrap-around |
| Temporary copy for rotation | 8 | Copy row/column before writing back to avoid read-after-write corruption |
| Byte-level scanning | 9 | Work on `&[u8]` for direct indexing and ASCII byte comparisons |
| Pointer walk with variable jumps | 9 | Manual index `i` with skip-ahead — more natural than iterators for marker parsing |
| Length-only computation | 9 | Count output bytes without building the string — essential for billion-byte outputs |
| Slice recursion | 9 | Zero-copy `&data[start..end]` as recursive input for nested marker expansion |
| Multiplicative nesting | 9 | Nested `(LENxREP)` markers create exponential growth from small input |
| Same algorithm, different depth | 9 | Part 1 and Part 2 share identical structure; only the counting line differs |
| Event-driven simulation | 10 | Bots fire when accumulating 2 inputs — dataflow network processing |
| BFS shortest path | 11 | Unweighted implicit graph — generate successors on the fly, never build graph |
| State canonicalization | 11 | Sort interchangeable components to collapse equivalent states (N! reduction) |
| Symmetry breaking | 11 | Element pairs are fungible — only the (gen_floor, chip_floor) pattern matters |
| Enum-tagged destinations | 10 | `Destination::Bot` vs `Output` — compiler-enforced exhaustive handling |
| Register VM | 12 | Fixed-size register array + enum instruction set — reusable for Days 23, 25 |
| Zero-cost trace mode | 12 | `tracing: bool` gates all trace recording — zero overhead when off |
| Dual-purpose Value enum | 12 | `Value::Lit` / `Value::Reg` — single type handles immediate and register operands |
| Implicit graph | 13 | No adjacency list — neighbors computed on the fly via formula |
| Hardware popcount | 13 | `count_ones()` maps to single-cycle `popcnt` instruction for wall classification |
| Dual-answer BFS | 13 | Single traversal answers both shortest-path (Part 1) and flood-fill (Part 2) |
| Hash caching | 14 | `Vec<String>` stores computed hashes to avoid recomputation during 1000-index lookahead |
| Generic hash function | 14 | `Fn(&str, u64) -> String + Sync` — same matching logic for plain and stretched hashing |
| Manual hex formatting | 14 | Nibble-to-char conversion avoids `format!` overhead in hot loop (millions of calls) |
| Batch parallel hashing | 14 | Pre-compute hash batches with Rayon `par_iter()` — embarrassingly parallel, ~10× speedup |
| Triplet/quintuplet matching | 14 | `windows(3)` finds candidates, `windows(5)` confirms within lookahead window |
| Chinese Remainder Theorem | 15 | Combine pairwise congruences via extended GCD — O(n) exact solution |
| Extended Euclidean algorithm | 15 | Compute `(gcd, x, y)` for modular inverse in CRT combine step |
| Modular arithmetic alignment | 15 | `(offset + time) % period == 0` for cyclic disc alignment |
| Clone-and-extend | 15 | Clone Part 1 data, append element for Part 2's extended problem |
| Dragon curve expansion | 16 | `a + "0" + reverse(flip(a))` — doubles+1 per step for pseudo-random fill |
| Iterative checksum halving | 16 | Pair bits (same→1, diff→0), repeat until odd length — geometric series convergence |
| XOR transition simplification | 18 | Four trap cases collapse to `left != right`, enabling branch-light next-row generation |
| Josephus closed-form (k=2) | 19 | Binary bit rotation: move leading 1 to end gives survivor position in O(1) |
| Power-of-3 piecewise formula | 19 | Across-circle elimination resets at powers of 3; linear climb by 1s then 2s between resets |
| Interval merging | 20 | Sort ranges by start, merge overlapping/adjacent in one pass — textbook sweep-line |
| u64 overflow promotion | 20 | Promote `u32` to `u64` for adjacency check to avoid overflow when range ends at `u32::MAX` |
| Enum-based instruction set | 21 | Parse text instructions into typed enum variants for clean pattern matching |
| Self-inverse operations | 21 | Swap and reverse are their own inverses — reuse `apply` for `unapply` |
| Brute-force inverse | 21 | When forward mapping isn't cleanly invertible, try all candidates and verify — feasible for small domains |
| Byte-level string ops | 21 | `Vec<u8>` gives direct indexing, `swap`, `rotate_left/right`, `reverse` from std |
| Sliding puzzle geometry | 22 | Reduce 2D grid movement to BFS + fixed-cycle formula instead of full state-space search |
| Node type classification | 22 | Threshold-based wall detection (used > 100T) separates normal, wall, and empty nodes |
| Two-phase solve | 22 | BFS for pathfinding phase, then arithmetic formula for repetitive sliding phase |
| Pairwise BFS reduction | 24 | Collapse large grid into small distance matrix between points of interest |
| Brute-force TSP | 24 | With few nodes (n<=8), 7! = 5,040 permutations is instant — no need for Held-Karp DP |
| wrapping_add for direction offsets | 24 | `r.wrapping_add(!0usize)` for -1 avoids signed arithmetic; fails bounds check naturally |
| Binary pattern analysis | 25 | Reverse-engineer VM to identify mathematical structure; `a+2550` as binary digit extractor |
| Signal verification simulation | 25 | Run VM with step limit, check output sequence against expected pattern; robust validation |

---

## Mission Integration

| Mission | Days Used | Components |
|---------|-----------|------------|
| Mission 5 (HashMap) | - | - |
| Mission 6 (Grid) | 8 | `Grid<bool>` for 50x6 pixel screen simulation |
| Mission 8 (Graph) | - | - |
| Mission 10 (Union-Find) | - | - |

---

**Last Updated**: 2026-03-25 (ALL 25 DAYS COMPLETE -- 50 stars!)
