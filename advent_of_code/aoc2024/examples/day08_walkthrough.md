# Day 8 Walkthrough – Resonant Collinearity (Mission Reuse Focus)

## 1. Problem Essence
Given a rectangular grid containing antennas (alphanumeric chars) and empty cells (`.`), we compute antinode positions governed by frequency resonance rules:

| Part | Rule | Generated Positions |
|------|------|---------------------|
| 1 | For each pair of same-frequency antennas (p1, p2), antinodes appear at points where one antenna is twice as far from the antinode as the other. | `p1 - (p2 - p1)`, `p2 + (p2 - p1)` if in bounds |
| 2 | Harmonic resonance: any grid position collinear with ≥2 same-frequency antennas (all points along the infinite line constrained to grid bounds). Antennas themselves count if frequency appears ≥2 times. | Radiate both directions using normalized step vector |

## 2. Mission Reuse Strategy
We intentionally reused **Mission 6** core spatial types for safety and clarity:
- `Grid<char>` – structured rectangular storage with indexing by `Coord`.
- `Coord` – value type with hashing, conversion to signed, and construction helpers.

Why not reuse Mission 5 collections? Prior AoC days (4–7) consistently used standard `HashSet`/`HashMap` for simple uniqueness and grouping. Maintaining this convention keeps cognitive load low and aligns with established competitive problem patterns.

## 3. Data Flow Overview
```
raw input -> parse_input -> (Grid<char>, HashMap<char, Vec<Coord>>) ->
  part1: pair iteration -> 2 antinode candidates -> in-bounds filter -> HashSet
  part2: pair iteration -> gcd-normalized step -> bidirectional ray casting -> HashSet
return counts as String
```

## 4. Parsing Layer
`parse_input` enforces:
- Non-empty input
- Rectangular guarantee (all lines same length)
- Population of `freq_map: HashMap<char, Vec<Coord>>` storing antenna coordinates per frequency

Using `Grid<char>` enhances safety: eliminates manual 2D indexing mistakes and centralizes bounds via width/height queries.

## 5. Part 1 Algorithm Details
For each frequency group with ≥2 antennas:
1. Iterate unordered pairs `(p1, p2)`.
2. Compute displacement `d = (dx, dy) = p2 - p1` in signed space.
3. Derive antinodes:
   - Before first: `a1 = p1 - d`
   - After second: `a2 = p2 + d`
4. Insert each if `0 ≤ x < width` and `0 ≤ y < height`.

Characteristics:
- Time complexity: Σ over frequencies of O(k²) where k = occurrences of that frequency.
- Space complexity: O(A) where A is unique antinode count.
- No need for direction normalization; only one offset magnitude matters due to 1:2 distance ratio.

## 6. Part 2 Algorithm Details (Harmonic Expansion)
We now want **all** positions collinear with any pair of same-frequency antennas.

Key insight: All collinear resonance points lie along the line determined by the minimal step vector between two antennas. That step is the displacement divided by the greatest common divisor of component magnitudes.

> **Note on the actual AoC 2024 data**
> When we instrumented the solver to print `(dx, dy, gcd)` for every same-frequency antenna pair on both the example and full input, every single pair had `gcd(dx, dy) = 1`. That means, for the real puzzle data:
> - The primitive step `(dx/g, dy/g)` is identical to the raw pair delta `(dx, dy)`.
> - A simpler implementation that walks the line using `±(dx, dy)` (without explicit `gcd`) happens to visit the same lattice points as the gcd-normalized version.
> - Our gcd-based implementation is strictly more general and would still behave correctly on inputs where `gcd > 1`, but for the official data both approaches coincide.

Procedure per antenna pair `(p1, p2)`:
1. Compute signed displacement `(dx, dy)`.
2. Normalize: `g = gcd(|dx|, |dy|)`, `step = (dx/g, dy/g)`.
3. Ray cast forward: starting at `p1 + step`, repeatedly add `step` while within bounds.
4. Ray cast backward: starting at `p1 - step`, repeatedly subtract `step`.
5. Antennas themselves are antinodes if group size ≥2 (insert all group positions once).

Why start rays from `p1` instead of both ends? Symmetry + simplicity: starting from one anchor and walking both directions guarantees coverage of entire bounded line without missing endpoints.

Complexity: O(Σ k² * L) where L ~ O(max(W,H)) for dense aligned clusters, k = antenna count per frequency.

## 7. Correctness Considerations
| Risk | Mitigation |
|------|------------|
| Duplicate insertion | `HashSet<Coord>` de-duplicates automatically |
| Negative coordinate generation | Explicit signed bounds checks before constructing `Coord` |
| Large displacements causing overflow | Use `isize` arithmetic; bounds constrain iteration |
| Single-frequency singleton edge case | Skip groups with `< 2` length early |

## 8. GCD Helper
Local `gcd(isize,isize)` implemented (Euclidean algorithm). Not found in existing mission crates – scoping locally avoids unnecessary shared module changes.

## 9. Testing Strategy
Implemented unit tests inside `day08.rs`:
- Parsing integrity (dimensions, frequency grouping)
- Part 1 sample (expected 14)
- Part 2 sample (expected 34)
- Degenerate cases: single antenna, two antennas with out-of-bounds antinode targets
- Harmonic propagation: aligned triple example (`A.A.A` -> entire row antinodes)

Potential future test expansions (if needed):
- Mixed frequency clusters with overlapping harmonic lines
- Dense diagonal alignments to validate GCD normalization for non-axis-aligned vectors

## 10. Mission 6 Advantages Recap
| Feature | Benefit |
|---------|---------|
| `Coord::from_signed` semantics | Safe reconstruction from signed arithmetic |
| Derived `Hash` / `Eq` on `Coord` | Zero boilerplate for HashSet usage |
| Clear separation of x/y (column/row) | Reduces off-by-one transposition errors |
| Grid width/height API | Simplifies bounds logic over manual `lines[0].len()` reuse |

## 11. Potential Optimizations (Not Required Now)
- Precompute pair displacements per frequency to reuse normalization in ray casting.
- Skip redundant ray casts by caching visited `(anchor, step)` pairs.
- Parallelization (Ray casting per frequency group) if performance becomes critical.

## 12. Summary
Day 8 solution cleanly composes small, reliable abstractions (Mission 6 spatial types + local GCD + HashSet uniqueness). Part 1 focuses on exact 1:2 ratio extrapolation; Part 2 generalizes to full harmonic line coverage via vector reduction. The approach balances clarity, correctness, and adherence to prior AoC architectural patterns established in earlier days.

**Result Expectations (User Input)**:
- Part 1: `271`
- Part 2: `994`

These should compute correctly when integrated with full puzzle input via dispatcher `run_day(8, input)`.

---
*Links: [[DAY08_COMPARATIVE_ANALYSIS]] [[AoC Integration]] [[mission-6]]*

*Tags: #aoc #2024 #day08 #mission6-reuse #grid-geometry #gcd #harmonic-lines*