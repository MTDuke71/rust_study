# Chinese Remainder Theorem (CRT)

**Category**: Number Theory, Modular Arithmetic, Congruences  
**Difficulty**: Intermediate

## Definition

The **Chinese Remainder Theorem** (CRT) states that if you have a system of simultaneous congruences with **pairwise coprime** moduli, there exists a unique solution modulo the product of all moduli.

### Formal Statement

Given:
- Pairwise coprime integers m₁, m₂, ..., mₖ (i.e., gcd(mᵢ, mⱼ) = 1 for all i ≠ j)
- Arbitrary integers a₁, a₂, ..., aₖ

Then the system of congruences:
```
x ≡ a₁ (mod m₁)
x ≡ a₂ (mod m₂)
...
x ≡ aₖ (mod mₖ)
```

Has a **unique solution** modulo M = m₁ × m₂ × ... × mₖ.

### Simple Example

**Problem**: Find x such that:
```
x ≡ 2 (mod 3)
x ≡ 3 (mod 5)
x ≡ 2 (mod 7)
```

**Solution**: x = 23 (mod 105)
- Check: 23 % 3 = 2 ✓
- Check: 23 % 5 = 3 ✓
- Check: 23 % 7 = 2 ✓
- Modulus: 3 × 5 × 7 = 105

All solutions: 23, 128, 233, ... (adding multiples of 105)

## History and Etymology

