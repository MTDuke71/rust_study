# AoC 2017 - Summary

**Status**: IN PROGRESS (21/25)
**Project**: [README](../README.md)

---

## Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 21/25 (42 stars) |
| **Total Runtime** | 324.40ms |
| **Average per Day** | 15.45ms |
| **Mission Integration** | Mission 6 (Day 19), Mission 10 (Days 12, 14) |

---

## Performance Table

| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [1](days/day01_function_guide.md) | — | — | 4.26µs | Modular circular comparison | None | Single-pass filter+sum per part |
| [2](days/day02_function_guide.md) | — | — | 4.87µs | Min/max + pairwise divisibility | None | Pair enumeration with index slicing |
| [3](days/day03_function_guide.md) | — | — | 9.09µs | Ring math + spiral neighbor-sum | None | O(√n) closed-form Part 1, HashMap spiral Part 2 |
| [4](days/day04_function_guide.md) | — | — | 360.49µs | HashSet dedup + byte-sort canonicalization | None | Short-circuit `.all()`, zero-copy parsing |
| [5](days/day05_function_guide.md) | — | — | 34.09ms | In-place jump offset simulation | None | Conditional mutation → 74× step increase |
| [6](days/day06_function_guide.md) | — | — | 861.60µs | HashMap cycle detection + division redistribution | None | 6,681 cycles; O(n) with HashMap state tracking |
| [7](days/day07_function_guide.md) | — | — | 288.71µs | Tree root finding + recursive weight balancing | None | Bottom-up Result<Ok,Err> for early exit on imbalance |
| [8](days/day08_function_guide.md) | — | — | 164.98µs | Conditional register machine simulation | None | Single-pass dual-answer with pre-computed deltas |
| [9](days/day09_function_guide.md) | — | — | 9.17µs | State machine stream parsing | None | O(1) space, no intermediate representation |
| [10](days/day10_function_guide.md) | — | — | 54.92µs | Circular list reversal hash | None | 64-round Knot Hash, XOR dense compression |
| [11](days/day11_function_guide.md) | — | — | 37.40µs | Hex grid cube coordinates | None | Distance = max(|q|,|r|,|s|), track max during walk |
| [12](days/day12_function_guide.md) | — | — | 150.98µs | Union-Find connected components | Mission 10 | `members(0).count()` + `components().count()` from one UF |
| [13](days/day13_function_guide.md) | — | — | 13.01ms | Modular arithmetic (no simulation) | None | Scanner period = 2*(R-1); closed-form catch test |
| [14](days/day14_function_guide.md) | — | — | 3.47ms | Knot Hash rows + Union-Find regions | Mission 10 | Bit-packed 128×128 grid; right+down unioning |
| [15](days/day15_function_guide.md) | — | — | 22.73ms | Park–Miller LCG + Rayon blocks | None | Mersenne fast-mod + jump-ahead parallelism (14.7× from naive) |
| [16](days/day16_function_guide.md) | — | — | 4.82ms | Permutation cycle detection + history lookup | None | Cycle length 60; 960-byte lookup table eliminates recomputation |
| [17](days/day17_function_guide.md) | — | — | 131.84µs | Circular-buffer insertion + position-1 tracking | None | Batch no-wrap iterations → 937× speedup (50M → ~4.3k wraps) |
| [18](days/day18_function_guide.md) | 3.05µs | 406.58µs | 400.13µs | Assembly interpreter + coroutine scheduling | None | Part 1 = last LCG sample (3,188); Part 2 = 56 × 127 distributed bubble-sort sends |
| [19](days/day19_function_guide.md) | 85.50µs | 85.26µs | 89.97µs | ASCII-maze walk with cardinal turns | Mission 6 | One traversal yields both answers; `Option<Coord>` unifies move + boundary |
| [20](days/day20_function_guide.md) | 214.84µs | 2.115ms | 2.124ms | Closed-form ranking + bucket-filter simulation | None | Part 1 is `min_by_key(\|a\|,\|v\|,\|p\|)` — no simulation; Part 2 buckets by position, retain count==1 |
| [21](days/day21_function_guide.md) | 555.89µs | 234.25ms | 241.89ms | Dihedral-group rule expansion + block step | None | Reference; alternates: `day21_bitpacked` 5.68ms (40×), `day21_memo` **32.46µs (7,070×)** via 3-iter cache — grid never materialized |

---

## Quick Navigation

