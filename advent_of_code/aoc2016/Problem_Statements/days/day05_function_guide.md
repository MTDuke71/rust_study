# Day 5: How About a Nice Game of Chess? --- Function Guide

**Problem**: Mine MD5 hashes with 5 leading zeroes to extract password characters.
**Answers**: Part 1 = **d4cd2ee1**, Part 2 = **f2c730e5**
**Code**: [day05.rs](../../src/solver/day05.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Algorithm Overview](#algorithm-overview)
3. [find_hits_in_batch](#find_hits_in_batch)
4. [solve_both](#solve_both)
5. [Benchmarks](#benchmarks)
6. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: A door ID string (`ugkcyxxp`).

**Part 1**: Compute `MD5(door_id + index)` for increasing indices starting at 0. When the hex hash starts with 5 zeroes (`00000...`), the 6th hex character is the next password character. Collect 8 characters left-to-right.

**Part 2**: Same hash mining, but the 6th hex char is the **position** (0-7) and the 7th hex char is the **character** to place there. First valid character for each position wins. Ignore positions >= 8.

**Example** (Door ID = `abc`):
- Index 3231929 -> hash `00000155...` -> Part 1: char `1`, Part 2: char `5` at pos `1`
- Password 1: `18f47a30`
- Password 2: `05ace8e3`

---

## Algorithm Overview

This is a brute-force hash mining problem --- no algorithmic shortcut exists. Two optimizations make it fast:

1. **Single-pass**: Mine once, fill both passwords simultaneously (Part 1 comes free while mining for Part 2)
2. **Rayon parallel batches**: Process 500K indices per batch across all CPU cores

```
while either password incomplete:
    batch = next 500K indices
    hits = parallel_mine(batch)  // Rayon par_iter
    sort hits by index           // preserve ordering
    for each hit:
        fill Part 1 (sequential) + Part 2 (positional)
```

---

## `find_hits_in_batch`

```rust
const BATCH_SIZE: u64 = 500_000;

fn find_hits_in_batch(door_id: &[u8], start: u64) -> Vec<(u64, [u8; 16])> {
    (start..start + BATCH_SIZE)
        .into_par_iter()
        .filter_map(|i| {
            let mut hasher = Md5::new();
            hasher.update(door_id);
            hasher.update(i.to_string().as_bytes());
            let hash = hasher.finalize();
            if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
                Some((i, hash.into()))
            } else {
                None
            }
        })
        .collect()
}
```

**Rayon `par_iter`**: Distributes 500K hash computations across all CPU cores. Each hash is completely independent --- textbook embarrassingly parallel workload.

**Returns `(index, hash)` pairs**: The index is needed to sort results back into order after parallel execution (Rayon doesn't guarantee ordering).

**Five-zero check via bytes**: `hash[0] == 0 && hash[1] == 0 && hash[2] < 16` checks the first 5 hex chars are zero without any string conversion.

**Batch size choice**: 500K balances Rayon overhead vs. responsiveness. Too small = thread pool overhead dominates. Too large = wasted work after passwords are complete.

---

## `solve_both`

```rust
fn solve_both(door_id: &str) -> (String, String) {
    let id_bytes = door_id.as_bytes();
    let mut p1 = String::with_capacity(8);
    let mut p2 = [None; 8];
    let mut p2_found = 0;
    let mut batch_start = 0u64;

    while p1.len() < 8 || p2_found < 8 {
        let mut hits = find_hits_in_batch(id_bytes, batch_start);
        hits.sort_unstable_by_key(|(i, _)| *i);

        for (_, hash) in hits {
            if p1.len() < 8 {
                let nibble = hash[2] & 0x0F;
                p1.push(char::from_digit(nibble as u32, 16).unwrap());
            }
            if p2_found < 8 {
                let pos = (hash[2] & 0x0F) as usize;
                if pos < 8 && p2[pos].is_none() {
                    let nibble = (hash[3] >> 4) & 0x0F;
                    p2[pos] = Some(char::from_digit(nibble as u32, 16).unwrap());
                    p2_found += 1;
                }
            }
        }
        batch_start += BATCH_SIZE;
    }

    let p2_str: String = p2.iter().map(|c| c.unwrap()).collect();
    (p1, p2_str)
}
```

**Single-pass design**: Each qualifying hash is checked against both passwords. Part 1 always completes first (only needs 8 sequential hits), then Part 2 continues filling positional slots.

**Sort after parallel**: `sort_unstable_by_key` restores index order so Part 1 gets characters in the correct sequence. Part 2 needs ordering too (first-wins semantics).

**Early termination at batch boundary**: The `while` loop checks after each batch. At most 500K extra hashes are computed beyond what's needed.

---

## Benchmarks

### Current (single-pass + Rayon)

| Function | Time |
|----------|------|
| `solve` (combined) | **262ms** |
| `solve_part1` (via solve_both) | 262ms |
| `solve_part2` (via solve_both) | 262ms |

### Before optimization (sequential, two-pass)

| Function | Time |
|----------|------|
| `solve_part1` (standalone) | 1.15s |
| `solve_part2` (standalone) | 2.90s |
| `solve` (combined) | 4.05s |

### Speedup analysis

| Metric | Before | After | Speedup |
|--------|--------|-------|---------|
| Combined | 4.05s | 262ms | **15.5x** |
| Part 1 only | 1.15s | 262ms | 4.4x |
| Part 2 only | 2.90s | 262ms | 11.1x |

**Why all three benchmarks are ~262ms**: `solve_both` always mines until Part 2 completes (the harder task). Part 1 finishes early and rides for free. Standalone part1/part2 both call `solve_both` internally.

**15.5x speedup sources**:
- **~2.5x from single-pass**: Eliminated redundant Part 1 mining (was iterating from 0 twice)
- **~6x from Rayon**: Parallelized across CPU cores (embarrassingly parallel workload)

### Mining statistics (measured)

| Metric | Value |
|--------|-------|
| Batch size | 500,000 |
| Batches needed | 51 |
| Hashes searched | 25,500,000 (25.5M) |
| Qualifying hits | 25 |
| Hit rate (measured) | 1 in 1,020,000 |
| Last qualifying index | 25,176,241 |
| Part 1 complete after | 8 hits |
| Part 2 complete after | 25 hits (17 wasted: duplicate/invalid positions) |

### Probability theory vs practice

The expected hit rate for 5 leading hex zeroes is `1/16^5 = 1/1,048,576`. Our measured rate of `1/1,020,000` is within 2.7% of the theoretical value --- a satisfying confirmation that MD5 behaves as a uniform random function over this range.

**Why Part 2 needs ~3x more hits than Part 1**: Part 1 takes any qualifying hash sequentially (8 hits = 8 characters). Part 2 needs specific positions 0-7 filled. Of 25 qualifying hits, only 8 had valid *unfilled* positions --- the other 17 were either position >= 8 (invalid) or position already filled (duplicate). This is essentially a coupon collector problem: collecting all 8 distinct positions from 16 possible values (0-F, but only 0-7 are valid).

**For comparison --- Bitcoin mining** (as of early 2025): Bitcoin requires ~19-20 leading hex zeroes (the target adjusts dynamically). That's `1/16^20 = 1/1.2 x 10^24` --- roughly **10^18 times harder** than this AoC puzzle. The global Bitcoin network computes ~700 exahashes/second (700 x 10^18 SHA-256 hashes/sec) to find ~one block every 10 minutes. Our 25.5M MD5 hashes in 262ms would barely register as a rounding error.

---

## Key Patterns

### Parallel batch mining with Rayon

```rust
(start..start + BATCH_SIZE)
    .into_par_iter()
    .filter_map(|i| { /* hash + check */ })
    .collect()
```

Same pattern as AoC 2023 Day 16: independent work items -> `par_iter` -> collect results. The key requirement is **no shared mutable state** between iterations.

### Sort-after-parallel for ordering guarantees

Rayon's `par_iter` doesn't guarantee result ordering. When order matters (Part 1's sequential password, Part 2's first-wins), sort by original index after collecting.

### Single-pass dual extraction

Processing each hash for both passwords simultaneously avoids mining the same index range twice. Part 2 determines the stopping point; Part 1 is a free bonus.

### Byte-level hash inspection

```rust
if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
```

Checking raw bytes avoids hex string conversion. Each MD5 byte = 2 hex chars, so 5 leading zeroes = first 2 bytes zero + third byte < 16.

### Nibble extraction

```rust
let low_nibble  = hash[2] & 0x0F;       // 6th hex char
let high_nibble = (hash[3] >> 4) & 0x0F; // 7th hex char
```

Bitwise operations extract individual hex characters from hash bytes without string conversion.

### Option array for positional filling

`[Option<char>; 8]` tracks which positions are filled with natural `.is_none()` for first-wins semantics.

---

**Navigation**: [<- Day 4](day04_function_guide.md) | [All Days](../summary_2016.md) | [Day 6 ->](day06.md)
