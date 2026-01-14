# Hamming Distance and Discrete Metrics

*Mathematical Foundation: Distance metrics on discrete spaces*

## Overview

**Hamming distance** is a fundamental concept in coding theory, information theory, and pattern matching that measures the difference between two sequences of equal length. Unlike continuous metrics (Euclidean distance), Hamming distance counts the number of positions at which corresponding symbols differ.

**Mathematical Definition**:
For two strings $s, t$ of length $n$:

$$d_H(s, t) = \sum_{i=1}^{n} \mathbb{1}_{s_i \neq t_i}$$

where $\mathbb{1}$ is the indicator function (1 if condition is true, 0 otherwise).

## Core Concepts

### Metric Space Properties

Hamming distance forms a **metric space** satisfying:

1. **Non-negativity**: $d_H(s, t) \geq 0$
2. **Identity**: $d_H(s, t) = 0 \iff s = t$
3. **Symmetry**: $d_H(s, t) = d_H(t, s)$
4. **Triangle inequality**: $d_H(s, u) \leq d_H(s, t) + d_H(t, u)$

### Hamming Weight

**Hamming weight** $w_H(s)$ is the number of non-zero symbols in $s$:

$$w_H(s) = d_H(s, \mathbf{0})$$

where $\mathbf{0}$ is the zero vector.

For binary strings, Hamming weight = number of 1-bits (popcount).

### Relationship to XOR

For binary strings, Hamming distance equals the Hamming weight of XOR:

$$d_H(s, t) = w_H(s \oplus t)$$

This enables efficient bit-manipulation implementations.

## Applications

### Error Detection and Correction

**Error-correcting codes** use Hamming distance to detect and correct transmission errors:

- **Detection**: Code with minimum distance $d$ can detect up to $d-1$ errors
- **Correction**: Code can correct up to $\lfloor (d-1)/2 \rfloor$ errors

**Example**: Hamming(7,4) code has minimum distance 3:
- Detects 2-bit errors
- Corrects 1-bit errors

### Pattern Matching

**Near-duplicate detection**: Find strings within Hamming distance $k$ of a pattern.

**Fuzzy matching**: Allow up to $k$ mismatches when searching.

**Spell checking**: Find words with small Hamming distance to misspelling.

### Computational Biology

**DNA sequence alignment**: Count nucleotide substitutions between sequences.

**Mutation analysis**: Measure genetic distance between organisms.

## Algorithms

### Naive Counting

**Time**: $O(n)$, **Space**: $O(1)$

```
function hamming_distance(s, t):
    count = 0
    for i = 1 to n:
        if s[i] ≠ t[i]:
            count++
    return count
```

### Bit Manipulation (Binary Strings)

**Time**: $O(n/w)$ where $w$ is word size, **Space**: $O(1)$

```
function hamming_distance_binary(s, t):
    xor = s XOR t
    return popcount(xor)  // Count 1-bits
```

### Vectorized Comparison

Modern SIMD instructions can compare multiple positions simultaneously:

**Time**: $O(n/k)$ where $k$ is vector width

## Rust Implementations

### AoC 2023 Day 13: Point of Incidence

**Problem**: Find mirror reflections with exactly 0 or 1 mismatches (smudges).

**Implementation**:
```rust
// Count Hamming distance between reflected row pairs
fn count_horizontal_mismatches(pattern: &[Vec<char>], above_idx: usize) -> usize {
    let mut mismatches = 0;
    let mut distance = 0;
    
    loop {
        let upper = above_idx.checked_sub(distance);
        let lower = above_idx + 1 + distance;
        
        if upper.is_none() || lower >= pattern.len() {
            return mismatches;
        }
        
        // Hamming distance accumulation across all reflected pairs
        let upper_row = upper.unwrap();
        for (a, b) in pattern[upper_row].iter().zip(pattern[lower].iter()) {
            if a != b {
                mismatches += 1;
            }
        }
        
        distance += 1;
    }
}

// Part 1: Perfect reflection (Hamming distance = 0)
// Part 2: Smudge reflection (Hamming distance = 1)
```

**Key Insight**: Generalizing from boolean "exact match" to integer "mismatch count" unifies both problem parts.

**File**: `advent_of_code/aoc2023/src/solver/day13.rs`

**Mathematical Foundation**:
- **Reflection symmetry**: Pattern mirrors across a line
- **Hamming distance**: Total mismatches across all reflected pairs
- **Target matching**: Part 1 requires $d_H = 0$, Part 2 requires $d_H = 1$

**Performance**: $O(mn)$ where $m$ rows, $n$ columns. Each reflection line checked in linear time.

## Common Patterns in Rust

### Iterator-based Counting

```rust
// Hamming distance using zip and filter
fn hamming_distance(s: &str, t: &str) -> usize {
    s.chars()
        .zip(t.chars())
        .filter(|(a, b)| a != b)
        .count()
}
```

### Byte-level Comparison

```rust
// Efficient for ASCII/UTF-8 byte sequences
fn hamming_distance_bytes(s: &[u8], t: &[u8]) -> usize {
    s.iter()
        .zip(t.iter())
        .filter(|(a, b)| a != b)
        .count()
}
```

### Binary XOR + Popcount

```rust
// For u32/u64 bit patterns
fn hamming_distance_bits(x: u64, y: u64) -> u32 {
    (x ^ y).count_ones()
}
```

## Extensions

### Weighted Hamming Distance

Assign different costs to different mismatches:

$$d_W(s, t) = \sum_{i=1}^{n} w_i \cdot \mathbb{1}_{s_i \neq t_i}$$

### Generalized Edit Distance

**Hamming distance** is a special case of **Levenshtein distance** (edit distance) restricted to:
- Substitutions only (no insertions/deletions)
- Equal-length strings

### Hamming Sphere

**Sphere of radius $r$**: All strings within Hamming distance $r$ of center $s$:

$$B_r(s) = \{t : d_H(s, t) \leq r\}$$

**Volume**: Number of strings in sphere of radius $r$ over alphabet $\Sigma$:

$$|B_r(s)| = \sum_{k=0}^{r} \binom{n}{k} (|\Sigma| - 1)^k$$

## Related Concepts

**Within this repository**:
- [[set-theory-fundamentals]] - Set operations for membership testing
- [[graph-theory-fundamentals]] - Distance metrics in graphs

**Mathematical connections**:
- [[metric-spaces]] - General distance function theory (TODO)
- [[coding-theory]] - Error-correcting codes (TODO)
- [[edit-distance]] - Levenshtein/edit distance generalization (TODO)

**Rust patterns**:
- [[iterator-patterns]] - Zip and filter for comparisons
- [[bit-manipulation]] - XOR and popcount optimizations

## References

1. **Hamming, R.W.** (1950). "Error detecting and error correcting codes". *Bell System Technical Journal*.
2. **MacWilliams & Sloane** (1977). *The Theory of Error-Correcting Codes*.
3. **Rust std library**: `Iterator::zip`, `count_ones` for efficient implementations

## Practice Problems

**Advent of Code**:
- 2023 Day 13: Point of Incidence (reflection symmetry with Hamming distance)

**LeetCode**:
- #461: Hamming Distance
- #477: Total Hamming Distance
- #1177: Can Make Palindrome from Substring (Hamming weight)

**Project Euler**:
- Various bit manipulation problems using Hamming weight

---

*Tags: #mathematics #discrete-math #coding-theory #distance-metrics #hamming-distance #pattern-matching #aoc2023*

*Related Implementations*: [[aoc2023-day13]], [[bit-manipulation-patterns]]

*Last Updated: 2026-01-13*
