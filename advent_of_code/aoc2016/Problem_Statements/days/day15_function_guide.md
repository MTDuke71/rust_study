# Day 15: Timing is Everything — Function Guide

**Problem**: Find the first time to press a button so a capsule falls through
all rotating discs. Each disc must be at position 0 when the capsule reaches it.

**Part 1**: First time with 6 discs? → **317,371**
**Part 2**: First time with 7 discs (extra disc added)? → **2,080,951**

---

## Performance

| Metric | Brute Force | CRT | Speedup |
|--------|-------------|-----|---------|
| Combined | 3,065µs | 1.50µs | **2,043×** |
| Part 1 | 414µs | 1.29µs | 321× |
| Part 2 | 2,690µs | 1.40µs | 1,921× |
| Parse | negligible | negligible | — |

Both approaches implemented; CRT is the default solver.
Brute force available via `solve_brute_force()` for comparison.

---

## Architecture

```
Input: "Disc #1 has 17 positions; at time=0, it is at position 1.\n..."
    │
    ▼
parse_input(input)                 ──→ Vec<Disc>
    │
    ├── solve_part1_with_data(&discs)  ──→ 317,371
    │     └── solve_crt(discs)         ──→ fold congruences via CRT
    │           ├── extended_gcd()     ──→ modular inverse
    │           └── combine()          ──→ merge two congruences
    │
    └── solve_part2_with_data(&discs)  ──→ 2,080,951
          ├── Clone discs + push Disc { positions: 11, initial: 0 }
          └── solve_crt(extended)      ──→ same CRT with 7 congruences

    [Alternative: solve_brute_force() — linear scan with all_aligned()]
```

---

## Data Structures

### `Disc`
```rust
struct Disc {
    number: usize,     // disc number (1-based), also time offset
    positions: usize,  // total positions on this disc
    initial: usize,    // position at time=0
}
```

Simple value type. `number` doubles as the time delay — disc N is reached
N seconds after the button press.

---

## Function-by-Function

### `parse_input(input: &str) -> Vec<Disc>`
Splits each line by whitespace and extracts fields by position:
- `parts[1][1..]` → disc number (strips `#` prefix)
- `parts[3]` → total positions
- `parts[11]` → initial position (strips trailing `.`)

### CRT Functions (Primary Solver)

### `extended_gcd(a: i64, b: i64) -> (i64, i64, i64)`
Extended Euclidean algorithm. Returns `(gcd, x, y)` where `a*x + b*y = gcd`.
Used to compute modular inverses needed by `combine()`. Recursive implementation
with base case `b == 0 → (a, 1, 0)`.

### `combine(a: i64, m: i64, b: i64, n: i64) -> (i64, i64)`
Merges two congruences `t ≡ a (mod m)` and `t ≡ b (mod n)` into a single
congruence `t ≡ r (mod lcm(m,n))`. Uses the extended GCD to find the
modular inverse of `m` modulo `n`.

### `solve_crt(discs: &[Disc]) -> usize`
Folds all disc constraints into a single congruence. Starts with
`t ≡ 0 (mod 1)` (any time works) and combines each disc's constraint:
`t ≡ -(initial + number) (mod positions)`. After processing all discs,
the remainder is the exact answer — no scanning needed.

### Brute-Force Functions (Alternative)

### `all_aligned(discs: &[Disc], time: usize) -> bool`
Checks if all discs are at position 0 when the capsule reaches them.
For a button press at `time`, disc N's position is:
```
(initial + time + disc_number) % positions
```
Uses `is_multiple_of()` instead of `% == 0` per Clippy recommendation.

### `solve_brute(discs: &[Disc]) -> usize`
Linear scan from `t = 0` upward using `(0..).find()`. Returns the first
time where all discs align. Simple but O(answer × num_discs).

### Entry Points

### `solve_part1_with_data(discs: &[Disc]) -> usize`
Delegates to `solve_crt()`. Answer: 317,371.

### `solve_part2_with_data(discs: &[Disc]) -> usize`
Clones the disc list, appends a 7th disc (11 positions, initial 0),
then delegates to `solve_crt()`. Answer: 2,080,951.

### `solve(input) -> (String, String)`
Parse-once entry point. Parses discs once, passes to both `_with_data` functions.

### `solve_brute_force(input) -> (String, String)`
Public entry point for the brute-force approach. Used for benchmarking comparison.

---

## Algorithm: Modular Arithmetic Alignment

### The Physical Model
- Capsule dropped at time `t`, falls through stacked rotating discs
- Disc N is reached at time `t + N` (1 second per disc)
- Each disc has a slot at position 0 — capsule passes only when disc is at 0
- Disc rotates 1 position per second from its initial position

