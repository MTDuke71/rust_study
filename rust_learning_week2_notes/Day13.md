# Day 13 · Advanced Iterators (transforming and processing collections)

> **Learning Context**: Day 13 completes Week 2's collections mastery with iterator patterns essential for efficient data processing in Mission5 and AoC problems.

**Cross-Track Integration:**
- **Mission5 Connection**: Iterator patterns for HashMap traversal and batch operations - see [[Mission5 Overview]]
- **Daily Study**: Culminates Week 2 collections (HashMap → BTreeMap → Iterators)
- **Rust Book**: Builds on Chapter 13 Functional Language Features with practical applications

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Iterator patterns across all data structures
- [[HashMap Internals]] - Internal iteration vs external iteration patterns
- [[zettel-index]] - Main learning hub

## Core Concepts

### Iterator Fundamentals
- **Lazy Evaluation**: Iterators do nothing until consumed
- **Zero-Cost Abstractions**: Compile to same performance as manual loops
- **Functional Style**: Chain operations for readable data processing
- **Memory Efficiency**: Process elements one at a time, not entire collections

### The Iterator Trait
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    
    // Default implementations for 70+ methods built on next()!
}

// Three ways to create iterators
let vec = vec![1, 2, 3];

vec.iter()      // Iterator over &T (borrowing)
vec.into_iter() // Iterator over T (consuming)
vec.iter_mut()  // Iterator over &mut T (mutable borrowing)
```

## Iterator Adaptors vs Consuming Adaptors

### Iterator Adaptors (Lazy - Return New Iterators)
```rust
let numbers = vec![1, 2, 3, 4, 5];

// These are lazy - nothing happens yet!
let doubled = numbers.iter().map(|x| x * 2);
let filtered = numbers.iter().filter(|&&x| x % 2 == 0);
let chained = numbers.iter().chain(numbers.iter());

// Still lazy - can be chained infinitely
let complex = numbers
    .iter()
    .map(|x| x * 2)
    .filter(|&&x| x > 4)
    .enumerate()
    .map(|(i, x)| (i, x * 10));

// Nothing computed until consumed!
let result: Vec<_> = complex.collect(); // NOW it runs
```

### Consuming Adaptors (Eager - Produce Final Results)
```rust
let numbers = vec![1, 2, 3, 4, 5];

// These consume the iterator immediately
let sum: i32 = numbers.iter().sum();
let product: i32 = numbers.iter().product();
let count = numbers.iter().count();
let max = numbers.iter().max(); // Option<&i32>

// Collect into different types
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let doubled_set: std::collections::HashSet<i32> = 
    numbers.iter().map(|x| x * 2).collect();
```

## Essential Iterator Methods

### Transformation Methods
```rust
let words = vec!["hello", "world", "rust"];

// map - transform each element
let lengths: Vec<usize> = words.iter().map(|s| s.len()).collect();

// enumerate - add indices
let indexed: Vec<(usize, &str)> = words.iter().enumerate().collect();
// [(0, "hello"), (1, "world"), (2, "rust")]

// zip - combine with another iterator
let numbers = vec![1, 2, 3];
let combined: Vec<(i32, &str)> = numbers.iter().zip(words.iter())
    .map(|(&n, &s)| (n, s))
    .collect();
// [(1, "hello"), (2, "world"), (3, "rust")]

// flat_map - flatten nested structures
let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
let flattened: Vec<i32> = nested.iter().flat_map(|v| v.iter()).cloned().collect();
// [1, 2, 3, 4, 5, 6]
```

### Filtering Methods
```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// filter - keep elements that match predicate
let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();

// filter_map - filter and transform in one step
let even_squares: Vec<i32> = numbers
    .iter()
    .filter_map(|&x| if x % 2 == 0 { Some(x * x) } else { None })
    .collect();

// take/skip - limit or skip elements
let first_three: Vec<i32> = numbers.iter().take(3).cloned().collect();
let skip_first_three: Vec<i32> = numbers.iter().skip(3).cloned().collect();

// take_while/skip_while - conditional taking/skipping
let take_while_small: Vec<i32> = numbers
    .iter()
    .take_while(|&&x| x < 5)
    .cloned()
    .collect();
```

### Searching and Testing
```rust
let numbers = vec![1, 2, 3, 4, 5];

// find - first matching element
let found = numbers.iter().find(|&&x| x > 3); // Some(&4)

// any/all - test conditions
let has_even = numbers.iter().any(|&x| x % 2 == 0); // true
let all_positive = numbers.iter().all(|&x| x > 0); // true

// position - find index
let pos = numbers.iter().position(|&x| x == 3); // Some(2)

