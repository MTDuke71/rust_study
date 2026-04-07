# AoC 2017 - Summary

**Status**: IN PROGRESS (1/25)
**Project**: [README](../README.md)

---

## Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 6/25 (12 stars) |
| **Total Runtime** | 35.33ms |
| **Average per Day** | 5.89ms |
| **Mission Integration** | None yet |

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

---

## Quick Navigation

**By Day**:
- [Day 1](days/day01_function_guide.md) - Inverse Captcha | [Code](../src/solver/day01.rs) ✅
- [Day 2](days/day02_function_guide.md) - Corruption Checksum | [Code](../src/solver/day02.rs) ✅
- [Day 3](days/day03_function_guide.md) - Spiral Memory | [Code](../src/solver/day03.rs) ✅
- [Day 4](days/day04_function_guide.md) - High-Entropy Passphrases | [Code](../src/solver/day04.rs) ✅
- [Day 5](days/day05_function_guide.md) - A Maze of Twisty Trampolines | [Code](../src/solver/day05.rs) ✅
- [Day 6](days/day06_function_guide.md) - Memory Reallocation | [Code](../src/solver/day06.rs) ✅

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
