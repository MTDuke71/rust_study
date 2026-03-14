# AoC 2016 - Summary

**Status**: In Progress (13/25 complete)

---

## Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 13/25 |
| **Total Runtime** | 340.1ms |
| **Average per Day** | 26.2ms |
| **Fastest Day** | Day 6 (8.1us) |
| **Slowest Day** | Day 5 (262ms) |
| **Mission Integration** | Day 8 (Mission 6 Grid) |
| **Patterns Extracted** | 33 |
| **Optimizations Applied** | Parse-once, byte-level hash checks, Rayon parallelization, single-pass, fixed-size freq arrays, sliding window, bracket state machine, screen simulation, slice recursion, event-driven simulation, canonical BFS, register VM, implicit graph BFS |

**1-Second Goal**: 340.1ms / 1,000ms (34.0% used after 13 days) --- comfortably on track

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
| [14](days/day14.md) | - | - | - | - | - | |
| [15](days/day15.md) | - | - | - | - | - | |
| [16](days/day16.md) | - | - | - | - | - | |
| [17](days/day17.md) | - | - | - | - | - | |
| [18](days/day18.md) | - | - | - | - | - | |
| [19](days/day19.md) | - | - | - | - | - | |
| [20](days/day20.md) | - | - | - | - | - | |
| [21](days/day21.md) | - | - | - | - | - | |
| [22](days/day22.md) | - | - | - | - | - | |
| [23](days/day23.md) | - | - | - | - | - | |
| [24](days/day24.md) | - | - | - | - | - | |
| [25](days/day25.md) | - | - | - | - | - | |

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
- [Day 14](days/day14.md) - One-Time Pad
- [Day 15](days/day15.md) - Timing is Everything
- [Day 16](days/day16.md) - Dragon Checksum
- [Day 17](days/day17.md) - Two Steps Forward
- [Day 18](days/day18.md) - Like a Rogue
- [Day 19](days/day19.md) - An Elephant Named Joseph
- [Day 20](days/day20.md) - Firewall Rules
- [Day 21](days/day21.md) - Scrambled Letters and Hash
- [Day 22](days/day22.md) - Grid Computing
- [Day 23](days/day23.md) - Safe Cracking
- [Day 24](days/day24.md) - Air Duct Spelunking
- [Day 25](days/day25.md) - Clock Signal

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

---

## Mission Integration

| Mission | Days Used | Components |
|---------|-----------|------------|
| Mission 5 (HashMap) | - | - |
| Mission 6 (Grid) | 8 | `Grid<bool>` for 50x6 pixel screen simulation |
| Mission 8 (Graph) | - | - |
| Mission 10 (Union-Find) | - | - |

---

**Last Updated**: 2026-03-13 (Day 13 complete)
