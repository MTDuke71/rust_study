# Side-by-Side: Source to Assembly

**Example**: `items_to_bitset` function from AoC 2022 Day 3

---

## Stage-by-Stage Transformation

### Stage 0: Source Code

```rust
/// File: src/solver/day03.rs, line 88

#[inline]
fn items_to_bitset(s: &str) -> u128 {
    s.bytes().fold(0u128, |acc, b| acc | (1u128 << b))
}
```

**What it does**: Convert string to bitmask
- Input: `"abc"` 
- Output: `0b...01110` (bits 97, 98, 99 set)

---

### Stage 1: MIR (from aoc2022.mir, line 1330)

```mir
fn items_to_bitset(_1: &str) -> u128 {
    debug s => _1;
    let mut _0: u128;                    // return value
    let mut _2: std::str::Bytes<'_>;    // iterator

    bb0: {
        StorageLive(_2);
        _2 = core::str::<impl str>::bytes(move _1) -> [return: bb1, unwind continue];
    }

    bb1: {
        _0 = <std::str::Bytes<'_> as Iterator>::fold::<u128, {closure@...}>(
            move _2, 
            const 0_u128, 
            const ZeroSized: {closure@...}
        ) -> [return: bb2, unwind continue];
    }

    bb2: {
        StorageDead(_2);
        return;
    }
}

fn items_to_bitset::{closure#0}(_1: &mut {...}, _2: u128, _3: u8) -> u128 {
    debug acc => _2;
    debug b => _3;
    let mut _0: u128;
    let mut _4: u128;

    bb0: {
        StorageLive(_4);
        _4 = Shl(const 1_u128, copy _3);      // 1 << b
        _0 = BitOr(copy _2, move _4);         // acc | (1 << b)
        StorageDead(_4);
        return;
    }
}
```

**Key changes from source**:
- ✅ Control flow explicit (basic blocks `bb0`, `bb1`, `bb2`)
- ✅ Temporaries explicit (`_0`, `_2`, `_4`)
- ✅ Closure separated into own function
- ✅ Iterator methods are function calls
- ✅ `Shl` and `BitOr` are MIR operations

---

### Stage 2: LLVM IR (conceptual - function is inlined)

```llvm
; When inlined into caller, approximately:

define i128 @solve_part1_bitset(...) {
entry:
  ; ... parsing code ...
  
  %acc.init = i128 0
  %len = ; string length
  br label %loop.header
  
loop.header:
  %acc = phi i128 [ %acc.init, %entry ], [ %acc.next, %loop.body ]
  %idx = phi i64 [ 0, %entry ], [ %idx.next, %loop.body ]
  
  ; Check loop condition
  %done = icmp eq i64 %idx, %len
  br i1 %done, label %exit, label %loop.body
  
loop.body:
  ; Load byte from string
  %byte.ptr = getelementptr inbounds i8, ptr %string.ptr, i64 %idx
  %byte = load i8, ptr %byte.ptr, align 1
  
  ; Extend byte to u128
  %shift.amt = zext i8 %byte to i128
  
  ; Compute (1 << byte)
  %bit = shl i128 1, %shift.amt
  
  ; OR into accumulator
  %acc.next = or i128 %acc, %bit
  
  ; Increment index
  %idx.next = add nuw i64 %idx, 1
  br label %loop.header
  
exit:
  ; %acc now contains the bitset
  ; ... continue with rest of solve_part1_bitset ...
}
```

**Key changes from MIR**:
- ✅ SSA form (phi nodes merge values)
- ✅ Function fully inlined (no separate `items_to_bitset`)
- ✅ Types explicit (`i8`, `i64`, `i128`)
- ✅ Memory operations explicit (`load`, `getelementptr`)
- ✅ Still separate `shl` and `or` operations

---

### Stage 3: Assembly (x86-64, conceptual)

Since function is inlined, this appears inside `solve_part1_bitset`:

