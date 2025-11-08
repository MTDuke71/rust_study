# Iteration Patterns in Rust

## Overview

Iteration is a fundamental programming pattern for processing sequences of data. Rust provides powerful, zero-cost iteration abstractions that compile to the same performance as manual loops while offering superior composability and safety.

## Core Iterator Concepts

### The Iterator Trait
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    
    // 70+ default implementations built on next()
    // map(), filter(), collect(), fold(), etc.
}
```

### Three Ownership Patterns
```rust
let vec = vec![1, 2, 3, 4, 5];

// 1. Borrowing iteration (most common)
for item in &vec {          // item: &i32
    println!("{}", item);
}
// vec still usable after loop

// 2. Mutable borrowing iteration
let mut vec = vec![1, 2, 3, 4, 5];
for item in &mut vec {      // item: &mut i32
    *item *= 2;            // Modify in place
}

// 3. Consuming iteration (takes ownership)
for item in vec {          // item: i32
    println!("{}", item);  // vec consumed, no longer accessible
}
```

## Basic Iteration Patterns

### 1. For Loop Iteration
```rust
// Array/slice iteration
let arr = [10, 20, 30, 40, 50];
for element in arr {
    println!("Value: {}", element);
}

// Range iteration
for number in 0..5 {
    println!("Number: {}", number);
}

// Reverse range
for number in (1..4).rev() {
    println!("{}!", number);
}

// With enumerate (index + value)
let items = ["apple", "banana", "cherry"];
for (index, item) in items.iter().enumerate() {
    println!("{}: {}", index, item);
}
```

### 2. While Loop with Iterator
```rust
let mut iter = vec![1, 2, 3].into_iter();
while let Some(value) = iter.next() {
    println!("Got: {}", value);
}
```

### 3. Manual Index-Based Iteration
```rust
let arr = [1, 2, 3, 4, 5];

// Traditional C-style (not idiomatic in Rust)
let mut index = 0;
while index < arr.len() {
    println!("arr[{}] = {}", index, arr[index]);
    index += 1;
}

// Better: use enumerate
for (i, value) in arr.iter().enumerate() {
    println!("arr[{}] = {}", i, value);
}
```

## Iterator Methods and Transformations

### Transformation Methods (Lazy)
```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

// map - transform each element
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect();

// filter - keep elements matching predicate
let evens: Vec<&i32> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .collect();

// enumerate - add indices
let indexed: Vec<(usize, &i32)> = numbers.iter()
    .enumerate()
    .collect();

// zip - combine with another iterator
let letters = vec!['a', 'b', 'c'];
let combined: Vec<(i32, char)> = numbers.iter()
    .zip(letters.iter())
    .map(|(&n, &c)| (n, c))
    .collect();

// flat_map - flatten nested structures
let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
let flattened: Vec<i32> = nested.iter()
    .flat_map(|v| v.iter())
    .cloned()
    .collect();
```

### Consuming Methods (Eager)
```rust
let numbers = vec![1, 2, 3, 4, 5];

// collect - consume into collection
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// reduce operations
let sum: i32 = numbers.iter().sum();
let product: i32 = numbers.iter().product();
let max: Option<&i32> = numbers.iter().max();
let min: Option<&i32> = numbers.iter().min();

// fold - custom reduction
let sum_fold: i32 = numbers.iter().fold(0, |acc, x| acc + x);

// find operations
let found: Option<&i32> = numbers.iter().find(|&&x| x > 3);
let position: Option<usize> = numbers.iter().position(|&x| x == 3);

// boolean tests
let any_even = numbers.iter().any(|&x| x % 2 == 0);
let all_positive = numbers.iter().all(|&x| x > 0);

// count
let count = numbers.iter().filter(|&&x| x % 2 == 0).count();
```

## Custom Iterator Implementation

### Basic Custom Iterator
```rust
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// Usage
let sum: usize = Counter::new(5).sum(); // 0 + 1 + 2 + 3 + 4 = 10
let doubled: Vec<usize> = Counter::new(3)
    .map(|x| x * 2)
    .collect(); // [0, 2, 4]
```

### Iterator with Lifetime Parameters
```rust
struct RangeIter<'a, T> {
    slice: &'a [T],
    start: usize,
    end: usize,
    current: usize,
}

impl<'a, T> RangeIter<'a, T> {
    fn new(slice: &'a [T], start: usize, end: usize) -> Self {
        RangeIter {
            slice,
            start,
            end: end.min(slice.len()),
            current: start,
        }
    }
}

