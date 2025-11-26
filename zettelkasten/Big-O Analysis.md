# 📊 Big-O Analysis in Rust

**Comprehensive guide to analyzing time and space complexity of algorithms and data structures**

---

## 📖 Core Concept

**Big-O notation** describes the **upper bound** of an algorithm's growth rate as input size approaches infinity. It answers: *"How does runtime/memory scale as input grows?"*

*For comprehensive algorithm analysis including empirical methods and profiling, see [[Algorithm Analysis]]*

**Key Idea:** We care about the **shape of the curve**, not exact timings.

```
O(1)       → Constant: Always same time, regardless of input
O(log n)   → Logarithmic: Doubles input, adds constant time
O(n)       → Linear: Doubles input, doubles time
O(n log n) → Linearithmic: Efficient sorting algorithms
O(n²)      → Quadratic: Doubles input, quadruples time
O(2ⁿ)      → Exponential: Adds 1 to input, doubles time
O(n!)      → Factorial: Becomes impractical very quickly
```

---

## 🎯 **Common Complexity Classes**

### **O(1) - Constant Time**

*Performance doesn't depend on input size*

```rust
// ✅ Array/Vec access by index
let vec = vec![1, 2, 3, 4, 5];
let value = vec[2];  // O(1) - direct memory access

// ✅ HashMap lookup (average case)
let mut map = HashMap::new();
map.insert("key", "value");
let value = map.get("key");  // O(1) average

// ✅ Stack push/pop (Vec-based)
let mut stack = Vec::new();
stack.push(10);      // O(1) amortized
stack.pop();         // O(1)

// ✅ Getting length
let len = vec.len(); // O(1) - stored as field
```

**Mission Applications:**

- **Mission1 (Stack):** `push()`, `pop()`, `peek()` all O(1)
- **Mission5 (HashMap):** `insert()`, `get()`, `remove()` average O(1)

---

### **O(log n) - Logarithmic Time**

*Halves problem size each step (binary search, balanced trees)*

```rust
// ✅ Binary search on sorted array
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
            Ordering::Equal => return Ok(mid),
        }
    }
    Err(left)
}
// Time: O(log n) - halves search space each iteration
// Example: 1,000,000 elements → max 20 comparisons (log₂ 1,000,000 ≈ 20)

// ✅ BTreeMap operations
let mut btree = BTreeMap::new();
btree.insert(key, value);  // O(log n)
btree.get(&key);           // O(log n)
```

**Why O(log n) is efficient:**

```
n = 1,000         → ~10 operations
n = 1,000,000     → ~20 operations  (100x larger, 2x operations)
n = 1,000,000,000 → ~30 operations  (1000x larger, 3x operations)
```

**Mission Applications:**

- **Mission3 (Binary Search):** Core algorithm O(log n)
- **Mission9 (Priority Queue):** Heap operations O(log n)

---

### **O(n) - Linear Time**

*Must process each element once*

```rust
// ✅ Linear search
fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (i, item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}
// Time: O(n) - worst case checks every element

// ✅ Sum all elements
let sum: i32 = vec.iter().sum();  // O(n)

// ✅ Find min/max
let max = vec.iter().max();  // O(n)

// ✅ Filter elements
let evens: Vec<_> = vec.iter()
    .filter(|&&x| x % 2 == 0)
    .collect();
// O(n) - must check each element

// ✅ Reverse a vector
vec.reverse();  // O(n)
```

**Mission Applications:**

- **Mission1 (Stack):** Printing all elements O(n)
- **Mission4 (LinkedList):** Traversal O(n)
- **Mission5 (HashMap):** Iterating all entries O(n)

---

### **O(n log n) - Linearithmic Time**

*Efficient comparison-based sorting*

```rust
// ✅ Merge sort, quick sort (average), heap sort
let mut vec = vec![5, 2, 8, 1, 9];
vec.sort();  // O(n log n) - Rust uses adaptive algorithm

// ✅ Sort with custom comparator
vec.sort_by(|a, b| b.cmp(a));  // O(n log n)

// ✅ Sort by key
vec.sort_by_key(|x| x.abs());  // O(n log n)
```

**Why O(n log n) for sorting:**

