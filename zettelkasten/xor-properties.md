# XOR Properties & Applications

*Tags: #bit-manipulation #algorithms #optimization #branchless #math-foundations #xor*  
*Links: [[zettel-index]] | [[Algorithms MOC]] | [[sliding-window-patterns]] | [[aoc-optimization-strategies]] | [[set-theory-fundamentals]]*

---

## 🎯 Overview

**XOR (exclusive or)** is a bitwise operation that returns 1 when exactly one of two bits is 1. Written as `^` in Rust. Its mathematical properties — self-inverse, commutative, associative — make it uniquely powerful for problems involving toggling, pairing, cancellation, and branchless state management.

**Core truth table**:
```
A ^ B:
  0 ^ 0 = 0    (same → 0)
  0 ^ 1 = 1    (different → 1)
  1 ^ 0 = 1    (different → 1)
  1 ^ 1 = 0    (same → 0)
```

**One-sentence insight**: XOR tells you "are these different?" at the bit level.

---

## 🔍 Fundamental Properties

### The Big Three

| Property | Formula | Why It Matters |
|----------|---------|----------------|
| **Self-inverse** | `a ^ b ^ b == a` | Undo an operation by repeating it (toggle semantics) |
| **Commutative** | `a ^ b == b ^ a` | Order doesn't matter |
| **Associative** | `(a ^ b) ^ c == a ^ (b ^ c)` | Grouping doesn't matter |

### Derived Properties

| Property | Formula | Application |
|----------|---------|-------------|
| **Identity** | `a ^ 0 == a` | XOR with zero is no-op |
| **Self-cancellation** | `a ^ a == 0` | Any value XORed with itself vanishes |
| **Involution** | `(a ^ b) ^ b == a` | XOR is its own inverse — the key to everything |
| **Bit-flip** | `a ^ mask` flips selected bits | Toggle specific bits without branches |
| **No-carry addition** | `a ^ b` = addition without carries | Foundation of binary arithmetic |

### The Self-Inverse Property (Most Important)

This single property enables most XOR applications:

```
XOR in:   state ^= value    →  bit toggled ON
XOR out:  state ^= value    →  bit toggled OFF (back to original)

It's like a light switch: flip once = ON, flip again = OFF.
```

**In Rust**:
```rust
let mut state: u32 = 0;
state ^= 1 << 5;   // bit 5 ON   → state = 0b0010_0000
state ^= 1 << 3;   // bit 3 ON   → state = 0b0010_1000
state ^= 1 << 5;   // bit 5 OFF  → state = 0b0000_1000 (back to just bit 3)
```

---

## 💡 Application Patterns

### Pattern 1: Rolling Window State (AoC 2022 Day 6)

**Problem**: Detect when the last `w` characters in a stream are all unique.

**XOR solution**: Toggle each character's bit in a u32. When the window slides, XOR out the leaving character and XOR in the entering character. If `popcount == w`, all characters are unique.

```rust
fn find_unique_window(input: &[u8], w: usize) -> usize {
    let mut bits: u32 = 0;
    for (i, &b) in input.iter().enumerate() {
        bits ^= 1 << (b - b'a');                    // XOR in entering
        if i >= w {
            bits ^= 1 << (input[i - w] - b'a');     // XOR out leaving
        }
        if i >= w - 1 && bits.count_ones() as usize == w {
            return i + 1;
        }
    }
    panic!("No marker found");
}
```

**Why it works**: With exactly `w` characters in the window:
- Each unique character sets its bit ON (odd count = 1)
- A duplicate character toggles its bit OFF (even count = 0), then another duplicate toggles it ON again
- `popcount == w` means `w` distinct bits ON → `w` characters each appearing an odd number of times → exactly once (since we have `w` total)

**Why XOR beats alternatives here**:
| Approach | Per-slide work | Branches | State size |
|----------|---------------|----------|------------|
| Rebuild bitset (OR) | w OR ops | 0 | u32 |
| Frequency counter | 2 array ops | 4 branches | [u8; 26] |
| **XOR bitset** | **2 XOR ops** | **0** | **u32** |

