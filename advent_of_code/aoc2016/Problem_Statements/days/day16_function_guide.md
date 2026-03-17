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

| Part | Answer | Time |
|------|--------|------|
| 1 | `10010010110011010` | 720ns |
| 2 | `01010100101011100` | 51.0ms |
| Combined | — | 53.9ms |

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

Starting with `disk_size` bits, each round halves the length:

| Round | Length | Even? |
|-------|--------|-------|
| 0 | 35,651,584 | Yes |
| 1 | 17,825,792 | Yes |
| 2 | 8,912,896 | Yes |
| ... | ... | ... |
| 24 | 17 | Odd → stop |

The checksum divides by 2 until the result is odd. Since 35,651,584 = 17 × 2²¹, it takes exactly 21 rounds to reach length 17 (odd). Total work across all rounds: 35M + 17.8M + 8.9M + ... ≈ 70M comparisons (geometric series converges to ~2× first term).

For Part 1: 272 = 17 × 2⁴, so 4 rounds to reach length 17.

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
day16_combined          time:   [53.9 ms]
day16_part1             time:   [720 ns]
day16_part2             time:   [51.0 ms]
```

Part 1 is essentially free (272 bits). Part 2 dominates at 51ms — mostly memory allocation and sequential writes for the 35M-bit dragon fill.

---

## Potential Optimizations (Not Needed)

- **Bit-packing** (`Vec<u64>` with bitwise ops): ~8× less memory, better cache utilization
- **In-place checksum**: Reuse the same buffer instead of allocating new `Vec<bool>` each round
- **Avoid the final extend**: Since we truncate anyway, stop the last dragon step when we hit disk_size

None worthwhile at 51ms — well under the 100ms target.
