# 🔄 Iterator Patterns in Rust

**Comprehensive guide to Rust's Iterator trait ecosystem, patterns, and zero-cost abstractions**

---

## 🎯 **Iterator Philosophy in Rust**

### **Zero-Cost Abstractions**

Rust's iterators are designed as zero-cost abstractions - they compile down to the same performance as hand-written loops while providing higher-level, more expressive code.

```rust
// These produce identical assembly when optimized:

// Iterator style (functional)
let sum: i32 = numbers.iter().filter(|&&x| x > 0).map(|&x| x * 2).sum();

// Loop style (imperative)
let mut sum = 0;
for &number in &numbers {
    if number > 0 {
        sum += number * 2;
    }
}
```

### **Core Iterator Concepts**

- **Lazy evaluation** - Iterators do nothing until consumed
- **Composability** - Chain operations together fluently
- **Type safety** - Compile-time guarantees about iteration behavior
- **Memory efficiency** - No intermediate collections unless explicitly created

---

## 🏗️ **The Iterator Trait**

### **Basic Iterator Structure**

```rust
trait Iterator {
    type Item;
    
    // Required method - defines the iteration logic
    fn next(&mut self) -> Option<Self::Item>;
    
    // Provided methods build on next()
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
    fn count(self) -> usize { /* default implementation */ }
    fn map<B, F>(self, f: F) -> Map<Self, F> { /* default implementation */ }
    // ... many more default methods
}
```

### **Custom Iterator Implementation**

```rust
struct CountDown {
    current: usize,
}

impl CountDown {
    fn new(start: usize) -> Self {
        Self { current: start }
    }
}

impl Iterator for CountDown {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            None
        } else {
            let current = self.current;
            self.current -= 1;
            Some(current)
        }
    }
    
    // Optional: Provide size hint for optimization
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.current, Some(self.current))
    }
}

// Usage
let countdown = CountDown::new(3);
let values: Vec<usize> = countdown.collect();
assert_eq!(values, vec![3, 2, 1]);
```

---

## 🔧 **Iterator Creation Patterns**

### **Standard Collection Iterators**

```rust
let vec = vec![1, 2, 3, 4, 5];

// iter() - Iterator over references
let iter_refs: Vec<&i32> = vec.iter().collect();
// into_iter() - Iterator that takes ownership
let iter_owned: Vec<i32> = vec.into_iter().collect();
// iter_mut() - Iterator over mutable references (if vec is mutable)
let mut vec_mut = vec![1, 2, 3];
for item in vec_mut.iter_mut() {
    *item *= 2;
}

// Range iterators
let range_iter = 0..10;
let inclusive_range = 0..=10;
let step_by = (0..20).step_by(2); // 0, 2, 4, 6, ...
```

### **Generator Patterns**

```rust
// Infinite iterators
fn fibonacci() -> impl Iterator<Item = u64> {
    let mut a = 0;
    let mut b = 1;
    std::iter::from_fn(move || {
        let next = a + b;
        a = b;
        b = next;
        Some(a)
    })
}

// Take first 10 Fibonacci numbers
let fib_10: Vec<u64> = fibonacci().take(10).collect();

// Repeat patterns
let repeated = std::iter::repeat(42).take(5); // [42, 42, 42, 42, 42]
let cycled = vec![1, 2, 3].into_iter().cycle().take(7); // [1, 2, 3, 1, 2, 3, 1]

// Successors (mathematical sequences)
let powers_of_2 = std::iter::successors(Some(1), |&n| Some(n * 2));
let first_10_powers: Vec<i32> = powers_of_2.take(10).collect();
```

### **Conditional Creation**

```rust
// Create iterator based on condition
fn create_iterator(use_range: bool) -> Box<dyn Iterator<Item = i32>> {
    if use_range {
        Box::new(1..=10)
    } else {
        Box::new(vec![2, 4, 6, 8, 10].into_iter())
    }
}

// Option/Result as iterators
let maybe_value: Option<i32> = Some(42);
let iter_from_option: Vec<i32> = maybe_value.into_iter().collect(); // [42] or []

let result: Result<i32, &str> = Ok(42);
let iter_from_result: Vec<i32> = result.into_iter().collect(); // [42] or []
```

---

## 🔀 **Iterator Adaptor Patterns**

### **Transformation Adaptors**