- Must touch all n elements (factor of n)
- Comparison tree has height log n (factor of log n)
- Cannot beat O(n log n) for comparison-based sorting (proven lower bound)

**Mission Applications:**

- **Mission3 (Binary Search):** Preprocessing sorted array O(n log n)
- **AoC Problems:** Many require sorting before processing

---

### **O(n²) - Quadratic Time**

*Nested loops over same data*

```rust
// ❌ Bubble sort (inefficient)
fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {           // O(n)
        for j in 0..arr.len() - 1 {   // O(n)
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
// Time: O(n²) - nested loops

// ❌ Check all pairs
fn has_duplicate_pairs(arr: &[i32]) -> bool {
    for i in 0..arr.len() {           // O(n)
        for j in (i + 1)..arr.len() { // O(n)
            if arr[i] == arr[j] {
                return true;
            }
        }
    }
    false
}
// Time: O(n²)

// ✅ Better approach with HashSet
fn has_duplicate_pairs_fast(arr: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &x in arr {                   // O(n)
        if !seen.insert(x) {          // O(1) average
            return true;
        }
    }
    false
}
// Time: O(n) - much faster!
```

**When O(n²) is acceptable:**

- Small datasets (n < 100)
- Simple implementation needed
- Cache-friendly operations

**Mission Applications:**

- **Mission5 (HashMap):** Naive collision resolution O(n²) worst case
- **AoC Problems:** Often need to optimize from O(n²) to O(n log n) or O(n)

---

### **O(2ⁿ) - Exponential Time**

*Branches double at each step*

```rust
// ❌ Naive Fibonacci (extremely slow)
fn fibonacci_slow(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    fibonacci_slow(n - 1) + fibonacci_slow(n - 2)
}
// Time: O(2ⁿ) - each call makes 2 more calls
// fib(40) takes ~1 second, fib(50) takes ~minutes!

// ✅ Fibonacci with memoization
fn fibonacci_fast(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let result = fibonacci_fast(n - 1, memo) + fibonacci_fast(n - 2, memo);
    memo.insert(n, result);
    result
}
// Time: O(n) - each value computed once
// fib(50) nearly instant!

// ❌ Generating all subsets
fn all_subsets<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![vec![]];
    }
    let first = &items[0];
    let rest_subsets = all_subsets(&items[1..]);
    let mut result = rest_subsets.clone();
    for subset in rest_subsets {
        let mut with_first = vec![first.clone()];
        with_first.extend(subset);
        result.push(with_first);
    }
    result
}
// Time: O(2ⁿ) - n items = 2ⁿ subsets
// Space: O(2ⁿ) - storing all subsets
```

**AoC Applications:**

- Day 10 (Look-and-Say): Can exhibit exponential growth
- Subset problems often require dynamic programming to avoid O(2ⁿ)

---

## 📏 **Space Complexity Analysis**

### **O(1) - Constant Space**

```rust
// ✅ In-place operations
fn reverse_in_place(arr: &mut [i32]) {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
}
// Space: O(1) - only uses a few variables
```

### **O(n) - Linear Space**

```rust
// ✅ Creating new collection
fn double_all(arr: &[i32]) -> Vec<i32> {
    arr.iter().map(|&x| x * 2).collect()
}
// Space: O(n) - new Vec with n elements

// ✅ HashMap storage
let map: HashMap<i32, String> = HashMap::new();
// Space: O(n) where n is number of entries
```

### **O(log n) - Logarithmic Space**

```rust
// ✅ Recursive binary search (call stack)
fn binary_search_recursive<T: Ord>(
    arr: &[T], 
    target: &T, 
    left: usize, 
    right: usize
) -> Option<usize> {
    if left >= right {
        return None;
    }
    let mid = left + (right - left) / 2;
    match arr[mid].cmp(target) {
        Ordering::Equal => Some(mid),
        Ordering::Less => binary_search_recursive(arr, target, mid + 1, right),
        Ordering::Greater => binary_search_recursive(arr, target, left, mid),
    }
}
// Space: O(log n) - recursion depth
```

---

## 🔍 **Analyzing Rust Collections**

### **Vec<T>**

