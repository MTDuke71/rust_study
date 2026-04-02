# AoC 2017 - Summary

**Status**: IN PROGRESS (1/25)
**Project**: [README](../README.md)

---

## Stats Dashboard

| Metric | Value |
|--------|-------|
| **Progress** | 2/25 (4 stars) |
| **Total Runtime** | 9.13µs |
| **Average per Day** | 4.57µs |
| **Mission Integration** | None yet |

---

## Performance Table

| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [1](days/day01_function_guide.md) | — | — | 4.26µs | Modular circular comparison | None | Single-pass filter+sum per part |
| [2](days/day02_function_guide.md) | — | — | 4.87µs | Min/max + pairwise divisibility | None | Pair enumeration with index slicing |

---

## Quick Navigation

**By Day**:
- [Day 1](days/day01_function_guide.md) - Inverse Captcha | [Code](../src/solver/day01.rs) ✅
- [Day 2](days/day02_function_guide.md) - Corruption Checksum | [Code](../src/solver/day02.rs) ✅

---

## Algorithms Used

| Day | Algorithm | Key Insight |
|-----|-----------|-------------|
| 1 | Modular circular comparison | Same pattern for both parts — only the offset changes (1 vs len/2) |
| 2 | Min/max + pairwise divisibility | Normalize pair ordering (big/small) to simplify division check |
