# Palindromes

**Category**: Number Theory, String Algorithms  
**Created**: January 27, 2026  
**Related Problems**: [[project-euler-p004]], [[project-euler-p036]]

## Definition

A **palindrome** is a sequence that reads the same forwards and backwards.

**Examples**:
- Numbers: 121, 9009, 906609
- Words: "racecar", "noon", "level"
- Phrases: "A man, a plan, a canal: Panama" (ignoring spaces/punctuation)

## Mathematical Palindromes

### Properties of Numeric Palindromes

**Single-digit**: All single digits (0-9) are palindromes.

**Two-digit**: Only multiples of 11 (11, 22, 33, ..., 99).

**Proof**: A 2-digit palindrome has form $\overline{aa} = 10a + a = 11a$ where $1 \le a \le 9$.

**Three-digit**: Form $\overline{aba} = 100a + 10b + a = 101a + 10b$

**Examples**: 121, 131, 565, 999

**Four-digit**: Form $\overline{abba} = 1000a + 100b + 10b + a = 1001a + 110b = 11(91a + 10b)$

All 4-digit palindromes are divisible by 11!

**Six-digit**: Form $\overline{abccba} = 100001a + 10010b + 1100c = 11(9091a + 910b + 100c)$

Also divisible by 11!

### Divisibility Patterns

- **Even-length palindromes**: Always divisible by 11 (except single digit)
- **Odd-length palindromes**: No general divisibility rule

## Checking for Palindromes

### Method 1: String Reversal

**Algorithm**:
1. Convert number to string
2. Reverse the string
3. Compare original with reversed

**Complexity**: 
- Time: $O(d)$ where $d$ = number of digits
- Space: $O(d)$ for string storage

**Rust Implementation**:
```rust
fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}
```

**Pros**: Simple, clear, works for any base
**Cons**: String allocation overhead

### Method 2: Mathematical Reversal

**Algorithm**:
1. Extract digits by repeated `% 10` and `/ 10`
2. Build reversed number
3. Compare original with reversed

**Complexity**:
- Time: $O(d)$ where $d$ = number of digits
- Space: $O(1)$ (no string allocation)

**Rust Implementation**:
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

**Pros**: No string allocation, pure arithmetic
**Cons**: Slightly more complex logic

### Method 3: Half-Reversal (Optimized)

**Insight**: Only need to reverse half the digits!

**Algorithm**:
1. Extract digits from right, build reversed half
2. Stop when reversed ≥ remaining
3. Compare reversed with remaining (handle odd-length case)

**Example**: 12321
- Extract 1: reversed = 1, remaining = 1232
- Extract 2: reversed = 12, remaining = 123
- Stop (reversed ≥ remaining)
- Check: reversed (12) == remaining / 10 (12) ✓

**Complexity**:
- Time: $O(d/2)$ ≈ half as many iterations
- Space: $O(1)$

**Rust Implementation**:
```rust
fn is_palindrome_half(mut n: u64) -> bool {
    // Edge cases
    if n < 0 || (n % 10 == 0 && n != 0) {
        return false;
    }
    
    let mut reversed_half = 0u64;
    while n > reversed_half {
        reversed_half = reversed_half * 10 + n % 10;
        n /= 10;
    }
    
    // Even length: n == reversed_half
    // Odd length: n == reversed_half / 10 (middle digit doesn't matter)
    n == reversed_half || n == reversed_half / 10
}
```

## Generating Palindromes

### Pattern-Based Generation

For $d$-digit palindromes:

**Even length** ($d$ even):
- First half: any $d/2$-digit number
- Second half: reverse of first half

**Example** (4-digit):
- First half: 12
- Palindrome: 1221

**Odd length** ($d$ odd):
- First $(d+1)/2$ digits: any
- Remaining: reverse of first $(d-1)/2$ digits

**Example** (5-digit):
- First 3 digits: 123
- Palindrome: 12321

### Counting Palindromes

**$d$-digit palindromes**:
- Even length: $9 \times 10^{d/2 - 1}$ (first digit ≠ 0)
- Odd length: $9 \times 10^{(d-1)/2}$

