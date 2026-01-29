# Pythagorean Triplets

**Category**: Number Theory, Geometry  
**Difficulty**: Intermediate  
**Applications**: Geometry, cryptography, Diophantine equations  
**Related**: [[number-theory-basics]], [[diophantine-equations]], [[euclidean-algorithm]]

## Definition

A **Pythagorean triplet** (or Pythagorean triple) is a set of three positive integers $a, b, c$ such that:

$$a^2 + b^2 = c^2$$

This represents the side lengths of a right triangle where $a$ and $b$ are the legs (catheti) and $c$ is the hypotenuse.

**Ordering Convention**: Typically written with $a < b < c$ or $a \leq b < c$

## Examples

### Classic Triplets

**Smallest Pythagorean triplet**:
- $(3, 4, 5)$: $3^2 + 4^2 = 9 + 16 = 25 = 5^2$ ✓

**Common triplets**:
- $(5, 12, 13)$: $25 + 144 = 169$ ✓
- $(8, 15, 17)$: $64 + 225 = 289$ ✓
- $(7, 24, 25)$: $49 + 576 = 625$ ✓
- $(20, 21, 29)$: $400 + 441 = 841$ ✓
- $(9, 40, 41)$: $81 + 1600 = 1681$ ✓
- $(12, 35, 37)$: $144 + 1225 = 1369$ ✓
- $(11, 60, 61)$: $121 + 3600 = 3721$ ✓
- $(13, 84, 85)$: $169 + 7056 = 7225$ ✓

### From [[project-euler-p009]]

**Target sum = 1000**:
- $(200, 375, 425)$: $200^2 + 375^2 = 40000 + 140625 = 180625 = 425^2$ ✓
- Sum: $200 + 375 + 425 = 1000$ ✓
- Product: $200 \times 375 \times 425 = 31,875,000$

## Types of Pythagorean Triplets

### Primitive Triplets

**Definition**: A Pythagorean triplet $(a, b, c)$ is **primitive** if $\gcd(a, b, c) = 1$ (the three numbers have no common factor greater than 1).

