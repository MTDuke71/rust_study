# Modular Arithmetic

**Concept**: Arithmetic system where numbers "wrap around" after reaching a modulus value.

**Created**: 2026-01-21  
**Tags**: #mathematics #number-theory #modular-arithmetic #euclidean-division

---

## Definition

**Modular Congruence**: 
```
a ≡ b (mod n)   means   n divides (a - b)
```

**Equivalently**: a and b have the same remainder when divided by n.

**Examples**:
```
17 ≡ 5 (mod 12)   because  17 = 1×12 + 5, 5 = 0×12 + 5
-3 ≡ 8 (mod 11)   because  -3 = -1×11 + 8
```

---

## Division Algorithms

### Euclidean Division (Standard)

**Definition**: For integers a, n (n > 0):
```
a = qn + r   where  0 ≤ r < n
```

**Properties**:
- Quotient q and remainder r are **unique**
- Remainder **always non-negative**: 0 ≤ r < n
- Works for **negative** a too!

**Examples**:
```
 17 ÷ 5:  q=3,  r=2   (17 = 3×5 + 2) ✓
-17 ÷ 5:  q=-4, r=3   (-17 = -4×5 + 3) ✓  Note: r=3, not -2!
```

### Truncated Division (Programming Default)

**Definition**: Round quotient toward zero.

**Examples**:
```
 17 / 5 = 3.4  → q=3,  r=2   (17 = 3×5 + 2) ✓
-17 / 5 = -3.4 → q=-3, r=-2  (-17 = -3×5 + -2) ❌ Negative remainder!
```

**Problem**: Remainder can be negative, violating 0 ≤ r < n.

---

## Rust Implementation

### Standard Modulo (`%` operator)

Rust's `%` uses **truncated division** (rounds toward zero).

```rust
assert_eq!(17 % 5, 2);    // ✓ Positive case works
assert_eq!(-3 % 11, -3);  // ❌ Negative result!

// This breaks grid wrapping:
let row = -3_isize;
let grid_size = 11_isize;
let grid_row = row % grid_size;  // -3, not 8!
```

**Use case**: When remainder sign matches dividend sign.

### Euclidean Modulo (`rem_euclid()`)

Implements proper Euclidean division (0 ≤ r < n).

```rust
assert_eq!(17_isize.rem_euclid(5), 2);   // ✓ 17 = 3×5 + 2
assert_eq!(-3_isize.rem_euclid(11), 8);  // ✓ -3 = -1×11 + 8
assert_eq!(-17_isize.rem_euclid(5), 3);  // ✓ -17 = -4×5 + 3

// Grid wrapping works correctly:
let row = -3_isize;
let grid_size = 11_isize;
let grid_row = row.rem_euclid(grid_size);  // 8 ✓
```

**Use case**: Infinite grid wrapping, cyclic indexing, mathematical correctness.

---

## Modular Arithmetic Properties

### Basic Operations

**Addition**:
```
(a + b) mod n = ((a mod n) + (b mod n)) mod n
```

**Subtraction**:
```
(a - b) mod n = ((a mod n) - (b mod n) + n) mod n
```
(+n ensures non-negative result)

**Multiplication**:
```
(a × b) mod n = ((a mod n) × (b mod n)) mod n
```

**Exponentiation** (modular exponentiation):
```
aᵇ mod n  computed efficiently via repeated squaring
```

**Division**: NOT generally defined! (unless gcd(a,n)=1)

### Useful Identities

**Distributive**:
```
(a + b) mod n = ((a mod n) + (b mod n)) mod n
a(b mod n) ≡ ab (mod n)
```

**Inverse** (when gcd(a,n)=1):
```
ax ≡ 1 (mod n)  has unique solution x (modular inverse)
```

**Fermat's Little Theorem** (n prime):
```
aⁿ⁻¹ ≡ 1 (mod n)  if gcd(a,n)=1
```

---

## Applications

### Infinite Grid Wrapping (AoC 2023 Day 21)

**Problem**: Map infinite 2D coordinates to finite grid tiles.

**Grid**: 131×131, repeats infinitely in all directions.

**Coordinates**: Can be negative (e.g., walk left from row 0 → row -1).

**Solution**: Euclidean modulo!

```rust
fn map_to_grid(infinite_row: isize, infinite_col: isize, grid_size: isize) 
    -> (usize, usize) 
{
    let grid_row = infinite_row.rem_euclid(grid_size) as usize;
    let grid_col = infinite_col.rem_euclid(grid_size) as usize;
    (grid_row, grid_col)
}

// Examples:
map_to_grid(5, 7, 131)     → (5, 7)      // Inside grid
map_to_grid(132, 7, 131)   → (1, 7)      // One tile right
map_to_grid(-3, 5, 131)    → (128, 5)    // One tile left (wraps!)
map_to_grid(5, -10, 131)   → (5, 121)    // One tile up (wraps!)
```

**Why `rem_euclid`?**:
- Position (-3, 5) should map to (128, 5), NOT (-3, 5)
- Standard `%` gives -3, breaking array indexing
- `rem_euclid` gives 128 = 131 - 3 ✓

**See**: [[aoc2023-day21]], [[infinite-grid-patterns]]

### Cyclic Buffers

**Problem**: Circular array with wrapping index.

```rust
struct CircularBuffer<T> {
    data: Vec<T>,
    write_pos: usize,
}

impl<T> CircularBuffer<T> {
    fn push(&mut self, item: T) {
        let idx = self.write_pos % self.data.len();
        self.data[idx] = item;
        self.write_pos += 1;
    }
}
```

**Why standard `%` works here**: `write_pos` always non-negative!

### Cryptography

**RSA Encryption**:
```
Encrypt: c = mᵉ mod n
Decrypt: m = cᵈ mod n
```