```asm
; Part of solve_part1_bitset function
; Simplified for clarity

.L_bitset_loop:
    ; rdi = string pointer
    ; rsi = string length
    ; rcx = loop index
    ; rax/rdx = accumulator (128-bit split across two registers)
    
    ; Load byte from string[index]
    movzx   r8d, BYTE PTR [rdi + rcx]    ; r8d = byte (zero-extended)
    
    ; Set bit in accumulator
    ; For ASCII 0-63: use rax, for 64-127: use rdx
    cmp     r8d, 64                      ; Check which half
    jb      .L_low_bits                  ; Jump if < 64
    
.L_high_bits:
    mov     r9d, r8d                     ; Copy byte value
    sub     r9d, 64                      ; Adjust for high half
    bts     rdx, r9                      ; rdx |= (1 << (byte-64))
    jmp     .L_next
    
.L_low_bits:
    bts     rax, r8                      ; rax |= (1 << byte)
    
.L_next:
    inc     rcx                          ; index++
    cmp     rcx, rsi                     ; index < length?
    jb      .L_bitset_loop               ; Loop if not done
    
    ; Continue with rest of solve_part1_bitset
    ; rax:rdx now contains the bitset result
```

**Key changes from LLVM IR**:
- ✅ **`bts` instruction** combines shift + OR into one!
- ✅ Registers allocated (rax, rdx, rcx, r8, r9)
- ✅ 128-bit value split across two 64-bit registers
- ✅ Branch for high vs low bits
- ✅ Calling convention applied

---

## The Magic Instruction: `bts`

### What is `bts`?

**BTS** = Bit Test and Set

```asm
bts  destination, bit_position
```

**Effect**: `destination |= (1 << bit_position)`

**In one CPU cycle!**

### Comparison

| Approach | Instructions | Cycles |
|----------|-------------|--------|
| **Naive**: `(1 << b) \| acc` | 2-3 (shift, or, mov) | 3-4 |
| **Optimized**: `bts acc, b` | 1 | 1-2 |

**This is why bitset is 15× faster than HashSet!**

---

## Complete Journey Summary

```
┌────────────────────────────────────────────────────┐
│ Source: s.bytes().fold(0u128, |acc, b| acc | ...)  │
│ • Iterator chain                                   │
│ • Closure                                          │
│ • High-level abstraction                           │
└────────────────┬───────────────────────────────────┘
                 │ rustc parsing + type checking
                 ↓
┌────────────────────────────────────────────────────┐
│ MIR: Basic blocks + Iterator::fold call            │
│ • Control flow explicit                            │
│ • Closure as separate function                     │
│ • Shl + BitOr as MIR operations                    │
└────────────────┬───────────────────────────────────┘
                 │ rustc → LLVM codegen
                 ↓
┌────────────────────────────────────────────────────┐
│ LLVM IR: Loop with phi nodes + shl + or            │
│ • SSA form                                         │
│ • Function inlined                                 │
│ • Loop in canonical form                           │
└────────────────┬───────────────────────────────────┘
                 │ LLVM optimization + codegen
                 ↓
┌────────────────────────────────────────────────────┐
│ Assembly: Loop with bts instruction                │
│ • Shift + OR → single bts                          │
│ • Registers allocated                              │
│ • 128-bit → two 64-bit registers                   │
└────────────────────────────────────────────────────┘

Result: 23.8µs (15× faster than HashSet version)
```

---

## Performance Impact

### HashSet Version (359µs)
- Hash function: ~10 instructions per char
- Memory access: heap allocation + pointer chasing
- Set insertion: collision checking
- **Total**: ~15 instructions per char × 301 rucksacks

### Bitset Version (23.8µs)
- Load byte: 1 instruction (`movzx`)
- Set bit: 1 instruction (`bts`)
- Loop overhead: 2 instructions (`inc`, `cmp`, `jb`)
- **Total**: ~4 instructions per char × 301 rucksacks

**Speedup**: 359µs / 23.8µs = **15.1×**

---

## Key Insights

1. **Zero-cost abstractions are real**
   - High-level: `fold`, closure, iterator
   - Low-level: Single `bts` instruction
   - **No overhead!**

2. **Inlining eliminates function calls**
   - Source: Separate `items_to_bitset` function
   - Assembly: Inlined directly into caller
   - **Zero function call overhead**

3. **Instruction selection matters**
   - LLVM chooses `bts` over shift + OR
   - x86-64 specific optimization
   - **2 operations → 1 instruction**

4. **Compiler is smarter than humans**
   - We wrote iterator chain
   - Compiler generated optimal loop
   - **Trust the optimizer!**

---

*Files referenced*:
- Source: `../src/solver/day03.rs` line 88
- MIR: `aoc2022.mir` line 1330
- LLVM IR: `aoc2022.ll` (inlined, search for `shl i128`)
- Assembly: `aoc2022.s` (inlined, search for `bts`)


[[Index]]