```rust
let mut vec = Vec::new();

// O(1) operations
vec.len()           // O(1) - stored field
vec.capacity()      // O(1) - stored field
vec.is_empty()      // O(1) - checks len
vec[index]          // O(1) - direct memory access
vec.first()         // O(1) - access first element
vec.last()          // O(1) - access last element
vec.push(x)         // O(1) amortized (may resize)
vec.pop()           // O(1)

// O(n) operations
vec.insert(i, x)    // O(n) - shifts elements
vec.remove(i)       // O(n) - shifts elements
vec.contains(&x)    // O(n) - linear search
vec.iter()          // O(1) to create, O(n) to consume
vec.reverse()       // O(n)
vec.clone()         // O(n)

// O(n log n) operations
vec.sort()          // O(n log n)
vec.sort_by()       // O(n log n)
```

### **HashMap<K, V>**

```rust
let mut map = HashMap::new();

// O(1) average case operations
map.insert(k, v)    // O(1) average, O(n) worst (collision)
map.get(&k)         // O(1) average, O(n) worst
map.remove(&k)      // O(1) average, O(n) worst
map.contains_key(&k)// O(1) average, O(n) worst
map.len()           // O(1)

// O(n) operations
map.iter()          // O(n) to visit all entries
map.clear()         // O(n)
map.clone()         // O(n)
```

**Mission5 Insight:**

- Good hash function → O(1) operations
- Poor hash function → O(n) operations (many collisions)

### **BTreeMap<K, V>**

```rust
let mut btree = BTreeMap::new();

// O(log n) operations (guaranteed!)
btree.insert(k, v)  // O(log n)
btree.get(&k)       // O(log n)
btree.remove(&k)    // O(log n)

// O(n) operations
btree.iter()        // O(n)
btree.range(min..max) // O(log n) to find start, then O(k) for k elements
```

**When to use BTreeMap over HashMap:**

- Need sorted/ordered keys
- Need range queries
- Need guaranteed O(log n) (no hash collision issues)

### **VecDeque<T>**

```rust
let mut deque = VecDeque::new();

// O(1) operations
deque.push_front(x) // O(1) amortized
deque.push_back(x)  // O(1) amortized
deque.pop_front()   // O(1)
deque.pop_back()    // O(1)
deque[index]        // O(1)

// O(n) operations
deque.insert(i, x)  // O(n)
deque.remove(i)     // O(n)
```

---

## 🎓 **Practical Analysis Techniques**

### **1. Count the Loops**

```rust
// Single loop → O(n)
for x in arr {
    println!("{}", x);
}

// Nested loops → O(n²)
for i in 0..n {
    for j in 0..n {
        println!("{}, {}", i, j);
    }
}

// Nested with different sizes → O(n * m)
for i in 0..n {
    for j in 0..m {
        println!("{}, {}", i, j);
    }
}

// Sequential loops → O(n + m) = O(n) if same magnitude
for x in arr1 {
    process(x);
}
for y in arr2 {
    process(y);
}
```

### **2. Identify the Dominant Term**

```rust
// O(n² + n + 10) = O(n²)
// Drop lower order terms and constants
fn example(arr: &[i32]) {
    // O(n²) - dominates
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            // ...
        }
    }
    
    // O(n) - doesn't matter when n² present
    for x in arr {
        // ...
    }
    
    // O(1) - doesn't matter
    let sum = arr[0] + arr[1];
}
```

### **3. Watch for Hidden Complexity**

```rust
// ❌ Looks like O(n), actually O(n²)!
fn append_all(arr: &[i32]) -> String {
    let mut result = String::new();
    for &x in arr {
        result.push_str(&x.to_string());  // String concat can reallocate!
    }
    result
}

// ✅ Truly O(n) with capacity hint
fn append_all_fast(arr: &[i32]) -> String {
    let mut result = String::with_capacity(arr.len() * 4);
    for &x in arr {
        result.push_str(&x.to_string());
    }
    result
}

// ✅ Or use iterator join (preallocates)
fn append_all_idiomatic(arr: &[i32]) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}
```

### **4. Analyze Recursion**

