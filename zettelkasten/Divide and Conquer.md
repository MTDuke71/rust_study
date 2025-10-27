# Divide and Conquer

**Related:** [[Recursion]], [[Dynamic Programming]], [[Merge Sort]], [[Quick Sort]], [[Binary Search]], [[Algorithms MOC]]

## Overview

**Divide and Conquer** is a fundamental algorithm design paradigm that solves problems by:
1. **Divide** - Breaking the problem into smaller subproblems
2. **Conquer** - Recursively solving the subproblems
3. **Combine** - Merging subproblem solutions into the final answer

**Key Characteristic:** Subproblems are **independent** (unlike Dynamic Programming where subproblems overlap).

## Core Pattern

```rust
fn divide_and_conquer<T>(problem: Problem<T>) -> Solution<T> {
    // Base case: problem small enough to solve directly
    if problem.is_base_case() {
        return problem.solve_directly();
    }
    
    // Divide: break into smaller subproblems
    let subproblems = problem.divide();
    
    // Conquer: recursively solve each subproblem
    let subsolutions: Vec<_> = subproblems
        .into_iter()
        .map(|sub| divide_and_conquer(sub))
        .collect();
    
    // Combine: merge subsolutions
    combine(subsolutions)
}
```

## Classic Divide and Conquer Algorithms

### **1. Binary Search** - O(log n)

```rust
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    // Base case: empty array
    if arr.is_empty() {
        return None;
    }
    
    let mid = arr.len() / 2;
    
    match arr[mid].cmp(target) {
        std::cmp::Ordering::Equal => Some(mid),
        std::cmp::Ordering::Greater => {
            // Divide: search left half
            binary_search(&arr[..mid], target)
        }
        std::cmp::Ordering::Less => {
            // Divide: search right half
            binary_search(&arr[mid + 1..], target)
                .map(|i| i + mid + 1)
        }
    }
}
```

**Analysis:**
- **Divide:** O(1) - split array in half
- **Conquer:** O(log n) - one recursive call, halving problem size
- **Combine:** O(1) - just return result
- **Total:** O(log n)

### **2. Merge Sort** - O(n log n)

```rust
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    // Base case: arrays of size 0 or 1 already sorted
    if arr.len() <= 1 {
        return;
    }
    
    // Divide: split into two halves
    let mid = arr.len() / 2;
    
    // Conquer: recursively sort each half
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    
    // Combine: merge sorted halves
    let mut temp = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut temp);
    arr.copy_from_slice(&temp);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], result: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i].clone();
            i += 1;
        } else {
            result[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    
    // Copy remaining elements
    result[k..].clone_from_slice(&left[i..]);
    result[k..].clone_from_slice(&right[j..]);
}
```

**Analysis:**
- **Divide:** O(1) - find midpoint
- **Conquer:** 2T(n/2) - two recursive calls on half-size problems
- **Combine:** O(n) - merge two sorted arrays
- **Recurrence:** T(n) = 2T(n/2) + O(n)
- **Total:** O(n log n)

### **3. Quick Sort** - O(n log n) average

```rust
fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    // Divide: partition around pivot
    let pivot_idx = partition(arr);
    
    // Conquer: recursively sort partitions
    quick_sort(&mut arr[..pivot_idx]);
    quick_sort(&mut arr[pivot_idx + 1..]);
    
    // Combine: nothing needed (in-place)
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_idx = arr.len() - 1;
    let mut i = 0;
    
    for j in 0..pivot_idx {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot_idx);
    i
}
```

**Analysis:**
- **Average case:** O(n log n) with balanced partitions
- **Worst case:** O(n²) with unbalanced partitions
- **Space:** O(log n) stack space

### **4. Maximum Subarray (Kadane's with D&C)**

