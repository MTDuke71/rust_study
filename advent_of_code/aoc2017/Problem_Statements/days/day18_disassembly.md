# Day 18: Duet — Disassembly & Walkthrough

**Input**: [inputs/day18.txt](../../inputs/day18.txt) — 41 instructions.
**Answers**: Part 1 = **3188**, Part 2 = **7112** (= 56 × 127).

This isn't a random program. It's a **cooperative distributed bubble sort of 127 LCG-generated numbers**, with a clever program-splitter trick that makes the two participants (`p = 0` and `p = 1`) take different paths through the same bytecode.

**Companion files**:
- [day18_trace.rs](../../examples/day18_trace.rs) — pc reachability probe (`cargo run --example day18_trace`).
- [day18_sort_sim.py](../../analysis/day18_sort_sim.py) — standalone 10-value sort simulator with full rcv/snd trace and progression table (`python day18_sort_sim.py`).

---

## Full Disassembly

Addresses are 0-indexed (`pc`). Comments label each logical block.

```
; ─────────────────────────────────────────────────────────────
; BLOCK A — Init + program-splitter (both programs)
; ─────────────────────────────────────────────────────────────
 0: set i 31         ; loop counter for doubling a
 1: set a 1
 2: mul p 17         ; p0: 0*17=0.  p1: 1*17=17.
 3: jgz p p          ; p0 falls through (p=0).
                     ; p1 jumps +17 → pc 20 (skips seeding entirely).

; ─────────────────────────────────────────────────────────────
; BLOCK B — Build a = 2^31 - 1 (program 0 only)
; ─────────────────────────────────────────────────────────────
 4: mul a 2          ; a ← 2a
 5: add i -1
 6: jgz i -2         ; repeat 31 times → a = 2^31
 7: add a -1         ; a = 2^31 - 1 = 2147483647  (Mersenne prime M31)

; ─────────────────────────────────────────────────────────────
; BLOCK C — LCG seeder: 127 random values in [0, 9999] (p0 only)
;   The two muls compose: 8505 · 129749 = 1103515245 — glibc's
;   rand() multiplier. Increment 12345 and output mod 10000 also
;   match glibc. Modulus is M31 instead of 2^31.
; ─────────────────────────────────────────────────────────────
 8: set i 127
 9: set p 680        ; seed (p0 re-uses p as LCG state, no longer PID)
10: mul p 8505       ; \
11: mod p a          ;  > together: p ← (1103515245 · p) mod M31
12: mul p 129749     ; /
13: add p 12345      ; glibc rand() increment
14: mod p a          ; mod M31
15: set b p
16: mod b 10000      ; b = p mod 10000  (values fit in 14 bits)
17: snd b            ; ship to p1's inbox
18: add i -1
19: jgz i -9         ; 127 iterations → pc 10

; ─────────────────────────────────────────────────────────────
; BLOCK D — Part-1 trap / Part-2 branchpoint
; ─────────────────────────────────────────────────────────────
20: jgz a 3          ; p0: a = M31 > 0 → jump +3 to pc 23 (enter sort).
                     ; p1: a = 1   > 0 → jump +3 to pc 23 (enter sort).
                     ;   (In Part 1 single-program mode, a = M31 here too,
                     ;    so execution also goes to pc 23 — but the sort
                     ;    loop's first `rcv a` is the first non-zero rcv
                     ;    and immediately halts Part 1 with the last snd.)
21: rcv b            ; Endgame drain: reached from pc 40 after the final
                     ; f=0 pass. The program that finishes pass 56 first
                     ; (p1) lands here with 127 values still queued from
                     ; its peer's final pass, and spins through them via
                     ; pc 22. The program that finishes second (p0) lands
                     ; here with an empty queue and blocks immediately.
22: jgz b -1         ; Drain-loop back-edge (offset -1 → back to pc 21).
                     ; b is an LCG output mod 10000, always > 0, so this
                     ; always jumps. Drain exits only by the rcv blocking.

; ─────────────────────────────────────────────────────────────
; BLOCK E — Sort pass: min-extraction over 127 values
; ─────────────────────────────────────────────────────────────
23: set f 0          ; f = "did any swap happen this pass?"
24: set i 126        ; inner-loop counter: 126 more rcv's
25: rcv a            ; seed the running-min with the first value

; ---- inner loop ----
26: rcv b            ; next value
27: set p a
28: mul p -1
29: add p b          ; p = b - a
30: jgz p 4          ; if b > a → pc 34  (keep a, ship b)
                     ; else        fall through (a ≥ b: ship a, keep b)
31: snd a            ; fall-through path (a ≥ b): ship current min
32: set a b          ;                              running min ← b
33: jgz 1 3          ; unconditional jump +3 → pc 36  (skip "else")
34: snd b            ; taken-branch path (b > a): ship the bigger one
35: set f 1          ;                             record that a swap happened
36: add i -1
37: jgz i -11        ; back to pc 26 (126 total iterations)

38: snd a            ; after the loop, ship the final minimum
39: jgz f -16        ; if f=1 (some b>a occurred), restart at pc 23
40: jgz a -19        ; f=0 and a>0 → jump back to pc 21 (the "trap"):
                     ;   rcv b on an empty queue + the other program
                     ;   also stuck → deadlock → termination.
```