```rust
// Recursion depth → space complexity
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}
// Time: O(n) - n recursive calls
// Space: O(n) - n stack frames

// Tail recursion (can be optimized)
fn factorial_tail(n: u32, acc: u32) -> u32 {
    if n <= 1 {
        return acc;
    }
    factorial_tail(n - 1, n * acc)
}
// Time: O(n)
// Space: O(1) if tail-call optimized (Rust doesn't guarantee this)
```

---

## 📊 **Complexity Comparison Table**

| Algorithm/Operation | Time Complexity | Space Complexity | Use Case |
|---------------------|-----------------|------------------|----------|
| **Binary Search** | O(log n) | O(1) | Sorted array lookup |
| **Linear Search** | O(n) | O(1) | Unsorted array search |
| **Quick Sort (avg)** | O(n log n) | O(log n) | General sorting |
| **Merge Sort** | O(n log n) | O(n) | Stable sorting |
| **Heap Sort** | O(n log n) | O(1) | In-place sorting |
| **Bubble Sort** | O(n²) | O(1) | Small/nearly sorted |
| **Hash Table Insert** | O(1) avg | O(1) | Fast key-value storage |
| **BTree Insert** | O(log n) | O(1) | Ordered key-value storage |
| **BFS/DFS** | O(V + E) | O(V) | Graph traversal |
| **Dijkstra** | O((V + E) log V) | O(V) | Shortest path |
| **A*** | O(E) best | O(V) | Heuristic shortest path |

---

## 🎯 **Mission-Specific Complexity Analysis**

### **Mission1: Stack**

```rust
impl<T> Stack<T> {
    pub fn new() -> Self              // O(1) time, O(1) space
    pub fn push(&mut self, item: T)   // O(1) amortized time
    pub fn pop(&mut self) -> Option<T>// O(1) time
    pub fn peek(&self) -> Option<&T>  // O(1) time
    pub fn len(&self) -> usize        // O(1) time
}
```

### **Mission3: Binary Search**

```rust
pub fn binary_search<T: Ord>(
    arr: &[T], 
    target: &T
) -> Result<usize, usize> {
    // Time: O(log n) - halves search space
    // Space: O(1) - iterative version
}
```

### **Mission5: HashMap**

```rust
impl<K, V> HashMap<K, V> {
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // Average: O(1)
        // Worst: O(n) with many collisions
        // Resize: O(n) amortized
    }
    
    pub fn get(&self, k: &K) -> Option<&V> {
        // Average: O(1)
        // Worst: O(n) with many collisions
    }
}
```

### **Mission9: Pathfinding**

```rust
// Dijkstra's Algorithm
pub fn dijkstra(graph: &Graph, start: NodeId) -> HashMap<NodeId, Cost> {
    // Time: O((V + E) log V) with binary heap
    //   - V vertices, each pushed/popped from heap: V log V
    //   - E edges, each may update heap: E log V
    // Space: O(V) for distances and priority queue
}

// A* Algorithm
pub fn astar(graph: &Graph, start: NodeId, goal: NodeId) -> Option<Path> {
    // Best case: O(b^d) where b=branching factor, d=depth
    // Worst case: O(E) - explores all edges
    // Heuristic quality determines actual performance
}
```

---

## ⚡ **Performance Optimization Guidelines**

### **Choose the Right Data Structure**

```rust
// ❌ Wrong: Vec for membership testing
fn has_duplicate_slow(items: &[i32]) -> bool {
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            if items[i] == items[j] {
                return true;
            }
        }
    }
    false
}
// O(n²)

// ✅ Right: HashSet for membership testing
fn has_duplicate_fast(items: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &item in items {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}
// O(n)
```

### **Avoid Unnecessary Allocations**

```rust
// ❌ Allocates n times
fn sum_strings_slow(strings: &[String]) -> String {
    let mut result = String::new();
    for s in strings {
        result = result + s;  // Allocates new String each time!
    }
    result
}
// O(n²) time due to repeated allocations

// ✅ Pre-allocate capacity
fn sum_strings_fast(strings: &[String]) -> String {
    let total_len: usize = strings.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len);
    for s in strings {
        result.push_str(s);  // No reallocation
    }
    result
}
// O(n) time
```

### **Use Iterators for Zero-Cost Abstractions**

