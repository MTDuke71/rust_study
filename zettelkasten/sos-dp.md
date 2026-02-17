# SOS DP - Sum Over Subsets Dynamic Programming

**Type**: Algorithm Note
**Source**: AoC 2022 Day 16 (Proboscidea Volcanium)
**Related**: [[top-down-dp-pattern]] | [[dp-aggregation-patterns]] | [[boolean-to-counting-dp]]

---

## Overview

SOS DP propagates information across all subsets of a bitmask in O(n × 2^n) time, answering questions like "what's the best value achievable using **any subset** of these items?" for every possible item set simultaneously.

**Key Insight**: Without SOS DP, finding the best over all subsets of a mask requires O(3^n) submask enumeration. SOS DP reduces this to O(n × 2^n) by reusing intermediate results — the same "avoid recomputation" principle as all DP.

---

## The Problem SOS DP Solves

Given an array `f[mask]` indexed by bitmask, compute for every mask `m`:

```
g[m] = max( f[s] ) for all s ⊆ m
```

In words: for each set of items `m`, find the best value achievable by any subset of `m`.

**Why this matters**: Many problems require partitioning items between two actors. If actor A takes set `m` and actor B gets the complement `full ^ m`, you need:
- `best_A[m]` = best A can do with any subset of `m`
- `best_B[full ^ m]` = best B can do with any subset of the remaining items

SOS DP makes `best[m]` already represent "best over any subset of m," so the partition scan becomes a simple O(2^n) loop.

---

## The Algorithm

### Naive Approach: O(3^n) Submask Enumeration

```rust
// For each mask, iterate all its submasks
for mask in 0..size {
    let mut sub = mask;
    loop {
        g[mask] = g[mask].max(f[sub]);
        if sub == 0 { break; }
        sub = (sub - 1) & mask;  // Next submask trick
    }
}
```

Why O(3^n)? Each bit is in one of 3 states: (1) in mask and in sub, (2) in mask but not in sub, (3) not in mask. Total iterations = 3^n.

For n=15: 3^15 = 14,348,907 iterations. Workable but wasteful.

### SOS DP Approach: O(n × 2^n)

```rust
// g[mask] starts as f[mask]
let mut g = f.clone();

for i in 0..n {
    for mask in 0..size {
        if mask & (1 << i) != 0 {
            g[mask] = g[mask].max(g[mask ^ (1 << i)]);
        }
    }
}
```

For n=15: 15 × 32,768 = 491,520 iterations. **29× fewer** than naive.

---

## How It Works

### Mental Model: Dimension-by-Dimension Relaxation

Think of each bit position as a **dimension**. SOS DP relaxes one dimension at a time.

**Example with 3 bits (n=3, items A, B, C):**

```
Initial: g[mask] = f[mask] (best for exactly this set)

Pass i=0 (relax bit A):
  g[101] = max(g[101], g[100])  ← "with or without A"
  g[011] = max(g[011], g[010])
  g[111] = max(g[111], g[110])
  ...

Pass i=1 (relax bit B):
  g[110] = max(g[110], g[100])  ← "with or without B"
  g[011] = max(g[011], g[001])
  g[111] = max(g[111], g[101])  ← 101 already includes "±A" from pass 0
  ...

Pass i=2 (relax bit C):
  g[111] = max(g[111], g[011])  ← now covers ALL subsets of 111
  ...
```

After pass `i`, `g[mask]` is the best over all subsets that differ from `mask` only in bits 0..i. After all n passes, `g[mask]` = best over ALL subsets of `mask`.

### Why It's Correct

After processing dimension `i`, the value `g[mask]` represents:

```
g[mask] = max( f[s] ) for all s that:
  - match mask in bits i+1..n-1 (upper bits identical)
  - are a subset of mask in bits 0..i (lower bits relaxed)
```

Each pass doubles the number of subsets considered per mask. After n passes, all 2^(popcount(mask)) subsets have been considered.