See [[sliding-window-patterns]] for the full optimization journey.

### Pattern 2: Find the Single Unique Element

**Problem**: Array where every element appears twice except one. Find it.

```rust
fn find_unique(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &x| acc ^ x)
}

// [2, 3, 5, 3, 2] → 2^3^5^3^2 = (2^2)^(3^3)^5 = 0^0^5 = 5
```

**Why**: Pairs cancel (`a ^ a == 0`), leaving only the unpaired element. O(n) time, O(1) space — no HashSet needed.

### Pattern 3: Swap Without Temporary

```rust
fn xor_swap(a: &mut i32, b: &mut i32) {
    *a ^= *b;   // a = a ^ b
    *b ^= *a;   // b = b ^ (a ^ b) = a
    *a ^= *b;   // a = (a ^ b) ^ a = b
}
```

**Why**: Self-inverse property chains — each step undoes part of the entanglement. (Note: in practice, `std::mem::swap` is preferred for clarity and handles `a == b` case.)

### Pattern 4: PRNG / Mixing (AoC 2024 Day 17)

XOR is commonly used in pseudo-random number generators for bit mixing:

```rust
fn prng_step(mut state: u64) -> u64 {
    state ^= state << 13;   // Mix bits left
    state ^= state >> 7;    // Mix bits right  
    state ^= state << 17;   // Mix bits left again
    state
}
```

**Why**: XOR preserves entropy — it never loses information (invertible), unlike AND/OR which can only set/clear bits.

### Pattern 5: Hamming Distance

Count differing bit positions between two values:

```rust
fn hamming_distance(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}
// XOR produces 1-bits exactly where a and b differ
```

### Pattern 6: Bit Toggling / Branchless Selection

Toggle specific bits without conditional logic:

```rust
// Flip bit at position p
fn toggle_bit(value: u32, p: u32) -> u32 {
    value ^ (1 << p)
}

// Conditional flip: only toggle if condition is true
fn conditional_toggle(value: u32, mask: u32, condition: bool) -> u32 {
    value ^ (mask * condition as u32)
}
```

---

## 🧠 Mental Models

### The Light Switch
XOR is a light switch — flip once turns ON, flip again turns OFF. You don't need to know the current state to change it. This is why it's perfect for "toggle" operations where you track *changes* rather than *absolute state*.

### The Difference Detector
`a ^ b` produces 1-bits exactly where `a` and `b` differ. This makes XOR the fundamental "what changed?" operation. Hamming distance, error detection, and diff operations all use this property.

### The Pairing Canceller
When you XOR a set of values, everything that appears an even number of times vanishes. Only odd-count values survive. This is why "find the unique element" works — the pairs cancel themselves out.

### The Integration Perspective
XOR is a **stateless toggle component**: it doesn't need to know what the current state is, it just flips. Compare with a frequency counter that needs to *read* the current count, *decide* (branch), then *write* the new count. XOR skips the read-decide step entirely — it's the ultimate branchless state update.

---

## 📊 XOR vs Alternative Approaches

### For Set Membership / Uniqueness

| Approach | State | Add | Remove | Check Unique | Branches |
|----------|-------|-----|--------|-------------|----------|
| HashSet | Heap-allocated | O(1) avg | O(1) avg | `.len()` | Many (hash, resize) |
| Frequency array | [u8; N] | `freq[x] += 1` | `freq[x] -= 1` | Scan or track count | 2+ per op |
| OR bitset | u32/u128 | `\|= bit` | **Cannot remove** | `popcount` | 0 |
| **XOR bitset** | **u32/u128** | **`^= bit`** | **`^= bit`** | **`popcount`** | **0** |

Key insight: OR bitset can't remove (OR is not self-inverse: `a | b | b == a | b ≠ a`). XOR can because it IS self-inverse.

### When XOR Doesn't Work

