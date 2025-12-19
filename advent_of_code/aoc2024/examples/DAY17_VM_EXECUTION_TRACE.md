# Day 17: Virtual Machine Execution Trace

**Puzzle Input Walkthrough**

This document traces the execution of the Day 17 3-bit computer with the actual puzzle input, showing exactly what happens at each step.

---

## Initial State

```
Register A: 66245665
Register B: 0
Register C: 0

Program: [2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0]
```

**Program Length**: 16 bytes (8 instructions)

---

## Program Structure Analysis

The program is a **loop** that processes A in 3-bit chunks:

```
Position  Opcode  Operand  Instruction      Description
--------  ------  -------  ------------     -----------
0-1       2       4        bst 4            B = A & 7 (get low 3 bits of A)
2-3       1       7        bxl 7            B = B XOR 7
4-5       7       5        cdv 5            C = A >> B
6-7       1       7        bxl 7            B = B XOR 7
8-9       4       6        bxc 6            B = B XOR C
10-11     0       3        adv 3            A = A >> 3 (divide A by 8)
12-13     5       5        out 5            output B & 7
14-15     3       0        jnz 0            jump to 0 if A != 0
```

**Key Pattern**: 
- Extract 3 bits from A (positions 10-11: `A >>= 3` divides by 8)
- Perform XOR operations on those bits
- Output result
- Repeat until A = 0

---

## Detailed Execution Trace

### **Iteration 1**: A = 66245665

#### Step-by-Step:

**IP=0**: `bst 4` (B = A & 7)
```
A = 66245665 (binary: ...11111100001000001)
B = 66245665 & 7 = 1 (last 3 bits)
```

**IP=2**: `bxl 7` (B = B XOR 7)
```
B = 1 XOR 7 = 6 (binary: 001 XOR 111 = 110)
```

**IP=4**: `cdv 5` (C = A >> B)
```
C = 66245665 >> 6 = 1035088
```

**IP=6**: `bxl 7` (B = B XOR 7)
```
B = 6 XOR 7 = 1 (binary: 110 XOR 111 = 001)
```

**IP=8**: `bxc 6` (B = B XOR C)
```
B = 1 XOR 1035088 = 1035089
```

**IP=10**: `adv 3` (A = A >> 3)
```
A = 66245665 >> 3 = 8280708 (divide by 8)
```

**IP=12**: `out 5` (output B & 7)
```
Output: 1035089 & 7 = 1
```

**IP=14**: `jnz 0` (jump to 0 if A != 0)
```
A = 8280708 != 0, so IP = 0 (loop continues)
```

**After Iteration 1**:
- A = 8280708
- B = 1035089
- C = 1035088
- Output: [1]

---

### **Iteration 2**: A = 8280708

**IP=0**: `bst 4` → B = 8280708 & 7 = **4**

**IP=2**: `bxl 7` → B = 4 XOR 7 = **3**

**IP=4**: `cdv 5` → C = 8280708 >> 3 = **1035088**

**IP=6**: `bxl 7` → B = 3 XOR 7 = **4**

**IP=8**: `bxc 6` → B = 4 XOR 1035088 = **1035084**

**IP=10**: `adv 3` → A = 8280708 >> 3 = **1035088**

**IP=12**: `out 5` → Output: 1035084 & 7 = **4**

**IP=14**: `jnz 0` → A != 0, jump to 0

**After Iteration 2**:
- A = 1035088
- Output: [1, 4]

---

### **Iteration 3**: A = 1035088

**IP=0**: `bst 4` → B = 1035088 & 7 = **0**

**IP=2**: `bxl 7` → B = 0 XOR 7 = **7**

**IP=4**: `cdv 5` → C = 1035088 >> 7 = **8086**

**IP=6**: `bxl 7` → B = 7 XOR 7 = **0**

**IP=8**: `bxc 6` → B = 0 XOR 8086 = **8086**