```rust
let numbers = vec![1, 2, 3, 4, 5];

// map - Transform each element
let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();

// enumerate - Add indices
let with_indices: Vec<(usize, &i32)> = numbers.iter().enumerate().collect();

// zip - Combine with another iterator
let letters = vec!['a', 'b', 'c', 'd', 'e'];
let paired: Vec<(&i32, &char)> = numbers.iter().zip(&letters).collect();

// scan - Stateful transformation (like fold but yields intermediate values)
let running_sum: Vec<i32> = numbers.iter()
    .scan(0, |state, &x| {
        *state += x;
        Some(*state)
    })
    .collect(); // [1, 3, 6, 10, 15]
```

### **Filtering Adaptors**

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// filter - Keep elements matching predicate
let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();

// filter_map - Filter and transform in one step
let even_doubled: Vec<i32> = numbers.iter()
    .filter_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None })
    .collect();

// take/skip - Positional filtering
let first_three: Vec<&i32> = numbers.iter().take(3).collect();
let skip_first_three: Vec<&i32> = numbers.iter().skip(3).collect();

// take_while/skip_while - Conditional positional filtering
let while_small: Vec<&i32> = numbers.iter().take_while(|&&x| x < 6).collect();
let after_small: Vec<&i32> = numbers.iter().skip_while(|&&x| x < 6).collect();
```

### **Grouping and Batching Adaptors**

```rust
use itertools::Itertools; // External crate for advanced patterns

let data = vec![1, 1, 2, 2, 2, 3, 1, 1];

// chunk - Group into fixed-size chunks
let chunks: Vec<Vec<&i32>> = data.iter().chunks(3).into_iter()
    .map(|chunk| chunk.collect())
    .collect();

// group_by - Group consecutive equal elements
let groups: Vec<(i32, Vec<&i32>)> = data.iter()
    .group_by(|&&x| x)
    .into_iter()
    .map(|(key, group)| (key, group.collect()))
    .collect();

// windows - Sliding windows
let windows: Vec<Vec<&i32>> = data.iter().windows(3)
    .map(|window| window.iter().collect())
    .collect();
```

---

## 🎯 **Iterator Consumer Patterns**

### **Collection Consumers**

```rust
let numbers = vec![1, 2, 3, 4, 5];

// collect - Most flexible consumer
let collected_vec: Vec<i32> = numbers.iter().cloned().collect();
let collected_set: std::collections::HashSet<i32> = numbers.iter().cloned().collect();
let collected_map: std::collections::HashMap<usize, i32> = 
    numbers.iter().enumerate().map(|(i, &v)| (i, v)).collect();

// partition - Split into two collections
let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.iter()
    .cloned()
    .partition(|&x| x % 2 == 0);

// unzip - Separate pairs
let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
let (nums, chars): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
```

### **Aggregation Consumers**

```rust
let numbers = vec![1, 2, 3, 4, 5];

// fold/reduce - Accumulate values
let sum = numbers.iter().fold(0, |acc, &x| acc + x);
let product = numbers.iter().fold(1, |acc, &x| acc * x);

// reduce - Like fold but uses first element as initial value
let max = numbers.iter().copied().reduce(|acc, x| acc.max(x));

// for_each - Side effects
numbers.iter().for_each(|&x| println!("Processing: {}", x));

// Built-in aggregations
let sum: i32 = numbers.iter().sum();
let product: i32 = numbers.iter().product();
let min = numbers.iter().min(); // Option<&i32>
let max = numbers.iter().max(); // Option<&i32>
let count = numbers.iter().count();
```

### **Search and Testing Consumers**

```rust
let numbers = vec![1, 2, 3, 4, 5];

// find - First element matching predicate
let first_even = numbers.iter().find(|&&x| x % 2 == 0); // Some(&2)

// position - Index of first match
let position = numbers.iter().position(|&x| x > 3); // Some(3)

// any/all - Boolean tests
let has_even = numbers.iter().any(|&x| x % 2 == 0); // true
let all_positive = numbers.iter().all(|&x| x > 0); // true

// nth - Element at specific position
let third = numbers.iter().nth(2); // Some(&3)

// last - Final element
let last = numbers.iter().last(); // Some(&5)
```

---

## 🏆 **Mission-Specific Iterator Patterns**

### **Stack Iterator (Mission 1)**

```rust
impl<T> Stack<T> {
    // Iterator that drains the stack (LIFO order)
    pub fn drain(&mut self) -> DrainStack<T> {
        DrainStack { stack: self }
    }
    
    // Iterator over references (top to bottom)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().rev() // Assuming Vec<T> internal storage
    }
}

