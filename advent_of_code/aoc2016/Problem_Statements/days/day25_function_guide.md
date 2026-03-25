# Day 25: Clock Signal -- Function Guide

## Problem Summary

Find the lowest positive integer to initialize register `a` that causes an assembunny program (extended with `out`) to output the clock signal `0, 1, 0, 1, ...` repeating forever.

**Part 1**: Lowest `a` producing clock signal. Answer: **180**
**Part 2**: Collect all 50 stars (freebie). Answer: **Deliver the Signal!**

---

## Answers

| Part | Answer | Time (release) |
|------|--------|------|
| 1 | `180` | 6.43ms |
| 2 | `Deliver the Signal!` | -- |
| Combined | -- | 6.25ms |

---

## Function Map

```
solve(input) -> (String, String)
  |- parse_input(input) -> Vec<Instruction>     # Parse assembunny + out instruction
  |- solve_part1(&program) -> i64               # Search for lowest valid a
  |    `- produces_clock_signal(program, a, 50) # Run VM, check output pattern
  |         |- resolve(regs, val) -> i64        # Evaluate Value (Lit or Reg)
  |         `- VM execution loop                # cpy/inc/dec/jnz/out
  `- Part 2: literal string (freebie)
```

---

## Algorithm Details

### Program Analysis

The input program has three phases:

1. **Initialization (lines 1-9)**: Computes `d = a + 15 * 170 = a + 2550`, then copies `d` back to `a`.

2. **Division loop (lines 11-27)**: Repeatedly divides `a` by 2 using subtract-and-branch logic, computing quotient and remainder.

3. **Output + loop (lines 28-30)**: Outputs the remainder (`b`), then jumps back. When `a` reaches 0, jumps back to line 9 which reloads `a` from `d`, repeating the cycle.

The net effect: the program outputs the binary digits of `a + 2550` in LSB-first order, repeating forever.

### Mathematical Solution

For the output `0, 1, 0, 1, ...`, the value `a + 2550` must have the binary pattern `...10101010`:

| Binary | Decimal | a = val - 2550 |
|--------|---------|----------------|
| `1010` | 10 | -2540 (negative) |
| `101010` | 42 | -2508 (negative) |
| `10101010` | 170 | -2380 (negative) |
| `1010101010` | 682 | -1868 (negative) |
| `101010101010` | 2730 | **180** |

The smallest positive result is `a = 2730 - 2550 = 180`.

### Simulation Approach

Rather than rely on hand-analysis alone, the implementation simulates the VM for each candidate `a` starting from 1, checking that the first 50 output values match the expected `0, 1, 0, 1, ...` pattern. This is robust against edge cases and confirms the mathematical analysis.

The search terminates quickly because `a = 180` is found after only 180 trials, each running at most ~50 output cycles.

---

## Performance

| Metric | Value |
|--------|-------|
| Combined (release) | 6.25ms |
| Candidates tested | 180 |
| Outputs verified per candidate | Up to 50 |
| Max VM steps per candidate | 1,000,000 (safety limit) |
| Program size | 30 instructions |

---

## Key Insight

The assembunny program is a binary digit extractor: it computes `a + C` (where C is input-dependent, here 2550) and outputs the binary representation LSB-first in an infinite loop. The clock signal `0, 1, 0, 1, ...` corresponds to the repeating binary pattern `...101010`, which is `0xAAAA...` The problem reduces to finding the smallest `a` such that `a + C` has this alternating-bit pattern.

This is the third evolution of the assembunny VM across AoC 2016:
- **Day 12**: Base VM (cpy, inc, dec, jnz)
- **Day 23**: Added `tgl` (self-modifying code) + multiplication optimization
- **Day 25**: Added `out` (signal output) for the finale

---

## Data Flow

```
Input (30-line assembunny program with `out` instruction)
  |
  +- Parse: 5 instruction types (cpy, inc, dec, jnz, out)
  |
  +- For a = 1, 2, 3, ...:
  |    |
  |    +- Initialize regs = [a, 0, 0, 0]
  |    |
  |    +- Run VM, collect `out` values
  |    |
  |    +- Check: outputs match 0, 1, 0, 1, ... ?
  |    |    |
  |    |    +- No  -> try next a
  |    |    +- Yes -> return a
  |    |
  |    (a = 180: program computes 180 + 2550 = 2730 = 0b101010101010)
  |
  +- Part 1: 180
  |
  +- Part 2: "Deliver the Signal!" (50-star freebie)
```

---

**See also**: [AoC 2016 Summary](../summary_2016.md) | [Day 12 (Base VM)](day12_function_guide.md) | [Day 23 (Extended VM)](day23_function_guide.md)
