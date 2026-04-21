# Day 20: Particle Swarm — Function Guide

**Problem**: 1000 particles in 3D space, each with integer position `p`, velocity `v`, and acceleration `a`. Per tick: `v += a`, then `p += v`.
**Answers**: Part 1 = **300**, Part 2 = **502**
**Code**: [day20.rs](../../src/solver/day20.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Model](#data-model)
3. [Parsing](#parsing)
4. [Part 1 — Closed-Form Ranking](#part-1--closed-form-ranking)
5. [Part 2 — Simulated Collisions](#part-2--simulated-collisions)
6. [Termination Heuristic](#termination-heuristic)
7. [Benchmarks](#benchmarks)
8. [Key Patterns](#key-patterns)
9. [Integrator Notes](#integrator-notes)

---

## Problem Summary

Input format, one particle per line:

```
p=<-11104,1791,5208>, v=<-6,36,-84>, a=<19,-5,-4>
```

Each tick, every particle updates simultaneously: first `v += a`, then `p += v`. "Distance from origin" is Manhattan: `|x| + |y| + |z|`.

- **Part 1**: which particle (0-indexed) stays closest to the origin in the long term?
- **Part 2**: particles sharing a position *at the same tick* are all destroyed together. Simulate until the swarm stabilises; report survivor count.

Part 1 is a closed-form ranking. Part 2 is a simulation with collision detection and a termination heuristic.

---

## Data Model

```rust
type Vec3 = [i64; 3];

#[derive(Debug, Clone, Copy)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}
```

`[i64; 3]` was chosen over a named struct because:
- It hashes directly (`Vec3` is `Hash + Eq` via the array impl), enabling `HashMap<Vec3, usize>` collision buckets in Part 2.
- Component-wise arithmetic stays explicit (`q.v[0] += q.a[0]`), which compiles to three straight adds — no method call overhead.
- `i64` gives headroom: after ~60 ticks at `|a| ≤ 20`, positions stay well under `2³¹`, but this bound depends on the input and the cost of `i64` vs `i32` is zero on x86-64.

Mission 6's `Coord` is 2D `usize`, so it doesn't fit — this is the sort of problem where a bespoke numeric type is the right call.

---

## Parsing

```rust
fn parse_line(line: &str) -> Particle {
    let mut p = [0i64; 3];
    let mut v = [0i64; 3];
    let mut a = [0i64; 3];
    for field in line.split(", ") {
        let (tag, rest) = field.split_once('=').unwrap();
        let triple = parse_triple(rest);
        match tag.trim() {
            "p" => p = triple,
            "v" => v = triple,
            "a" => a = triple,
            other => panic!("unexpected field tag: {other}"),
        }
    }
    Particle { p, v, a }
}
```

Tag-dispatched assignment rather than positional parsing: if the input ever reorders fields (some AoC variants do), the parser stays correct. The `split("> ")` / regex alternatives are either more fragile or drag in a dependency.

`parse_triple` strips `<` and `>`, splits on `,`, parses three `i64`s. Kept as a helper so the main parser reads as data flow rather than character arithmetic.

---

## Part 1 — Closed-Form Ranking

```rust
fn closest_long_term(particles: &[Particle]) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, q)| (manhattan(q.a), manhattan(q.v), manhattan(q.p)))
        .map(|(i, _)| i)
        .unwrap()
}
```

**Why this is correct without simulation.** Position at tick `t` is a quadratic in `t`:

```
p(t) = p₀ + v₀·t + a·(t·(t+1)/2)     (discrete-time, because v is updated before p)
```

So Manhattan distance from the origin is a polynomial in `t` whose coefficients are `|a|/2` (degree 2), `|v| + |a|/2` (degree 1), and `|p₀|` (degree 0). As `t → ∞`, the highest-degree non-zero term dominates. **Ranking by `(|a|, |v|, |p|)` lexicographically is ranking by the polynomial coefficients of `|p(t)|` from highest degree down — exactly what "long-term closest" means.**

**The special case that trips you up.** For this input, the winner (particle #300) has **`|a| = 0`**. Its position grows *linearly* (`|p(t)| ≈ |v|·t`) while every particle with `|a| > 0` grows *quadratically*. Linear always loses to quadratic eventually, so **any particle with `|a|=0` beats any particle with `|a|>0` long-term** — no contest on the magnitude. The lexicographic key handles this automatically: `|a|=0` sorts before any `|a|>0`, regardless of how nice the non-zero `|a|` looks.

Among zero-acceleration particles, the one with smallest `|v|` wins (they all grow linearly, ranked by slope). Among same-`|v|` particles, `|p₀|` breaks it. All of this falls out of the tuple `(|a|, |v|, |p|)` with Rust's derived `Ord`.

**Why Manhattan is the right norm here.** The problem's distance metric is Manhattan. Manhattan distance of a vector `p(t)` with components `pᵢ(t) = pᵢ₀ + vᵢ·t + aᵢ·t·(t+1)/2` is `Σᵢ |pᵢ(t)|`. For large `t`, each `|pᵢ(t)|` is dominated by `|aᵢ|·t²/2` (when `aᵢ ≠ 0`) or `|vᵢ|·t` (when `aᵢ = 0`). So `|p(t)|₁ ~ |a|₁·t²/2` asymptotically — the `|a|₁` sum is exactly the dominant coefficient. If the problem used Euclidean, you'd rank by `|a|₂`; same principle, different norm.

**"Long term" is not short.** The predicted winner takes a while to actually *become* the closest. For this input, empirically:

```
tick     0  closest=#99  (|p|=252)    predicted=#300 (|p|=3590)   gap=3338
tick    10  closest=#144 (|p|=43)     predicted=#300 (|p|=2330)   gap=2287
tick   100  closest=#172 (|p|=5284)   predicted=#300 (|p|=9010)   gap=3726
tick   500  closest=#300 (|p|=59410)  predicted=#300 (|p|=59410)  gap=0
tick  2000  closest=#300 (|p|=248410) predicted=#300 (|p|=248410) gap=0

T₀ ≈ 334 — from tick 334 onward, #300 is the single closest at every tick.
```

#300 starts 14× farther from the origin than the momentary leader, and takes over 300 ticks to overtake. Early on the leaderboard churns (#99 → #144 → #15 → #172 → …) — these are particles with non-zero acceleration whose trajectories happen to pass near the origin before rocketing away. They look closest in the interim but are doomed by their own `|a|·t²/2` term.

The runnable visualiser is at [`examples/day20_longterm.rs`](../../examples/day20_longterm.rs):

```
cargo run --release --example day20_longterm [TICKS]
```

Useful for building intuition — shows exactly how the closed-form ranking relates to what's actually happening in the simulation.

**Cost.** O(n) arithmetic. The measured 215 µs is dominated by *parsing* (1000 lines, ~45 integer parses each) — the min-scan itself is under a microsecond.

---

## Part 2 — Simulated Collisions

```rust
fn survivors_after_collisions(particles: &[Particle]) -> usize {
    let mut alive: Vec<Particle> = particles.to_vec();
    let mut ticks_since_collision = 0usize;
    const QUIET_WINDOW: usize = 60;

    while ticks_since_collision < QUIET_WINDOW {
        for q in alive.iter_mut() {
            q.v[0] += q.a[0]; q.v[1] += q.a[1]; q.v[2] += q.a[2];
            q.p[0] += q.v[0]; q.p[1] += q.v[1]; q.p[2] += q.v[2];
        }

        let mut buckets: HashMap<Vec3, usize> = HashMap::with_capacity(alive.len());
        for q in &alive { *buckets.entry(q.p).or_insert(0) += 1; }

        let before = alive.len();
        alive.retain(|q| buckets[&q.p] == 1);
        if alive.len() == before { ticks_since_collision += 1; }
        else                      { ticks_since_collision = 0; }
    }
    alive.len()
}
```

**Order of operations matters.** The problem specifies that *within a tick*, velocities update before positions. So the tick body is `v += a; p += v;` in that order — not `p += v + a/2` or anything fancier. Flipping the order shifts every particle by one tick's velocity and produces wrong answers on the example.

**Simultaneous collision.** All particles move first, *then* we check for collisions. This is why we update every particle in one pass before hashing — we can't interleave motion and collision checks.

**Bucket-and-filter instead of pair-scan.** Naive `O(n²)` pair comparison at 1000 particles is ~500k compares per tick. Building `HashMap<Vec3, usize>` and then retaining `count == 1` entries is `O(n)` per tick. At ~60 ticks of actual work, that's ~60k hash ops vs ~30M compares — a ~500× difference in the inner loop.

**`retain` preserves order**, which doesn't matter for the answer (we return a count), but matters if you ever want to extend this to "which particles survive" — the survivor indices stay stable across ticks.

---

## Collision Timeline (Empirical)

Running the simulation for real on this input ([`examples/day20_collisions.rs`](../../examples/day20_collisions.rs) logs every tick at which ≥1 particle died):

```
tick  died  alive  frac
  10    18    982  98.2%   ← first collision, 10 ticks in
  11     5    977  97.7%
  12    41    936  93.6%
  14    46    890  89.0%   ← peak carnage begins
  15     6    884  88.4%
  16    25    859  85.9%
  17    35    824  82.4%
  18    45    779  77.9%
  19     9    770  77.0%
  21    19    751  75.1%   ← tick 20 was a brief lull
  22    14    737  73.7%
  23    15    722  72.2%
  24    35    687  68.7%
  25    14    673  67.3%
  27     9    664  66.4%   ← tick 26 quiet
  28    24    640  64.0%
  29     8    632  63.2%
  31    39    593  59.3%   ← tick 30 quiet; then big resurgence
  32     8    585  58.5%
  33    11    574  57.4%
  34    18    556  55.6%
  35    17    539  53.9%
  37     3    536  53.6%   ← tick 36 quiet
  38     8    528  52.8%
  39    26    502  50.2%   ← last collision; 60 quiet ticks follow
```

Summary: **99 total ticks**, last collision at **tick 39**, **25 distinct collision events**, **498 of 1000 particles destroyed** (the answer is what's left: **502 survivors**, 50.2% of the swarm).

Three things worth noticing:

**1. No collisions in ticks 0–9.** Particles haven't converged on the origin yet. The collision phase has a clear start.

**2. The collision phase is ~30 ticks wide.** All destruction happens in ticks 10–39. Inside that window the death rate peaks around tick 14–18 (45+ per tick) and tails off but never vanishes until tick 39.

**3. There are brief lulls mid-storm.** Ticks 13, 20, 26, 30, 36 had zero deaths, even though collisions resumed right after. This is why `QUIET_WINDOW` must be a *sliding* counter (reset on any death) rather than a cumulative one — a fixed "wait N ticks then stop" heuristic with a too-small N would have terminated during a lull and reported a wrong answer.

---

## Termination Heuristic

Not the prettiest part. The problem doesn't give an explicit stopping condition — "simulate until no more collisions occur" is well-defined in principle but requires proving divergence, which is awkward across 502 surviving particles in 3D.

**What would be rigorous.** For every pair `(i, j)` still alive, check whether `p_i(t) == p_j(t)` has any integer solution `t > now`. Each axis gives a quadratic `(pᵢ - pⱼ) + (vᵢ - vⱼ)·t + (aᵢ - aⱼ)·t·(t+1)/2 = 0`, and all three axes must share a root. That's ~125k pair checks, each a small integer-root search — doable, but the code runs into a lot of edge cases (zero acceleration, zero velocity, negative `t`).

**What I did instead.** Run a sliding window of `QUIET_WINDOW = 60` ticks with no collisions; bail out. 60 was chosen a priori because the input's accelerations are bounded by ~20 in magnitude and the initial position spread is ~10k, so diverging particles separate enough within 60 ticks that no late collision can plausibly occur. **Empirically validated by the timeline above**: the last real collision is at tick 39, so the solver terminates at tick 99 — comfortably past the last event. The longest mid-storm lull between collision events on this input is just 1 tick (ticks 13, 20, 26, 30, 36), so any `QUIET_WINDOW ≥ 2` would survive those pauses. Even `QUIET_WINDOW = 20` would have given the right answer here; `60` is chosen as margin against weirder inputs.

Cost of the heuristic: we run ~60 extra ticks past the last collision. At ~2.1 ms total runtime those 60 quiet ticks are roughly 60% of the measured cost, but each quiet tick is cheaper than a collision tick (no map entries churn, fewer alive particles), so the real overhead is more like 30–40%. Acceptable tax for a ~20-line implementation instead of a 200-line quadratic-root-finder.

---

## Benchmarks

| Stage | Time |
|-------|------|
| Part 1 alone | **214.84 µs** |
| Part 2 alone | **2.115 ms** |
| Combined `solve()` | **2.124 ms** |

Observations:
- **Part 1 ≈ parsing cost.** The min-scan is trivial; 215 µs is essentially the time to `parse_line` × 1000.
- **Combined ≈ Part 2 alone.** Part 1's work is dominated by Part 2's simulation — no reason to split them.
- **Parse-once pattern works here**: `solve()` parses once and runs both computations; Part 1 pays the parse tax once in the combined path.
- **Part 2 scales as O(ticks × n)** with `n` shrinking over time. Measured ~60–80 simulated ticks (most ticks after tick ~40 are collision-free and only pad the quiet window).

---

## Key Patterns

### Closed-form over simulation when the asymptote is clear
Any problem that asks "long-term behavior of a polynomial system" has a closed-form ranking available — you just need to identify the dominant term. Part 1 is O(n) arithmetic; a naïve "simulate for 10000 ticks then rank by distance" would be O(n · ticks) with no accuracy gain. Recognising the polynomial structure collapses the whole first half of the problem to one `min_by_key`.

### Lexicographic keys as polynomial-coefficient comparison
`(|a|, |v|, |p|)` isn't three arbitrary fallbacks — it's the coefficient vector of `|p(t)|` as a polynomial in `t`, from highest degree to lowest. Lex-compare by this key and you're comparing polynomials in the "as `t → ∞`" ordering. This generalises: any polynomial-state system with a definable norm has a long-term ranking you can compute without simulation, as long as you order the key by descending degree. It also means *do not* reach for the `|p|` field as a first-pass filter — the degrees matter, and `|a|=0` must win over `|a|=huge` even when `|p₀|` is reversed. Think of the tuple as `(t²-coeff, t¹-coeff, t⁰-coeff)`.

### Validate the closed-form with a throwaway simulator
Closed-form reasoning is high-leverage but easy to get subtly wrong — especially around boundary cases like `|a|=0` (linear vs quadratic) or Euclidean-vs-Manhattan norms. A 100-line simulation example ([`examples/day20_longterm.rs`](../../examples/day20_longterm.rs)) that snapshots the rankings at a few ticks is a cheap sanity check. It also yields the empirical `T₀` — the tick by which the predicted winner has permanently taken the lead — which is interesting context on its own (for this input, `T₀ ≈ 334`, meaning "long-term" is hundreds of ticks, not a handful).

### `[T; N]` as a hashable vector type
`HashMap<[i64; 3], _>` just works — no derive, no newtype, no `Hash` impl. When the semantics are "a small fixed-size tuple of integers used as a map key," the array type is the cleanest choice in current Rust. `(i64, i64, i64)` hashes too, but indexing and arithmetic on arrays read better once you're doing componentwise ops.

### Bucket-and-filter for multi-way equality
The "all particles sharing a cell die together" rule is equivalent to "keep only particles whose position bucket has count 1." A `HashMap<Pos, count>` pass followed by `retain(|q| bucket[q.p] == 1)` expresses this in two lines with no pair enumeration. This pattern shows up often in AoC — any time the question is "which items are unique by some key," reach for bucket-and-filter instead of `O(n²)` pair checking.

### Quiet window as a termination proxy
When a rigorous termination condition is expensive to compute and the system is known to be monotone (here: accelerations are fixed, so divergence is eventually monotone), a quiet-window heuristic trades a small runtime tax for a large implementation simplification. Important: this is a heuristic you should *understand why it works for your input* — the window size depends on bounds you can read off the input (max `|a|` vs initial position spread).

---

## Integrator Notes

- **No mission reuse here.** Mission 6 (`Grid<T>` / `Coord`) is 2D `usize`-indexed and doesn't fit 3D signed integer space. This is a case where a bespoke `[i64; 3]` is cleaner than bending a general-purpose component to fit — the integrator mindset is "compose when composition helps," not "compose at any cost."
- **AUTOSAR analogue.** The simulation is a classic periodic task: at each tick, every particle's state updates from its own data plus a global clock. No cross-particle dependency *except* the collision check — which is a read-only reduction after the motion update. If this were an AUTOSAR runnable, `motion_update()` would be a per-particle runnable and `collision_filter()` would be a follow-up task that reads all positions and marks dead particles. The "update all, then reduce" split is exactly the pattern AUTOSAR runnables use for simultaneous update semantics.
- **Where I'd revisit.** The `QUIET_WINDOW` heuristic works but is input-specific. For a reusable particle simulator, I'd replace it with the rigorous quadratic pairwise collision-time solver — more code, but defensible on any input.

---

**Navigation**: [← Day 19](day19_function_guide.md) | [All Days](../summary_2017.md) | Day 21 →
