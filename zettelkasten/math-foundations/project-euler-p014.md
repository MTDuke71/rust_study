# Project Euler Problem 14: Longest Collatz Sequence - Mathematical Analysis

*Status: ✅ Solved*
*Difficulty: 5%*
*Answer: 837799*
*Date: 2026-02-26*

---

## Problem Statement

The following iterative sequence is defined for positive integers:

- n → n/2 (n is even)
- n → 3n + 1 (n is odd)

Which starting number, under one million, produces the longest chain?

**NOTE**: Once the chain starts, terms are allowed to go above one million.

---

## Mathematical Foundation

### Core Concepts

1. **The Collatz Conjecture**: One of the most famous unsolved problems in mathematics
2. **Memoization / Dynamic Programming**: Overlapping subproblems with shared chain suffixes
3. **Even-number elimination**: Dominance argument to prune search space
4. **Odd shortcut**: Algebraic observation that 3n+1 is always even

---

## The Collatz Conjecture

> **Conjecture** (Lothar Collatz, 1937): For every positive integer n, the Collatz sequence eventually reaches 1.

**Status**: Unproven as of 2026. Verified computationally for all n < 2^68 (~2.95 x 10^20).

**Why it's hard**: The sequence is deterministic but chaotic. Small changes in starting value produce wildly different chain lengths. No algebraic structure has been found to prove termination.

**Paul Erdos**: "Mathematics may not be ready for such problems."

### Formal Definition

The Collatz function T: N → N is defined as:

```
T(n) = n/2       if n ≡ 0 (mod 2)
T(n) = 3n + 1    if n ≡ 1 (mod 2)
```

The **total stopping time** σ(n) is the smallest k such that T^k(n) = 1.

**Example**: σ(13) = 9 (but chain has 10 *terms*, counting the starting value)

```
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
       9 steps between 13 and 1, but 10 terms total
```

---

## Approach 1: Memoized Bottom-Up with Optimizations (Optimal)

### Algorithm

```
1. Allocate cache[0..1_000_000], set cache[1] = 1
2. For each i from 2..1_000_000:
   - Skip even numbers < 500,000 (Optimization 1)
   - Walk the Collatz chain using odd shortcut (Optimization 2)
   - Stop when we hit a cached value
   - Backfill the chain using per-entry step costs
   - Only store values < 1_000_000 in cache (others are transient)
3. Return argmax of cache (excluding skipped even candidates)
```

### Why Memoization Works Here

**Overlapping subproblems**: The same number appears in many chains.

Consider all numbers whose chains pass through 16:

```
16 → 8 → 4 → 2 → 1                    (5 terms)
5 → 16 → 8 → 4 → 2 → 1               (6 terms)
10 → 5 → 16 → ...                      (7 terms)
20 → 10 → 5 → 16 → ...                (8 terms)
3 → 10 → 5 → 16 → ...                 (8 terms)
```

Without memoization, every chain recomputes 16 → 8 → 4 → 2 → 1. With caching, this suffix is computed once and reused.

**Optimal substructure**: chain_length(n) = 1 + chain_length(T(n))

This is exactly Mission 11's memoization pattern:
- Boolean → counting transformation (here: "does it reach 1?" → "how many steps?")
- Cache strategy: Dense array for values < limit, skip values > limit
- Backfill pattern: Walk until cached, then propagate backwards

---

## Optimization 1: Even-Number Elimination

### Theorem

For any even number e where 2e < limit, e cannot be the answer.

### Proof

**Given**: e is even, 2e < limit

**Step 1**: chain_length(2e) = 1 + chain_length(e)

Since 2e is even, T(2e) = 2e/2 = e. Therefore:
```
chain(2e) = [2e] ++ chain(e)
chain_length(2e) = 1 + chain_length(e) > chain_length(e)
```

**Step 2**: 2e is in the search range

Since 2e < limit, both e and 2e are valid starting numbers.

**Step 3**: e is dominated

For any even e < limit/2, there exists 2e < limit with a strictly longer chain. Therefore e is never the argmax. ∎

### What This Eliminates

- All even numbers from 2 to limit/2 - 1
- That's ~250,000 numbers for limit = 1,000,000
- Reduces search candidates from 1M to ~750K

### What We Keep

Even numbers ≥ 500,000 must still be checked: their double (≥ 1,000,000) exceeds the limit and falls outside the search range.

### Recursive Domination Chains

The argument applies recursively, creating "domination chains":

```
chain_length(2)  < chain_length(4)   < chain_length(8)   < chain_length(16) < ...
chain_length(6)  < chain_length(12)  < chain_length(24)  < chain_length(48) < ...
chain_length(10) < chain_length(20)  < chain_length(40)  < chain_length(80) < ...
```