**Origin**: Ancient Chinese mathematical text *Sunzi Suanjing* (孫子算經, Master Sun's Mathematical Manual), circa 3rd-5th century AD.

**Original Problem** (Master Sun's problem):
> "There are things of unknown number. When divided by 3, the remainder is 2; when divided by 5, the remainder is 3; when divided by 7, the remainder is 2. What is the number?"

**Historical significance**: One of the earliest recorded applications of modular arithmetic, predating European number theory by over 1000 years.

**Modern name**: Coined in the 19th century when the theorem was studied and generalized by Western mathematicians.

## Construction of Solution

### Algorithm (For Coprime Moduli)

Given system: x ≡ aᵢ (mod mᵢ) for i = 1, 2, ..., k

**Step 1**: Compute M = m₁ × m₂ × ... × mₖ (product of all moduli)

**Step 2**: For each i, compute Mᵢ = M / mᵢ (product of all moduli except mᵢ)

**Step 3**: For each i, find yᵢ such that Mᵢ × yᵢ ≡ 1 (mod mᵢ)  
(This is the modular inverse of Mᵢ modulo mᵢ)

**Step 4**: Solution is:
```
x = (a₁M₁y₁ + a₂M₂y₂ + ... + aₖMₖyₖ) mod M
```

### Why It Works

**Key insight**: Each term aᵢMᵢyᵢ is:
- ≡ aᵢ (mod mᵢ) because Mᵢyᵢ ≡ 1 (mod mᵢ)
- ≡ 0 (mod mⱼ) for j ≠ i because Mᵢ contains mⱼ as a factor

So when we sum all terms, for each modulus mᵢ, only one term contributes: aᵢ.

## Practical Applications

### 1. Calendar Calculations

**Example**: Chinese zodiac cycles (12-year) and stem cycles (10-year) combine to create a 60-year cycle.

If year is:
- Year 3 in 12-year cycle
- Year 7 in 10-year cycle

Then year is 43 in the 60-year cycle (by CRT).

### 2. Memory-Efficient Large Number Arithmetic

**Idea**: Represent large numbers by their remainders modulo several small primes.

```rust
// Instead of storing huge number directly:
let n = 123456789012345678901234567890_u128;  // May overflow!

// Store as remainders:
let r_mod_7 = n % 7;
let r_mod_11 = n % 11;
let r_mod_13 = n % 13;
// ... etc

// Can reconstruct n mod (7 × 11 × 13 × ...) using CRT
```

### 3. Competitive Programming: Bounding Simulation Values

**Pattern**: When simulation generates unbounded growth but only divisibility matters.

**Key property**: If we only test divisibility by d₁, d₂, ..., dₖ, we can work modulo M = d₁ × d₂ × ... × dₖ.

**Why**: `(a % M) % dᵢ == a % dᵢ` for all dᵢ that divide M.

This is an **informal application** of CRT - we don't need to solve congruences, just use the product of moduli to bound values.

## AoC 2022 Day 11 Application

### Problem Context

**Scenario**: Simulate monkeys passing items for 10,000 rounds
- Each monkey applies operation (add/multiply) to worry level
- Tests divisibility by specific number
- WITHOUT division by 3 for "relief"

**Challenge**: Worry levels grow exponentially → overflow after ~10-15 rounds

### CRT Solution (Informal)

**Observation**: Only care about divisibility tests, not exact worry values.

**Moduli**: Monkey divisibility tests use d₁ = 11, d₂ = 19, d₃ = 5, d₄ = 3, d₅ = 13, d₆ = 17, d₇ = 7, d₈ = 2

**Product**: M = 11 × 19 × 5 × 3 × 13 × 17 × 7 × 2 = 9,699,690

**Implementation**:
```rust
// Product of all monkey divisors
let modulo: u64 = game.monkeys
    .iter()
    .map(|m| m.divisible_by)
    .product();

// After each operation, reduce modulo M
new_worry %= modulo;  // Keeps worry < 10 million
```

**Why it works**:
- Test: `worry % 23 == 0` (example divisor)
- Property: `(worry % M) % 23 == worry % 23` (since 23 divides M)
- All divisibility tests preserved while keeping worry bounded

**Result**: 10,000 rounds run efficiently without overflow ✅

**Note**: Not a "true" CRT application (not solving congruences), but uses the CRT principle that remainders are preserved modulo the product of divisors.

### Mathematical Justification

**Theorem**: If d divides M, then for any integer a:
```
(a mod M) mod d = a mod d
```

**Proof**:
1. Let a = qM + r where 0 ≤ r < M (by division algorithm)
2. Then a mod M = r
3. Since d | M, we have M = kd for some integer k
4. So a = q(kd) + r = (qk)d + r
5. Thus a mod d = r mod d
6. And (a mod M) mod d = r mod d = a mod d ∎

**Application to Day 11**:
- M = product of all divisors
- Each test divisor dᵢ divides M (by construction)
- So reducing worry modulo M preserves all divisibility tests

## Extensions and Generalizations

### Non-Coprime Moduli

**Problem**: What if moduli are NOT coprime?

**Answer**: System has solution **if and only if**:
```
aᵢ ≡ aⱼ (mod gcd(mᵢ, mⱼ))
```
for all pairs i, j.

If solution exists, it's unique modulo lcm(m₁, m₂, ..., mₖ).

**Example where solution exists**:
```
x ≡ 2 (mod 4)
x ≡ 6 (mod 8)
```
gcd(4, 8) = 4, and 2 ≡ 6 (mod 4) ✓ → solution exists: x = 6 (mod 8)

**Example with no solution**:
```
x ≡ 1 (mod 4)
x ≡ 3 (mod 8)
```
gcd(4, 8) = 4, but 1 ≢ 3 (mod 4) ✗ → no solution

### Efficient Construction (Iterative)

For **two** moduli m₁, m₂:
```rust
fn solve_two(a1: i64, m1: i64, a2: i64, m2: i64) -> Option<i64> {
    // Extended Euclidean algorithm
    let (gcd, x, _y) = extended_gcd(m1, m2);
    
    if (a2 - a1) % gcd != 0 {
        return None;  // No solution
    }
    
    let lcm = m1 * m2 / gcd;
    let solution = (a1 + m1 * x * ((a2 - a1) / gcd)) % lcm;
    Some((solution + lcm) % lcm)  // Ensure positive
}
```

For **multiple** moduli, apply iteratively:
```rust
fn solve_crt(remainders: &[i64], moduli: &[i64]) -> Option<i64> {
    assert_eq!(remainders.len(), moduli.len());
    
    let mut result = remainders[0];
    let mut m = moduli[0];
    
    for i in 1..remainders.len() {
        result = solve_two(result, m, remainders[i], moduli[i])?;
        m = lcm(m, moduli[i]);
    }
    
    Some(result)
}
```

## Rust Implementation Patterns

### Basic CRT (Coprime Moduli)

```rust
/// Extended Euclidean Algorithm
/// Returns (gcd, x, y) where gcd = a*x + b*y
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

/// Modular multiplicative inverse of a modulo m
/// Returns b such that (a * b) % m == 1
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1 {
        None  // Inverse doesn't exist
    } else {
        Some((x % m + m) % m)  // Ensure positive
    }
}

/// Solve CRT for coprime moduli
fn chinese_remainder_theorem(remainders: &[i64], moduli: &[i64]) -> Option<i64> {
    assert_eq!(remainders.len(), moduli.len());
    
    // Product of all moduli
    let m: i64 = moduli.iter().product();
    
    let mut result = 0;
    
    for i in 0..remainders.len() {
        let mi = m / moduli[i];
        let yi = mod_inverse(mi, moduli[i])?;
        result += remainders[i] * mi * yi;
    }
    
    Some((result % m + m) % m)  // Ensure positive
}
```

### AoC Pattern (Bounding with Product)

```rust
/// Simple product-of-divisors bounding (AoC Day 11 pattern)
fn bounded_simulation_modulo(divisors: &[u64]) -> u64 {
    // Product of all divisors
    divisors.iter().product()
}

// Usage:
let modulo = bounded_simulation_modulo(&[11, 19, 5, 3, 13, 17, 7, 2]);
// modulo = 9,699,690

// In simulation loop:
worry = apply_operation(worry);
worry %= modulo;  // Keep bounded while preserving divisibility
```

## Common Mistakes

### 1. ❌ Assuming Moduli Are Coprime

```rust
// WRONG: Assumes all moduli are coprime
let modulo: u64 = moduli.iter().product();
```

**Fix**: Check coprimality or use LCM:
```rust
// CORRECT: Use LCM for potentially non-coprime moduli
let modulo = moduli.iter().fold(1, |acc, &m| lcm(acc, m));
```

### 2. ❌ Negative Remainders

```rust
// WRONG: May produce negative result
let result = (a1 * m1_inv * m1 + a2 * m2_inv * m2) % m;
```

**Fix**: Ensure positive:
```rust
// CORRECT: Add m to handle negative values
let result = ((a1 * m1_inv * m1 + a2 * m2_inv * m2) % m + m) % m;
```

### 3. ❌ Overflow in Product Calculation

```rust
// WRONG: Product may overflow u64
let m: u64 = moduli.iter().product();
```

**Fix**: Use checked arithmetic or u128:
```rust
// CORRECT: Use u128 for large products
let m: u128 = moduli.iter().map(|&x| x as u128).product();
```

## Performance Considerations

### Time Complexity

- **Construction**: O(k²) for k congruences using extended GCD
- **Iterative method**: O(k log²M) where M is largest modulus
- **Simple bounding** (AoC pattern): O(k) for product calculation

### Space Complexity

- O(k) for storing moduli and remainders
- O(1) for iterative construction

### Optimization: Precompute Moduli Product

```rust
struct CRTSolver {
    moduli: Vec<u64>,
    product: u64,
}

impl CRTSolver {
    fn new(moduli: Vec<u64>) -> Self {
        let product = moduli.iter().product();
        Self { moduli, product }
    }
    
    fn reduce(&self, value: u64) -> u64 {
        value % self.product
    }
}

// Usage in simulation:
let solver = CRTSolver::new(divisors);
// ... in loop:
worry = solver.reduce(apply_operation(worry));
```

## Related AoC Problems

### 2020 Day 13: Shuttle Search (Part 2)

**Problem**: Find earliest timestamp where buses depart at specific offsets.

**System**:
```
t ≡ 0 (mod 7)
t ≡ -1 (mod 13)
t ≡ -4 (mod 59)
...
```

**Solution**: Direct CRT application with coprime moduli (bus IDs are primes).

### 2022 Day 11: Monkey in the Middle (Part 2)

**Problem**: Bound exponentially growing values while preserving divisibility tests.

**Solution**: Informal CRT - use product of divisors as modulo.

### General Pattern

**When to use CRT in AoC**:
1. Multiple modular constraints (explicit CRT)
2. Divisibility tests with bounded values (informal CRT)
3. Cycle detection with different periods (Chinese remainder theorem)
4. Calendar/rotation problems with different cycle lengths

## References

- *Elementary Number Theory* - David Burton (Ch. 7)
- *An Introduction to the Theory of Numbers* - Hardy & Wright (Ch. 5)
- *Concrete Mathematics* - Graham, Knuth, Patashnik (§4.3)
- [[modular-arithmetic]] - Foundation for CRT
- [[extended-euclidean-algorithm]] - Key component for construction
- [[number-theory-fundamentals]] - General number theory background

---

*Links:*
- **Related Theory**: [[modular-arithmetic]], [[number-theory-fundamentals]], [[divisibility]], [[coprime-numbers]]
- **Algorithms**: [[extended-euclidean-algorithm]], [[modular-inverse]]
- **Applications**: [[advent_of_code/aoc2022/src/solver/day11.rs]], [[aoc-2020-day13]]
- **Related Problems**: [[aoc-2022-day11]], [[cycle-detection]], [[calendar-arithmetic]]
- **Daily Notes**: [[zettelkasten/Daily Notes/2026-02-11]] - Day 11 session
- **Function Guide**: [[advent_of_code/aoc2022/Problem_Statements/days/day11_function_guide.md]]

*Tags:* #number-theory #modular-arithmetic #chinese-remainder-theorem #congruences #crt #mathematics #competitive-programming #aoc-pattern
