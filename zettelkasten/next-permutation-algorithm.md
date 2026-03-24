# Next Permutation Algorithm (Lexicographic)

**Tags**: #algorithms #permutations #combinatorics #competitive-programming #aoc
**Related**: [[Heap's Algorithm Deep Dive]], [[Algorithms MOC]], [[AoC Patterns MOC]], [[Recursion]]

## Overview

The **next permutation** algorithm generates the lexicographically next permutation of a sequence by modifying it in-place. Repeated calls starting from a sorted array produce all n! permutations in dictionary order. Also known as **Narayana Pandita's algorithm** (14th century!).

## The Algorithm (3 Steps)

Given an array, find the next permutation in lexicographic order:

### Step 1: Find the rightmost ascent
Scan right-to-left for the largest index `i` where `arr[i-1] < arr[i]`.

### Step 2: Find the successor
Scan right-to-left for the largest index `j` where `arr[j] > arr[i-1]`. Swap `arr[i-1]` and `arr[j]`.

### Step 3: Reverse the suffix
Reverse `arr[i..]` to make it the smallest possible (ascending).

If no ascent exists (Step 1 fails), the array is fully descending -- it's the last permutation.

## Walkthrough

Starting with `[1, 3, 4, 2]`:

```
Step 1: Find rightmost ascent
  [1, 3, 4, 2]
         ^  ^   2 < 4? No (scanning arr[i-1] < arr[i])
      ^  ^      3 < 4? Yes! i=2, pivot = arr[1] = 3

Step 2: Find smallest element > 3 from the right
  [1, 3, 4, 2]
            ^   2 > 3? No
         ^      4 > 3? Yes! Swap positions 1 and 2
  [1, 4, 3, 2]

Step 3: Reverse suffix after position 1
  [1, 4, 3, 2]  ->  [1, 4, 2, 3]
        ^^^^              ^^^^  (reversed: descending -> ascending)
```

Result: `[1, 4, 2, 3]`

## Full Sequence Example

All permutations of `[1, 2, 3]` in lexicographic order:

```
[1, 2, 3]  start (sorted)
[1, 3, 2]  swap 2,3; reverse (nothing)
[2, 1, 3]  pivot=1, swap with 2, reverse suffix
[2, 3, 1]
[3, 1, 2]
[3, 2, 1]  last permutation (fully descending) -> returns false
```

## Rust Implementation

```rust
fn next_permutation(arr: &mut [usize]) -> bool {
    let n = arr.len();
    if n <= 1 {
        return false;
    }

    // Step 1: Find rightmost ascent
    let mut i = n - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false; // Last permutation
    }

    // Step 2: Find successor and swap
    let mut j = n - 1;
    while arr[j] <= arr[i - 1] {
        j -= 1;
    }
    arr.swap(i - 1, j);

    // Step 3: Reverse suffix
    arr[i..].reverse();
    true
}
```

### Usage Pattern

```rust
let mut items: Vec<usize> = (1..=n).collect(); // Start sorted!
loop {
    // Process current permutation
    process(&items);

    if !next_permutation(&mut items) {
        break; // All permutations exhausted
    }
}
```

## Why It Works

The key insight is that a **descending suffix has no more permutations** -- it's the largest arrangement of those elements. To get the next permutation:

1. **Find where it's not descending** (the pivot) -- this is the rightmost element that can still increase
2. **Bump it up** by swapping with the smallest element to its right that's larger
3. **Minimize the rest** by reversing the (now still descending) suffix to ascending

This produces the very next permutation in dictionary order with minimal changes.

## Complexity

| Metric | Value |
|--------|-------|
| Time per call | O(n) worst case, O(1) amortized |
| Space | O(1) -- in-place mutations only |
| Total for all n! perms | O(n! * n) worst, O(n!) amortized |
| Swaps per call | At most n/2 + 1 |

The **amortized O(1)** is because most calls only touch the last few elements. Only rarely does the reversal sweep the entire array.

## Comparison with Heap's Algorithm

| Property | Next Permutation | Heap's Algorithm |
|----------|-----------------|------------------|
| **Order** | Lexicographic | Non-lexicographic |
| **Swaps per perm** | O(n) worst, O(1) amortized | Exactly 1 |
| **Total swaps** | O(n!) | O(n!) |
| **Use case** | Ordered enumeration, early termination | Generate all permutations fast |
| **Partial generation** | Natural (call N times) | Requires full recursion |
| **Sorted output** | Yes | No |

**Choose next_permutation when:**
- You need lexicographic order (e.g., finding the Nth permutation)
- You might early-terminate (e.g., TSP with pruning)
- You want to iterate one-at-a-time without recursion

**Choose Heap's when:**
- You need all permutations and don't care about order
- Minimizing swaps is critical
- You're generating permutations to feed into another algorithm

## Applications in AoC

### TSP (Travelling Salesman Problem)
The most common use -- try all orderings of locations:

```rust
// AoC 2016 Day 24: Air Duct Spelunking
let mut others: Vec<usize> = (1..n).collect();
let mut best = u32::MAX;
loop {
    let cost = route_cost(0, &others, &dist);
    best = best.min(cost);
    if !next_permutation(&mut others) { break; }
}
```

This works because n is typically small (7-10 locations), giving 5,040 - 3.6M permutations.

### Feasibility Guide

| n | n! | Time estimate |
|---|-----|---------------|
| 7 | 5,040 | < 1ms |
| 8 | 40,320 | < 1ms |
| 10 | 3,628,800 | ~10ms |
| 12 | 479,001,600 | ~1s |
| 13+ | 6B+ | Too slow |

For n > 12, use bitmask DP (Held-Karp) instead: O(2^n * n^2).

## Historical Note

This algorithm was first described by **Narayana Pandita** in his 1356 work *Ganita Kaumudi*. It was independently rediscovered multiple times and is the algorithm behind C++'s `std::next_permutation`. Rust's standard library doesn't include it, so we implement it manually.

---

*Tags: #algorithms #permutations #combinatorics #competitive-programming #aoc*
*Links: [[Heap's Algorithm Deep Dive]] | [[Algorithms MOC]] | [[AoC Patterns MOC]]*
*Created: 2026-03-24 | Source: AoC 2016 Day 24 (Air Duct Spelunking)*