```rust
// ✅ Iterator chains compile to efficient loops
let result: Vec<_> = data
    .iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)
    .take(10)
    .collect();
// No temporary allocations, single pass
```

---

## 🧪 **Benchmarking in Rust**

### **Using Criterion**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn binary_search_benchmark(c: &mut Criterion) {
    let data: Vec<i32> = (0..1_000_000).collect();
    
    c.bench_function("binary_search", |b| {
        b.iter(|| {
            binary_search(black_box(&data), black_box(&500_000))
        })
    });
}

criterion_group!(benches, binary_search_benchmark);
criterion_main!(benches);
```

### **Verify Big-O Claims**

```rust
// Test with increasing sizes
fn verify_complexity() {
    for size in [100, 1000, 10000, 100000] {
        let data: Vec<_> = (0..size).collect();
        let start = Instant::now();
        binary_search(&data, &(size / 2));
        let duration = start.elapsed();
        println!("n={}: {:?}", size, duration);
    }
}
// If O(log n), doubling size should add constant time
// If O(n), doubling size should double time
// If O(n²), doubling size should quadruple time
```

---

## 📚 **Common Mistakes to Avoid**

### **1. Hidden Complexity in Method Calls**

```rust
// ❌ Looks O(n), actually O(n²)
for i in 0..n {
    vec.insert(0, i);  // insert at front is O(n)!
}
// Total: O(n²)

// ✅ Better approach
for i in (0..n).rev() {
    vec.push(i);  // push is O(1) amortized
}
vec.reverse();    // O(n)
// Total: O(n)
```

### **2. Premature Optimization**

```rust
// ❌ Micro-optimization that hurts readability
fn sum_complex(arr: &[i32]) -> i32 {
    unsafe {
        let mut sum = 0;
        for i in 0..arr.len() {
            sum += *arr.get_unchecked(i);
        }
        sum
    }
}

// ✅ Idiomatic - compiler optimizes anyway
fn sum_simple(arr: &[i32]) -> i32 {
    arr.iter().sum()
}
// Both compile to same machine code!
```

### **3. Ignoring Space Complexity**

```rust
// ❌ Time optimal, space wasteful
fn fibonacci_memo(n: u32) -> HashMap<u32, u64> {
    let mut memo = HashMap::new();
    for i in 0..=n {
        if i <= 1 {
            memo.insert(i, i as u64);
        } else {
            let val = memo[&(i-1)] + memo[&(i-2)];
            memo.insert(i, val);
        }
    }
    memo
}
// Returns entire HashMap when only need final value

// ✅ Time and space optimal
fn fibonacci_optimal(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let (mut prev, mut curr) = (0, 1);
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
// O(n) time, O(1) space
```

---

## 🔗 **Related Concepts**

- **[[Algorithm Analysis]]** - Comprehensive algorithm analysis techniques including empirical analysis and profiling
- **[[Amortized Analysis]]** - Average cost over operation sequences
- **[[Performance Optimization]]** - Practical optimization techniques
- **[[Rust Collections MOC]]** - Collection performance characteristics
- **[[mission-5]]** - HashMap complexity analysis
- **[[mission-9]]** - Graph algorithm complexity
- **[[AoC Patterns MOC]]** - Complexity analysis for competitive programming
- **[[Complexity Analysis]]** - Related complexity concepts
- **[[Algorithm Design Patterns]]** - Efficient algorithm patterns
- **[[Benchmarking]]** - Measuring actual performance

---

## 📖 **Further Reading**

### **Rust Performance**

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- Criterion benchmarking guide
- [[zettelkasten/rust_book/rust-book-ch10]] - Zero-cost abstractions

### **Algorithm Analysis**

- *Introduction to Algorithms* (CLRS)
- *The Algorithm Design Manual* (Skiena)
- [[Performance Analysis]] - Real-world performance

---

*Tags: #big-o #complexity-analysis #algorithms #performance #time-complexity #space-complexity #optimization #computer-science*

*Links: [[Algorithm Analysis]] | [[Amortized Analysis]] | [[Performance Optimization]] | [[Rust Collections MOC]] | [[mission-5]] | [[mission-9]] | [[AoC Patterns MOC]] | [[Algorithm Design Patterns]] | [[zettel-index]]*
