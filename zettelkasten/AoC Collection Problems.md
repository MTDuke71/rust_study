# AoC Collection Problems - Comprehensive Guide

**A systematic reference for using Rust collections to solve Advent of Code challenges efficiently**

## 🎯 Overview

Collections are the **foundation** of most AoC solutions. Understanding when and how to use each collection type can transform a difficult problem into an elegant, efficient solution. This guide provides patterns, examples, and performance considerations for competitive programming success.

---

## 📊 Collection Type Decision Matrix

| Problem Pattern | Primary Collection | Secondary Options | Key Operations |
|----------------|-------------------|------------------|----------------|
| **Frequency Counting** | `HashMap<T, usize>` | `BTreeMap<T, usize>` | `.entry().or_insert()`, `.get()` |
| **Unique Elements** | `HashSet<T>` | `BTreeSet<T>` | `.insert()`, `.contains()` |
| **Dynamic Arrays** | `Vec<T>` | `VecDeque<T>` | `.push()`, `.pop()`, indexing |
| **Queue Operations** | `VecDeque<T>` | `Vec<T>` | `.push_back()`, `.pop_front()` |
| **String Processing** | `String`, `Vec<char>` | `&str` slices | `.chars()`, `.split()`, `.parse()` |
| **2D Grids** | `Vec<Vec<T>>` | `HashMap<(i32,i32), T>` | `grid[y][x]`, neighbor iteration |
| **Lookup Tables** | `HashMap<K, V>` | `Vec<V>` (if K is index) | `.get()`, pattern matching |
| **Ordered Processing** | `BTreeMap<K, V>` | `Vec<(K, V)>` + sort | `.range()`, ordered iteration |

---

## 🗂️ HashMap/HashSet Patterns

### **Frequency Analysis Pattern**
*Most common AoC pattern - appears in ~30% of problems*

```rust
use std::collections::HashMap;

// Character frequency counting (AoC 2018 Day 2 style)
fn count_characters(text: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for ch in text.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

// Word frequency with parsing
fn count_words(input: &str) -> HashMap<String, usize> {
    input.lines()
        .flat_map(|line| line.split_whitespace())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word.to_string()).or_insert(0) += 1;
            acc
        })
}

// Multi-dimensional counting (coordinates, states, etc.)
fn count_positions(moves: &str) -> HashMap<(i32, i32), usize> {
    let mut position = (0, 0);
    let mut visits = HashMap::new();
    *visits.entry(position).or_insert(0) += 1; // Starting position
    
    for direction in moves.chars() {
        match direction {
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            _ => continue,
        }
        *visits.entry(position).or_insert(0) += 1;
    }
    visits
}
```

### **Set Operations for Deduplication**
```rust
use std::collections::HashSet;

// Finding unique elements across groups (AoC 2020 Day 6)
fn unique_answers(groups: &[Vec<String>]) -> Vec<usize> {
    groups.iter().map(|group| {
        group.iter()
            .flat_map(|person| person.chars())
            .collect::<HashSet<_>>()
            .len()
    }).collect()
}

// Set intersection for common elements
fn common_items(group: &[String]) -> HashSet<char> {
    if group.is_empty() { return HashSet::new(); }
    
    group.iter()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .reduce(|acc, set| acc.intersection(&set).cloned().collect())
        .unwrap_or_default()
}

// Coordinate tracking (AoC 2015 Day 3)
fn unique_houses(directions: &str) -> usize {
    let mut visited = HashSet::new();
    let mut pos = (0, 0);
    visited.insert(pos);
    
    for dir in directions.chars() {
        match dir {
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            _ => continue,
        }
        visited.insert(pos);
    }
    visited.len()
}
```

---

## 📈 Vector Patterns

### **Dynamic Array Operations**
```rust
// Growing sequences (Fibonacci, population growth, etc.)
fn simulate_growth(initial: Vec<u64>, days: usize) -> Vec<u64> {
    let mut population = initial;
    for _ in 0..days {
        let births = population[0];
        population.rotate_left(1);
        population[6] += births;
        population[8] = births;
    }
    population
}

// Sliding window operations
fn sliding_window_sums(nums: &[i32], window_size: usize) -> Vec<i32> {
    nums.windows(window_size)
        .map(|window| window.iter().sum())
        .collect()
}

// Two-pointer technique for sorted arrays
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left < right {
        let sum = nums[left] + nums[right];
        match sum.cmp(&target) {
            std::cmp::Ordering::Equal => return Some((left, right)),
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
        }
    }
    None
}
```