---

## High-level pseudocode

Same logic, with both programs' paths merged:

```python
# BLOCK A — run by both
i = 31
a = 1
p *= 17
if p > 0: pc += p       # p1 leaps over Blocks B, C, D's seeding

# BLOCK B — program 0 only
while i > 0:
    a *= 2
    i -= 1
a -= 1                  # a = 2^31 - 1

# BLOCK C — program 0 only: LCG producing 127 values in [0, 9999]
i = 127
p = 680
while i > 0:
    p = (p * 8505)  % a
    p = (p * 129749 + 12345) % a
    send(p % 10000)
    i -= 1

# BLOCK D — both programs reach here
# a is M31 for p0 and 1 for p1; both > 0 so both go to Block E.

# BLOCK E — the sort loop (both programs)
while True:
    f = 0
    a = recv()            # seed running min
    for _ in range(126):
        b = recv()
        if b > a:
            send(b)       # ship the larger
            f = 1
        else:
            send(a)       # ship the current min
            a = b         # new running min
    send(a)               # emit the minimum at the end
    if f == 0:
        break             # nothing moved → we're done
```

---

## Why this works — round-by-round

### The per-pass operation, stripped down

Receive 127 values, call them `x₀, x₁, …, x₁₂₆`. Let `mᵢ = min(x₀, …, xᵢ)` — the **running minimum** through position `i`.

In the loop, iteration `i` (for `i = 1..126`) does:
- Receive `xᵢ`.
- Compare `xᵢ` vs the current running min `mᵢ₋₁`.
- **Send the bigger of the two.**
- Keep the smaller as the new running min `mᵢ`.

After the loop finishes, send the final running min `m₁₂₆`.

Position-by-position, the output of one pass is:

```
out[i-1] = max(xᵢ, mᵢ₋₁)   for i = 1 .. 126
out[126] = m₁₂₆            (the global minimum of the input)
```

In plain English: **"at each step, emit the larger of (the new arrival) vs (the smallest value seen so far)."**

### Tiny example — 5 values

Input: `[3, 1, 4, 1, 5]`. Running mins: `m = [3, 1, 1, 1, 1]`.

| step | `xᵢ` | `mᵢ₋₁` | bigger? | emitted | new min `mᵢ` |
|------|------|--------|---------|---------|-------------|
| seed | 3    | —      | —       | (nothing yet) | 3 |
| 1    | 1    | 3      | 3       | **3**   | 1 |
| 2    | 4    | 1      | 4       | **4**   | 1 |
| 3    | 1    | 1      | 1       | **1**   | 1 |
| 4    | 5    | 1      | 5       | **5**   | 1 |
| end  | —    | —      | —       | **1** (`m₄`) | — |

