# Project Euler Problem 33: Digit Cancelling Fractions

**Solved**: 2026-04-26
**Difficulty**: 5%
**Category**: Number Theory / Fractions

## Problem Summary

Find the four non-trivial 2-digit-over-2-digit fractions (less than 1) where
"cancelling" a shared digit gives a numerically correct simplification, then
report the denominator of their product in lowest terms.

## Mathematical Concepts

### Primary Concepts
- **Cross-multiplication** — exact equality without floating-point error
- **GCD reduction** — fraction in lowest terms
- **Position-pair enumeration** — checking all four ways digits can match

### Supporting Concepts
- **Triviality criterion** — defining what makes a cancellation "interesting"
- **Brute force with pruning** — small search space, no algorithmic trick

## Solution Approach

### Position Pairings

Write n = 10·n₁ + n₂ and d = 10·d₁ + d₂. For "cancelling" a digit to make
sense, some n-digit must equal some d-digit. There are four position pairings:

| Cancel n digit | Cancel d digit | Reduced fraction |
|----------------|----------------|------------------|
| n₁ | d₁ | n₂ / d₂ |
| n₁ | d₂ | n₂ / d₁ |
| n₂ | d₁ | n₁ / d₂ |
| n₂ | d₂ | n₁ / d₁ |

A pairing is *valid* if (a) the chosen digits are equal and (b) the resulting
denominator is non-zero.

### Cross-Multiplication for Exact Equality

Comparing rational numbers via floats invites precision bugs. Instead, use:

$$\frac{n}{d} = \frac{n'}{d'} \iff n \cdot d' = d \cdot n'$$

Both sides are exact integer products well within u64 range (max ≈ 99 × 99 =
9801). No overflow, no rounding, deterministic.

### The Triviality Criterion

The problem flags `30/50 = 3/5` as a trivial example. Both numerator and
denominator end in 0, so cancelling the trailing zeros is the same as dividing
both by 10 — uninteresting. The exclusion rule:

```text
Skip if n₂ == 0 AND d₂ == 0
```

What about other "obvious" cases like 11/22? Note that 11 and 22 share *no*
digits — {1,1} ∩ {2,2} = ∅ — so the algorithm never even considers them. The
triviality filter only needs to handle the trailing-zero case.

What about 33/66? Same situation: {3,3} ∩ {6,6} = ∅. No shared digit; no
candidate. The position-pair enumeration *automatically* excludes
"aa/bb" cases because those digits, by construction, don't overlap.

### The Four Curious Fractions

Brute-force enumeration over `10 ≤ n < d ≤ 99` (4,005 pairs) yields exactly
four matches:

| Fraction | Cancel | Reduced | Decimal Check |
|----------|--------|---------|---------------|
| 16/64 | cancel 6s | 1/4 | 0.25 = 0.25 ✓ |
| 19/95 | cancel 9s | 1/5 | 0.2 = 0.2 ✓ |
| 26/65 | cancel 6s | 2/5 | 0.4 = 0.4 ✓ |
| 49/98 | cancel 9s | 4/8 = 1/2 | 0.5 = 0.5 ✓ |

### Computing the Final Answer

Product of fractions in lowest terms:

$$\frac{16 \cdot 19 \cdot 26 \cdot 49}{64 \cdot 95 \cdot 65 \cdot 98}
= \frac{387{,}296}{38{,}729{,}600}$$

Apply Euclidean GCD:

$$\gcd(387{,}296, 38{,}729{,}600) = 387{,}296$$

Reduced: 1 / 100. **Denominator = 100.**

The shortcut: since each curious fraction reduces to its cancellation form,

$$\frac{1}{4} \cdot \frac{1}{5} \cdot \frac{2}{5} \cdot \frac{1}{2}
= \frac{2}{200} = \frac{1}{100}$$

Either path lands at 100.

## Complexity

- **Time**: O(N²) where N = 90 (count of 2-digit numbers)
  - 4,005 pairs × constant work
- **Space**: O(k) where k = 4 (matched fractions)

| Metric | Value |
|--------|-------|
| Time | ~8.6 µs |
| Pairs tested | 4,005 |
| Matches found | 4 |

## Rust Implementation Details

### Borrowed GCD from utils

The shared `utils::number_theory::gcd` function provides Euclidean GCD. This is
the third Project Euler problem to use it (after p005 and p021), validating the
shared-utilities strategy: extract once, reuse forever.

### Tuple Destructuring in Match Arms

```rust
for (cn, cd, new_n, new_d) in pairings {
    if cn != cd || new_d == 0 { continue; }
    if n * new_d == d * new_n { return true; }
}
```

The 4-tuple is destructured directly in the for-loop binding. This is more
readable than indexing `pairings[i].0` and matches the conceptual "for each
position pairing, try this check" pattern.

## Cross-Language: An Elegant Haskell Solution

A Haskell list-comprehension solution exposes a deeper mathematical structure
that the Rust enumeration hides:

```haskell
[(10*x + y, 10*y + z) | x <- [1..9], y <- [1..9], z <- [1..9],
                        x /= y,
                        (9*x*z) + (y*z) == (10*x*y)]
```

### Variable Roles

The three variables encode positional roles directly:

- **x** = tens digit of numerator
- **y** = the *shared* (cancelled) digit — units of numerator, tens of denominator
- **z** = units digit of denominator

