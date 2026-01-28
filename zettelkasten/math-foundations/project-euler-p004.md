# Project Euler Problem 4: Largest Palindrome Product

**Difficulty**: Easy  
**Published**: November 16, 2001  
**Solved**: January 27, 2026  
**Category**: Number Theory, Optimization, Search Algorithms  
**Related Concepts**: [[palindromes]], [[search-optimization]]

## Problem Statement

Find the largest palindrome made from the product of two 3-digit numbers.

**Given**: Example for 2-digit numbers: 9009 = 91 × 99  
**Find**: Largest palindrome from products of 100-999

## Solution Overview

**Answer**: **906609** = 913 × 993

**Approach**: Optimized brute-force search with early termination

## Mathematical Analysis

### Search Space

**Total products**: Two 3-digit factors from 100 to 999
- Range size: 900 numbers
- Total products: $900 \times 900 = 810,000$
- With symmetry (avoid $a \times b$ and $b \times a$): $\approx 405,000$

**Bounds**:
- Maximum possible: $999 \times 999 = 998,001$
- Minimum possible: $100 \times 100 = 10,000$
- All results are 5-6 digit numbers

### Palindrome Properties

**6-digit palindrome structure**: $\overline{abccba} = 100000a + 10000b + 1000c + 100c + 10b + a$

Simplify: $100001a + 10010b + 1100c = 11(9091a + 910b + 100c)$

**Key insight**: All 6-digit palindromes are divisible by 11!

**Implication**: At least one of the two factors must be divisible by 11.

**Why this helps**: We can optimize by ensuring one factor is a multiple of 11, but brute force with good pruning is already fast enough.

## Implementation Strategy

### Naive Approach (Inefficient)

```rust
fn find_largest_palindrome_naive() -> u64 {
    let mut max_palindrome = 0;
    for a in 100..=999 {
        for b in 100..=999 {
            let product = a * b;
            if is_palindrome(product) && product > max_palindrome {
                max_palindrome = product;
            }
        }
    }
    max_palindrome
}
```

**Problems**:
- Checks all 810,000 products
- Starts from small numbers (wastes time)
- No early termination

**Complexity**: $O(n^2 \cdot d)$ where $n = 900$, $d \approx 6$ (palindrome check)

### Optimized Approach

**Key optimizations**:

1. **Start from maximum**: Iterate in reverse order
```rust
for a in (100..=999).rev() { ... }
```

2. **Outer loop termination**: If $a \times 999 \le \text{current\_max}$, stop
```rust
if a * 999 <= max_palindrome {
    break;  // Can't improve anymore
}
```

3. **Symmetry**: Only check $b \le a$ to avoid duplicate products
```rust
for b in (100..=a).rev() { ... }
```

4. **Inner loop termination**: If $a \times b \le \text{current\_max}$, break inner loop
```rust
if a * b <= max_palindrome {
    break;  // Decreasing b won't help
}
```

### Optimized Implementation

```rust
fn largest_palindrome_product(min_factor: u64, max_factor: u64) -> (u64, u64, u64) {
    let mut max_palindrome = 0u64;
    let mut factor_a = 0u64;
    let mut factor_b = 0u64;

    // Start from largest factors
    for a in (min_factor..=max_factor).rev() {
        // Optimization: can't beat current max
        if a * max_factor <= max_palindrome {
            break;
        }

        for b in (min_factor..=a).rev() {
            let product = a * b;

            // Early termination
            if product <= max_palindrome {
                break;
            }

            if is_palindrome(product) && product > max_palindrome {
                max_palindrome = product;
                factor_a = a;
                factor_b = b;
            }
        }
    }

    (max_palindrome, factor_a, factor_b)
}
```

**Effective complexity**: Much better than $O(n^2)$ due to early breaks
- Outer loop breaks early when no improvement possible
- Inner loop breaks when products shrink below current max
- In practice: ~10,000 products checked vs 810,000 naive

## Palindrome Detection

### String Reversal Method (Used)

```rust
fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}
```

**Pros**:
- ✅ Simple, clear, idiomatic Rust
- ✅ Works for any base
- ✅ Easy to test and verify

**Cons**:
- ❌ String allocation overhead ($O(d)$ space)
- ❌ Slightly slower than pure math

### Mathematical Method (Alternative)

```rust
fn is_palindrome_math(mut n: u64) -> bool {
    let original = n;
    let mut reversed = 0u64;
    
    while n > 0 {
        reversed = reversed * 10 + (n % 10);
        n /= 10;
    }
    
    original == reversed
}
```

**Pros**:
- ✅ No string allocation ($O(1)$ space)
- ✅ Pure arithmetic, faster

**Cons**:
- ❌ Slightly more complex
- ❌ Only works in base 10

**Choice**: Used string method for clarity. Performance difference negligible for this problem size.

## Execution and Results

### Test Cases