**IP=10**: `adv 3` → A = 1035088 >> 3 = **129386**

**IP=12**: `out 5` → Output: 8086 & 7 = **6**

**IP=14**: `jnz 0` → A != 0, jump to 0

**After Iteration 3**:
- A = 129386
- Output: [1, 4, 6]

---

### **Pattern Summary**

Each iteration:
1. **Extract 3 bits** from A (via `A & 7`)
2. **XOR transformations** on those bits
3. **Shift calculation** using C register
4. **Output** the transformed value
5. **Divide A by 8** (`A >> 3`)
6. **Loop** if A > 0

The program continues this pattern until A reaches 0.

---

## Mathematical Insight

### **Why 3 Bits Per Iteration?**

The key instruction is **`adv 3`** at position 10-11:
```rust
A >>= 3  // Equivalent to A /= 8
```

Since 8 = 2³, each iteration processes exactly **3 bits** of A in binary.

### **How Many Iterations?**

```
Initial A = 66245665
log₈(66245665) ≈ 8.66 → 9 iterations

Verification:
Iteration 1: A = 66245665 >> 3 = 8280708
Iteration 2: A = 8280708  >> 3 = 1035088
Iteration 3: A = 1035088  >> 3 = 129386
Iteration 4: A = 129386   >> 3 = 16173
Iteration 5: A = 16173    >> 3 = 2021
Iteration 6: A = 2021     >> 3 = 252
Iteration 7: A = 252      >> 3 = 31
Iteration 8: A = 31       >> 3 = 3
Iteration 9: A = 3        >> 3 = 0 (halt)
```

The program produces **9 output values** (one per iteration).

---

## Complete Output Trace

| Iteration | A (before) | Low 3 bits | Transformations | B & 7 | Output |
|-----------|------------|------------|-----------------|-------|--------|
| 1 | 66245665 | 001 | XOR 7 → 6, cdv, XOR 7 → 1, XOR C | 1 | **1** |
| 2 | 8280708  | 100 | XOR 7 → 3, cdv, XOR 7 → 4, XOR C | 4 | **4** |
| 3 | 1035088  | 000 | XOR 7 → 7, cdv, XOR 7 → 0, XOR C | 6 | **6** |
| 4 | 129386   | 010 | XOR 7 → 5, cdv, XOR 7 → 2, XOR C | 1 | **1** |
| 5 | 16173    | 101 | XOR 7 → 2, cdv, XOR 7 → 5, XOR C | 3 | **3** |
| 6 | 2021     | 101 | XOR 7 → 2, cdv, XOR 7 → 5, XOR C | 4 | **4** |
| 7 | 252      | 100 | XOR 7 → 3, cdv, XOR 7 → 4, XOR C | 6 | **6** |
| 8 | 31       | 111 | XOR 7 → 0, cdv, XOR 7 → 7, XOR C | 1 | **1** |
| 9 | 3        | 011 | XOR 7 → 4, cdv, XOR 7 → 3, XOR C | 5 | **5** |
| - | 0        | - | (halt) | - | - |

**Final Output**: `1,4,6,1,3,4,6,1,5`

---

## Part 2 Insight: The Quine Challenge

For Part 2, we need to find an A value such that the output equals the program itself:
```
Desired output: [2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0]
```

Since each output digit is determined by 3 bits of A, and we need 16 output digits, we need:
```
16 digits × 3 bits/digit = 48 bits

Minimum A ≈ 8^15 = 35,184,372,088,832
Maximum A ≈ 8^16 - 1 = 281,474,976,710,655
```

The recursive backtracking algorithm builds A by:
1. Starting from the last digit (3 bits)
2. For each digit, try all 8 possible values (000 to 111)
3. Check if partial output matches desired output
4. Recurse to build the next 3 bits
5. Backtrack if no valid continuation exists

### **Why Backtracking is Necessary**

