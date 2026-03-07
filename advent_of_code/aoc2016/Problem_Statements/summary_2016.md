# AoC 2016 - Summary

**Status**: In Progress (6/25 complete)

---

## Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 6/25 |
| **Total Runtime** | 262.5ms |
| **Average per Day** | 43.8ms |
| **Fastest Day** | Day 6 (8.1us) |
| **Slowest Day** | Day 5 (262ms) |
| **Mission Integration** | None yet |
| **Patterns Extracted** | 13 |
| **Optimizations Applied** | Parse-once, byte-level hash checks, Rayon parallelization, single-pass, fixed-size freq arrays |

**1-Second Goal**: 262.5ms / 1,000ms (26.3% used after 6 days) --- comfortably on track

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
| [7](days/day07.md) | - | - | - | - | - | |
| [8](days/day08.md) | - | - | - | - | - | |
| [9](days/day09.md) | - | - | - | - | - | |
| [10](days/day10.md) | - | - | - | - | - | |
| [11](days/day11.md) | - | - | - | - | - | |
| [12](days/day12.md) | - | - | - | - | - | |
| [13](days/day13.md) | - | - | - | - | - | |
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
- [Day 7](days/day07.md) - Internet Protocol Version 7
- [Day 8](days/day08.md) - Two-Factor Authentication
- [Day 9](days/day09.md) - Explosives in Cyberspace
- [Day 10](days/day10.md) - Balance Bots
- [Day 11](days/day11.md) - Radioisotope Thermoelectric Generators
- [Day 12](days/day12.md) - Leonardo's Monorail
- [Day 13](days/day13.md) - A Maze of Twisty Little Cubicles
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

---

## Mission Integration

| Mission | Days Used | Components |
|---------|-----------|------------|
| Mission 5 (HashMap) | - | - |
| Mission 6 (Grid) | - | - |
| Mission 8 (Graph) | - | - |
| Mission 10 (Union-Find) | - | - |

---

**Last Updated**: 2026-03-06 (Day 6 complete)
