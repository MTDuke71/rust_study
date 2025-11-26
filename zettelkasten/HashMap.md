# HashMap - Key-Value Collections in Rust

**Tags:** #rust #collections #data-structures #hash-table #performance
**Created:** 2025-10-22  
**Related:** [[Vec Type]], [[Collections MOC]], [[HashMap Deep Dive]], [[Ownership and Borrowing]], [[zettelkasten/rust_book/rust-book-ch8]]

## Overview

`HashMap<K, V>` is Rust's hash table implementation, providing fast O(1) average-case key-value lookups. It's essential for algorithms requiring fast association and lookup operations.

## Key Concepts

### Creation and Initialization

```rust
use std::collections::HashMap;

// Empty HashMap
let mut map: HashMap<String, i32> = HashMap::new();

// With capacity
let mut map = HashMap::with_capacity(10);

// From iterator
let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();

// Using entry API
let mut scores = HashMap::new();
scores.entry("Blue".to_string()).or_insert(10);
```

### Basic Operations

#### Insertion and Access

```rust
let mut scores = HashMap::new();

// Insert values
scores.insert("Blue".to_string(), 25);
scores.insert("Red".to_string(), 50);

// Access values
let blue_score = scores.get("Blue");        // Returns Option<&V>
let red_score = scores["Red"];              // Direct access (panics if missing)

// Check existence
if scores.contains_key("Blue") {
    println!("Blue team exists");
}
```

#### Updating Values

```rust
let mut map = HashMap::new();

// Overwrite
map.insert("key", 1);
map.insert("key", 2);  // Now contains 2

// Only insert if key doesn't exist
map.entry("key").or_insert(3);  // Still contains 2

// Update based on old value
*map.entry("key").or_insert(0) += 1;  // Now contains 3
```

### Entry API - The Ergonomic Way

The entry API provides efficient ways to work with HashMap entries:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// Count word frequencies
let words = vec!["hello", "world", "hello"];
for word in words {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

// Complex initialization
map.entry("key")
   .and_modify(|e| *e += 1)
   .or_insert(1);
```

### Iteration Patterns

```rust
let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);

// Iterate over key-value pairs
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// Keys only
for key in map.keys() {
    println!("Key: {}", key);
}

// Values only  
for value in map.values() {
    println!("Value: {}", value);
}

// Mutable values
for value in map.values_mut() {
    *value *= 2;
}
```

## Memory and Performance

### Hash Function and Collision Handling

- Uses **SipHash** by default (cryptographically secure)
- **Open addressing** with Robin Hood probing
- Load factor maintained around 0.9 for optimal performance

### Performance Characteristics

```rust
// Average case: O(1)
map.get("key");
map.insert("key", value);
map.remove("key");

// Worst case: O(n) - when all keys hash to same bucket
// Resize operation: O(n) - when load factor exceeded
```

### Memory Layout

```rust
let map: HashMap<String, i32> = HashMap::new();
println!("Capacity: {}", map.capacity());  // Initial capacity
// Grows in powers of 2: 0 -> 3 -> 7 -> 14 -> 28...
```

## Common Patterns

### Grouping and Aggregation

```rust
use std::collections::HashMap;

// Group items by category
let items = vec![("fruit", "apple"), ("fruit", "banana"), ("veggie", "carrot")];
let mut grouped: HashMap<&str, Vec<&str>> = HashMap::new();

for (category, item) in items {
    grouped.entry(category).or_insert_with(Vec::new).push(item);
}
```

### Caching and Memoization

```rust
use std::collections::HashMap;

struct Fibonacci {
    cache: HashMap<u64, u64>,
}

impl Fibonacci {
    fn new() -> Self {
        let mut cache = HashMap::new();
        cache.insert(0, 0);
        cache.insert(1, 1);
        Fibonacci { cache }
    }
    
    fn calculate(&mut self, n: u64) -> u64 {
        if let Some(&result) = self.cache.get(&n) {
            return result;
        }
        
        let result = self.calculate(n - 1) + self.calculate(n - 2);
        self.cache.insert(n, result);
        result
    }
}
```

### Configuration and Lookups

```rust
use std::collections::HashMap;

fn create_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert("host".to_string(), "localhost".to_string());
    config.insert("port".to_string(), "8080".to_string());
    config.insert("debug".to_string(), "true".to_string());
    config
}
```

## Advanced Usage

### Custom Hash Functions

```rust
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::collections::hash_map::DefaultHasher;