### Analogy to Other DP

This is the same principle as Floyd-Warshall for shortest paths:
- Floyd-Warshall: "relax through intermediate node k" for k=0..n
- SOS DP: "relax along dimension i" for i=0..n

Both build optimal results incrementally by considering one new degree of freedom per pass.

---

## AoC 2022 Day 16 Application

### The Problem

You and an elephant each open disjoint sets of valves to maximize total pressure released. Each actor starts at valve AA with 26 minutes.

### Why SOS DP?

**Phase 1** — DFS records `best_for_mask[opened]`: the best pressure achievable by one actor opening exactly the valves in `opened`.

**Phase 2** — SOS DP propagates so `best_for_mask[m]` = best achievable by opening **any subset** of `m`. This is needed because neither actor is required to open ALL valves available to them — just the best reachable subset within 26 minutes.

**Phase 3** — Scan all masks: `answer = max(best[m] + best[full ^ m])`.

```rust
// Phase 1: DFS with 26 minutes, record best per opened mask
let mut best_for_mask = vec![0u32; 1 << n];  // 2^15 = 32,768 entries
dfs_collect(data, 0, 0u32, 26, 0, n, &mut best_for_mask);

// Phase 2: SOS DP — propagate best over all subsets
for i in 0..n {
    for mask in 0..size {
        if mask & (1 << i) != 0 {
            best_for_mask[mask] = best_for_mask[mask]
                .max(best_for_mask[mask ^ (1 << i)]);
        }
    }
}

// Phase 3: Find best disjoint partition
let full = size - 1;
let mut best = 0u32;
for m in 0..size {
    best = best.max(best_for_mask[m] + best_for_mask[full ^ m]);
}
```

### Without SOS DP

You'd need to iterate all submask pairs for each partition, which is O(3^n) = 14M operations. With SOS DP: O(n × 2^n + 2^n) ≈ 500K operations. **29× faster.**

In practice, both are fast for n=15 (microseconds), but SOS DP is the principled approach that scales.

---

## Complexity Analysis

| Component | Time | Space |
|-----------|------|-------|
| SOS DP propagation | O(n × 2^n) | O(2^n) — in-place |
| Partition scan | O(2^n) | O(1) |
| Naive submask enum | O(3^n) | O(2^n) |

**For n=15 (Day 16):**

| Approach | Operations | Measured Time |
|----------|-----------|---------------|
| SOS DP + scan | ~500K | ~5µs |
| Naive submask | ~14M | ~50µs |

Both negligible vs the DFS (~1.15ms), but the gap widens with larger n.

**Scaling:**

| n | SOS DP (n × 2^n) | Naive (3^n) | Ratio |
|---|-------------------|-------------|-------|
| 10 | 10,240 | 59,049 | 6× |
| 15 | 491,520 | 14,348,907 | 29× |
| 20 | 20,971,520 | 3,486,784,401 | 166× |
| 25 | 838,860,800 | 847,288,609,443 | 1,010× |

SOS DP becomes essential as n grows beyond ~20.

---

## Operator Variants

SOS DP works with any associative, commutative operator — not just `max`:

| Operator | Propagation | Use Case |
|----------|------------|----------|
| **max** | `g[m] = max(g[m], g[m ^ bit])` | Best value over subsets |
| **min** | `g[m] = min(g[m], g[m ^ bit])` | Worst case over subsets |
| **sum** | `g[m] = g[m] + g[m ^ bit]` | Total over subsets (inclusion-exclusion prep) |
| **or** | `g[m] = g[m] \| g[m ^ bit]` | Reachability over subsets |
| **gcd** | `g[m] = gcd(g[m], g[m ^ bit])` | GCD over subsets |

The `sum` variant is particularly common in competitive programming for Möbius inversion / inclusion-exclusion problems.

---

## When to Use SOS DP

### Use When