pub struct DrainStack<'a, T> {
    stack: &'a mut Stack<T>,
}

impl<'a, T> Iterator for DrainStack<'a, T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.stack.len();
        (len, Some(len))
    }
}

// Usage
let mut stack = Stack::new();
stack.push(1);
stack.push(2);
stack.push(3);

let drained: Vec<i32> = stack.drain().collect(); // [3, 2, 1]
```

### **Tree Traversal Iterator (Mission 6)**

```rust
pub struct TreeIterator<T> {
    stack: Vec<&'a TreeNode<T>>,
    traversal_type: TraversalType,
}

pub enum TraversalType {
    PreOrder,
    InOrder,
    PostOrder,
    BreadthFirst,
}

impl<T> BinaryTree<T> {
    pub fn iter(&self, traversal: TraversalType) -> TreeIterator<T> {
        TreeIterator {
            stack: if let Some(root) = &self.root {
                vec![root]
            } else {
                vec![]
            },
            traversal_type: traversal,
        }
    }
}

impl<T> Iterator for TreeIterator<T> {
    type Item = &T;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.traversal_type {
            TraversalType::PreOrder => self.next_preorder(),
            TraversalType::InOrder => self.next_inorder(),
            TraversalType::PostOrder => self.next_postorder(),
            TraversalType::BreadthFirst => self.next_breadth_first(),
        }
    }
}

// Usage
let tree = build_sample_tree();
let values: Vec<&i32> = tree.iter(TraversalType::InOrder).collect();
```

### **HashMap Iterator Patterns (Mission 5)**

```rust
impl<K, V> HashMap<K, V> {
    // Iterator over key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets.iter()
            .flat_map(|bucket| bucket.iter())
            .map(|(k, v)| (k, v))
    }
    
    // Iterator over keys only
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(k, _)| k)
    }
    
    // Iterator over values only
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, v)| v)
    }
    
    // Mutable iterator over values
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.buckets.iter_mut()
            .flat_map(|bucket| bucket.iter_mut())
            .map(|(_, v)| v)
    }
    
    // Draining iterator (takes ownership)
    pub fn drain(&mut self) -> impl Iterator<Item = (K, V)> {
        self.buckets.iter_mut()
            .flat_map(|bucket| bucket.drain(..))
    }
}

// Usage patterns
let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);

// Functional transformations
let sum_values: i32 = map.values().sum();
let uppercase_keys: Vec<String> = map.keys()
    .map(|k| k.to_uppercase())
    .collect();

// Filtering and mapping
let filtered_pairs: Vec<(&str, &i32)> = map.iter()
    .filter(|(_, &v)| v > 1)
    .collect();
```

---

## ⚡ **Performance Iterator Patterns**

### **Avoiding Allocations**

```rust
// ❌ Inefficient: Creates intermediate collections
fn process_data_bad(data: &[i32]) -> Vec<i32> {
    let filtered: Vec<i32> = data.iter().filter(|&&x| x > 0).cloned().collect();
    let doubled: Vec<i32> = filtered.iter().map(|&x| x * 2).collect();
    let summed: Vec<i32> = doubled.iter().scan(0, |sum, &x| { *sum += x; Some(*sum) }).collect();
    summed
}

// ✅ Efficient: Chained iterator adaptors
fn process_data_good(data: &[i32]) -> Vec<i32> {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .scan(0, |sum, x| { *sum += x; Some(*sum) })
        .collect()
}
```

### **Iterator Size Hints**

```rust
struct EfficientIterator<I> {
    inner: I,
    remaining: usize,
}

impl<I: Iterator> Iterator for EfficientIterator<I> {
    type Item = I::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(item) => {
                self.remaining = self.remaining.saturating_sub(1);
                Some(item)
            }
            None => None,
        }
    }
    
    // Providing accurate size hints enables optimizations
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

// ExactSizeIterator for even better optimization
impl<I: Iterator> ExactSizeIterator for EfficientIterator<I> 
where 
    I: ExactSizeIterator 
{
    fn len(&self) -> usize {
        self.remaining
    }
}
```

### **Parallel Iterator Patterns**

```rust
use rayon::prelude::*;

// CPU-intensive work benefits from parallelization
let numbers: Vec<i32> = (0..1_000_000).collect();

// Sequential processing
let sequential_sum: i32 = numbers.iter()
    .map(|&x| expensive_calculation(x))
    .sum();

