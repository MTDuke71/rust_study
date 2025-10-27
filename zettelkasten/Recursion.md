# Recursion

**Related:** [[Dynamic Programming]], [[Divide and Conquer]], [[DFS Patterns]], [[Algorithms MOC]], [[Backtracking]]

## Overview

**Recursion** is a programming technique where a function calls itself to solve a problem by breaking it down into smaller, similar subproblems. It's fundamental to many algorithms including dynamic programming, tree traversal, divide-and-conquer, and backtracking.

**Core Principle:** Every recursive function must have a **base case** (termination condition) and a **recursive case** (self-call with simpler input).

## Key Concepts

### 1. **Anatomy of Recursion**

```rust
fn recursive_function(input: T) -> Result {
    // Base case: smallest problem we can solve directly
    if is_base_case(input) {
        return base_solution(input);
    }
    
    // Recursive case: break into smaller subproblems
    let smaller_input = make_smaller(input);
    let sub_result = recursive_function(smaller_input);
    
    // Combine subproblem solutions
    combine(input, sub_result)
}
```

**Essential Components:**
1. **Base Case** - Termination condition (prevents infinite recursion)
2. **Recursive Case** - Self-call with reduced problem size
3. **Progress** - Each call moves toward base case
4. **Combination** - How subproblem results build final answer

### 2. **Classic Recursion Example: Factorial**

```rust
fn factorial(n: u64) -> u64 {
    // Base case
    if n == 0 || n == 1 {
        return 1;
    }
    
    // Recursive case
    n * factorial(n - 1)
}

// Call stack visualization for factorial(4):
// factorial(4) = 4 * factorial(3)
//              = 4 * (3 * factorial(2))
//              = 4 * (3 * (2 * factorial(1)))
//              = 4 * (3 * (2 * 1))
//              = 24
```

### 3. **Fibonacci: Multiple Recursive Calls**

```rust
// Naive recursive Fibonacci - exponential time O(2^n)
fn fib_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

// With memoization - linear time O(n)
use std::collections::HashMap;

fn fib_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    
    let result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo.insert(n, result);
    result
}
```

## Types of Recursion

### **1. Linear Recursion** (Single Recursive Call)

```rust
// Sum of array elements
fn sum_array(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    arr[0] + sum_array(&arr[1..])
}

// Reverse a string
fn reverse(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let (first, rest) = s.split_at(1);
    reverse(rest) + first
}
```

### **2. Binary Recursion** (Two Recursive Calls)

```rust
// Binary tree traversal
fn inorder_traversal(node: Option<&TreeNode>) {
    if let Some(n) = node {
        inorder_traversal(n.left.as_ref());
        println!("{}", n.val);
        inorder_traversal(n.right.as_ref());
    }
}

// Fibonacci (branching recursion)
fn fib(n: u64) -> u64 {
    if n <= 1 { return n; }
    fib(n - 1) + fib(n - 2) // Two branches
}
```

### **3. Tail Recursion** (Optimizable)

```rust
// Tail-recursive factorial
fn factorial_tail(n: u64, accumulator: u64) -> u64 {
    if n == 0 {
        return accumulator;
    }
    // Last operation is recursive call
    factorial_tail(n - 1, n * accumulator)
}

// Usage
fn factorial(n: u64) -> u64 {
    factorial_tail(n, 1)
}
```

**Note:** Rust doesn't guarantee tail call optimization, but tail recursion can be manually converted to iteration.

### **4. Mutual Recursion** (Functions Call Each Other)

```rust
fn is_even(n: i32) -> bool {
    if n == 0 {
        true
    } else {
        is_odd(n - 1)
    }
}

fn is_odd(n: i32) -> bool {
    if n == 0 {
        false
    } else {
        is_even(n - 1)
    }
}
```

## Recursion Patterns

### **Pattern 1: Divide and Conquer**