Each chain doubles until it exceeds the limit. Only the last element (the one whose double falls outside the range) is a valid candidate.

---

## Optimization 2: Odd Shortcut — n → (3n+1)/2

### Algebraic Observation

**Claim**: When n is odd, 3n+1 is always even.

**Proof**: If n is odd, then n = 2k+1 for some k. So 3n+1 = 3(2k+1)+1 = 6k+4 = 2(3k+2), which is even. ∎

### Consequence

When n is odd, the Collatz sequence always performs two steps:
```
n → 3n+1 → (3n+1)/2
```

We can combine these into a single computation: n → (3n+1)/2

### Step Counting

The shortcut computes one operation but represents **two** Collatz steps. Each chain entry must track its step cost:

```rust
if n % 2 == 0 {
    chain.push((n, 1));   // even: n → n/2 = 1 step
    n /= 2;
} else {
    chain.push((n, 2));   // odd: n → 3n+1 → (3n+1)/2 = 2 steps
    n = (3 * n + 1) / 2;
}
```

During backfill, we accumulate `steps` instead of a flat +1:

```rust
let mut length = cache[n as usize];
for &(val, steps) in chain.iter().rev() {
    length += steps;
    // ...
}
```

### Impact

This halves the number of loop iterations for odd numbers. Since roughly half of all Collatz chain values are odd, this reduces total iterations by ~25%.

---

## Approach 2: Naive Per-Number (Baseline)

### Algorithm

```
For each n in 1..1_000_000:
  Walk chain to 1, counting steps
  Track maximum
```

### Complexity

Much slower in practice due to redundant computation. Every chain recomputes its shared suffix from scratch.

### Comparison

| Aspect | Memoized + Optimized ⭐ | Naive |
|--------|------------------------|-------|
| **Redundant work** | None (each value computed once) | Massive (shared suffixes recomputed) |
| **Starting numbers checked** | ~750K (skip ~250K evens < 500K) | All 1M |
| **Loop iterations per odd** | 1 (shortcut) | 2 (separate 3n+1 and /2) |
| **Memory** | O(n) cache array | O(1) |
| **Time** | O(n) amortized | O(n x avg_chain_length) |
| **Cache-friendly** | Yes (dense array) | N/A |

---

## Complexity Analysis

### Memoized Approach

- **Time**: O(n) amortized where n = 1,000,000
  - Each number < n is visited and cached exactly once
  - Numbers > n visited during chains are not cached but don't dominate
  - Total work proportional to sum of all chain lengths, but with deduplication

- **Space**: O(n) for the cache array
  - `vec![0u32; 1_000_000]` = ~4 MB

### Collatz Chain Properties

- **Average chain length** for numbers under 1M: ~120 steps
- **Maximum chain length**: 525 steps (starting from 837799)
- **Maximum intermediate value**: Can exceed 1M significantly (837799 reaches ~2.97 billion)
- **u64 safety**: Maximum intermediate value for any start < 1M fits comfortably in u64

### Empirical Cache Effectiveness

Analysis test tracks every cache hit and step saved at two scales:

**Limit = 500**:
```
Starting numbers (2..500):     498
  Evens skipped (< half):     124  (25%)
  Already cached (backfill):  175  (35%)
  Chains computed:            199  (40%)

Steps walked (memoized):      1,219
Steps saved (cache hits):     10,342
Naive total steps:            26,033

Reduction vs naive:           95.3% fewer steps (26,033 → 1,219)
Cache hit ratio:              89.5% of total steps came from cache
```

**Limit = 10,000**:
```
Starting numbers (2..10000):   9,998
  Evens skipped (< half):     2,499  (25%)
  Already cached (backfill):  3,517  (35.2%)
  Chains computed:            3,982  (39.8%)

Steps walked (memoized):      24,961
Steps saved (cache hits):     335,429
Naive total steps:            849,637

Reduction vs naive:           97.1% fewer steps (849,637 → 24,961)
Cache hit ratio:              93.1% of total steps came from cache
```

**Scaling comparison**:

| Metric | Limit 500 | Limit 10,000 | Trend |
|--------|-----------|-------------|-------|
| Step reduction vs naive | 95.3% | 97.1% | Improves |
| Cache hit ratio | 89.5% | 93.1% | Improves |
| Already cached (backfill) | 35.1% | 35.2% | Stable |
| Chains actually computed | 40.0% | 39.8% | Stable |

**Key observations**:

- **60% of starting numbers never need a walk** — either skipped as even < half (25%) or already filled by a previous chain's backfill (35%)
- **Cache effectiveness improves with scale** — at larger limits, chains share more common suffixes, pushing the hit ratio from 89.5% to 93.1%
- **Step reduction approaches 97%+** — at 10K we walk only 24,961 of the 849,637 naive steps
- **Backfill is the hero**: When we walk from number X and pass through smaller numbers, those all get cached "for free." Later when we reach those numbers in the loop, they're already done.

