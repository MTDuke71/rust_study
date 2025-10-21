# Algorithm Analysis - Performance, Complexity, and Optimization

## Overview

Algorithm analysis is the systematic study of computational complexity, performance characteristics, and optimization opportunities in algorithms and data structures. This foundational skill enables developers to make informed design decisions, predict performance behavior, and optimize code for real-world applications.

## Core Concepts

### What is Algorithm Analysis?

Algorithm analysis examines:
- **Time Complexity**: How execution time grows with input size
- **Space Complexity**: How memory usage scales with input size  
- **Best/Average/Worst Case**: Performance across different scenarios
- **Amortized Analysis**: Average performance over a sequence of operations
- **Empirical Analysis**: Real-world performance measurement and profiling

### Why Algorithm Analysis Matters

```rust
// Example: Different approaches, different complexities
fn find_duplicates_naive(arr: &[i32]) -> Vec<i32> {
    let mut duplicates = Vec::new();
    
    // O(n²) - nested loops
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] == arr[j] && !duplicates.contains(&arr[i]) {
                duplicates.push(arr[i]);
            }
        }
    }
    duplicates
}

fn find_duplicates_optimized(arr: &[i32]) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    
    let mut counts = HashMap::new();
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();
    
    // O(n) - single pass with hash table
    for &num in arr {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
        
        if *count == 2 && seen.insert(num) {
            duplicates.push(num);
        }
    }
    duplicates
}
```

## Big-O Notation Fundamentals

### Common Complexity Classes

| Notation | Name | Example | Description |
|----------|------|---------|-------------|
| O(1) | Constant | Array access, HashMap get | Independent of input size |
| O(log n) | Logarithmic | Binary search, tree height | Halves search space each step |
| O(n) | Linear | Array scan, single loop | Proportional to input size |
| O(n log n) | Log-linear | Merge sort, heap sort | Efficient sorting algorithms |
| O(n²) | Quadratic | Nested loops, bubble sort | Grows quickly with input |
| O(2ⁿ) | Exponential | Recursive Fibonacci | Intractable for large inputs |

### Practical Examples in Rust

```rust
// O(1) - Constant Time
fn get_first_element<T>(vec: &[T]) -> Option<&T> {
    vec.first()  // Always same time regardless of vec length
}

// O(log n) - Logarithmic Time  
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}

// O(n) - Linear Time
fn find_max<T: Ord>(arr: &[T]) -> Option<&T> {
    arr.iter().max()  // Must examine every element
}

// O(n²) - Quadratic Time  
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
```

## Space Complexity Analysis

### Memory Usage Patterns

```rust
// O(1) Space - Constant memory usage
fn sum_iterative(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum  // Uses fixed amount of memory
}

// O(n) Space - Linear memory usage
fn sum_recursive(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        n + sum_recursive(n - 1)  // Stack grows with input size
    }
}

// O(n) Space - Explicit memory allocation
fn generate_fibonacci_sequence(n: usize) -> Vec<i64> {
    let mut fib = Vec::with_capacity(n);  // Pre-allocate for efficiency
    
    if n >= 1 { fib.push(0); }
    if n >= 2 { fib.push(1); }
    
    for i in 2..n {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next);
    }
    fib
}
```

### Memory Layout Considerations

```rust
// Cache-friendly: Sequential memory access
fn sum_array_sequential(arr: &[i32]) -> i32 {
    arr.iter().sum()  // O(n) time, excellent cache performance
}

// Cache-unfriendly: Random memory access
fn sum_linked_list(head: Option<Box<Node>>) -> i32 {
    let mut sum = 0;
    let mut current = head.as_ref();
    
    while let Some(node) = current {
        sum += node.value;  // Pointer chasing, poor cache performance
        current = node.next.as_ref();
    }
    sum
}

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}
```

## Amortized Analysis

### Dynamic Array Growth