```rust
// Merge sort - classic divide and conquer
fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return; // Base case
    }
    
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);      // Divide
    merge_sort(&mut arr[mid..]);      // Divide
    merge(arr, mid);                   // Conquer
}
```

### **Pattern 2: Backtracking**

```rust
// Generate all permutations
fn permute<T: Clone>(items: &[T], result: &mut Vec<Vec<T>>, current: &mut Vec<T>) {
    // Base case: complete permutation
    if current.len() == items.len() {
        result.push(current.clone());
        return;
    }
    
    // Try each unused item
    for (i, item) in items.iter().enumerate() {
        if !current.contains(item) {
            current.push(item.clone());
            permute(items, result, current); // Recurse
            current.pop();                   // Backtrack
        }
    }
}
```

### **Pattern 3: Tree Recursion**

```rust
// Count paths in binary tree
fn count_paths(node: Option<&TreeNode>, sum: i32) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let mut count = 0;
            if n.val == sum {
                count += 1;
            }
            count += count_paths(n.left.as_ref(), sum - n.val);
            count += count_paths(n.right.as_ref(), sum - n.val);
            count
        }
    }
}
```

### **Pattern 4: Memoization (Top-Down DP)**

```rust
use std::collections::HashMap;

// Longest increasing subsequence
fn lis_memo(arr: &[i32], idx: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if idx >= arr.len() {
        return 0;
    }
    
    if let Some(&cached) = memo.get(&idx) {
        return cached;
    }
    
    let mut max_len = 1;
    for i in (idx + 1)..arr.len() {
        if arr[i] > arr[idx] {
            max_len = max_len.max(1 + lis_memo(arr, i, memo));
        }
    }
    
    memo.insert(idx, max_len);
    max_len
}
```

## Rust-Specific Considerations

### **1. Stack Overflow Risk**

```rust
// ❌ Deep recursion can overflow stack
fn deep_recursion(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        1 + deep_recursion(n - 1) // Stack frame per call
    }
}

// ✅ Use iteration for deep calls
fn deep_iteration(n: u64) -> u64 {
    (0..n).count() as u64
}

// ✅ Or increase stack size
use std::thread;

thread::Builder::new()
    .stack_size(10 * 1024 * 1024) // 10 MB
    .spawn(|| deep_recursion(100000))
    .unwrap()
    .join()
    .unwrap();
```

### **2. Ownership in Recursive Calls**

```rust
// ❌ Ownership issues
fn process_recursive(data: Vec<i32>) -> i32 {
    if data.is_empty() {
        return 0;
    }
    let rest = data[1..].to_vec(); // Expensive clone
    data[0] + process_recursive(rest) // data moved
}

// ✅ Use references
fn process_recursive_ref(data: &[i32]) -> i32 {
    if data.is_empty() {
        return 0;
    }
    data[0] + process_recursive_ref(&data[1..]) // Borrow, no clone
}
```

### **3. Lifetime Management**

```rust
// Recursive function with lifetimes
fn find_node<'a>(tree: &'a TreeNode, target: i32) -> Option<&'a TreeNode> {
    if tree.val == target {
        return Some(tree);
    }
    
    if let Some(left) = tree.left.as_ref() {
        if let Some(result) = find_node(left, target) {
            return Some(result);
        }
    }
    
    if let Some(right) = tree.right.as_ref() {
        return find_node(right, target);
    }
    
    None
}
```

## When to Use Recursion

### **✅ Use Recursion When:**
- Problem has natural recursive structure (trees, graphs)
- Divide-and-conquer approach applies
- Problem size is guaranteed small (no stack overflow)
- Code clarity is more important than performance
- Implementing backtracking algorithms
- Working with recursive data structures

### **❌ Avoid Recursion When:**
- Problem requires deep recursion (risk of stack overflow)
- Iterative solution is simpler
- Performance is critical (function call overhead)
- Tail recursion not guaranteed to be optimized
- Problem has simple loop-based solution

## Recursion → Iteration Conversion

