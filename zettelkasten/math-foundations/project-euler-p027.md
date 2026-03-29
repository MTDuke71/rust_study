# Project Euler Problem 27: Quadratic Primes

**Solved**: 2026-03-29
**Difficulty**: 5%
**Category**: Number Theory / Prime Generation

## Problem Summary

Find coefficients a, b (|a| < 1000, |b| ≤ 1000) for n² + an + b that produces
the maximum number of consecutive primes starting from n = 0.

## Mathematical Concepts

### Primary Concepts
- **Prime-generating polynomials** — quadratic formulas that produce many primes
- **Euler's prime formula** — n² + n + 41 produces 40 consecutive primes

### Supporting Concepts
- **Primality testing** — trial division for testing candidates
- **Search space pruning** — using constraints to eliminate impossible pairs

## Solution Approach

### Euler's Original Discovery (1772)

Euler noticed that n² + n + 41 produces primes for n = 0 through 39:

```text
n=0:  41    prime ✓
n=1:  43    prime ✓
n=2:  47    prime ✓
n=3:  53    prime ✓
...
n=39: 1601  prime ✓
n=40: 1681 = 41²  ✗  (the formula "breaks" at its own constant!)
```

Why does it fail at n = 40? Because f(40) = 40² + 40 + 41 = 40(40+1) + 41 = 40·41 + 41 = 41·41.
More generally, f(b-1) is always divisible by b when f(n) = n² + n + b.

### Pruning the Search Space

**Constraint 1: b must be prime**

f(0) = b. If b isn't prime, the formula fails immediately at n=0.
There are 168 primes ≤ 1000, cutting b from 2001 values to 168.

**Constraint 2: 1 + a + b must be prime and positive**

f(1) = 1 + a + b. This must be prime, which means:
- 1 + a + b ≥ 2, so a ≥ 1 - b (gives lower bound for a)

**Constraint 3: b must be positive**

Since b = f(0) must be prime, and all primes are positive, b > 0.

**Result**: ~168 × 2000 = ~336,000 pairs instead of ~4,000,000. An 12× reduction.

### The Winning Formula

```text
n² − 61n + 971

a = -61, b = 971, product a×b = -59231
Produces 71 consecutive primes (n = 0 through 70)
```

### Parabola Symmetry: The Full Picture

The formula is m² − m + 41 where m = n − 30. The parabola has its vertex
at n = 30.5, creating perfect mirror symmetry:

```text
 n   f(n)   Structure
──   ────   ─────────────────────────────────
 0    971   ┐
 1    911   │
 2    853   │
 3    797   │  Descending arm (n=0 to 30)
 4    743   │  These are Euler's primes
 ...  ...   │  walking DOWN the parabola
28     83   │
29     43   │
30     41   ┘← vertex (minimum)
31     41   ┐← same value! mirror starts
32     43   │
33     47   │  Ascending arm (n=31 to 61)
34     53   │  SAME values as n=0..30
 ...  ...   │  walking back UP the parabola
60    911   │
61    971   │
62   1033   ┘← first NEW prime (past the mirror)
63   1097   ┐
64   1163   │
65   1231   │  9 additional primes (n=62 to 70)
66   1301   │  continuing up the curve
67   1373   │  into unexplored territory
68   1447   │
69   1523   │
70   1601   │← same 1601 from n²−79n+1601!
71   1681   ┘← 41² — the inescapable wall
```

The 71 consecutive primes break down as:
- **n=0 to 30**: 31 primes descending to the vertex (Euler's primes in reverse)
- **n=31 to 61**: 31 primes ascending (mirror of n=0..30, same values)
- **n=62 to 70**: 9 new primes continuing up the curve
- **n=71**: 1681 = 41² — the same barrier that kills Euler's original formula

The mirrored section (n=0..60) produces only 31 unique prime values.
The formula squeezes out 10 more primes beyond the mirror before 41²
terminates the run.

### Connection to Euler's Formula

n² − 61n + 971 = (n-30)² − (n-30) + 41. It's Euler's formula m² + m + 41
reflected (substitute −m) and shifted by 30.

Similarly, n² − 79n + 1601 = (n-40)² + (n-40) + 41. It's Euler's formula
shifted by 40! This gives it 80 consecutive primes (n = 0..79), but it
uses b = 1601 > 1000, so it's outside our search range.

### Why Do Quadratic Formulas Generate Primes?

There is no polynomial that produces *only* primes (proven). But some
quadratics hit primes with remarkable frequency. This connects to deep
number theory:

- **Heegner numbers**: The discriminant b² - 4ac relates to class numbers
  of imaginary quadratic fields
- **Class number 1**: Euler's 41 works because Q(√(-163)) has class
  number 1, and 163 = 4·41 - 1
- **Rabinowitz's theorem**: n² + n + q produces primes for n = 0..q-2
  if and only if 4q - 1 is a Heegner number
- There are exactly 9 Heegner numbers: 1, 2, 3, 7, 11, 19, 43, 67, 163

This is why 41 is special — 4·41 - 1 = 163, the largest Heegner number!

## Complexity

- **Time**: O(P × A × C) where P = primes ≤ 1000, A = range of a, C = avg chain length
- **Space**: O(1) beyond the prime sieve for b candidates

| Component | Value |
|-----------|-------|
| Prime candidates for b | 168 |
| Range of a per b | ~2000 (varies) |
| Average chain length | ~5-10 primality tests |
| Total primality tests | ~2-3 million |

## Performance

| Metric | Value |
|--------|-------|
| Time | 3.27 ms |
| Target | < 100 ms |
| Bottleneck | Primality testing via trial division |

### Possible Optimizations (Not Needed)

1. **Sieve-based primality**: Pre-compute primes up to ~1M for O(1) lookups
2. **Additional pruning**: a must have same parity as b+1 (since 1+a+b must be odd to be prime >2)
3. **Early termination**: Track best count, skip (a,b) pairs where b < best count

At 3.27ms none of these are necessary.

## Rust Implementation Details

### Clean Separation of Concerns

```rust
pub fn consecutive_primes(a: i64, b: i64) -> u64 {
    let mut n = 0i64;
    loop {
        let val = n * n + a * n + b;
        if val < 2 || !is_prime(val as u64) {
            return n as u64;
        }
        n += 1;
    }
}
```

The `val < 2` check handles negative results (when a is large negative)
before casting to u64 for the primality test.

### Signed Answer in Unsigned Registry

The answer -59231 is the first negative answer in our Project Euler suite.
`solve()` returns `i64`; the registry casts to `u64` for storage and
the CLI casts back to `i64` for display.

## Answer

**-59231** (a = -61, b = 971, 71 consecutive primes)

## Related Problems

- **Problem 7** (10001st Prime) — prime generation
- **Problem 10** (Summation of Primes) — prime sieve
- **Problem 50** (Consecutive Prime Sum) — longest sum of consecutive primes
- **Problem 58** (Spiral Primes) — prime density in number spirals

## Links

- [[project-euler-p026]] — previous problem (number theory / modular arithmetic)
- [[project-euler-p010]] — prime sieve utility
- [Euler's Prime Formula (Wikipedia)](https://en.wikipedia.org/wiki/Formula_for_primes#Euler's_formula)
- [Heegner Number (Wikipedia)](https://en.wikipedia.org/wiki/Heegner_number)
- [Ulam Spiral (Wikipedia)](https://en.wikipedia.org/wiki/Ulam_spiral) — visual prime patterns in quadratics
