# Day 11: Cache Analysis - Memoization Deep Dive

*See [[../Problem_Statements/day11]] for the complete problem statement.*

## Problem Overview

**Stone Transformation Rules:**
1. Stone with value `0` → becomes `1`
2. Stone with even number of digits → splits into two stones (left half, right half)
3. Otherwise → multiply by 2024

**Challenge:** Count total stones after N blinks (Part 1: 25 blinks, Part 2: 75 blinks)

## Naive vs. Memoized Approach

### Naive Simulation
The straightforward approach simulates each blink by maintaining a vector of all stones:

```rust
fn simulate_blinks(initial_stones: Vec<u64>, blinks: usize) -> Vec<u64> {
    let mut stones = initial_stones;
    
    for _ in 0..blinks {
        stones = stones
            .into_iter()
            .flat_map(transform_stone)
            .collect();
    }
    
    stones
}
```

**Example progression with `[125, 17]`:**
```
Blink 0: [125, 17]                           // 2 stones
Blink 1: [253000, 1, 7]                      // 3 stones
Blink 2: [253, 0, 2024, 14168]               // 4 stones
Blink 3: [512072, 1, 20, 24, 28676032]       // 5 stones
...
Blink 25: [stone1, stone2, ..., stone55312]  // 55,312 stones in memory!
```

### Performance Breakdown: Naive Simulation

| Blinks | Stones | Time | Memory (est.) | Status |
|--------|--------|------|---------------|--------|
| 25 | 55,312 | 20ms | ~440 KB | ✅ Acceptable |
| 30 | 445,882 | 162ms | ~3.5 MB | ✅ Still OK |
| 35 | 3,604,697 | 1.3s | ~29 MB | ⚠️ Time becoming painful |
| 40 | 29,115,525 | 10.6s | ~233 MB | ❌ Unacceptable |
| 75 | ~10^15+ | Hours/Days | Terabytes | 💥 Impossible |

**Growth pattern:** Roughly exponential due to stone splitting (Rule 2).

### Memoization Approach

Instead of tracking all stones, we use **dynamic programming with caching**:

```rust
fn count_stones_memoized(
    stone: u64,
    blinks_remaining: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    // Base case: no more blinks
    if blinks_remaining == 0 {
        return 1;
    }

    // Check cache
    if let Some(&cached) = cache.get(&(stone, blinks_remaining)) {
        return cached;
    }

    // Apply transformation and recursively count
    let transformed = transform_stone(stone);
    let count = transformed
        .iter()
        .map(|&s| count_stones_memoized(s, blinks_remaining - 1, cache))
        .sum();

    // Cache the result
    cache.insert((stone, blinks_remaining), count);
    count
}
```

**Key insight:** We cache `(stone_value, blinks_remaining) → stone_count` to avoid recomputing identical subproblems.

## Performance Comparison: Naive vs. Memoized

| Blinks | Naive Time | Memoized Time | Speedup |
|--------|-----------|---------------|---------|
| 10 | 128µs | 164µs | 0.8x (overhead) |
| 15 | 332µs | 263µs | 1.3x |
| 20 | 2.5ms | 497µs | 5x |
| 25 | 20ms | 711µs | **28x** |

At 25 blinks, memoization is already **28× faster**. Beyond 40 blinks, naive becomes computationally infeasible.

## Cache Efficiency Analysis

### Example Input: `[125, 17]`

| Blinks | Total Stones | Cache Entries | Ratio |
|--------|--------------|---------------|-------|
| 10 | 109 | 124 | 1.1× |
| 25 | 55,312 | 891 | **62×** |
| 50 | 1,900,433,601 | 2,241 | **848,000×** |
| 75 | 65,601,038,650,482 | 3,591 | **18,267,000,000×** |

### Puzzle Input: `[77, 515, 6779622, 6, 91370, 959685, 0, 9861]`

| Blinks | Total Stones | Cache Entries | Memory (est.) |
|--------|--------------|---------------|---------------|
| 25 | 187,738 | 3,420 | ~55 KB |
| 75 | 223,767,210,249,237 | 129,787 | ~2 MB |

