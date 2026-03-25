# Day 16: Dragon Checksum — Function Guide

## Problem Summary

Fill a disk with pseudo-random data using a modified dragon curve expansion, then compute a checksum by repeatedly pairing bits until the result has odd length.

**Two operations:**
1. **Dragon curve expansion**: `a + "0" + reverse(flip(a))` — doubles+1 each step
2. **Checksum reduction**: pair bits (same→1, diff→0), halve until odd length

**Part 1**: Disk size = 272
**Part 2**: Disk size = 35,651,584

---

## Answers

| Part | Answer | Time (v1) | Time (v2) | Speedup |
|------|--------|-----------|-----------|---------|
| 1 | `10010010110011010` | 720ns | 415ns | 1.7× |
| 2 | `01010100101011100` | 51.0ms | 22.1ms | 2.3× |
| Combined | — | 53.9ms | 24.3ms | 2.2× |

---

## Function Map

```
solve(input) → (String, String)
  ├── parse_input(input) → Vec<bool>         # Convert "01..." to bit vector
  ├── solve_part1_with_data(&data)            # disk_size = 272
  │   └── solve_with_disk_size(&data, 272)
  │       ├── dragon_fill(&data, 272)         # Expand until ≥ 272, truncate
  │       └── checksum(&filled)               # Reduce pairs until odd length
  └── solve_part2_with_data(&data)            # disk_size = 35,651,584
      └── solve_with_disk_size(&data, 35_651_584)
          ├── dragon_fill(&data, 35_651_584)  # Expand until ≥ 35M, truncate
          └── checksum(&filled)               # Reduce pairs until odd length
```

---

## Algorithm Details

### Dragon Curve Expansion

Starting with 17-bit input, each step produces `2n + 1` bits:

| Step | Length | Formula |
|------|--------|---------|
| 0 | 17 | Initial input |
| 1 | 35 | 2(17) + 1 |
| 2 | 71 | 2(35) + 1 |
| ... | ... | ... |
| 20 | 17,825,791 | Still < 35,651,584 |
| 21 | 35,651,583 | Still < 35,651,584! |
| 22 | 71,303,167 | ≥ 35,651,584 ✓ (truncate) |

The expansion is `O(disk_size)` — each step roughly doubles, so only ~log₂(disk_size/input_size) ≈ 21 steps needed. But total bytes written across all steps sums to ~2× disk_size.

### Checksum Reduction

#### v1: Iterative Halving (original)

Starting with `disk_size` bits, each round halves the length:

| Round | Length | Even? |
|-------|--------|-------|
| 0 | 35,651,584 | Yes |
| 1 | 17,825,792 | Yes |
| 2 | 8,912,896 | Yes |
| ... | ... | ... |
| 21 | 17 | Odd → stop |

Total work: 35M + 17.8M + 8.9M + ... ≈ 70M comparisons (geometric series ≈ 2× first term).

#### v2: Single-Pass Parity (optimized)

**Key insight**: Repeatedly pairing bits (same→1, diff→0) for k rounds is equivalent to counting 1s in chunks of 2^k:
- 1 round over 2 bits: `same→1, diff→0` = "even count of 1s → 1"
- 2 rounds over 4 bits: even count of 1s in all 4 → 1
- k rounds over 2^k bits: **even parity of 1s → '1', odd → '0'**

Since 35,651,584 = 17 × 2²¹, chunk size = 2²¹ = 2,097,152 bits. The checksum walks 17 chunks in one pass:

```
chunk_size = 1 << data.len().trailing_zeros()  // 2^21 = 2,097,152
data.chunks(chunk_size)
    .map(|chunk| count_ones(chunk) % 2 == 0 → '1' or '0')
```

**Work comparison**:
| Approach | Passes | Total comparisons |
|----------|--------|-------------------|
| v1 (iterative) | 21 | ~70M (geometric sum) |
| v2 (single-pass) | 1 | 35M (one scan) |

The 2× reduction in comparisons plus eliminating 20 intermediate allocations yields the 2.3× speedup.

For Part 1: 272 = 17 × 2⁴, chunk size = 16, same principle.

---

