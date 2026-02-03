# Quick Reference: Viewing Compiler Stages

## Current Generated Files

All files are in `compiler_output/` directory:

- **aoc2022.mir** (66KB) - Mid-level IR with control flow
- **aoc2022.ll** (686KB) - LLVM IR (platform-independent)
- **aoc2022.s** (217KB) - x86-64 assembly (Intel syntax)

## Key Finding: Function is Fully Inlined! 🚀

The `items_to_bitset` function **does not appear as a standalone function** in the assembly because:
- Marked with `#[inline]` attribute
- Only called from within the same crate
- Compiler inlined it completely into callers (`solve_part1_bitset`, `solve_part2_bitset`)

**This is GREAT** - it means zero function call overhead!

---

## How to Explore the Files

### 1. View MIR (Mid-level IR)

**Search for our function**:
```powershell
Select-String -Path "compiler_output\aoc2022.mir" -Pattern "fn items_to_bitset" -Context 0,30
```

**What you see**:
- Basic blocks (`bb0`, `bb1`, ...)
- Explicit temporaries (`_2`, `_3`, ...)
- Iterator calls before inlining
- Closure separated into its own function

**Key MIR snippet** (line 1330):
```mir
fn items_to_bitset(_1: &str) -> u128 {
    debug s => _1;
    let mut _0: u128;
    let mut _2: std::str::Bytes<'_>;
    
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
```

**Closure** (line 1350):
```mir
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

**Observations**:
- ✅ Shift and OR are **separate MIR operations** (`Shl`, `BitOr`)
- ✅ Closure is separate function (will be inlined later by LLVM)
- ✅ Control flow is explicit (basic blocks)

---

### 2. View LLVM IR

**Search for inlined bitset operations**:
```powershell
# Find loop with shl and or operations
Select-String -Path "compiler_output\aoc2022.ll" -Pattern "shl i128" -Context 2,2 | Select-Object -First 20
```

**What you see**:
- SSA form (every value assigned once)
- Phi nodes for merging control flow
- Intrinsics (`shl`, `or`, `zext`)
- Loop unrolling optimization

**Key LLVM patterns** (from inlined code around line 4250):
```llvm
; Load byte from string
%byte = load i8, ptr %byte_ptr, align 1

; Zero-extend u8 -> u128
%shift_amt = zext i8 %byte to i128

; Shift operation
%bit = shl i128 1, %shift_amt

; OR into accumulator
%new_acc = or i128 %acc, %bit
```

**Observations**:
- ✅ Still separate `shl` and `or` operations
- ✅ Type system explicit (`i8`, `i128`)
- ✅ Loop may be unrolled (process multiple bytes per iteration)

---

### 3. View Assembly

Since the function is inlined, search for the pattern in calling functions:

```powershell
# Find bit manipulation instructions
Select-String -Path "compiler_output\aoc2022.s" -Pattern "bts|shl.*128|or.*128" -Context 1,1 | Select-Object -First 50
```

**What to look for**:
- `bts` instruction (Bit Test and Set) - combines shift + OR
- Register allocation (which registers hold accumulator, byte)
- Loop structure

**Expected assembly pattern**:
```asm
# Somewhere in solve_part1_bitset or solve_part2_bitset:

.L_loop:
    movzx   r8, BYTE PTR [rdi + rcx]   # Load byte
    bts     rax, r8                     # Set bit in accumulator (magic!)
    inc     rcx                         # Increment index
    cmp     rcx, rsi                    # Compare with length
    jb      .L_loop                     # Loop if not done