```rust
// Vec<T> amortized O(1) push operation
fn demonstrate_amortized_growth() {
    let mut vec = Vec::new();
    
    // Individual operations:
    // - Most pushes: O(1) 
    // - Occasional resize: O(n)
    // - Amortized over many operations: O(1)
    
    for i in 0..1000 {
        vec.push(i);  // Amortized O(1) per operation
    }
    
    println!("Final capacity: {}", vec.capacity());
}

// Custom implementation showing resize strategy
struct GrowableArray<T> {
    data: Vec<T>,
    len: usize,
}

impl<T> GrowableArray<T> {
    fn new() -> Self {
        Self {
            data: Vec::with_capacity(4),  // Start small
            len: 0,
        }
    }
    
    fn push(&mut self, item: T) {
        if self.len == self.data.capacity() {
            // Resize: O(n) operation, but infrequent
            let new_capacity = self.data.capacity() * 2;
            self.data.reserve(new_capacity - self.data.capacity());
        }
        
        self.data.push(item);  // O(1) operation
        self.len += 1;
    }
}
```

### Hash Table Amortized Analysis

```rust
use std::collections::HashMap;

fn demonstrate_hashmap_amortization() {
    let mut map = HashMap::new();
    
    // Hash table operations are amortized O(1):
    // - Most insertions: O(1)
    // - Occasional resize/rehash: O(n) 
    // - Load factor management keeps performance good
    
    for i in 0..10000 {
        map.insert(i, i * 2);  // Amortized O(1)
    }
    
    // Lookup remains O(1) average case
    for i in 0..1000 {
        let value = map.get(&i);  // O(1) average
        assert_eq!(value, Some(&(i * 2)));
    }
}
```

## Performance Measurement in Rust

### Benchmarking with Criterion

```rust
// Example benchmark setup
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    b
}

fn fibonacci_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    
    for &size in [10, 15, 20, 25].iter() {
        group.bench_with_input(format!("recursive_{}", size), &size, |b, &size| {
            b.iter(|| fibonacci_recursive(black_box(size)))
        });
        
        group.bench_with_input(format!("iterative_{}", size), &size, |b, &size| {
            b.iter(|| fibonacci_iterative(black_box(size)))
        });
    }
    
    group.finish();
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
```

### Profiling and Analysis Tools

```rust
// Using std::time for simple measurements
use std::time::Instant;

fn time_algorithm<F, R>(f: F, name: &str) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    
    println!("{}: {:?}", name, duration);
    result
}

fn compare_sorting_algorithms() {
    let mut data1: Vec<i32> = (0..10000).rev().collect();
    let mut data2 = data1.clone();
    let mut data3 = data1.clone();
    
    time_algorithm(|| data1.sort(), "Standard sort");
    time_algorithm(|| bubble_sort(&mut data2), "Bubble sort");
    time_algorithm(|| data3.sort_unstable(), "Unstable sort");
}
```

## Algorithm Design Patterns

### Divide and Conquer

```rust
// Merge Sort: O(n log n) divide and conquer
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    
    // Divide: O(log n) levels
    merge_sort(left);
    merge_sort(right);
    
    // Conquer: O(n) merge at each level
    merge(left, right, arr);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], result: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    
    // Merge in O(n) time
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
    
    // Handle remaining elements
    while i < left.len() {
        result[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        result[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
```

### Dynamic Programming

```rust
use std::collections::HashMap;

// Fibonacci with memoization: O(n) time, O(n) space
fn fibonacci_memoized(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = memo.get(&n) {
        return result;  // O(1) lookup
    }
    
    let result = match n {
        0 | 1 => 1,
        _ => fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo),
    };
    
    memo.insert(n, result);  // O(1) insertion
    result
}

// Longest Common Subsequence: O(mn) time and space
fn longest_common_subsequence(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let (m, n) = (chars1.len(), chars2.len());
    
    // Create DP table
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in 1..=m {
        for j in 1..=n {
            if chars1[i - 1] == chars2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    
    dp[m][n]
}
```

