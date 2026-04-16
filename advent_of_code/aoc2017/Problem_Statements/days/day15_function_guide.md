# Day 15: Dueling Generators — Function Guide

**Problem**: Two generators produce values via `next = (prev * factor) % 2147483647`. Count pairs whose lowest 16 bits match.
**Answers**: Part 1 = **609**, Part 2 = **253**
**Code**: [day15.rs](../../src/solver/day15.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [parse_input](#parse_input)
3. [mersenne_mod & next_value](#mersenne_mod--next_value)
4. [mod_pow & jump_ahead](#mod_pow--jump_ahead)
5. [solve_part1_with_data](#solve_part1_with_data)
6. [solve_part2_with_data](#solve_part2_with_data)
7. [Optimization Journey](#optimization-journey)
8. [Benchmarks](#benchmarks)
9. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: two seed values (mine: A=883, B=879).

- Generator A: factor = 16807
- Generator B: factor = 48271
- Modulus: 2147483647 (Mersenne prime 2³¹ − 1, the classic Park–Miller LCG)

Both generators produce their next value by `(prev * factor) mod modulus`. A match happens when the low 16 bits of a pair agree.

- **Part 1**: 40,000,000 pairs, count matches directly.
- **Part 2**: only A-values that are multiples of 4 and B-values that are multiples of 8 count. Skip others and take 5,000,000 qualifying pairs.

---

## `parse_input`

```rust
struct Seeds { a: u64, b: u64 }

fn parse_input(input: &str) -> Seeds {
    let mut lines = input.lines();
    let a = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let b = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    Seeds { a, b }
}
```

- Two-line input; grab the last whitespace token per line.
- `u64` is mandatory: `prev * factor` can reach ~2³¹ × 48271 ≈ 2⁴⁶, well above `u32::MAX`.

---

## `mersenne_mod` & `next_value`

```rust
#[inline]
fn mersenne_mod(n: u64) -> u64 {
    let sum = (n & MODULUS) + (n >> 31);
    if sum >= MODULUS { sum - MODULUS } else { sum }
}

#[inline]
fn next_value(prev: u64, factor: u64) -> u64 {
    mersenne_mod(prev * factor)
}
```

This is **Optimization Step 1** (see [Optimization Journey](#optimization-journey)).

The modulus `2^31 - 1` is a Mersenne prime, which means `2^31 ≡ 1 (mod 2^31-1)`. Any product up to ~2⁴⁶ can be reduced without hardware division:

1. Split the 64-bit product at bit 31: lower 31 bits + upper bits
2. Add the two halves (the upper bits "wrap around" as 1× themselves)
3. The sum is at most `2 × MODULUS - 1`, so one conditional subtract finishes

This replaces a 64-bit `DIV` instruction (~35-90 cycles) with `AND` + `SHR` + `ADD` + `CMP` + `SUB` (~5 cycles). Called 45M+ times, this alone yielded a **1.8× speedup**.

---

## `mod_pow` & `jump_ahead`

```rust
fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MODULUS;
    while exp > 0 {
        if exp & 1 == 1 { result = mersenne_mod(result * base); }
        exp >>= 1;
        base = mersenne_mod(base * base);
    }
    result
}

fn jump_ahead(seed: u64, factor: u64, steps: u64) -> u64 {
    mersenne_mod(seed * mod_pow(factor, steps))
}
```

This is the mathematical foundation for **Optimization Step 2**.

For a multiplicative LCG `x_{n+1} = a * x_n mod m`, the value `k` steps ahead is:

```
x_{n+k} = a^k * x_n mod m
```

`mod_pow` computes `factor^k mod m` in O(log k) multiplications via exponentiation by squaring. This lets us compute the starting seed for any arbitrary position in the sequence without running through all prior steps — enabling parallel block decomposition.

Example: to find where generator A is after 5,000,000 steps:
- Naive: run `next_value` 5M times sequentially
- Jump-ahead: one `mod_pow(16807, 5_000_000)` call (~25 multiply-and-square operations)

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(seeds: &Seeds) -> u32 {
    let num_blocks = PART1_PAIRS / BLOCK_SIZE;  // 40M / 1M = 40 blocks

    (0..num_blocks)
        .into_par_iter()
        .map(|block| {
            let offset = block * BLOCK_SIZE;
            let a = jump_ahead(seeds.a, FACTOR_A, offset);
            let b = jump_ahead(seeds.b, FACTOR_B, offset);
            count_block(a, b, BLOCK_SIZE)
        })
        .sum()
}
```

**Parallelization strategy**: Split 40M pairs into 40 blocks of 1M each. For each block:
1. Compute starting seeds via `jump_ahead` (O(log n), ~microseconds)
2. Run 1M sequential pair comparisons within the block
3. Return match count

Rayon's `par_iter` distributes blocks across threads. Results `.sum()` to get the total.

The `count_block` inner loop uses `(a ^ b) & MASK_16 == 0` instead of masking both sides — one XOR + AND vs two ANDs + CMP.

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(seeds: &Seeds) -> u32 {
    let (vals_a, vals_b) = rayon::join(
        || generate_filtered_parallel(seeds.a, FACTOR_A, 3, PART2_PAIRS),
        || generate_filtered_parallel(seeds.b, FACTOR_B, 7, PART2_PAIRS),
    );

    vals_a.iter().zip(vals_b.iter())
        .filter(|(a, b)| a == b)
        .count() as u32
}
```

Part 2 uses **two levels of parallelism**:

**Outer level**: `rayon::join` runs generators A and B concurrently (same as before).

**Inner level**: Each generator's raw output is split into 16 parallel blocks via `generate_filtered_parallel`:

```rust
fn generate_filtered_parallel(seed: u64, factor: u64, filter_mask: u64, count: usize) -> Vec<u16> {
    let raw_estimate = count * (filter_mask + 1) * 105 / 100;  // 5% safety margin
    let num_blocks = 16;
    let block_raw = raw_estimate / num_blocks;

    // Each block: jump_ahead to its starting seed, generate raw values, filter
    let block_results: Vec<Vec<u16>> = (0..num_blocks)
        .into_par_iter()
        .map(|i| {
            let block_seed = jump_ahead(seed, factor, i * block_raw);
            generate_filtered_block(block_seed, factor, filter_mask, block_raw)
        })
        .collect();

    // Concatenate in order, trim to exactly 5M
    all.truncate(count);
    // Tail-fill if the 5% margin wasn't enough (rare)
}
```

**The key challenge**: We can't `jump_ahead` to a specific *filtered* position — we don't know how many raw values produce N filtered values. The solution:

1. **Estimate raw values needed**: For filter `& 3 == 0`, roughly 1-in-4 raw values pass. So 5M filtered values ≈ 20M raw values. Add 5% safety margin → 21M.
2. **Split raw range into 16 blocks**: Each block processes ~1.3M raw values via `jump_ahead`.
3. **Filter independently**: Each block produces a `Vec<u16>` of varying length.
4. **Concatenate in order**: Sort by block index, extend into one Vec.
5. **Trim to exactly 5M**: The over-estimate guarantees enough (or a small tail-fill handles the rare miss).

Each `generate_filtered_block` call:
- Starts from a precomputed seed (via `jump_ahead`)
- Spins the generator for a fixed number of raw steps
- Keeps only values passing the filter (`& 3 == 0` or `& 7 == 0`)
- Stores as `u16` (already masked to low 16 bits) — halves memory vs `u64`

The final zip-and-compare is a cache-friendly linear scan over two contiguous `u16` arrays (~10MB total).

---

## Optimization Journey

Three distinct versions, each building on the last:

### Step 0: Naive Baseline — 333ms

```rust
fn next_value(prev: u64, factor: u64) -> u64 {
    (prev * factor) % MODULUS        // hardware 64-bit DIV
}
// Sequential loop: 40M pairs for Part 1, filtered spin for Part 2
```

Straightforward simulation. The `%` operator on `u64` emits a hardware `DIV` instruction, which is the most expensive single-operand instruction on x86 (~35-90 cycles). Called 45M+ times, this dominates runtime.

### Step 1: Fast Mersenne Modulo — 186ms (1.8× faster)

```rust
fn mersenne_mod(n: u64) -> u64 {
    let sum = (n & MODULUS) + (n >> 31);       // split at bit 31, add halves
    if sum >= MODULUS { sum - MODULUS } else { sum }  // at most one subtract
}
```

**Key insight**: 2³¹ ≡ 1 (mod 2³¹−1). The upper bits of the product are equivalent to themselves modulo the Mersenne prime. Split + add + conditional subtract replaces the division entirely.

**Why it works**: For any `n < MODULUS²` (guaranteed since both operands < MODULUS and MODULUS < 2³¹):
- `n & MODULUS` extracts the lower 31 bits (range 0..2³¹−1)
- `n >> 31` extracts the upper bits (range 0..48270 for factor B)
- Their sum is at most `(2³¹ − 2) + 48270`, which fits in 32 bits
- If sum ≥ MODULUS, one subtraction brings it into range

**Impact**: Eliminated ~80 cycles per call × 45M calls ≈ 3.6 billion saved cycles.

### Step 2: Rayon Part 1 Block Decomposition — 85ms (3.9× faster than baseline)

```rust
// Part 1: jump-ahead enables independent blocks
let a = jump_ahead(seeds.a, FACTOR_A, offset);  // O(log n) via mod_pow
// ... then 1M sequential steps within the block

// Part 2: independent generators run on separate threads
rayon::join(|| generate_filtered(A), || generate_filtered(B));
```

**Key insight**: For a multiplicative LCG, `x_{n+k} = factor^k * x_n mod m`. Computing `factor^k` via exponentiation by squaring takes O(log k) steps (~25 multiplies for k=1M), letting us compute any position's starting seed instantly.

**Part 1 parallelization**:
- Split 40M pairs into 40 blocks of 1M
- Each block computes its own starting seeds via `jump_ahead`
- Blocks execute independently on Rayon's thread pool
- Match counts sum at the end

**Part 2 parallelization** (limited — only 2 threads):
- `rayon::join` runs A and B generation on separate threads
- Each generator still runs sequentially on a single core
- Part 2 bottleneck: the slower generator (B, ~1-in-8 filter) holds up the result

### Step 3: Rayon Part 2 Block Decomposition — 22.7ms (14.7× faster than baseline)

```rust
// Part 2: BOTH generators now use parallel block decomposition
fn generate_filtered_parallel(seed, factor, filter_mask, count) -> Vec<u16> {
    let raw_estimate = count * (filter_mask + 1) * 105 / 100;  // over-estimate
    let num_blocks = 16;

    (0..num_blocks).into_par_iter().map(|i| {
        let block_seed = jump_ahead(seed, factor, i * block_raw);
        generate_filtered_block(block_seed, factor, filter_mask, block_raw)
    }).collect()
    // → concatenate in order, trim to exactly 5M
}

// Outer level still uses rayon::join for A vs B
rayon::join(
    || generate_filtered_parallel(A, 3),   // 16 blocks × all cores
    || generate_filtered_parallel(B, 7),   // 16 blocks × all cores
);
```

**Key insight**: Part 2's filtered streams can't use `jump_ahead` directly (we don't know which raw position gives the Nth filtered value). But we CAN:

1. **Estimate** the raw values needed: filter `& 3 == 0` passes ~1-in-4 → 5M filtered ≈ 20M raw
2. **Over-allocate by 5%**: 21M raw values, split into 16 blocks of ~1.3M each
3. **Jump-ahead to each block's starting seed**: Same `mod_pow` trick as Part 1
4. **Filter independently per block**: Each block produces a `Vec<u16>` of variable length
5. **Concatenate and trim**: Sort blocks by index, join, truncate to exactly 5M
6. **Tail-fill if needed**: If the 5% margin wasn't enough, generate remaining sequentially (rare)

**Why 16 blocks?** Balances parallelism vs overhead. With 8 cores:
- 2 blocks (Step 2): only 2 cores active → other 6 idle
- 16 blocks: all cores stay busy, block granularity fine enough for load balancing
- 64+ blocks: diminishing returns, `jump_ahead` overhead starts to matter

**Result**: Part 2 went from ~30ms (2 threads) to ~8ms (all cores).

### Summary

| Version | Time | Speedup | Technique |
|---------|------|---------|-----------|
| Step 0: Naive | 333ms | 1.0× | `% MODULUS` + sequential loop |
| Step 1: Fast mod | 186ms | 1.8× | Mersenne bit-trick eliminates DIV |
| Step 2: + Rayon (Part 1) | 85ms | 3.9× | Part 1 block decomposition + jump-ahead |
| Step 3: + Rayon (Part 2) | 22.7ms | 14.7× | Part 2 estimated-block decomposition |

### What Would Step 4 Look Like?

AVX2 SIMD could process 4 generator pairs per cycle using 256-bit registers:
- Pack 4 lane states, advance all with `factor^4` as the step
- Compare 4 pairs simultaneously
- Estimated additional 3-4× → ~5-8ms target
- Requires `std::arch::x86_64` intrinsics (unsafe, platform-specific)

---

## Benchmarks

| Part | Time | Notes |
|------|------|-------|
| Combined (parse + both parts) | **22.7ms** | Full Rayon parallelism + fast mod |
| Part 1 (40 parallel blocks) | ~15ms (est.) | 40 × 1M pairs across all cores |
| Part 2 (16 blocks × 2 generators) | ~8ms (est.) | Estimated-block decomposition, all cores |

Target: <100ms ✅ (4.4× under budget)

---

## Key Patterns

### Mersenne prime fast modulo
`2^31 - 1` is special: `n mod (2^31-1)` = `(lower 31 bits) + (upper bits)`, with at most one subtraction. Eliminates hardware division from the hot loop. This trick works for ANY Mersenne prime (2^p - 1), not just 2^31 - 1.

### LCG jump-ahead via modular exponentiation
For `x_{n+1} = a * x_n mod m`, position `k` is `a^k * x_0 mod m`. Exponentiation by squaring computes this in O(log k), enabling parallel block decomposition of an otherwise sequential recurrence.

### XOR-based comparison
`(a ^ b) & MASK == 0` tests equality of masked bits with one fewer operation than `(a & MASK) == (b & MASK)`. Marginal per call, meaningful at 40M iterations.

### Two-level parallelism with `rayon::join` + `par_iter`
When two generators are independent (outer level), `rayon::join` runs both simultaneously. Within each generator, `par_iter` over 16 blocks spreads the raw generation across all cores (inner level). The `Vec<u16>` buffers trade ~10MB memory for thread independence.

### Over-estimate + trim for filtered parallel streams
When you can't predict how many raw inputs produce N filtered outputs, estimate conservatively (e.g., `N × filter_ratio × 1.05`), split the raw range into parallel blocks, filter each independently, concatenate in order, and trim. A small sequential tail-fill handles the rare under-estimate.

### u64 for the LCG product
Park–Miller needs ~46 bits of headroom for `prev * factor` before the modulo. `u32` would overflow; `u128` would cost a multiprecision mul. `u64` is the sweet spot.
