# Day 12: Leonardo's Monorail — Function Guide

**Problem**: Simulate a simple 4-register "assembunny" computer to find the
value of register `a` after the program halts.

**Part 1**: All registers start at 0, what's in `a`? → **318,117**
**Part 2**: Register `c` starts at 1, what's in `a`? → **9,227,771**

---

## Performance

| Metric | Value |
|--------|-------|
| Combined | 46.17ms |
| Part 1 | 1.57ms |
| Part 2 | 44.53ms |
| Parse | negligible |

Part 2 dominates: `c=1` activates a branch that adds 7 to `d`, making the
outer Fibonacci loop run 33 iterations instead of 26.

---

## Architecture

```
Input (23 instructions)
    │
    ▼
parse(input)              ──→ Vec<Instruction>
    │                          Line-by-line split, token matching
    │
    ├──→ Vm::new(program, tracing=false)
    │       │
    │       ├── regs = [0, 0, 0, 0]      (Part 1)
    │       └── vm.run()  ──→ a = 318,117
    │
    └──→ Vm::new(program, tracing=false)
            │
            ├── regs = [0, 0, 1, 0]      (Part 2: c=1)
            └── vm.run()  ──→ a = 9,227,771
```

---

## Data Structures

### `Value` (enum)
```rust
enum Value {
    Lit(i64),     // Immediate integer value
    Reg(usize),   // Register index: a=0, b=1, c=2, d=3
}
```

Represents the source operand in `cpy` and `jnz` — can be either a literal
number or a register reference. Parsed once, resolved at execution time via
`Vm::resolve()`.

### `Instruction` (enum)
```rust
enum Instruction {
    Cpy(Value, usize),  // cpy src dst_reg
    Inc(usize),         // inc reg
    Dec(usize),         // dec reg
    Jnz(Value, i64),    // jnz test offset
}
```

Four variants, one per assembunny opcode. All `Copy` — no heap allocation.
The `usize` for registers maps directly to array indices (a=0, b=1, c=2, d=3).

### `Vm` (struct)
```rust
struct Vm {
    regs: [i64; 4],             // Register file: a, b, c, d
    pc: usize,                  // Program counter
    program: Vec<Instruction>,  // Loaded program
    trace: Vec<TraceEntry>,     // Execution log (empty when tracing=false)
    tracing: bool,              // Enable/disable trace recording
}
```

The virtual machine. Fixed-size register file means zero-cost register access.
Trace recording is **completely inert** when `tracing=false` — no allocations,
no string formatting, no overhead on benchmarked paths.

### `TraceEntry` (struct)
```rust
struct TraceEntry {
    pc: usize,              // Which instruction ran
    instruction: String,    // Human-readable instruction text
    regs_after: [i64; 4],   // Register snapshot after execution
}
```

Used for step-through debugging in tests. Implements `Display` for pretty-printing
with aligned columns. Only populated when `tracing=true`.

---

## Function-by-Function

### `reg_index(name: &str) -> usize`
Maps register names to array indices. Called during parsing only.
Panics on unknown register (invalid input).

### `parse_value(token: &str) -> Value`
Attempts to match token as a register name; if not, parses as integer literal.
This dual interpretation is the key to the `cpy` and `jnz` instructions accepting
either registers or immediates.

### `parse(input: &str) -> Vec<Instruction>`
Splits each line into whitespace-delimited tokens, matches the opcode, and
constructs the appropriate `Instruction` variant. Returns a `Vec` — program
is immutable after parsing.

### `Vm::new(program, tracing) -> Vm`
Initializes all registers to 0, PC to 0. The caller sets initial register
values (like `c=1` for Part 2) after construction.

### `Vm::resolve(&self, v: Value) -> i64`
Dereferences a `Value`: literals pass through, register references index
into `self.regs`. Called on every `cpy` source and `jnz` test operand.

### `Vm::step(&mut self) -> bool`
Executes **one instruction** at the current PC:

| Opcode | Action | PC Change |
|--------|--------|-----------|
| `cpy src dst` | `regs[dst] = resolve(src)` | PC += 1 |
| `inc r` | `regs[r] += 1` | PC += 1 |
| `dec r` | `regs[r] -= 1` | PC += 1 |
| `jnz test off` | If `resolve(test) != 0`: PC += off, else PC += 1 | conditional |

