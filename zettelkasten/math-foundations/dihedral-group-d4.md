# Dihedral Group D₄ — Symmetries of the Square

**Field**: Abstract Algebra (Group Theory)

**Prerequisites**: [[set-theory-fundamentals]] (groups are sets with structure), basic 2D geometry

---

## 📐 Definition

**Dihedral group Dₙ**: The group of symmetries of a regular n-sided polygon, containing n rotations and n reflections — 2n elements total.

**D₄**: The symmetry group of a square. $|D_4| = 8$ elements.

**Formal notation** (composition read right-to-left: $gh$ means "first $h$, then $g$"):
- Rotations: $\{e, r, r^2, r^3\}$ where $r$ is rotation by 90° CW
- Reflections: $\{f, fr, fr^2, fr^3\}$ where $f$ is any fixed reflection. Note $\{fr^i\} = \{r^i f\}$ as sets (via $fr = r^{-1} f$), but the index labeling differs
- Relations: $r^4 = e$, $f^2 = e$, $frf = r^{-1}$

**Presentation**: $D_4 = \langle r, f \mid r^4 = f^2 = e,\ frf = r^{-1} \rangle$

**Intuition**: Pick up a square and put it back down. There are exactly 8 ways it can land so that it covers the same region — 4 rotations times 2 (original or flipped).

---

## 🔑 Key Properties

### **Property 1**: Closure and 8 elements

Every composition of rotations and reflections is again a rotation or reflection.

Enumerating all 8 (starting tile has corners labeled A/B/C/D clockwise from top-left, so rows are `A B` over `D C`):

| Element | Description | Result |
|---------|-------------|--------|
| $e$     | identity | `A B / D C` |
| $r$     | rotate 90° CW | `D A / C B` |
| $r^2$   | rotate 180° | `C D / B A` |
| $r^3$   | rotate 270° CW | `B C / A D` |
| $f$     | horizontal flip (reverse each row) | `B A / C D` |
| $fr$    | rotate 90°, then flip | `A D / B C` |
| $fr^2$  | rotate 180°, then flip | `D C / A B` |
| $fr^3$  | rotate 270°, then flip | `C B / D A` |

The last four are the four reflections of the square (across the vertical axis, across the NE–SW diagonal, across the horizontal axis, and across the NW–SE diagonal, respectively).

### **Property 2**: Non-abelian

$D_4$ is **non-commutative**: $rf \neq fr$ in general. Order of operations matters.

Proof by computation: `rf` rotates then flips (a horizontal flip applied to the already-rotated square), producing the top-left corner at position `A`. `fr` flips then rotates, producing a *different* top-left corner. The fact that these disagree is what makes the group interesting — abelian groups would collapse to $\mathbb{Z}/8\mathbb{Z}$ or $\mathbb{Z}/2 \times \mathbb{Z}/4$, neither of which matches the geometry.

### **Property 3**: Two generators suffice

Every element of $D_4$ can be written as a product of $r$ and $f$ alone. This is why the Rust generation loop only needs *one* rotation primitive and *one* flip primitive:

```rust
// Generates all 8 elements of D_4 from just `rotate` and `flip`.
for _ in 0..4 {
    out.push(flip(&cur));   // f · r^i (apply f to r^i(x)) = f r^i
    out.push(cur.clone());  // r^i
    cur = rotate(&cur);     // advance r
}
```

After 4 iterations we've emitted $\{e, f, r, fr, r^2, fr^2, r^3, fr^3\}$ — the complete group.

### **Property 4**: Subgroup structure

$D_4$ has several interesting subgroups:
- $\{e, r, r^2, r^3\}$ — the **cyclic subgroup** $C_4$ (rotations only)
- $\{e, r^2\}$ — center of the group (180° commutes with everything)
- $\{e, f\}, \{e, fr\}, \{e, fr^2\}, \{e, fr^3\}$ — four reflection subgroups of order 2
- $\{e, r^2, f, fr^2\}$ and $\{e, r^2, fr, fr^3\}$ — two Klein four-groups $V_4$

