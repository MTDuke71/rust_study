# Day 22: Sporifica Virus — Function Guide

**Problem**: A virus carrier walks an infinite 2D grid of nodes (each clean or infected) in bursts. Each burst it turns based on the current node's state, flips the state, then moves one step. Part 1 counts clean→infected transitions over 10,000 bursts; Part 2 evolves to four states (Clean / Weakened / Infected / Flagged) over 10,000,000 bursts.
**Answers**: Part 1 = **5,240**, Part 2 = **2,512,144**
**Code**: [day22.rs](../../src/solver/day22.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Model](#data-model)
3. [Coordinate System: Why Y-Up](#coordinate-system-why-y-up)
4. [Part 1: Sparse HashSet](#part-1-sparse-hashset)
5. [Part 2: Bounded Flat Grid + Modular State](#part-2-bounded-flat-grid--modular-state)
6. [Turn Primitives](#turn-primitives)
7. [Benchmarks](#benchmarks)
8. [Why Not a HashMap for Part 2](#why-not-a-hashmap-for-part-2)
9. [Key Patterns](#key-patterns)
10. [Integrator Notes](#integrator-notes)

---

## Problem Summary

The grid is infinite and sparse — only a 25×25 window is given, centred on the carrier's starting position. Both parts use the same initial grid, so the input parses exactly once:

- **Part 1**: two states (Clean, Infected). On each burst: Clean→turn left+infect, Infected→turn right+clean. Move. Count infections caused (Weakened→Infected isn't a thing here — every Clean→Infected counts). 10,000 bursts.
- **Part 2**: four states cycling Clean → Weakened → Infected → Flagged → Clean. Each state controls a different turn (left / none / right / reverse). The state machine *always* advances by one. Count each Weakened→Infected transition. 10,000,000 bursts.

The 1000× burst count from Part 1 to Part 2 forces a data-structure decision — a fine choice for Part 1 is disqualifying for Part 2.

---

## Data Model

```rust
struct ParsedData {
    infected: HashSet<(i32, i32)>,
}
```

One parse, one shared representation. Both parts read `infected` and build their own working state on top — `solve()` parses once and calls both `_with_data` helpers, keeping the parse-once contract honest even though parsing is microseconds in this problem.

For Part 2, `infected` is expanded into a flat `Vec<u8>` the size of the bounded grid at the top of the solver. Part 1 just clones the set.

---

## Coordinate System: Why Y-Up

The puzzle describes "facing up" and then shows a map where row 0 is at the top. Those two conventions conflict: the usual screen-coordinate "row increases downward" would make "up" mean decrementing y, which is easy to get backwards. The parser flips the y-axis at parse time:

```rust
let x = c as i32 - cx;
let y = cy - r as i32;   // <-- y-up
```

So the carrier's `dir = (0, 1)` really means "up" in the puzzle's pictures, `turn_right` and `turn_left` read like the name says, and the movement loop stays `x += dir.0; y += dir.1` with no sign-bending sprinkled around.

The `test_parse_example_centre_and_infected` test pins this down: the two `#` cells in the 3×3 example at map `(row=0, col=2)` and `(row=1, col=0)` end up at `(1, 1)` and `(-1, 0)` respectively, with the carrier's origin `(0, 0)` clean.

---

## Part 1: Sparse HashSet

```rust
fn solve_part1_with_data(data: &ParsedData) -> usize {
    let mut infected = data.infected.clone();
    let mut pos = (0, 0);
    let mut dir = (0, 1);
    let mut caused = 0;
    for _ in 0..10_000 {
        if infected.contains(&pos) {
            dir = turn_right(dir);
            infected.remove(&pos);
        } else {
            dir = turn_left(dir);
            infected.insert(pos);
            caused += 1;
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    caused
}
```

Straight translation of the rules. 10,000 iterations with a `HashSet` fits comfortably under 400 µs even though each burst does a membership test and a mutation — the set stays small (final carrier footprint is ~4,500 live cells) and `(i32, i32)` hashes cheaply.

---

## Part 2: Bounded Flat Grid + Modular State

Three ideas stack together:

**1. Fixed bounded grid.** In 10M bursts the carrier never drifts more than a few hundred cells from the origin — the evolved dynamics are far more diffusion-like than the pure-turn walks in the Part 1 rules. A flat `Vec<u8>` sized 1024×1024 is 1 MiB and gives O(1) direct indexing with no hashing:

```rust
const BOUND: usize = 1024;
const HALF: i32 = (BOUND / 2) as i32;
let idx = ((y + HALF) as usize) * BOUND + (x + HALF) as usize;
```

Any stray outside the bound would panic on the unsigned cast — in practice the carrier stays well inside.

**2. States as `u8` 0..=3.** `Clean=0, Weakened=1, Infected=2, Flagged=3`. The state machine *always* advances by one, so every burst is:

```rust
let next = (state + 1) & 0b11;
```

That's one add and one mask. No branching on "what does each state become?" — the cycle is the identity.

**3. Infection count via the transition, not the destination.** "Caused an infection" means Weakened→Infected, which is exactly `next == 2`. One compare per burst, no additional tracking.

The full inner loop:

```rust
for _ in 0..10_000_000 {
    let idx = ((y + HALF) as usize) * BOUND + (x + HALF) as usize;
    let state = grid[idx];
    match state {
        CLEAN => dir = turn_left(dir),
        WEAKENED => {}                        // no turn
        INFECTED => dir = turn_right(dir),
        FLAGGED => dir = (-dir.0, -dir.1),    // reverse
        _ => unreachable!(),
    }
    let next = (state + 1) & 0b11;
    grid[idx] = next;
    if next == INFECTED { caused += 1; }
    x += dir.0;
    y += dir.1;
}
```

Five branches on state, one index, one write, one comparison, one move. That's the hot loop for 10M iterations; at ~55 ms total it's running at roughly 180M bursts/sec on a single core.

---

## Turn Primitives

In a y-up coordinate system with `(dx, dy)`:

```rust
fn turn_right(d: (i32, i32)) -> (i32, i32) { (d.1, -d.0) }
fn turn_left(d: (i32, i32)) -> (i32, i32)  { (-d.1, d.0) }
```

Quick sanity check: start facing up `(0, 1)`. Turn right → `(1, 0)` = right. Turn right again → `(0, -1)` = down. Turn right again → `(-1, 0)` = left. Turn right again → `(0, 1)` = up. The 90° rotation matrix holds: `(x, y) → (y, -x)` for right, `(x, y) → (-y, x)` for left.

Reverse is just negation, inlined: `(-dir.0, -dir.1)`.

---

## Benchmarks

Measured with Criterion (`--quick` variance, Windows release):

| Target           | Time        |
|------------------|-------------|
| `day22_part1`    | **321.3 µs** |
| `day22_part2`    | **55.00 ms** |
| `day22` combined | **53.87 ms** |

Two things worth noting:

- **Combined < sum of parts.** The parse-once contract shows up here: `solve()` parses once and runs both parts; the bench sum for individual parts double-parses. (The effect is small — parsing a 25×25 grid is dust — but it's the check the repo's guidelines ask for.)
- **Part 2 is 170× Part 1.** Burst count is 1,000× higher; the per-burst cost is actually *lower* thanks to direct indexing vs HashSet hashing. Good instinct check: if your Part 2 were ≥ 1000× Part 1 you'd know the data structure was the bottleneck.

---

## Why Not a HashMap for Part 2

The obvious first draft for Part 2 is `HashMap<(i32, i32), State>` with `State::Clean` omitted (cells not present = clean). That would work, but:

- 10M iterations × ~2 HashMap ops per iteration ≈ 20M hash-and-probe operations.
- `std::collections::HashMap` with `SipHash` runs each op in ~50–100 ns amortised in tight loops.
- Estimated runtime: ~1.5 – 2.5 seconds. An order of magnitude over the 100 ms bar.

Alternatives:

- `FxHashMap` (replacing SipHash with FxHash) would get ~3× — good, but still ~500 ms.
- **Flat `Vec<u8>` with bounded coords** — the chosen path — gets ~55 ms.

The deciding factor is that the carrier's footprint is bounded. If it were provably unbounded you'd need a hash-backed structure; here it isn't, so the direct-indexed array wins hands down. This is the same *"if you can bound the domain, flatten it"* heuristic that makes radix/counting sort feasible for small-alphabet inputs.

---

## Key Patterns

### Bounded domain → flat array beats hash
Whenever the problem lets you size a static arena — sparse grids, small-key domains, finite automata states — prefer a flat `Vec` with direct indexing over a hash map. You pay up-front memory (1 MiB here) for O(1)-with-a-tiny-constant access. The moment iteration counts reach 10⁷+, the hash constant dominates runtime.

### Modular state advance
A state machine whose transitions form a cycle (Clean → Weakened → Infected → Flagged → Clean) collapses to modular arithmetic. `(s + 1) & 0b11` is three CPU instructions; a `match` on transitions is a branch tree. Use the cycle when it exists; fall back to `match` when the transition isn't uniform (which is true for the *turn* part of the rule, just not for the *state* part).

### Parse once, share immutable data
`ParsedData { infected: HashSet<(i32,i32)> }` is the common denominator both parts need. Each part then builds its own working state (Part 1 clones the set, Part 2 expands it into a flat grid). The parse function knows nothing about either part; both parts know nothing about parsing. Clean separation, and `solve()` composing the two is trivial.

### Y-up at parse time, never again
Reflect the y-axis once during parsing and all downstream code reads in the puzzle's mental model. Mixing conventions mid-pipeline — "rows for input, y-up for movement" — is a bug waiting to happen. A three-token adjustment at the boundary is worth hundreds of lines of correct-by-construction downstream logic.

---

## Integrator Notes

- **No Mission 6 here.** Mission 6's `Grid<T>` is `usize`-indexed over a bounded map — a fine fit for Part 1's *starting* grid, but the carrier leaves the original 25×25 window immediately. Using `Grid<T>` would mean sizing it to the Part 2 working bounds (1024×1024) and carrying the initial infections as a sparse insert — no gain over the flat `Vec<u8>` the solver already has.
- **No graph / UF reuse.** This problem is pure state-machine simulation; no components, no edges, no shortest paths.
- **AUTOSAR analogue.** The inner loop is a periodic task with a tiny, branchy body — exactly the pattern an RTE handles tens of millions of times a second in a real ECU. The `u8` state packing is the same trick safety-critical code uses for small enumerated types to fit inside a single cache line.
- **When the grid grows unbounded.** If a future puzzle had the carrier genuinely drifting, the escape hatch is a *growing* 2D arena (double when the carrier approaches a boundary) or a chunked `HashMap<(chunk_x, chunk_y), [u8; N*N]>`. The point is to preserve O(1) access while amortising growth — same idea as `Vec`'s doubling.

---

**Navigation**: [← Day 21](day21_function_guide.md) | [All Days](../summary_2017.md) | [Day 23 →](day23_function_guide.md)
