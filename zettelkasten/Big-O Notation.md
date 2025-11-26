# Big-O Notation

*Created: 2025-11-08*
*Tags: #algorithm-analysis #complexity-theory #performance-analysis #time-complexity #space-complexity*

## Overview

Big-O notation describes the **asymptotic behavior** of algorithms by expressing how runtime or space requirements **scale with input size**. It provides a mathematical framework for **comparing algorithm efficiency** and **predicting performance** at scale.

## Fundamental Concepts

### Mathematical Definition

For functions f(n) and g(n), we say **f(n) = O(g(n))** if there exist positive constants c and n₀ such that:

**f(n) ≤ c × g(n)** for all n ≥ n₀

### Common Complexity Classes

```rust
// O(1) - Constant Time
fn constant_time_access(arr: &[i32], index: usize) -> Option<i32> {
    arr.get(index).copied() // Same time regardless of array size
}

// O(log n) - Logarithmic Time  
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}

// O(n) - Linear Time
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target) // Must check each element
}

// O(n log n) - Linearithmic Time
fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 { return; }
    
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    
    merge_sort(left);   // T(n/2)
    merge_sort(right);  // T(n/2)
    merge(left, right); // O(n)
    // Total: T(n) = 2T(n/2) + O(n) = O(n log n)
}

// O(n²) - Quadratic Time
fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {           // n iterations
        for j in 0..arr.len() - 1 - i { // n iterations  
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);   // O(1) operation
            }
        }
    }
    // Total: n × n × O(1) = O(n²)
}

// O(2^n) - Exponential Time
fn fibonacci_naive(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fibonacci_naive(n - 1) + fibonacci_naive(n - 2), // Two recursive calls
    }
    // T(n) = T(n-1) + T(n-2) + O(1) ≈ O(2^n)
}
```

## Complexity Hierarchy

### Time Complexity Ranking (Best to Worst)

1. **O(1)** - Constant: Hash table lookup, array access
2. **O(log n)** - Logarithmic: Binary search, balanced tree operations  
3. **O(n)** - Linear: Array traversal, linear search
4. **O(n log n)** - Linearithmic: Efficient sorting algorithms
5. **O(n²)** - Quadratic: Nested loops, naive sorting
6. **O(n³)** - Cubic: Triple nested loops, naive matrix multiplication
7. **O(2^n)** - Exponential: Recursive fibonacci, subset enumeration
8. **O(n!)** - Factorial: Permutation generation, traveling salesman brute force

### Growth Rate Visualization

```rust
// Relative growth for n = 1000
// O(1):        1 operation
// O(log n):    ~10 operations  
// O(n):        1,000 operations
// O(n log n):  ~10,000 operations
// O(n²):       1,000,000 operations
// O(2^n):      2^1000 operations (more than atoms in universe)
```

## Mission Integration Examples

### Mission 1: Stack Analysis

```rust
impl<T> Stack<T> {
    // O(1) - Amortized constant time
    fn push(&mut self, item: T) {
        self.items.push(item); // Vec push is O(1) amortized
    }
    
    // O(1) - Constant time
    fn pop(&mut self) -> Option<T> {
        self.items.pop() // Direct access to last element
    }
    
    // O(1) - Constant time
    fn peek(&self) -> Option<&T> {
        self.items.last() // Direct access, no traversal
    }
    
    // Space Complexity: O(n) where n is number of elements
}

// Validation through benchmarking
fn validate_stack_complexity() {
    let sizes = [1000, 10000, 100000, 1000000];
    
    for &size in &sizes {
        let start = std::time::Instant::now();
        
        let mut stack = Stack::new();
        for i in 0..size {
            stack.push(i); // Should be O(1) each
        }
        
        let duration = start.elapsed();
        println!("Size: {}, Time: {:?}", size, duration);
        // Time should scale linearly with size (O(n) total for n operations)
    }
}
```

### Mission 4: Linked List Complexity