### The Math
For each disc, we need:
```
(initial_position + t + disc_number) mod total_positions == 0
```

This is a system of **linear congruences** — one per disc:
```
t ≡ -initial - disc_number  (mod positions)
```

### Chinese Remainder Theorem (Implemented)
This system can be solved directly using the Chinese Remainder Theorem (CRT)
when all moduli are coprime. Our disc sizes (17, 7, 19, 5, 3, 13) happen
to all be prime and distinct, so CRT applies perfectly.

The CRT approach combines congruences pairwise using the extended Euclidean
algorithm to compute modular inverses:

```
Start:  t ≡ 0 (mod 1)                    — any time works
Disc 1: t ≡ 15 (mod 17)                  — combined: t ≡ 15 (mod 17)
Disc 2: t ≡ 5  (mod 7)                   — combined: t ≡ 117 (mod 119)
Disc 3: t ≡ 14 (mod 19)                  — combined: t ≡ 832 (mod 2261)
Disc 4: t ≡ 1  (mod 5)                   — combined: t ≡ 5,356 (mod 11,305)
Disc 5: t ≡ 1  (mod 3)                   — combined: t ≡ 16,661 (mod 33,915)
Disc 6: t ≡ 2  (mod 13)                  — combined: t ≡ 317,371 (mod 442,065)
```

Result: **317,371** — the exact answer in 6 combine operations.

### Benchmark: CRT vs Brute Force

| Approach | Combined | Operations | Speedup |
|----------|----------|------------|---------|
| Brute force | 3,065µs | ~2M × 7 = ~14M modular checks | 1× |
| CRT | 1.5µs | 6 combines × ~10 arithmetic ops | **2,043×** |

The brute force scans ~2M candidate times, checking all 7 discs each time.
CRT computes the exact answer directly — no scanning, no wasted work.

### Why Part 2 Takes Longer
The 7th disc has 11 positions, making the LCM of all disc sizes much larger.
The search space grows from ~317K to ~2.08M — a ~6.6× increase, which
matches the observed benchmark ratio (~6.5×).

### Complexity
- **Brute force**: O(answer × num_discs) — linear scan with constant-time alignment check
- **CRT**: O(num_discs) — one combine per disc, constant-time each
- **Space**: O(num_discs) — just the disc list

---

## Patterns Used

| Pattern | Description |
|---------|-------------|
| Modular arithmetic | `(offset + time) % period == 0` for cyclic alignment |
| Chinese Remainder Theorem | Combine congruences pairwise via extended GCD — O(n) exact solution |
| Extended Euclidean algorithm | Compute modular inverse for CRT combine step |
| Linear scan (alternative) | `(0..).find()` — brute-force over time values |
| Positional parsing | `split_whitespace()` + index fields by position |
| Clone-and-extend | Part 2 clones Part 1's data and adds one element |

---

## Optimization Applied: CRT over Brute Force

The CRT optimization was implemented alongside the brute-force approach to
demonstrate the algorithmic improvement. Both are available in the code:

- `solve()` / `solve_part1()` / `solve_part2()` — use CRT (default)
- `solve_brute_force()` — linear scan (for benchmarking comparison)

The 2,043× speedup comes from eliminating all scanning. CRT computes the
exact answer from the congruence system — it doesn't guess-and-check.

### Remaining Potential Optimizations (Not Applied)

| Optimization | Expected Impact | Why Not |
|-------------|-----------------|---------|
| Skip by largest disc period (brute) | ~17× fewer checks | CRT already solves it |
| Pre-compute `-initial - number` (brute) | Avoid repeated addition | CRT already solves it |
| Iterative extended GCD | Avoid recursion overhead | 6 levels deep — negligible |

---

## Input Analysis

```
Disc #1 has 17 positions; at time=0, it is at position 1.
Disc #2 has 7 positions; at time=0, it is at position 0.
Disc #3 has 19 positions; at time=0, it is at position 2.
Disc #4 has 5 positions; at time=0, it is at position 0.
Disc #5 has 3 positions; at time=0, it is at position 0.
Disc #6 has 13 positions; at time=0, it is at position 5.
```

- All disc sizes are **prime** → CRT directly applicable
- LCM of all 6: 17 × 7 × 19 × 5 × 3 × 13 = 442,065
- LCM with 7th disc (11): 442,065 × 11 = 4,862,715
- Answer for Part 1 (317,371) < LCM (442,065) ✓
- Answer for Part 2 (2,080,951) < LCM (4,862,715) ✓

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