Returns `false` when PC goes out of bounds (program halted).
If tracing is enabled, records a `TraceEntry` after execution.

### `Vm::run(&mut self)`
Calls `step()` in a loop until it returns `false`. Simple but effective —
the entire Part 1 executes in ~180K steps, Part 2 in ~5.2M steps.

### `Vm::fmt_instr(instr: &Instruction) -> String`
Formats an instruction back to human-readable assembunny syntax for trace output.
Only called when tracing is enabled.

### `Vm::print_trace(&self)`
Dumps the full execution trace with aligned columns:
```
  PC  Instruction   a       b       c       d
----------------------------------------------------------
[ 0] cpy 41 a      a=41     b=0      c=0      d=0
[ 1] inc a         a=42     b=0      c=0      d=0
...
```

Used in tests for step-through debugging. Marked `#[allow(dead_code)]`
since it's only called from test code.

### `solve(input: &str) -> (String, String)`
Entry point. Parses once, runs Part 1 (all zeros), clones program for Part 2
(c=1), returns both answers.

---

## Algorithm: What Does This Program Actually Compute?

The 23-instruction program computes a **Fibonacci-like sequence**. Here's the
annotated assembunny with high-level pseudo-code:

```
 0: cpy 1 a          # a = 1
 1: cpy 1 b          # b = 1
 2: cpy 26 d         # d = 26 (loop counter)
 3: jnz c 2          # if c != 0: skip to line 5 (Part 2 path)
 4: jnz 1 5          # unconditional jump to line 9
 5: cpy 7 c          # c = 7 (Part 2 only)
 6: inc d            # ┐
 7: dec c            # ├── d += 7 (d becomes 33 for Part 2)
 8: jnz c -2         # ┘
 9: cpy a c          # ┐
10: inc a            # │
11: dec b            # ├── a = a + b (using c as temp)
12: jnz b -2         # │   a += b via increment loop
13: cpy c b          # │   b = old_a (saved in c)
14: dec d            # │
15: jnz d -6         # ┘   repeat d times
16: cpy 17 c         # ┐
17: cpy 18 d         # │
18: inc a            # │
19: dec d            # ├── a += 17 * 18 = 306
20: jnz d -2         # │   (nested multiply via inc/dec loops)
21: dec c            # │
22: jnz c -5         # ┘
```

### High-Level Equivalent

```
Part 1: fib(28) + 306 = 317,811 + 306 = 318,117
Part 2: fib(35) + 306 = 9,227,465 + 306 = 9,227,771
```

Where `fib(n)` is the nth Fibonacci number (with fib(1)=1, fib(2)=1):
- Lines 0-1: Initialize a=1, b=1 (first two Fibonacci numbers)
- Lines 2-8: Set d=26 (Part 1) or d=33 (Part 2, adds 7)
- Lines 9-15: Fibonacci loop — `(a, b) = (a+b, a)` repeated d times
- Lines 16-22: Add 17×18=306 to a (multiply via nested loops)

### Step Counts

| Part | Fibonacci iters (d) | Inner loop steps | Multiply steps | Total ~steps |
|------|---------------------|-----------------|----------------|-------------|
| Part 1 | 26 | ~180K | 306 | ~180K |
| Part 2 | 33 | ~5.2M | 306 | ~5.2M |

Part 2 is ~29× slower because Fibonacci numbers grow exponentially — the inner
addition loop (`inc a` repeated `b` times) does more work each iteration.

---

## Trace Mode: Stepping Through Execution

The VM has built-in trace capability for understanding execution flow:

```rust
// In tests: enable tracing to see every instruction
let mut vm = Vm::new(program, true);  // tracing = true
vm.run();
vm.print_trace();  // dumps full execution log
```

**No overhead when disabled**: `tracing=false` means `step()` skips the trace
recording entirely — no `String` allocations, no `Vec::push`, no `Display`
formatting. The `solve()` function always runs with tracing off.

