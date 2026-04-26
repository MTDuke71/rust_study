# Project Euler Problem 31: Coin Sums

**Solved**: 2026-04-26
**Difficulty**: 5%
**Category**: Dynamic Programming / Restricted Integer Partitions

## Problem Summary

Count the number of ways to make 200 pence using any number of UK coins from
the set {1, 2, 5, 10, 20, 50, 100, 200}, where order does not matter.

## Mathematical Concepts

### Primary Concepts
- **Restricted integer partitions** — partitions of n using only certain part sizes
- **Coin change counting** — combinatorial enumeration of multisets
- **Generating functions** — algebraic encoding of partition counts

### Supporting Concepts
- **Dynamic programming** — overlapping subproblems with optimal substructure
- **Combinations vs compositions** — unordered vs ordered tuples

## Solution Approach

### The Loop-Order Trick

Let `ways[s]` = number of unordered ways to make sum `s`. The standard recurrence is:

```text
ways[0] = 1                       (empty multiset is the unique way to make 0)
for each coin c:
    for s = c..=target:
        ways[s] += ways[s - c]
```

**Why coins-outer / sums-inner?** This forces a canonical ordering on the
multiset. When processing coin c, we are deciding "how many of coin c to use,
given that all earlier coins are already locked in." Any combination is
counted exactly once because coins are visited in a fixed order.

If we swapped the loops (sums outer, coins inner), then `ways[s] += ways[s-c]`
would count permutations: the sequence (1p, 2p) and (2p, 1p) become two
distinct ways of making 3p instead of one. Wrong answer.

### Generating Function View

The number of ways to make n using coins {c₁, c₂, ..., cₖ} is the coefficient
of x^n in the product:

$$\prod_{i=1}^{k} \frac{1}{1 - x^{c_i}}$$

Each factor `1/(1 - x^c) = 1 + x^c + x^{2c} + x^{3c} + ...` encodes "use 0, 1,
2, 3, ... copies of coin c." Multiplying picks one term from each factor —
exactly a multiset selection.

The DP above is the iterative computation of this product, truncated at degree
200.

### Worked Example: Make 5p with {1, 2, 5}

```text
Initial:  ways = [1, 0, 0, 0, 0, 0]

After coin 1 (every sum has exactly 1 way using only 1p coins):
          ways = [1, 1, 1, 1, 1, 1]

After coin 2 (add 1 or more 2p coins to each sum):
          ways[2] += ways[0] = 1 + 1 = 2
          ways[3] += ways[1] = 1 + 1 = 2
          ways[4] += ways[2] = 1 + 2 = 3
          ways[5] += ways[3] = 1 + 2 = 3
          ways = [1, 1, 2, 2, 3, 3]

After coin 5:
          ways[5] += ways[0] = 3 + 1 = 4
          ways = [1, 1, 2, 2, 3, 4]
```

Answer: ways[5] = 4. Manually verifiable:
- 5
- 2 + 2 + 1
- 2 + 1 + 1 + 1
- 1 + 1 + 1 + 1 + 1

### Generalization

The same algorithm answers many variants:
- **Bounded coin counts**: cap multiplicity per coin (knapsack with limits)
- **Postage stamps**: minimum coins for an amount (change argument to `min`)
- **Unreachable amounts**: detect zero-count sums (Frobenius / Chicken McNugget)

## Complexity

- **Time**: O(target × |coins|) = O(200 × 8) = 1,600 ops
- **Space**: O(target) = O(200)

| Metric | Value |
|--------|-------|
| Time | ~744 ns |
| Coins | 8 denominations |
| Target | 200p |
| Operations | ~1,600 |

## Rust Implementation Details

### Vec<u64> for In-Place Update

A single `Vec<u64>` of length `target + 1` is mutated in place. The forward
iteration `for s in c..=target` is critical: when we write `ways[s] += ways[s-c]`,
we want `ways[s-c]` to reflect updates from this coin (allowing multiple uses).
Iterating backward would give the bounded-coin (0/1 knapsack) variant instead.

### usize Conversion

`coin: u64` is converted to `usize` once outside the inner loop to avoid
repeated casts. Index arithmetic on `usize` is the natural Rust idiom for
buffer access.

## Answer

**73,682** ways to make £2 using UK coins.

## Related Problems

- **Problem 76** (Counting Summations) — same algorithm, parts are 1..=n−1
- **Problem 77** (Prime Summations) — coin set is the primes
- **Problem 78** (Coin Partitions) — partition function p(n) modulo arithmetic
- **Problem 191** (Prize Strings) — DP with state machines (different DP shape)

## Links

- [[project-euler-p030]] — previous problem (digit fifth powers)
- [[combinatorics-fundamentals]] — multiset counting basics
- [[harmonic-series-and-logarithmic-growth]] — for partition asymptotics
- [Coin Problem (Wikipedia)](https://en.wikipedia.org/wiki/Coin_problem)
- [Generating Functions (Wikipedia)](https://en.wikipedia.org/wiki/Generating_function)