```rust
fn max_subarray_dc(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    if arr.len() == 1 {
        return arr[0];
    }
    
    let mid = arr.len() / 2;
    
    // Divide: split into left and right
    let left_max = max_subarray_dc(&arr[..mid]);
    let right_max = max_subarray_dc(&arr[mid..]);
    
    // Combine: find max crossing subarray
    let cross_max = max_crossing_subarray(arr, mid);
    
    left_max.max(right_max).max(cross_max)
}

fn max_crossing_subarray(arr: &[i32], mid: usize) -> i32 {
    // Find max sum extending left from mid
    let mut left_sum = i32::MIN;
    let mut sum = 0;
    for &val in arr[..mid].iter().rev() {
        sum += val;
        left_sum = left_sum.max(sum);
    }
    
    // Find max sum extending right from mid
    let mut right_sum = i32::MIN;
    sum = 0;
    for &val in &arr[mid..] {
        sum += val;
        right_sum = right_sum.max(sum);
    }
    
    left_sum + right_sum
}
```

### **5. Closest Pair of Points** - O(n log n)

```rust
use std::f64;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn closest_pair(points: &mut [Point]) -> f64 {
    if points.len() <= 3 {
        return brute_force_closest(points);
    }
    
    // Divide: sort by x-coordinate and split
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    let mid = points.len() / 2;
    
    // Conquer: find closest in each half
    let left_min = closest_pair(&mut points[..mid]);
    let right_min = closest_pair(&mut points[mid..]);
    let delta = left_min.min(right_min);
    
    // Combine: check points near dividing line
    closest_split_pair(points, mid, delta)
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

fn brute_force_closest(points: &[Point]) -> f64 {
    let mut min_dist = f64::INFINITY;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            min_dist = min_dist.min(distance(&points[i], &points[j]));
        }
    }
    min_dist
}

fn closest_split_pair(points: &[Point], mid: usize, delta: f64) -> f64 {
    let mid_x = points[mid].x;
    
    // Consider only points within delta of dividing line
    let strip: Vec<_> = points
        .iter()
        .filter(|p| (p.x - mid_x).abs() < delta)
        .copied()
        .collect();
    
    // Sort strip by y-coordinate
    let mut strip = strip;
    strip.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    
    let mut min_dist = delta;
    for i in 0..strip.len() {
        // Only need to check next 7 points (proven mathematically)
        for j in (i + 1)..strip.len().min(i + 8) {
            min_dist = min_dist.min(distance(&strip[i], &strip[j]));
        }
    }
    
    min_dist
}
```

## Divide and Conquer vs Other Paradigms

### **vs Dynamic Programming**

| Aspect | Divide and Conquer | Dynamic Programming |
|--------|-------------------|---------------------|
| **Subproblems** | Independent | Overlapping |
| **Approach** | Top-down (naturally) | Top-down or bottom-up |
| **Memoization** | Usually unnecessary | Essential |
| **Example** | Merge sort | Fibonacci |

### **vs Greedy Algorithms**

| Aspect | Divide and Conquer | Greedy |
|--------|-------------------|--------|
| **Decisions** | Consider all options | Make local optimal choice |
| **Backtracking** | Implicit in recursion | No backtracking |
| **Optimality** | Guaranteed (with proof) | Not always optimal |
| **Example** | Quick sort | Dijkstra's algorithm |

## Complexity Analysis with Master Theorem

The **Master Theorem** helps analyze divide-and-conquer recurrences:

**Recurrence:** T(n) = aT(n/b) + f(n)
- a = number of subproblems
- b = factor by which problem size decreases
- f(n) = cost of divide + combine steps

**Three Cases:**

1. **f(n) = O(n^(log_b(a) - ε))** for some ε > 0
   - **Result:** T(n) = Θ(n^(log_b(a)))
   - **Example:** Binary tree traversal: T(n) = 2T(n/2) + O(1) → O(n)

2. **f(n) = Θ(n^(log_b(a)) × log^k(n))** for k ≥ 0
   - **Result:** T(n) = Θ(n^(log_b(a)) × log^(k+1)(n))
   - **Example:** Merge sort: T(n) = 2T(n/2) + O(n) → O(n log n)

3. **f(n) = Ω(n^(log_b(a) + ε))** for some ε > 0, and af(n/b) ≤ cf(n)
   - **Result:** T(n) = Θ(f(n))
   - **Example:** Special divide step dominates