- **Bitmask state space**: Items represented as bits (n ≤ ~25)
- **Subset queries**: Need best/sum/count over all subsets of each mask
- **Partition problems**: Two actors splitting items optimally
- **Multiple queries**: Need answer for many different masks simultaneously

### Don't Use When

- **n > 25**: 2^25 = 33M entries, memory becomes an issue
- **Single query**: Only need subset answer for one specific mask → direct submask enumeration is simpler
- **Non-subset relationship**: Items have dependencies that break the subset structure
- **No bitmask representation**: If items don't map cleanly to bits

---

## Common Variations

### Superset SOS (Reverse Direction)

Propagate over supersets instead of subsets:

```rust
for i in 0..n {
    for mask in 0..size {
        if mask & (1 << i) == 0 {  // ← Note: == 0, not != 0
            g[mask] = g[mask].max(g[mask | (1 << i)]);
        }
    }
}
```

After this, `g[mask]` = max over all supersets of `mask`.

### Counting Subsets with Property

Combined with the `sum` operator, SOS DP counts how many subsets satisfy some condition:

```rust
// f[mask] = 1 if mask is "valid", 0 otherwise
// After SOS DP with sum: g[mask] = number of valid subsets of mask
for i in 0..n {
    for mask in 0..size {
        if mask & (1 << i) != 0 {
            g[mask] += g[mask ^ (1 << i)];
        }
    }
}
```

---

## Implementation Pitfalls

### Pitfall 1: Wrong Loop Order

```rust
// ❌ WRONG: mask loop outside, i loop inside
for mask in 0..size {
    for i in 0..n {
        if mask & (1 << i) != 0 {
            g[mask] = g[mask].max(g[mask ^ (1 << i)]);
        }
    }
}
```

This doesn't propagate correctly — `g[mask ^ (1 << i)]` hasn't been fully relaxed yet for dimensions 0..i-1 when you use it.

The **outer loop must be over dimensions** (`i`), inner loop over masks.

### Pitfall 2: Forgetting the Condition Check

```rust
// ❌ WRONG: Missing the bit check
for i in 0..n {
    for mask in 0..size {
        // Missing: if mask & (1 << i) != 0
        g[mask] = g[mask].max(g[mask ^ (1 << i)]);
    }
}
```

Without the check, `mask ^ (1 << i)` is a **superset** of `mask` when bit `i` is 0, which propagates in the wrong direction.

### Pitfall 3: Using SOS DP for Non-Commutative Operations

SOS DP requires the operator to be **commutative** and **associative**. Operations like "first found" or "lexicographically smallest path" don't work because the order of combination matters.

---

## Key Takeaways

1. **O(n × 2^n) beats O(3^n)** for subset queries — same principle as all DP (avoid recomputation)
2. **Dimension-by-dimension relaxation** — relax one bit at a time, each pass doubles subsets considered
3. **Outer loop = dimensions, inner loop = masks** — loop order matters for correctness
4. **Partition problems** are the killer app — SOS DP enables O(2^n) partition scan for two-actor problems
5. **Scales with any commutative associative operator** — max, min, sum, or, gcd all work

---

## Related Concepts

- [[top-down-dp-pattern]] — General memoization template (SOS DP is bottom-up but same "avoid recomputation" principle)
- [[dp-aggregation-patterns]] — SOS DP aggregates with MAX/SUM/etc. operators
- [[boolean-to-counting-dp]] — Operator transformation pattern (OR→SUM parallels SOS operator variants)
- [[Dynamic Programming]] — General DP overview

**AoC Applications:**
- AoC 2022 Day 16 — Two-actor valve optimization (partition with MAX)

---

*Tags: #algorithm #dynamic-programming #bitmask #optimization #competitive-programming #aoc-pattern*

*Links: [[top-down-dp-pattern]] | [[dp-aggregation-patterns]] | [[boolean-to-counting-dp]] | [[Dynamic Programming]]*