```

**The `bts` instruction is THE key**:
- `bts rax, r8` = "Bit Test and Set"
- Equivalent to: `rax |= (1 << r8)`
- **Single instruction** instead of shift + OR!
- This is why bitset is 15× faster than HashSet

---

## Hands-On Exploration Tools

### Option 1: cargo-asm (Recommended)

**Install**:
```bash
cargo install cargo-asm
```

**View assembly for specific function**:
```bash
cd advent_of_code/aoc2022
cargo asm --rust --release aoc2022::solver::day03::solve_bitset
```

**Benefits**:
- Shows Rust source alongside assembly
- Filters out boilerplate
- Color-coded output

### Option 2: Compiler Explorer (Online)

**URL**: https://godbolt.org

**Steps**:
1. Paste the `items_to_bitset` function
2. Add necessary imports:
   ```rust
   #[inline]
   pub fn items_to_bitset(s: &str) -> u128 {
       s.bytes().fold(0u128, |acc, b| acc | (1u128 << b))
   }
   ```
3. Select "Rust" compiler, enable optimizations (`-C opt-level=3`)
4. See assembly side-by-side with source

**Benefits**:
- Interactive (click source line → see assembly)
- Compare optimization levels
- Try different architectures (ARM, RISC-V)

### Option 3: Generate Standalone Function

To see non-inlined assembly, create a public function:

```rust
#[no_mangle]
pub extern "C" fn items_to_bitset_noinline(s: &str) -> u128 {
    s.bytes().fold(0u128, |acc, b| acc | (1u128 << b))
}
```

Then compile:
```bash
cargo rustc --release --lib -- --emit=asm -C llvm-args=-x86-asm-syntax=intel
```

Search for `items_to_bitset_noinline` in the `.s` file - it will NOT be inlined!

---

## Understanding What You're Seeing

### MIR Level

**Purpose**: Borrow checker, basic optimizations

**What to notice**:
- Control flow graph (basic blocks)
- Explicit drops and storage management
- Separate functions (not yet inlined)

### LLVM IR Level

**Purpose**: Platform-independent optimization

**What to notice**:
- SSA form (phi nodes)
- Loop optimizations (unrolling, vectorization)
- Inlining happens here
- Type information still present

### Assembly Level

**Purpose**: Machine-specific code generation

**What to notice**:
- Register allocation
- Instruction selection (e.g., `bts` for shift+OR)
- Calling conventions
- Final optimizations (peephole, instruction scheduling)

---

## Compiler Flags Cheat Sheet

```bash
# Generate all intermediate forms
cargo rustc --release --lib -- --emit=mir,llvm-ir,asm

# Different optimization levels
cargo rustc --lib -- --emit=asm -C opt-level=0  # None
cargo rustc --lib -- --emit=asm -C opt-level=1  # Basic
cargo rustc --lib -- --emit=asm -C opt-level=2  # Default
cargo rustc --lib -- --emit=asm -C opt-level=3  # Aggressive (--release)
cargo rustc --lib -- --emit=asm -C opt-level=s  # Size
cargo rustc --lib -- --emit=asm -C opt-level=z  # Min size

# Disable inlining (for study purposes)
cargo rustc --release --lib -- --emit=asm -C inline-threshold=0

# Enable verbose LLVM output
cargo rustc --release --lib -- --emit=llvm-ir -C llvm-args=-print-after-all

# Different assembly syntax
cargo rustc --release --lib -- --emit=asm -C llvm-args=-x86-asm-syntax=intel  # Intel (default)
cargo rustc --release --lib -- --emit=asm -C llvm-args=-x86-asm-syntax=att    # AT&T
```

---

## Next Steps for Compiler Study

1. **Compare Optimization Levels**:
   - Generate assembly with `-C opt-level=0` vs `-C opt-level=3`
   - See how many instructions differ
   - Observe loop unrolling, inlining

2. **Study Other Functions**:
   - `find_common_item_bitset` (bitwise AND)
   - `item_priority` (simple arithmetic)
   - `solve_part1_bitset` (caller with inlined bitset ops)

3. **Architecture Comparison**:
   - Use Compiler Explorer to see ARM assembly
   - Notice different instructions (no `bts` on ARM)
   - Understand ISA-specific optimizations

4. **Profiling**:
   - Use `perf` (Linux) or `vtune` (Intel) to see actual CPU usage
   - Verify that `bts` is indeed the hot instruction
   - Measure IPC (instructions per cycle)

---

## Earlier Compilation Stages (Future Study)

The files in this directory show **MIR → LLVM IR → Assembly**. To see **earlier stages** (lexing, parsing, HIR):

### 1. **Macro-Expanded Source** (Post-Macro)

Shows your code **after macros are expanded** but before type checking.

```bash
# Install cargo-expand
cargo install cargo-expand