### **Common Patterns:**

```rust
// Pattern 1: T(n) = T(n/2) + O(1) → O(log n)
// Example: Binary search
fn pattern1(n: usize) -> usize {
    if n <= 1 { return 1; }
    pattern1(n / 2) + 1
}

// Pattern 2: T(n) = 2T(n/2) + O(n) → O(n log n)
// Example: Merge sort
fn pattern2(n: usize) -> usize {
    if n <= 1 { return 1; }
    2 * pattern2(n / 2) + n
}

// Pattern 3: T(n) = 2T(n/2) + O(1) → O(n)
// Example: Tree traversal
fn pattern3(n: usize) -> usize {
    if n <= 1 { return 1; }
    2 * pattern3(n / 2) + 1
}
```

## Rust Implementation Tips

### **1. Avoid Unnecessary Cloning**

```rust
// ❌ Expensive: clones entire array
fn bad_divide(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let mid = arr.len() / 2;
    let left = bad_divide(arr[..mid].to_vec()); // Clone!
    let right = bad_divide(arr[mid..].to_vec()); // Clone!
    merge(left, right)
}

// ✅ Efficient: uses slices
fn good_divide(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    good_divide(&mut arr[..mid]);     // Borrow
    good_divide(&mut arr[mid..]);     // Borrow
    merge_in_place(arr, mid);
}
```

### **2. Use Iterators When Appropriate**

```rust
// Functional divide-and-conquer style
fn sum_divide_conquer(arr: &[i32]) -> i32 {
    match arr.len() {
        0 => 0,
        1 => arr[0],
        _ => {
            let mid = arr.len() / 2;
            sum_divide_conquer(&arr[..mid]) + sum_divide_conquer(&arr[mid..])
        }
    }
}

// Compare with iterator approach (often clearer)
fn sum_iterator(arr: &[i32]) -> i32 {
    arr.iter().sum()
}
```

### **3. Consider Stack Depth**

```rust
// May overflow stack for large n
fn deep_recursion(n: usize) -> usize {
    if n == 0 { return 0; }
    deep_recursion(n - 1) + 1
}

// Use iteration or increase stack size
use std::thread;

let result = thread::Builder::new()
    .stack_size(10 * 1024 * 1024)
    .spawn(|| deep_recursion(1_000_000))
    .unwrap()
    .join()
    .unwrap();
```

## When to Use Divide and Conquer

### **✅ Use When:**
- Problem naturally divides into independent subproblems
- Subproblems have same structure as original
- Efficient combine step exists
- Logarithmic recursion depth is acceptable
- Examples: Sorting, searching, tree operations

### **❌ Avoid When:**
- Subproblems overlap significantly (use DP instead)
- Division step is expensive
- Too many subproblems (exponential branching)
- Iterative solution is simpler
- Stack depth could be problematic

## Related Patterns

- [[Recursion]] - Foundation for divide-and-conquer
- [[Dynamic Programming]] - For overlapping subproblems
- [[Binary Search]] - Classic divide-and-conquer search
- [[Merge Sort]] - Optimal comparison-based sorting
- [[Quick Sort]] - Average-case optimal sorting

## Learning Resources

- [[Algorithms MOC]] - Algorithm patterns overview
- [[mission-3]] - Binary search implementation
- [[Algorithm Analysis]] - Complexity analysis
- [[Master Theorem]] - Recurrence solving

---

## Links & Navigation

**Core Concepts:**
- [[Algorithms MOC]] - Algorithm catalog
- [[Recursion]] - Recursive fundamentals
- [[Dynamic Programming]] - Related paradigm
- [[Algorithm Analysis]] - Complexity theory

**Implementations:**
- [[Binary Search]] - Search with D&C
- [[Merge Sort]] - Sorting with D&C
- [[Quick Sort]] - Partition-based D&C

---

*Tags: #divide-and-conquer #algorithms #recursion #sorting #searching #complexity-analysis #master-theorem*

*Created: 2025-10-27 | Status: 🎯 Reference Ready | Related: [[Recursion]], [[Dynamic Programming]], [[Algorithms MOC]]*
