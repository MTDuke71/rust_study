# Day 17: Spinlock — Function Guide

**Problem**: A "spinlock" repeatedly inserts values `1..=N` into a circular buffer, each time stepping forward `steps` positions (wrapping) before inserting.
**Answers**: Part 1 = **136**, Part 2 = **1080289**
**Code**: [day17.rs](../../src/solver/day17.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [parse_input](#parse_input)
3. [solve_part1_with_data](#solve_part1_with_data)
4. [solve_part2_with_data](#solve_part2_with_data)
5. [Part 2 Key Insights](#part-2-key-insights)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: a single integer (your step size). For my input: `363`.

The buffer starts as `[0]` with current position `0`. For each value `i = 1..=N`:
1. Step forward `steps` positions in the circular buffer (wrapping).
2. Insert `i` immediately after that position.
3. The inserted value becomes the new current position.

- **Part 1**: Run for `N = 2017`. Report the value *immediately after* `2017` in the final buffer.
- **Part 2**: Run for `N = 50,000,000`. Report the value *immediately after* `0`.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> usize
```

Trim whitespace and parse as `usize`. That's the entire input.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(steps: usize) -> usize
```

Direct simulation is fine for N = 2017:

```rust
let mut buf = Vec::with_capacity(2018);
buf.push(0);
let mut pos = 0;
for i in 1..=2017 {
    pos = (pos + steps) % i + 1;    // i == buffer size before insert
    buf.insert(pos, i);
}
buf[(pos + 1) % buf.len()]
```

- `(pos + steps) % i` is the index of the value we stop on (buffer has `i` elements before the insert).
- `+ 1` shifts to the index where `i` is placed (and becomes the new position).
- `Vec::insert` is O(n), but n ≤ 2018 keeps it cheap.
- The answer sits at `(pos + 1) % len` — the element just after the final insertion.

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(steps: usize) -> usize
```

We **cannot** build a 50M-element buffer (too slow, too much memory). But we don't have to.

```rust
const N: usize = 50_000_000;
let mut pos = 0usize;
let mut value_at_1 = 0usize;
let mut i = 1usize;
while i <= N {
    if pos + steps < i {
        // Batch: skip all consecutive no-wrap iterations
        let d = i - pos - steps;
        let k = d.div_ceil(steps).min(N + 1 - i);
        pos += k * (steps + 1);
        i += k;
    } else {
        pos = (pos + steps) % i;
        if pos == 0 {
            value_at_1 = i;
        }
        pos += 1;
        i += 1;
    }
}
value_at_1
```

---

## Part 2 Key Insights

### Insight 1 — Position 0 never changes

Insertions always happen **after** the current position, and after the first insert `pos ≥ 1` for the rest of the run. So index 0 of the buffer is frozen as `0` forever.

That means: *the value immediately after 0* = *the value at index 1 of the final buffer* = *the last value ever inserted at index 1*.

An insertion lands at index 1 iff `(pos + steps) % i == 0`. We just track the most recent `i` for which that holds.

### Insight 2 — Batch the no-wrap iterations

Each iteration computes `pos_new = (pos + steps) % i + 1`. When `pos + steps < i` there's no wrap and `pos_new = pos + steps + 1`. Such an iteration never lands at index 1, so we can skip it.

Starting from `(pos, i)`, after `k` consecutive no-wrap iterations:
- `pos' = pos + k · (steps + 1)`
- `i'   = i + k`

The no-wrap condition at iteration `j` (for `j = 0..k-1`) is:
```
pos + j·(steps+1) + steps < i + j
⇔ j · steps < i − pos − steps = D
```

So `k = ⌈D / steps⌉` consecutive iterations are all no-wrap, and the `(k+1)`-th wraps. We fold them into one arithmetic step.

**Effect**: the while-loop runs roughly `Σ_{i=steps}^{N} (steps/i) ≈ steps · ln(N/steps)` times — about `363 · ln(50M/363) ≈ 4,300` wrap iterations plus the same number of batches, instead of 50M single iterations.

---

## Benchmarks

| Version | Time | Speedup |
|---------|------|---------|
| Naive 50M loop (no batching) | 123.56ms | 1× |
| Batched no-wrap skip | **131.84µs** | **937×** |

Part 1 alone is trivial (< 20µs); Part 2 dominates the runtime.

---

## Key Patterns

### Simulate → Observe Invariant → Skip
Pattern across AoC: first solve by direct simulation, then spot a structural invariant (here: position 0 is fixed), and reduce the problem to tracking a single scalar. No buffer needed at all for Part 2.

### Arithmetic Batching
When the inner loop has a predictable no-branch path (the no-wrap case is pure addition), compute how many steps of that path will execute in closed form and jump. The branchy case — the only one that matters — runs far fewer times.

### Division-Ceiling via `div_ceil`
`d.div_ceil(steps)` (stable since Rust 1.73) replaces `(d + steps - 1) / steps`. Both correct; the former is self-documenting.

---

## Math Foundations

The $\sim 4{,}260$ wrap-iteration count and the $\sim 11.7$ expected-hits prediction both come from the **harmonic sum** $\sum_{i=a}^{b} 1/i \approx \ln(b/a)$. See the zettelkasten note [[harmonic-series-and-logarithmic-growth]] for:

- Derivation of $H_N \approx \ln(N) + \gamma$ via integral comparison
- Why problems with $N$ linear iterations often have only $\log N$ meaningful events
- Classical occurrences (coupon collector, quicksort, divisor sums)
- The log-log perception trap that makes $1/x$ densities look uniformly spread
