# Day 10 · HashMap & HashSet Deep Dive

## 🔗 Zettelkasten Connections
- **Implementation**: [[Mission5 Overview]] - Custom HashMap from scratch
- **Tutorial**: [[Mission5_tut Overview]] - Guided learning progression
- **Internals**: [[HashMap Internals]] - How hash tables work under the hood
- **Collections**: [[Collections MOC]] - Broader data structures context
- **Next**: [[daily-study/Day11]] - Set theory applications
- **Previous**: [[daily-study/Day09]] - String handling in Rust

## Core Concepts

### HashMap<K,V> Fundamentals
- **Storage**: Key-value pairs with O(1) average access time
- **Trait Requirements**: `K: Eq + Hash` (keys must be hashable and comparable)
- **Internal Structure**: Hash table with collision resolution
- **Security**: Random hash seeds prevent DoS attacks via hash collision

### HashSet<T> Implementation
- **Zero-Cost Abstraction**: `HashSet<T>` = `HashMap<T, ()>` internally
- **Same Requirements**: `T: Eq + Hash`
- **Performance**: Identical to HashMap operations

## Essential Trait Bounds

### Why `Eq + Hash`?
```rust
// HashMap requires both traits working together
impl<K, V> HashMap<K, V> 
where 
    K: Eq + Hash,  // Essential combination
{
    // Hash: Convert key to bucket location
    // Eq: Resolve collisions by comparing keys
}
```

### Types That Work as Keys
```rust
// ✅ Primitive types (auto-implement Eq + Hash)
HashMap<i32, String>
HashMap<char, usize>

// ✅ String types
HashMap<String, i32>
HashMap<&str, i32>

// ✅ Tuples (if all elements are Eq + Hash)
HashMap<(i32, String), bool>

// ❌ Floating point (no Eq due to NaN)
// HashMap<f64, String>  // Won't compile
```

## Advanced Patterns

### Entry API - The Rust Way
```rust
use std::collections::HashMap;

let mut word_count = HashMap::new();
let words = ["hello", "world", "hello", "rust"];

// Pattern 1: or_insert for counting
for word in words {
    *word_count.entry(word).or_insert(0) += 1;
}

// Pattern 2: or_insert_with for expensive defaults
let mut cache = HashMap::new();
cache.entry("key").or_insert_with(|| expensive_computation());

// Pattern 3: and_modify for conditional updates
scores.entry("player")
    .and_modify(|score| *score += 10)
    .or_insert(10);
```

### Wrapper Pattern - Custom Collections
```rust
// Professional pattern: Wrapper struct with specific methods
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Counter<K> 
where 
    K: Eq + Hash,
{
    counts: HashMap<K, usize>,
    total: usize,
}

// Implementation blocks with different trait bounds
impl<K> Counter<K>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self { 
            counts: HashMap::new(), 
            total: 0 
        }
    }
    
    pub fn increment(&mut self, key: K) {
        *self.counts.entry(key).or_insert(0) += 1;
        self.total += 1;
    }
    
    pub fn get(&self, key: &K) -> usize {
        self.counts.get(key).copied().unwrap_or(0)
    }
}

// Advanced methods requiring Clone
impl<K> Counter<K>
where
    K: Eq + Hash + Clone,  // More restrictive bounds
{
    pub fn most_common(&self, n: usize) -> Vec<(K, usize)> {
        let mut items: Vec<_> = self.counts.iter()
            .map(|(k, &v)| (k.clone(), v))  // Clone needed here
            .collect();
        
        items.sort_by(|a, b| b.1.cmp(&a.1));
        items.into_iter().take(n).collect()
    }
}
```

## Implementation Design Patterns

### Progressive Enhancement Pattern
**Philosophy**: Multiple impl blocks with increasing constraints
```rust
// Block 1: Basic operations (minimal constraints)
impl<K: Eq + Hash> Counter<K> {
    fn new() -> Self { ... }
    fn increment(&mut self, key: K) { ... }
}

// Block 2: Advanced operations (additional constraints)
impl<K: Eq + Hash + Clone> Counter<K> {
    fn most_common(&self, n: usize) -> Vec<(K, usize)> { ... }
}
```

**Benefits**:
- Maximum compatibility (basic ops work with any hashable type)
- Feature enhancement for types that support it
- Clear separation of concerns

### Alternative: Constraint Minimization
```rust
// Group methods by actual requirements, not logical grouping
impl<K: Eq + Hash> Counter<K> {
    fn increment(&mut self, key: K) { ... }
    fn count_multiple<I>(&mut self, items: I) { ... }  // Doesn't need Clone
}

impl<K: Eq + Hash + Clone> Counter<K> {
    fn most_common(&self, n: usize) -> Vec<(K, usize)> { ... }  // Actually needs Clone
}
```

## Types That Are NOT Clone

Understanding why some methods require `Clone`:

### Non-Cloneable Types
```rust
// System resources
use std::fs::File;
use std::net::TcpStream;

let file = File::open("data.txt")?;     // Unique file handle
let stream = TcpStream::connect("addr")?;  // Unique connection

// Synchronization primitives
use std::sync::MutexGuard;
let guard = mutex.lock().unwrap();      // Unique lock ownership

// Custom types without Clone
struct DatabaseConnection { id: u64 }   // Intentionally unique
struct LargeBuffer([u8; 1_000_000]);    // Too expensive to clone
```

### Impact on API Design
```rust
// This works - first impl block
let mut counter: Counter<DatabaseConnection> = Counter::new();
counter.increment(connection);  // ✅ Only needs Eq + Hash

// This fails - second impl block  
counter.most_common(5);  // ❌ DatabaseConnection doesn't implement Clone
```

## Real-World Usage Patterns

### AoC Common Patterns
```rust
// Pattern 1: Coordinate tracking
let mut visited = HashSet::new();
visited.insert((x, y));
if visited.contains(&(next_x, next_y)) { /* cycle detected */ }

// Pattern 2: State transitions
let mut states = HashMap::new();
states.insert(current_state, next_state);

// Pattern 3: Frequency analysis
let mut char_count = HashMap::new();
for ch in input.chars() {
    *char_count.entry(ch).or_insert(0) += 1;
}
```

### Performance Considerations
- **Hash Quality**: Good hash function prevents clustering
- **Load Factor**: Automatic resizing maintains performance
- **Memory**: ~25% overhead vs arrays for small collections
- **Cache Locality**: Random access pattern (vs Vec's sequential)

## Key Takeaways

1. **HashMap/HashSet are ubiquitous** in competitive programming and real applications
2. **Trait bounds matter** - understand what `Eq + Hash` means and why
3. **Entry API is idiomatic** - prefer it over manual `contains` + `insert` patterns
4. **Progressive enhancement** - design APIs with minimal constraints, add features incrementally  
5. **Clone requirement** - understand when and why methods need cloneable types
6. **Zero-cost abstractions** - HashSet is just HashMap with different interface

## Implementation Insights

The power of Rust's type system shines in collection design:
- **Compile-time safety**: Invalid key types caught at compile time
- **Zero-cost abstractions**: HashSet adds no runtime overhead to HashMap
- **Progressive APIs**: Basic operations work broadly, advanced features when supported
- **Memory safety**: No null pointer derefs or buffer overflows in hash table operations

Maps and sets are fundamental to almost every non-trivial program - mastering their patterns and constraints is essential for effective Rust development.