Output: `[3, 4, 1, 5, 1]`. The multiset is unchanged — it's a permutation. The `f` flag got set twice (iterations 2 and 4, where `b > a`).

### The fixed point: descending order

`f` stays `0` for an entire pass iff every comparison satisfied `xᵢ ≤ mᵢ₋₁`. That means every new value was `≤` the minimum so far — i.e., the input was **non-increasing (descending sorted)**.

Sanity check: for a descending input, `xᵢ ≤ xᵢ₋₁ = mᵢ₋₁`, so `max(xᵢ, mᵢ₋₁) = mᵢ₋₁ = xᵢ₋₁`. That means `out[i-1] = xᵢ₋₁` (shifted by one) and `out[126] = x₁₂₆`. Output equals input — a true fixed point.

### Why it converges — it's bubble sort

Any "too-small" value has to bubble leftward toward its descending-sort slot. Trace of `[5, 4, 3, 2, 1, 0, 7]` (a single out-of-place element at the tail):

| pass | state |
|------|-------|
| 0 | `[5, 4, 3, 2, 1, 0, 7]` |
| 1 | `[5, 4, 3, 2, 1, 7, 0]` — `7` moves left one slot |
| 2 | `[5, 4, 3, 2, 7, 1, 0]` |
| 3 | `[5, 4, 3, 7, 2, 1, 0]` |
| 4 | `[5, 4, 7, 3, 2, 1, 0]` |
| 5 | `[5, 7, 4, 3, 2, 1, 0]` |
| 6 | `[7, 5, 4, 3, 2, 1, 0]` |
| 7 | same as pass 6 — `f = 0`, halt |

**Each pass moves every out-of-place element exactly one position toward its correct slot.** Worst case is O(N) passes — same dynamics as bubble sort. With 127 random-ish LCG values, 56 passes is well within the expected range (many small anomalies bubble in parallel).

### Pipelined concurrency — not strict ping-pong

A common misconception (and one this guide held in an earlier draft) is that the two programs take turns pass-by-pass. They don't. The scheduler in [day18.rs](../../src/solver/day18.rs) calls `step()` on **both** programs every tick — each attempts one instruction per tick. A "blocked" program simply means its current `rcv` found an empty inbox and will retry next tick.

So once messages start flowing, **both programs are executing simultaneously**, interleaved at instruction granularity. It's a dataflow pipeline, not a turn-based game.

#### How p1 bootstraps

Block E's preamble consumes **2 inputs before producing the 1st output**:

```
pc 23: set f 0        ; arithmetic
pc 24: set i 126      ; arithmetic
pc 25: rcv a          ; consumes input #1 (seeds running-min)
pc 26: rcv b          ; consumes input #2 (first comparison value)
pc 27-30: compute b-a, branch
pc 31 or 34: snd      ; ← first output into q0
```

That "2-in, 1-out" warm-up is the pipeline's priming cost. For the rest of the pass each `rcv b` is paired with one `snd` (1-in, 1-out), and the trailing `snd a` at pc 38 settles the extra accumulated value. Net per pass: **127 in, 127 out** — perfect flow balance.

#### Startup timing (tick-by-tick sketch)

| Tick range | p0 | p1 | q0 | q1 |
|-----------|-----|-----|-----|-----|
| 0-6  | Block A (splitter) | Block A → skip to pc 20 | [] | [] |
| 7-100 | Block B (31 iterations of `mul a 2`) | **blocked** at pc 25 (empty inbox) | [] | [] |
| ~110 | Block C emits 1st LCG value | unblocks, reads v₁ as `a`, blocks at pc 26 | [] | [] |
| ~120 | Block C emits 2nd | unblocks, reads v₂ as `b` | [] | [] |
| ~126 | Block C emits 3rd+ | **1st `snd` into q0** | [w₁] | [v₃] |
| ~126 onwards | Block C continues emitting (producer only) | full sort-pass loop (producer + consumer) | growing | drained |
| ~1,370 | Block C done; p0 enters Block E at pc 23 | part-way through pass 1, still producing | **rich in w values** | partially drained |