### Greedy Algorithms

```rust
// Activity Selection: O(n log n) due to sorting
#[derive(Debug, Clone)]
struct Activity {
    start: i32,
    finish: i32,
    id: usize,
}

fn select_activities(mut activities: Vec<Activity>) -> Vec<Activity> {
    if activities.is_empty() {
        return Vec::new();
    }
    
    // Sort by finish time: O(n log n)
    activities.sort_by_key(|a| a.finish);
    
    let mut selected = vec![activities[0].clone()];
    let mut last_finish = activities[0].finish;
    
    // Greedy selection: O(n)
    for activity in activities.iter().skip(1) {
        if activity.start >= last_finish {
            selected.push(activity.clone());
            last_finish = activity.finish;
        }
    }
    
    selected
}

// Fractional Knapsack: O(n log n)
#[derive(Debug, Clone)]
struct Item {
    weight: f64,
    value: f64,
}

impl Item {
    fn value_per_weight(&self) -> f64 {
        self.value / self.weight
    }
}

fn fractional_knapsack(mut items: Vec<Item>, capacity: f64) -> f64 {
    // Sort by value-to-weight ratio: O(n log n)
    items.sort_by(|a, b| {
        b.value_per_weight()
            .partial_cmp(&a.value_per_weight())
            .unwrap()
    });
    
    let mut total_value = 0.0;
    let mut remaining_capacity = capacity;
    
    // Greedy selection: O(n)
    for item in items {
        if remaining_capacity >= item.weight {
            // Take entire item
            total_value += item.value;
            remaining_capacity -= item.weight;
        } else {
            // Take fraction of item
            let fraction = remaining_capacity / item.weight;
            total_value += item.value * fraction;
            break;
        }
    }
    
    total_value
}
```

## Data Structure Analysis

### Array vs Linked List Trade-offs

```rust
// Array-based stack: Better cache performance
struct ArrayStack<T> {
    data: Vec<T>,
}

impl<T> ArrayStack<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.data.push(item);  // O(1) amortized
    }
    
    fn pop(&mut self) -> Option<T> {
        self.data.pop()  // O(1)
    }
    
    fn peek(&self) -> Option<&T> {
        self.data.last()  // O(1), excellent cache locality
    }
}

// Linked list stack: Predictable O(1) operations
struct LinkedStack<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedStack<T> {
    fn new() -> Self {
        Self { head: None }
    }
    
    fn push(&mut self, item: T) {
        // Always O(1), no amortization needed
        let new_node = Box::new(Node {
            data: item,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}
```

### Hash Table vs Tree Comparison

```rust
use std::collections::{HashMap, BTreeMap};

fn compare_data_structures() {
    let mut hash_map = HashMap::new();
    let mut btree_map = BTreeMap::new();
    
    // Insertion comparison
    for i in 0..1000 {
        hash_map.insert(i, i * 2);    // O(1) average, O(n) worst
        btree_map.insert(i, i * 2);   // O(log n) guaranteed
    }
    
    // Lookup comparison  
    for i in 0..1000 {
        let _hash_result = hash_map.get(&i);    // O(1) average
        let _tree_result = btree_map.get(&i);   // O(log n) guaranteed
    }
    
    // Iteration comparison
    let _hash_iter: Vec<_> = hash_map.iter().collect();  // Unordered
    let _tree_iter: Vec<_> = btree_map.iter().collect(); // Sorted order
}
```

## Algorithm Optimization Strategies

### Early Termination