This is memoization at its best — the overlapping subproblems aren't just theoretical, they're massive in practice.

### Criterion Benchmarks

Wall-clock performance measured with Criterion (release mode, 100 samples):

| Limit | Memoized | Naive | Speedup |
|-------|----------|-------|---------|
| 1,000,000 | 23.3ms | 99.9ms | **4.3×** |
| 10,000,000 | 265.6ms | 1,168ms | **4.4×** |

The wall-clock speedup holds steady at ~4.3-4.4× despite cache effectiveness improving from 95% to 97%. This is because memory allocation (`vec![0u32; limit]`) and access patterns dominate at scale — the bottleneck shifts from computation to memory bandwidth. The memoized approach at 10M allocates ~40MB of cache, and sequential traversal of that cache becomes the limiting factor rather than step computation.

---

## Why 837799?

The chain from 837799 (525 steps) visits values up to ~2.97 billion before descending to 1. Some observations:

- **Odd numbers dominate**: 837799 is odd, so the first step triples it (→ 2,513,398)
- **Long chains cluster**: Numbers near 837799 also tend to have long chains
- **No simple pattern**: The winner can't be predicted analytically — brute force with memoization is the approach

### Chain Visualization (first 20 steps)

```
837799 → 2513398 → 1256699 → 3770098 → 1885049 →
5655148 → 2827574 → 1413787 → 4241362 → 2120681 →
6362044 → 3181022 → 1590511 → 4771534 → 2385767 →
7157302 → 3578651 → 10735954 → 5367977 → 16103932 → ...
```

Notice how it rockets up past 10M within 20 steps before eventually winding back down over 500+ steps.

---

## Rust Implementation Details

### Memoized Solver (with both optimizations)

```rust
pub fn longest_collatz_under(limit: u64) -> u64 {
    let limit = limit as usize;
    let mut cache = vec![0u32; limit];
    cache[1] = 1;
    let half = limit / 2;

    for i in 2..limit {
        // Opt 1: Skip even numbers < half — their double always has a longer chain
        if i < half && i % 2 == 0 { continue; }
        if cache[i] != 0 { continue; }

        // Walk chain, collecting (value, step_cost) pairs
        let mut chain: Vec<(u64, u32)> = Vec::new();
        let mut n = i as u64;
        while n as usize >= limit || cache[n as usize] == 0 {
            if n % 2 == 0 {
                chain.push((n, 1));       // even: 1 step
                n /= 2;
            } else {
                chain.push((n, 2));       // Opt 2: odd shortcut, 2 steps
                n = (3 * n + 1) / 2;
            }
        }

        // Backfill using per-entry step costs
        let mut length = cache[n as usize];
        for &(val, steps) in chain.iter().rev() {
            length += steps;
            if (val as usize) < limit {
                cache[val as usize] = length;
            }
        }
    }

    // Search: skip even < half (can't win), check odds + upper evens
    cache.iter().enumerate().skip(half)
        .chain(cache.iter().enumerate().skip(1).take(half - 1)
            .filter(|&(i, _)| i % 2 != 0))
        .max_by_key(|&(_, &len)| len)
        .unwrap().0 as u64
}
```

**Key Rust features**:
- **Dense cache**: `vec![0u32; limit]` — flat array, cache-friendly access
- **Tuple chain**: `Vec<(u64, u32)>` — pairs each value with its step cost
- **Iterator composition**: Complex search expressed as chained iterators
- **Zero-cost abstractions**: `enumerate()`, `filter()`, `max_by_key()` compile to tight loops

### Naive Chain Length (for verification)

```rust
pub fn collatz_chain_length(mut n: u64) -> u64 {
    let mut count = 1;
    while n != 1 {
        if n % 2 == 0 { n /= 2; } else { n = 3 * n + 1; }
        count += 1;
    }
    count
}
```

---

## Connection to Binary Representation

The Collatz operation has a bit-level interpretation:
- **Even** (n/2): Right-shift by 1 bit
- **Odd** (3n+1): Always produces an even number, so the next step is always n/2

### Bit Patterns

```
n = 837799 = 0b11001100100011100111 (20 bits)

After 3n+1:
2513398    = 0b1001100101101010110110 (22 bits, grew by 2)

After /2:
1256699    = 0b100110010110101011011 (21 bits)
```

The odd step roughly multiplies by 1.5 (grows ~0.58 bits on average), while the even step divides by 2 (removes exactly 1 bit). Chains grow when odd steps dominate, and shrink when trailing zeros allow multiple halvings.