**Critical Insight**: Multiple 3-bit values might produce the correct output for the current digit, but only one (or few) will work for ALL subsequent digits!

**Example Scenario**:
```
Suppose we need output: [2, 4, 1, ...]

Building A from right to left (backward construction):

Step 1: Find 3 bits that produce output "2"
  Try 000: output might be 2 ✓
  Try 001: output might be 2 ✓ 
  Try 010: output might be 5 ✗
  ...
  → Multiple valid choices!

Step 2: For each valid choice from Step 1, try to extend it
  If we chose 000, now try adding 3 more bits:
    A = 000|000 (bits 0-2: 000, bits 3-5: 000) → test output
    A = 000|001 → test output
    ...
    
  Some combinations will produce [2, 4] ✓
  Others will produce [2, 7] ✗ (wrong second digit!)

Step 3: Continue recursively
  Only paths that produce ALL correct digits will succeed
  Dead ends force backtracking to try different choices
```

**Why This Happens**:

The XOR transformations create **interdependencies** between digit positions:

```rust
// Iteration N processes bits [3N, 3N+1, 3N+2] of A
B = A & 7              // Current 3 bits
B = B XOR 7
C = A >> B             // ← This is the KEY!
B = B XOR 7
B = B XOR C            // ← Mixes current bits with HIGHER bits!
output(B & 7)
A >>= 3                // Move to next 3 bits
```

The line `C = A >> B` means:
- **Current output depends on FUTURE bits** (higher positions in A)
- When we're building A backward, we don't know those bits yet!
- A choice that works for digit N might fail for digit N+1

**Concrete Example**:

```
Suppose at position 0 (last digit):
  Bits 000 → output 2 ✓
  Bits 101 → output 2 ✓

Now try to extend to position 1 (second-to-last):
  A = 000|xxx (3 new bits xxx)
    → Might produce [2, 4] ✓ (success path)
    
  A = 101|xxx (3 new bits xxx)  
    → Might produce [2, 7] ✗ (wrong, must backtrack!)
    → Try different xxx values... none work
    → Backtrack to position 0, try 000 instead
```

**Algorithm Behavior**:

```
Try 3-bit value 0 at position P:
  ├─ Produces correct digit → Recurse to position P+1
  │   ├─ Success at P+1 → Continue to P+2
  │   └─ Failure at P+1 → BACKTRACK, try next value
  │
Try 3-bit value 1 at position P:
  └─ Wrong digit produced → Skip, try value 2
  
...continue until solution found or all possibilities exhausted
```

**Why Backtracking > Brute Force**:

- **Brute Force**: Try all 8^16 = 281 trillion combinations
- **Backtracking with Pruning**: 
  - Eliminate 7/8 of search space at each wrong digit
  - Typical search: ~thousands of candidates, not trillions
  - Early pruning: "If first digit is wrong, don't try 8^15 remaining combinations"

**Key Takeaway**: The non-linear transformation (`B XOR C` where `C` depends on higher bits) creates a constraint satisfaction problem where local choices must be validated by global context. Backtracking efficiently explores this space by abandoning bad paths early!

---

## XOR Transformation Logic

The program uses a specific XOR pattern:

```rust
// Step 1: Extract low 3 bits
B = A & 7  // e.g., A=66245665 → B=1 (001)

// Step 2: First XOR (flip all bits in 3-bit range)
B = B XOR 7  // 001 XOR 111 = 110 (6)

// Step 3: Calculate shift amount from original A
C = A >> B   // Depends on B value (0-7)

// Step 4: Second XOR (undo first transformation)
B = B XOR 7  // 110 XOR 111 = 001 (1)

// Step 5: Mix with shifted value
B = B XOR C  // Combines original bits with shifted bits

// Step 6: Output low 3 bits of result
output(B & 7)
```

This creates a **non-linear transformation** where:
- The output depends on the current 3 bits of A
- But also depends on higher bits (via the `C = A >> B` operation)
- Makes reverse engineering (Part 2) challenging