```rust
// Search with early termination
fn contains_target(arr: &[i32], target: i32) -> bool {
    for &item in arr {
        if item == target {
            return true;  // Early termination saves unnecessary work
        }
    }
    false
}

// All pairs shortest path with early termination
fn has_negative_cycle(graph: &[Vec<(usize, i32)>]) -> bool {
    let n = graph.len();
    let mut dist = vec![vec![i32::MAX; n]; n];
    
    // Initialize distances
    for i in 0..n {
        dist[i][i] = 0;
        for &(j, weight) in &graph[i] {
            dist[i][j] = weight;
        }
    }
    
    // Floyd-Warshall with early termination
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != i32::MAX && dist[k][j] != i32::MAX {
                    let new_dist = dist[i][k] + dist[k][j];
                    if new_dist < dist[i][j] {
                        dist[i][j] = new_dist;
                        
                        // Early termination: negative cycle detected
                        if i == j && new_dist < 0 {
                            return true;
                        }
                    }
                }
            }
        }
    }
    
    false
}
```

### Space-Time Trade-offs

```rust
use std::collections::HashSet;

// Space-efficient: O(1) space, O(n²) time
fn has_duplicate_space_efficient(arr: &[i32]) -> bool {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] == arr[j] {
                return true;
            }
        }
    }
    false
}

// Time-efficient: O(n) space, O(n) time  
fn has_duplicate_time_efficient(arr: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &item in arr {
        if !seen.insert(item) {
            return true;  // insert returns false if already present
        }
    }
    false
}

// Hybrid approach: Limited space, better time
fn has_duplicate_hybrid(arr: &[i32]) -> bool {
    const CHUNK_SIZE: usize = 1000;
    
    for chunk in arr.chunks(CHUNK_SIZE) {
        let mut seen = HashSet::new();
        for &item in chunk {
            if !seen.insert(item) {
                return true;
            }
        }
        
        // Check current chunk against previous elements
        for &item in chunk {
            for &prev in arr.iter().take_while(|&&x| x != chunk[0]) {
                if item == prev {
                    return true;
                }
            }
        }
    }
    false
}
```

### Algorithmic Improvements

```rust
// Naive string matching: O(nm)
fn naive_string_search(text: &str, pattern: &str) -> Option<usize> {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    
    for i in 0..=(text_chars.len().saturating_sub(pattern_chars.len())) {
        let mut found = true;
        for j in 0..pattern_chars.len() {
            if text_chars[i + j] != pattern_chars[j] {
                found = false;
                break;
            }
        }
        if found {
            return Some(i);
        }
    }
    None
}

// KMP string matching: O(n + m)
fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let lps = compute_lps_array(pattern_bytes);
    
    let mut i = 0; // text index
    let mut j = 0; // pattern index
    
    while i < text_bytes.len() {
        if text_bytes[i] == pattern_bytes[j] {
            i += 1;
            j += 1;
        }
        
        if j == pattern_bytes.len() {
            return Some(i - j);
        } else if i < text_bytes.len() && text_bytes[i] != pattern_bytes[j] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    
    None
}

fn compute_lps_array(pattern: &[u8]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut length = 0;
    let mut i = 1;
    
    while i < pattern.len() {
        if pattern[i] == pattern[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else {
            if length != 0 {
                length = lps[length - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    
    lps
}
```

## Real-World Applications

### Mission Integration Examples

```rust
// Mission1 Stack Analysis: O(1) operations
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    // Analysis: O(1) amortized push due to Vec growth strategy
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    // Analysis: O(1) pop, no memory reallocation needed
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    // Analysis: O(1) peek, just index access
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

// Mission2 Queue Analysis: Different implementations, different trade-offs
use std::collections::VecDeque;

// Ring buffer queue: True O(1) operations, bounded capacity
struct RingQueue<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> RingQueue<T> {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
        }
    }
    
    // Analysis: Always O(1), no resizing
    fn enqueue(&mut self, item: T) -> Result<(), T> {
        if self.size == self.buffer.len() {
            return Err(item);
        }
        
        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.buffer.len();
        self.size += 1;
        Ok(())
    }
    
    // Analysis: Always O(1)
    fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        
        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.buffer.len();
        self.size -= 1;
        item
    }
}
```

