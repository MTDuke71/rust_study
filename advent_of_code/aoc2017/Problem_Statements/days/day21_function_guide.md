# Day 21: Fractal Art — Function Guide

**Problem**: Grow a pixel grid by repeatedly splitting it into 2×2 or 3×3 blocks and replacing each via an enhancement rule that matches under any of 8 orientations (4 rotations × 2 flips). Count the `#` pixels after N iterations.
**Answers**: Part 1 = **167** (5 iterations), Part 2 = **2,425,195** (18 iterations)
**Code**: [day21.rs](../../src/solver/day21.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Model](#data-model)
3. [The Dihedral Group D₄](#the-dihedral-group-d)
4. [Rule Expansion vs Canonicalization](#rule-expansion-vs-canonicalization)
5. [The Step Function](#the-step-function)
6. [Grid Size Progression](#grid-size-progression)
7. [Benchmarks](#benchmarks)
8. [Alternate Implementations](#alternate-implementations)
9. [Key Patterns](#key-patterns)
10. [Integrator Notes](#integrator-notes)

---

## Problem Summary

Starting grid is fixed at `.#./..#/###` (3×3). On each step:

- If side length is divisible by 2, split into 2×2 blocks → each becomes 3×3 (rule LHS is 2×2, RHS is 3×3).
- Otherwise side is divisible by 3, split into 3×3 blocks → each becomes 4×4.

The rulebook only lists *one* orientation per pattern. When looking up a block, try all 8 orientations (4 rotations × 2 flips) until one matches. Never rotate/flip the output.

Part 1 runs 5 iterations. Part 2 runs 18. Same machinery, different count.

---

## Data Model

```rust
type Tile = Vec<Vec<bool>>;
```

The tile representation has to handle three different sizes (2×2, 3×3, 4×4) within the same step. `[[bool; N]; N]` won't work because `N` isn't fixed. Three reasonable choices:

| Rep | Pros | Cons |
|-----|------|------|
| `Vec<Vec<bool>>` | Trivial rotate/flip by index math | Cache-unfriendly, allocates per-row |
| `String` (puzzle's slash form) | Hashable as-is, zero conversion for lookup | Rotate/flip means manipulating characters through a 1D-indexing scheme |
| `u16` bitmask | Tiny, hashable directly, nanosecond operations | Rotate/flip requires bit shuffling; bit positions depend on `N` |

I went with `Vec<Vec<bool>>` because (a) the rotation/flip code reads like the math, and (b) it doesn't matter — the naive rep gets Part 2 in 234 ms, plenty of margin. Bitmasking is the right call when you need another 10×. (See [Where I'd Revisit](#where-id-revisit).)

HashMap keys are the serialized `".#./..#/###"` string form. Tiles get serialized on every lookup, which is not free, but strings of length ≤ 15 hash fast and there's no correctness risk from a canonical-form bug.

---

## The Dihedral Group D₄

All 8 orientations of a square tile are the elements of the **dihedral group D₄** — the symmetry group of a square. You can generate the full 8-element group from two operations: rotation by 90° (`r`) and any single reflection (`f`). The 8 elements are:

```
{ e, r, r², r³, f, rf, r²f, r³f }
```

Which matches the "rotate up to 3 times, then flip, then rotate up to 3 more times" intuition. The implementation:

```rust
fn orientations(t: &Tile) -> Vec<Tile> {
    let mut out = Vec::with_capacity(8);
    let mut cur = t.clone();
    for _ in 0..4 {
        out.push(flip(&cur));   // rⁿ·f
        out.push(cur.clone());  // rⁿ
        cur = rotate(&cur);
    }
    out
}
```

For **symmetric** tiles (e.g. `.../.../...`, all off) some of the 8 collapse into equivalents. That's fine — HashMap inserts overwrite with the same value, and the extra memory is trivial (8 × ~15 bytes per rule × 108 rules = under 13 KB in the worst case).

### Rotation math

90° clockwise rotation: `new[r][c] = old[N-1-c][r]`. Sanity-check by rotating a 3×3:

```
. # .         # . .
. . #   →     # . #
# # #         # # .
```

Row 0 of the new tile is column 0 of the old, read bottom-to-top: `(#,.,.)` → `#..`. The code does exactly that:

```rust
fn rotate(t: &Tile) -> Tile {
    let n = t.len();
    (0..n)
        .map(|r| (0..n).map(|c| t[n - 1 - c][r]).collect())
        .collect()
}
```

Horizontal flip is just reverse-each-row: `new[r][c] = old[r][N-1-c]`. Either flip (horizontal or vertical) works to generate the group — the other four elements fall out of combining it with rotation.

---

## Rule Expansion vs Canonicalization

Two clean ways to handle the "rule matches any orientation" requirement:

**Option A — Canonicalize at lookup.** For every tile, compute its canonical form (e.g. the lex-smallest string over all 8 orientations). Rules store exactly one entry per pattern. Each lookup costs 8 orientation computations + 8 string comparisons.

**Option B — Expand at parse.** For each rule's LHS, insert all 8 orientations into the HashMap, all pointing at the same RHS. Parse pays an 8× cost once; every lookup is `HashMap::get` on the tile's serialized form.

I chose B. Justification:
- Ruleset is 108 rules → at most 864 map entries. Negligible.
- Part 2 runs 18 iterations with a final grid of ≈2.4M blocks. **Lookup is the hot path, parse is once.** Trading parse-time work for lookup-time work is obviously correct.
- No canonical-form bugs possible — each orientation's string *is* the key.

The trade shifts if rules are dynamic or the input rulebook is enormous. Neither applies.

---

## The Step Function

```rust
fn step(grid: &Tile, rules: &HashMap<String, Tile>) -> Tile {
    let n = grid.len();
    let b = if n.is_multiple_of(2) { 2 } else { 3 };
    let out_b = b + 1;
    let blocks_per_side = n / b;
    let new_n = blocks_per_side * out_b;

    let mut out: Tile = vec![vec![false; new_n]; new_n];

    for br in 0..blocks_per_side {
        for bc in 0..blocks_per_side {
            let block: Tile = (0..b)
                .map(|r| (0..b).map(|c| grid[br * b + r][bc * b + c]).collect())
                .collect();

            let enhanced = rules
                .get(&tile_to_string(&block))
                .unwrap_or_else(|| panic!("no rule for block {}", tile_to_string(&block)));

            for r in 0..out_b {
                for c in 0..out_b {
                    out[br * out_b + r][bc * out_b + c] = enhanced[r][c];
                }
            }
        }
    }
    out
}
```

Three index-math facts worth internalising:

1. **Block count is `n / b`** per side. Since `b ∈ {2, 3}` and `n` is always divisible by the chosen `b`, there's no remainder to worry about.
2. **Output side is `(n / b) * (b + 1)`**. Each block contributes a `(b+1)`-sided tile at the scaled position. For `b=2` → grows by 3/2. For `b=3` → grows by 4/3.
3. **Write-back offset is `br * out_b`, not `br * b`**. The source block lives at `br * b` in the input coordinate system and at `br * (b+1)` in the output one — two different strides.

Easy to mistype and get a crumpled grid with the right *pixel count* but wrong arrangement. Guard test: the "12 pixels after 2 iterations" example from the puzzle statement catches this immediately.

---

## Grid Size Progression

Size follows a deterministic sequence, decided entirely by the `is_multiple_of(2)` check at each step:

| Iter | Size | Divisor | Cells | On (real input) | Density |
|------|------|---------|-------|-----------------|---------|
| 0    | 3    | —       | 9     | 5               | 55.6%   |
| 1    | 4    | /3      | 16    | 10              | 62.5%   |
| 2    | 6    | /2      | 36    | 20              | 55.6%   |
| 3    | 9    | /2      | 81    | 52              | 64.2%   |
| 4    | 12   | /3      | 144   | 85              | 59.0%   |
| 5    | 18   | /2      | 324   | **167**         | 51.5%   |
| 6    | 27   | /2      | 729   | ?               | —       |
| 9    | 81   | —       | 6 561 | ?               | —       |
| 12   | 243  | —       | 59 049 | ?              | —       |
| 15   | 729  | —       | 531 441 | ?             | —       |
| 18   | **2 187** | — | **4 782 969** | **2 425 195** | **50.7%** |

Two nice observations:

- **Size triples every 3 iterations.** The cycle "/3, /2, /2" multiplies by `4/3 × 3/2 × 3/2 = 3`. So `size(iter) = 3 * 3^(iter/3)` for `iter` divisible by 3. 18 = 6 × 3, so `3 * 3^6 = 2187`. That's `3^7` exactly.
- **Density hovers around 50–60%.** Not a fluke — the rulebook produces RHS patterns whose density is approximately `3^(b+1) / 2^(b+1)` times the LHS density weighted by rule frequency. Converges quickly to a fixed point determined by the ruleset's eigenvector. (Think of each iteration as multiplying a density vector by a transition matrix; after enough iterations the dominant eigenvalue's eigenvector wins.)

---

## Benchmarks

Three implementations coexist in the crate. Reference ([day21.rs](../../src/solver/day21.rs)) is the teaching version; the other two are explored in [Alternate Implementations](#alternate-implementations).

| Implementation | Part 1 | Part 2 | Combined | Part 2 vs. reference |
|----------------|--------|--------|----------|----------------------|
| Reference (`Vec<Vec<bool>>` + `HashMap<String, _>`) | 555.89 µs | 229.52 ms | 241.89 ms | 1× (baseline) |
| Bit-packed (flat grid + `u16` LUTs) | 21.03 µs | 5.68 ms | 6.87 ms | **40×** |
| Memo (3-iter algorithmic memoization) | 24.39 µs | **32.46 µs** | 58.23 µs | **7,070×** |

Observations on the reference:

- **Part 1 is cheap.** Final grid is 18×18 = 324 cells, 6 steps of work. Sub-millisecond.
- **Part 2 is ~420× slower than Part 1**, dominated by the last two steps (final grid is 2187×2187 = 4.78M cells, penultimate is 1458² = 2.13M). Each step does `blocks_per_side²` block extracts + string hashes + RHS copies. The last step alone does ~531k block ops.
- **`solve()` ≈ Part 2.** Part 1's work is dwarfed; parse-once still helps but is noise-level.

Where the 234 ms actually goes, roughly:
- ~35%  `tile_to_string` allocations on every lookup
- ~30%  `Vec::push` / `Vec::with_capacity` in block extraction
- ~20%  `HashMap::get` hashing + probing
- ~15%  output write-back

The first two rows are exactly what the bit-packed implementation eliminates; the memo implementation skips the grid materialization entirely.

---

## Key Patterns

### Expand-at-parse for symmetry-aware lookup
Whenever the rule says "match modulo some small symmetry group," prefer expanding every rule into all variants at parse time over canonicalizing at lookup. Lookups happen millions of times; parses happen once. The exception is when the group is large (e.g. the ~24-element rotation group of a cube) — then memory pressure can flip the decision.

### Index arithmetic separates input and output strides
The step function manipulates the same block at *two* different strides: `b` in the input, `b+1` in the output. Keeping the two offsets named and obvious (`br * b + r` vs `br * out_b + r`) is worth far more than a single-letter variable. Misreading those strides is the most common bug on this kind of problem.

### Puzzle-provided worked examples are gold
"12 pixels after 2 iterations" from the problem statement is a stronger test than any hand-crafted unit test. It exercises both the /3 and /2 branches, uses the rule's symmetry property (the `../.#` match requires one or more flips), and validates the stitching. When a puzzle gives a worked example, lock it down as a test before touching the real input.

### Dihedral-group fluency buys correctness for free
The 4-rotate + 2-flip code is 10 lines. Knowing that these 8 elements *are* the whole D₄ group means no "did I miss an orientation?" anxiety. A similar pattern shows up for 3D rotations (24-element rotation group of a cube, or 48 with reflections) on problems like Day 19 of AoC 2020 — same mental model, larger group.

---

## Alternate Implementations

Two alternate solvers live in the same crate, one mechanical and one algorithmic. Both carry full parity tests against the reference (167 / 2,425,195).

### Bit-packed — `day21_bitpacked.rs`

**Same algorithm, tighter representation.** Replaces:

| | Reference | Bit-packed |
|---|---|---|
| Tile key | `String` (`"##./.../..#"`) | `u16` row-major bitmask |
| Rule map | `HashMap<String, Vec<Vec<bool>>>` | `[u16; 16]` + `[u16; 512]` direct-indexed LUTs |
| Grid storage | `Vec<Vec<bool>>` | flat `Vec<bool>`, `cells[r * n + c]` |
| Lookup | `HashMap::get(&tile_to_string(&block))` | `rules_3[tile as usize]` |

2×2 tiles pack into 4 bits; 3×3 into 9 bits; 4×4 into 16 bits — all fit in `u16`. Rotation and flip become bit permutations. Because the LHS space is fully enumerable (16 for 2×2, 512 for 3×3), the rule tables become direct-indexed arrays, not hash tables.

**Measured: 40× speedup on Part 2** (229.52 ms → 5.68 ms). Where the savings came from:

- No `String` allocation per lookup (~35% of reference runtime)
- No per-row `Vec` allocation in block extraction (~30%)
- Array index instead of HashMap hash + probe (~20%)

### Memo — `day21_memo.rs`

**Different algorithm.** Exploits a structural property:

> Starting from any 3×3 block, three iterations produce a 9×9 grid whose nine 3×3 sub-blocks evolve *independently* under every subsequent iteration.

This holds because iter 4 does a /3 split, whose block boundaries align exactly with the sub-blocks from iter 3. The 3-iter structure repeats forever.

So we memoize:

```
f(block, 0) = popcount(block)
f(block, d) = Σ f(sub, d-1)  for sub in expand3(block)
```

where `expand3` maps a 3×3 block to the 9 sub-blocks it becomes after 3 iterations.

Part 2 is `f(initial, 6)` (since 18 = 6×3). Part 1 has 5 iterations — not a multiple of 3 — so it delegates to the bit-packed solver.

**Measured: 7,070× speedup on Part 2** (229.52 ms → 32.46 µs). The entire 18-iteration computation fits in a few thousand HashMap lookups + sums. The 2187×2187 grid is **never materialized** — we only count pixels.

I predicted 100–500× and was off by an order of magnitude. The reason: I assumed the cache would approach its theoretical max of 512 × 7 = 3,584 entries. In practice only a small fraction of the 512 possible 3×3 blocks ever appear in the expansion tree, so the cache stays small and lookups stay fast.

### When to pick which

| Goal | Choice |
|------|--------|
| Teaching / readability | Reference — closest to how the puzzle reads |
| Same algorithm, fast | Bit-packed — drop-in, no new ideas |
| Maximum speed | Memo — but only works for iteration counts that are multiples of 3 |

`solve()` in [mod.rs](../../src/solver/mod.rs) still dispatches to the reference implementation. The alternates are available as `aoc2017::solver::day21_bitpacked` and `aoc2017::solver::day21_memo`.

### Further optimizations not yet tried

- **Rayon on the block loop** inside `step()` (bit-packed version): blocks are independent, same pattern as Day 16's beam traces. At 5.68 ms for Part 2 the setup overhead may dominate — unclear win.
- **Flat-grid memory for the grid in bit-packed**: already done (contiguous `Vec<bool>`). Next step would be `Vec<u64>` bit-packing the grid itself, not just the lookup keys — avoids another factor of 8 in memory bandwidth on the large iterations. Diminishing returns once the memo version exists.

---

## Integrator Notes

- **No Mission 6 reuse here.** Mission 6's `Grid<T>` / `Coord` is `usize`-indexed and would work for the outer grid, but the per-tile manipulation (rotate/flip of 2×2 and 3×3 lookup keys) is happier with the raw `Vec<Vec<bool>>`. Wrapping then unwrapping to call `rotate`/`flip` would be friction without payoff. Example of "compose when it helps, not at any cost."
- **AUTOSAR analogue.** Each `step()` is a periodic task. The block loop inside it is a perfect "parallel runnable" pattern — each `(br, bc)` iteration reads disjoint input regions and writes disjoint output regions. Rayon is the natural RTE for this.
- **Dihedral group D₄** — see [[dihedral-group-d4]] for the group-theoretic reference (orbit-stabilizer, canonical forms, generalizations to Dₙ and the 24-element cube rotation group). D₄ recurs across years of puzzles (AoC 2020 Day 20 "Jurassic Jigsaw", cellular automata symmetry reductions) — one zettel amortizes the concept.

---

**Navigation**: [← Day 20](day20_function_guide.md) | [All Days](../summary_2017.md) | Day 22 →