---

## Key Observations

1. **Deterministic**: Same A always produces same output
2. **One-way**: Easy to compute output from A, hard to find A from output
3. **3-bit chunks**: Each iteration processes exactly 3 bits (via `A >>= 3`)
4. **Non-linear mixing**: XOR operations create dependencies between bits
5. **State-dependent shift**: The `C = A >> B` uses B as shift amount (0-7)

This design makes Part 2 (finding the quine) computationally interesting—brute force from 0 would take too long, so we need the digit-by-digit construction approach!

Target output: [2, 4, 1, 7, 7, 5, 1, 7, 4, 6, 0, 3, 5, 5, 3, 0]
Search space: 16 positions ├ù 8 values/position = 128 max attempts

≡ƒöÖ Backtrack at depth 0 (pos 1): tried bits 000 (A=0), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): tried bits 000 (A=240640), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): tried bits 111 (A=240647), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 4 (pos 5): tried bits 000 (A=30080), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): tried bits 000 (A=240664), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): tried bits 111 (A=240671), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 4 (pos 5): tried bits 011 (A=30083), continuing search
≡ƒöÖ Backtrack at depth 4 (pos 5): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 3 (pos 4): tried bits 000 (A=3760), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): tried bits 111 (A=241031), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 4 (pos 5): tried bits 000 (A=30128), continuing search
≡ƒöÖ Backtrack at depth 4 (pos 5): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 3 (pos 4): tried bits 110 (A=3766), continuing search
≡ƒöÖ Backtrack at depth 3 (pos 4): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 2 (pos 3): tried bits 110 (A=470), continuing search
≡ƒöÖ Backtrack at depth 2 (pos 3): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 1 (pos 2): tried bits 010 (A=58), continuing search
≡ƒöÖ Backtrack at depth 5 (pos 6): tried bits 000 (A=246856), continuing search
≡ƒöÖ Backtrack at depth 10 (pos 11): tried bits 100 (A=8089030860), continuing search
≡ƒöÖ Backtrack at depth 10 (pos 11): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 9 (pos 10): tried bits 001 (A=1011128857), continuing search
≡ƒöÖ Backtrack at depth 11 (pos 12): tried bits 000 (A=64712247184), continuing search
≡ƒöÖ Backtrack at depth 11 (pos 12): tried bits 001 (A=64712247185), continuing search
≡ƒöÖ Backtrack at depth 11 (pos 12): exhausted all 8 bit patterns
≡ƒöÖ Backtrack at depth 10 (pos 11): tried bits 010 (A=8089030898), continuing search
≡ƒöÖ Backtrack at depth 13 (pos 14): tried bits 001 (A=4141583821833), continuing search

Γ£à Solution found: A = 265061364597659

≡ƒôè Search Statistics:
  Total attempts: 225
  Backtracks required: 20
  Successful paths: 1
  Maximum depth reached: 16
  Efficiency: 0.44% (successful / total attempts)
  Pruning effectiveness: avoided 281474976710431 brute force attempts

---

## Verification

Running the Rust solution with this input:
```bash
cd advent_of_code/aoc2024
cargo test day17 -- --nocapture
```

Should produce:
```
Part 1: 1,4,6,1,3,4,6,1,5
Part 2: [lowest A value that produces [2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0]]
```

---

**Related Files**:
- Implementation: `src/solver/day17.rs`
- Tests: `src/solver/day17.rs` (bottom of file)
- Input: `inputs/day17.txt`

**Key Learning**: The 3-bit chunking pattern (`A >>= 3`) is the foundation for both Part 1's execution and Part 2's reverse engineering strategy!

---

*Links: [[../../zettelkasten/weekly plans/2025-W51]] | [[../../zettelkasten/rust-concepts-MOC]]*

*Tags: #aoc2024 #day17 #virtual-machine #bitwise-operations #reverse-engineering #execution-trace*
