# Day 18: Like a Rogue — Function Guide

## Problem Summary

Generate rows of tiles where each tile is either safe (`.`) or trap (`^`).
Each new row is derived from the previous row using left/center/right neighbor rules.

**Part 1**: Count safe tiles across 40 rows
**Part 2**: Count safe tiles across 400,000 rows

---

## Answers

| Part | Answer | Time |
|------|--------|------|
| 1 | `1978` | 3.0µs |
| 2 | `20003246` | 29.2ms |
| Combined | — | 29.9ms |

---

## Function Map

```
solve(input) -> (String, String)
  ├── parse_input(input) -> Vec<bool>            # '^' => true, '.' => false
  ├── solve_part1_with_data(&row)                # count_safe_tiles(row, 40)
  └── solve_part2_with_data(&row)                # count_safe_tiles(row, 400_000)

count_safe_tiles(first_row, rows)
  ├── count_safe_in_row(current)
  ├── for each next row:
  │     next[i] = left ^ right                    # trap rule simplification
  │     count_safe_in_row(next)
  │     swap(current, next)
  └── return total_safe_count
```

---

## Algorithm Details

### Rule Simplification: Four Cases to XOR

Original puzzle says next tile is trap in exactly four situations:
1. `^^.`
2. `.^^`
3. `^..`
4. `..^`

These are exactly the cases where **left and right differ**.
So we can ignore center and compute:

```
next_is_trap = left ^ right
```

That gives branch-light row generation and clean code.

### Row Generation

- Represent row as `Vec<bool>` (`true` = trap, `false` = safe)
- Out-of-bounds neighbors are treated as safe (`false`)
- Build each next row in O(width)
- Reuse two buffers (`current`, `next`) and `swap` each iteration

### Complexity

If width is `w` and total rows is `r`:

- **Time**: O(w × r)
- **Space**: O(w)

For Part 2 (`r = 400,000`), this is still efficient because width is small and memory stays constant per row.

---

## Key Observations

1. **No full grid storage needed**: only previous row is needed to compute the next row.
2. **Part 2 dominates runtime**: 400,000 rows vs 40 rows.
3. **XOR formulation is the core insight**: removes case-by-case branching.
4. **Parse-once structure is preserved**: `solve()` parses input once and reuses parsed data for both parts.

---

## Benchmarks

```
day18_combined          time:   [29.714 ms 29.894 ms 30.073 ms]
day18_part1             time:   [2.9914 µs 3.0091 µs 3.0271 µs]
day18_part2             time:   [29.065 ms 29.234 ms 29.409 ms]
```

Part 1 is effectively free; Part 2 is the full workload.

---

## Potential Optimizations (Not Needed)

- **Bit-packing** (`u64` chunks + bitwise ops) could reduce memory traffic further.
- **SIMD-friendly row updates** could accelerate large-width variants.

Current performance (~30ms combined) is already within the usual AoC target budget for a day.

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