## Data Representation

Bits stored as `Vec<bool>`:
- 1 byte per bit (not optimal, but simple and fast enough)
- 35M bools = ~35MB allocation for Part 2
- Alternative: `BitVec` or `Vec<u8>` with bit packing would use ~4.4MB, but adds complexity for marginal gain at 51ms

---

## Key Observations

1. **Growth is exponential**: 17 bits → 35M bits in ~21 doubling steps. The dragon fill loop runs very few iterations — most time is spent in the final (largest) step.

2. **Checksum is a geometric series**: Each halving pass processes half the data of the previous. Total work ≈ 2× the first pass, so checksum is roughly the same cost as a single scan of the data.

3. **No mission integration needed**: Pure bit manipulation with no grid, graph, or collection patterns.

4. **Vec<bool> is sufficient**: Despite using 8× more memory than bit-packing, the CPU cache benefits of simple sequential access keep performance well under 100ms.

---

## Benchmarks

```
v1 (iterative checksum):
  day16_combined          time:   [53.9 ms]
  day16_part1             time:   [720 ns]
  day16_part2             time:   [51.0 ms]

v2 (single-pass parity checksum):
  day16_combined          time:   [24.3 ms]   (-55%)
  day16_part1             time:   [415 ns]    (-42%)
  day16_part2             time:   [22.1 ms]   (-57%)
```

Part 1 is essentially free (272 bits). Part 2 dominates — now 22ms with the single-pass optimization. The remaining time is mostly dragon fill (memory allocation and sequential writes for 35M bits).

---

## Optimization Applied

### Single-Pass Parity Checksum (v1 → v2)

**Observation** (from community video analysis): The iterative pairing reduction over 2^k rounds collapses to a single parity check. For any chunk of 2^k bits, the final checksum bit equals: count 1s → even = '1', odd = '0'.

**Why it works**: Each pairing round asks "are these two bits the same?" — which is an XNOR. Chaining k rounds of XNOR over 2^k bits reduces to checking whether the total count of 1s is even (parity).

**Computing chunk size with `trailing_zeros()`**:

The iterative checksum halves the length until it's odd — that's just repeated right-shifting in binary.
`trailing_zeros()` tells you upfront how many shifts you'd need, in a single CPU instruction (`TZCNT` on x86):

```
Part 2: disk_size = 35,651,584
Binary: 10 0001 1111 1111 0000 0000 0000 0000 0
                                   ─────────────────────
                                   21 trailing zeros

35,651,584 >> 1  = 17,825,792  (even, keep halving)
17,825,792 >> 1  =  8,912,896  (even)
 ...21 shifts...
            34 >> 1  =      17  (odd → stop!)

trailing_zeros() = 21  →  chunk_size = 1 << 21 = 2,097,152
```

| Disk size | Binary | Trailing zeros | Chunk size | Final length |
|-----------|--------|---------------|------------|--------------|
| 20 | `10100` | 2 | 4 | 5 |
| 272 | `100010000` | 4 | 16 | 17 |
| 35,651,584 | `10000...0` | 21 | 2,097,152 | 17 |

This replaces the v1 loop that discovered the same answer by actually halving 21 times.

**Implementation change**:
```rust
// v1: 21 passes, allocating new Vec each time
while current.len().is_multiple_of(2) {
    current = current.chunks(2)
        .map(|pair| pair[0] == pair[1]).collect();
}

// v2: 1 pass, no intermediate allocations
let chunk_size = 1usize << data.len().trailing_zeros();
data.chunks(chunk_size)
    .map(|chunk| {
        let ones = chunk.iter().filter(|&&b| b).count();
        if ones % 2 == 0 { '1' } else { '0' }
    }).collect()
```

**Result**: 53.9ms → 24.3ms (2.2× speedup)

---

## Potential Further Optimizations (Not Needed)

- **Bit-packing** (`Vec<u64>` with bitwise ops): ~8× less memory, `count_ones()` uses POPCNT instruction
- **Avoid the final extend**: Since we truncate anyway, stop the last dragon step when we hit disk_size

None worthwhile at 24ms — well under the 100ms target.

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
