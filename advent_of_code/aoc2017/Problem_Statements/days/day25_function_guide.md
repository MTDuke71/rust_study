# Day 25: The Halting Problem — Function Guide

**Problem**: Simulate a Turing machine described by a structured blueprint — start state, step count, and per-state rules of the form *"on reading 0 (or 1): write a bit, move L/R, transition to state X."* After the configured number of steps, return the *diagnostic checksum* — the count of `1`s on the tape.
**Answers**: Part 1 = **4,287** | Part 2 = *Merry Christmas!* (the traditional 50th-star freebie — Day 25 has no second puzzle)
**Code**: [day25.rs](../../src/solver/day25.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Mental Model: A Toy CPU](#mental-model-a-toy-cpu)
3. [Modelling Rules as a Flat Lookup Table](#modelling-rules-as-a-flat-lookup-table)
4. [The Tape: Vec<u8> with a Floating Centre](#the-tape-vecu8-with-a-floating-centre)
5. [The Hot Loop](#the-hot-loop)
6. [Parsing the Blueprint](#parsing-the-blueprint)
7. [Why a HashMap Tape Would Be Bad](#why-a-hashmap-tape-would-be-bad)
8. [Why Not Bit-Pack the Tape?](#why-not-bit-pack-the-tape)
9. [Benchmarks](#benchmarks)
10. [Key Patterns](#key-patterns)
11. [Integrator Notes](#integrator-notes)

---

## Problem Summary

The blueprint is two-section:

```
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  ...
```

The example runs 6 steps; the actual input runs **12,919,244** steps across 6 states (A–F). After the simulation, count the `1`s on the tape — that's the answer.

Day 25 traditionally has only one part — Part 2 unlocks once you've solved every other day in the year. With Days 1–24 complete, Part 2 is the celebration string and the 50th star.

---

## Mental Model: A Toy CPU

Strip the framing and a Turing machine is the smallest possible CPU:

| Turing-machine concept              | CPU analogue                                                |
|:------------------------------------|:------------------------------------------------------------|
| Tape                                | Memory (one bit per address, infinite both ways)            |
| Cursor                              | The address register / program-counter-into-data            |
| Current state                       | The current "microcode page" / instruction decoder mode     |
| Rule for state S, value V           | The instruction selected by `(state, value)`                |
| Write bit                           | Store to `mem[cursor]`                                      |
| Move L/R                            | Increment/decrement the address register                    |
| Continue with state X               | Jump to a different microcode page                          |

Every step does the same four things, no exceptions: **read** `tape[pos]`, **look up** `rules[state][value]`, **write** the rule's bit, **update** `pos += delta` and `state = next`. There are no halts, no errors, no branches besides the implicit `value`-indexed rule selection. That uniformity is what lets the inner loop compile down to a handful of integer ops per step.

The *"Halting Problem"* in the title is a joke — the machine is given a step budget up front, so halting is trivial. It's a tongue-in-cheek nod to Turing's actual undecidability result.

---

## Modelling Rules as a Flat Lookup Table

Each `(state, value)` pair maps to one rule, which is three fields: write bit, move direction, next state. With six states and two values, the entire program is **12 rules**.

```rust
#[derive(Debug, Clone, Copy)]
struct Rule {
    write: u8,      // 0 or 1
    delta: i8,      // -1 (left) or +1 (right)
    next: u8,       // 0..6, A=0, B=1, ...
}

#[derive(Debug, Clone)]
struct Machine {
    /// `rules[state][value]` is the rule fired when in `state` reading `value`.
    rules: Vec<[Rule; 2]>,
    start: u8,
    steps: u64,
}
```

`Vec<[Rule; 2]>` over `Vec<Vec<Rule>>` matters a little: the inner `[Rule; 2]` is a fixed-size array stored inline in the outer `Vec`'s backing buffer, so a step's rule lookup is **two indexed loads from contiguous memory** — `rules.as_ptr().add(state).cast::<[Rule; 2]>().add(value)`. With `Rule` being `Copy` and 3 bytes wide, the whole rule table fits in L1 trivially.

`u8` for state IDs and `i8` for the move delta are deliberate — they let `Rule` pack to the natural alignment of its widest field (1 byte) and keep the table small. The codebase wraps the conversions: `state_index('A') -> 0`, `state_index('B') -> 1`, etc., via a single subtraction.

---

## The Tape: `Vec<u8>` with a Floating Centre

The tape is conceptually infinite in both directions, but in practice the cursor's reachable footprint after `N` steps is bounded by `±N`. For 12.9M steps that's 25.8M cells of headroom; the *actual* footprint is much smaller (the cursor wanders, but it doesn't escape to infinity). So the right move is to allocate a moderate buffer up front and grow on demand.

```rust
let mut tape: Vec<u8> = vec![0u8; 1 << 16];   // 64 KiB starter
let mut offset: isize = 1 << 15;              // logical 0 sits in the middle
let mut pos: isize = 0;
```

**The mapping:** logical position `pos` lives at buffer index `pos + offset`. Logical `0` starts at buffer index `32768`; the cursor can drift `32 768` cells in either direction before we run out of room.

**The grow-and-recentre move** when the cursor steps out:

```rust
if idx < 0 || (idx as usize) >= tape.len() {
    let old_len = tape.len();
    let new_len = old_len * 2;
    let extra = old_len / 2;                  // half a buffer of pad on each side
    let mut new_tape = vec![0u8; new_len];
    new_tape[extra..extra + old_len].copy_from_slice(&tape);
    tape = new_tape;
    offset += extra as isize;                 // shift origin so logical 0 still maps right
    idx = pos + offset;                       // recompute
}
```

The trick: doubling the buffer and copying the old contents to the *middle* of the new one means **logical positions stay stable** — we just shift `offset` forward by the amount of leading pad we added. No remapping of the cursor, no rescan, no re-write.

Doubling makes growth amortised O(1) per step. For this input, the cursor drifts a few thousand cells either way, the initial 64 KiB is plenty, and growth never fires in practice.

---

## The Hot Loop

```rust
for _ in 0..m.steps {
    let mut idx = pos + offset;
    if idx < 0 || (idx as usize) >= tape.len() {
        // grow + recentre (cold path)
    }
    let i = idx as usize;
    let val = tape[i] as usize;
    let rule = &m.rules[state][val];
    tape[i] = rule.write;
    pos += rule.delta as isize;
    state = rule.next as usize;
}
```

A single step is:

1. Compute buffer index from `pos + offset` (one add).
2. Bounds-check (one compare in the common case — predicted false).
3. Load the bit at the cursor.
4. Index into `rules[state][val]` (two contiguous loads).
5. Store the rule's `write` bit at the cursor.
6. Update `pos` and `state`.

That's ~6 operations per step on the hot path, no allocations, no branching beyond the bounds check that's always predicted in-bounds. At 2.9 ns/step measured (37 ms / 12.9 M steps), the simulator is bottlenecked by L1 latency on the rule and tape loads, which is about as good as you can do without SIMD or vectorising the trace itself.

The final checksum is `tape.iter().map(|&b| b as u32).sum()` — one linear pass over the buffer at the end. Cheap relative to the simulation.

---

## Parsing the Blueprint

The blueprint is regular enough to parse by **block-and-offset** rather than reaching for `regex` or `nom`. Blocks are separated by blank lines (`\n\n`), and within each state block the four lines per rule sit at fixed offsets:

```
Line 0:   In state X:
Line 1:     If the current value is 0:
Line 2:       - Write the value V.        ← write bit at last char before '.'
Line 3:       - Move one slot to the L/R.  ← contains "right"?  +1 : -1
Line 4:       - Continue with state Y.     ← next state at last char before '.'
Line 5:     If the current value is 1:
Line 6-8: same shape as 2-4
```

The parser is a closure that takes the start-line offset (1 for the value-0 rule, 5 for the value-1 rule) and pulls write/delta/next out by indexing `lines[start+1..start+4]`. Trim the trailing `'.'`, take the last character, parse it as a digit (write/value) or convert to an index (next state). The header block is parsed separately to extract the start state and step count.

**Why not regex?** The format is rigid, the parser is ~30 lines, and the `regex` crate adds 200KB of compile-time and a runtime engine to do what `.contains("right")` does in one comparison.

**Why first-pass collect state names?** State letters in the blueprint appear in declaration order (A, B, C, ...) but we don't *rely* on that — `state_index('A')` always returns `0`, `state_index('B')` returns `1`, etc., so we can write the rule for state `B` into `rules[1]` regardless of which order the blocks appear. The Vec is pre-sized to `state_blocks.len()` and rules are indexed-assigned, not pushed.

---

## Why a HashMap Tape Would Be Bad

The "obvious" data structure for an infinite tape is a `HashMap<i64, u8>`:

```rust
let mut tape: HashMap<i64, u8> = HashMap::new();
let val = *tape.get(&pos).unwrap_or(&0);
tape.insert(pos, rule.write);
```

It works, but every step is now:

- Hash of `i64` (~10 ns)
- Probe into the bucket (~5–20 ns depending on load factor)
- Likely an L2/L3 miss because the entries aren't spatially adjacent

Across 12.9M steps that's at least 130 ms of hash overhead alone — **3–4× slower** than the `Vec<u8>` approach in this implementation, before counting the cache misses. And we'd still need to iterate every entry at the end to count `1`s.

The `Vec<u8>`-with-offset structure is faster because:

- Adjacent positions are adjacent in memory → cursor movement walks a cache line.
- Bounds-checked indexing compiles to two integer ops, not a hash.
- Initial zeros are free (the buffer was zeroed at `vec![0; N]`), so a "blank tape" is not paid for per cell.

---

## Why Not Bit-Pack the Tape?

A bit-packed tape (`Vec<u64>` with one bit per cell) is 8× smaller and lets you AND / OR / popcount whole words for the final checksum. For 12.9M steps with a footprint in the low thousands of cells, the savings are real but marginal — the working set already fits comfortably in L1.

The hot-loop cost would *increase* slightly, because every step has to:

1. Compute `(idx >> 6, idx & 63)` to find word + bit.
2. Read the word, mask the bit, branch on it.
3. Write back the modified word (read-modify-write).

Step 3 is the killer: the simple byte tape does a single store, the bit tape does a load-mask-store. For a 12.9M-step machine on a pre-warmed L1, the byte version wins on instructions-retired and tied or wins on cycles-per-step.

Where bit-packing *would* help: a machine that runs 100B+ steps and grows the tape to gigabytes. At that scale the cache-miss savings from 8× density flip the trade-off. Not this puzzle.

---

## Benchmarks

Measured with the `--bench=20` harness on Windows (release build):

| Target           | Time         | Per-step |
|------------------|--------------|----------|
| `day25` combined | **37.53 ms** | ~2.9 ns  |

Observations:

- **Single-pass simulation.** There's no Part 2 search, no second loop — the entire runtime is the 12.9M step trace plus a tiny final pop-count pass.
- **Tape never grows** on this input. The initial 64 KiB buffer is enough for the cursor's actual excursion, so the cold-path recentre code is never hit.
- **2.9 ns/step** is L1-bound. The next optimisations (vectorising the trace, JIT-compiling the rules, recognising cycles in the (state, local-tape-window) pair) cost more engineering than the ~37 ms is worth saving.

For comparison, [Day 5's twisty trampolines](day05_function_guide.md) runs 27.7M jump-list steps in 34 ms — same order of magnitude, both bottlenecked by the same "one indexed load + one indexed store per step" rate. The Turing machine is the same shape of work as a 4-instruction VM.

---

## Key Patterns

### Vec-with-offset for "logically infinite, practically bounded" arrays

The pattern — a `Vec<T>` plus an integer offset, with grow-and-recentre on overflow — is the right call any time you have a 1D coordinate space that's symmetric around 0 and the working set is bounded by something like O(steps). It's faster than a `HashMap<i64, T>` on every dimension that matters: cache-friendliness, lookup cost, iteration cost, default-zero cost.

The same pattern shows up in [Day 22's Part 2](day22_function_guide.md) (10M-burst Sporifica with a flat 1024² grid) and in any random-walk-with-bounded-excursion problem. Once you spot "infinite tape but the cursor doesn't escape," default to this shape.

### Flat 2D rule table over `HashMap<(State, Value), Rule>`

`rules: Vec<[Rule; 2]>` indexed as `rules[state][value]` is faster than the equivalent HashMap and almost as readable. It works any time the keying domain is small and dense — finite states × finite values, finite ops × finite operand types, etc.

The same shape appears in [Day 18 / 23 assembly interpreters](day18_function_guide.md) where the opcode-to-handler mapping is a small enum dispatch rather than a hash lookup. The principle is *"if your keys are 0..K for small K, the array is the hash table."*

### Block-and-offset parsing for rigid formats

When the input format is regular enough that line N within a block always means the same thing, skip the parser-combinator library and just `lines.collect::<Vec<_>>()` then index. It's a rare format that's both rigid enough for this and complex enough to make the savings noticeable, but Day 25's blueprint is exactly that shape.

For one-off puzzle inputs, this approach is *more* maintainable than regex — the line numbers are visible in the code, and a format change just means renumbering the offsets.

### Single-pass simulation, single-pass count

`run_machine` produces the tape; `tape.iter().sum()` produces the answer. There's no fused "simulate-and-count" optimisation because counting `1`s mid-simulation would add a branch per step and the final pass is `O(buffer_size)`, which is much smaller than `O(steps)`.

The principle: **post-process at the end** when the post-pass is fundamentally cheaper than the work that produced the data. The same shape appears in [Day 14 (Mission 10 Union-Find)](day14_function_guide.md) — fill the grid, then count regions in one final pass.

---

## Integrator Notes

- **No mission reuse.** A 1D tape with a moving cursor doesn't fit Mission 6 (`Grid<T>` is 2D), Mission 8 (`Graph` traversals don't have a tape), or Mission 10 (`UnionFind` doesn't apply). The closest match is the *spirit* of Mission 6 — "structured access to a contiguous backing buffer" — but the offset-tracking and dynamic-growth needs are too tape-specific to refactor through a generic Grid. Inline `Vec<u8>` with `isize` offset is 30 lines and exactly fit-to-purpose.
- **AUTOSAR analogue.** Each state is a runnable; each rule is a runnable's exit point selecting the next runnable based on a single input. The "tape" is the shared dataspace the runnables read and write. The fixed step budget is the watchdog. The whole machine is essentially a tiny RTE schedule with one runnable active at a time, dispatching by mode.
- **Cross-link to AoC 2017 Day 18 / 23.** All three days are *"interpret a small instruction set"* puzzles. Day 18 has assembunny-style ops over registers (and a coroutine trick in Part 2); Day 23 reads its own bytecode to recognise a primality loop; Day 25 has a 12-rule state machine over a tape. The shared technique is **flat dispatch table over (mode, input)** — instructions for 18/23, `(state, value)` for 25.
- **Project Euler crossover.** Cellular automata, Wolfram-style 1D rules, and bit-vector simulations are recurring PE shapes. The Vec-with-offset tape generalises directly to those — replace the rule lookup with the cell update function, keep the buffer-growth pattern. PE's "long-running 1D simulations with bounded footprint" problems are this exact shape.
- **The 50th star.** Completing Day 25 of any AoC year requires having all 49 prior stars in that year. With Days 1–24 of 2017 done, this finishes the year. The traditional Day 25 Part 2 *"Send a free copy of yourself by mail"* is just a button-click on the AoC site — it's a celebration of finishing, not a puzzle.

---

**Navigation**: [← Day 24](day24_function_guide.md) | [All Days](../summary_2017.md) | *(end of year)*