// Parallel processing (using rayon crate)
let parallel_sum: i32 = numbers.par_iter()
    .map(|&x| expensive_calculation(x))
    .sum();

// Parallel collection processing
let results: Vec<i32> = numbers.par_iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .collect();
```

---

## 🔗 **Advanced Iterator Patterns**

### **Iterator Chaining**

```rust
let vec1 = vec![1, 2, 3];
let vec2 = vec![4, 5, 6];
let vec3 = vec![7, 8, 9];

// Chain multiple iterators
let chained: Vec<i32> = vec1.iter()
    .chain(vec2.iter())
    .chain(vec3.iter())
    .cloned()
    .collect(); // [1, 2, 3, 4, 5, 6, 7, 8, 9]

// Flatten nested iterators
let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
let flattened: Vec<i32> = nested.iter()
    .flatten()
    .cloned()
    .collect(); // [1, 2, 3, 4, 5, 6]

// flat_map combines map and flatten
let words = vec!["hello", "world"];
let chars: Vec<char> = words.iter()
    .flat_map(|s| s.chars())
    .collect(); // ['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']
```

### **Error Handling with Iterators**

```rust
// Collect Results - fails fast on first error
fn parse_numbers(strings: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    strings.iter()
        .map(|s| s.parse::<i32>())
        .collect()
}

// Filter out errors, keep successes
fn parse_numbers_lenient(strings: &[&str]) -> Vec<i32> {
    strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect()
}

// Partition into successes and errors
fn parse_numbers_partition(strings: &[&str]) -> (Vec<i32>, Vec<std::num::ParseIntError>) {
    let results: Vec<Result<i32, _>> = strings.iter()
        .map(|s| s.parse())
        .collect();
    
    let (successes, errors): (Vec<_>, Vec<_>) = results.into_iter()
        .partition(Result::is_ok);
    
    (
        successes.into_iter().map(Result::unwrap).collect(),
        errors.into_iter().map(Result::unwrap_err).collect()
    )
}
```

### **State Machine Iterator**

```rust
struct StateMachineIterator {
    state: State,
    counter: usize,
}

enum State {
    Start,
    Processing { remaining: usize },
    Done,
}

impl StateMachineIterator {
    fn new(count: usize) -> Self {
        Self {
            state: State::Start,
            counter: 0,
        }
    }
}

impl Iterator for StateMachineIterator {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            State::Start => {
                self.state = State::Processing { remaining: 5 };
                Some("Starting".to_string())
            }
            State::Processing { remaining } => {
                if *remaining > 0 {
                    *remaining -= 1;
                    self.counter += 1;
                    Some(format!("Processing step {}", self.counter))
                } else {
                    self.state = State::Done;
                    Some("Finishing".to_string())
                }
            }
            State::Done => None,
        }
    }
}
```

---

## 🧪 **Testing Iterator Patterns**

### **Iterator Testing Strategies**

```rust
#[cfg(test)]
mod iterator_tests {
    use super::*;
    
    #[test]
    fn test_custom_iterator_basic() {
        let countdown = CountDown::new(3);
        let values: Vec<usize> = countdown.collect();
        assert_eq!(values, vec![3, 2, 1]);
    }
    
    #[test]
    fn test_iterator_size_hint() {
        let countdown = CountDown::new(5);
        assert_eq!(countdown.size_hint(), (5, Some(5)));
    }
    
    #[test]
    fn test_iterator_chain_operations() {
        let result: Vec<i32> = (1..=10)
            .filter(|&x| x % 2 == 0)
            .map(|x| x * x)
            .take(3)
            .collect();
        
        assert_eq!(result, vec![4, 16, 36]);
    }
    
    #[test]
    fn test_iterator_lazy_evaluation() {
        let mut call_count = 0;
        let expensive_iter = (1..=5).map(|x| {
            call_count += 1; // This shouldn't happen until consumed
            x * 2
        });
        
        // Iterator created but not consumed
        assert_eq!(call_count, 0);
        
        // Only consume first element
        let first = expensive_iter.take(1).collect::<Vec<_>>();
        assert_eq!(call_count, 1);
        assert_eq!(first, vec![2]);
    }
    