// Using a different hasher
type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;
```

### Owned vs Borrowed Keys

```rust
use std::collections::HashMap;

// Owned keys (HashMap owns the strings)
let mut owned: HashMap<String, i32> = HashMap::new();
owned.insert("key".to_string(), 42);

// Borrowed keys (strings must live as long as HashMap)  
let mut borrowed: HashMap<&str, i32> = HashMap::new();
borrowed.insert("key", 42);
```

### Converting Between Collections

```rust
// HashMap to Vec of tuples
let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
let vec: Vec<(&str, i32)> = map.into_iter().collect();

// Vec to HashMap
let data = vec![("x", 10), ("y", 20)];
let map: HashMap<&str, i32> = data.into_iter().collect();
```

## Error Handling and Safety

### Safe Access Patterns

```rust
match map.get("key") {
    Some(value) => println!("Found: {}", value),
    None => println!("Key not found"),
}

// Or with if-let
if let Some(value) = map.get("key") {
    println!("Value: {}", value);
}
```

### Avoiding Panics

```rust
// This panics if key doesn't exist:
// let value = map["missing_key"];

// Safe alternatives:
let value = map.get("missing_key").unwrap_or(&0);
let value = map.get("missing_key").copied().unwrap_or(0);
```

## Integration with Ownership System

### Borrowing Rules

```rust
let mut map = HashMap::new();
map.insert("key", vec![1, 2, 3]);

let value_ref = map.get("key").unwrap();
// map.insert("other", vec![4, 5, 6]);  // Error: can't mutate while borrowed
println!("{:?}", value_ref);  // Borrow ends here
```

### Clone vs Move

```rust
let map1: HashMap<String, Vec<i32>> = HashMap::new();
let map2 = map1.clone();  // Deep clone - expensive
let map3 = map1;          // Move - map1 no longer accessible
```

## Use Cases in Rust Study Projects

### Advent of Code Applications

- **Day 4:** MD5 hash mining and result caching
- **Day 7:** Circuit wire value storage  
- **Day 9:** Distance tables and path optimization
- **Memoization:** Dynamic programming solutions

### Mission Projects

- **Mission 5:** Primary data structure focus
- **API patterns:** Key-value configuration storage
- **Caching:** Performance optimization techniques

### Advanced Examples

- **Frequency counting:** Character and word analysis
- **Graph algorithms:** Adjacency maps and vertex properties
- **Data aggregation:** Grouping and statistical operations

## Performance Tips

1. **Pre-size when possible**

   ```rust
   let mut map = HashMap::with_capacity(expected_size);
   ```

2. **Use entry API for conditional operations**

   ```rust
   // Efficient
   map.entry(key).or_insert(default_value);
   
   // Less efficient  
   if !map.contains_key(&key) {
       map.insert(key, default_value);
   }
   ```

3. **Consider `&str` vs `String` for keys**

   ```rust
   // If you own the strings
   HashMap<String, V>
   
   // If strings live elsewhere  
   HashMap<&str, V>
   ```

## Common Pitfalls

### Key Ownership Issues

```rust
let key = "temporary".to_string();
let mut map = HashMap::new();
map.insert(&key, 42);  // key must live as long as map
drop(key);  // Error: key borrowed by map
```

### Hash Quality

```rust
// Poor hash distribution can degrade performance
#[derive(Hash, Eq, PartialEq)]
struct BadKey(u32);

// All instances hash the same - avoid this!
impl std::hash::Hash for BadKey {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        // DON'T DO THIS: state.write_u32(0);
    }
}
```

## Related Concepts

- [[Vec Type]] - Dynamic arrays, often used as HashMap values
- [[HashMap Deep Dive]] - Advanced implementation details
- [[Collections MOC]] - Overview of all Rust collections  
- [[Ownership and Borrowing]] - Memory management in collections
- [[zettelkasten/rust_book/rust-book-ch8]] - Collections chapter in The Rust Book
- [[daily-study/Day10]] - Learning progression reference

## Quick Reference

```rust
// Creation
HashMap::new(), HashMap::with_capacity()

// Basic ops
map.insert(k, v), map.get(&k), map.remove(&k)

// Entry API
map.entry(k).or_insert(v), .and_modify(|e| ...)

// Iteration  
for (k, v) in &map, .keys(), .values(), .into_iter()

// Utilities
map.len(), map.is_empty(), map.contains_key(&k)
```

---

*Fast key-value lookups with Rust's ownership guarantees and zero-cost abstractions.*