Relies on modular exponentiation and modular inverses.

### Hash Functions

**Remainder Hash**:
```rust
fn hash(key: i32, table_size: usize) -> usize {
    key.abs() as usize % table_size
}
```

Maps arbitrary integers to valid array indices.

### Clock Arithmetic

**12-hour clock**: 
```
(10 + 5) mod 12 = 3   (10am + 5 hours = 3pm)
```

**Day of week** (7-day cycle):
```
(Monday + 10 days) mod 7 = Thursday
```

---

## Common Pitfalls

### Negative Remainders with `%`

**Wrong**:
```rust
let idx = -5_isize % 10;  // idx = -5, not 5!
arr[idx as usize];        // PANIC: can't cast negative to usize
```

**Right**:
```rust
let idx = -5_isize.rem_euclid(10);  // idx = 5 ✓
arr[idx as usize];                   // arr[5] ✓
```

### Division by Zero

**Both `%` and `rem_euclid` panic if divisor is zero!**

```rust
// Panics at runtime:
let r = 10 % 0;
let r = 10_isize.rem_euclid(0);
```

**Defense**: Check divisor before operation.

### Integer Overflow

**Multiplication can overflow before modulo!**

**Wrong**:
```rust
let result = (a * b) % n;  // a*b might overflow!
```

**Right** (for u64/i64):
```rust
// Use u128 for intermediate result
let result = ((a as u128 * b as u128) % n as u128) as u64;

// Or modular multiplication:
let result = ((a % n) * (b % n)) % n;
```

---

## Rust Language Specifics

### Available Methods

**Integer types** (i8, i16, i32, i64, i128, isize):
```rust
.rem_euclid(n)   // Euclidean remainder: 0 ≤ r < |n|
.div_euclid(n)   // Euclidean quotient
```

**Unsigned types** (u8, u16, u32, u64, u128, usize):
```rust
x % n            // Standard modulo (always non-negative anyway)
.rem_euclid(n)   // Same as % for unsigned
```

### Performance

**Both `%` and `rem_euclid` compile to CPU instructions** (fast!).
- x86: `idiv` (signed) or `div` (unsigned)
- ARM: `sdiv`/`udiv`
- Modern CPUs: ~10-40 cycles

**Constant modulus**: Compiler can optimize to multiply + shift!
```rust
x % 16  →  x & 15  (bitwise AND, 1 cycle!)
```

---

## Mathematical Theory

### Congruence Classes

**Equivalence relation**: Partitions integers into n classes.

**Example** (mod 3):
```
[0] = {..., -6, -3, 0, 3, 6, ...}
[1] = {..., -5, -2, 1, 4, 7, ...}
[2] = {..., -4, -1, 2, 5, 8, ...}
```

**Operations**: Well-defined on classes.

### Modular Inverse

**Definition**: x is the inverse of a (mod n) if:
```
ax ≡ 1 (mod n)
```

**Existence**: Inverse exists ⟺ gcd(a, n) = 1

**Extended Euclidean Algorithm**: Computes inverse efficiently.

```rust
fn mod_inverse(a: i64, n: i64) -> Option<i64> {
    let (gcd, x, _) = extended_gcd(a, n);
    if gcd != 1 {
        None  // Inverse doesn't exist
    } else {
        Some(x.rem_euclid(n))
    }
}
```

### Chinese Remainder Theorem (CRT)

**Problem**: Solve system of congruences:
```
x ≡ a₁ (mod n₁)
x ≡ a₂ (mod n₂)
...
x ≡ aₖ (mod nₖ)
```

**Condition**: All nᵢ pairwise coprime (gcd(nᵢ, nⱼ) = 1 for i≠j).

**Solution**: Unique solution modulo N = n₁n₂...nₖ.

**See**: [[math-foundations/chinese-remainder-theorem]]

---

## Complexity

**Time**: O(1) - constant time (single CPU instruction)  
**Space**: O(1) - no extra memory

**Modular Exponentiation** (aᵇ mod n):
- Naive: O(b) multiplications
- Repeated squaring: O(log b) multiplications

---

## Related Concepts

**Number Theory**:
- [[math-foundations/divisibility]] - Basic properties
- [[math-foundations/gcd-and-lcm]] - Greatest common divisor
- [[math-foundations/prime-numbers]] - Fermat's theorem, inverses

**Data Structures**:
- [[circular-buffers]] - Ring buffers with modulo indexing
- [[hash-tables]] - Modulo for bucket selection

**AoC Applications**:
- [[aoc2023-day21]] - Infinite grid wrapping
- [[aoc2023-day08]] - Cycle detection (implicit modulo)
- [[fast-forward-patterns]] - Skip repetitions using modulo

---

## Zettelkasten Links

**Prerequisites**:
- [[math-foundations/division-algorithms]] - Euclidean vs truncated
- [[math-foundations/integer-arithmetic]] - Basic operations

**Applications**:
- [[aoc2023-day21]] - rem_euclid for infinite grid
- [[mission-cyclic-structures]] - When Mission uses modular indexing

**Advanced**:
- [[math-foundations/number-theory-basics]] - Broader context
- [[math-foundations/chinese-remainder-theorem]] - CRT and system of congruences
- [[cryptography-foundations]] - RSA, Diffie-Hellman

---

**References**:
- [Rust std::ops::Rem](https://doc.rust-lang.org/std/ops/trait.Rem.html)
- [Rust int::rem_euclid()](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid)
- [Euclidean Division](https://en.wikipedia.org/wiki/Euclidean_division)
- [AoC 2023 Day 21 Implementation](d:/repos/rust_study/advent_of_code/aoc2023/src/solver/day21.rs)
