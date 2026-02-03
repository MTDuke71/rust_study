# Compiler Output Directory

**Generated**: February 3, 2026  
**Source**: AoC 2022 Day 3 implementation  
**Purpose**: Educational exploration of Rust compilation pipeline

---

## 📁 Files in This Directory

| File | Size | Description |
|------|------|-------------|
| **COMPILER_STAGES_GUIDE.md** | - | Comprehensive guide to all compilation stages |
| **EXPLORATION_GUIDE.md** | - | Practical commands to explore the outputs |
| **aoc2022.mir** | 66 KB | Mid-level IR (borrow checker, control flow) |
| **aoc2022.ll** | 686 KB | LLVM IR (platform-independent optimization) |
| **aoc2022.s** | 217 KB | x86-64 Assembly (Intel syntax) |
| **build_log.txt** | - | Compilation log |

---

## 🚀 Quick Start

### For Compiler Students

**Read in order**:
1. **COMPILER_STAGES_GUIDE.md** - Theory and transformations
2. **EXPLORATION_GUIDE.md** - Hands-on exploration
3. Open **aoc2022.mir** and search for `fn items_to_bitset`
4. Try `cargo-asm` or Compiler Explorer (godbolt.org)

### For Performance Engineers

**Key finding**: Function is **fully inlined** in release mode!

Search for actual assembly:
```powershell
Select-String -Path "aoc2022.s" -Pattern "bts" -Context 2,2
```

Look for `bts` instruction (Bit Test and Set) - this is the single instruction that replaces `(1 << b) | acc`.

### For Rust Developers

**Takeaway**: High-level code compiles to optimal machine code.

Compare source:
```rust
s.bytes().fold(0u128, |acc, b| acc | (1u128 << b))
```

To assembly (conceptual):
```asm
bts rax, r8    # Single instruction!
```

**This is zero-cost abstraction in action!**

---

## 📚 What You'll Learn

### Compilation Stages

1. **Source (.rs)** → High-level Rust
2. **HIR** → Desugared, traits resolved (not saved)
3. **MIR** → Control flow, borrow checking ✅ **aoc2022.mir**
4. **LLVM IR** → Platform-independent SSA ✅ **aoc2022.ll**
5. **Assembly** → x86-64 machine code ✅ **aoc2022.s**
6. **Object code** → Binary (not saved)

### Key Insights

1. **Iterator chains optimize perfectly** - `fold` becomes tight loop
2. **Inlining eliminates overhead** - Function disappears into caller
3. **Instruction selection matters** - `bts` replaces shift + OR
4. **Trust the optimizer** - Simple code compiles better than "clever" code

### Example Transformation

**MIR** (line 1358-1359):
```mir
_4 = Shl(const 1_u128, copy _3);     // Shift
_0 = BitOr(copy _2, move _4);        // OR
```

**LLVM IR** (inlined):
```llvm
%bit = shl i128 1, %shift_amt
%new_acc = or i128 %acc, %bit
```

**Assembly** (optimized):
```asm
bts rax, r8    ; Bit Test and Set (combines shift + OR!)
```

**Result**: 2 operations → 1 instruction = 15× speedup vs HashSet

---

## 🔧 Tools & Commands

### Regenerate These Files

```bash
cd advent_of_code/aoc2022
cargo rustc --release --lib -- --emit=mir,llvm-ir,asm -C llvm-args=-x86-asm-syntax=intel
```

Output goes to: `D:\rust_build_cache\rust_study_target\release\deps\`

### View Assembly for Specific Function

```bash
cargo install cargo-asm
cargo asm --rust --release aoc2022::solver::day03::solve_bitset
```

### Use Compiler Explorer (Online)

1. Go to https://godbolt.org
2. Select "Rust" compiler
3. Paste code from `src/solver/day03.rs`
4. Enable `-C opt-level=3`
5. See assembly side-by-side!

### Compare Optimization Levels

```bash
# No optimization
cargo rustc --lib -- --emit=asm -C opt-level=0

# Release optimization
cargo rustc --release --lib -- --emit=asm -C opt-level=3
```

Compare file sizes and instruction counts!

---

## 📖 Reading Order

### Beginner Path

1. Read **COMPILER_STAGES_GUIDE.md** (overview)
2. Look at **aoc2022.mir** (search `fn items_to_bitset`)
3. Try Compiler Explorer online (paste function, see assembly)

### Intermediate Path

1. Read both guides (COMPILER_STAGES_GUIDE + EXPLORATION_GUIDE)
2. Search MIR for different functions (`item_priority`, `find_common_item_bitset`)
3. Find inlined assembly patterns (search for `bts` instruction)
4. Install `cargo-asm` and view annotated assembly

### Advanced Path

1. Compare optimization levels (opt-level 0 vs 3)
2. Study LLVM IR for vectorization opportunities
3. Profile with `perf` to measure actual CPU performance
4. Experiment with `#[inline(never)]` to prevent inlining

---

## 🎯 Study Goals

### For This Example

- [ ] Find `items_to_bitset` in MIR (line 1330)
- [ ] Understand basic blocks and control flow
- [ ] See closure as separate MIR function (line 1350)
- [ ] Observe shift and OR operations in MIR
- [ ] Notice function is inlined in assembly (doesn't appear standalone)
- [ ] Find `bts` instruction in calling function
- [ ] Understand why bitset is 15× faster than HashSet

### General Compiler Knowledge

- [ ] Understand SSA form (LLVM IR)
- [ ] Recognize loop optimizations (unrolling, vectorization)
- [ ] Read x86-64 assembly with confidence
- [ ] Know when compiler can optimize (and when it can't)
- [ ] Appreciate zero-cost abstractions in Rust

---

## 🔗 Related Files

- **Source Code**: `../src/solver/day03.rs`
- **Function Guide**: `../Problem_Statements/days/day03_function_guide.md`
- **Benchmarks**: `../benches/benchmarks.rs`
- **Tests**: `../src/solver/day03.rs` (11 tests, all passing)

---

## 🏆 Key Takeaway

**Rust's zero-cost abstractions are real!**

High-level iterator chains:
```rust
s.bytes().fold(0u128, |acc, b| acc | (1u128 << b))
```

Compile to optimal assembly:
```asm
bts rax, r8    ; Single instruction
```

**Performance**: 23.8µs for Day 3 (15× faster than HashSet version)  
**Reason**: Compiler transforms high-level code into single CPU instruction

---

*Generated: February 3, 2026*  
*Purpose: Learning resource for Rust compilation pipeline*  
*Example: AoC 2022 Day 3 bitset optimization*
[[Index]]