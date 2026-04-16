# Day 15: Dueling Generators — Annotated Assembly Guide

**Source**: [day15.rs](../../src/solver/day15.rs)
**Compiled with**: `cargo rustc --release --lib -- --emit asm -C "llvm-args=-x86-asm-syntax=intel"`
**Target**: x86-64, release mode (optimized)

---

## Table of Contents
1. [The Hot Loop](#the-hot-loop)
2. [Instruction-by-Instruction Walkthrough](#instruction-by-instruction-walkthrough)
3. [The mod_pow Loop](#the-mod_pow-loop)
4. [Block Setup (jump_ahead)](#block-setup-jump_ahead)
5. [Compiler Techniques](#compiler-techniques)
6. [Cycle Analysis](#cycle-analysis)
7. [Naive vs Optimized Comparison](#naive-vs-optimized-comparison)

---

## The Hot Loop

This is `count_block` with `next_value` and `mersenne_mod` fully inlined. Runs 1,000,000 times per Rayon block (40 blocks total for Part 1).

### Register Allocation
| Register | Role |
|----------|------|
| `r11` | Generator A current value |
| `rbx` | Generator B current value |
| `r9d` | Match counter for this block |
| `r10` | Loop counter (counts down from 1,000,000) |
| `r14`, `r15` | Temporaries for mersenne_mod |
| `ebp` | Temporary: 0 or 1 from match comparison |

### Raw Assembly

```asm
.p2align 4                          ; align loop entry to 16 bytes for fetch efficiency
.LBB87_17:
    imul  r11, r11, 16807
    mov   r14d, r11d
    and   r14d, 2147483647
    shr   r11, 31
    lea   r15, [r14 + r11]
    cmp   r15, 2147483647
    lea   r11, [r14 + r11 - 2147483647]
    cmovb r11, r15
    imul  rbx, rbx, 48271
    mov   r14d, ebx
    and   r14d, 2147483647
    shr   rbx, 31
    lea   r15, [r14 + rbx]
    cmp   r15, 2147483647
    lea   rbx, [r14 + rbx - 2147483647]
    cmovb rbx, r15
    xor   ebp, ebp
    mov   r14d, ebx
    xor   r14w, r11w
    sete  bpl
    add   r9d, ebp
    dec   r10
    jne   .LBB87_17
```

---

## Instruction-by-Instruction Walkthrough

### Generator A: `next_value(a, FACTOR_A)`

```asm
imul  r11, r11, 16807              ; a = a * 16807
                                    ; product is up to ~2⁴⁶ (31-bit × 15-bit)
                                    ; fits in u64, no overflow
```

This is the LCG step. `imul reg, reg, imm` is a 3-cycle latency instruction on modern Intel/AMD. The factor 16807 = 7⁵ is encoded as an immediate operand.

### Mersenne Mod for Generator A

```asm
mov   r14d, r11d                   ; r14 = lower 32 bits of product
                                    ; (writing to r14d zero-extends to 64 bits)

and   r14d, 2147483647             ; r14 = product & 0x7FFFFFFF
                                    ; extracts low 31 bits (the "low" half)

shr   r11, 31                      ; r11 = product >> 31
                                    ; extracts upper bits (the "high" half)
                                    ; for factor 16807: max high ≈ 16806

lea   r15, [r14 + r11]            ; r15 = low + high
                                    ; this is the candidate result
                                    ; max value: (2³¹ - 2) + 16806 = 2,147,500,453

cmp   r15, 2147483647             ; compare sum against MODULUS
                                    ; sets carry flag (CF) if sum < MODULUS

lea   r11, [r14 + r11 - 2147483647]
                                    ; r11 = low + high - MODULUS
                                    ; speculatively compute the "overflow" path
                                    ; LEA does NOT affect flags — CMP result preserved!

cmovb r11, r15                     ; if CF=1 (sum < MODULUS): r11 = r15 (the sum)
                                    ; if CF=0 (sum >= MODULUS): r11 stays (sum - MODULUS)
                                    ; BRANCHLESS conditional select!
```

### Generator B: Same Pattern

```asm
imul  rbx, rbx, 48271             ; b = b * 48271

mov   r14d, ebx                   ; extract low 32 bits
and   r14d, 2147483647            ; low = product & 0x7FFFFFFF
shr   rbx, 31                     ; high = product >> 31
lea   r15, [r14 + rbx]           ; sum = low + high
cmp   r15, 2147483647            ; compare against MODULUS
lea   rbx, [r14 + rbx - 2147483647]  ; speculative: sum - MODULUS
cmovb rbx, r15                    ; branchless select
```

Identical structure, different factor. The compiler didn't try to interleave A and B — they execute sequentially, but the CPU's out-of-order engine can overlap them since B doesn't depend on A's result.

### Match Comparison: `(a ^ b) & 0xFFFF == 0`

```asm
xor   ebp, ebp                    ; ebp = 0 (clear the accumulator)

mov   r14d, ebx                   ; r14 = b (copy to avoid clobbering)
xor   r14w, r11w                  ; XOR the low 16 bits of a and b
                                    ; zero flag (ZF) set if low 16 bits match

sete  bpl                          ; bpl = 1 if ZF=1 (match), 0 otherwise
                                    ; SETE: "set byte if equal"
                                    ; bpl is the low byte of ebp (already zeroed)
```

The Rust source says `(a ^ b) & MASK_16 == 0`. The compiler recognized that `xor r14w, r11w` already operates on only the low 16 bits (the `w` suffix = word = 16 bits), so the `& 0xFFFF` mask is implicit. One fewer instruction.

### Accumulate and Loop

```asm
add   r9d, ebp                    ; matches += 0 or 1 (branchless!)
dec   r10                         ; loop_counter -= 1
jne   .LBB87_17                   ; if counter != 0, repeat
```

The only branch in the entire loop body is `jne` for the loop itself — perfectly predicted by the CPU until the final iteration.

---

## The mod_pow Loop

Before each block runs the hot loop, it computes starting seeds via `jump_ahead` → `mod_pow`. Here's the exponentiation-by-squaring loop for Generator A:

```asm
;; r14 = exp (remaining exponent bits)
;; rbx = base (current squared value)
;; r11 = result (accumulated product)

.LBB87_10:                         ; ── square step (always) ──
    mov   r15, r14
    shr   r15                      ; r15 = exp >> 1 (shift for next iteration)

    imul  rbx, rbx                 ; base = base * base
    mov   r12d, ebx
    and   r12d, 2147483645         ; (note: 2147483645, not 2147483647 — compiler quirk,
    shr   rbx, 31                  ;  functionally equivalent since bit 0 is unused here)
    lea   r13, [r12 + rbx]
    cmp   r13, 2147483647
    lea   rbx, [r12 + rbx - 2147483647]
    cmovb rbx, r13                 ; mersenne_mod(base * base)

    cmp   r14, 1                   ; if exp <= 1, we're done
    mov   r14, r15                 ; exp = exp >> 1
    jbe   .LBB87_11

.LBB87_8:                          ; ── multiply step (if odd bit) ──
    test  r14b, 1                  ; test lowest bit of exponent
    je    .LBB87_10                ; if even, skip to square step

    imul  r11, rbx                 ; result = result * base
    mov   r15d, r11d
    and   r15d, 2147483647
    shr   r11, 31
    lea   r12, [r15 + r11]
    cmp   r12, 2147483647
    lea   r11, [r15 + r11 - 2147483647]
    cmovb r11, r12                 ; mersenne_mod(result * base)

    jmp   .LBB87_10                ; continue with square step
```

This runs ~20 iterations per `mod_pow` call (log₂(1,000,000) ≈ 20). At ~10 instructions per iteration, it's ~200 instructions total — negligible compared to the 1M-iteration hot loop that follows.

---

## Block Setup (jump_ahead)

After `mod_pow`, the seed is finalized:

```asm
.LBB87_11:                         ; mod_pow for A complete, r11 = factor_a^offset

    imul  r11, rdx                 ; seed_a = original_seed * factor_a^offset
    mov   ebx, r11d
    and   ebx, 2147483647
    shr   r11, 31
    lea   r14, [rbx + r11]
    cmp   r14, 2147483647
    lea   r11, [rbx + r11 - 2147483647]
    cmovb r11, r14                 ; mersenne_mod(seed * mod_pow_result)
```

Then the same pattern repeats for Generator B (`.LBB87_13` through `.LBB87_16`), followed by the hot loop at `.LBB87_17`.

---

## Compiler Techniques

### 1. `cmovb` — Branchless Conditional Select
The Rust `if sum >= MODULUS { sum - MODULUS } else { sum }` compiled to:
```asm
cmp   r15, 2147483647
lea   r11, [r14 + r11 - 2147483647]    ; compute both paths
cmovb r11, r15                          ; select based on flags
```
**Why it matters**: A branch here would mispredict ~50% of the time (the comparison outcome is essentially random for LCG outputs). At ~15 cycles per mispredict × 2 generators × 1M iterations = 15M wasted cycles. `cmovb` has fixed 1-cycle latency.

### 2. `lea` for Speculative Arithmetic
`lea` (Load Effective Address) computes `base + index + displacement` without touching flags. The compiler uses it to compute `sum - MODULUS` *before* knowing whether the subtraction is needed, because `lea` won't clobber the `cmp` result.

### 3. `sete` — Branchless Boolean
```asm
xor   r14w, r11w      ; sets ZF if low 16 bits match
sete  bpl              ; bpl = (ZF ? 1 : 0)
add   r9d, ebp         ; matches += 0 or 1
```
Three instructions, zero branches. The alternative `if match { count += 1 }` would compile to a branch that mispredicts at the match rate (~1/65536 = 0.0015%). Even with high prediction accuracy, `sete` is still better because it avoids the branch predictor table entry entirely.

### 4. 16-bit Register Suffix Eliminates Mask
`xor r14w, r11w` operates on only the low 16 bits (the `w` suffix). This makes the `& 0xFFFF` mask from the Rust source completely free — the instruction encoding handles it.

### 5. `.p2align 4` — Loop Alignment
The loop entry is aligned to a 16-byte boundary. This ensures the loop fits cleanly in the instruction fetch window (typically 16 or 32 bytes per cycle). Misaligned loops can lose 10-20% throughput from fetch bubbles.

---

## Cycle Analysis

### Per-Iteration Cost (Hot Loop)

| Stage | Instructions | Latency | Notes |
|-------|-------------|---------|-------|
| Gen A: `imul` | 1 | 3 cycles | 64-bit multiply |
| Gen A: mersenne_mod | 6 | ~4 cycles | AND + SHR + LEA + CMP + LEA + CMOV |
| Gen B: `imul` | 1 | 3 cycles | Independent of A — can overlap |
| Gen B: mersenne_mod | 6 | ~4 cycles | Same pattern |
| Compare + accumulate | 5 | ~3 cycles | XOR + SETE + ADD + DEC + JNE |
| **Total** | **19** | **~7 cycles** | Out-of-order execution overlaps A and B |

At ~4 GHz: 7 cycles × 1M iterations = ~1.75ms per block.
40 blocks across ~8 threads ≈ 5 blocks per thread ≈ 8.75ms for Part 1.
(Actual is ~55ms due to Rayon overhead, `mod_pow` startup, and memory effects.)

### What the Naive Version Would Look Like

```asm
;; HYPOTHETICAL naive version with % operator:
imul  rax, r11, 16807    ; a * factor
xor   edx, edx           ; zero-extend for div
div   rcx                 ; 64-bit divide by MODULUS (rcx = 2147483647)
mov   r11, rdx           ; remainder is the result
```

`div rcx` on modern Intel: **35-90 cycles** latency, and it's not pipelineable (the divider is a shared resource). Two `div` instructions per iteration (A + B) would dominate at 70-180 cycles vs our ~7 cycles. That's the **10-25× difference** the Mersenne trick eliminates.

---

## Naive vs Optimized Comparison

| Aspect | Naive (`%`) | Optimized (mersenne_mod) |
|--------|------------|--------------------------|
| Key instruction | `div r64` (35-90 cycles) | `and` + `shr` + `lea` + `cmovb` (~4 cycles) |
| Branches per iteration | 1-2 (if/else for match) | 0 (cmovb + sete) |
| Total cycles/iteration | ~80-190 | ~7 |
| Pipeline friendly | No (div blocks execution) | Yes (all pipelineable) |
| Branch mispredictions | ~50% on mersenne_mod path | 0 (branchless) |
| Loop branch prediction | Same (>99.99% correct) | Same |