The lattice of subgroups of $D_4$ is a common textbook example because every case (normal/non-normal, abelian/non-abelian inclusion) shows up in one small group.

---

## 💻 Rust Implementations

### **AoC 2017 Day 21**: Fractal Art — Rule matching under all symmetries

**Problem**: A rulebook maps a small pixel pattern (2×2 or 3×3) to a larger one. Rules match under any of D₄'s 8 orientations. Build a HashMap such that every orientation of an LHS maps to its RHS.

**Code** ([aoc2017/day21.rs](../../advent_of_code/aoc2017/src/solver/day21.rs)):

```rust
type Tile = Vec<Vec<bool>>;

// 90° clockwise rotation: new[r][c] = old[N-1-c][r]
fn rotate(t: &Tile) -> Tile {
    let n = t.len();
    (0..n)
        .map(|r| (0..n).map(|c| t[n - 1 - c][r]).collect())
        .collect()
}

// Horizontal flip: reverse each row
fn flip(t: &Tile) -> Tile {
    t.iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

// Generate all 8 elements of D_4 acting on the tile
fn orientations(t: &Tile) -> Vec<Tile> {
    let mut out = Vec::with_capacity(8);
    let mut cur = t.clone();
    for _ in 0..4 {
        out.push(flip(&cur));   // f · r^i  (f applied to r^i(t))
        out.push(cur.clone());  // r^i
        cur = rotate(&cur);
    }
    out  // exactly |D_4| = 8 tiles
}
```

