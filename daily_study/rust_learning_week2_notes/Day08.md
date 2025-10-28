# Day 8 · Vectors (`Vec<T>`)

## 🔗 Zettelkasten Links
- **Previous**: [[zettelkasten/daily-study/Day07]] - Foundations complete
- **Next**: [[daily-study/Day09]] - String types and UTF-8
- **Concept**: [[Collections MOC]] - Dynamic arrays and collections
- **Mission**: [[Mission1 Overview]] - Stack uses Vec internally
- **Mission**: [[Mission2 Overview]] - Queue ring buffer with Vec
- **Rust Book**: [[Chapter 8.1 - Vectors]] - Storing lists of values
- **Week Summary**: [[daily-study/Day14]] - Collections review

## 📋 Overview
`Vec<T>` is Rust's **dynamic array** - a growable, heap-allocated sequence that's fundamental to Rust programming. It's owned, generic, and provides both safety and performance.

## 🔑 Key Characteristics
- **Growable**: Can dynamically resize at runtime
- **Contiguous**: Elements stored in adjacent memory locations (cache-friendly)
- **Heap-allocated**: Data lives on the heap, stack holds pointer/len/capacity
- **Generic**: `Vec<T>` works with any type T
- **Owned**: Takes ownership of its elements

## 🚀 Creation Patterns

### Empty Vector Creation
```rust
// Explicit type annotation
let mut v1: Vec<i32> = Vec::new();

// Type inference from usage
let mut v2 = Vec::new();
v2.push(42); // Rust infers Vec<i32>

// With pre-allocated capacity (performance optimization)
let mut v3 = Vec::with_capacity(10);

// From macro (convenient)
let v4 = vec![1, 2, 3, 4, 5];

// From iterator
let v5: Vec<i32> = (1..=5).collect();

// From array/slice
let v6 = Vec::from([1, 2, 3]);
```

## 🔧 Core Operations

### Adding Elements
```rust
let mut v = Vec::new();
v.push(1);           // Add to end - O(1) amortized
v.insert(0, 0);      // Insert at index - O(n)
v.extend([2, 3, 4]); // Add multiple - O(k) for k elements
v.append(&mut other_vec); // Move all elements from another vec
```

### Removing Elements
```rust
let mut v = vec![1, 2, 3, 4, 5];
let last = v.pop();         // Remove last - O(1) -> Some(5)
let elem = v.remove(1);     // Remove at index - O(n) -> 2
v.clear();                  // Remove all elements
```

### Accessing Elements
```rust
let v = vec![10, 20, 30];

// Direct indexing (panics if out of bounds)
let first = v[0];           // 10

// Safe access (returns Option)
let first = v.get(0);       // Some(&10)
let missing = v.get(10);    // None

// First and last
let first = v.first();      // Some(&10)
let last = v.last();        // Some(&30)

// Mutable access
let mut v = vec![1, 2, 3];
if let Some(first) = v.get_mut(0) {
    *first = 100;
}
```

## 📊 Memory Model

### Capacity vs Length
```rust
let mut v = Vec::with_capacity(10);
println!("Length: {}, Capacity: {}", v.len(), v.capacity()); // 0, 10

v.push(1);
println!("Length: {}, Capacity: {}", v.len(), v.capacity()); // 1, 10

// Capacity grows when needed (typically doubles)
for i in 0..15 {
    v.push(i);
    println!("Len: {}, Cap: {}", v.len(), v.capacity());
}
```

### Memory Layout
```
Stack:           Heap:
┌─────────────┐  ┌───┬───┬───┬───┬───┬───┬───────┐
│ ptr         │──┤ 1 │ 2 │ 3 │   │   │   │  ...  │
│ len: 3      │  └───┴───┴───┴───┴───┴───┴───────┘
│ capacity: 8 │   ^─── used ───^   ^─ available ─^
└─────────────┘
```

## 🔄 Iteration Patterns

### Different Ways to Iterate
```rust
let v = vec![1, 2, 3, 4, 5];

// Borrowing elements (most common)
for item in &v {
    println!("{}", item);
}

// With indices
for (i, item) in v.iter().enumerate() {
    println!("v[{}] = {}", i, item);
}

// Mutable iteration
let mut v = vec![1, 2, 3];
for item in &mut v {
    *item *= 2;
}

// Taking ownership (consumes vector)
for item in v {
    println!("{}", item); // v is no longer accessible after this
}

// Functional style
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
let sum: i32 = v.iter().sum();
let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
```

## 🎯 Advanced Techniques

### Slicing
```rust
let v = vec![1, 2, 3, 4, 5];
let slice = &v[1..4];       // [2, 3, 4]
let first_two = &v[..2];    // [1, 2]
let last_two = &v[3..];     // [4, 5]
```

### Splitting and Chunking
```rust
let v = vec![1, 2, 3, 4, 5, 6];
let (left, right) = v.split_at(3);  // ([1, 2, 3], [4, 5, 6])

for chunk in v.chunks(2) {
    println!("{:?}", chunk); // [1, 2], [3, 4], [5, 6]
}
```

### Deduplication and Sorting
```rust
let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
v.sort();                   // [1, 1, 2, 3, 4, 5, 6, 9]
v.dedup();                  // [1, 2, 3, 4, 5, 6, 9]

// Custom sorting
let mut strings = vec!["apple", "pie", "a"];
strings.sort_by_key(|s| s.len()); // ["a", "pie", "apple"]
```

## ⚡ Performance Considerations