// contains - check membership (only for PartialEq types)
let contains_five = numbers.contains(&5); // true
```

## Mission5 Integration: Iterator Patterns for HashMap Operations

### Efficient HashMap Processing (REQ-6 Advanced Operations)
```rust
use std::collections::HashMap;

// Mission5 HashMap with iterator-based batch operations
let mut user_scores: HashMap<String, i32> = HashMap::new();
user_scores.insert("Alice".to_string(), 95);
user_scores.insert("Bob".to_string(), 87);
user_scores.insert("Charlie".to_string(), 92);
user_scores.insert("Diana".to_string(), 78);

// Iterator patterns for Mission5 REQ-6 advanced features:

// 1. Filtering and transforming in one pass (zero-cost abstraction)
let high_performers: Vec<_> = user_scores
    .iter()
    .filter(|(_, &score)| score >= 90)
    .map(|(name, score)| format!("{}: {}", name, score))
    .collect();

// 2. Aggregation operations 
let total_score: i32 = user_scores.values().sum();
let average_score: f64 = user_scores.values().sum::<i32>() as f64 / user_scores.len() as f64;

// 3. Batch updates using iterator chains
let updated_scores: HashMap<String, i32> = user_scores
    .into_iter()
    .map(|(name, score)| (name, score + 5)) // Bonus points
    .filter(|(_, score)| *score <= 100)     // Cap at 100
    .collect();

println!("High performers: {:?}", high_performers);
println!("Average score: {:.1}", average_score);
```

### Iterator Performance vs Manual Loops (Mission5 REQ-5 Validation)
```rust
// Mission5 performance comparison: Iterator vs manual
use std::collections::HashMap;

let data: HashMap<i32, String> = (0..10000).map(|i| (i, format!("value_{}", i))).collect();

// ✅ Iterator approach - compiles to same performance as manual loop
let sum_keys: i32 = data.keys().sum();

// ✅ Manual loop equivalent - same assembly output
let mut manual_sum = 0;
for key in data.keys() {
    manual_sum += key;
}

// Both approaches have identical performance after optimization
// Iterator provides: readability, composability, safety
// Manual provides: explicit control, debugging clarity
```

## Advanced Patterns

### Iterator Chains
```rust
let numbers1 = vec![1, 2, 3];
let numbers2 = vec![4, 5, 6];

// Chain multiple iterators
let combined: Vec<i32> = numbers1
    .iter()
    .chain(numbers2.iter())
    .cloned()
    .collect();
// [1, 2, 3, 4, 5, 6]

// Complex processing pipeline
let result: Vec<String> = (1..=10)
    .filter(|&x| x % 2 == 0)           // Keep evens: [2, 4, 6, 8, 10]
    .map(|x| x * x)                    // Square: [4, 16, 36, 64, 100]
    .filter(|&x| x < 50)               // Filter: [4, 16, 36]
    .map(|x| format!("#{}", x))        // Format: ["#4", "#16", "#36"]
    .collect();
```

### Folding and Reducing
```rust
let numbers = vec![1, 2, 3, 4, 5];

// fold - accumulate with initial value
let sum = numbers.iter().fold(0, |acc, &x| acc + x); // 15
let product = numbers.iter().fold(1, |acc, &x| acc * x); // 120

// reduce - accumulate without initial value
let sum_reduce = numbers.iter().cloned().reduce(|acc, x| acc + x); // Some(15)
let max_reduce = numbers.iter().cloned().reduce(|acc, x| acc.max(x)); // Some(5)

// Complex folding
let word_lengths = vec!["hello", "world", "rust", "programming"];
let stats = word_lengths.iter().fold((0, 0), |(count, total_len), word| {
    (count + 1, total_len + word.len())
});
// (4, 23) - 4 words, 23 total characters
```

### Grouping and Partitioning
```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// partition - split into two collections
let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
    .iter()
    .cloned()
    .partition(|&x| x % 2 == 0);

// Manual grouping using fold
use std::collections::HashMap;

let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
let grouped: HashMap<char, Vec<&str>> = words
    .iter()
    .fold(HashMap::new(), |mut acc, &word| {
        let first_char = word.chars().next().unwrap();
        acc.entry(first_char).or_insert(Vec::new()).push(word);
        acc
    });
```

## Custom Iterators

### Creating Custom Iterator Types
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
```

### Iterator Extensions
```rust
// Extend Iterator trait with custom methods
trait IteratorExt: Iterator {
    fn batch(self, size: usize) -> BatchIterator<Self>
    where
        Self: Sized,
    {
        BatchIterator::new(self, size)
    }
}

impl<I: Iterator> IteratorExt for I {}

// Usage
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
let batches: Vec<Vec<i32>> = numbers
    .iter()
    .cloned()
    .batch(3)
    .collect();
// [[1, 2, 3], [4, 5, 6], [7, 8]]
```