| Limitation | Why | Alternative |
|-----------|-----|-------------|
| Character appears 3+ times | Odd count ≠ unique count | Frequency counter |
| Need to know exact count | XOR only gives parity | Frequency array |
| Unbounded domain | Can't map to bit positions | HashSet |
| Order matters | XOR is commutative | Sequence tracking |

**Important**: XOR bitset for uniqueness only works when:
1. Each element maps to a unique bit position (bounded domain)
2. Window size ≤ bit width (w ≤ 32 for u32)
3. We only need uniqueness (all distinct), not arbitrary frequency checks

---

## 🏗️ AoC Applications

| Problem | XOR Application | Key Property Used |
|---------|----------------|-------------------|
| **AoC 2022 Day 6** | Rolling window uniqueness | Self-inverse (toggle in/out) |
| **AoC 2024 Day 17** | VM XOR instruction, PRNG mixing | Entropy preservation |
| **AoC PRNG problems** | Xorshift generators | Invertible bit mixing |
| **Find unique in array** | Fold XOR over elements | Pair cancellation |
| **Hamming distance** | XOR + popcount | Difference detection |

---

## ⚡ Rust-Specific Notes

### Operator and Methods
```rust
let a: u32 = 0b1010;
let b: u32 = 0b1100;

a ^ b           // XOR: 0b0110
a ^= b;         // XOR-assign (in-place toggle)
(a ^ b).count_ones()  // Popcount of XOR result (Hamming distance)
```

### Hardware Support
- `count_ones()` compiles to x86 `POPCNT` — single cycle
- XOR itself is a single-cycle instruction on all architectures
- No branch prediction penalty — the CPU pipeline stays full

### Branchless is Key for Performance
In tight loops, branches cause pipeline stalls when mispredicted. XOR eliminates branches entirely:

```rust
// Branchy (frequency counter): 4 branches per slide
if freq[entering] == 0 { unique += 1; }
freq[entering] += 1;
if freq[entering] == 2 { unique -= 1; }

// Branchless (XOR): 0 branches per slide
bits ^= 1 << (entering - b'a');
```

This is a constant-factor improvement, but it compounds in hot inner loops. AoC 2022 Day 6 showed 20% improvement from branch elimination alone.

---

## 💡 Key Takeaways

1. **Self-inverse is the superpower**: `a ^ b ^ b == a`. This one property enables rolling windows, swap-without-temp, pair cancellation, and toggle semantics.

2. **XOR is branchless**: Two XOR ops replace multiple conditional branches. In tight inner loops, this 20%+ improvement is free.

3. **XOR preserves entropy**: Unlike AND (can only clear bits) or OR (can only set bits), XOR is invertible — no information is lost. This makes it ideal for hashing and mixing.

4. **Know the limitation**: XOR bitset only proves uniqueness when the domain is bounded and elements map 1:1 to bit positions. For general frequency tracking, use a frequency array.

5. **XOR + popcount is a building block**: Hamming distance, uniqueness detection, set symmetric difference — many problems reduce to "XOR then count the 1-bits."

---

## 🔗 Integration Points

### **Builds On**
- [[set-theory-fundamentals]] - XOR relates to symmetric difference in set theory
- [[Algorithm Analysis]] - Understanding O(1) vs O(w) per-slide analysis

### **Enables**
- [[sliding-window-patterns]] - Rolling XOR bitset is the fastest fixed-window uniqueness check
- [[aoc-optimization-strategies]] - Branchless bit manipulation for AoC performance

### **Related Concepts**
- [[Amortized Analysis]] - XOR in Hamming distance / bit-diffing context
- [[string-processing-patterns]] - Character set operations using XOR

---

*Tags: #bit-manipulation #algorithms #optimization #branchless #math-foundations #xor*

*Links: [[zettel-index]] | [[Algorithms MOC]] | [[sliding-window-patterns]] | [[aoc-optimization-strategies]] | [[set-theory-fundamentals]]*