**Examples**:
- 2-digit: $9 \times 10^0 = 9$ (11, 22, ..., 99)
- 3-digit: $9 \times 10^1 = 90$ (101, 111, ..., 191, 202, ..., 999)
- 4-digit: $9 \times 10^1 = 90$ (1001, 1111, ..., 9999)
- 6-digit: $9 \times 10^2 = 900$

## Palindromic Products

### Problem

Find palindromes that are products of specific factors.

**Example**: Largest palindrome from product of two 3-digit numbers?

### Search Strategy

**Brute Force**:
```rust
let mut max_palindrome = 0;
for a in 100..=999 {
    for b in 100..=999 {
        let product = a * b;
        if is_palindrome(product) && product > max_palindrome {
            max_palindrome = product;
        }
    }
}
```

**Complexity**: $O(n^2 \cdot d)$ where $n$ = range size, $d$ = digits in product

**Optimizations**:
1. **Start from maximum**: `for a in (100..=999).rev()`
2. **Symmetry**: Only check $b \ge a$ (avoid duplicates)
3. **Early termination**: If $a \times 999 < \text{current\_max}$, break outer loop
4. **Inner break**: If $a \times b < \text{current\_max}$, break inner loop

### Example: PE Problem 4

**Question**: Largest palindrome from product of two 3-digit numbers?

**Answer**: 906609 = 913 × 993

**Search space**: $900 \times 900 = 810,000$ products (reduced to ~405,000 with symmetry)

**With optimizations**: Only ~10,000 products checked before finding maximum

## Applications

### 1. Number Theory

- Palindromic primes: 2, 3, 5, 7, 11, 101, 131, 151, ...
- Palindromic squares: 1, 4, 9, 121, 484, 676, 10201, ...
- Lychrel numbers: Numbers that never form palindromes under "reverse and add"

### 2. Computer Science

- **String algorithms**: Longest palindromic substring (dynamic programming)
- **DNA sequences**: Finding palindromic sequences (restriction sites)
- **Data validation**: Credit card checksums, ISBN verification

### 3. Recreational Mathematics

- **Palindromic dates**: 2002-02-20, 2112-11-12
- **Odometer palindromes**: 12321 miles
- **Word puzzles**: Palindromic sentences, poems

## Special Palindromes

### Palindromic Primes

Primes that are also palindromes: 2, 3, 5, 7, 11, 101, 131, 151, 181, 191, 313, 353, ...

**Property**: All palindromic primes except 11 have odd length (even-length palindromes divisible by 11).

### Palindromic Squares

Numbers whose square is palindromic:
- $1^2 = 1$
- $2^2 = 4$
- $3^2 = 9$
- $11^2 = 121$
- $22^2 = 484$
- $26^2 = 676$
- $101^2 = 10201$

**Question**: Are there infinitely many? Unknown!

### Base-Dependent Palindromes

**Example**: 15 in base 10 = 1111 in base 2 (palindrome in binary!)

**Curiosity**: Numbers palindromic in multiple bases simultaneously

## Algorithms for Longest Palindromic Substring

### Problem

Given string $s$, find longest contiguous palindromic substring.

### Manacher's Algorithm

**Complexity**: $O(n)$ time, $O(n)$ space

**Idea**: Use symmetry to avoid redundant checks.

**Applications**: DNA analysis, pattern matching

## Related Concepts

- [[string-algorithms]] - Palindrome detection, longest palindromic substring
- [[project-euler-p004]] - Largest palindromic product
- [[number-properties]] - Special number classes
- [[base-conversion]] - Palindromes in different bases

## References

- *Project Euler Problem 4* - Palindromic products
- *LeetCode 125* - Valid Palindrome
- *LeetCode 5* - Longest Palindromic Substring
- [[project-euler-p004]] - Rust implementation

---

*Links:*
- **Applications**: [[project-euler-p004]]
- **Related**: [[string-algorithms]], [[number-properties]]
- **Implementations**: `project_euler/src/problems/p004.rs`

*Tags:* #palindromes #number-theory #string-algorithms #pattern-matching