**For 75 blinks:**
- Naive would need: ~2 **petabytes** of RAM (223 trillion u64 values)
- Memoized needs: ~**2 MB** of RAM (130K cache entries)
- **That's a 1,000,000,000,000× memory reduction!**

## Traced Example: `[0, 1, 2]` with 5 Blinks

### Execution Trace

```
Processing initial stone: 0
  Stone 0 with 5 blinks → transforms to [1]
    Stone 1 with 4 blinks → transforms to [2024]
      Stone 2024 with 3 blinks → transforms to [20, 24]
        Stone 20 with 2 blinks → transforms to [2, 0]
          Stone 2 with 1 blinks → transforms to [4048]
            Stone 4048 with 0 blinks → count = 1 (base case)
          → CACHE: (2, 1) = 1
          Stone 0 with 1 blinks → transforms to [1]
            Stone 1 with 0 blinks → count = 1 (base case)
          → CACHE: (0, 1) = 1
        → CACHE: (20, 2) = 2
        Stone 24 with 2 blinks → transforms to [2, 4]
          Stone 2 with 1 blinks → count = 1 (CACHED ✓)  ← REUSE!
          Stone 4 with 1 blinks → transforms to [8096]
            Stone 8096 with 0 blinks → count = 1 (base case)
          → CACHE: (4, 1) = 1
        → CACHE: (24, 2) = 2
      → CACHE: (2024, 3) = 4
    → CACHE: (1, 4) = 4
  → CACHE: (0, 5) = 4
→ Stone 0 produces 4 stones after 5 blinks

Processing initial stone: 1
  Stone 1 with 5 blinks → transforms to [2024]
    Stone 2024 with 4 blinks → transforms to [20, 24]
      Stone 20 with 3 blinks → transforms to [2, 0]
        Stone 2 with 2 blinks → transforms to [4048]
          Stone 4048 with 1 blinks → transforms to [40, 48]
            Stone 40 with 0 blinks → count = 1 (base case)
            Stone 48 with 0 blinks → count = 1 (base case)
          → CACHE: (4048, 1) = 2
        → CACHE: (2, 2) = 2
        Stone 0 with 2 blinks → transforms to [1]
          Stone 1 with 1 blinks → transforms to [2024]
            Stone 2024 with 0 blinks → count = 1 (base case)
          → CACHE: (1, 1) = 1
        → CACHE: (0, 2) = 1
      → CACHE: (20, 3) = 3
      Stone 24 with 3 blinks → transforms to [2, 4]
        Stone 2 with 2 blinks → count = 2 (CACHED ✓)  ← REUSE!
        Stone 4 with 2 blinks → transforms to [8096]
          Stone 8096 with 1 blinks → transforms to [80, 96]
            Stone 80 with 0 blinks → count = 1 (base case)
            Stone 96 with 0 blinks → count = 1 (base case)
          → CACHE: (8096, 1) = 2
        → CACHE: (4, 2) = 2
      → CACHE: (24, 3) = 4
    → CACHE: (2024, 4) = 7
  → CACHE: (1, 5) = 7
→ Stone 1 produces 7 stones after 5 blinks

Processing initial stone: 2
  Stone 2 with 5 blinks → transforms to [4048]
    Stone 4048 with 4 blinks → transforms to [40, 48]
      Stone 40 with 3 blinks → transforms to [4, 0]
        Stone 4 with 2 blinks → count = 2 (CACHED ✓)  ← REUSE!
        Stone 0 with 2 blinks → count = 1 (CACHED ✓)  ← REUSE!
      → CACHE: (40, 3) = 3
      Stone 48 with 3 blinks → transforms to [4, 8]
        Stone 4 with 2 blinks → count = 2 (CACHED ✓)  ← REUSE!
        Stone 8 with 2 blinks → transforms to [16192]
          Stone 16192 with 1 blinks → transforms to [32772608]
            Stone 32772608 with 0 blinks → count = 1 (base case)
          → CACHE: (16192, 1) = 1
        → CACHE: (8, 2) = 1
      → CACHE: (48, 3) = 3
    → CACHE: (4048, 4) = 6
  → CACHE: (2, 5) = 6
→ Stone 2 produces 6 stones after 5 blinks
```

