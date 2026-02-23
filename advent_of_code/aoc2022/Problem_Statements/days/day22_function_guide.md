# Day 22: Monkey Map - Function Guide

**Problem**: Navigate a 2D board with wrapping — flat for Part 1, cube-folded for Part 2.

**Navigation**: [← Day 21](day21_function_guide.md) | [Problem](day22.md) | [Code](../../src/solver/day22.rs) | [Summary](../summary_2022.md) | [Day 23 →](day23_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Follow a path of moves and turns on a sparse board. When stepping off-map, flat-wrap to the opposite edge of the same row or column.
- **Part 2**: The board is the unfolded net of a cube (6 faces of 50x50). When stepping off a face edge, wrap onto the adjacent cube face with correct position and facing.

### Performance
- **Parse**: 813µs (HashMap construction from ~12,000 tiles + row/col range tracking)
- **Part 1**: 318µs (flat wrapping walk)
- **Part 2**: 327µs (cube wrapping walk)
- **Combined**: ~1.46ms

### Key Insight
**The board is sparse and non-rectangular** — rows have different starting columns and widths. A `HashMap<(row, col), Tile>` naturally handles this, with per-row and per-column range maps enabling O(1) flat wrapping. For cube wrapping, the 14 non-adjacent edge transitions are hardcoded as a match table on the out-of-bounds coordinates.

---

## Algorithm Analysis

### Parsing: Sparse Board + Instruction Sequence

The input has two sections separated by a blank line:
1. **Map**: Lines of varying length with `.` (open), `#` (wall), and ` ` (off-map)
2. **Path**: Alternating numbers and `L`/`R` turns (e.g., `10R5L5`)

Only `.` and `#` tiles are inserted into the HashMap. For each tile, row and column range maps are updated to track the min/max extent:

```
Row 0: cols 50-149 (faces 1+2)
Row 50: cols 50-99  (face 3)
Row 100: cols 0-99  (faces 4+5)
Row 150: cols 0-49  (face 6)
```

### Part 1: Flat Wrapping

Walk following instructions. On each step:
1. Compute `(row + dr, col + dc)` for the current facing
2. If the new position is not in the tiles map → **flat wrap**:
   - Moving right → wrap to `row_ranges[row].min_col`
   - Moving left → wrap to `row_ranges[row].max_col`
   - Moving down → wrap to `col_ranges[col].min_row`
   - Moving up → wrap to `col_ranges[col].max_row`
3. If the (possibly wrapped) tile is a wall → stop moving for this instruction
4. If open → update position

### Part 2: Cube Wrapping

Same walk, but when stepping off-map, the out-of-bounds coordinates are passed to `cube_wrap()` which maps them to the correct destination face.

**Cube net layout** (face size 50):
```
    ┌───┬───┐
    │ 1 │ 2 │     rows   0- 49, cols  50-149
    ├───┼───┘
    │ 3 │           rows  50- 99, cols  50- 99
┌───┼───┤
│ 4 │ 5 │           rows 100-149, cols   0- 99
├───┼───┘
│ 6 │               rows 150-199, cols   0- 49
└───┘
```

**Adjacent faces** (natural steps, no cube_wrap needed):
- 1↔2 (share col 99/100 boundary)
- 1↔3 (share row 49/50 boundary)
- 3↔5 (share row 99/100 boundary)
- 4↔5 (share col 49/50 boundary)
- 4↔6 (share row 149/150 boundary)

**14 non-adjacent edge transitions** (require coordinate remapping):

| Source Edge | Destination Edge | Coordinate Transform |
|------------|-----------------|---------------------|
| Face 1 left (col→49) | Face 4 left (reversed) | row→149-row, col→0, face RIGHT |
| Face 1 top (row→-1) | Face 6 left | row→col+100, col→0, face RIGHT |
| Face 2 top (row→-1) | Face 6 bottom | row→199, col→col-100, face UP |
| Face 2 right (col→150) | Face 5 right (reversed) | row→149-row, col→99, face LEFT |
| Face 2 bottom (row→50) | Face 3 right | row→col-50, col→99, face LEFT |
| Face 3 left (col→49) | Face 4 top | row→100, col→row-50, face DOWN |
| Face 3 right (col→100) | Face 2 bottom | row→49, col→row+50, face UP |
| Face 4 left (col→-1) | Face 1 left (reversed) | row→149-row, col→50, face RIGHT |
| Face 4 top (row→99) | Face 3 left | row→col+50, col→50, face RIGHT |
| Face 5 right (col→100) | Face 2 right (reversed) | row→149-row, col→149, face LEFT |
| Face 5 bottom (row→150) | Face 6 right | row→col+100, col→49, face LEFT |
| Face 6 left (col→-1) | Face 1 top | row→0, col→row-100, face DOWN |
| Face 6 right (col→50) | Face 5 bottom | row→149, col→row-100, face UP |
| Face 6 bottom (row→200) | Face 2 top | row→0, col→col+100, face DOWN |

Each transition is a pair — if A→B exists, then B→A also exists (the reverse mapping). The match arm dispatches on the out-of-bounds `(row, col, facing)` tuple.

---

## Implementation Details

### Data Flow
```
Input → parse_input() → Board { tiles, row_ranges, col_ranges, instructions }
                           ↓
                     solve_part1() → flat wrap walk → password
                           ↓
                     solve_part2() → cube wrap walk → password
```

### Function Signatures

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `parse_input` | `&str` | `Board` | Parse map into HashMap + ranges, parse path |
| `find_start` | `&Board` | `(i32, i32)` | Leftmost open tile on row 0 |
| `solve_part1` | `&Board` | `i64` | Walk with flat wrapping |
| `solve_part2` | `&Board` | `i64` | Walk with cube wrapping |
| `cube_wrap` | `(i32, i32, Facing)` | `(i32, i32, Facing)` | Map out-of-bounds step to cube face |

### Key Design Decisions

**Why HashMap instead of Mission 6 Grid?**
The board is sparse and non-rectangular — rows start at different columns (col 0 vs col 50) with different widths. Mission 6's `Grid<T>` requires rectangular dimensions and panics on unequal rows. A HashMap naturally handles the sparse layout with O(1) tile lookups.

**Why hardcoded cube wrapping?**
The 14 edge transitions depend on the specific cube net layout. A general cube-folding algorithm would be significantly more complex for minimal benefit — each AoC input uses one of ~11 possible net shapes, and the actual input always uses the same shape.

**Password formula**: `1000 * (row+1) + 4 * (col+1) + facing` — 1-indexed row/col, facing 0=right, 1=down, 2=left, 3=up.

---

## Performance Analysis

### Benchmark Results
```
Parse:    813µs  (HashMap from ~12,000 tiles + row/col ranges)
Part 1:   318µs  (flat wrapping walk)
Part 2:   327µs  (cube wrapping walk)
Combined: 1.46ms
```

Parse dominates (56%) due to HashMap insertions for ~12,000 tiles plus building three HashMaps (tiles, row_ranges, col_ranges). Part 1 and Part 2 are nearly identical — the cube wrapping match adds negligible overhead over flat wrapping.

### Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| Parse | O(n) | One pass through map + path |
| Part 1 walk | O(S) | S = total steps across all Move instructions |
| Part 2 walk | O(S) | Same, with O(1) cube_wrap per edge crossing |
| cube_wrap | O(1) | Single match on (row, col, facing) tuple |
| HashMap lookup | O(1) avg | ~12,000 entries, well within fast-lookup range |

---

## Edge Cases

- **Wall after wrapping**: Both flat and cube wrapping check the destination tile — if it's a wall, the move is blocked and position stays unchanged
- **Facing changes on cube wrap**: Only Part 2 changes facing direction during wrapping; Part 1 facing is unchanged
- **Non-rectangular rows**: Row ranges vary (50-149 for top, 0-49 for bottom) — HashMap handles this naturally
- **Negative coordinates**: Steps can produce row=-1 or col=-1 (going off the top/left edge) — `i32` coordinates handle this

---

## Key Takeaways

1. **Sparse HashMap beats rectangular grid** for non-rectangular boards — no wasted padding, natural bounds checking via `contains_key`
2. **Row/column range maps enable O(1) flat wrapping** — precomputed during parsing, no scanning needed
3. **Hardcoded cube face transitions are practical** — 14 match arms with clear comments beat a general folding algorithm for AoC
4. **Out-of-bounds coordinates as dispatch key** — stepping one past the edge produces unique (row, col, facing) tuples that cleanly identify which face transition to apply
5. **Part 1 and Part 2 share 95% of code** — only the wrapping logic differs, everything else (parsing, walk loop, password formula) is identical

---

**Answer**: Part 1: `88226` | Part 2: `57305`

**Related patterns**: [[sparse-grid]] | [[cube-net-wrapping]] | [[coordinate-remapping]]