At tick ~1,370, p0 finishes Block C and drops into Block E. By that point q0 already contains dozens of values that p1 produced during p0's Block-C seeding phase — so p0 unblocks almost immediately at its first `rcv a` and starts running **its own** sort pass. From here out, both programs run concurrently, each feeding the other's queue.

#### Why p1 is roughly "one pass ahead" throughout

p1 enters Block E while p0 is still running Block C. So p1 starts its pass 1 almost 1,370 ticks before p0 starts its pass 1. This offset is **preserved** (approximately) for the entire run, because both programs execute the same instruction stream at the same rate:

```
              ← p0 seeding (Block C) →|  p0 Pass1   |  p0 Pass2   | … |  p0 Pass56  |
                                      |             |             |   |             |
|← p1 blocked →|  p1 Pass1            |  p1 Pass2   |  p1 Pass3   | … |  p1 Pass56  | (drain)
```

Both hit pc 38 (`snd a` end-of-pass) exactly **56 times**, verified by the reachability probe. Both send 127 values per pass. The answer is `56 × 127 = **7112**` — and because both programs do equal work, p0's send count is also 7112, but the puzzle asks for p1's.

#### The corrected mental model

> The scheduler isn't a turn-taker; it's a **parallel driver**. Each tick, both programs advance one instruction if they can. A blocked program "busy-waits" at its `rcv` until the peer's next `snd` fills its inbox, which is typically within a few ticks. Over the full run the two programs stay in an instruction-level lockstep with a small constant offset — a pipeline, not a relay race.

### The Duet sort IS classical bubble sort

The running-min mechanic isn't some novel algorithm — it's a **faithful, one-register implementation of descending bubble sort** with a left-to-right sweep. Compare one pass of each on `[4, 7, 1, 9, 3, 8, 2, 6, 5, 10]`:

**Descending bubble sort** (swap if `A[i] < A[i+1]`):
```
i=0: 4<7 swap → [7,4,1,9,3,8,2,6,5,10]
i=1: 4<1 keep
i=2: 1<9 swap → [7,4,9,1,3,8,2,6,5,10]
i=3: 1<3 swap → [7,4,9,3,1,8,2,6,5,10]
…
i=8: 1<10 swap → [7,4,9,3,8,2,6,5,10,1]
```

**Duet pass** (verified by [day18_sort_sim.py](../../analysis/day18_sort_sim.py)):
```
output: [7, 4, 9, 3, 8, 2, 6, 5, 10, 1]
```

Byte-for-byte identical. Register `a` is literally "the element being carried during an adjacent-pair sweep":

| Bubble-sort event | Duet instruction sequence |
|-------------------|---------------------------|
| No swap (`A[i] ≥ A[i+1]`) | `snd b` (pc 34) — ship the incoming, keep `a` |
| Swap (`A[i] < A[i+1]`) | `snd a; set a b` (pc 31-32) — ship the old left, carry new right |

So the Duet puzzle is solving a **bubble sort of the LCG output**, distributed across two programs.

### The two-speed structure of bubble sort

Bubble sort has a characteristic asymmetry that your trace makes visceral:

| Direction | Speed | Why |
|-----------|-------|-----|
| **Small values → right** (the "with-sweep" direction in descending bubble sort) | **Unbounded** — one pass can carry a value from position 0 to position N-1 | Each swap hands the value off one step right and the sweep keeps going |
| **Large values → left** (the "against-sweep" direction) | **Exactly one slot per pass** | A big value only gets "left behind" by one swap before the sweep moves on |

Consequence — the **tail-locking invariant**:

> **After pass `k` of descending bubble sort, the last `k` positions hold the `k` smallest values in ascending order (their final positions).**

Watch this in the 10-value progression table ([day18_sort_sim.py](../../analysis/day18_sort_sim.py)):