### Pre-allocation
```rust
// ❌ Inefficient - multiple reallocations
let mut v = Vec::new();
for i in 0..1000 {
    v.push(i);
}

// ✅ Efficient - single allocation
let mut v = Vec::with_capacity(1000);
for i in 0..1000 {
    v.push(i);
}
```

### Avoid Frequent Insertions/Deletions in Middle
```rust
// ❌ O(n) for each operation - expensive
let mut v = vec![1, 2, 3, 4, 5];
v.insert(2, 99);  // Shifts everything after index 2
v.remove(1);      // Shifts everything after index 1

// ✅ O(1) operations at the end
v.push(99);       // Fast
v.pop();          // Fast
```

## 🏗️ Common Patterns in Data Structures

### Stack Implementation
```rust
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);  // O(1) amortized
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()        // O(1)
    }
}
```

### Ring Buffer Foundation
```rust
pub struct RingBuffer<T> {
    data: Vec<Option<T>>,  // Fixed-size Vec with optional elements
    head: usize,
    tail: usize,
}
```

### Dynamic Programming Table
```rust
// 2D DP table using nested vectors
let mut dp: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
```

## 🐛 Common Pitfalls

### Index Out of Bounds
```rust
let v = vec![1, 2, 3];
// ❌ Panics at runtime
let x = v[5];

// ✅ Safe alternative
if let Some(x) = v.get(5) {
    println!("{}", x);
} else {
    println!("Index out of bounds");
}
```

### Borrowing During Modification
```rust
let mut v = vec![1, 2, 3];
// ❌ Won't compile - can't borrow immutably while borrowed mutably
let first = &v[0];
v.push(4);
println!("{}", first);

// ✅ Solution - limit borrow scope
{
    let first = v[0];
    println!("{}", first);
}
v.push(4);
```

## 🎮 Real-World Examples

### Advent of Code Parsing
```rust
// Parse input into Vec<i32>
let numbers: Vec<i32> = input
    .lines()
    .map(|line| line.parse().unwrap())
    .collect();

// 2D grid
let grid: Vec<Vec<char>> = input
    .lines()
    .map(|line| line.chars().collect())
    .collect();
```

### 2D Grid Access Patterns
```rust
let grid: Vec<Vec<char>> = vec![
    vec!['#', '.', '#'],
    vec!['.', 'S', '.'],
    vec!['#', '.', 'E'],
];

// ❌ Direct indexing (unsafe - can panic)
let cell = grid[1][1];  // 'S'

// ✅ Safe access with get()
let cell = grid.get(1).and_then(|row| row.get(1)); // Some(&'S')

// ✅ Helper function for cleaner code
fn get_cell(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Option<char> {
    grid.get(row)?.get(col).copied()
}

// ✅ Bounds checking before access
if row < grid.len() && col < grid[row].len() {
    let cell = grid[row][col]; // Safe to use direct indexing here
}

// ✅ Neighbor access (common AoC pattern)
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<char> {
    DIRECTIONS
        .iter()
        .filter_map(|(dr, dc)| {
            let new_row = (row as isize + dr) as usize;
            let new_col = (col as isize + dc) as usize;
            grid.get(new_row)?.get(new_col).copied()
        })
        .collect()
}

// ✅ Find positions of specific elements
let start_pos: Option<(usize, usize)> = grid
    .iter()
    .enumerate()
    .find_map(|(row, line)| {
        line.iter()
            .enumerate()
            .find_map(|(col, &cell)| {
                if cell == 'S' { Some((row, col)) } else { None }
            })
    });
```

### Coordinate Storage
```rust
// Store (x, y) coordinates
let positions: Vec<(i32, i32)> = vec![(0, 0), (1, 1), (2, 3)];

// Or custom Point struct
#[derive(Debug, Clone)]
struct Point { x: i32, y: i32 }
let points: Vec<Point> = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 1 },
];
```

## ✅ Best Practices

1. **Pre-allocate when size is known**: `Vec::with_capacity(n)`
2. **Use safe indexing**: `get(i)` instead of `[i]` when bounds uncertain
3. **Prefer iterators**: More expressive and often faster
4. **Consider `Vec<T>` vs `&[T]`**: Use slices for function parameters
5. **Clone judiciously**: Vectors are expensive to clone
6. **Use `drain()` for efficient removal**: When you need to remove and use elements

## 🔗 Relationship to Other Concepts

### With HashMap/HashSet
```rust
// Collect HashMap keys/values into Vec
let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into();
let keys: Vec<&str> = map.keys().copied().collect();
let values: Vec<i32> = map.values().copied().collect();
```

### With Iterators
```rust
let numbers: Vec<i32> = (1..=10)
    .filter(|&x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

## 💡 Key Takeaways

- **Foundation of Rust collections**: Understanding `Vec<T>` is crucial for all other data structures
- **Performance-conscious**: Pre-allocation and end-operations are fast
- **Memory-safe**: No buffer overflows or use-after-free
- **Flexible**: Works with any type T and supports functional programming patterns
- **Building block**: Used internally by Stack, Queue, and many other structures

`Vec<T>` is the workhorse of Rust programming - master it, and you've mastered a core piece of the Rust ecosystem! 🦀

---

*Links: [[zettelkasten/daily-study/Day07]] | [[daily-study/Day09]] | [[Collections MOC]] | [[Mission1 Overview]] | [[Mission2 Overview]]*
*Tags: #vector #vec #collections #dynamic-arrays #daily-study #week2 #rust-book #chapter8 #foundation*