impl<'a, T> Iterator for RangeIter<'a, T> {
    type Item = (usize, &'a T);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let index = self.current;
            self.current += 1;
            Some((index, &self.slice[index]))
        } else {
            None
        }
    }
}

// Usage
let data = [10, 20, 30, 40, 50];
for (index, value) in RangeIter::new(&data, 1, 4) {
    println!("data[{}] = {}", index, value);
}
// Output: data[1] = 20, data[2] = 30, data[3] = 40
```

## Specialized Iteration Patterns

### 1. Grid/2D Iteration Patterns
```rust
// Row-major iteration
fn iterate_grid_rows<T>(grid: &[Vec<T>]) {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            println!("grid[{}][{}] = {:?}", row_idx, col_idx, cell);
        }
    }
}

// Coordinate-based iteration
#[derive(Debug, Clone, Copy)]
struct Coord { x: usize, y: usize }

fn iterate_coordinates(width: usize, height: usize) -> impl Iterator<Item = Coord> {
    (0..height).flat_map(move |y| {
        (0..width).map(move |x| Coord { x, y })
    })
}

// Usage
for coord in iterate_coordinates(3, 2) {
    println!("Visiting {:?}", coord);
}
// Output: (0,0), (1,0), (2,0), (0,1), (1,1), (2,1)

// Neighbor iteration (4-connectivity)
fn neighbors_4(coord: Coord, width: usize, height: usize) -> impl Iterator<Item = Coord> {
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    
    deltas.iter().filter_map(move |(dx, dy)| {
        let new_x = coord.x as i32 + dx;
        let new_y = coord.y as i32 + dy;
        
        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            Some(Coord { x: new_x as usize, y: new_y as usize })
        } else {
            None
        }
    })
}
```

### 2. Enum Iteration Patterns
```rust
// Manual array (simple enums)
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North, South, East, West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }
}

// Usage
for direction in Direction::all() {
    println!("Moving {:?}", direction);
}

// Strum-based iteration (recommended for complex enums)
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Color {
    Red, Green, Blue, Yellow, Purple,
}

// Usage
for color in Color::iter() {
    println!("Color: {:?}", color);
}
```

### 3. Tree/Graph Iteration Patterns
```rust
use std::collections::VecDeque;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: Vec<TreeNode>,
}

// Breadth-first iteration
fn bfs_iter(root: &TreeNode) -> impl Iterator<Item = &TreeNode> {
    BfsIterator::new(root)
}

struct BfsIterator<'a> {
    queue: VecDeque<&'a TreeNode>,
}

impl<'a> BfsIterator<'a> {
    fn new(root: &'a TreeNode) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        BfsIterator { queue }
    }
}

impl<'a> Iterator for BfsIterator<'a> {
    type Item = &'a TreeNode;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.pop_front() {
            // Add children to queue
            for child in &node.children {
                self.queue.push_back(child);
            }
            Some(node)
        } else {
            None
        }
    }
}
```

## Performance Patterns and Optimizations

### Zero-Cost Abstractions
```rust
// These compile to identical assembly
fn manual_sum(data: &[i32]) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < data.len() {
        sum += data[i];
        i += 1;
    }
    sum
}

fn iterator_sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn fold_sum(data: &[i32]) -> i32 {
    data.iter().fold(0, |acc, x| acc + x)
}

// All three functions generate the same optimized assembly
```

### Lazy vs Eager Evaluation
```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Lazy - no work done until collect()
let lazy_iter = numbers.iter()
    .map(|x| x * x)          // Not executed yet
    .filter(|&&x| x > 10);   // Not executed yet

// Only now does the work happen
let results: Vec<&i32> = lazy_iter.collect();

// Chain multiple transformations efficiently
let processed: Vec<i32> = numbers.iter()
    .filter(|&&x| x % 2 == 0)  // Keep evens
    .map(|&x| x * x)           // Square them  
    .filter(|&x| x > 10)       // Keep > 10
    .collect();                // Single pass through data!
```

### Memory-Efficient Patterns
```rust
// Bad: Creates intermediate collections
fn process_data_inefficient(data: Vec<i32>) -> Vec<i32> {
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    let filtered: Vec<i32> = doubled.iter().filter(|&&x| x > 10).cloned().collect();
    let mapped: Vec<i32> = filtered.iter().map(|x| x + 1).cloned().collect();
    mapped
}

// Good: Single pass, no intermediate allocations
fn process_data_efficient(data: Vec<i32>) -> Vec<i32> {
    data.iter()
        .map(|x| x * 2)
        .filter(|&&x| x > 10)
        .map(|&x| x + 1)
        .collect()
}

