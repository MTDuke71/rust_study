# Day 20: Firewall Rules — Function Guide

## Problem Summary

A firewall blocks IP addresses based on a blacklist of ranges. IPs are unsigned 32-bit integers (0 to 4,294,967,295).

**Part 1**: Find the **lowest** IP address that is not blocked by any range.
**Part 2**: Count **how many** IP addresses are allowed (not blocked by any range).

---

## Answers

| Part | Answer | Time |
|------|--------|------|
| 1 | `22887907` | 41.3us |
| 2 | `109` | 41.6us |
| Combined | — | 42.5us |

---

## Function Map

```
solve(input) -> (String, String)
  ├── parse_input(input) -> Vec<Range>           # split "lo-hi" lines into (u32, u32) pairs
  ├── merge_ranges(&mut [Range]) -> Vec<Range>   # sort + merge overlapping/adjacent ranges
  ├── solve_part1_with_data(&[Range]) -> u64     # first gap after merged block 0
  └── solve_part2_with_data(&[Range]) -> u64     # total IP space minus blocked count
```

---

## Algorithm Details

### Interval Merging

The core technique is **interval merging** — a classic sweep-line algorithm:

1. **Sort** all 1005 ranges by start value
2. **Merge** overlapping or adjacent ranges into a minimal set
3. **Query** the merged set for gaps

### Parse

Each line `"lo-hi"` is split on `'-'` and parsed into a `(u32, u32)` tuple. No allocations beyond the output `Vec`.

### Merge Ranges

After sorting by start:
- For each range `(lo, hi)`, check if it overlaps or is adjacent to the last merged range
- **Overlap/adjacent**: `lo <= last.1 + 1` — extend the merged range
- **Gap**: push a new merged range

**Overflow handling**: When `last.1 == u32::MAX`, `last.1 + 1` would overflow. The adjacency check promotes to `u64` arithmetic to avoid this:
```rust
if (lo as u64) <= (last.1 as u64) + 1 {
    last.1 = last.1.max(hi);
}
```

### Part 1: Lowest Allowed IP

After merging, the first allowed IP is either:
- `0` if the first merged range doesn't start at 0
- `merged[0].1 + 1` — the IP right after the first block ends

For our input, ranges cover 0 through 22,887,906, so the answer is **22,887,907**.

### Part 2: Count Allowed IPs

Total address space is 2^32 = 4,294,967,296 IPs. Sum the size of each merged block, subtract from total:
```rust
allowed = 4,294,967,296 - sum(hi - lo + 1 for each merged range)
```

The 1005 input ranges merge down to a small number of blocks, leaving exactly **109** unblocked IPs scattered across the full range.

### Complexity

- **Time**: O(n log n) for the sort, O(n) for the merge — dominated by sort
- **Space**: O(n) for the merged output

With n=1005 ranges, this runs in ~42us.

---

## Key Observations

1. **Classic interval problem**: Sort + linear merge is the textbook approach — no fancy data structures needed.
2. **u32 overflow is the trap**: Adjacent ranges ending at `u32::MAX` overflow on `+1`. Promoting to `u64` for the comparison is the cleanest fix.
3. **Parse-once**: Both parts share the same merged interval list — sort and merge happen exactly once.
4. **Part 2 is subtraction**: Rather than scanning all 4B IPs, just subtract blocked ranges from total space.
5. **Only 109 allowed**: Out of 4.3 billion IPs, the blacklist covers almost everything — the gaps are tiny.

---

## Benchmarks

```
day20_combined          42.5us
day20_part1             41.3us
day20_part2             41.6us
```

Combined is barely more than individual parts — parsing + sorting dominates, and both queries are O(merged_count) scans.