### **Manual Conversion with Stack**

```rust
// Recursive DFS
fn dfs_recursive(node: Option<&TreeNode>) {
    if let Some(n) = node {
        println!("{}", n.val);
        dfs_recursive(n.left.as_ref());
        dfs_recursive(n.right.as_ref());
    }
}

// Iterative DFS with explicit stack
fn dfs_iterative(root: Option<&TreeNode>) {
    let mut stack = vec![root];
    
    while let Some(node_opt) = stack.pop() {
        if let Some(node) = node_opt {
            println!("{}", node.val);
            stack.push(node.right.as_ref());
            stack.push(node.left.as_ref());
        }
    }
}
```

## Common Pitfalls

### **1. Missing Base Case**

```rust
// ❌ Infinite recursion
fn bad_countdown(n: i32) {
    println!("{}", n);
    bad_countdown(n - 1); // Never stops!
}

// ✅ Proper base case
fn good_countdown(n: i32) {
    if n <= 0 {
        return; // Base case
    }
    println!("{}", n);
    good_countdown(n - 1);
}
```

### **2. Not Making Progress**

```rust
// ❌ No progress toward base case
fn stuck(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    stuck(n) // Calls itself with same value!
}

// ✅ Makes progress
fn progress(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    1 + progress(n - 1) // Moves toward base case
}
```

### **3. Redundant Computation**

```rust
// ❌ Exponential time - recomputes same values
fn fib_slow(n: u64) -> u64 {
    if n <= 1 { return n; }
    fib_slow(n - 1) + fib_slow(n - 2) // Many repeated calls
}

// ✅ Memoization - linear time
fn fib_fast(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&val) = memo.get(&n) { return val; }
    let result = fib_fast(n - 1, memo) + fib_fast(n - 2, memo);
    memo.insert(n, result);
    result
}
```

## Performance Analysis

| Pattern | Time Complexity | Space Complexity | Notes |
|---------|----------------|------------------|-------|
| **Linear Recursion** | O(n) | O(n) stack | One recursive call |
| **Binary Recursion** | O(2ⁿ) worst | O(n) stack | Two calls, no memo |
| **Tail Recursion** | O(n) | O(n) or O(1)* | Can be optimized |
| **With Memoization** | O(n) typical | O(n) memo | Caches results |
| **Divide & Conquer** | O(n log n) | O(log n) stack | Balanced splitting |

*Rust doesn't guarantee TCO

## Related Patterns

- [[Dynamic Programming]] - Recursion with memoization
- [[Divide and Conquer]] - Recursive problem decomposition
- [[Backtracking]] - Recursive search with pruning
- [[DFS Patterns]] - Depth-first recursive traversal
- [[Memoization]] - Caching recursive results

## Learning Resources

- [[mission-4]] - Recursive linked list operations
- [[mission-7]] - Graph traversal with recursion
- [[DFS Patterns]] - Recursive graph exploration
- [[Dynamic Programming]] - Advanced recursive optimization

---

## Links & Navigation

**Core Concepts:**
- [[Algorithms MOC]] - Algorithm patterns overview
- [[Dynamic Programming]] - Optimized recursion
- [[Divide and Conquer]] - Recursive decomposition
- [[Backtracking]] - Recursive search

**Applications:**
- [[DFS Patterns]] - Recursive graph traversal
- [[Tree Traversal]] - Binary tree recursion
- [[Graph Algorithms]] - Recursive graph algorithms

**Related Topics:**
- [[Stack Data Structure]] - Implicit recursion stack
- [[Memoization]] - Caching recursive results
- [[Algorithm Analysis]] - Recursion complexity

---

*Tags: #recursion #algorithms #divide-and-conquer #backtracking #tree-traversal #memoization #dynamic-programming*

*Created: 2025-10-27 | Status: 🎯 Reference Ready | Related: [[Dynamic Programming]], [[Algorithms MOC]]*
