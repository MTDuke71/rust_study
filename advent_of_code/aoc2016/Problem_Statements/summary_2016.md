# AoC 2016 - Summary

**Status**: In Progress (2/25 complete)

---

## Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 2/25 |
| **Total Runtime** | 33.8µs |
| **Average per Day** | 16.9µs |
| **Fastest Day** | Day 1 (13.5µs) |
| **Slowest Day** | Day 2 (20.3µs) |
| **Mission Integration** | None yet |
| **Patterns Extracted** | 2 |
| **Optimizations Applied** | Parse-once |

**1-Second Goal**: TBD

---

## Performance Table

| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [1](days/day01_function_guide.md) | 1.6µs | 13.2µs | 13.5µs | Coordinate walk + HashSet | None | Part 2 walks block-by-block |
| [2](days/day02_function_guide.md) | 6.7µs | 9.8µs | 20.3µs | Keypad walk + sentinel bounds | None | Generic `decode` serves both parts |
| [3](days/day03.md) | - | - | - | - | - | |
| [4](days/day04.md) | - | - | - | - | - | |
| [5](days/day05.md) | - | - | - | - | - | |
| [6](days/day06.md) | - | - | - | - | - | |
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
- [Day 3](days/day03.md) - Squares With Three Sides
- [Day 4](days/day04.md) - Security Through Obscurity
- [Day 5](days/day05.md) - How About a Nice Game of Chess?
- [Day 6](days/day06.md) - Signals and Noise
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

---

## Patterns Catalog

| Pattern | Days | Description |
|---------|------|-------------|
| Parse-once | 1, 2 | Parse instructions once, reuse parsed data for both parts |
| Sentinel padding | 2 | Use `b'.'` to pad irregular shapes into rectangles for uniform bounds checking |

---

## Mission Integration

| Mission | Days Used | Components |
|---------|-----------|------------|
| Mission 5 (HashMap) | - | - |
| Mission 6 (Grid) | - | - |
| Mission 8 (Graph) | - | - |
| Mission 10 (Union-Find) | - | - |

---

**Last Updated**: 2026-03-02 (Day 2 complete)