### Trailing Zeros Acceleration

A potential further optimization (not implemented): count trailing zeros and shift by that many bits at once:

```
n.trailing_zeros() → shift right by that count, add that count to steps
```

This would turn sequences like 16 → 8 → 4 → 2 → 1 into a single operation.

---

## Stopping Time vs Total Stopping Time

- **Stopping time**: Steps until n drops below its starting value
- **Total stopping time**: Steps until n reaches 1 (what this problem asks)

For 837799:
- **Stopping time**: After some steps, the chain first drops below 837799
- **Total stopping time**: 524 steps (or 525 terms counting the start)

---

## Mathematical Insights

### 1. Memoization Transforms Complexity

**Without cache**: Each of the ~1M numbers walks an average ~120-step chain = ~120M steps total, but with massive overlap.

**With cache**: Each number is computed once. Total unique values visited ≈ n (for values < limit) + some overhead for values > limit.

**Key insight**: The Collatz graph (where edges go n → T(n)) is a tree rooted at 1. Memoization turns "walk from leaf to root" into "walk from leaf to nearest cached ancestor."

### 2. Why Dense Array Beats HashMap

For this problem, values 1 through 999,999 all need caching. A dense `Vec<u32>` is optimal:
- O(1) lookup (direct indexing)
- Cache-friendly (sequential memory access)
- ~4 MB total (1M x 4 bytes)

A HashMap would add per-entry overhead (~40 bytes/entry), increasing memory 10x and adding hash computation cost.

### 3. The Backfill Pattern

The "walk forward, backfill backward" pattern is elegant:

```
Forward:  collect [a, b, c, d] until hitting cached value e
Backward: d = cache[e] + steps_d
          c = d + steps_c
          b = c + steps_b
          a = b + steps_a
```

This avoids recursion (no stack overflow risk for long chains) while achieving the same result as top-down memoization.

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_collatz_example() {
    // From problem statement: 13 has chain of 10 terms
    assert_eq!(collatz_chain_length(13), 10);
}

#[test]
fn test_base_cases() {
    assert_eq!(collatz_chain_length(1), 1);
    assert_eq!(collatz_chain_length(2), 2);  // 2 → 1
}

#[test]
fn test_small_range() {
    // Under 10, starting number 9 has the longest chain (20 steps)
    assert_eq!(collatz_chain_length(9), 20);
    assert_eq!(longest_collatz_under(10), 9);
}

#[test]
fn test_solve() {
    assert_eq!(solve(), 837799);
}
```

### Cross-Validation

The naive `collatz_chain_length` function serves as an oracle to verify the memoized result:

```rust
let answer = solve();                           // memoized
let chain_len = collatz_chain_length(answer);   // naive
assert_eq!(chain_len, 525);                     // both agree
```

---

## Related Problems

### Project Euler

- **Problem 15**: Lattice paths (also uses DP/memoization)
- **Problem 31**: Coin sums (overlapping subproblems)
- **Problem 76**: Counting summations (memoized recursion)

### Mission Connections

- **Mission 11**: Memoization patterns — same cache-and-backfill approach
- **Mission 5**: HashMap vs alternatives — dense array chosen here for cache-friendliness

---

## Code Repository

**Location**: `project_euler/src/problems/p014.rs`

**Key functions**:
- `collatz_chain_length(n)` - Naive chain length (for verification)
- `longest_collatz_under(limit)` - Memoized solver with optimizations
- `solve()` - Returns answer (837799)

**Tests**: 4 unit tests

**Problem Statement**: `project_euler/Problem_Statements/p014.md`

---

## Metadata

**Tags**: `#collatz` `#memoization` `#dynamic-programming` `#number-theory` `#unsolved-conjecture` `#optimization`

**Concepts**: Collatz conjecture, memoization, even-number elimination, odd shortcut, dense cache, backfill pattern

**Difficulty**: Mathematical reasoning (easy-medium), Optimization (medium)

**Created**: 2026-02-26
**Language**: Rust
**Performance**: 23.3ms memoized / 99.9ms naive (4.3× speedup, limit 1M)

---

## Bidirectional Links

**Links From This Note**:
- [[collatz-conjecture]] - The unsolved conjecture itself
- [[memoization-pattern]] - Cache-and-backfill technique
- [[dynamic-programming]] - Overlapping subproblems framework
- [[mission-11]] - Same memoization patterns applied to string matching

**Links To This Note**:
- [[project-euler-problems]] - Full list of solved problems
- [[dynamic-programming]] - Uses P14 as memoization example
- [[number-theory]] - Collatz as number theory problem

---

*Last updated: 2026-02-26*
*Solution verified: ✅ Answer 837799 (chain length 525)*
*Both optimizations tested and verified*
