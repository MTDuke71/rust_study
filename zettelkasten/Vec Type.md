# Vec - Dynamic Arrays in Rust

**Tags:** #rust #collections #data-structures #memory #ownership
**Created:** 2025-10-22
**Related:** [[HashMap]], [[Collections MOC]], [[Ownership and Borrowing]], [[zettelkasten/rust_book/rust-book-ch8]]

## Overview

`Vec<T>` is Rust's growable array type, implemented as a heap-allocated buffer with automatic memory management. It's one of the most fundamental collection types in Rust.

## Key Concepts

### Creation and Initialization

```rust
// Empty vector
let mut v: Vec<i32> = Vec::new();

// Using vec! macro
let v = vec![1, 2, 3, 4, 5];

// With capacity
let mut v = Vec::with_capacity(10);

// From iterator
let v: Vec<i32> = (0..5).collect();
```

### Common Operations

#### Adding Elements
```rust
let mut v = Vec::new();
v.push(5);                    // Add to end
v.insert(0, 42);             // Insert at index
v.extend([1, 2, 3]);         // Add multiple
```

#### Accessing Elements
```rust
let v = vec![1, 2, 3, 4, 5];

// Indexing (panics if out of bounds)
let third = v[2];

// Safe access
let third = v.get(2);        // Returns Option<&T>

// First and last
let first = v.first();       // Option<&T>
let last = v.last();         // Option<&T>
```

#### Removing Elements
```rust
let mut v = vec![1, 2, 3, 4, 5];
let last = v.pop();          // Removes and returns Option<T>
let third = v.remove(2);     // Remove at index, returns T
v.clear();                   // Remove all elements
```

### Memory Layout and Performance

- **Contiguous memory:** Elements stored in sequence for cache efficiency
- **Exponential growth:** Capacity typically doubles when exceeded
- **Move semantics:** Ownership transfer prevents unnecessary copies

```rust
let mut v = Vec::new();
println!("Capacity: {}", v.capacity());  // 0
v.push(1);
println!("Capacity: {}", v.capacity());  // Usually 4
// Grows: 0 -> 4 -> 8 -> 16 -> 32...
```

## Common Patterns

### Stack Implementation
```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
```

### Iteration Patterns
```rust
let v = vec![1, 2, 3, 4, 5];

// By reference
for item in &v {
    println!("{}", item);
}

// By value (consumes vec)
for item in v {
    println!("{}", item);
}

// With indices
for (i, item) in v.iter().enumerate() {
    println!("{}: {}", i, item);
}

// Functional style
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

### Deduplication
```rust
let mut v = vec![1, 2, 2, 3, 2, 4];
v.sort();
v.dedup();  // Removes consecutive duplicates
// Result: [1, 2, 3, 4]
```

## Advanced Usage

### Custom Types
```rust
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

let mut people: Vec<Person> = Vec::new();
people.push(Person {
    name: "Alice".to_string(),
    age: 30,
});
```

### Slice Conversions
```rust
let v = vec![1, 2, 3, 4, 5];
let slice: &[i32] = &v;       // Vec to slice
let slice = &v[1..4];         // Subslice [2, 3, 4]
```

### Performance Tips

1. **Pre-allocate when size is known**
   ```rust
   let mut v = Vec::with_capacity(1000);  // Avoids reallocations
   ```

2. **Prefer `extend()` over multiple `push()` calls**
   ```rust
   v.extend([1, 2, 3, 4, 5]);  // More efficient than 5 pushes
   ```

3. **Use `shrink_to_fit()` to reclaim memory**
   ```rust
   v.shrink_to_fit();  // Reduce capacity to length
   ```

## Common Pitfalls

### Borrowing During Modification
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];        // Borrow
// v.push(4);             // ERROR: Can't mutate while borrowed
println!("{}", first);
```

### Index Out of Bounds
```rust
let v = vec![1, 2, 3];
// let x = v[10];         // PANIC: index out of bounds
let x = v.get(10);        // Safe: returns None
```

## Integration with Other Collections

- **Converting to [[HashMap]]:** Use `into_iter()` with key-value pairs
- **With [[Stack Data Structure]]:** Vec provides efficient stack operations
- **Sorting:** Built-in `sort()`, `sort_by()`, `sort_unstable()` methods

## Use Cases in Rust Study Projects

### Advent of Code Applications
- **Day 1-9:** Input parsing and number collections
- **Graph problems:** Adjacency lists
- **Dynamic programming:** Memoization arrays

### Mission Projects  
- **Mission 1:** Basic collection operations
- **Mission 5:** HashMap backing storage
- **Bracket matching:** Character stack implementation

### Advanced Examples
- **Brackets:** Token storage and validation
- **Competitive programming:** Fast I/O and result collections

## Related Concepts

- [[Ownership and Borrowing]] - Understanding Vec's memory management
- [[HashMap]] - Key-value alternative to indexed access  
- [[Stack Data Structure]] - Vec as stack implementation
- [[Collections MOC]] - Overview of all Rust collections
- [[zettelkasten/rust_book/rust-book-ch8]] - The Rust Book's collections chapter

## Quick Reference

```rust
// Creation
Vec::new(), vec![], Vec::with_capacity()

// Access  
v[i], v.get(i), v.first(), v.last()

// Modification
v.push(), v.pop(), v.insert(), v.remove()

// Iteration
for item in &v, v.iter(), v.into_iter()

// Size
v.len(), v.is_empty(), v.capacity()
```

---

*Core collection type enabling dynamic, growable arrays with Rust's ownership guarantees.*