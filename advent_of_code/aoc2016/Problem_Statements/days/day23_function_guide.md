# Day 23: Safe Cracking — Function Guide

## Problem Summary

Extend the Day 12 assembunny VM with a `tgl` (toggle) instruction that modifies instructions at runtime. The program is self-modifying — `tgl x` changes the instruction at offset `x` from the current PC according to fixed rules. The program computes `a! + C` where C is an input-dependent constant (93 * 80 = 7440 for our input).

**Part 1**: Start with `a=7`, run the program, return register `a`. Answer: `7! + 7440 = 12480`
**Part 2**: Start with `a=12`, run the program, return register `a`. Answer: `12! + 7440 = 479009040`

---

## Answers

| Part | Answer | Time |
|------|--------|------|
| 1 | `12480` | 35.7us |
| 2 | `479009040` | 30.9us |
| Combined | — | 63.6us |

---

## Function Map

```
solve(input) -> (String, String)
  ├── parse_input(input) -> Vec<Instruction>         # parse assembunny with tgl support
  ├── solve_part1_with_data(&program) -> i64         # run VM with a=7
  └── solve_part2_with_data(&program) -> i64         # run VM with a=12
        └── run_vm(&program, init_a) -> i64
              ├── detect_mul_loop(&prog, pc)          # pattern-match multiply loop
              └── toggle(instr) -> Instruction        # apply toggle rules
```

---

## Algorithm Details

### Toggle Rules
- **1-argument**: `inc` <-> `dec`; `tgl` -> `inc`
- **2-argument**: `jnz` <-> `cpy`
- Invalid instructions (e.g., `cpy x 1`) are silently skipped

### Key Design Changes from Day 12
1. **All arguments are `Value`**: After toggling, `jnz` offset can be a register (from `cpy x reg`), and `cpy` destination can be a literal (invalid, skip). Using `Value` everywhere handles this cleanly.
2. **Mutable program**: `tgl` modifies instructions in-place, so the program must be cloned per run.
3. **Multiplication loop optimization**: Without it, Part 2 would execute ~479M iterations (12!). The optimizer detects the 6-instruction multiply pattern and replaces it with `a += b * d` in one step.

### Program Analysis
The input computes factorial via nested loops:
- Lines 0-9: Multiply loop (`a = b * d`)
- Lines 10-18: Decrement `b`, toggle future instructions, jump back
- Lines 19-26: Add constant `93 * 80 = 7440` to result

### Multiply Loop Pattern (6 instructions)
```
cpy B C        # C = B
inc A          # A++        (inner)
dec C          # C--        (inner)
jnz C -2      # loop inner: A += B
dec D          # D--        (outer)
jnz D -5      # loop outer: A += B * D
```
Detected at runtime and replaced with: `regs[A] += regs[B] * regs[D]; regs[C] = 0; regs[D] = 0;`

---

## Performance

| Metric | Value |
|--------|-------|
| Part 1 (a=7) | 35.7us |
| Part 2 (a=12) | 30.9us |
| Combined | 63.6us |
| Without mul-loop opt (Part 2 est.) | ~seconds (479M iterations) |
| Speedup from optimization | ~100,000x+ |

---

## Key Insight

The program is a factorial calculator in disguise. The `tgl` instruction progressively disables the loop-back jump (line 18: `jnz 1 c` -> `cpy 1 c`) once `b` reaches 1, causing execution to fall through to the constant-addition epilogue. Recognizing the multiplication loop pattern and short-circuiting it is essential for Part 2 performance.

```asm
                            # Initial state: a = 7 (Part 1) or 12 (Part 2)
                            # --- Factorial loop: a = a * (a-1) * ... * 1 ---
 0: cpy a b                 # b = a (first iter), then b = previous b
 1: dec b                   # b = a - 1 (loop counter for next multiply)
                            #
                            # --- Multiply: a = b * d (nested loop) ---
 2: cpy a d                 # d = a (multiplicand, previous product)
 3: cpy 0 a                 # a = 0 (accumulator reset)
 4: cpy b c                 # c = b (inner loop counter)
 5: inc a                   #   a++
 6: dec c                   #   c--
 7: jnz c -2               #   loop 5-7: a += b (one iteration of inner)
 8: dec d                   # d--
 9: jnz d -5               # loop 4-9: repeat d times -> a += b*d (multiply!)
                            #
                            # --- Toggle future instructions, loop back ---
10: dec b                   # b-- (next multiplier)
11: cpy b c                 # c = b
12: cpy c d                 # d = b
13: dec d                   #   d--
14: inc c                   #   c++
15: jnz d -2               #   loop 13-15: c = b + b = 2*b (toggle offset)
16: tgl c                   # toggle instruction at PC+2b (modifies epilogue)
17: cpy -16 c               # c = -16
18: jnz 1 c                # jump to PC 2 (unconditional, offset=-16)
                            #   -> toggled to "cpy 1 c" when b=1, ending loop
                            #
                            # --- Epilogue: a += 93 * 80 = 7440 ---
19: cpy 93 c                # c = 93 (outer counter)
20: jnz 80 d               #   -> toggled to "cpy 80 d" (d = 80, inner counter)
21: inc a                   #   a++
22: inc d                   #   -> toggled to "dec d"
23: jnz d -2               #   loop 21-23: a += 80
24: inc c                   #   -> toggled to "dec c"
25: jnz c -5               #   loop 20-25: repeat 93 times -> a += 93*80
                            #
                            # Result: a = a! + 7440
```

### Toggle Trace (a=7)

Each outer loop iteration decrements `b` by 1, so `c = 2*b` walks the toggle target backwards by 2:

| b | c = 2*b | tgl target (16+c) | Effect |
|---|---------|-------------------|--------|
| 6 | 12 | 28 | outside program, noop |
| 5 | 10 | 26 | outside program, noop |
| 4 | 8 | 24 | `inc c` -> `dec c` |
| 3 | 6 | 22 | `inc d` -> `dec d` |
| 2 | 4 | 20 | `jnz 80 d` -> `cpy 80 d` |
| 1 | 2 | 18 | `jnz 1 c` -> `cpy 1 c` (breaks factorial loop) |