### **Stack Operations with Vec**
```rust
// Bracket matching and parsing
fn validate_brackets(input: &str) -> bool {
    let mut stack = Vec::new();
    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => continue,
        }
    }
    stack.is_empty()
}

// Depth-first traversal with explicit stack
fn dfs_iterative<T>(graph: &HashMap<T, Vec<T>>, start: T) -> Vec<T> 
where T: Clone + Eq + std::hash::Hash {
    let mut visited = HashSet::new();
    let mut stack = vec![start.clone()];
    let mut result = Vec::new();
    
    while let Some(node) = stack.pop() {
        if !visited.contains(&node) {
            visited.insert(node.clone());
            result.push(node.clone());
            
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors.iter().rev() { // Reverse for consistent order
                    if !visited.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }
    }
    result
}
```

---

## 🔤 String Collection Patterns

### **String Parsing and Transformation**
```rust
// Multi-step string processing pipeline
fn process_input_lines(input: &str) -> Vec<(String, i32, char)> {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some((
                    parts[0].to_string(),
                    parts[1].parse().ok()?,
                    parts[2].chars().next()?
                ))
            } else {
                None
            }
        })
        .collect()
}

// Character manipulation for ciphers/encoding
fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars().map(|c| {
        if c.is_ascii_lowercase() {
            ((c as u8 - b'a' + shift) % 26 + b'a') as char
        } else if c.is_ascii_uppercase() {
            ((c as u8 - b'A' + shift) % 26 + b'A') as char
        } else {
            c
        }
    }).collect()
}

// String building with capacity optimization
fn build_large_string(parts: &[&str]) -> String {
    let total_len: usize = parts.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len + parts.len()); // +separators
    
    for (i, part) in parts.iter().enumerate() {
        if i > 0 { result.push(' '); }
        result.push_str(part);
    }
    result
}
```

---

## 🗺️ Grid and 2D Collection Patterns

### **2D Grid Representation**
```rust
// Dense grid with Vec<Vec<T>>
type Grid<T> = Vec<Vec<T>>;

fn create_grid<T: Clone>(width: usize, height: usize, default: T) -> Grid<T> {
    vec![vec![default; width]; height]
}

// Sparse grid with HashMap for large/infinite grids
type SparseGrid<T> = HashMap<(i32, i32), T>;

// Grid navigation helpers
const DIRECTIONS_4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // N, E, S, W
const DIRECTIONS_8: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1)
];

fn get_neighbors_4(pos: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    DIRECTIONS_4.iter().map(move |(dx, dy)| (pos.0 + dx, pos.1 + dy))
}

// Flood fill algorithm
fn flood_fill(grid: &mut Grid<char>, start: (usize, usize), new_char: char) -> usize {
    let original = grid[start.1][start.0];
    if original == new_char { return 0; }
    
    let mut count = 0;
    let mut stack = vec![start];
    
    while let Some((x, y)) = stack.pop() {
        if grid[y][x] != original { continue; }
        
        grid[y][x] = new_char;
        count += 1;
        
        // Add valid neighbors
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if ny < grid.len() && nx < grid[ny].len() && grid[ny][nx] == original {
                    stack.push((nx, ny));
                }
            }
        }
    }
    count
}
```

---

## 🎲 Real AoC Problem Examples

### **2015 Day 1: Floor Navigation**
```rust
// Simple character counting with fold
fn final_floor(instructions: &str) -> i32 {
    instructions.chars().fold(0, |floor, ch| {
        match ch {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        }
    })
}

fn basement_position(instructions: &str) -> Option<usize> {
    let mut floor = 0;
    for (pos, ch) in instructions.chars().enumerate() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 {
            return Some(pos + 1); // 1-indexed
        }
    }
    None
}
```

### **2015 Day 3: Grid Navigation with HashSet**
```rust
fn santa_houses(directions: &str) -> usize {
    let mut visited = HashSet::new();
    let mut pos = (0, 0);
    visited.insert(pos);
    
    for dir in directions.chars() {
        match dir {
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            _ => continue,
        }
        visited.insert(pos);
    }
    visited.len()
}

// Part 2: Two actors (Santa + Robo-Santa)
fn santa_and_robot_houses(directions: &str) -> usize {
    let mut visited = HashSet::new();
    let mut santa_pos = (0, 0);
    let mut robot_pos = (0, 0);
    visited.insert((0, 0));
    
    for (i, dir) in directions.chars().enumerate() {
        let pos = if i % 2 == 0 { &mut santa_pos } else { &mut robot_pos };
        
        match dir {
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            _ => continue,
        }
        visited.insert(*pos);
    }
    visited.len()
}
```