```rust
impl<T> LinkedList<T> {
    // O(1) - Constant time insertion at head
    fn push_front(&mut self, data: T) {
        let new_node = Node::new(data);
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }
    
    // O(n) - Linear time search
    fn find(&self, target: &T) -> Option<&Node<T>> 
    where T: PartialEq 
    {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == *target {
                return Some(node);
            }
            current = &node.next;
        }
        None
    }
    
    // O(n) - Linear time insertion at arbitrary position
    fn insert_at(&mut self, index: usize, data: T) -> Result<(), String> {
        if index == 0 {
            self.push_front(data); // O(1)
            return Ok(());
        }
        
        let mut current = &mut self.head;
        for _ in 0..index - 1 {  // O(n) traversal
            current = match current {
                Some(node) => &mut node.next,
                None => return Err("Index out of bounds".to_string()),
            };
        }
        // Insertion is O(1), but getting there is O(n)
        // Total: O(n)
    }
}
```

### Mission 5: HashMap Analysis

```rust
use std::collections::HashMap;

impl<K, V> MyHashMap<K, V> 
where K: Hash + Eq 
{
    // O(1) average case, O(n) worst case
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash = self.hash(&key);
        let index = hash % self.capacity;
        
        // Robin Hood hashing keeps probe distance low
        // Average case: O(1) with good hash function
        // Worst case: O(n) with all keys colliding
    }
    
    // O(1) average case, O(n) worst case  
    fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        let index = hash % self.capacity;
        
        // Linear probing until found or empty slot
        // Performance depends on load factor and hash quality
    }
    
    // Amortized Analysis: Why HashMap is O(1)
    // - Good hash function distributes keys evenly
    // - Load factor kept below 0.75 through resizing  
    // - Robin Hood hashing minimizes probe distances
    // - Resize cost is O(n) but amortized to O(1) per operation
}

// Space Complexity Analysis
fn analyze_hashmap_space() {
    // Space = O(n) where n is number of key-value pairs
    // Additional overhead:
    // - Empty buckets for load factor management
    // - Hash values may be cached
    // - Metadata (size, capacity, hasher state)
    
    let map: HashMap<i32, String> = HashMap::with_capacity(1000);
    println!("Capacity: {}", map.capacity()); // Usually > 1000 for load factor
}
```

### Mission 6: Grid Operations

```rust
impl<T> Grid<T> {
    // O(1) - Direct array indexing
    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.height && col < self.width {
            Some(&self.data[row * self.width + col]) // Mathematical indexing
        } else {
            None
        }
    }
    
    // O(n) where n = width × height - must visit each cell
    fn fill(&mut self, value: T) 
    where T: Clone 
    {
        for cell in &mut self.data {
            *cell = value.clone();
        }
    }
    
    // O(min(width, height)) - traverse until boundary
    fn get_neighbors_8(&self, row: usize, col: usize) -> Vec<&T> {
        let mut neighbors = Vec::with_capacity(8); // O(1) allocation
        
        // Check all 8 directions - constant number of operations
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 { continue; }
                
                let new_row = (row as isize + dr) as usize;
                let new_col = (col as isize + dc) as usize;
                
                if let Some(cell) = self.get(new_row, new_col) { // O(1)
                    neighbors.push(cell);
                }
            }
        }
        neighbors // O(1) since at most 8 neighbors
    }
}
```

## AoC Complexity Analysis

### Day 15: Combinatorial Optimization

```rust
// Problem: Find optimal ingredient combination (4 ingredients, 100 teaspoons)

// Naive approach: O(n^4) where n = 100
fn solve_naive() -> i32 {
    let mut max_score = 0;
    
    for a in 0..=100 {           // 101 iterations
        for b in 0..=100-a {     // ~100 iterations  
            for c in 0..=100-a-b { // ~100 iterations
                let d = 100-a-b-c;
                let score = calculate_score([a, b, c, d]); // O(1)
                max_score = max_score.max(score);
            }
        }
    }
    
    max_score
    // Total: ~100³ = 1,000,000 operations
}

// Optimized approach: O(n²) using constraint propagation
fn solve_optimized() -> i32 {
    (0..=100)
        .flat_map(|a| (0..=100-a).map(move |b| (a, b)))           // O(n²) combinations
        .map(|(a, b)| {
            (0..=100-a-b)
                .map(|c| calculate_score([a, b, c, 100-a-b-c]))    // O(n) for each (a,b)
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
    // Total: O(n³) but with better constant factors
}
```

### Day 11: Password Generation