## Performance and Best Practices

### Zero-Cost Abstractions in Action
```rust
// This iterator chain...
let result: Vec<i32> = (0..1_000_000)
    .filter(|&x| x % 2 == 0)
    .map(|x| x * x)
    .take(1000)
    .collect();

// ...compiles to essentially the same assembly as:
let mut result = Vec::with_capacity(1000);
let mut count = 0;
let mut i = 0;
while count < 1000 && i < 1_000_000 {
    if i % 2 == 0 {
        result.push(i * i);
        count += 1;
    }
    i += 1;
}
```

### Memory Efficiency
```rust
// ✅ Iterator - processes one element at a time
fn process_large_file(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let line_count = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| line.starts_with("ERROR"))
        .count();
    
    Ok(line_count)
}

// ❌ Collection - loads everything into memory
fn process_large_file_bad(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let lines: Vec<&str> = content.lines().collect(); // Entire file in memory!
    let error_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("ERROR"))
        .cloned()
        .collect();
    
    Ok(error_lines.len())
}
```

## AoC Applications

### Grid Processing
```rust
type Point = (usize, usize);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_all_positions(grid: &[Vec<char>], target: char) -> Vec<Point> {
    grid.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col, &ch)| {
                    if ch == target {
                        Some((row, col))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

// Usage
let input = "..#..\n.#.#.\n#...#";
let grid = parse_grid(input);
let rocks = find_all_positions(&grid, '#');
```

### Data Parsing and Transformation
```rust
fn parse_input(input: &str) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
    input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(format!("Invalid line {}: {}", line_num + 1, line).into());
            }
            
            let name = parts[0].to_string();
            let value = parts[1].parse::<i32>()
                .map_err(|e| format!("Invalid number on line {}: {}", line_num + 1, e))?;
            
            Ok((name, value))
        })
        .collect()
}
```

### Statistical Analysis
```rust
fn analyze_numbers(numbers: &[i32]) -> (f64, i32, i32, f64) {
    let count = numbers.len() as f64;
    
    let sum: i32 = numbers.iter().sum();
    let mean = sum as f64 / count;
    
    let min = *numbers.iter().min().unwrap_or(&0);
    let max = *numbers.iter().max().unwrap_or(&0);
    
    let variance = numbers
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / count;
    
    (mean, min, max, variance.sqrt())
}
```

## Common Patterns and Idioms

### Option and Result with Iterators
```rust
// Working with Option
let numbers = vec![Some(1), None, Some(3), Some(4), None];

// filter_map removes None values
let valid: Vec<i32> = numbers.into_iter().filter_map(|x| x).collect();

// Working with Result
let strings = vec!["1", "not_a_number", "3", "4"];
let parsed: Result<Vec<i32>, _> = strings
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();

// Separate successes from failures
let results: Vec<Result<i32, _>> = strings
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();

let (successes, failures): (Vec<_>, Vec<_>) = results
    .into_iter()
    .partition(|r| r.is_ok());
```

### String Processing
```rust
fn word_frequency(text: &str) -> std::collections::HashMap<String, usize> {
    text.split_whitespace()
        .map(|word| word.to_lowercase())
        .map(|word| word.chars().filter(|c| c.is_alphabetic()).collect())
        .filter(|word: &String| !word.is_empty())
        .fold(std::collections::HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}
```

## Learning Progression Summary

From Day 13, you should understand:
1. **Lazy Evaluation**: Iterator chains don't execute until consumed
2. **Zero-Cost Abstractions**: Iterator code compiles to optimal loops
3. **Functional Style**: Chain operations for readable data processing
4. **Memory Efficiency**: Process elements one at a time
5. **Rich API**: 70+ methods for transforming, filtering, and consuming data
6. **Custom Iterators**: Implement Iterator trait for custom types
7. **Real-World Applications**: File processing, data parsing, statistical analysis

**Next**: Day 14 will cover **Error Handling Patterns** - robust error management in Rust applications!

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
use std::collections::HashMap;