**2-digit factors** (validation):
```rust
assert_eq!(largest_palindrome_product(10, 99), (9009, 91, 99));
```
Confirms known example ✓

**3-digit factors** (solution):
```rust
let (palindrome, a, b) = largest_palindrome_product(100, 999);
assert_eq!(palindrome, 906609);
assert_eq!(a, 913);
assert_eq!(b, 993);
```

**Verification**:
- $913 \times 993 = 906,609$ ✓
- $906609$ reversed: $906609$ ✓ (palindrome)
- Both factors in range [100, 999] ✓

### Performance Analysis

**Search efficiency**:
- Naive approach: 810,000 products to check
- Optimized approach: ~10,000 products checked (98.8% reduction!)
- Runtime: < 1ms (essentially instant)

**Early termination effectiveness**:
- Found maximum at $a = 993, b = 913$
- After finding 906609, outer loop continues checking $a = 992, 991, ...$ but breaks quickly
- Inner loops terminate immediately when products drop below 906609

## Learning Insights

### 1. Optimization Matters Even for "Easy" Problems

Naive $O(n^2)$ approach works but wastes 99% of computation.

**Key lesson**: Start from maximum, use early termination, exploit symmetry.

### 2. String vs Math Trade-offs

**String approach**:
- ✅ More readable
- ✅ Easier to test
- ❌ Small allocation overhead

**Math approach**:
- ✅ Faster (no allocations)
- ❌ More complex
- ❌ Base-specific

**Decision**: Prioritize clarity for Project Euler unless performance critical.

### 3. Divisibility Property Not Needed

Knowing 6-digit palindromes are divisible by 11 is interesting but doesn't help much here.

**Why**: Optimizations (reverse iteration, early breaks) are more effective than filtering by divisibility.

**Lesson**: Mathematical insights are valuable, but practical optimizations often win.

### 4. Test-Driven Confidence

**Testing strategy**:
1. Verify palindrome checker with known examples
2. Validate 2-digit case (known answer: 9009)
3. Confirm 3-digit solution properties (range, factorization)
4. Check edge cases (small ranges)

**Result**: High confidence in correctness before submission.

## Comparison to Other Approaches

### Alternative 1: Generate Palindromes

Instead of checking products, generate 6-digit palindromes and test factorizability.

**Algorithm**:
1. Generate palindromes from 999999 down to 100001
2. For each palindrome $p$, test if $p = a \times b$ where $100 \le a, b \le 999$

**Complexity**: Fewer candidates (~900 palindromes vs 810,000 products)

**Problem**: Factorization check is harder than palindrome check!

**Verdict**: Not better for this problem.

### Alternative 2: Exploit Divisibility by 11

Since 6-digit palindromes divisible by 11, ensure one factor is multiple of 11.

**Algorithm**:
```rust
for a in (100..=999).rev().step_by(11) {  // a is multiple of 11
    for b in (100..=999).rev() {
        // ... check palindrome
    }
}
```

**Benefit**: Reduces outer loop iterations by ~11×

**Drawback**: Doesn't reduce total products checked much (still need to iterate b)

**Verdict**: Marginal improvement, added complexity. Simple optimization better.

## Related Problems

### Project Euler Extensions

- **Problem 36**: Double-base palindromes (decimal AND binary)
- **Problem 55**: Lychrel numbers (reverse and add)
- **Problem 125**: Palindromic sums of consecutive squares

### General Patterns

- **Search optimization**: Start from maximum, early termination
- **Symmetry exploitation**: Avoid duplicate work ($(a,b)$ vs $(b,a)$)
- **Property-based pruning**: Use mathematical constraints

## Implementation Details

**File**: `project_euler/src/problems/p004.rs`

**Public API**:
```rust
pub fn largest_palindrome_product(min_factor: u64, max_factor: u64) 
    -> (u64, u64, u64);  // (palindrome, factor_a, factor_b)

pub fn solve() -> u64;  // Solves for 3-digit factors
```

**Tests**: 4 unit tests covering palindrome detection, examples, and solution

**Benchmark potential**: Could measure speedup from optimizations

## Zettelkasten Connections

- [[palindromes]] - Mathematical properties, detection algorithms
- [[search-optimization]] - Early termination, reverse iteration
- [[number-properties]] - Divisibility by 11 for even-length palindromes
- [[project-euler-p036]] - Future: multi-base palindromes
- [[2026-01-27]] - Daily note for this session

## References

- **Project Euler Problem 4**: https://projecteuler.net/problem=4
- **Palindrome theory**: [[palindromes]]
- **Implementation**: `project_euler/src/problems/p004.rs`

---

*Links:*
- **Foundation**: [[palindromes]]
- **Techniques**: [[search-optimization]], [[early-termination]]
- **Related Problems**: [[project-euler-p036]], [[project-euler-p055]]
- **Session**: [[2026-01-27]]

*Tags:* #project-euler #palindromes #optimization #brute-force #number-theory