### **2018 Day 2: Checksum with HashMap**
```rust
fn checksum(box_ids: &[&str]) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    
    for id in box_ids {
        let counts = count_characters(id);
        if counts.values().any(|&count| count == 2) { twos += 1; }
        if counts.values().any(|&count| count == 3) { threes += 1; }
    }
    
    twos * threes
}

fn find_similar_boxes(box_ids: &[&str]) -> Option<String> {
    for i in 0..box_ids.len() {
        for j in (i + 1)..box_ids.len() {
            let id1 = box_ids[i];
            let id2 = box_ids[j];
            
            let diffs: Vec<_> = id1.chars().zip(id2.chars())
                .enumerate()
                .filter(|(_, (c1, c2))| c1 != c2)
                .collect();
            
            if diffs.len() == 1 {
                let common: String = id1.chars().zip(id2.chars())
                    .filter(|(c1, c2)| c1 == c2)
                    .map(|(c1, _)| c1)
                    .collect();
                return Some(common);
            }
        }
    }
    None
}
```

### **2020 Day 1: Two-Sum/Three-Sum with Vec**
```rust
// Two-sum variant
fn find_two_entries(expenses: &[i32], target: i32) -> Option<(i32, i32)> {
    let expense_set: HashSet<i32> = expenses.iter().cloned().collect();
    
    for &expense in expenses {
        let complement = target - expense;
        if expense_set.contains(&complement) && expense != complement {
            return Some((expense, complement));
        }
    }
    None
}

// Three-sum variant
fn find_three_entries(expenses: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            for k in (j + 1)..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k] == target {
                    return Some((expenses[i], expenses[j], expenses[k]));
                }
            }
        }
    }
    None
}
```

---

## ⚡ Performance Optimization Strategies

### **Memory Allocation Patterns**
```rust
// Pre-allocate when size is known
fn efficient_collection_building(input_size: usize) -> Vec<String> {
    let mut result = Vec::with_capacity(input_size);
    // ... populate result
    result
}

// Use entry API for complex HashMap operations
fn efficient_counting(items: &[String]) -> HashMap<String, Vec<usize>> {
    let mut map = HashMap::with_capacity(items.len());
    for (i, item) in items.iter().enumerate() {
        map.entry(item.clone()).or_insert_with(Vec::new).push(i);
    }
    map
}

// Reuse allocations when possible
fn process_batches(batches: &[Vec<String>]) -> Vec<String> {
    let mut buffer = String::new();
    let mut results = Vec::new();
    
    for batch in batches {
        buffer.clear(); // Reuse allocation
        for item in batch {
            buffer.push_str(item);
            buffer.push(' ');
        }
        results.push(buffer.trim().to_string());
    }
    results
}
```

### **Iterator vs Loop Performance**
```rust
// Iterator chains (often fastest due to LLVM optimization)
fn functional_approach(numbers: &[i32]) -> i32 {
    numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}

// Explicit loop (more control, sometimes clearer)
fn imperative_approach(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        if num % 2 == 0 {
            sum += num * num;
        }
    }
    sum
}

// Hybrid approach for complex logic
fn hybrid_approach(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(numbers.len() / 2); // Estimate
    for &num in numbers {
        if num % 2 == 0 {
            let processed = complex_calculation(num);
            if processed > 0 {
                result.push(processed);
            }
        }
    }
    result
}
```

---

## 🔧 Common Troubleshooting Patterns

### **Ownership Issues**
```rust
// ❌ Common mistake: borrowing while mutating
fn bad_example(map: &mut HashMap<String, Vec<i32>>) {
    for key in map.keys() {
        map.get_mut(key).unwrap().push(42); // ❌ Borrow checker error
    }
}

// ✅ Solution: collect keys first
fn good_example(map: &mut HashMap<String, Vec<i32>>) {
    let keys: Vec<String> = map.keys().cloned().collect();
    for key in keys {
        map.get_mut(&key).unwrap().push(42);
    }
}

// ✅ Better: use entry API
fn better_example(map: &mut HashMap<String, Vec<i32>>) {
    for value_vec in map.values_mut() {
        value_vec.push(42);
    }
}
```

### **Performance Pitfalls**
```rust
// ❌ Inefficient: repeated allocations
fn slow_string_building(words: &[&str]) -> String {
    let mut result = String::new();
    for word in words {
        result = result + word + " "; // Creates new String each time!
    }
    result
}

// ✅ Efficient: single allocation
fn fast_string_building(words: &[&str]) -> String {
    let capacity = words.iter().map(|s| s.len() + 1).sum();
    let mut result = String::with_capacity(capacity);
    for word in words {
        result.push_str(word);
        result.push(' ');
    }
    result.trim_end().to_string()
}

// ❌ Inefficient: Vec inside loop
fn slow_processing(data: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    for &item in data {
        let mut group = Vec::new(); // New allocation each time
        for i in 0..item {
            group.push(i);
        }
        results.push(group);
    }
    results
}

// ✅ Efficient: reuse buffer when possible
fn fast_processing(data: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::with_capacity(data.len());
    let mut buffer = Vec::new();
    
    for &item in data {
        buffer.clear(); // Reuse allocation
        buffer.extend(0..item);
        results.push(buffer.clone()); // Only clone when needed
    }
    results
}
```