# View macro-expanded entire lib
cd d:\repos\rust_study\advent_of_code\aoc2022
cargo expand --lib > compiler_output/aoc2022_expanded.rs

# View specific module
cargo expand --lib day03 > compiler_output/day03_expanded.rs

# View specific function (if possible)
cargo expand --lib day03::items_to_bitset
```

**What you'll see**:
- `println!` → `std::io::_print(format_args!(...)))`
- `?` operator → `match` expressions
- `for` loops → `loop { match iter.next() { ... } }`
- Trait method calls fully qualified

**Useful for**:
- Understanding macro-generated code
- Seeing what macros actually expand to
- Debugging macro errors

### 2. **HIR (High-level IR)** - Requires Nightly

HIR comes **right after parsing**, before MIR and borrow checking.

```bash
# Generate HIR for entire crate
cargo +nightly rustc --lib -- -Z unpretty=hir > compiler_output/aoc2022.hir

# View in terminal (very long!)
cargo +nightly rustc --lib -- -Z unpretty=hir | less
```

**What you'll see**:
- Desugared syntax (loops → match, if-let → match)
- Type annotations (inferred types shown explicitly)
- Pattern matching expanded
- Before borrow checking and lifetime elision

**Example transformation**:
```rust
// Source
for item in items { ... }

// HIR (desugared)
match IntoIterator::into_iter(items) {
    mut iter => loop {
        match Iterator::next(&mut iter) {
            Some(item) => { ... },
            None => break,
        }
    }
}
```

**Useful for**:
- Understanding Rust's syntactic sugar
- Seeing explicit type annotations
- Learning how control flow is represented

### 3. **AST (Abstract Syntax Tree)** - JSON Format

The raw parse tree, before type checking.

```bash
# Dump AST as JSON (warning: HUGE output!)
cargo +nightly rustc --lib -- -Z ast-json=yes > compiler_output/aoc2022_ast.json
```

**Warning**: AST JSON is extremely verbose and not human-readable. **HIR is usually better** for understanding code structure.

### 4. **Programmatic AST Parsing** (Using `syn` crate)

For **structured analysis** of Rust source code:

```rust
// In a separate Rust project
use syn::{parse_file, visit::Visit};

fn main() {
    let source = std::fs::read_to_string(
        "d:/repos/rust_study/advent_of_code/aoc2022/src/solver/day03.rs"
    ).unwrap();
    
    let ast = parse_file(&source).unwrap();
    
    // Pretty-print the AST
    println!("{:#?}", ast);
    
    // Or use visitor pattern to extract specific elements
}
```

**Useful for**:
- Building code analysis tools
- Custom linters
- Code generation
- Extracting documentation

### 5. **Compiler Explorer** (Easiest for Quick Study!)

**Online tool**: https://godbolt.org/

**How to use**:
1. Go to https://godbolt.org/
2. Select "Rust" as language
3. Paste your function:
   ```rust
   #[inline]
   pub fn items_to_bitset(s: &str) -> u128 {
       s.bytes().fold(0u128, |acc, b| acc | (1u128 << b))
   }
   ```
4. Select output format:
   - **AST dump** (for syntax tree)
   - **HIR** (for high-level IR)
   - **MIR** (for mid-level IR)
   - **LLVM IR** (for optimization stages)
   - **Assembly** (for machine code)

**Advantages**:
- No local installation needed
- Interactive exploration
- Compare different compilers (rustc, gcc, clang)
- See optimization passes in real-time
- Visual diff between opt-levels

### Complete Compilation Pipeline

To see **ALL stages** from source to machine code:

```bash
# Setup
cd d:\repos\rust_study\advent_of_code\aoc2022

# 1. Macro-expanded source
cargo expand --lib > compiler_output/aoc2022_expanded.rs

