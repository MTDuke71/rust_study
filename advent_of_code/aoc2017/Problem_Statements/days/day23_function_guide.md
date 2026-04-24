# Day 23: Coprocessor Conflagration — Function Guide

**Problem**: A reduced version of the Day 18 VM (8 registers `a..h`, all starting at 0) executes a program using only four opcodes: `set`, `sub`, `mul`, `jnz`. Part 1 runs the program in "debug mode" (`a = 0`) and reports how many times `mul` fires. Part 2 flips the debug switch (`a = 1`) and asks for the final value of register `h` — which, naively simulated, would take hours.
**Answers**: Part 1 = **8,281**, Part 2 = **911**
**Code**: [day23.rs](../../src/solver/day23.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [The ISA](#the-isa)
3. [Part 1: Straight Simulation](#part-1-straight-simulation)
4. [Part 2: Reading the Program](#part-2-reading-the-program)
5. [Extracting Constants from the Parsed Program](#extracting-constants-from-the-parsed-program)
6. [Primality by Trial Division](#primality-by-trial-division)
7. [Benchmarks](#benchmarks)
8. [Why Not Just Simulate With a=1](#why-not-just-simulate-with-a1)
9. [Key Patterns](#key-patterns)
10. [Integrator Notes](#integrator-notes)

---

## Problem Summary

The puzzle ships a 32-line program on a tiny VM. Register `a` is a *mode switch*:

- **Part 1** (`a = 0`): execute the program and count `mul` invocations. The setup block is bypassed entirely; `b` and `c` both end up at 93, the outer loop runs exactly once, and the `d/e` double-loop does 91 × 91 = 8281 multiplications on that single value.
- **Part 2** (`a = 1`): execute the program to completion and read `h`. The setup block now runs, blowing `b` up to 109,300 and `c` to 126,300. The outer loop then walks `b` to `c` in steps of 17 — 1,001 outer iterations — and the `d/e` double-loop becomes an O(b²) check that `b` has no factors `d*e` in `[2, b)`. That's **over a trillion multiplications** the naïve way.

The puzzle all but hands you the hint: "Technically, if it had that... it wouldn't even need to run the program." The whole point of Part 2 is to *read the bytecode*, figure out what it computes, and do that thing directly.

---

## The ISA

Four opcodes, eight registers, signed 64-bit integers:

| Opcode  | Semantics                                                        |
|---------|------------------------------------------------------------------|
| `set X Y` | X ← Y (X is always a register; Y is a register or literal)     |
| `sub X Y` | X ← X − Y                                                      |
| `mul X Y` | X ← X × Y                                                      |
| `jnz X Y` | if X ≠ 0, pc ← pc + Y; else pc ← pc + 1                        |

Notably missing (vs Day 18): `snd`, `rcv`, `add`, `mod`, `jgz`. `sub` with a *negative* literal is how the program adds — the whole setup section reads `sub b -100000`, `sub c -17000`, `sub b -17`, etc. That quirk matters when extracting constants (see below).

```rust
#[derive(Debug, Clone, Copy)]
enum Value { Reg(usize), Num(i64) }

#[derive(Debug, Clone, Copy)]
enum Instr {
    Set(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Jnz(Value, Value),
}
```

Registers live in a fixed `[i64; 8]`, indexed by `b'a'..=b'h'` offsets. The VM loop is ~30 lines of Rust: one big `match` on the current instruction, `pc: i64` so `jnz -13` doesn't panic on cast, bounds-check on `pc as usize < instrs.len()` to terminate.

---

## Part 1: Straight Simulation

Run the VM in debug mode (`a = 0`) and count `mul` instructions:

```rust
fn solve_part1_with_data(instrs: &[Instr]) -> usize {
    let mut regs = [0i64; 8];
    let mut pc: i64 = 0;
    let mut mul_count = 0;
    while pc >= 0 && (pc as usize) < instrs.len() {
        match instrs[pc as usize] {
            Instr::Set(r, v) => { regs[r] = val(v, &regs); pc += 1; }
            Instr::Sub(r, v) => { regs[r] -= val(v, &regs); pc += 1; }
            Instr::Mul(r, v) => { regs[r] *= val(v, &regs); mul_count += 1; pc += 1; }
            Instr::Jnz(x, y) => {
                if val(x, &regs) != 0 { pc += val(y, &regs); } else { pc += 1; }
            }
        }
    }
    mul_count
}
```

What actually runs in debug mode:

1. `set b 93; set c b` → `b = c = 93`
2. `jnz a 2` — `a = 0`, no jump, fall through to `jnz 1 5` which unconditionally jumps to the loop (skipping `mul b 100 … sub c -17000`).
3. The outer loop starts at line 9 with `b = c = 93`. The outer-loop exit test compares `b` to `c`; since they're equal, the loop runs exactly once before terminating.
4. That single pass does a full `d ∈ [2, 93), e ∈ [2, 93)` double-loop, executing `mul g e` every inner iteration: **91 × 91 = 8281 multiplications**.

Closed-form check: `(93 − 2)² = 8281`. Matches the answer exactly, no accident.

---

## Part 2: Reading the Program

Once `a = 1`, line 3's `jnz a 2` skips the unconditional `jnz 1 5` and instead executes the setup block. Let me annotate what that block computes:

```
 4:  mul b 100          ; b = 93 * 100           = 9300
 5:  sub b -100000      ; b = 9300 + 100000      = 109300   (b_start)
 6:  set c b            ; c = 109300
 7:  sub c -17000       ; c = 109300 + 17000     = 126300   (b_end)
```

After the setup, the outer loop at lines 8–24 does:

```
 8: set f 1
 9: set d 2
10: set e 2
11: set g d
12: mul g e
13: sub g b                ; g = d*e − b
14: jnz g 2                ; skip next if d*e ≠ b
15: set f 0                ; d*e == b: b is composite
16: sub e -1               ; e++
17: set g e
18: sub g b
19: jnz g -8               ; loop e while e ≠ b
20: sub d -1               ; d++
21: set g d
22: sub g b
23: jnz g -13              ; loop d while d ≠ b
24: jnz f 2                ; if prime (f still 1), skip
25: sub h -1               ; h++ (b is composite)
26: set g b
27: sub g c                ; g = b − c
28: jnz g 2                ; if b ≠ c, skip the exit
29: jnz 1 3                ; unconditional exit jump
30: sub b -17              ; b += 17
31: jnz 1 -23              ; loop back to line 8
```

Stripped to its intent:

```python
for b in range(109300, 126300 + 1, 17):   # 1001 values
    if any(d*e == b for d in range(2, b) for e in range(2, b)):
        h += 1
```

The `∃ d, e ∈ [2, b): d*e = b` is exactly a naïve primality test — with the horrifying complexity of O(b²) per `b`. The answer is the number of **composite** values in the arithmetic progression `{109300, 109317, 109334, …, 126300}`.

Replace the O(b²) inner check with real primality and we're done.

---

## Extracting Constants from the Parsed Program

Hardcoding `109300, 126300, 17` would technically work for *my* input, but the whole point of the VM-abstraction is that the problem is the same for every user's input — only the constants differ. So the solver reads them from the parsed instructions rather than the puzzle text:

```rust
fn extract_bounds(instrs: &[Instr]) -> (i64, i64, i64) {
    // Expected layout:
    //   0: set b <seed>
    //   4: mul b <mult>
    //   5: sub b -<add>       (negative literal → adds |add|)
    //   7: sub c -<range>
    //  30: sub b -<step>
    let seed  = match instrs[0]  { Instr::Set(_, Value::Num(n)) => n, _ => panic!(/* ... */) };
    let mult  = match instrs[4]  { Instr::Mul(_, Value::Num(n)) => n, _ => panic!(/* ... */) };
    let add   = match instrs[5]  { Instr::Sub(_, Value::Num(n)) => -n, _ => panic!(/* ... */) };
    let range = match instrs[7]  { Instr::Sub(_, Value::Num(n)) => -n, _ => panic!(/* ... */) };
    let step  = match instrs[30] { Instr::Sub(_, Value::Num(n)) => -n, _ => panic!(/* ... */) };
    let b_start = seed * mult + add;
    let b_end   = b_start + range;
    (b_start, b_end, step)
}
```

The sign flip (`-n`) on the `sub` literals is the ISA's "add with negative" trick showing up in the reader. A unit test (`test_bounds_match_setup_simulation`) cross-checks `extract_bounds` against *actually simulating* the first 8 instructions with `a = 1`, so if a future input were laid out slightly differently the test would fire instead of silently returning a wrong answer.

Part 2 then becomes three lines:

```rust
fn solve_part2_with_data(instrs: &[Instr]) -> i64 {
    let (b_start, b_end, step) = extract_bounds(instrs);
    (b_start..=b_end).step_by(step as usize)
                     .filter(|&n| !is_prime(n))
                     .count() as i64
}
```

(Written as a loop in the actual source to keep `step` typed as `i64` without the cast dance, but same thing.)

---

## Primality by Trial Division

1,001 values, each ≤ 126,300. `√126300 ≈ 356`. Trial division up to √n is fine — no sieve needed:

```rust
fn is_prime(n: i64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}
```

Could be ~2× faster with a sieve, ~5× faster with a wheel skipping multiples of 2/3/5, but the whole Part 2 takes **26 µs** already — no optimisation needed.

**Sanity check on the answer**: 1001 values, 911 composites → 90 primes. The prime-counting density around n = 118,000 is 1/ln(n) ≈ 1/11.68 ≈ 0.0856; over 1001 values that's ≈ 86 expected primes. We see 90, which is a +0.5σ variance. The AP spacing of 17 is coprime to every small prime so it doesn't skew the density much. 911 looks right.

---

## Benchmarks

Measured with Criterion (Windows release):

| Target           | Time        |
|------------------|-------------|
| `day23_part1`    | **89.41 µs** |
| `day23_part2`    | **26.07 µs** |
| `day23` combined | **116.07 µs** |

A few observations:

- **Part 2 is faster than Part 1.** Part 1 runs the full VM simulation for 8,281 muls; Part 2 just does 1,001 trial-division primality tests on values ≤ 126,300. The primality test amortises to ~25 ns per value.
- **Parse-once holds.** Combined (116 µs) ≈ 89 µs + 26 µs = 115 µs + one parse. Each individual benchmark re-parses, so the individual sum slightly over-counts; `solve()` parses once.
- **Naïve Part 2 vs analytical.** Simulating the VM with `a = 1` would do roughly Σ_{b ∈ AP} (b − 2)² ≈ 1001 × (118,000 − 2)² ≈ **1.4 × 10¹³** operations. Even at 1 ns per operation that's ~4 hours. The analytical solve is ~5 × 10⁸× faster.

---

## Why Not Just Simulate With `a=1`

Three reasons to go analytical instead of patching/JIT'ing the VM:

1. **Runtime.** A trillion inner-loop iterations wouldn't finish during the visible universe of an AoC session. Even the tightest hand-written VM (native codegen, no match, inlined register access) would still need hours.
2. **The puzzle is telling you to read the bytecode.** "Technically, if it had that, it wouldn't even need to run the program" is the author's polite hint that the *input program* is the real problem, not the ISA. That matches the feedback memory's guidance on these bytecode puzzles: dig into the hidden program, not just the ask.
3. **The transformation teaches the underlying structure.** Recognising that the `d/e` double-loop is a primality test by trial multiplication — and that the outer loop is "count composites in an AP" — is both the whole point and a generalisable pattern (this is the same `is_prime` structure Project Euler problems lean on in a few weeks).

There's an alternate path worth naming: write a peephole optimiser that rewrites the inner double-loop as a primality check and then runs the *rest* of the VM. That'd be more general (works on any Day 23–style input) but more engineering than the puzzle warrants for a 5 × 10⁸× speedup the reader can get by hand.

---

## Key Patterns

### The VM is a red herring; the *program* is the problem

AoC has a class of puzzles where Part 2 asks for something the given VM cannot practically compute — Day 23 here, and [2015 Day 23](https://adventofcode.com/2015/day/23), [2019 Day 7/9](https://adventofcode.com/2019/day/9) (Intcode), etc. in various forms. The ISA is a delivery mechanism for a program that encodes a mathematical routine. The solve is: read the program, recognise the routine, compute it directly.

This is exactly the trap pinned in the `feedback_aoc_bytecode_puzzles` memory: *"dig into the hidden program, not just the ask."* Build the simulator for Part 1, then open the assembly file.

### Extract constants from the AST, not the puzzle text

Reading `109300, 126300, 17` from the prose would work, but then the code is bound to one input. Reading them from the parsed `Instr::Set(_, Value::Num(n))` pattern makes the solution portable — every user's input has the same program shape with different constants, and `extract_bounds` handles them all. The unit test that cross-checks `extract_bounds` vs a direct simulation of the setup section gives confidence the layout assumption is still valid.

### `sub X -N` is `add X N` in disguise

When an ISA doesn't have `add`, the program subtracts a negated literal. Readers need to flip the sign when extracting numeric constants — `-n` in the match arm. A subtle enough detail that getting it wrong here would silently return `b_start = 93*100 - 100000 = -90700`, the primality test would think every value is negative (composite? prime? depends on `is_prime`'s handling of negatives), and the answer would be garbage. The `test_bounds_match_setup_simulation` test fires on this class of mistake.

### Primality test complexity: O(√n), not O(n)

The `while i*i <= n` loop is the textbook trial-division template. It's worth writing from muscle memory because every Euler-style puzzle has one. No sieve needed below ~10⁷ values.

---

## Integrator Notes

- **No mission reuse.** This problem needs a tiny custom VM and a `is_prime` helper. Mission components (Grid, Graph, UnionFind) don't apply. The closest analogue in the workspace is Day 18's interpreter — same ISA family but different opcodes and different problem shape. A shared `advent_of_code/shared/vm/` module would be possible if a third VM puzzle appeared, but two-of-a-kind isn't enough to justify the abstraction yet (the rule-of-three heuristic from *Rust for Rustaceans* Ch3.3).
- **AUTOSAR analogue.** The `a` register as a mode switch is exactly how AUTOSAR DEM/DEM-sub-modules expose "debug vs production" behaviour — same value, different code paths. Reading the bytecode to skip the slow path is analogous to replacing a runtime service with a closed-form computation via the RTE.
- **Project Euler prerequisite.** This is the first primality test in the AoC 2017 track. Project Euler lands on primality/GCD/modular-arithmetic puzzles constantly, so the trial-division template from here will appear in almost every early PE problem. See [math-foundations/primality-fundamentals](../../../../zettelkasten/math-foundations/primality-fundamentals.md) for the full reference: trial division with the √n bound, 6k±1 candidates, Fermat's little theorem, and Miller–Rabin witness sets for `u64`-range inputs.
- **When the domain is larger.** For primality testing below ~10⁹ trial division still works; above that, switch to Miller–Rabin (deterministic for 64-bit with a known witness set). For *counting* primes in a range, Meissel–Mertens or a Sieve of Eratosthenes. None needed here.

---

**Navigation**: [← Day 22](day22_function_guide.md) | [All Days](../summary_2017.md) | Day 24 →