---

## 📝 Code Templates

### **Standard Parsing Template**
```rust
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn parse_aoc_input(input: &str) -> Result<Vec<ProcessedLine>, Box<dyn std::error::Error>> {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<ProcessedLine>())
        .collect()
}

#[derive(Debug)]
struct ProcessedLine {
    // Define fields based on problem
}

impl FromStr for ProcessedLine {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Custom parsing logic
        todo!()
    }
}
```

### **Collection-Based Solution Template**
```rust
fn solve_part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 1. Parse input into appropriate collection
    let data = parse_input(input)?;
    
    // 2. Choose primary collection based on problem pattern
    let mut state = HashMap::new(); // or HashSet, Vec, etc.
    
    // 3. Process data with collection operations
    for item in data {
        // Apply problem logic
        process_item(&mut state, item);
    }
    
    // 4. Extract answer from collection
    let answer = calculate_answer(&state);
    
    Ok(answer.to_string())
}

fn process_item<T>(state: &mut HashMap<String, T>, item: ProcessedLine) {
    // Problem-specific processing
}

fn calculate_answer<T>(state: &HashMap<String, T>) -> usize {
    // Extract final answer
    state.len() // or other collection-based calculation
}
```

---

## 🔗 Integration Points

### **Mission Connections**
- **[[../Mission2/README]]**: Queue operations and VecDeque usage patterns
- **[[../Mission5/README]]**: HashMap/HashSet implementation details and performance
- **[[../Mission6/README]]**: Grid patterns and 2D coordinate systems
- **[[../Mission1/README]]**: Stack operations using Vec as foundation

### **Daily Study Connections**
- **[[../daily_study/rust_learning_week2_notes/Day08]]**: Vector fundamentals and capacity management
- **[[../daily_study/rust_learning_week2_notes/Day09]]**: String processing and UTF-8 considerations  
- **[[../daily_study/rust_learning_week2_notes/Day10]]**: HashMap operations and entry API patterns
- **[[../daily_study/rust_learning_week2_notes/Day11]]**: HashSet operations and set theory applications

### **Rust Book References**
- **[[../Ch8/vectors/README]]**: Vector details and performance characteristics
- **[[../Ch8/strings/README]]**: String vs &str usage patterns
- **[[../Ch8/hashmaps/README]]**: HashMap API and ownership considerations

---

## 🎯 Quick Reference

### **When to Use Each Collection**

| Use Case | Collection | Reason |
|----------|------------|---------|
| Count frequencies | `HashMap<T, usize>` | O(1) insert/lookup, handles any hashable type |
| Track unique items | `HashSet<T>` | O(1) membership testing, automatic deduplication |
| Ordered processing | `BTreeMap<T, U>` | Maintains sort order, range queries |
| Dynamic arrays | `Vec<T>` | Best general-purpose collection, cache-friendly |
| Queue operations | `VecDeque<T>` | Efficient front/back insertion, ring buffer |
| String building | `String` + `push_str()` | Minimizes allocations vs concatenation |
| 2D dense data | `Vec<Vec<T>>` | Cache-friendly for small/medium grids |
| 2D sparse data | `HashMap<(i32,i32), T>` | Memory efficient for large/infinite grids |

### **Performance Rules of Thumb**
1. **Pre-allocate** when size is known: `Vec::with_capacity()`, `HashMap::with_capacity()`
2. **Prefer iterators** for functional operations: `.map()`, `.filter()`, `.fold()`
3. **Use entry API** for complex HashMap operations: `.entry().or_insert()`
4. **Reuse allocations** in loops: `.clear()` instead of new collections
5. **Profile before optimizing**: Measure actual performance bottlenecks

---

*Tags: #collections #hashmap #hashset #vector #aoc #competitive-programming #patterns #performance #reference #troubleshooting*
*Links: [[../Mission2/README]] | [[../Mission5/README]] | [[../Mission6/README]] | [[../daily_study/rust_learning_week2_notes/README]] | [[Rust Collections MOC]] | [[AoC Patterns MOC]] | [[Performance Optimization Guide]]*