**Trace output format**:
```
  PC  Instruction   a       b       c       d
----------------------------------------------------------
[ 0] cpy 1 a       a=1      b=0      c=0      d=0
[ 1] cpy 1 b       a=1      b=1      c=0      d=0
[ 2] cpy 26 d      a=1      b=1      c=0      d=26
[ 3] jnz c 2       a=1      b=1      c=0      d=26     ← c=0, falls through
[ 4] jnz 1 5       a=1      b=1      c=0      d=26     ← unconditional jump to 9
[ 9] cpy a c       a=1      b=1      c=1      d=26     ← Fibonacci iteration 1
[10] inc a         a=2      b=1      c=1      d=26
[11] dec b         a=2      b=0      c=1      d=26
[12] jnz b -2      a=2      b=0      c=1      d=26     ← b=0, exit inner loop
[13] cpy c b       a=2      b=1      c=1      d=26     ← b = old_a = 1
[14] dec d         a=2      b=1      c=1      d=25
[15] jnz d -6      a=2      b=1      c=1      d=25     ← d=25, loop back to 9
```

Each line shows: which instruction ran, and the full register state afterward.
The `jnz` entries are particularly informative — you can see whether the jump
was taken by checking whether the condition value was zero.

---

## Concepts for Study

### Register Machines
This is a textbook **register machine** (also called a counter machine):
- Fixed number of registers holding integer values
- Instructions operate on registers (no memory/stack)
- Program counter for sequential execution with jumps

The same model appears in:
- **Real CPUs**: x86, ARM (but with hundreds of instructions)
- **AUTOSAR**: Runnables with local state (registers) and scheduling (PC)
- **Future AoC**: Days 23 and 25 extend this same assembunny VM

### Fibonacci via Increment Loops
The program can't multiply directly — it simulates `a += b` by incrementing
`a` one at a time, `b` times. This is O(b) per addition, making the total
runtime proportional to the sum of all Fibonacci numbers computed.

This is analogous to how hardware multipliers evolved:
- **Shift-and-add**: Early CPUs, like this program
- **Hardware multiply**: Modern CPUs, single instruction
- **The optimization**: Day 23 adds `tgl` (toggle) which hints at self-modifying
  code to "compile" the multiply loop into something faster

### Conditional Initialization (Part 1 vs Part 2)
Lines 3-8 are a clever conditional block:
```
jnz c 2    ← if c != 0, skip next instruction
jnz 1 5    ← unconditional jump (skip the c=7 block)
cpy 7 c    ← only reached when c was nonzero initially
inc d / dec c / jnz c -2  ← d += 7
```

This is the assembunny equivalent of `if (c != 0) { d += 7; }`.
Part 2 sets c=1, activating this path: d goes from 26 to 33.

---

## Potential Optimizations (Not Applied)

| Optimization | Expected Impact | Why Not |
|-------------|-----------------|---------|
| Detect Fibonacci loop, compute directly | 1000×+ faster | Would bypass the VM entirely |
| Detect multiply loops (inc/dec/jnz) | 10-50× faster | Premature; 48ms is fine |
| JIT compile to native code | Orders of magnitude | Way beyond scope |
| Pre-resolve all `Value::Reg` | Negligible | Array lookup already O(1) |

At 48ms combined, this is well within the 100ms budget. The VM is clean,
traceable, and reusable for Days 23 and 25 which extend the instruction set.

---

## Input Analysis

The 23-instruction program:
```
 0: cpy 1 a       ┐
 1: cpy 1 b       ┤ Initialize: a=1, b=1, d=26
 2: cpy 26 d      ┘
 3: jnz c 2       ┐
 4: jnz 1 5       ┤ Part 2 gate: if c≠0, add 7 to d
 5: cpy 7 c       │
 6: inc d         │
 7: dec c         │
 8: jnz c -2      ┘
 9: cpy a c       ┐
10: inc a         │
11: dec b         ├ Fibonacci: (a,b) = (a+b, a), d times
12: jnz b -2      │
13: cpy c b       │
14: dec d         │
15: jnz d -6      ┘
16: cpy 17 c      ┐
17: cpy 18 d      │
18: inc a         ├ Multiply-add: a += 17 × 18 = 306
19: dec d         │
20: jnz d -2      │
21: dec c         │
22: jnz c -5      ┘
```

Three logical blocks: init + conditional, Fibonacci loop, multiply epilogue.

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