**Mathematical translation**:
- `rotate` implements $r$ (90° clockwise)
- `flip` implements $f$ (horizontal reflection)
- The loop body is the orbit-generating pattern: given a group $G = \langle r, f \rangle$ with $r^n = e$, iterate $n$ times emitting $\{r^i, r^i f\}$ — this enumerates all $2n$ elements
- Exactly $|D_4| = 8$ tiles come out (some may coincide for symmetric inputs; that's the *stabilizer* shrinking the orbit)

**Performance impact**: Expanding each rule LHS into 8 orientations at parse time turns lookup into one `HashMap::get`. For an 18-iteration grid of ~4.8M blocks, that's tens of millions of lookups — any work shifted from lookup to parse is a giant win. See [aoc2017 Day 21 Function Guide](../../advent_of_code/aoc2017/Problem_Statements/days/day21_function_guide.md).

### **Rotation via index arithmetic**

Why does `new[r][c] = old[N-1-c][r]` implement 90° clockwise rotation? Think of it as a coordinate change:

```
Original square (N×N):        Rotated 90° CW:
  (r, c) = (row, col)            (r', c') = new position

  (0, 0) →  top-left             maps to  (0, N-1) →  top-right
  (0, 1) →  top row, col 1       maps to  (1, N-1)
  ...
  (N-1, N-1) → bottom-right      maps to  (N-1, 0) → bottom-left
```

General rule: $(r, c) \mapsto (c, N-1-r)$ (rotating the *point*). To write the new grid by reading from the old one, we invert that map: the new position $(r', c')$ came from $(N-1-c', r')$. Hence `new[r'][c'] = old[N-1-c'][r']`.

This is the difference between **active** transformation (move points) and **passive** transformation (rewrite coordinates). Both describe the same rotation; choose whichever makes the code cleaner.

---

## 🌳 Related Concepts

### **Prerequisites**
- [[set-theory-fundamentals]] — A group *is* a set with binary operation + axioms
- 2D Cartesian geometry (rows, columns, square symmetry)

### **Related Mathematical Concepts**
- [[Symmetry in Algorithms]] — D₄ is one instance of the broader "exploit symmetry to shrink search space" pattern
- Cyclic groups ($C_n$) — the rotation subgroup of $D_n$
- Klein four-group $V_4$ — D₂ and a subgroup of D₄ in two different ways
- Symmetric groups ($S_n$) — D_n embeds in $S_n$ (permutes $n$ vertices); D₄ ⊂ S₄
- Orbit-stabilizer theorem — explains why some tiles have fewer than 8 distinct orientations (larger stabilizer)

### **Generalizations**
- **$D_n$** for any $n \geq 3$: symmetries of a regular n-gon, $|D_n| = 2n$
- **Cube rotation group**: 24 elements (proper rotations only, no reflections). Appears in AoC 2020 Day 20 ("Jurassic Jigsaw") and 3D puzzle problems
- **Full cube symmetry group**: 48 elements (rotations + reflections)
- **Space groups** (crystallography): infinite groups combining translations with point-group symmetries

### **Related Rust Patterns**
- Orbit enumeration: same pattern of "generators × one coset representative" applies to any finitely-generated group — swap `rotate`/`flip` for whatever primitives fit the problem
- Canonical-form hashing: an alternative to expand-at-parse is to map each tile to the lex-smallest of its orbit and hash that (see [[common-traits-pattern]] for Hash + Eq requirements)
- HashMap of orbit representatives: classical technique for de-duplicating configurations under a symmetry group (game board states, molecular conformations, circuit layouts)

### **Applications in AoC / Competitive Programming**
- **Tile-matching problems**: AoC 2017 Day 21 (Fractal Art), AoC 2020 Day 20 (Jurassic Jigsaw — uses *cube* rotations for 3D tile edges)
- **Polyomino enumeration**: counting distinct shapes up to rotation/reflection
- **Board-game state dedup**: reduce 8× (or more) search space in puzzles with bilateral/quadrilateral symmetry
- **Sudoku, n-queens**: D₄ symmetry cuts solution search space by a factor up to 8
- **Cellular automata**: identifying equivalent patterns under symmetry (Conway's Life, Rule 30 generalizations)

---

## 🎯 Common Patterns

### **Pattern 1: Generate the orbit of a tile under D₄**

```rust
fn orbit<T: Clone>(start: T, rotate: fn(&T) -> T, flip: fn(&T) -> T) -> Vec<T> {
    let mut out = Vec::with_capacity(8);
    let mut cur = start;
    for _ in 0..4 {
        out.push(flip(&cur));
        out.push(cur.clone());
        cur = rotate(&cur);
    }
    out
}
```

Applies to any type with a "rotate 90° CW" and a "flip" operation — tiles, 2D grids, bitmasks, polynomials in two variables.

### **Pattern 2: Canonical representative (lex-min orbit)**

```rust
fn canonical<T: Clone + Ord>(t: &T, rotate: fn(&T) -> T, flip: fn(&T) -> T) -> T {
    orbit(t.clone(), rotate, flip).into_iter().min().unwrap()
}
```

Used to dedupe configurations — two tiles are equivalent under $D_4$ iff their canonical forms are equal.

### **Pattern 3: Expand-at-parse for symmetry-aware lookup**

```rust
// For each rule, insert all 8 orientations of the LHS into the HashMap
for variant in orientations(&lhs_tile) {
    rules.insert(tile_to_string(&variant), rhs_tile.clone());
}
// Lookup is now a single HashMap::get — lookup-time cost O(1)
```

Trades a fixed 8× cost at parse for a permanent O(1) lookup. Right call whenever lookups outnumber rule definitions by any significant factor.

### **Pattern 4: Symmetry detection (stabilizer size)**

```rust
// A tile is "fully symmetric" iff all 8 orientations are equal
fn stabilizer_size<T: Eq + Clone>(t: &T, rotate: fn(&T) -> T, flip: fn(&T) -> T) -> usize {
    let orbit_unique: HashSet<_> = orbit(t.clone(), rotate, flip).into_iter().collect();
    8 / orbit_unique.len()
}
```

By the **orbit-stabilizer theorem**, $|\text{orbit}| \cdot |\text{stabilizer}| = |G|$. Measuring orbit size gives stabilizer size for free — a cheap symmetry classifier.

---

## 📖 Resources

### **Mathematical Foundations**
- [Dihedral group (Wikipedia)](https://en.wikipedia.org/wiki/Dihedral_group) — formal treatment, full Cayley table for D₄
- [Orbit-stabilizer theorem](https://en.wikipedia.org/wiki/Group_action#Orbit-stabilizer_theorem) — counting orbit sizes without enumeration
- Dummit & Foote, *Abstract Algebra*, Ch. 1 — standard undergrad treatment

### **Visualisation**
- [Symmetries of a square](https://en.wikipedia.org/wiki/Dihedral_group_of_order_8) — animations of each of the 8 elements
- Cayley graphs of $D_4$ — two connected octagons, good for building composition intuition

---

## 💡 Key Insights

**Why care about group theory as a Rust programmer?**

- **Correctness without case analysis**: If you know the group has exactly 8 elements, the code that emits "4 rotations × 2 flips" is complete by construction. No need to enumerate "have I covered rotate-then-flip-then-rotate? what about flip-twice?" — group-theoretic closure guarantees it.
- **Clean generator-based code**: Two primitives (`rotate`, `flip`) generate everything. Your code has 2 non-trivial functions, not 8.
- **Generalisation**: The same loop structure works for $D_5$, $D_6$, $D_{10}$, the cube rotation group — only the generators and loop bound change.

**Why "expand at parse" beats "canonicalize at lookup" for AoC Day 21:**

Canonicalize-at-lookup costs 8 orientations + 8 comparisons per lookup. Expand-at-parse costs ~8 rule insertions per rule, then one HashMap::get per lookup. With 108 rules and millions of lookups, the math is obvious. This generalises: **whenever symmetry classes dominate your algorithm, do the group work once at setup**.

**The non-obvious bit about rotation index math.**

`new[r][c] = old[N-1-c][r]` is the **passive** transformation (how to rewrite coordinates). The **active** version — moving a pixel from $(r, c)$ to $(c, N-1-r)$ — describes the same rotation but is harder to write as a single loop. The passive form falls out of the equation $P_{\text{new}} = R \cdot P_{\text{old}}$ when you solve for old-in-terms-of-new, which is why it's the natural choice for grid code.

**Orbit sizes aren't always 8.**

For a 3×3 tile with full D₄ symmetry (all rows and columns identical), the orbit is just one tile. For a tile with only rotational symmetry (but no reflective), the orbit is 4 (just the rotations are distinct). The orbit-stabilizer theorem handles this: `|orbit| = 8 / |stabilizer|`. In AoC Day 21 this shows up as duplicate HashMap inserts — benign, but good to know the theory accounts for it.

---

## 📊 Quick Reference

| Fact | Value |
|------|-------|
| Group name | $D_4$ (dihedral group of order 8) |
| $|D_4|$ | 8 elements |
| Generators | $\{r, f\}$ where $r^4 = f^2 = e$, $frf = r^{-1}$ |
| Rotation subgroup | $C_4 = \{e, r, r^2, r^3\}$, order 4 |
| Center | $\{e, r^2\}$, order 2 |
| Is abelian? | No |
| Conjugacy classes | 5 (singletons $\{e\}, \{r^2\}$; pairs $\{r, r^3\}$, $\{f, fr^2\}$, $\{fr, fr^3\}$) |
| Related 3D analogue | Cube rotation group (24 elements) |

---

**Navigation**: [[set-theory-fundamentals]] · [[Symmetry in Algorithms]] · [AoC 2017 Day 21](../../advent_of_code/aoc2017/Problem_Statements/days/day21_function_guide.md) · [math-foundations/README](README.md)