    #[test]
    fn test_iterator_performance() {
        let large_data: Vec<i32> = (0..100_000).collect();
        
        let start = std::time::Instant::now();
        let _result: i32 = large_data.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .sum();
        let duration = start.elapsed();
        
        // Should complete quickly due to iterator optimizations
        assert!(duration < std::time::Duration::from_millis(50));
    }
}
```

---

## 📚 **Iterator Best Practices**

### **Prefer Iterator Methods Over Loops**

```rust
// ❌ Imperative style
fn sum_even_squares_loop(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &number in numbers {
        if number % 2 == 0 {
            sum += number * number;
        }
    }
    sum
}

// ✅ Functional style with iterators
fn sum_even_squares_iter(numbers: &[i32]) -> i32 {
    numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}
```

### **Choose Appropriate Iterator Types**

```rust
fn demonstrate_iterator_types() {
    let mut vec = vec![1, 2, 3, 4, 5];
    
    // Use iter() when you need references
    let doubled_refs: Vec<i32> = vec.iter().map(|&x| x * 2).collect();
    
    // Use into_iter() when you own the data and want to consume it
    let consumed: Vec<i32> = vec.clone().into_iter().map(|x| x * 3).collect();
    
    // Use iter_mut() when you need to modify in place
    vec.iter_mut().for_each(|x| *x *= 2);
    
    // After iter_mut(), vec is now [2, 4, 6, 8, 10]
}
```

### **Optimize for Common Patterns**

```rust
// Collecting to specific container types
use std::collections::{HashMap, HashSet};

let numbers = vec![1, 2, 3, 2, 1, 4];

// Collect to HashSet for uniqueness
let unique: HashSet<i32> = numbers.iter().cloned().collect();

// Collect to HashMap with computed values
let squares: HashMap<i32, i32> = numbers.iter()
    .map(|&x| (x, x * x))
    .collect();

// Use collect() alternatives when appropriate
let exists = numbers.iter().any(|&x| x > 3);
let first_large = numbers.iter().find(|&&x| x > 2);
```

---

## 🔗 **Integration with Learning System**

### **Mission Integration**

- **[[mission-1]]** - Stack iterator implementation with LIFO ordering
- **[[mission-2]]** - Queue iterator with FIFO guarantees
- **[[mission-3]]** - Algorithm iterators for search and sort operations  
- **[[mission-5]]** - HashMap iterator patterns for key-value access
- **[[mission-6]]** - Tree traversal iterators (pre/in/post-order, BFS)
- **[[mission-7]]** - Graph iterator patterns for traversal algorithms

### **Rust Book Integration**

- **[[Rust Book MOC]]** - Chapter 13 covers functional programming and iterators
- **[[Chapter 4 Overview]]** - Ownership patterns in iterator implementation
- **[[zettelkasten/rust_book/rust-book-ch8]]** - Collection iterators and common patterns

### **Pattern Integration**

- **[[API Design Patterns]]** - Iterator as core API design pattern
- **[[Testing Patterns]]** - Comprehensive iterator testing strategies
- **[[Rest Patterns]]** - Pattern matching in iterator implementations

### **AoC Applications**

- **[[AoC Patterns MOC]]** - Iterator patterns in competitive programming
- **Data processing pipelines** for parsing AoC input
- **Algorithm optimization** using iterator chains
- **Performance patterns** for time-critical solutions

---

## 📚 **External Resources**

### **Official Documentation**

- **[Iterator Trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)** - Complete Iterator trait reference
- **[Iterator Module](https://doc.rust-lang.org/std/iter/index.html)** - All iterator types and functions
- **[Rust Book Chapter 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html)** - Functional programming features

### **Advanced Iterator Libraries**

- **[itertools](https://crates.io/crates/itertools)** - Additional iterator adaptors and utilities
- **[rayon](https://crates.io/crates/rayon)** - Parallel iterator processing
- **[streaming-iterator](https://crates.io/crates/streaming-iterator)** - Zero-allocation iterator patterns

### **Performance Resources**

- **[Iterator Zero Cost Abstractions](https://blog.rust-lang.org/2013/11/11/Rust-0.8.html)** - How Rust achieves zero-cost iterators
- **[Iterator Benchmarking](https://github.com/rust-lang/rfcs/blob/master/text/0199-ownership-variants.md)** - Performance characteristics

---

*Tags: #iterator #patterns #rust #functional-programming #zero-cost-abstractions #performance #collections #api-design #mission-iterators*
*Links: [[zettel-index]] | [[API Design Patterns]] | [[Testing Patterns]] | [[Rust Book MOC]] | [[mission-1]] | [[mission-5]] | [[mission-6]] | [[AoC Patterns MOC]] | [[Rest Patterns]]*