```
  step          0  1  2  3  4  5  6  7  8  9
  start         4  7  1  9  3  8  2  6  5 10
  p1 pass       7  4  9  3  8  2  6  5 10  1   ← col 9 locks to 1 (global min)
  p0 pass       7  9  4  8  3  6  5 10  2  1   ← col 8 locks to 2
  p1 pass       9  7  8  4  6  5 10  3  2  1   ← col 7 locks to 3
  p0 pass       9  8  7  6  5 10  4  3  2  1   ← cols 5-9 all locked
  …
  p1 pass      10  9  8  7  6  5  4  3  2  1   ← 10 finally crawls to col 0
```

- The **tail is locked in almost immediately** — most of the array already looks correct after a handful of passes.
- The **head takes forever** — the maximum value crawls leftward one slot per pass and dictates the termination time.

### The pass-count law

The total number of passes bubble sort needs is:

> **passes = maximum leftward displacement of any element** (+1 for the confirmation pass that observes `f = 0`)

Where *leftward displacement* = `original_position − target_position` for any element whose target is to its left in the sorted order.

Verified for both cases:

| Scenario | Max leftward displacement | Single-program pass count | Matches? |
|----------|---------------------------|---------------------------|----------|
| 10-value `[4,7,1,9,3,8,2,6,5,10]` | 9 (the `10` from pos 9 → pos 0) | 10 (9 + confirm) | ✓ |
| 127-value LCG with seed 680 | **110** (value `9370` from pos 118 → descending-rank 8) | **111** | ✓ |

Interestingly, the maximum **value** (`9936`) isn't the most displaced — it's at position 43 and only needs to travel 43 slots. The *straggler* that dictates convergence is the value that happens to be farthest from its sorted rank. Here, that's `9370` — the 9th-largest value, but stranded near the tail of the initial stream.

### Why 56 per program, not 111

Single-program convergence takes **111 passes**. The Duet answer `7112 / 127 = 56` is **half** of that (±1). The halving comes from the two-program structure:

- Each sort pass applied = one application of `f`.
- p1 and p0 alternate: `f` is applied to p1's inbox → result becomes p0's inbox → `f` applied again → back to p1 → …
- So `2k` total applications of `f` correspond to `k` passes per program.
- `111 total applications` → `~56 per program`. The two-program scheduler splits the work exactly in half.

This is why the puzzle author chose **two** cooperating programs — a single program doing the sort alone would also work (and would take 111 passes), but the duet structure advertises the coroutine mechanics `snd` and `rcv` were added for.

### Termination — and the asymmetric endgame at pc 21-22

When a program finishes a pass with `f = 0`, the `jgz f -16` falls through to `jgz a -19`. Since `a` is the global minimum of the LCG stream (= **95** for this input — verified in [day18_trace.rs](../../examples/day18_trace.rs)), `a > 0` always, so the jump fires and the program lands at pc 21 — the drain loop.

**Per-program hit counts at pc 21-22** (from the reachability probe):

| pc | p0 | p1 | interpretation |
|----|----|----|----------------|
| 21 | 1 attempt, blocked | 127 successful reads | |
| 22 | **0** (never executed) | 127 jumps back to pc 21 | |

The drain loop is **asymmetric**:

- **p1 finishes pass 56 first.** p0 has just shipped 127 values into p1's inbox during its own pass 56. p1 jumps to pc 21, reads all 127 via the `rcv b / jgz b -1` spin, then blocks on an empty inbox.
- **p0 finishes pass 56 second.** By the time p0 jumps to pc 21, p1 is already blocked and no longer sending. p0's first `rcv b` blocks immediately. pc 22 is never reached by p0.

So pc 22 is dead code for p0, and the drain loop in general is functional ceremony — it doesn't affect the answer. The send-count for p1 is already finalised at `56 × 127 = 7112` before either program reaches the drain.

The scheduler in [day18.rs](../../src/solver/day18.rs) then detects "both blocked + both inboxes empty" and returns.

