# Project Euler P067: Maximum Path Sum II

**Tags**: #project-euler #dynamic-programming #DAG
**Date**: 2026-02-28
**Answer**: 7,273
**Performance**: 48.5µs

---

Same problem as [[project-euler-p018]], 100-row triangle from file instead of 15-row inline.

## Implementation

```rust
use super::p018::{max_path_sum, parse_triangle};

const TRIANGLE_INPUT: &str = include_str!("../../Problem_Statements/0067_triangle.txt");

pub fn solve() -> u64 {
    let mut triangle = parse_triangle(TRIANGLE_INPUT);
    max_path_sum(&mut triangle) as u64
}
```

`include_str!` embeds the file at compile time — no runtime I/O, no path issues.

## Benchmarks

| Problem | Rows | Cells | Time |
|---------|------|-------|------|
| P018 | 15 | 120 | 1.92µs |
| P067 | 100 | 5,050 | 48.5µs |

~25× more time for ~42× more cells — parsing overhead dominates over the DP.

## Integrator Lesson

P067 is 8 lines of code because P018 provided validated, reusable components.
This is exactly the integrator philosophy: `parse_triangle` + `max_path_sum`
composed without modification.

See [[project-euler-p018]] for the full algorithm analysis.