So `(10x + y, 10y + z)` is the fraction n/d where the cancelled digit y straddles
both numbers. For x=4, y=9, z=8 this is (49, 98) — the canonical example.

### The Algebraic Filter

The condition `9xz + yz = 10xy` is the cross-multiplication of "n/d = x/z"
after cancelling y, fully expanded:

$$\frac{10x + y}{10y + z} = \frac{x}{z}
\iff z(10x + y) = x(10y + z)
\iff 10xz + yz = 10xy + xz
\iff 9xz + yz = 10xy$$

No floats, no GCD in the search — just integer arithmetic on three single-digit
variables. Pre-deriving this form turns a runtime equality check into a
compile-time-known polynomial constraint.

### The Brilliant Insight

Of the four cancellation pairings (n's tens with d's tens, n's tens with d's
units, **n's units with d's tens**, n's units with d's units), only the
bolded one yields fractions strictly less than 1 with non-trivial cancellation.
The others either force n = d, force a 0 digit, or have no integer solutions.

The Haskell solution **bakes this single valid shape into its parametrization**.
It doesn't enumerate (n, d) pairs and check four pairings each — it directly
constructs only the (n, d) pairs that match the productive shape.

The `x /= y` filter handles one residual algebraic degenerate: x = y forces
z = y too, making n/d = x/x = 1, violating "less than one."

### Solution Set

Solving `9xz + yz = 10xy` for y ∈ [1..9] with x ≠ y gives exactly four triples:

| x | z | y = 9xz / (10x − z) | Fraction |
|---|---|---------------------|----------|
| 1 | 4 | 36 / 6 = 6 | 16/64 |
| 1 | 5 | 45 / 5 = 9 | 19/95 |
| 2 | 5 | 90 / 15 = 6 | 26/65 |
| 4 | 8 | 288 / 32 = 9 | 49/98 |

The complete answer set, falling out of the parametrization without an
explicit triviality filter (which the digit range [1..9] handles implicitly).

### Comparison to the Rust Solution

| Aspect | Rust (this problem) | Haskell list comprehension |
|--------|---------------------|----------------------------|
| Search space | (n, d) pairs: 4,005 | (x, y, z) triples: 729 (5.5× smaller) |
| Iterations per candidate | 4 cancellation pairings | 1 (shape pre-determined) |
| Triviality filter | Explicit `n2 == 0 && d2 == 0` | Implicit (digits ∈ [1..9]) |
| Equality check | Runtime cross-multiply | Pre-derived polynomial constraint |
| Style | Enumerate and check | Describe and generate |
| Math/code ratio | Math at runtime | Math at compile time |

The Haskell version isn't just shorter — it's **more mathematically refined**.
The author did the algebra *first*, derived the productive shape, then wrote
the search. The Rust version does the search first and lets the equality
check do the algebra at runtime. Both are correct; one is more elegant.

### Lesson for Rust Solutions

This pattern can be ported back to Rust without losing clarity:

```rust
pub fn solve_parametric() -> u64 {
    let mut num_prod: u64 = 1;
    let mut den_prod: u64 = 1;
    for x in 1..=9u64 {
        for y in 1..=9u64 {
            if x == y { continue; }
            for z in 1..=9u64 {
                if 9 * x * z + y * z == 10 * x * y {
                    num_prod *= 10 * x + y;
                    den_prod *= 10 * y + z;
                }
            }
        }
    }
    den_prod / utils::number_theory::gcd(num_prod, den_prod)
}
```

The Rust `for x in / for y in / for z in` idiom carries the same structure as
the Haskell `x <- / y <- / z <-` generators. The equality filter is identical.
The result is a Rust solution that's both faster (fewer iterations) and more
self-documenting about *why* these particular fractions exist — the algebra is
on the page, not buried in cross-multiplications across position pairings.

**Takeaway**: Mathematical refactoring (proving which shape is productive)
can yield code that's simultaneously shorter, faster, and more transparent.
Brute force is the *baseline*, not the goal.

## Why This Problem Matters

This is a "discovery" problem: the answer (4 specific fractions) was unknown
in advance. The brute force is fast enough that mathematical cleverness isn't
required, but the **triviality criterion** is the subtle part: a careless
implementation might include 30/50, 40/80, etc., and report a wrong answer.

The lesson: in number-theory problems, "non-trivial" is a definition the
solver must carry from the problem statement to the code. Encoding that
correctly is half the battle.

## Answer

**100** — denominator of (16/64) · (19/95) · (26/65) · (49/98) in lowest terms.

## Related Problems

- **Problem 5** (Smallest Multiple) — also uses GCD/LCM
- **Problem 21** (Amicable Numbers) — also uses divisor sums
- **Problem 71** (Ordered Fractions) — fractions in reduced form
- **Problem 72** (Counting Fractions) — Farey sequences and Euler's totient
- **Problem 73** (Counting Fractions in a Range) — bounded fraction enumeration

## Links

- [[project-euler-p032]] — previous problem (pandigital products)
- [[lcm-gcd-euclidean]] — Euclidean GCD for fraction reduction
- [[number-theory-basics]] — divisibility and reduced forms
- [[modular-arithmetic]] — digit extraction with `/ 10` and `% 10`
- [Reduced Fraction (Wikipedia)](https://en.wikipedia.org/wiki/Irreducible_fraction)