---

## Accounting for the answer

Program 1 sends inside Block E only:

| Source | Count per pass |
|--------|----------------|
| `snd a` or `snd b` in the inner loop (pc 31 / 34) | 126 |
| Final `snd a` at pc 38 | 1 |
| **Total per pass** | **127** |

Program 1 runs Block E a total of **7112 / 127 = 56 times** before deadlock. That's the depth of the cooperative sort on this particular LCG sequence — the first pass that program 1 observes as already-sorted takes 56 full passes to materialise.

Program 0 does the same dance in parallel and would yield a similar (not necessarily identical) pass count; we don't need it for the answer.

---

## The LCG is glibc's `rand()` in disguise

Block C's two multiplications fold into a single LCG step:

```
p  ←  (8505 · p)              mod M31
p  ←  (129749 · p + 12345)    mod M31
```
```
⇒  p' = 129749 · (8505 · p) + 12345   mod M31
      = (8505 · 129749) · p + 12345   mod M31
      =   1103515245    · p + 12345   mod (2^31 - 1)
```

Multiplier **`1103515245`** and increment **`12345`** are the exact constants of **glibc's `rand()`** (see `stdlib/random_r.c`, `TYPE_0`). The only difference is modulus: glibc uses `2^31`, Duet uses `2^31 - 1` — the Mersenne prime Block B painstakingly constructed by doubling.

The author factored `1103515245 = 8505 × 129749` so that the huge multiplier — which won't fit as a single Duet literal comfortably without making the intent obvious — decomposes into two small `mul` instructions. Read in isolation, `mul p 8505` and `mul p 129749` look like random obfuscation; composed, they're a well-known RNG.

The trailing `mod b 10000` is classical LCG output scaling, compressing the 31-bit state into a 4-digit tail suitable for the sort loop's value comparisons.

So Block C isn't "some arbitrary arithmetic to produce 127 numbers" — it's **glibc `rand()` ported to Duet**, reseeded to 680, emitting 127 samples mod 10000. Deterministic, reproducible, and a small hat-tip to the C standard library smuggled into the assembly.

---

## Three pieces of author cleverness

### 1. `jgz p p` as a program-splitter

```
mul p 17
jgz p p
```

For **p0** (`p = 0`): the jump target is 0 (not greater than 0), so control falls through to the next line.

For **p1** (`p = 17`): the jump target is 17, which skips over exactly the 17 instructions of Blocks B + C + the early part of D.

The multiplier `17` isn't arbitrary — it's the exact distance from pc 4 (where p0 continues) to pc 20 (where p1 should land). The program is literally encoding "jump over this block" as data in the initial `p` value. Neat.

### 2. Reusing `p` as the LCG state

After the splitter, p0 no longer needs `p` to identify itself. Block C clobbers it with `set p 680` and uses it purely as scratch for the LCG. This is a nice reminder that `p` is just a register — the problem statement's "program ID" interpretation is a convention, not a hardware fact.

### 3. Factoring the glibc multiplier across two `mul`s

See the previous section — `1103515245 = 8505 × 129749` is not a coincidence. A single `mul p 1103515245` would have been a dead giveaway; splitting it keeps the easter egg hidden unless you actually work out the composition. Eric Wastl is a genius.

---

## Cross-references

- **Day 15 — Dueling Generators**: also uses an LCG with a Mersenne modulus (`2^31 - 1`). Day 18 reuses both the modulus and the "two generators feeding each other" motif — this time coordinated through `snd`/`rcv` instead of a fixed lockstep.
- **Day 23 — Coprocessor Conflagration** *(upcoming)*: also hides a real algorithm behind the Duet ISA. Day 23's hidden function is "count composite numbers in a range"; Day 18's is "cooperative distributed sort". Both exploit the same pattern — the solver is supposed to *read the assembly*, not just run it.
- **Day 12 / 14 / 17** patterns don't apply here; this is purely an interpreter-and-scheduler problem.