**Examples**:
- ✅ $(3, 4, 5)$ - primitive ($\gcd = 1$)
- ✅ $(5, 12, 13)$ - primitive ($\gcd = 1$)
- ❌ $(6, 8, 10)$ - NOT primitive ($\gcd = 2$, it's $2 \times (3, 4, 5)$)
- ❌ $(9, 12, 15)$ - NOT primitive ($\gcd = 3$, it's $3 \times (3, 4, 5)$)

**Property**: All Pythagorean triplets can be derived by scaling primitive triplets:
$$(ka, kb, kc) \text{ for any positive integer } k$$

### Non-Primitive (Scaled) Triplets

**Definition**: Multiples of primitive triplets.

**Example**: From primitive $(3, 4, 5)$:
- $k=2$: $(6, 8, 10)$
- $k=3$: $(9, 12, 15)$
- $k=4$: $(12, 16, 20)$
- $k=100$: $(300, 400, 500)$

**Implication**: If you can generate all primitive triplets, you can generate **all** triplets by scaling.

## Euclid's Formula

**The fundamental theorem** for generating Pythagorean triplets.

### Formula

For any two positive integers $m > n$ where:
- $\gcd(m, n) = 1$ (coprime)
- $m$ and $n$ are not both odd

The following generates a **primitive** Pythagorean triplet:

$$\boxed{a = m^2 - n^2, \quad b = 2mn, \quad c = m^2 + n^2}$$

All **non-primitive** triplets are then:

$$\boxed{a = k(m^2 - n^2), \quad b = k(2mn), \quad c = k(m^2 + n^2)}$$

for any positive integer $k$.

### Proof of Euclid's Formula

**Claim**: If $a = m^2 - n^2$, $b = 2mn$, $c = m^2 + n^2$, then $a^2 + b^2 = c^2$.

**Proof**:
$$a^2 + b^2 = (m^2 - n^2)^2 + (2mn)^2$$
$$= m^4 - 2m^2n^2 + n^4 + 4m^2n^2$$
$$= m^4 + 2m^2n^2 + n^4$$
$$= (m^2 + n^2)^2 = c^2$$ ✓

**Converse (harder)**: Every primitive Pythagorean triplet arises this way.

### Examples Using Euclid's Formula

**Example 1**: $m=2, n=1$ (coprime, not both odd)
- $a = 2^2 - 1^2 = 4 - 1 = 3$
- $b = 2 \cdot 2 \cdot 1 = 4$
- $c = 2^2 + 1^2 = 4 + 1 = 5$
- **Result**: $(3, 4, 5)$ ✓

**Example 2**: $m=3, n=2$ (coprime, not both odd)
- $a = 3^2 - 2^2 = 9 - 4 = 5$
- $b = 2 \cdot 3 \cdot 2 = 12$
- $c = 3^2 + 2^2 = 9 + 4 = 13$
- **Result**: $(5, 12, 13)$ ✓

**Example 3**: $m=4, n=1$ (coprime, not both odd)
- $a = 4^2 - 1^2 = 16 - 1 = 15$
- $b = 2 \cdot 4 \cdot 1 = 8$
- $c = 4^2 + 1^2 = 16 + 1 = 17$
- **Result**: $(8, 15, 17)$ (note: $a$ and $b$ are swapped from standard order) ✓

**Example 4**: $m=4, n=3$ (coprime, not both odd)
- $a = 4^2 - 3^2 = 16 - 9 = 7$
- $b = 2 \cdot 4 \cdot 3 = 24$
- $c = 4^2 + 3^2 = 16 + 9 = 25$
- **Result**: $(7, 24, 25)$ ✓

**Example 5 (scaled)**: $m=2, n=1, k=100$ 
- $a = 100(2^2 - 1^2) = 100 \cdot 3 = 300$
- $b = 100(2 \cdot 2 \cdot 1) = 100 \cdot 4 = 400$
- $c = 100(2^2 + 1^2) = 100 \cdot 5 = 500$
- **Result**: $(300, 400, 500)$ ✓

### Conditions Explained

**Why $m > n$?**
- Ensures $a = m^2 - n^2 > 0$ (positive integer)

**Why coprime $\gcd(m,n) = 1$?**
- Ensures the triplet is **primitive**
- If $\gcd(m,n) = d > 1$, the resulting triplet would have $\gcd = d^2$

**Why not both odd?**
- If both $m$ and $n$ are odd: $a = m^2 - n^2$ is even, $b = 2mn$ is even, $c = m^2 + n^2$ is even
- This means $\gcd \geq 2$, violating primitivity
- **Allowed combinations**:
  - $m$ even, $n$ odd ✓
  - $m$ odd, $n$ even ✓
  - ❌ Both odd (non-primitive)
  - ❌ Both even (violates coprimality)

### Completeness Theorem

**Theorem**: Euclid's formula generates **all primitive** Pythagorean triplets exactly once (up to order of $a$ and $b$).

**Implication**: To find all Pythagorean triplets, iterate over valid $(m, n)$ pairs and scale by $k$.

## Alternative Generation Methods

### Method 1: Brute Force Search

**Algorithm**: Iterate through all possible $(a, b)$ and check if $a^2 + b^2$ is a perfect square.

```rust
for a in 1..limit {
    for b in a..limit {
        let c_squared = a * a + b * b;
        let c = (c_squared as f64).sqrt() as u32;
        if c * c == c_squared {
            // Found triplet (a, b, c)
        }
    }
}
```

**Complexity**: $O(n^2)$ where $n$ is the search limit

**Used in**: [[project-euler-p009]] (sufficient for small bounds)

### Method 2: Constrained Search (Optimization)

When searching for triplets with a specific sum $S = a + b + c$:

**Bounds**:
- Since $a < b < c$ and $a + b + c = S$:
  - $a < S/3$ (if $a$ were $S/3$, then $b$ and $c$ would be at least $S/3$ each)
  - For given $a$: $b < (S - a)/2$ (since $b < c$ and $b + c = S - a$)
  - $c = S - a - b$ (determined by constraint)

**Algorithm**:
```rust
for a in 1..target / 3 {
    for b in (a + 1)..=(target - a) / 2 {
        let c = target - a - b;
        if a * a + b * b == c * c {
            return (a, b, c);
        }
    }
}
```

**Complexity**: Still $O(n^2)$ but with tighter bounds

**Advantage**: Exploits problem constraints to reduce search space

### Method 3: Euclid's Formula (Most Efficient)

**Algorithm**: Generate triplets using $(m, n)$ parameters

```rust
for m in 2..=max_m {
    for n in 1..m {
        if gcd(m, n) == 1 && (m % 2 != n % 2) {
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            // Found primitive triplet (a, b, c)
            
            // Generate scaled versions
            for k in 1..max_k {
                // (k*a, k*b, k*c)
            }
        }
    }
}
```

**Complexity**: $O(m^2)$ for primitive triplets, $O(m^2 \times k)$ with scaling

**Advantage**: 
- Generates only valid triplets (no checking needed)
- Systematic generation
- Efficient for large-scale enumeration

## Mathematical Properties

### Parity Properties

**Theorem**: In any primitive Pythagorean triplet:
1. Exactly one of $a, b$ is even
2. Exactly one of $a, b$ is divisible by 3
3. Exactly one of $a, b, c$ is divisible by 5

**Proof** (for property 1):
- From Euclid's formula with coprime $m, n$ not both odd:
  - One of $m, n$ is even, the other odd
  - $a = m^2 - n^2$ is odd (odd² - even² or even² - odd²)
  - $b = 2mn$ is even (contains factor 2)
  - $c = m^2 + n^2$ is odd (odd² + even²)

**Example verification**: $(3, 4, 5)$
- Even: 4 ✓ (exactly one)
- Divisible by 3: 3 ✓ (exactly one)
- Divisible by 5: 5 ✓ (exactly one)

### Counting Primitive Triplets

**Question**: How many primitive Pythagorean triplets have $c \leq N$?

**Answer**: Approximately $\frac{N}{2\pi^2} \approx 0.0507 \cdot N$

**Asymptotic behavior**: The number of primitive triplets with hypotenuse $\leq N$ grows linearly with $N$.

### Area of Pythagorean Triangles

**Theorem**: The area of a right triangle with Pythagorean triplet sides $(a, b, c)$ is:

$$A = \frac{1}{2}ab$$

**For primitive triplets** (Euclid's formula):
$$A = \frac{1}{2}(m^2 - n^2)(2mn) = mn(m^2 - n^2)$$

**Property**: The area of a primitive Pythagorean triangle is always divisible by 6.

**Proof**: 
- Either $m$ or $n$ is even → area divisible by 2
- Either $m^2 - n^2$ or $2mn$ is divisible by 3 (number theory argument)
- Therefore area divisible by $\text{lcm}(2, 3) = 6$

### Perimeter of Pythagorean Triangles

**Perimeter**: $P = a + b + c$

**For Euclid's formula**:
$$P = (m^2 - n^2) + 2mn + (m^2 + n^2) = 2m^2 + 2mn = 2m(m + n)$$

**Example**: $m=2, n=1$
- $P = 2 \cdot 2 \cdot (2 + 1) = 12$
- Triplet: $(3, 4, 5)$ with $3 + 4 + 5 = 12$ ✓

## Connection to Other Mathematics

### Complex Numbers

Pythagorean triplets correspond to factorizations in Gaussian integers:

$$c^2 = a^2 + b^2 = (a + bi)(a - bi)$$

where $i = \sqrt{-1}$.

### Rational Points on Circle

Pythagorean triplets $(a, b, c)$ correspond to **rational points** on the unit circle:

$$\left(\frac{a}{c}\right)^2 + \left(\frac{b}{c}\right)^2 = 1$$

**Example**: $(3, 4, 5)$ gives point $\left(\frac{3}{5}, \frac{4}{5}\right)$ on unit circle.

**Parametrization**: All rational points on the unit circle (except $(-1, 0)$) can be written as:

$$\left(\frac{1-t^2}{1+t^2}, \frac{2t}{1+t^2}\right)$$

for rational $t$. Setting $t = n/m$ recovers Euclid's formula.

### Fermat's Last Theorem

**Pythagorean equation**: $a^2 + b^2 = c^2$ has infinitely many integer solutions.

**Fermat's Last Theorem**: For $n > 2$, the equation $a^n + b^n = c^n$ has **no** positive integer solutions.

Proved by Andrew Wiles in 1995 after 358 years.

### Diophantine Equations

Pythagorean triplets are solutions to the **Diophantine equation** $x^2 + y^2 = z^2$.

This is one of the simplest non-trivial Diophantine equations with infinitely many solutions.

See: [[diophantine-equations]]

## Algorithms and Implementations

### Checking if Three Numbers Form a Triplet

```rust
fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    let (a, b, c) = if a > b { (b, a, c) } else { (a, b, c) };
    a * a + b * b == c * c
}
```

**Complexity**: $O(1)$

### Generating All Primitive Triplets up to Limit

```rust
fn generate_primitive_triplets(max_c: u32) -> Vec<(u32, u32, u32)> {
    let mut triplets = Vec::new();
    let max_m = (max_c as f64).sqrt() as u32 + 1;
    
    for m in 2..=max_m {
        for n in 1..m {
            if gcd(m, n) == 1 && (m % 2 != n % 2) {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                
                if c <= max_c {
                    triplets.push((a.min(b), a.max(b), c));
                }
            }
        }
    }
    
    triplets
}
```

**Complexity**: $O(\sqrt{N}^2) = O(N)$ where $N$ is max_c

### Finding Triplet with Specific Sum

**Problem**: Find $(a, b, c)$ such that $a + b + c = S$

**Approach 1 - Brute force** ([[project-euler-p009]]):
```rust
for a in 1..S / 3 {
    for b in (a + 1)..=(S - a) / 2 {
        let c = S - a - b;
        if a * a + b * b == c * c {
            return Some((a, b, c));
        }
    }
}
```

**Approach 2 - Euclid's formula**:
```rust
for m in 2..=(S as f64).sqrt() as u32 {
    for n in 1..m {
        if gcd(m, n) == 1 && (m % 2 != n % 2) {
            let p = 2 * m * (m + n); // Primitive perimeter
            
            if S % p == 0 {
                let k = S / p;
                let a = k * (m * m - n * n);
                let b = k * (2 * m * n);
                let c = k * (m * m + n * n);
                return Some((a, b, c));
            }
        }
    }
}
```

**Comparison**:
- Brute force: Simple, $O(S^2)$, works for small $S$
- Euclid: More complex, $O(\sqrt{S}^2) = O(S)$, better for large $S$

## Historical Context

### Ancient Origins

- **Babylonians** (c. 1800 BCE): Clay tablet Plimpton 322 lists 15 Pythagorean triplets
- **Pythagoras** (c. 570–495 BCE): Greek theorem relating sides of right triangle
- **Euclid** (c. 300 BCE): Formula for generating all primitive triplets in *Elements* Book X

### Modern Applications

**Cryptography**:
- RSA encryption uses properties of large numbers
- Related to factorization in rings of integers

**Computer Graphics**:
- Integer coordinates for exact geometric calculations
- Avoiding floating-point precision errors

**Network Routing**:
- Pythagorean distance in grid-based pathfinding

## Practice Problems

### Basic

1. **Verify triplets**: Which are valid Pythagorean triplets?
   - $(5, 12, 13)$ ✓
   - $(7, 24, 25)$ ✓
   - $(8, 9, 10)$ ❌
   - $(11, 60, 61)$ ✓

2. **Generate with Euclid**: Find triplets for $m=5, n=2$ and $m=5, n=4$

3. **Scaling**: Generate 5 non-primitive triplets from $(3, 4, 5)$

### Intermediate

4. **Count**: How many primitive triplets have $c \leq 100$?

5. **Sum constraint**: Find all triplets with $a + b + c = 120$

6. **Area**: Find the triplet with smallest area $> 1000$

### Advanced

7. **Prove**: Show that in any primitive triplet, $c$ is always odd

8. **Enumerate**: Generate all triplets with perimeter $\leq 1000$ using Euclid's formula

9. **Optimization**: Modify [[project-euler-p009]] to use Euclid's formula instead of brute force

## Summary

**Key Takeaways**:
1. Pythagorean triplets satisfy $a^2 + b^2 = c^2$
2. **Euclid's formula** generates all primitive triplets: $a = m^2 - n^2, b = 2mn, c = m^2 + n^2$
3. All triplets obtained by scaling primitive ones
4. Conditions: $m > n$, coprime, not both odd → primitive triplet
5. Applications in geometry, number theory, and computer science

**Generation Methods**:
- ✅ **Brute force**: Simple, $O(n^2)$, good for small problems
- ✅ **Constrained search**: Optimized bounds, still $O(n^2)$
- ✅ **Euclid's formula**: Systematic, $O(n)$, generates all triplets

**When to use each**:
- Small fixed constraint (e.g., sum = 1000): Brute force sufficient
- Enumerate all triplets up to limit: Euclid's formula
- Theoretical analysis: Euclid's formula for exact characterization

## References

### Implementations
- [[project-euler-p009]] - Finding triplet with sum = 1000
- `project_euler/src/problems/p009.rs` - Rust implementation

### Related Concepts
- [[number-theory-basics]] - Fundamental number theory
- [[diophantine-equations]] - Integer equation solutions
- [[euclidean-algorithm]] - GCD computation (needed for Euclid's formula)
- [[modular-arithmetic]] - Properties of primitive triplets

### Further Reading
- Euclid's *Elements* Book X
- "Elementary Number Theory" by David Burton
- [OEIS A020882](https://oeis.org/A020882) - Primitive Pythagorean triplets
- Wikipedia: Pythagorean triple

## Tags
*Tags: #number-theory #pythagorean-theorem #euclid #diophantine-equations #geometry #primitive-triplets #project-euler*

---

**Created**: January 29, 2026  
**Last Updated**: January 29, 2026  
**Status**: Complete ✓