```rust
// String-based approach: O(n×m) where n=iterations, m=string length
fn increment_password_string(password: &mut String) {
    let mut chars: Vec<char> = password.chars().collect(); // O(m)
    
    for i in (0..chars.len()).rev() { // O(m) worst case
        if chars[i] == 'z' {
            chars[i] = 'a';
        } else {
            chars[i] = ((chars[i] as u8) + 1) as char;
            break;
        }
    }
    
    *password = chars.into_iter().collect(); // O(m)
    // Total per increment: O(m)
}

// Byte-based approach: O(n×m) but with better constants  
fn increment_password_bytes(password: &mut [u8]) {
    for i in (0..password.len()).rev() { // O(m) worst case
        if password[i] == b'z' {
            password[i] = b'a';
        } else {
            password[i] += 1;
            break;
        }
    }
    // Total per increment: O(m) with no allocation overhead
}
```

### Day 18: Conway's Game of Life

```rust
// Each generation: O(n×m) where n=height, m=width
fn next_generation(current: &Grid<bool>) -> Grid<bool> {
    let mut next = Grid::new(current.width(), current.height(), false);
    
    for row in 0..current.height() {        // n iterations
        for col in 0..current.width() {     // m iterations
            let neighbors = count_neighbors(current, row, col); // O(1) - max 8 checks
            let alive = current.get(row, col).unwrap_or(&false);
            
            next.set(row, col, match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            });
        }
    }
    
    next
    // Total: O(n×m) per generation
    // k generations: O(k×n×m)
}

// Space complexity: O(n×m) for current + next grids
```

## Advanced Analysis Techniques

### Amortized Analysis

```rust
// Dynamic Array (Vec) push operation
impl<T> DynamicArray<T> {
    fn push(&mut self, item: T) {
        if self.len == self.capacity {
            // Resize: O(n) operation
            let new_capacity = self.capacity * 2;
            let mut new_data = Vec::with_capacity(new_capacity);
            new_data.extend_from_slice(&self.data);
            self.data = new_data;
            self.capacity = new_capacity;
        }
        
        self.data.push(item); // O(1) operation
        self.len += 1;
    }
}

// Amortized Analysis:
// - Resize happens at sizes: 1, 2, 4, 8, 16, 32, ...
// - Cost to resize at size n: O(n)
// - Total cost for n pushes: n + 1 + 2 + 4 + 8 + ... + n/2 ≈ 2n
// - Amortized cost per push: 2n / n = O(1)
```

### Best, Average, and Worst Case

```rust
// Quicksort complexity analysis
fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    
    let pivot_index = partition(arr); // O(n)
    let (left, right) = arr.split_at_mut(pivot_index);
    
    quicksort(left);  // T(k)
    quicksort(right); // T(n-k-1)
}

// Best Case: O(n log n)
// - Pivot always divides array in half
// - Recurrence: T(n) = 2T(n/2) + O(n)

// Average Case: O(n log n)  
// - Random pivot gives balanced partitions on average
// - Expected depth: log n

// Worst Case: O(n²)
// - Pivot is always smallest/largest element
// - Recurrence: T(n) = T(n-1) + O(n)
// - Total: 1 + 2 + 3 + ... + n = O(n²)
```

## Practical Performance Validation

### Empirical Complexity Testing

```rust
use std::time::Instant;

fn measure_algorithm_complexity<F>(
    algorithm: F,
    sizes: &[usize],
    name: &str
) where 
    F: Fn(usize) -> ()
{
    println!("Algorithm: {}", name);
    println!("Size\tTime (ms)\tRatio");
    
    let mut prev_time = None;
    
    for &size in sizes {
        let start = Instant::now();
        algorithm(size);
        let duration = start.elapsed().as_millis();
        
        let ratio = match prev_time {
            Some(prev) => duration as f64 / prev as f64,
            None => 1.0,
        };
        
        println!("{}\t{}\t\t{:.2}", size, duration, ratio);
        prev_time = Some(duration);
    }
}

// Usage example
fn test_sorting_complexity() {
    let sizes = vec![1000, 2000, 4000, 8000, 16000];
    
    // Test O(n²) algorithm - expect ~4x slowdown when doubling size
    measure_algorithm_complexity(
        |size| {
            let mut data: Vec<i32> = (0..size as i32).rev().collect();
            bubble_sort(&mut data);
        },
        &sizes,
        "Bubble Sort O(n²)"
    );
    
    // Test O(n log n) algorithm - expect ~2x slowdown when doubling size
    measure_algorithm_complexity(
        |size| {
            let mut data: Vec<i32> = (0..size as i32).rev().collect();
            data.sort(); // Timsort: O(n log n)
        },
        &sizes,
        "Timsort O(n log n)"
    );
}
```