### Final Results

```
Total stones: 17
Cache entries: 24
```

### Complete Cache Contents

Sorted by `(blinks_remaining, stone_value)`:

```
Blinks=1:
  (0, 1) → 1    (1, 1) → 1    (2, 1) → 1    (4, 1) → 1
  (4048, 1) → 2    (8096, 1) → 2    (16192, 1) → 1

Blinks=2:
  (0, 2) → 1    (2, 2) → 2    (4, 2) → 2    (8, 2) → 1
  (20, 2) → 2   (24, 2) → 2

Blinks=3:
  (20, 3) → 3   (24, 3) → 4   (40, 3) → 3   (48, 3) → 3
  (2024, 3) → 4

Blinks=4:
  (1, 4) → 4    (2024, 4) → 7    (4048, 4) → 6

Blinks=5:
  (0, 5) → 4    (1, 5) → 7    (2, 5) → 6
```

## Cache Reuse Examples

**Stone 2 with 1 blink:**
- Computed once during stone 0's recursion
- Reused when stone 1's path encounters it
- **1 computation, 1 cache hit**

**Stone 0 with 2 blinks:**
- Computed during stone 1's recursion
- Reused when stone 2's path encounters it
- **1 computation, 1 cache hit**

**Stone 4 with 2 blinks:**
- Computed once during stone 1's recursion
- Reused **3 times** during stone 2's recursion (appears in multiple branches)
- **1 computation, 3 cache hits**

Even in this tiny example with only 5 blinks, we see multiple cache hits eliminating redundant work.

## Why Memoization Works So Well

### Overlapping Subproblems

The transformation rules create **convergent paths**:
- Stone `0` is extremely common (appears from even-digit splits ending in 0)
- Stone `1` appears repeatedly (from `0 → 1` transformation)
- Common sequences like `1 → 2024 → [20, 24]` get reused extensively

### Cache Growth vs. Stone Count

As blinks increase:
- **Stone count:** Grows exponentially (~2^N for worst case)
- **Cache entries:** Grows sub-linearly (unique state combinations)

At 75 blinks:
- Stones: 223 **trillion** (would need 1.7 petabytes)
- Cache: 130 **thousand** (needs ~2 megabytes)

### Why It's Both a Time AND Memory Problem

**Time becomes the bottleneck first:**
- 40 blinks: 10.6 seconds (still fits in RAM but painfully slow)
- 75 blinks: Would take weeks/months even if memory allowed

**Memory becomes impossible second:**
- 50+ blinks: Billions/trillions of u64 values exceed available RAM
- Cache approach sidesteps this entirely

## Key Takeaways

1. **Naive simulation** works for small inputs but grows exponentially
2. **Memoization** exploits overlapping subproblems to cache computation paths
3. **Cache key is crucial:** `(stone_value, blinks_remaining)` captures the recursive state
4. **Memory efficiency:** Cache stores ~0.0001% of what naive would require
5. **Time efficiency:** 28× faster at 25 blinks, impossible to compare at 75 (naive can't finish)

## Running the Analysis

```bash
# Run cache size analysis
cargo test -p aoc2024 day11::tests::test_cache_size_analysis --lib -- --nocapture

# Run performance comparison (naive vs. memoized)
cargo test -p aoc2024 day11::tests::test_memoized_vs_naive_comparison --lib -- --nocapture --ignored

# Run detailed trace with [0, 1, 2] for 5 blinks
cargo test -p aoc2024 day11::tests::test_trace_example_5_blinks --lib -- --nocapture

# See naive simulation limits
cargo test -p aoc2024 day11::tests::test_naive_simulation_limits --lib -- --nocapture --ignored
```

## Solutions

- **Part 1 (25 blinks):** 187,738 stones
- **Part 2 (75 blinks):** 223,767,210,249,237 stones

---
*Links: [[../Problem_Statements/day11]]*