// Even better: Return iterator for maximum flexibility
fn process_data_iterator(data: &[i32]) -> impl Iterator<Item = i32> + '_ {
    data.iter()
        .map(|x| x * 2)
        .filter(|&&x| x > 10)
        .map(|&x| x + 1)
}
```

## Common Iterator Patterns by Use Case

### 1. Data Processing Pipelines
```rust
// Sales data analysis
#[derive(Debug)]
struct Sale {
    amount: f64,
    region: String,
    month: u32,
}

fn analyze_sales(sales: &[Sale]) -> Vec<(String, f64)> {
    use std::collections::HashMap;
    
    sales.iter()
        .filter(|sale| sale.amount > 100.0)          // High-value sales only
        .fold(HashMap::new(), |mut acc, sale| {       // Group by region
            *acc.entry(sale.region.clone()).or_insert(0.0) += sale.amount;
            acc
        })
        .into_iter()                                  // Convert to iterator
        .collect()                                    // Collect as Vec
}

// Functional pipeline style
fn top_regions(sales: &[Sale], limit: usize) -> Vec<String> {
    use std::collections::HashMap;
    
    let mut region_totals: Vec<(String, f64)> = sales.iter()
        .fold(HashMap::new(), |mut acc, sale| {
            *acc.entry(sale.region.clone()).or_insert(0.0) += sale.amount;
            acc
        })
        .into_iter()
        .collect();
    
    region_totals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    region_totals.into_iter()
        .take(limit)
        .map(|(region, _)| region)
        .collect()
}
```

### 2. String Processing
```rust
// Word frequency counting
fn word_frequency(text: &str) -> std::collections::HashMap<String, usize> {
    text.split_whitespace()                    // Split into words
        .map(|word| word.to_lowercase())       // Normalize case
        .filter(|word| word.len() > 2)         // Filter short words
        .fold(std::collections::HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}

// Line processing
fn process_csv_lines(content: &str) -> Vec<Vec<String>> {
    content.lines()                            // Split by lines
        .skip(1)                              // Skip header
        .filter(|line| !line.trim().is_empty()) // Skip empty lines
        .map(|line| {
            line.split(',')                    // Split by comma
                .map(|field| field.trim().to_string()) // Clean fields
                .collect()
        })
        .collect()
}
```

### 3. Numeric Computations
```rust
// Statistical operations
fn statistics(data: &[f64]) -> (f64, f64, f64) {
    let count = data.len() as f64;
    
    let mean = data.iter().sum::<f64>() / count;
    
    let variance = data.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / count;
    
    let std_dev = variance.sqrt();
    
    (mean, variance, std_dev)
}

// Running calculations
fn running_averages(data: &[i32], window_size: usize) -> Vec<f64> {
    data.windows(window_size)
        .map(|window| {
            window.iter().sum::<i32>() as f64 / window_size as f64
        })
        .collect()
}
```

## Error Handling in Iteration

### Result Iterator Patterns
```rust
// Collecting Results
fn parse_numbers(strings: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    strings.iter()
        .map(|s| s.parse::<i32>())
        .collect()  // Fails on first error
}

// Filter successful parses, ignore errors
fn parse_numbers_best_effort(strings: &[&str]) -> Vec<i32> {
    strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect()
}

// Partition successes and failures
fn parse_numbers_partition(strings: &[&str]) -> (Vec<i32>, Vec<String>) {
    let (successes, failures): (Vec<_>, Vec<_>) = strings.iter()
        .map(|s| s.parse::<i32>().map_err(|_| s.to_string()))
        .partition(Result::is_ok);
    
    let numbers = successes.into_iter().map(Result::unwrap).collect();
    let errors = failures.into_iter().map(Result::unwrap_err).collect();
    
    (numbers, errors)
}
```

### Option Iterator Patterns
```rust
// Working with Option iterators
fn find_valid_configs(configs: &[Option<Config>]) -> Vec<Config> {
    configs.iter()
        .filter_map(|opt| opt.as_ref())
        .cloned()
        .collect()
}

// Chaining optional iterators
fn process_optional_data(data: Option<Vec<i32>>) -> Vec<i32> {
    data.into_iter()
        .flatten()                 // Flatten Option<Vec<i32>> to Iterator<Item=i32>
        .filter(|&x| x > 0)       // Keep positive numbers
        .map(|x| x * 2)           // Double them
        .collect()
}
```

## Testing Iterator Implementations

### Unit Tests for Custom Iterators
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_counter_basic() {
        let mut counter = Counter::new(3);
        assert_eq!(counter.next(), Some(0));
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), None);
    }
    
    #[test]
    fn test_counter_collect() {
        let values: Vec<usize> = Counter::new(5).collect();
        assert_eq!(values, vec![0, 1, 2, 3, 4]);
    }
    
    #[test]
    fn test_counter_with_methods() {
        let sum: usize = Counter::new(4).sum();
        assert_eq!(sum, 6); // 0 + 1 + 2 + 3
        
        let doubled: Vec<usize> = Counter::new(3)
            .map(|x| x * 2)
            .collect();
        assert_eq!(doubled, vec![0, 2, 4]);
    }
    
    #[test]
    fn test_size_hint() {
        let counter = Counter::new(5);
        assert_eq!(counter.size_hint(), (5, Some(5)));
        
        let mut counter = Counter::new(3);
        counter.next();
        assert_eq!(counter.size_hint(), (2, Some(2)));
    }
}
```

## Best Practices and Guidelines

### When to Use Each Pattern

#### Use For Loops When:
- **Simple iteration** over collections
- **Clear, readable code** is priority
- **Index access** needed with `enumerate()`
- **Learning Rust** (most beginner-friendly)

```rust
// Good for simple cases
for item in &collection {
    process(item);
}

for (index, item) in collection.iter().enumerate() {
    println!("{}: {:?}", index, item);
}
```

#### Use Iterator Chains When:
- **Data transformation** pipelines
- **Functional programming** style preferred
- **Performance** is critical (zero-cost abstractions)
- **Composability** needed

```rust
// Good for complex transformations
let result: Vec<_> = data.iter()
    .filter(|&x| x > threshold)
    .map(|x| transform(x))
    .collect();
```

#### Use Custom Iterators When:
- **Complex iteration logic** needed
- **Lazy evaluation** beneficial
- **Reusable iteration patterns** across codebase
- **Memory efficiency** important

```rust
// Good for specialized iteration needs
struct WindowIterator<'a, T> {
    slice: &'a [T],
    window_size: usize,
    position: usize,
}

impl<'a, T> Iterator for WindowIterator<'a, T> {
    type Item = &'a [T];
    // ... implementation
}
```

### Performance Considerations

#### Do:
- ✅ **Prefer iterator chains** over manual loops for transformations
- ✅ **Use `collect()` only when necessary** - return iterators when possible
- ✅ **Chain operations** to avoid intermediate collections
- ✅ **Use `size_hint()`** in custom iterators for optimization
- ✅ **Implement `ExactSizeIterator`** when possible

#### Don't:
- ❌ **Create unnecessary intermediate collections**
- ❌ **Use `collect()` just to iterate again**
- ❌ **Ignore borrowing in iterator chains**
- ❌ **Over-complicate simple iteration**

```rust
// Bad: Unnecessary intermediate collection
let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
for item in &doubled {
    println!("{}", item);
}

// Good: Direct iteration
for item in data.iter().map(|x| x * 2) {
    println!("{}", item);
}
```

## Integration with Daily Study and Missions

### Daily Study Applications
- **Day 8 (Vectors)**: Basic iteration patterns with `Vec<T>`
- **Day 13 (Advanced Iterators)**: Transformation and custom iterators
- **Week 2 Focus**: Iterator methods and functional programming patterns

### Mission Integration
- **Mission 3**: Search algorithms using iterator patterns
- **Mission 4**: Linked list iteration implementation
- **Mission 5**: HashMap iteration and key-value processing
- **Mission 6**: Grid iteration for pathfinding algorithms

### Real-World Examples
- **AoC Day 9**: TSP problem using permutation iteration
- **AoC Day 10**: Look-and-say sequence with string iteration
- **Grid Navigation**: 2D coordinate iteration patterns
- **Graph Traversal**: BFS/DFS with custom iterator implementations

---

*Created: 2025-10-19*
*Last Updated: 2025-10-19*

*Tags: #iteration #iterators #for-loops #while-loops #functional-programming #zero-cost-abstractions #performance #data-processing #rust-patterns #collections #custom-iterators #lazy-evaluation*

*Links: [[Collections MOC]] | [[Day08]] | [[Day13]] | [[mission-3]] | [[Mission4 Overview]] | [[Mission5 Overview]] | [[Custom Iterator Implementation]] | [[Performance Engineering]] | [[Functional Programming Patterns]] | [[Zero-Cost Abstractions]] | [[enum-iteration-patterns]] | [[While Let Pattern Deep Dive]]*