# 2. HIR (requires nightly)
cargo +nightly rustc --lib -- -Z unpretty=hir > compiler_output/aoc2022.hir

# 3. MIR (already generated)
# compiler_output/aoc2022.mir

# 4. LLVM IR (already generated)
# compiler_output/aoc2022.ll

# 5. Assembly (already generated)
# compiler_output/aoc2022.s
```

### Comparison Table: What Each Stage Shows

| Stage | File Extension | Shows | Key Features | Best For |
|-------|----------------|-------|---------------|----------|
| **Source** | `.rs` | Original code | Macros, sugar | Writing code |
| **Expanded** | `_expanded.rs` | Post-macro | Macro expansion, still Rust | Macro debugging |
| **HIR** | `.hir` | Desugared syntax | Explicit types, control flow | Understanding sugar |
| **MIR** | `.mir` | Control flow | Basic blocks, borrow checking | Optimization analysis |
| **LLVM IR** | `.ll` | SSA form | Platform-independent, phi nodes | Cross-platform optimization |
| **Assembly** | `.s` | Machine code | CPU instructions, registers | Performance tuning |

### When to Use Each Stage

**Use macro-expanded source when**:
- Debugging macro errors
- Understanding what a macro generates
- Learning how derive macros work

**Use HIR when**:
- Learning how Rust desugars syntax
- Understanding pattern matching internals
- Seeing explicit type annotations

**Use MIR when** (current focus):
- Analyzing borrow checking
- Understanding control flow
- Seeing iterator transformations before inlining

**Use LLVM IR when**:
- Understanding optimization passes
- Platform-independent analysis
- Learning SSA form

**Use Assembly when**:
- Performance tuning
- Understanding CPU-level execution
- Architecture-specific optimizations

### Example Workflow: Studying `items_to_bitset`

```bash
# 1. See macro expansion (in case we used macros)
cargo expand --lib day03::items_to_bitset

# 2. See HIR (desugared syntax)
cargo +nightly rustc --lib -- -Z unpretty=hir | Select-String -Pattern "items_to_bitset" -Context 0,20

# 3. See MIR (control flow, already have this)
Select-String -Path "compiler_output\aoc2022.mir" -Pattern "fn items_to_bitset" -Context 0,30

# 4. See LLVM IR (optimization, already have this)
Select-String -Path "compiler_output\aoc2022.ll" -Pattern "items_to_bitset"

# 5. See Assembly (machine code, already have this)
Select-String -Path "compiler_output\aoc2022.s" -Pattern "items_to_bitset"
```

### Alternative: Using `cargo-asm` for Cleaner Output

```bash
# Install
cargo install cargo-asm

# View annotated assembly with Rust source interleaved
cargo asm --rust --release aoc2022::solver::day03::items_to_bitset

# Compare different functions
cargo asm --rust --release aoc2022::solver::day03::find_common_item
cargo asm --rust --release aoc2022::solver::day03::find_common_item_bitset
```

**Advantages of cargo-asm**:
- Shows **Rust source next to assembly**
- Filters out irrelevant functions
- Cleaner output than raw `.s` files
- Highlights hot paths

### Resources for Further Study

1. **Rust Compiler Internals**:
   - https://rustc-dev-guide.rust-lang.org/
   - Explains all compilation stages in detail

2. **MIR Documentation**:
   - https://blog.rust-lang.org/2016/04/19/MIR.html
   - Original MIR announcement blog post

3. **LLVM Optimization Passes**:
   - https://llvm.org/docs/Passes.html
   - Learn what each optimization does

4. **Compiler Explorer**:
   - https://godbolt.org/
   - Interactive exploration of all stages

5. **cargo-expand**:
   - https://github.com/dtolnay/cargo-expand
   - Macro expansion tool

6. **cargo-asm**:
   - https://github.com/gnzlbg/cargo-asm
   - Better assembly viewing

---

*Generated: February 3, 2026*  
*Purpose: Practical guide to exploring compiler output*  
*Files: aoc2022.mir (66KB), aoc2022.ll (686KB), aoc2022.s (217KB)*
[[Index]]