**By Day**:
- [Day 1](days/day01_function_guide.md) - Inverse Captcha | [Code](../src/solver/day01.rs) ✅
- [Day 2](days/day02_function_guide.md) - Corruption Checksum | [Code](../src/solver/day02.rs) ✅
- [Day 3](days/day03_function_guide.md) - Spiral Memory | [Code](../src/solver/day03.rs) ✅
- [Day 4](days/day04_function_guide.md) - High-Entropy Passphrases | [Code](../src/solver/day04.rs) ✅
- [Day 5](days/day05_function_guide.md) - A Maze of Twisty Trampolines | [Code](../src/solver/day05.rs) ✅
- [Day 6](days/day06_function_guide.md) - Memory Reallocation | [Code](../src/solver/day06.rs) ✅
- [Day 7](days/day07_function_guide.md) - Recursive Circus | [Code](../src/solver/day07.rs) ✅
- [Day 8](days/day08_function_guide.md) - I Heard You Like Registers | [Code](../src/solver/day08.rs) ✅
- [Day 9](days/day09_function_guide.md) - Stream Processing | [Code](../src/solver/day09.rs) ✅
- [Day 10](days/day10_function_guide.md) - Knot Hash | [Code](../src/solver/day10.rs) ✅
- [Day 11](days/day11_function_guide.md) - Hex Ed | [Code](../src/solver/day11.rs) ✅
- [Day 12](days/day12_function_guide.md) - Digital Plumber | [Code](../src/solver/day12.rs) ✅
- [Day 13](days/day13_function_guide.md) - Packet Scanners | [Code](../src/solver/day13.rs) ✅
- [Day 14](days/day14_function_guide.md) - Disk Defragmentation | [Code](../src/solver/day14.rs) ✅
- [Day 15](days/day15_function_guide.md) - Dueling Generators | [Code](../src/solver/day15.rs) ✅
- [Day 16](days/day16_function_guide.md) - Permutation Promenade | [Code](../src/solver/day16.rs) ✅
- [Day 17](days/day17_function_guide.md) - Spinlock | [Code](../src/solver/day17.rs) ✅
- [Day 18](days/day18_function_guide.md) - Duet | [Code](../src/solver/day18.rs) ✅
- [Day 19](days/day19_function_guide.md) - A Series of Tubes | [Code](../src/solver/day19.rs) ✅
- [Day 20](days/day20_function_guide.md) - Particle Swarm | [Code](../src/solver/day20.rs) ✅
- [Day 21](days/day21_function_guide.md) - Fractal Art | [Code](../src/solver/day21.rs) ✅

---

## Algorithms Used

| Day | Algorithm | Key Insight |
|-----|-----------|-------------|
| 1 | Modular circular comparison | Same pattern for both parts — only the offset changes (1 vs len/2) |
| 2 | Min/max + pairwise divisibility | Normalize pair ordering (big/small) to simplify division check |
| 3 | Ring math + spiral neighbor-sum | Part 1 is pure math (O(√n)), Part 2 needs simulation but converges in ~50 steps |
| 4 | HashSet dedup + byte-sort canonicalization | Anagrams share the same sorted bytes — reduce to duplicate detection |
| 5 | In-place jump offset simulation | One branch difference (≥3 decrement) changes 374K steps to 27.7M |
| 6 | HashMap cycle detection + division redistribution | Store step numbers in HashMap → cycle length is a simple subtraction |
| 7 | Tree root finding + recursive weight balancing | Result<Ok,Err> gives natural short-circuit; minority detection finds the outlier |
| 8 | Conditional register machine simulation | Pre-compute delta at parse time; track all-time max during single execution pass |
| 9 | State machine stream parsing | Three states (normal, garbage, escaped) — no intermediate representation, O(1) space |
| 10 | Circular list reversal hash | Dual input interpretation (numbers vs ASCII); state persists across 64 rounds |
| 11 | Hex grid cube coordinates | Cube coords make distance trivial — cancellation is implicit in accumulation |
| 12 | Union-Find connected components | Mission 10 reuse — `members(0)` for Part 1, `components()` for Part 2 from one UF |
| 13 | Modular arithmetic (closed-form) | Scanner at top iff (depth + delay) mod 2*(R-1) = 0 — no per-picosecond simulation needed |
| 14 | Knot Hash rows + Union-Find regions | Compose Day 10 + Mission 10 — bit-packed grid, union right+down only to halve edge work |
| 15 | Mersenne fast-mod + Rayon jump-ahead | Mersenne prime `2^31-1` eliminates DIV (1.8×); `mod_pow` jump-ahead enables parallel blocks (3.9× total) |
| 16 | Permutation cycle detection + history lookup | Any permutation has finite order; cycle of 60 means 1B dances reduces to a table lookup |
| 17 | Circular-buffer insertion + position-1 tracking | Position 0 never moves — only track the last value written to position 1; batch no-wrap iterations to skip 50M → ~4.3k |
| 18 | Assembly interpreter + coroutine scheduling | Same `Instr` enum drives both parts; Part 2 alternates two programs with FIFO queues and detects deadlock by "both blocked and both inboxes empty" |
| 19 | ASCII-maze walk with cardinal turns | Letters and step count are both side effects of a single traversal — combined runtime equals a single walk, not 2× |
| 20 | Closed-form ranking + bucket-filter simulation | Long-term position ~ ½at² → Part 1 is just `min_by_key` on acceleration magnitude; Part 2 uses `HashMap<Pos, count>` then `retain(count == 1)` to avoid O(n²) pair checks |
| 21 | Dihedral-group rule expansion + block step + 3-iter memoization | 8 orientations (4 rotations × 2 flips) is the dihedral group D₄; expand at parse so lookups become one `HashMap::get`; grid side triples every 3 iterations (`3 → 4 → 6 → 9`, cycle gain = 4/3 × 3/2 × 3/2 = 3), so 18 iter → 2187² = 3¹⁴ ≈ 4.78M cells. Memo alternate: 3-iter structural periodicity means `f(block, depth)` recurses with ≤ 512 distinct 3×3 blocks — Part 2 drops from 230ms to **32µs** (7,070×) without materializing the grid |