### AoC Algorithm Patterns

```rust
// Grid traversal analysis: Different algorithms, different complexities
use std::collections::{HashSet, VecDeque};

// BFS pathfinding: O(V + E) where V = cells, E = edges
fn bfs_shortest_path(
    grid: &[Vec<char>], 
    start: (usize, usize), 
    end: (usize, usize)
) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start, 0));  // (position, distance)
    visited.insert(start);
    
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    while let Some(((row, col), dist)) = queue.pop_front() {
        if (row, col) == end {
            return Some(dist);
        }
        
        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            
            if new_row >= 0 && new_row < rows as i32 
                && new_col >= 0 && new_col < cols as i32 {
                let pos = (new_row as usize, new_col as usize);
                
                if !visited.contains(&pos) && grid[pos.0][pos.1] != '#' {
                    visited.insert(pos);
                    queue.push_back((pos, dist + 1));
                }
            }
        }
    }
    
    None
}

// DFS path counting: O(4^n) in worst case (exponential)
fn count_paths_dfs(
    grid: &[Vec<char>],
    current: (usize, usize),
    end: (usize, usize),
    visited: &mut HashSet<(usize, usize)>
) -> usize {
    if current == end {
        return 1;
    }
    
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut paths = 0;
    
    for (dr, dc) in directions.iter() {
        let new_row = current.0 as i32 + dr;
        let new_col = current.1 as i32 + dc;
        
        if new_row >= 0 && new_row < rows as i32 
            && new_col >= 0 && new_col < cols as i32 {
            let pos = (new_row as usize, new_col as usize);
            
            if !visited.contains(&pos) && grid[pos.0][pos.1] != '#' {
                visited.insert(pos);
                paths += count_paths_dfs(grid, pos, end, visited);
                visited.remove(&pos);  // Backtrack
            }
        }
    }
    
    paths
}
```

## Practical Analysis Techniques

### Empirical Analysis Setup

```rust
use std::time::{Duration, Instant};

// Comprehensive algorithm analysis framework
struct AlgorithmAnalyzer {
    measurements: Vec<(usize, Duration)>,
}

impl AlgorithmAnalyzer {
    fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }
    
    fn measure<F, T>(&mut self, input_size: usize, algorithm: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = algorithm();
        let duration = start.elapsed();
        
        self.measurements.push((input_size, duration));
        result
    }
    
    fn analyze_complexity(&self) -> String {
        if self.measurements.len() < 2 {
            return "Insufficient data".to_string();
        }
        
        // Simple linear regression to estimate complexity
        let n = self.measurements.len() as f64;
        let sum_x: f64 = self.measurements.iter()
            .map(|(size, _)| *size as f64)
            .sum();
        let sum_y: f64 = self.measurements.iter()
            .map(|(_, duration)| duration.as_nanos() as f64)
            .sum();
        let sum_xy: f64 = self.measurements.iter()
            .map(|(size, duration)| *size as f64 * duration.as_nanos() as f64)
            .sum();
        let sum_x2: f64 = self.measurements.iter()
            .map(|(size, _)| (*size as f64).powi(2))
            .sum();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;
        
        // Estimate complexity class based on slope behavior
        let ratio = if self.measurements.len() >= 3 {
            let last_ratio = self.measurements[self.measurements.len() - 1].1.as_nanos() as f64 
                / self.measurements[self.measurements.len() - 2].1.as_nanos() as f64;
            let size_ratio = self.measurements[self.measurements.len() - 1].0 as f64 
                / self.measurements[self.measurements.len() - 2].0 as f64;
            last_ratio / size_ratio
        } else {
            1.0
        };
        
        let complexity = if ratio < 1.2 {
            "O(1) or O(log n)"
        } else if ratio < 2.0 {
            "O(n)"
        } else if ratio < 4.0 {
            "O(n log n)"
        } else if ratio < 8.0 {
            "O(n²)"
        } else {
            "O(2ⁿ) or worse"
        };
        
        format!("Estimated complexity: {}, Slope: {:.2e}, Intercept: {:.2e}", 
                complexity, slope, intercept)
    }
    
    fn print_report(&self) {
        println!("Algorithm Performance Analysis");
        println!("=============================");
        
        for (size, duration) in &self.measurements {
            println!("Input size: {}, Time: {:?}", size, duration);
        }
        
        println!("\n{}", self.analyze_complexity());
    }
}

// Example usage
fn analyze_sorting_algorithms() {
    let mut analyzer = AlgorithmAnalyzer::new();
    
    for size in [100, 200, 400, 800, 1600].iter() {
        let mut data: Vec<i32> = (0..*size as i32).rev().collect();
        
        analyzer.measure(*size, || {
            data.sort();  // O(n log n)
        });
    }
    
    analyzer.print_report();
}
```

