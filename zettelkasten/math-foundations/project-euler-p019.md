# Project Euler Problem 19: Counting Sundays

**Solved**: 2026-03-27
**Difficulty**: 5%
**Category**: Calendar Arithmetic / Modular Arithmetic

## Problem Summary

Count how many Sundays fell on the first of the month from 1 Jan 1901 to 31 Dec 2000.

## Mathematical Concepts

### Primary Concepts
- **Modular arithmetic** — Day-of-week cycles mod 7
- **Calendar arithmetic** — Advancing through months using day counts

### Supporting Concepts
- **Leap year rules** — Divisibility conditions (4, 100, 400)
- **Gregorian calendar structure** — Month lengths, year cycles

## Solution Approach

### Key Insights

1. **Day-of-week is a mod-7 residue**: Starting from a known anchor (1 Jan 1900 = Monday = 1), each month's first day is:
   ```text
   next_first = (current_first + days_in_month) mod 7
   ```

2. **No need for full date arithmetic**: We never need the actual date, just the day-of-week on the 1st of each month.

3. **The anchor year (1900) bootstraps but isn't counted**: We iterate from 1900 to accumulate the correct day-of-week, but only count Sundays from 1901 onward.

### Algorithm

```text
day = 1  (Monday, encoding: 0=Sun, 1=Mon, ..., 6=Sat)
count = 0

for year in 1900..=2000:
    for month in 1..=12:
        if year >= 1901 and day == 0:
            count += 1
        day = (day + days_in_month(month, year)) % 7

return count  // 171
```

### Leap Year Rules

The Gregorian calendar leap year rule is a three-tier divisibility test:

| Condition | Leap? | Example |
|-----------|-------|---------|
| Not divisible by 4 | No | 1901, 1999 |
| Divisible by 4, not by 100 | Yes | 1904, 1996 |
| Divisible by 100, not by 400 | No | 1900, 1800 |
| Divisible by 400 | Yes | 2000, 1600 |

In Rust: `(year % 4 == 0 && year % 100 != 0) || year % 400 == 0`

This creates a **400-year calendar cycle** with exactly 97 leap years and 146,097 days (= 20,871 weeks), meaning the calendar repeats exactly every 400 years.

## Complexity Analysis

- **Time**: O(n) where n = number of months (1,200 for the 20th century)
- **Space**: O(1) — only tracking day-of-week and count
- **Benchmark**: **2.98 µs** (Criterion, release build)
- **Justification**: Single pass through all months, constant work per month

## Rust Implementation

See [[project_euler/src/problems/p019.rs]] for complete code.

### Key Code Patterns

```rust
// Leap year check using Rust 1.73+ is_multiple_of
pub fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100))
        || year.is_multiple_of(400)
}

// Month-by-month day-of-week accumulation
let mut day_of_week: u32 = 1; // Monday
day_of_week = (day_of_week + days_in_month(month, year)) % 7;
```

### Design Decisions

- **Generic `count_first_sundays(start, end)`**: Parameterized over year range, not hardcoded to 1901–2000
- **Separate `is_leap_year` and `days_in_month`**: Reusable for future calendar problems
- **Encoding choice**: 0=Sunday aligns with checking `day == 0` cleanly

## Alternative Approaches

### Zeller's Congruence
Could compute the day-of-week for the 1st of each month directly using Zeller's formula:
```text
h = (1 + ⌊13(m+1)/5⌋ + K + ⌊K/4⌋ + ⌊J/4⌋ - 2J) mod 7
```
This avoids sequential accumulation but is harder to read and verify.

### chrono Crate
In production Rust, `chrono::NaiveDate::from_ymd(y, m, 1).weekday()` handles all calendar logic. We implemented from scratch for the mathematical exercise.

## Related Problems

- **Project Euler**: No directly related problems yet
- **AoC**: Calendar arithmetic appears in various scheduling puzzles
- **Future**: Any problem needing day-of-week calculations can reuse `is_leap_year` and `days_in_month`

## Learning Insights

- Modular arithmetic turns calendar problems into simple accumulation
- The 1900 anchor point is a clever misdirection — it's needed to bootstrap but isn't counted
- Leap year rules encode astronomical corrections: 365.2425 days/year average (365 + 1/4 - 1/100 + 1/400)
- The Gregorian calendar's 400-year cycle (146,097 days = exactly 20,871 weeks) is a beautiful consequence of the leap year rules

## References

- [Gregorian Calendar (Wikipedia)](https://en.wikipedia.org/wiki/Gregorian_calendar)
- [Zeller's Congruence (Wikipedia)](https://en.wikipedia.org/wiki/Zeller%27s_congruence)

---

*Links:*
- **Backlinks**: [[project-euler-p019|Problem Statement]]
- **Concept Tags**: #modular-arithmetic #calendar #project-euler
- **Difficulty**: #euler-easy
