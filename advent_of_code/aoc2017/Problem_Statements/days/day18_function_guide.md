# Day 18: Duet — Function Guide

**Problem**: Interpret a small assembly language twice — once as a "sound card" with a single program recovering the last frequency, once as a pair of concurrently-running programs exchanging values via FIFO queues.
**Answers**: Part 1 = **3188**, Part 2 = **7112**
**Code**: [day18.rs](../../src/solver/day18.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Model](#data-model)
3. [parse_input](#parse_input)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Deadlock Detection](#deadlock-detection)
7. [Benchmarks](#benchmarks)
8. [Key Patterns](#key-patterns)

---

## Problem Summary

The assembly has 7 instructions operating on 26 integer registers `a..z` (all default `0`):

| Op | Meaning |
|----|---------|
| `snd X` | Part 1: play sound of frequency X. Part 2: send X to peer's queue. |
| `set X Y` | `X ← Y` (X is a register) |
| `add X Y` | `X ← X + Y` |
| `mul X Y` | `X ← X * Y` |
| `mod X Y` | `X ← X % Y` |
| `rcv X` | Part 1: if X ≠ 0, return most-recent played sound. Part 2: pop from own queue into X; block if empty. |
| `jgz X Y` | If X > 0, `pc ← pc + Y`; else `pc ← pc + 1`. |

Y (and X in `snd`/`jgz`) is a **value** — either a register name or an integer literal.

- **Part 1**: run a single program until the first `rcv X` with X ≠ 0 fires; report the last-played frequency.
- **Part 2**: run **two** programs concurrently with distinct register banks (their own `p` starts at `0` and `1` respectively). Each `snd` pushes to the **other** program's queue; each `rcv` pops from **its own** queue and blocks when empty. Report how many times program 1 sends.

---

## Data Model

```rust
enum Value { Reg(usize), Num(i64) }

enum Instr {
    Snd(Value),
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Rcv(usize),
    Jgz(Value, Value),
}
```

Registers live in a `[i64; 26]` array indexed `a = 0 .. z = 25`. Values are resolved with a tiny helper:

```rust
fn val(v: Value, regs: &[i64; 26]) -> i64 {
    match v { Value::Reg(r) => regs[r], Value::Num(n) => n }
}
```

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<Instr>
```

Line-by-line split, match on the opcode, parse each operand as either a single lowercase-letter register or an `i64` literal. `rcv` and the first operand of register-writing ops are parsed as `usize` register indices directly — matching real inputs where those slots are always registers.

No regex, no allocation beyond the `Vec<Instr>`. ~41 lines of input → 41 `Instr` values.

---

## `solve_part1_with_data`

```rust
fn solve_part1_with_data(instrs: &[Instr]) -> i64
```

Classic tight interpreter loop:

```rust
let mut regs = [0i64; 26];
let mut last_sound = 0;
let mut pc = 0i64;
while pc >= 0 && (pc as usize) < instrs.len() {
    match instrs[pc as usize] {
        Instr::Snd(v)   => { last_sound = val(v, &regs); pc += 1; }
        Instr::Set(r,v) => { regs[r]  = val(v, &regs); pc += 1; }
        Instr::Add(r,v) => { regs[r] += val(v, &regs); pc += 1; }
        Instr::Mul(r,v) => { regs[r] *= val(v, &regs); pc += 1; }
        Instr::Mod(r,v) => { regs[r] %= val(v, &regs); pc += 1; }
        Instr::Rcv(r)   => if regs[r] != 0 { return last_sound } else { pc += 1 },
        Instr::Jgz(x,y) => if val(x,&regs) > 0 { pc += val(y,&regs) } else { pc += 1 },
    }
}
last_sound
```

`pc` is `i64` so `jgz` with negative offsets is well-defined; the bounds check `pc >= 0 && (pc as usize) < len` terminates on a fall-off in either direction.

---

## `solve_part2_with_data`

```rust
fn solve_part2_with_data(instrs: &[Instr]) -> usize
```

Two `Program` structs, two `VecDeque<i64>` mailboxes, one scheduler loop:

```rust
struct Program { regs: [i64; 26], pc: i64, send_count: usize }

// q0 holds messages destined for p0; q1 for p1.
loop {
    let r0 = step(&mut p0, instrs, &mut q0, &mut q1); // p0 reads from q0, writes to q1
    let r1 = step(&mut p1, instrs, &mut q1, &mut q0); // p1 reads from q1, writes to q0
    if both_done(r0, r1, &q0, &q1) { return p1.send_count; }
}
```

`step` returns one of three outcomes:

- `Ok` — instruction executed, `pc` advanced.
- `Blocked` — `Rcv` hit an empty queue; `pc` stays put so the program retries next round.
- `Terminated` — `pc` out of bounds.

**Key scheduling fact**: `snd` is non-blocking and `rcv` only blocks on an empty queue. So we can round-robin fairly: one step per program per tick. If p0 sends on this tick, p1 will observe it on its next step — strict alternation is enough, we don't need a work-stealing scheduler.

---

## Deadlock Detection

"Both programs stuck" isn't enough — the moment p0 steps and pushes to q1, p1 (who was blocked on an empty q1) now has work even though its own `step` hasn't fired yet. So we check both flags **and** the mailboxes:

```rust
let p0_stuck = r0 == Terminated || q0.is_empty(); // p0 has nothing incoming
let p1_stuck = r1 == Terminated || q1.is_empty(); // p1 has nothing incoming
if p0_done && p1_done && p0_stuck && p1_stuck { return p1.send_count; }
```

Interpretation: a blocked program that still has a message in its inbox will unblock next round, so it isn't actually stuck. Only when neither program can terminate **and** neither has a pending message do we stop.

This is the classic coroutine-deadlock rule — the same pattern Go channels enforce at runtime.

---

## Benchmarks

| Stage | Time | Ratio |
|-------|------|-------|
| Part 1 alone | **3.05 µs** | 1× |
| Part 2 alone | **406.58 µs** | 133× |
| Combined `solve()` | **400.13 µs** | (Part 2 dominates; parse is ~µs) |

- Part 1 executes only Block B + C + the `jgz a 3` jump + one `rcv a` — about 1,300 interpreter dispatches total.
- Part 2 runs the same init, then **56 sort passes × 127 values × 2 programs** ≈ 140,000 dispatches, plus `VecDeque` push/pop overhead per `snd`/`rcv`.
- 133× ratio aligns with the ~100× instruction-count ratio plus queue-operation overhead.
- No allocation in the hot loop apart from `VecDeque` growth on sends. Register file is a stack-allocated `[i64; 26]`.

---

## Key Patterns

### Shared instruction decoding
Two very different execution semantics (sound-card vs duet) share **the same parser and the same `Instr` enum**. The opcode decode is done once; only the interpreter loop differs. This is the natural shape for AoC "same language, two behaviors" problems and appears again in 2017 Day 23 and 2018 Day 19.

### Coroutine scheduling without threads
`Program` + `VecDeque` + round-robin `step()` is a cooperative coroutine scheduler in ~30 lines. No channels, no `std::thread`, no async runtime — just a loop that drives two state machines. Deterministic, testable, ~1000× faster than spinning up OS threads for a short program.

### `pc` as `i64` for negative jumps
A common bug: using `usize` for the program counter and then wrapping on `jgz X -N`. Keeping `pc: i64` and bounds-checking both ends (`pc >= 0 && (pc as usize) < len`) handles termination on either side cleanly.

### Deadlock ≠ both blocked
The trap: if you declare deadlock the first time both `step` calls return `Blocked`, you'll terminate too early when one program just queued work for the other. The correct predicate is "both stuck **and** both inboxes empty" — the same invariant real message-passing runtimes use.

---

## Integrator Notes

- **Mission reuse opportunity passed**: Mission 5 provides HashMap utilities but registers are better served by a fixed `[i64; 26]` — indexing on `(byte - b'a') as usize` is faster and communicates intent. No mission was a natural fit here; the interpreter is the reusable component for future days (2017 Day 23, 2018 Day 19).
- **AUTOSAR analogue**: The two-program scheduler is like two runnables sharing IPC queues under a single-core RTE — strict alternation, cooperative yielding via `rcv` blocking.
