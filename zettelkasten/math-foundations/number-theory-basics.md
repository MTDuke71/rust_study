# Number Theory Basics

**Field**: Discrete Mathematics / Number Theory

**Prerequisites**: Basic arithmetic, divisibility

---

## 📐 Definition

**Number theory** is the study of integers and their properties, including divisibility, prime numbers, and modular arithmetic.

**Intuition**: Understanding how numbers relate to each other through division, remainders, and patterns.

---

## 🔑 Key Concepts

### **Divisibility**
- **Statement**: Integer `a` divides `b` (written `a | b`) if there exists an integer `k` such that `b = a × k`
- **Example**: 3 | 12 because 12 = 3 × 4

### **Greatest Common Divisor (GCD)**
- **Statement**: The GCD of two integers is the largest positive integer that divides both numbers
- **Algorithm**: Euclidean algorithm - efficient O(log min(a,b)) computation
- **Properties**:
  - gcd(a, b) = gcd(b, a mod b)
  - gcd(a, 0) = a
  - Always exists and is unique

### **Least Common Multiple (LCM)**
- **Statement**: The LCM of two integers is the smallest positive integer that is divisible by both numbers
- **Formula**: lcm(a, b) = (a × b) / gcd(a, b)
- **Applications**: Finding when cycles align, synchronizing periodic events

### **Modular Arithmetic**
- **Statement**: `a ≡ b (mod n)` means `a` and `b` have the same remainder when divided by `n`
- **Properties**: 
  - (a + b) mod n = ((a mod n) + (b mod n)) mod n
  - (a × b) mod n = ((a mod n) × (b mod n)) mod n
- **Applications**: Cyclic patterns, hashing, cryptography

---

## 💻 Rust Implementations

### **AoC 2023 Day 8**: Haunted Wasteland (Ghost Navigation)
- **What**: Navigate multiple ghost paths through a network simultaneously
- **How it uses this concept**: 
  - Each ghost follows a cyclic pattern with specific period
  - Find when all cycles align using LCM of cycle lengths
  - Avoids simulating 8+ trillion steps
- **Link**: [[advent_of_code/aoc2023/src/solver/day08.rs]]
- **Performance**: ~6.7ms (Part 2) vs. impossibly long brute force

### **AoC 2023 Day 20**: Pulse Propagation (Counter Synchronization)
- **What**: Digital logic circuit with flip-flops, conjunctions, and broadcasters
- **How it uses this concept**:
  - 4 independent counter modules each have periodic behavior
  - Need to find when all counters simultaneously output high pulse
  - Use cycle detection to find each counter's period (4000-5000 iterations)
  - Compute LCM of periods to find synchronization point
  - Avoids simulating 238 trillion button presses
- **Mathematical pattern**: Identical to Day 8 (cycle alignment via LCM)
- **Link**: [advent_of_code/aoc2023/src/solver/day20.rs](advent_of_code/aoc2023/src/solver/day20.rs)
- **Shared utilities**: `math_utils.rs` module with `gcd()` and `lcm()` functions
- **Performance**: ~23.54ms (Part 2) using cycle detection + LCM
- **See also**: [[state-machines]] for state machine implementation details

---

## 📚 Code Examples

### Euclidean Algorithm (GCD)
```rust
/// Calculate Greatest Common Divisor using Euclidean algorithm
/// Time complexity: O(log min(a, b))
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;  // Key insight: gcd(a,b) = gcd(b, a mod b)
        a = temp;
    }
    a
}
```

**Mathematical foundation**: 
- Based on: gcd(a, b) = gcd(b, a mod b)
- Terminates when b = 0, returning a
- Each iteration reduces problem size logarithmically

### Least Common Multiple
```rust
/// Calculate Least Common Multiple using GCD
/// Formula: lcm(a, b) = (a × b) / gcd(a, b)
fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
```

**Mathematical foundation**:
- Relationship: a × b = gcd(a, b) × lcm(a, b)
- Avoids overflow by dividing before multiplying when possible
- Used to find cycle alignment in periodic processes

### Modular Arithmetic (Cycle Wrapping)
```rust
// Wrap index cyclically through instructions
instruction_idx = (instruction_idx + 1) % instructions.len();
```

**Mathematical foundation**:
- Implements cyclic behavior: after reaching end, wrap to beginning
- `i mod n` gives remainder in range [0, n-1]
- Common pattern in circular buffers, repeating sequences

---

## 🌳 Related Concepts

- **Prerequisites**: Basic arithmetic, divisibility
- **Related**: 
  - [[graph-theory-fundamentals]] - Cycle detection in graphs
  - [[complexity-theory]] - Algorithm efficiency analysis
- **Applications**: 
  - Cryptography (RSA uses modular exponentiation)
  - Hashing (modular arithmetic for index calculation)
  - Cycle detection (finding periodic patterns)

---

## 📖 Resources

- [Wikipedia: Number Theory](https://en.wikipedia.org/wiki/Number_theory)
- [Wikipedia: Euclidean Algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm)
- [Art of Computer Programming, Vol 2: Seminumerical Algorithms](https://www-cs-faculty.stanford.edu/~knuth/taocp.html)

---

*Tags: #number-theory #gcd #lcm #modular-arithmetic #euclidean-algorithm #math-foundations*

**Related Zettelkasten Links**:
- [[set-theory-fundamentals]] - Foundational discrete math
- [[quadratic-equations]] - Another applied math foundation