### Memory Analysis Tools

```rust
use std::mem;

// Memory usage analysis
fn analyze_memory_usage<T>() {
    println!("Memory Analysis for type: {}", std::any::type_name::<T>());
    println!("Size of T: {} bytes", mem::size_of::<T>());
    println!("Alignment of T: {} bytes", mem::align_of::<T>());
    
    // Vec<T> memory overhead
    let vec: Vec<T> = Vec::new();
    println!("Empty Vec<T> size: {} bytes", mem::size_of_val(&vec));
    
    // Box<T> overhead
    println!("Box<T> pointer size: {} bytes", mem::size_of::<Box<T>>());
}

// Cache performance estimation
fn estimate_cache_performance(access_pattern: &[usize], data_size: usize) -> f64 {
    const CACHE_LINE_SIZE: usize = 64; // Common cache line size
    const L1_CACHE_SIZE: usize = 32 * 1024; // 32KB L1 cache
    
    let mut cache_lines_used = std::collections::HashSet::new();
    let mut cache_misses = 0;
    
    for &index in access_pattern {
        let cache_line = (index * data_size) / CACHE_LINE_SIZE;
        
        if cache_line * CACHE_LINE_SIZE > L1_CACHE_SIZE {
            cache_misses += 1;
        }
        
        cache_lines_used.insert(cache_line);
    }
    
    let cache_efficiency = 1.0 - (cache_misses as f64 / access_pattern.len() as f64);
    
    println!("Cache lines used: {}", cache_lines_used.len());
    println!("Estimated cache misses: {}", cache_misses);
    println!("Cache efficiency: {:.2}%", cache_efficiency * 100.0);
    
    cache_efficiency
}
```

## Best Practices for Algorithm Analysis

### Do's ✅

1. **Measure What Matters**
   ```rust
   // Focus on the operations that scale with input size
   fn analyze_relevant_operations(data: &[i32]) {
       // Count comparisons, not print statements
       let mut comparisons = 0;
       
       for i in 0..data.len() {
           for j in (i + 1)..data.len() {
               comparisons += 1;  // This scales as O(n²)
               if data[i] > data[j] {
                   // Actual work that matters for complexity
               }
           }
       }
       
       println!("Comparisons performed: {}", comparisons);
   }
   ```

2. **Consider All Cases**
   ```rust
   // Analyze best, average, and worst cases
   fn analyze_quicksort_cases() {
       // Best case: O(n log n) - well-balanced pivots
       // Average case: O(n log n) - random pivots  
       // Worst case: O(n²) - already sorted with first element as pivot
       
       println!("Quicksort complexity depends on pivot selection strategy");
   }
   ```

