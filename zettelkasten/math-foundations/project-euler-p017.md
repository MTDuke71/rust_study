# Project Euler P017: Number Letter Counts

**Tags**: #project-euler #combinatorics #enumeration #lookup-tables
**Date**: 2026-02-28
**Answer**: 21,124
**Performance**: 1.97µs

---

## Problem

Count the total letters in all English words for numbers 1 to 1,000 (British
convention, no spaces or hyphens).

---

## Mathematical Foundation

### Key Insight: Arithmetic Structure, Not Strings

Instead of generating strings (allocations, char counting), we observe that
numbers have a fixed **hierarchical structure**:

```
n = (hundreds digit) × 100 + (tens digit) × 10 + (ones digit)
```

Each tier contributes a fixed letter count from a lookup table. This reduces
the problem to **table lookup + arithmetic**.

### Letter Count Tables

**Ones/Teens** (1–19):
| n | word | letters |
|---|------|---------|
| 1 | one | 3 |
| 2 | two | 3 |
| 3 | three | 5 |
| 4 | four | 4 |
| 5 | five | 4 |
| 6 | six | 3 |
| 7 | seven | 5 |
| 8 | eight | 5 |
| 9 | nine | 4 |
| 10 | ten | 3 |
| 11 | eleven | 6 |
| 12 | twelve | 6 |
| 13 | thirteen | 8 |
| 14 | fourteen | 8 |
| 15 | fifteen | 7 |
| 16 | sixteen | 7 |
| 17 | seventeen | 9 |
| 18 | eighteen | 8 |
| 19 | nineteen | 8 |

**Tens** (20, 30, …, 90):
| n | word | letters |
|---|------|---------|
| 20 | twenty | 6 |
| 30 | thirty | 6 |
| 40 | forty | 5 |
| 50 | fifty | 5 |
| 60 | sixty | 5 |
| 70 | seventy | 7 |
| 80 | eighty | 6 |
| 90 | ninety | 6 |

**Fixed words**: hundred=7, thousand=8, and=3

---

## Analytical Derivation

### Sum(1–9) = 36
3+3+5+4+4+3+5+5+4 = **36**

### Sum(10–19) = 70
3+6+6+8+8+7+7+9+8+8 = **70**

### Sum(20–99) = 748
- Each tens word appears 10 times (paired with 0–9):
  `(6+6+5+5+5+7+6+6) × 10 = 46 × 10 = 460`
- Ones (1–9) appear 8 times (once per tens group 20–90):
  `36 × 8 = 288`
- **Total: 460 + 288 = 748**

### Sum(1–99) = 854
36 + 70 + 748 = **854**

### Sum(100–999) = 20,259
For each hundreds digit h = 1..9, the 100 numbers h×100 to h×100+99 contribute:

| Component | Per group | × 9 groups |
|-----------|-----------|-----------|
| "X hundred" (letters(h) + 7) × 100 | `100×letters(h) + 700` | `100×36 + 9×700 = 9,900` |
| "and" for remainders 1–99: 3 × 99 | 297 | 2,673 |
| Remainders 1–99: Sum(1–99) | 854 | 7,686 |
| **Total** | | **20,259** |

### 1000 = 11
"one"(3) + "thousand"(8) = **11**

### Grand Total
854 + 20,259 + 11 = **21,124** ✓

---

## Rust Implementation

### Core: Letter Count by Arithmetic Decomposition

```rust
const ONES: [u32; 20] = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
const TENS: [u32; 10] = [0, 0, 6, 6, 5, 5, 5, 7, 6, 6];

pub fn letter_count(n: u32) -> u32 {
    if n == 1000 { return 11; }
    let mut count = 0;
    let mut remainder = n;
    if remainder >= 100 {
        count += ONES[(remainder / 100) as usize] + 7; // "X hundred"
        remainder %= 100;
        if remainder > 0 { count += 3; } // "and"
    }
    if remainder >= 20 {
        count += TENS[(remainder / 10) as usize];
        remainder %= 10;
    }
    if remainder > 0 { count += ONES[remainder as usize]; }
    count
}
```

### Solve: Simple Iterator Sum

```rust
pub fn solve() -> u64 {
    (1..=1000).map(|n| letter_count(n) as u64).sum()
}
```

### Key Patterns

- **Lookup tables as `const` arrays**: Zero-cost, cache-resident, no branching
- **Arithmetic decomposition**: `/` and `%` extract each digit tier
- **No string allocation**: Pure arithmetic — 1,000× faster than string generation

---

## Complexity

| Metric | Value |
|--------|-------|
| Time | O(n) — 1 lookup per number |
| Space | O(1) — two fixed arrays |
| Benchmark | 1.97µs for n=1,000 |

---

## Verification (Key Examples)

| n | Decomposition | Letters |
|---|---------------|---------|
| 342 | three(5)+hundred(7)+and(3)+forty(5)+two(3) | **23** ✓ |
| 115 | one(3)+hundred(7)+and(3)+fifteen(7) | **20** ✓ |
| 100 | one(3)+hundred(7) [no "and"] | **10** ✓ |
| 1000 | one(3)+thousand(8) | **11** ✓ |

---

## Related

- [[project-euler-p016]] — Power Digit Sum (digit-array arithmetic)
- [[project-euler-p013]] — Large number digit work
- [[combinatorics-fundamentals]] — Counting techniques

---

*Part of the [[project-euler-index]] series*