fn main() {
    println!("=== Advanced Iterators Demo from Day 13 ===\n");
    
    // 1. Lazy evaluation demonstration
    println!("1. Lazy Evaluation - Chains Don't Execute Until Consumed:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let lazy_chain = numbers
        .iter()
        .map(|x| { println!("  Processing: {}", x); x * 2 })
        .filter(|&x| x > 10);
    
    println!("   Chain created, but nothing printed yet!");
    println!("   Now consuming with collect():");
    let results: Vec<_> = lazy_chain.collect();
    println!("   Results: {:?}\n", results);
    
    // 2. Complex iterator chains (typical AoC pattern)
    println!("2. Complex Iterator Chains (AoC Style):");
    let data = vec!["123", "invalid", "456", "789", "bad"];
    
    let processed: Vec<i32> = data
        .iter()
        .enumerate()
        .inspect(|(i, s)| println!("   Step {}: Processing '{}'", i + 1, s))
        .filter_map(|(_, s)| s.parse::<i32>().ok())
        .map(|n| n * 10)
        .filter(|&n| n > 1000)
        .collect();
    
    println!("   Final results: {:?}\n", processed);
    
    // 3. Functional data processing
    println!("3. Functional Data Processing:");
    let sales_data = vec![
        ("Alice", vec![100, 200, 150]),
        ("Bob", vec![80, 120, 200]),
        ("Charlie", vec![300, 50, 100]),
    ];
    
    let top_performers: Vec<_> = sales_data
        .iter()
        .map(|(name, sales)| {
            let total: i32 = sales.iter().sum();
            (*name, total)
        })
        .filter(|(_, total)| *total > 300)
        .collect();
    
    println!("   Top performers (>300): {:?}\n", top_performers);
    
    // 4. Custom iterator implementation
    println!("4. Custom Iterator - Fibonacci Sequence:");
    struct Fibonacci {
        a: u64,
        b: u64,
    }
    
    impl Fibonacci {
        fn new() -> Self {
            Fibonacci { a: 0, b: 1 }
        }
    }
    
    impl Iterator for Fibonacci {
        type Item = u64;
        
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.a;
            let next = self.a + self.b;
            self.a = self.b;
            self.b = next;
            Some(current)
        }
    }
    
    let fib_numbers: Vec<_> = Fibonacci::new().take(10).collect();
    println!("   First 10 Fibonacci numbers: {:?}\n", fib_numbers);
    
    // 5. Real-world text processing
    println!("5. Text Processing - Word Frequency:");
    let text = "hello world hello rust world programming rust is great";
    
    let word_freq: HashMap<&str, usize> = text
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        });
    
    println!("   Word frequencies:");
    for (word, count) in &word_freq {
        println!("     '{}': {}", word, count);
    }
    
    // 6. Iterator performance vs manual loops
    println!("\n6. Zero-Cost Abstractions:");
    let large_data: Vec<i32> = (1..=1000).collect();
    
    // Iterator version (compiles to same assembly as manual loop)
    let sum_iter: i32 = large_data.iter().map(|x| x * x).sum();
    
    // Manual loop version
    let mut sum_manual = 0;
    for &x in &large_data {
        sum_manual += x * x;
    }
    
    println!("   Iterator sum: {}", sum_iter);
    println!("   Manual sum: {}", sum_manual);
    println!("   Both compile to identical optimized assembly!");
    
    // 7. Partition and collect patterns
    println!("\n7. Partition and Collect Patterns:");
    let mixed_results: Vec<Result<i32, &str>> = vec![
        Ok(1), Err("error1"), Ok(2), Ok(3), Err("error2"), Ok(4)
    ];
    
    let (successes, failures): (Vec<_>, Vec<_>) = mixed_results
        .into_iter()
        .partition(|r| r.is_ok());
    
    let success_values: Vec<i32> = successes.into_iter().map(|r| r.unwrap()).collect();
    let error_messages: Vec<&str> = failures.into_iter().map(|r| r.unwrap_err()).collect();
    
    println!("   Successes: {:?}", success_values);
    println!("   Errors: {:?}", error_messages);
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day13_demo.rs` and run `rustc day13_demo.rs && ./day13_demo`
3. **In this workspace**: `.\run_md.bat rust_learning_week2_notes\Day13.md`
4. **As Cargo example**: `cargo run --example day13_iterators_demo` (if you add it to Mission5_tut)

**3-Track Learning Integration:**
- **Mission5**: Iterator patterns enable efficient HashMap batch operations and traversal
- **Week 2 Completion**: HashMap basics → BTreeMap ordering → Iterator mastery
- **AoC Applications**: Data transformation pipelines, filtering, and aggregation patterns

**Cross-References:**
- [[Collections MOC]] - See "Iterator Patterns" section for cross-collection usage
- [[Mission5 Overview]] - REQ-6 advanced operations benefit from iterator chains
- [[HashMap Internals]] - Internal vs external iteration performance considerations

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[Mission5 Overview]] | [[HashMap Internals]] | [[zettel-index]]*

*Tags: #iterators #advanced-patterns #data-processing #collections #daily-study #week2 #functional-programming #mission5*