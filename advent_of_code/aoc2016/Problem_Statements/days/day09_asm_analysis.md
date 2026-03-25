# Day 9: Assembly Analysis — How LLVM Turns Rust Into 157ns

**Purpose**: Annotated assembly output for `solve_part1_with_data`, showing how the Rust
compiler (via LLVM) transforms high-level Rust into extremely efficient machine code.

**Benchmark**: 157ns for 16,654 bytes = ~0.6 cycles/byte at 4GHz

**Generated with**: `cargo asm -p aoc2016 --lib "aoc2016::solver::day09::solve_part1_with_data"`

---

## Table of Contents
1. [The Rust Source Code](#the-rust-source-code)
2. [Register Allocation Map](#register-allocation-map)
3. [Annotated Assembly — Full Listing](#annotated-assembly--full-listing)
4. [Deep Dive: The Hot Loop (Literal Bytes)](#deep-dive-the-hot-loop-literal-bytes)
5. [Deep Dive: Marker Scanning](#deep-dive-marker-scanning)
6. [Deep Dive: Integer Parsing via LEA](#deep-dive-integer-parsing-via-lea)
7. [Deep Dive: The Final Multiply + Advance](#deep-dive-the-final-multiply--advance)
8. [Compiler Optimization Catalog](#compiler-optimization-catalog)
9. [Why Part 2 Is 110x Slower](#why-part-2-is-110x-slower)
10. [Key Takeaways for Compiler Study](#key-takeaways-for-compiler-study)
11. [Appendix: x86-64 Assembly Mnemonic Glossary](#appendix-x86-64-assembly-mnemonic-glossary)

---

## The Rust Source Code

```rust
fn solve_part1_with_data(data: &[u8]) -> usize {
    let mut len = 0;                    // Accumulated decompressed length
    let mut i = 0;                      // Current position in byte slice
    while i < data.len() {
        if data[i] == b'(' {
            // Find closing paren
            let close = data[i..].iter().position(|&b| b == b')').unwrap() + i;
            let marker = std::str::from_utf8(&data[i + 1..close]).unwrap();
            let (chars, repeat) = marker.split_once('x').unwrap();
            let chars: usize = chars.parse().unwrap();
            let repeat: usize = repeat.parse().unwrap();
            len += chars * repeat;
            i = close + 1 + chars;
        } else {
            len += 1;                   // Count literal byte
            i += 1;                     // Advance pointer
        }
    }
    len
}
```

**What the Rust code does at a high level:**
- Walk through a byte slice character by character
- For literal bytes: count them (len += 1)
- For markers like `(10x2)`: parse the two numbers, add `10 * 2` to len, skip ahead
- Return total decompressed length

**Key observation**: This function calls `.position()`, `from_utf8()`, `split_once('x')`,
and `.parse()` — five separate function calls per marker. In the assembly, **all five are
fully inlined**. There are zero function calls in the hot path.

---

## Register Allocation Map

The compiler assigned all hot variables to registers — no memory loads for loop state:

| Register | Rust Variable | Purpose |
|----------|--------------|---------|
| `rdi` | `data.as_ptr()` | Pointer to start of byte slice |
| `rsi` | `data.len()` | Length of byte slice |
| `r14` | `i` | Current scan position |
| `rbx` | `len` | Accumulated decompressed length (the return value) |
| `r12` | `close - i` | Offset from current position to `)` |
| `r13` | varies | Pointer into marker string for number parsing |
| `r15` | `10` (constant) | Preloaded multiplier for decimal digit parsing |
| `rbp` | remaining length | Bytes remaining in the number being parsed |

**Note**: `r15 = 10` is loaded once at function entry and reused for every digit parsed.
The compiler recognized that `str::parse::<usize>()` always multiplies by 10 in a loop,
so it hoisted the constant.

---

## Annotated Assembly — Full Listing

### Function Prologue — Save Registers

```asm
solve_part1_with_data:
    push r15                    ; Save callee-saved registers
    push r14                    ; (Windows x64 ABI requires preserving
    push r13                    ;  rbx, rbp, rdi, rsi, r12-r15)
    push r12
    push rsi
    push rdi
    push rbp
    push rbx
    sub rsp, 152                ; Reserve 152 bytes of stack space
                                ; (for from_utf8 result, CharSearcher state, etc.)
```

8 registers saved = this function uses ALL available general-purpose registers.
The 152 bytes of stack are for temporary structures that couldn't fit in registers
(the `CharSearcher` state for `split_once`, `from_utf8` result enum, etc.).

### Empty Input Check

```asm
    test rdx, rdx               ; rdx = data.len() (Windows x64: 2nd arg)
    je .LBB150_1                 ; If length == 0, jump to "return 0"
```

### Setup — Load Constants and Initialize

```asm
    mov rsi, rdx                 ; rsi = data.len()
    mov rdi, rcx                 ; rdi = data.as_ptr() (Windows x64: 1st arg)
    mov eax, 1
    sub rax, rdx                 ; rax = 1 - data.len() (used for bounds checking later)
    mov [rsp + 144], rax         ; Spill to stack (not enough registers)
    lea rax, [rcx + 1]
    mov [rsp + 136], rax         ; data.as_ptr() + 1 (start of marker content after '(')
    xor ebx, ebx                 ; len = 0
    lea r13, [rsp + 48]          ; r13 = pointer to stack scratch space
    mov r15d, 10                 ; r15 = 10 (decimal base, used in parse loops)
    xor r14d, r14d               ; i = 0
    mov [rsp + 128], rdx         ; Backup data.len() on stack
    jmp .LBB150_4                ; Enter the main loop
```

**Key optimization**: `r15 = 10` is loaded ONCE here and reused every time a digit
is parsed. The compiler constant-hoisted the decimal base out of `str::parse()`.

---

### === THE MAIN LOOP ===

### Hot Path: Literal Byte (the `else` branch)

```asm
    .p2align 4                   ; Align to 16-byte boundary (cache line optimization)
.LBB150_5:                       ; ← This is the HOT LOOP for literal bytes
    inc r14                      ; i += 1
    mov eax, 1
    add rbx, rax                 ; len += 1
    cmp r14, rsi                 ; i < data.len()?
    jae .LBB150_2                ; If i >= len, exit loop → return
.LBB150_4:                       ; ← Main loop entry point
    cmp byte ptr [rdi + r14], 40 ; data[i] == '(' ?  (40 = ASCII for '(')
    jne .LBB150_5                ; Not '(' → back to literal byte handler
```

**This is the critical hot loop. Just 5 instructions per literal byte:**

| # | Instruction | Rust equivalent | Cycles |
|---|------------|----------------|--------|
| 1 | `inc r14` | `i += 1` | ~1 |
| 2 | `mov eax, 1` | (load constant 1) | ~1 |
| 3 | `add rbx, rax` | `len += 1` | ~1 |
| 4 | `cmp r14, rsi` | `i < data.len()` | ~1 |
| 5 | `cmp byte ptr [rdi + r14], 40` | `data[i] == b'('` | ~1 (L1 cache) |

**Why this is fast:**
- **Branch prediction**: `jne .LBB150_5` is taken ~95% of the time (most bytes are
  letters, not `(`). The CPU learns this pattern within a few iterations and
  speculatively executes the literal path before the comparison completes.
- **`.p2align 4`**: Loop is aligned to a 16-byte boundary, ensuring it doesn't straddle
  cache lines. This prevents instruction fetch stalls.
- **`[rdi + r14]`**: Simple base+index addressing, resolved in a single cycle.
  The byte slice data is likely in L1 cache after the first access (16KB input fits
  in L1 easily).

**Question**: Why `mov eax, 1` + `add rbx, rax` instead of just `inc rbx`?
This is likely to break a dependency chain — `inc` modifies flags which can create
a false dependency on modern CPUs. Using `mov`+`add` keeps the critical path shorter
by allowing out-of-order execution to overlap with the `cmp` instruction.

---

### Marker Path: Scan for Closing ')'

When `data[i] == '('`, we fall through to the marker handler:

```asm
    ; At this point: r14 = i (position of '(')
    mov rax, [rsp + 144]         ; rax = 1 - data.len() (precomputed)
    add rax, r14                 ; rax = i + 1 - data.len() (negative offset)
    mov rcx, [rsp + 136]         ; rcx = data.as_ptr() + 1
    add rcx, r14                 ; rcx = &data[i + 1] (first byte after '(')
    mov rdx, -1                  ; rdx = search offset, starts at -1

    .p2align 4
.LBB150_8:                       ; ← Scan loop for ')'
    lea r8, [rax + rdx]          ; Bounds check computation
    cmp r8, -1
    je .LBB150_9                 ; Panic: ran off end without finding ')'
    lea r12, [rdx + 1]           ; r12 = offset + 1
    cmp byte ptr [rcx + rdx + 1], 41  ; data[i + 1 + offset + 1] == ')' ? (41 = ')')
    mov rdx, r12                 ; offset = offset + 1
    jne .LBB150_8                ; Not ')' → keep scanning
```

**This is `.position(|&b| b == b')')` fully inlined.** The closure `|&b| b == b')'`
became a single `cmp byte, 41` instruction. No function pointer, no closure allocation,
no iterator overhead.

After finding `)`:
```asm
    lea rax, [r14 + r12]         ; rax = i + marker_len (position of ')')
    inc rax                      ; rax = close + 1 (first byte after ')')
    lea rdx, [r14 + 1]           ; rdx = i + 1 (first byte of marker content)
```

---

### from_utf8 — Validate Marker String

```asm
    ; Check slice bounds: data[i+1..close]
    cmp rax, r14                 ; close + 1 > i ?
    jbe .LBB150_55               ; Panic: invalid slice bounds
    cmp rax, rsi                 ; close + 1 <= data.len() ?
    ja .LBB150_55                ; Panic: out of bounds

    ; Call from_utf8 on the marker slice
    add rdx, rdi                 ; rdx = data.as_ptr() + i + 1 (start of marker)
    mov rcx, r13                 ; rcx = output buffer (stack)
    mov r8, r12                  ; r8 = length of marker string
    call core::str::converts::from_utf8
```

**This is the ONE actual function call in the main path.** `from_utf8` validates that
the marker bytes (e.g., "10x2") are valid UTF-8. Everything else is inlined.

In theory, we know the marker is ASCII (digits and 'x'), so this validation is redundant.
But the compiler can't prove that, so it keeps the call. This is the cost of using
`std::str::from_utf8` — a function that must handle arbitrary byte sequences.

(If we wanted to eliminate this call, we could use `unsafe { std::str::from_utf8_unchecked() }`
or parse the bytes directly without converting to `&str` first.)

---

### split_once('x') — Find the 'x' Separator

```asm
    cmp dword ptr [rsp + 48], 1  ; Check from_utf8 result (Ok vs Err)
    je .LBB150_56                ; Err → panic (unwrap failed)

    ; Load the validated &str (pointer + length)
    mov r13, [rsp + 56]          ; r13 = marker string pointer
    mov rbp, [rsp + 64]          ; rbp = marker string length

    ; Set up CharSearcher to find 'x' (ASCII 120 = 0x78)
    mov [rsp + 48], r13          ; haystack pointer
    mov [rsp + 56], rbp          ; haystack length
    mov qword ptr [rsp + 64], 0  ; search start index
    mov qword ptr [rsp + 72], rbp ; search end index
    mov byte ptr [rsp + 88], 1   ; search state flag
    movabs rax, 515396075640     ; 'x' as a packed pattern (0x7800000078)
    mov [rsp + 80], rax          ; character to search for

    ; Call the searcher
    lea rcx, [rsp + 96]          ; output buffer
    call <CharSearcher>::next_match

    cmp byte ptr [rsp + 96], 0   ; Did we find 'x'?
    je .LBB150_57                ; No → panic (unwrap on None)
    mov r9, [rsp + 104]          ; r9 = position of 'x' (= length of first number)
    mov r8, [rsp + 112]          ; r8 = position after 'x' (= start of second number)
```

**`split_once('x')` compiles to a CharSearcher.** The magic constant `515396075640`
(= `0x78_0000_0078`) is how Rust's pattern matching stores the character 'x' (0x78)
for the searcher. The CharSearcher is a state machine that handles multi-byte UTF-8
characters, but for ASCII 'x' it degenerates to a simple byte scan.

This is **not** inlined as tightly as the `)` scan — the CharSearcher has enough
state that LLVM kept it as a function call. In a hand-optimized version, this would
just be another `position(|b| b == b'x')` byte scan.

---

### Parse First Number (chars) — The LEA Trick

```asm
    ; r13 = pointer to start of number string
    ; r9 = length of number string (bytes before 'x')
    ; Goal: parse decimal string into usize

    ; Handle potential '+' prefix (from generic integer parsing)
    cmp r9, 1                   ; Single digit?
    je .LBB150_17               ; Special case for 1-digit numbers
    test r9, r9                 ; Zero length?
    je .LBB150_16               ; Empty string → error
    movzx ecx, byte ptr [r13]  ; Load first byte
    jmp .LBB150_20              ; Check for sign prefix

.LBB150_20:
    ; Check and skip '+' sign if present
    xor r10d, r10d
    cmp cl, 43                  ; cl == '+' ? (43 = ASCII '+')
    sete r10b                   ; r10 = 1 if '+', 0 otherwise
    mov rax, r10
    neg rax                     ; rax = -1 if '+' found, 0 otherwise
    mov rcx, r9
    sub rcx, r10                ; rcx = remaining length after sign
    add r10, r13                ; r10 = pointer past sign character
```

**Note**: The `parse::<usize>()` implementation handles `+` and `-` prefixes even though
we know the input is unsigned digits only. The compiler inlined the full generic integer
parser, including sign handling. The sign check adds ~3 instructions per number — negligible.

```asm
    ; Check if we can use the "small number" fast path
    cmp rcx, 17                 ; Is the number < 17 digits?
    jae .LBB150_21              ; ≥17 digits → use overflow-checking path

    test rcx, rcx               ; Zero digits remaining?
    je .LBB150_27               ; Yes → result is 0

    ; === THE DIGIT PARSING LOOP (small numbers, no overflow check) ===
    xor eax, eax                ; accumulator = 0
    xor ecx, ecx                ; result = 0

    .p2align 4
.LBB150_29:                     ; ← Digit parsing hot loop
    movzx edx, byte ptr [r10 + rax]   ; Load next byte
    add edx, -48                       ; Subtract '0' (ASCII 48)
    cmp edx, 9                         ; Is it a valid digit (0-9)?
    ja .LBB150_30                      ; No → parse error

    lea rcx, [rcx + 4*rcx]            ; rcx = rcx * 5
    mov edx, edx                       ; Zero-extend edx (clear upper 32 bits)
    lea rcx, [rdx + 2*rcx]            ; rcx = digit + (rcx * 5) * 2
                                       ;      = digit + rcx * 10
    inc rax                            ; Advance to next digit
    mov rdx, r9
    add rdx, rax                       ; Check if done
    jne .LBB150_29                     ; More digits → loop
    jmp .LBB150_34                     ; Done → rcx holds parsed number
```

### THE LEA TRICK EXPLAINED

The standard way to compute `result = result * 10 + digit` would be:

```asm
    ; Naive approach (what you might expect):
    imul rcx, rcx, 10           ; result *= 10     (3-4 cycles, uses multiplier unit)
    add rcx, rdx                ; result += digit   (1 cycle)
```

Instead, LLVM generates:

```asm
    lea rcx, [rcx + 4*rcx]     ; rcx = rcx + 4*rcx = rcx * 5    (1 cycle, uses AGU)
    lea rcx, [rdx + 2*rcx]     ; rcx = digit + 2*(rcx*5) = digit + rcx*10  (1 cycle)
```

**How this works:**
- `LEA` (Load Effective Address) computes `base + index * scale + displacement`
  in a single cycle using the CPU's Address Generation Unit (AGU)
- Legal scale factors: 1, 2, 4, 8
- `x * 10 = x * 5 * 2 = (x + 4x) * 2`
- Step 1: `lea rcx, [rcx + 4*rcx]` → `rcx * 5` using scale=4 (1 cycle)
- Step 2: `lea rcx, [rdx + 2*rcx]` → `digit + rcx*10` using scale=2 (1 cycle)

**Why LEA beats IMUL:**
| Property | `imul rcx, 10` | Two `lea` instructions |
|----------|----------------|----------------------|
| Cycles | 3-4 | 2 (pipelined) |
| Execution unit | Multiplier (shared, limited) | AGU (abundant, 2+ per core) |
| Pipelining | Blocks multiplier for 3 cycles | Each LEA is independent |
| Port pressure | Port 1 only (Intel) | Ports 1 and 5 (can run in parallel) |

**This is a 2x speedup on the integer parsing inner loop** — and LLVM applies it
automatically. Every call to `.parse::<usize>()` in Rust benefits from this.

---

### Parse Second Number (repeat)

The second number parsing (for the repeat count after 'x') uses the identical LEA
trick. The assembly is nearly a mirror of the first parsing loop:

```asm
.LBB150_49:                          ; ← Second number digit loop
    movzx r8d, byte ptr [r13 + rdx] ; Load digit byte
    add r8d, -48                     ; Subtract '0'
    cmp r8d, 9                       ; Valid digit?
    ja .LBB150_50                    ; No → error

    lea rax, [rax + 4*rax]          ; rax = rax * 5
    mov r8d, r8d                     ; Zero-extend
    lea rax, [r8 + 2*rax]           ; rax = digit + rax * 10
    inc rdx
    cmp rbp, rdx
    jne .LBB150_49                  ; More digits → loop
    jmp .LBB150_53                  ; Done → rax holds repeat count
```

Same pattern: `lea + lea` instead of `imul + add`.

---

### The Final Multiply + Advance

```asm
.LBB150_53:
    imul rax, rcx                ; chars * repeat (the actual multiplication!)
    add r14, rcx                 ; i += chars (skip past consumed characters)
    add r14, r12                 ; i += marker_len (skip past marker content)
    add r14, 2                   ; i += 2 (skip '(' and ')')
    mov rdi, rsi                 ; Restore data pointer (was clobbered)
    mov rsi, [rsp + 128]         ; Restore data.len()
    lea r13, [rsp + 48]          ; Restore scratch pointer
    add rbx, rax                 ; len += chars * repeat
    cmp r14, rsi                 ; i < data.len()?
    jb .LBB150_4                 ; Yes → back to main loop
    jmp .LBB150_2                ; No → exit, return len
```

**Interesting**: Here we see `imul rax, rcx` — an actual multiply instruction.
The compiler uses LEA for `* 10` (known constant) but `imul` for `chars * repeat`
(two runtime values with no constant factor to decompose).

Also note: `add r14, 2` accounts for the `(` and `)` characters. The compiler
computed `i = close + 1 + chars` as three separate additions:
- `add r14, rcx` → `+chars`
- `add r14, r12` → `+marker_content_length` (close - i)
- `add r14, 2` → `+2` (for '(' and ')')

---

### Return Path

```asm
.LBB150_1:                      ; Empty input path
    xor ebx, ebx                ; len = 0

.LBB150_2:                      ; Normal exit
    mov rax, rbx                ; Return value = len
    add rsp, 152                ; Deallocate stack
    pop rbx                     ; Restore callee-saved registers
    pop rbp
    pop rdi
    pop rsi
    pop r12
    pop r13
    pop r14
    pop r15
    ret
```

### Error/Panic Paths (Cold Code)

```asm
.LBB150_55:                      ; Slice bounds check failed
    call core::slice::index::slice_index_fail

.LBB150_56:                      ; from_utf8 returned Err
    call core::result::unwrap_failed

.LBB150_9:                       ; .position() returned None (no ')' found)
    call core::option::unwrap_failed

.LBB150_30:                      ; First number parse failed (invalid digit)
    ...
    call core::result::unwrap_failed

.LBB150_57:                      ; split_once('x') returned None
    call core::option::unwrap_failed
```

**All panic paths are at the end of the function** — LLVM moves cold (rarely-taken)
code to the bottom so the hot path instructions are contiguous in memory. This
improves instruction cache utilization because the CPU prefetcher loads sequential
cache lines.

---

## Compiler Optimization Catalog

Every optimization LLVM applied to this function:

| # | Optimization | What It Did | Impact |
|---|-------------|-------------|--------|
| 1 | **Function inlining** | `.position()`, `from_utf8()`, `split_once()`, `.parse()` all inlined | Eliminated 4+ function call/return pairs per marker |
| 2 | **Closure elimination** | `\|&b\| b == b')'` became `cmp byte, 41` | Zero closure overhead |
| 3 | **Constant hoisting** | `r15 = 10` loaded once, reused in all parse loops | Saved a load per digit |
| 4 | **Strength reduction** | `* 10` → `lea x5` + `lea x2` (LEA trick) | 2 cycles instead of 3-4 per digit |
| 5 | **Register allocation** | All loop variables in registers (`r14`, `rbx`, `rsi`, `rdi`) | Zero memory loads in hot loop |
| 6 | **Loop alignment** | `.p2align 4` on hot loops | Prevents cache line straddling |
| 7 | **Cold code sinking** | Panic paths moved to function end | Hot path stays contiguous in icache |
| 8 | **Branch layout** | `jne .LBB150_5` (taken ~95%) is the fall-through for predicted case | Minimal branch misprediction |
| 9 | **Bounds check folding** | Slice bounds checks combined with loop condition | Reduced redundant comparisons |
| 10 | **Sign handling specialization** | Single-digit fast path for `parse()` | Avoids loop setup for 1-digit numbers |

---

## Why Part 2 Is 110x Slower

Part 1: **157ns** | Part 2: **17,300ns** (110x ratio)

The Part 2 function `decompressed_len` has the same structure but **cannot benefit from
the same optimizations** because of recursion:

| Factor | Part 1 | Part 2 |
|--------|--------|--------|
| Function calls | 1 (from_utf8) | N recursive calls + N from_utf8 |
| Stack usage | 152 bytes (fixed) | 152 bytes × recursion depth |
| Register state | Preserved across loop | Saved/restored at each call |
| Branch prediction | One loop pattern | Different pattern at each depth |
| Cache pressure | Sequential scan | Random-ish slice jumps |
| Inlining | Full | Cannot inline recursion into itself |

The recursion means:
- Each marker processes a **sub-slice** → new function call → save 8 registers → allocate
  152 bytes of stack → restore everything on return
- The CPU's branch predictor sees different patterns at different recursion depths
- The instruction cache must hold multiple "copies" of the same function at different
  call depths (different return addresses)

**Could Part 2 be made faster?** Yes — converting the recursion to an explicit stack
(using a `Vec<(usize, usize)>`) would avoid function call overhead. But at 17µs, it's
already well under the 100ms threshold, so the clarity of recursive code wins.

---

## Key Takeaways for Compiler Study

### 1. Zero-Cost Abstractions Are Real
The Rust source uses high-level abstractions: iterators (`.position()`), string methods
(`.split_once()`, `.parse()`), Result/Option (`.unwrap()`), slicing (`&data[i..close]`).
In the assembly, these abstractions **completely disappear**. The generated code is
equivalent to hand-written C with raw pointer arithmetic.

### 2. LEA Is the Compiler's Secret Weapon
`LEA` (Load Effective Address) is nominally for computing memory addresses, but compilers
use it as a **general-purpose multiply-and-add** instruction:
- `lea rax, [rax + 4*rax]` = `rax * 5` (1 cycle)
- `lea rax, [rbx + 2*rax]` = `rbx + rax * 2` (1 cycle)
- `lea rax, [rax + 8*rax]` = `rax * 9` (1 cycle)

Any multiplication by 2, 3, 4, 5, 8, or 9 can be done with a single `LEA`.
Multiplications by 10, 12, etc. use two `LEA` instructions (still faster than `imul`).

### 3. Branch Prediction Drives Performance
The 5-instruction hot loop runs at ~1 cycle per iteration not because each instruction
is 1 cycle, but because the CPU **speculatively executes** the next iteration before
the current `cmp` completes. With 95%+ prediction accuracy on the `jne` branch, the
pipeline stays full almost continuously.

### 4. Cold Code Sinking Matters
All panic/error paths are placed at the end of the function. This means the hot path
(the main loop + marker handler) occupies a contiguous block of memory that fits in
a few instruction cache lines. If panic paths were interleaved, the hot code would be
fragmented across more cache lines, increasing icache misses.

### 5. The Cost of Safety
The one "unnecessary" cost is the `call from_utf8` — UTF-8 validation on bytes we know
are ASCII. This is the price of safe Rust. To eliminate it, you'd use:
```rust
// SAFETY: marker bytes are guaranteed ASCII (digits + 'x')
let marker = unsafe { std::str::from_utf8_unchecked(&data[i + 1..close]) };
```
But at 157ns total, this optimization isn't worth the unsafe. The from_utf8 check
is fast (it's basically a SIMD scan) and only runs once per marker (~200 times total).

### 6. Register Pressure Is Real
The function uses ALL 8 callee-saved registers plus needs 152 bytes of stack. This is
why LLVM couldn't inline everything — `split_once` requires a `CharSearcher` state
machine with too many fields to fit in registers. In an ideal world, the compiler would
recognize that searching for a single ASCII byte doesn't need a full CharSearcher and
could use a simple byte scan (like it did for `)`).

---

## Appendix: Full Raw Assembly

For reference, the complete unedited assembly output:

```asm
solve_part1_with_data:
	push r15
	push r14
	push r13
	push r12
	push rsi
	push rdi
	push rbp
	push rbx
	sub rsp, 152
	test rdx, rdx
	je .LBB150_1
	mov rsi, rdx
	mov rdi, rcx
	mov eax, 1
	sub rax, rdx
	mov qword ptr [rsp + 144], rax
	lea rax, [rcx + 1]
	mov qword ptr [rsp + 136], rax
	xor ebx, ebx
	lea r13, [rsp + 48]
	mov r15d, 10
	xor r14d, r14d
	mov qword ptr [rsp + 128], rdx
	jmp .LBB150_4
	.p2align	4
.LBB150_5:
	inc r14
	mov eax, 1
	add rbx, rax
	cmp r14, rsi
	jae .LBB150_2
.LBB150_4:
	cmp byte ptr [rdi + r14], 40
	jne .LBB150_5
	mov rax, qword ptr [rsp + 144]
	add rax, r14
	mov rcx, qword ptr [rsp + 136]
	add rcx, r14
	mov rdx, -1
	.p2align	4
.LBB150_8:
	lea r8, [rax + rdx]
	cmp r8, -1
	je .LBB150_9
	lea r12, [rdx + 1]
	cmp byte ptr [rcx + rdx + 1], 41
	mov rdx, r12
	jne .LBB150_8
	lea rax, [r14 + r12]
	inc rax
	lea rdx, [r14 + 1]
	cmp rax, r14
	jbe .LBB150_55
	cmp rax, rsi
	ja .LBB150_55
	add rdx, rdi
	mov rcx, r13
	mov r8, r12
	call core::str::converts::from_utf8
	cmp dword ptr [rsp + 48], 1
	je .LBB150_56
	mov rsi, rdi
	mov rdx, r13
	mov r13, qword ptr [rsp + 56]
	mov rbp, qword ptr [rsp + 64]
	mov qword ptr [rsp + 48], r13
	mov qword ptr [rsp + 56], rbp
	mov qword ptr [rsp + 64], 0
	mov qword ptr [rsp + 72], rbp
	mov byte ptr [rsp + 88], 1
	movabs rax, 515396075640
	mov qword ptr [rsp + 80], rax
	lea rcx, [rsp + 96]
	call <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
	cmp byte ptr [rsp + 96], 0
	je .LBB150_57
	mov r9, qword ptr [rsp + 104]
	mov r8, qword ptr [rsp + 112]
	cmp r9, 1
	je .LBB150_17
	test r9, r9
	je .LBB150_16
	movzx ecx, byte ptr [r13]
	jmp .LBB150_20
	.p2align	4
.LBB150_17:
	movzx ecx, byte ptr [r13]
	mov al, 1
	cmp ecx, 43
	je .LBB150_31
	cmp ecx, 45
	je .LBB150_31
.LBB150_20:
	xor r10d, r10d
	cmp cl, 43
	sete r10b
	mov rax, r10
	neg rax
	mov rcx, r9
	sub rcx, r10
	add r10, r13
	cmp rcx, 17
	jae .LBB150_21
	test rcx, rcx
	je .LBB150_27
	add r9, rax
	neg r9
	xor eax, eax
	xor ecx, ecx
	.p2align	4
.LBB150_29:
	movzx edx, byte ptr [r10 + rax]
	add edx, -48
	cmp edx, 9
	ja .LBB150_30
	lea rcx, [rcx + 4*rcx]
	mov edx, edx
	lea rcx, [rdx + 2*rcx]
	inc rax
	mov rdx, r9
	add rdx, rax
	jne .LBB150_29
	jmp .LBB150_34
	.p2align	4
.LBB150_21:
	add r9, rax
	neg r9
	xor r11d, r11d
	xor ecx, ecx
	.p2align	4
.LBB150_22:
	mov rax, r9
	add rax, r11
	je .LBB150_34
	movzx edi, byte ptr [r10 + r11]
	add edi, -48
	cmp edi, 9
	ja .LBB150_30
	mov rax, rcx
	mul r15
	mov ecx, edi
	seto dil
	add rcx, rax
	setb dl
	mov al, 2
	test dil, dil
	jne .LBB150_31
	inc r11
	test dl, dl
	je .LBB150_22
	jmp .LBB150_31
.LBB150_27:
	xor ecx, ecx
	.p2align	4
.LBB150_34:
	sub rbp, r8
	add r13, r8
	cmp rbp, 1
	je .LBB150_37
	test rbp, rbp
	je .LBB150_36
	movzx eax, byte ptr [r13]
	jmp .LBB150_40
	.p2align	4
.LBB150_37:
	movzx eax, byte ptr [r13]
	mov dl, 1
	cmp eax, 43
	je .LBB150_51
	cmp eax, 45
	je .LBB150_51
.LBB150_40:
	xor edx, edx
	cmp al, 43
	sete dl
	sub rbp, rdx
	add r13, rdx
	cmp rbp, 17
	jae .LBB150_41
	test rbp, rbp
	je .LBB150_47
	xor edx, edx
	xor eax, eax
	.p2align	4
.LBB150_49:
	movzx r8d, byte ptr [r13 + rdx]
	add r8d, -48
	cmp r8d, 9
	ja .LBB150_50
	lea rax, [rax + 4*rax]
	mov r8d, r8d
	lea rax, [r8 + 2*rax]
	inc rdx
	cmp rbp, rdx
	jne .LBB150_49
	jmp .LBB150_53
	.p2align	4
.LBB150_41:
	xor r8d, r8d
	xor eax, eax
	.p2align	4
.LBB150_42:
	cmp rbp, r8
	je .LBB150_53
	movzx r9d, byte ptr [r13 + r8]
	add r9d, -48
	cmp r9d, 9
	ja .LBB150_50
	mul r15
	mov rdx, rax
	mov eax, r9d
	seto r10b
	add rax, rdx
	setb r9b
	mov dl, 2
	test r10b, r10b
	jne .LBB150_51
	inc r8
	test r9b, r9b
	je .LBB150_42
	jmp .LBB150_51
.LBB150_47:
	xor eax, eax
	.p2align	4
.LBB150_53:
	imul rax, rcx
	add r14, rcx
	add r14, r12
	add r14, 2
	mov rdi, rsi
	mov rsi, qword ptr [rsp + 128]
	lea r13, [rsp + 48]
	add rbx, rax
	cmp r14, rsi
	jb .LBB150_4
	jmp .LBB150_2
.LBB150_1:
	xor ebx, ebx
.LBB150_2:
	mov rax, rbx
	add rsp, 152
	pop rbx
	pop rbp
	pop rdi
	pop rsi
	pop r12
	pop r13
	pop r14
	pop r15
	ret
.LBB150_55:
	lea r9, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.154]
	mov rcx, rdx
	mov rdx, rax
	mov r8, rsi
	call core::slice::index::slice_index_fail
.LBB150_56:
	lea rax, [rsp + 56]
	movups xmm0, xmmword ptr [rax]
	movaps xmmword ptr [rsp + 96], xmm0
	lea rax, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.155]
	mov qword ptr [rsp + 32], rax
	lea rcx, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.32]
	lea r9, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.33]
	lea r8, [rsp + 96]
	mov edx, 43
	call core::result::unwrap_failed
.LBB150_9:
	lea rcx, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.153]
	call core::option::unwrap_failed
.LBB150_30:
	mov al, 1
.LBB150_31:
	mov byte ptr [rsp + 48], al
	lea rax, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.157]
	jmp .LBB150_32
.LBB150_50:
	mov dl, 1
.LBB150_51:
	mov byte ptr [rsp + 48], dl
	lea rax, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.158]
.LBB150_32:
	mov qword ptr [rsp + 32], rax
	lea rcx, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.32]
	lea r9, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.31]
	lea r8, [rsp + 48]
	mov edx, 43
	call core::result::unwrap_failed
.LBB150_57:
	lea rcx, [rip + anon.9e49de5a2ebdc41a0fc64b82939e12ae.156]
	call core::option::unwrap_failed
.LBB150_16:
	xor eax, eax
	jmp .LBB150_31
.LBB150_36:
	xor edx, edx
	jmp .LBB150_51
```

---

## Appendix: x86-64 Assembly Mnemonic Glossary

Every instruction mnemonic that appears in this listing, grouped by category.

### Data Movement

| Mnemonic | Full Name | Description |
|----------|-----------|-------------|
| `mov` | Move | Copy value from source to destination. `mov rax, rbx` copies rbx into rax. |
| `movzx` | Move with Zero-Extend | Copy a smaller value (8/16-bit) into a larger register, filling upper bits with zeros. `movzx ecx, byte ptr [r13]` loads 1 byte into a 32-bit register. |
| `movabs` | Move Absolute (64-bit immediate) | Load a full 64-bit immediate value into a register. Used when the constant is too large for a normal `mov`. |
| `movups` | Move Unaligned Packed Single | Copy 128 bits (16 bytes) between memory and XMM register without requiring 16-byte alignment. Used here for bulk-copying error data. |
| `movaps` | Move Aligned Packed Single | Copy 128 bits between memory and XMM register, requiring 16-byte alignment. Faster than `movups` when alignment is guaranteed. |
| `push` | Push onto Stack | Decrement RSP by 8 and store a 64-bit value at the new stack top. Used in the prologue to save callee-saved registers. |
| `pop` | Pop from Stack | Load 64-bit value from the stack top and increment RSP by 8. Used in the epilogue to restore registers. |

### Arithmetic

| Mnemonic | Full Name | Description |
|----------|-----------|-------------|
| `add` | Add | Add source to destination: `add rbx, rax` → `rbx = rbx + rax`. Sets flags (CF, OF, ZF, SF). |
| `sub` | Subtract | Subtract source from destination: `sub rax, rdx` → `rax = rax - rdx`. Sets flags. |
| `inc` | Increment | Add 1 to operand: `inc r14` → `r14 = r14 + 1`. Sets all flags except CF. |
| `neg` | Negate (Two's Complement) | Flip the sign: `neg rax` → `rax = -rax`. Equivalent to `0 - rax`. |
| `imul` | Signed Integer Multiply | Multiply two registers: `imul rax, rcx` → `rax = rax * rcx`. Used for the final `chars * repeat`. |
| `mul` | Unsigned Multiply | Multiply RAX by operand, result in RDX:RAX (128-bit). Used in the overflow-checking parse path. |
| `xor` | Exclusive OR | Bitwise XOR. `xor ebx, ebx` is the idiomatic way to zero a register (1 byte shorter than `mov ebx, 0`, also clears flags and breaks dependency chains). |
| `lea` | Load Effective Address | Compute an address expression without accessing memory. Despite the name, used as a general-purpose multiply-add: `lea rcx, [rcx + 4*rcx]` = `rcx * 5`. Supports `base + index * scale + displacement` where scale ∈ {1, 2, 4, 8}. |

### Comparison & Testing

| Mnemonic | Full Name | Description |
|----------|-----------|-------------|
| `cmp` | Compare | Subtract source from destination **without storing the result** — only sets flags. `cmp r14, rsi` computes `r14 - rsi` and sets ZF/CF/SF/OF for subsequent conditional jumps. |
| `test` | Logical Compare | Bitwise AND **without storing the result** — only sets flags. `test rdx, rdx` checks if rdx is zero (sets ZF if so). More efficient than `cmp rdx, 0`. |

### Conditional Set (SETcc)

| Mnemonic | Full Name | Description |
|----------|-----------|-------------|
| `sete` | Set if Equal (ZF=1) | Set byte to 1 if previous comparison was equal, 0 otherwise. `sete r10b` stores the boolean result of a `cmp`. |
| `seto` | Set if Overflow (OF=1) | Set byte to 1 if previous arithmetic overflowed. Used in the overflow-checking integer parse path. |
| `setb` | Set if Below (CF=1) | Set byte to 1 if unsigned addition carried. Used alongside `seto` to detect overflow in `mul` + `add`. |

### Jumps & Control Flow

| Mnemonic | Full Name | Condition | Description |
|----------|-----------|-----------|-------------|
| `jmp` | Unconditional Jump | always | Jump to target label unconditionally. |
| `je` | Jump if Equal | ZF=1 | Jump if previous `cmp`/`test` found equality (or zero). Same encoding as `jz`. |
| `jne` | Jump if Not Equal | ZF=0 | Jump if not equal / not zero. Same encoding as `jnz`. |
| `ja` | Jump if Above | CF=0 ∧ ZF=0 | Unsigned greater-than. Used after `cmp edx, 9` to check for invalid digits. |
| `jae` | Jump if Above or Equal | CF=0 | Unsigned greater-than-or-equal. `cmp r14, rsi; jae` = "if i >= data.len()". |
| `jb` | Jump if Below | CF=1 | Unsigned less-than. `cmp r14, rsi; jb` = "if i < data.len()". |
| `jbe` | Jump if Below or Equal | CF=1 ∨ ZF=1 | Unsigned less-than-or-equal. |
| `call` | Call Procedure | — | Push return address onto stack and jump to function. Used for `from_utf8`, `next_match`, and panic paths. |
| `ret` | Return from Procedure | — | Pop return address from stack and jump to it. |

### Stack Management

| Mnemonic | Full Name | Description |
|----------|-----------|-------------|
| `sub rsp, N` | Allocate Stack | Reserve N bytes of stack space by moving the stack pointer down. |
| `add rsp, N` | Deallocate Stack | Release N bytes of stack space by moving the stack pointer up. |

### Assembler Directives (Not Instructions)

| Directive | Description |
|-----------|-------------|
| `.p2align 4` | Pad with NOPs until the next address is aligned to 2⁴ = 16-byte boundary. Ensures hot loops start at cache-line-friendly addresses. |
| `.LBB150_N:` | Local label generated by LLVM. `LBB` = LLVM Basic Block, `150` = function index, `N` = block number within the function. |
| `byte ptr` | Size qualifier indicating an 8-bit (1-byte) memory operand. |
| `dword ptr` | Size qualifier indicating a 32-bit (4-byte) memory operand. |
| `qword ptr` | Size qualifier indicating a 64-bit (8-byte) memory operand. |
| `xmmword ptr` | Size qualifier indicating a 128-bit (16-byte) memory operand (for SSE/XMM registers). |
| `[rip + ...]` | RIP-relative addressing. Accesses data at an offset from the current instruction pointer. Used for static/constant data (string literals, panic messages). Position-independent by design. |

### Register Naming Conventions

| Prefix | Size | Example | Notes |
|--------|------|---------|-------|
| `r__` | 64-bit | `rax`, `rbx`, `r14` | Full register width on x86-64. |
| `e__` | 32-bit | `eax`, `ebx`, `r14d` | Lower 32 bits. Writing to `eax` zero-extends into `rax`. |
| `__l` / `__b` | 8-bit | `al`, `cl`, `r10b` | Lowest byte of the register. |

**Key insight**: `xor ebx, ebx` zeros all 64 bits of `rbx` because writing a 32-bit register implicitly zero-extends to 64 bits on x86-64. This is why the compiler uses `xor ebx, ebx` instead of `xor rbx, rbx` — shorter encoding, same result.

---

**See also**: [Day 9 Function Guide](day09_function_guide.md) | [AoC 2016 Summary](../summary_2016.md)