### Space Complexity Analysis

```rust
// Recursive vs Iterative Space Usage
fn factorial_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial_recursive(n - 1),
    }
    // Space: O(n) due to call stack
}

fn factorial_iterative(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
    // Space: O(1) - constant space
}

// Tail recursion optimization (Rust doesn't guarantee TCO)
fn factorial_tail_recursive(n: u64) -> u64 {
    fn helper(n: u64, acc: u64) -> u64 {
        match n {
            0 | 1 => acc,
            _ => helper(n - 1, acc * n), // Tail call position
        }
    }
    helper(n, 1)
    // Space: O(n) in Rust (no guaranteed TCO), O(1) with TCO
}
```

## Integration with Other Concepts

- **[[Performance Benchmarking]]**: Empirical validation of theoretical complexity
- **[[Performance Patterns]]**: Optimization guided by complexity analysis
- **[[zero-cost-abstractions]]**: High-level code with proven O(1) cost
- **[[HashMap Deep Dive]]**: Hash table complexity analysis
- **[[interior-mutability]]**: Runtime checking overhead analysis

## Daily Study Applications

### Week 2: Algorithm Analysis Foundations

- Basic complexity classes and examples
- Empirical testing of theoretical predictions
- Trade-offs between time and space complexity

### Week 3: Advanced Analysis Techniques

- Amortized analysis for dynamic data structures
- Best/average/worst case scenario analysis
- Probabilistic complexity analysis

### Week 5: Real-World Performance

- Profiling tools and complexity validation
- Optimization guided by complexity analysis
- Measuring actual vs theoretical performance

## Mission Applications

### Complexity Validation Requirements

1. **Mission 1**: Stack operations must be O(1) amortized
2. **Mission 4**: LinkedList operations complexity documented
3. **Mission 5**: HashMap average O(1) with load factor analysis
4. **Mission 6**: Grid operations complexity per method
5. **Mission 10**: Union-Find path compression amortized analysis

### Performance Testing Protocol

```rust
// Standard complexity validation pattern
fn validate_complexity<T, F>(
    operation: F,
    sizes: &[usize],
    expected_growth: f64, // 1.0 for O(n), 2.0 for O(n²), etc.
    tolerance: f64
) -> bool 
where 
    F: Fn(usize) -> T,
{
    let mut ratios = Vec::new();
    let mut prev_time = None;
    
    for &size in sizes {
        let start = std::time::Instant::now();
        let _ = operation(size);
        let duration = start.elapsed().as_nanos() as f64;
        
        if let Some(prev) = prev_time {
            let ratio = duration / prev;
            ratios.push(ratio);
        }
        prev_time = Some(duration);
    }
    
    // Check if growth matches expected complexity
    let avg_ratio = ratios.iter().sum::<f64>() / ratios.len() as f64;
    let size_ratio = sizes[1] as f64 / sizes[0] as f64; // Assuming uniform scaling
    let expected_ratio = size_ratio.powf(expected_growth);
    
    (avg_ratio - expected_ratio).abs() < tolerance
}
```

## Further Reading

- **[[Performance Benchmarking]]**: Empirical complexity validation
- **[[Performance Patterns]]**: Complexity-guided optimization
- **[[HashMap Deep Dive]]**: Hash table complexity analysis
- **[[zero-cost-abstractions]]**: Compile-time complexity optimization

---

*Big-O Notation Links:*

- [[Performance Benchmarking]] - Empirical complexity validation
- [[Performance Patterns]] - Complexity-guided optimization
- [[HashMap Deep Dive]] - Hash table analysis
- [[zero-cost-abstractions]] - Compile-time optimization
- [[interior-mutability]] - Runtime overhead analysis
- [[mission-1]] - Stack complexity validation
- [[mission-5]] - HashMap performance analysis
- [[mission-6]] - Grid operation complexity