3. **Use Realistic Data**
   ```rust
   // Test with realistic input distributions
   fn benchmark_with_realistic_data() {
       // Don't just test with sequential data
       let mut random_data: Vec<i32> = (0..1000).collect();
       let mut sorted_data: Vec<i32> = (0..1000).collect();
       let mut reverse_sorted: Vec<i32> = (0..1000).rev().collect();
       
       // Test all scenarios
       measure_algorithm(&mut random_data, "random");
       measure_algorithm(&mut sorted_data, "already sorted");
       measure_algorithm(&mut reverse_sorted, "reverse sorted");
   }
   
   fn measure_algorithm(data: &mut [i32], scenario: &str) {
       let start = std::time::Instant::now();
       data.sort();
       println!("{}: {:?}", scenario, start.elapsed());
   }
   ```

### Don'ts ❌

1. **Don't Ignore Constants**
   ```rust
   // Both are O(n), but performance differs significantly
   fn inefficient_linear_search(arr: &[i32], target: i32) -> bool {
       // Bad: unnecessary work in each iteration
       for i in 0..arr.len() {
           if arr[i] == target {
               return true;
           }
           // Unnecessary work that increases constant factors
           let _ = i * i * i;  // Don't do this!
       }
       false
   }
   
   fn efficient_linear_search(arr: &[i32], target: i32) -> bool {
       // Good: minimal work per iteration
       for &item in arr {
           if item == target {
               return true;
           }
       }
       false
   }
   ```

2. **Don't Forget Memory Hierarchy**
   ```rust
   // Cache-unfriendly: jumps around memory
   fn cache_unfriendly_sum(matrix: &[Vec<i32>]) -> i32 {
       let mut sum = 0;
       // Accessing by columns (poor cache locality)
       for col in 0..matrix[0].len() {
           for row in 0..matrix.len() {
               sum += matrix[row][col];  // Cache miss likely
           }
       }
       sum
   }
   
   // Cache-friendly: sequential access
   fn cache_friendly_sum(matrix: &[Vec<i32>]) -> i32 {
       let mut sum = 0;
       // Accessing by rows (good cache locality)
       for row in matrix {
           for &value in row {
               sum += value;  // Cache hit likely
           }
       }
       sum
   }
   ```

## Integration with Mission Codebase

### Mission-Specific Analysis

- **Mission1 (Stack)**: Focus on amortized analysis of dynamic array growth
- **Mission2 (Queue)**: Compare ring buffer vs linked list trade-offs
- **Mission3 (Binary Search)**: Emphasize logarithmic complexity benefits
- **Mission4 (Linked Lists)**: Analyze pointer-chasing performance costs
- **Mission5 (HashMap)**: Deep dive into hash function performance
- **Mission6 (Grids)**: Spatial locality and memory layout optimization
- **Mission7 (Graphs)**: Graph algorithm complexity based on representation

### AoC Problem Analysis

- **Parsing Problems**: String processing complexity patterns
- **Grid Problems**: 2D traversal and pathfinding analysis  
- **Graph Problems**: BFS/DFS complexity in different graph types
- **Dynamic Programming**: Memoization trade-offs and space optimization

---

*Created: 2025-10-19*
*Last Updated: 2025-10-19*

*Tags: #algorithm-analysis #big-o #performance #complexity #optimization #benchmarking #profiling #space-time-tradeoffs #amortized-analysis #empirical-analysis #cache-performance #memory-hierarchy*

*Links: [[Big-O Notation]] | [[Amortized Analysis]] | [[Performance Optimization]] | [[Benchmarking]] | [[Cache Efficiency]] | [[Memory Layout]] | [[AoC Patterns MOC]] | [[Mission1 Overview]] | [[Mission2 Overview]] | [[Mission5 Overview]] | [[Testing Strategies]] | [[Quality Assurance]] | [[Rust Collections MOC]] | [[../advent_of_code/aoc2015/examples/day14_analysis]] | [[../advent_of_code/aoc2015/examples/DAY14_COMPLETE_SUMMARY]] | [[../advent_of_code/aoc2015/examples/DOCUMENTATION_ENHANCEMENTS]]*