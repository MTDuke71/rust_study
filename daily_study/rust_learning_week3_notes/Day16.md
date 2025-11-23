# Day 16 · Generic Types (type parameters, constraints)

> **Learning Context**: Day 16 expands on Day 15's traits with generic type parameters, enabling Mission5's `HashMap<K, V>` implementation and flexible, reusable data structures.

**Cross-Track Integration:**
- **Mission Focus**: Generic types enable Mission5's `HashMap<K, V>` where K and V can be any type - see [[mission-5]]
- **Daily Study**: Builds on Day 15 traits to create parameterized types
- **Rust Book**: Core of Chapter 10 Generic Types, Traits, and Lifetimes

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Generic patterns across all data structures
- [[mission-5]] - REQ-1 generic support implementation
- [[zettel-index]] - Main learning hub

## Core Concepts

### What Are Generics?
- **Type Parameters**: Placeholder types that are filled in at compile time
- **Zero-Cost Abstractions**: No runtime overhead - monomorphization creates specific versions
- **Code Reuse**: Write once, work with many types
- **Type Safety**: Compile-time guarantees about type compatibility

### Basic Generic Syntax
```rust
// Generic struct with single type parameter
struct Box<T> {
    value: T,
}

// Generic struct with multiple type parameters
struct Pair<T, U> {
    first: T,
    second: U,
}

// Generic enum (like Option<T> and Result<T, E>)
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}
```

## Generic Functions

### Function Type Parameters
```rust
// Generic function - works with any type T
fn identity<T>(x: T) -> T {
    x
}

// Multiple type parameters
fn make_pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

// Usage - Rust infers types
let num = identity(42);           // T inferred as i32
let text = identity("hello");     // T inferred as &str
let pair = make_pair(1, "one");   // T=i32, U=&str
```

### Generic Functions with Trait Bounds
```rust
// Constrain T to types that implement specific traits
fn print_and_clone<T>(x: &T) -> T
where 
    T: std::fmt::Debug + Clone,
{
    println!("Value: {:?}", x);
    x.clone()
}

// Multiple constraints
fn compare_max<T>(a: T, b: T) -> T
where 
    T: PartialOrd + Copy,
{
    if a > b { a } else { b }
}

let max_num = compare_max(10, 5);
let max_char = compare_max('z', 'a');
```

## Generic Structs and Implementations

### Basic Generic Struct
```rust
#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    // Generic constructor
    fn new() -> Self {
        Container { items: Vec::new() }
    }
    
    // Methods that work with any T
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}

// Different containers for different types
let mut int_container: Container<i32> = Container::new();
int_container.add(42);

let mut string_container: Container<String> = Container::new();
string_container.add("hello".to_string());
```

### Conditional Implementations
```rust
impl<T> Container<T>
where 
    T: std::fmt::Display,
{
    // Only available if T implements Display
    fn print_all(&self) {
        for item in &self.items {
            println!("{}", item);
        }
    }
}

impl<T> Container<T>
where 
    T: PartialEq,
{
    // Only available if T implements PartialEq
    fn contains(&self, item: &T) -> bool {
        self.items.contains(item)
    }
}
```

## Mission5 Integration: Generic HashMap Implementation

### HashMap Generic Structure
```rust
use std::hash::{Hash, Hasher};

// Mission5: Generic HashMap structure
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,  // Chaining for collision resolution
    size: usize,
    capacity: usize,
}

impl<K, V> HashMap<K, V> 
where 
    K: Hash + Eq,  // Key constraints for hashing and comparison
{
    pub fn new() -> Self {
        Self::with_capacity(16) // Default capacity
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        HashMap {
            buckets: vec![Vec::new(); capacity],
            size: 0,
            capacity,
        }
    }
    
    fn hash_key(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.hash_key(&key);
        
        // Check if key already exists
        for (existing_key, existing_value) in &mut self.buckets[index] {
            if *existing_key == key {
                return Some(std::mem::replace(existing_value, value));
            }
        }
        
        // New key-value pair
        self.buckets[index].push((key, value));
        self.size += 1;
        None
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash_key(key);
        
        for (existing_key, existing_value) in &self.buckets[index] {
            if existing_key == key {
                return Some(existing_value);
            }
        }
        None
    }
}

// Usage with different key-value type combinations
let mut string_map: HashMap<String, i32> = HashMap::new();
string_map.insert("count".to_string(), 42);

let mut id_map: HashMap<u64, String> = HashMap::new();
id_map.insert(12345, "User Name".to_string());
```

### Generic Iterator Implementation
```rust
// Mission5: Generic iterator for HashMap
pub struct HashMapIter<'a, K, V> {
    buckets: &'a [Vec<(K, V)>],
    bucket_index: usize,
    item_index: usize,
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);
    
    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_index < self.buckets.len() {
            let bucket = &self.buckets[self.bucket_index];
            
            if self.item_index < bucket.len() {
                let item = &bucket[self.item_index];
                self.item_index += 1;
                return Some((&item.0, &item.1));
            }
            
            // Move to next bucket
            self.bucket_index += 1;
            self.item_index = 0;
        }
        None
    }
}

impl<K, V> HashMap<K, V> 
where 
    K: Hash + Eq,
{
    pub fn iter(&self) -> HashMapIter<K, V> {
        HashMapIter {
            buckets: &self.buckets,
            bucket_index: 0,
            item_index: 0,
        }
    }
}
```

## Advanced Generic Patterns

### Associated Types vs Generic Parameters
```rust
// Generic trait with type parameter
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// Better: Associated type (only one implementation per type)
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Implementation specifies the associated type
impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Implementation here
    }
}
```

### Generic Enums
```rust
// Option<T> - standard library example
enum Option<T> {
    Some(T),
    None,
}

// Result<T, E> - error handling
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Custom generic enum
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    
    fn map_left<F, NewL>(self, f: F) -> Either<NewL, R>
    where 
        F: FnOnce(L) -> NewL,
    {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }
}
```

### Phantom Types
```rust
use std::marker::PhantomData;

// Generic struct that doesn't actually store T
struct Id<T> {
    value: u64,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(value: u64) -> Self {
        Id {
            value,
            _phantom: PhantomData,
        }
    }
    
    fn get(&self) -> u64 {
        self.value
    }
}

// Type-safe IDs for different entities
type UserId = Id<User>;
type ProductId = Id<Product>;

struct User { name: String }
struct Product { name: String }

let user_id = UserId::new(123);
let product_id = ProductId::new(456);

// Compiler prevents mixing up ID types
// let invalid = user_id == product_id; // ❌ Compile error
```

## Generic Constraints and Where Clauses

### Complex Constraint Patterns
```rust
// Multiple bounds on same type
fn process<T>(data: T) -> String
where 
    T: std::fmt::Debug + Clone + Send + Sync,
{
    format!("{:?}", data)
}

// Bounds on multiple types
fn transform<T, U, F>(items: Vec<T>, func: F) -> Vec<U>
where 
    T: Clone,
    U: std::fmt::Debug,
    F: Fn(T) -> U,
{
    items.into_iter().map(func).collect()
}

// Lifetime bounds
fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T
where 
    T: PartialOrd,
{
    if x > y { x } else { y }
}
```

### Conditional Trait Implementations
```rust
// Implement Clone only if T implements Clone
impl<T: Clone> Clone for Container<T> {
    fn clone(&self) -> Self {
        Container {
            items: self.items.clone(),
        }
    }
}

// Implement Debug only if T implements Debug
impl<T: std::fmt::Debug> std::fmt::Debug for Container<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("items", &self.items)
            .finish()
    }
}
```

## Performance Considerations

### Monomorphization
```rust
// Generic function
fn process<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}

// Compiler generates specialized versions:
// fn process_i32(x: i32) { println!("{:?}", x); }
// fn process_string(x: String) { println!("{:?}", x); }

// No runtime cost - each type gets its own optimized version
process(42);               // Calls process_i32
process("hello".to_string()); // Calls process_string
```

### Generic vs Trait Objects
```rust
// Generic: compile-time polymorphism (faster, larger binary)
fn generic_process<T: std::fmt::Debug>(items: Vec<T>) {
    for item in items {
        println!("{:?}", item);
    }
}

// Trait object: runtime polymorphism (smaller binary, dynamic dispatch)
fn dynamic_process(items: Vec<Box<dyn std::fmt::Debug>>) {
    for item in items {
        println!("{:?}", item);
    }
}
```

## Real-World Applications

### Generic Data Structures
```rust
// Stack implementation
#[derive(Debug)]
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
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// Works with any type
let mut int_stack: Stack<i32> = Stack::new();
int_stack.push(1);
int_stack.push(2);

let mut string_stack: Stack<String> = Stack::new();
string_stack.push("hello".to_string());
```

### Generic Algorithms
```rust
// Generic sorting function
fn bubble_sort<T>(arr: &mut [T])
where 
    T: PartialOrd + Copy,
{
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

let mut numbers = [64, 34, 25, 12, 22, 11, 90];
bubble_sort(&mut numbers);
println!("Sorted: {:?}", numbers);

let mut chars = ['d', 'b', 'a', 'c'];
bubble_sort(&mut chars);
println!("Sorted: {:?}", chars);
```

## Best Practices

### Generic Design Guidelines
```rust
// ✅ Good: Specific, minimal constraints
fn find_max<T>(items: &[T]) -> Option<&T>
where 
    T: PartialOrd,
{
    items.iter().max()
}

// ❌ Avoid: Too many unnecessary constraints
fn bad_find_max<T>(items: &[T]) -> Option<&T>
where 
    T: PartialOrd + Clone + Debug + Send + Sync, // Too much!
{
    items.iter().max()
}

// ✅ Good: Clear generic parameter names
struct Cache<Key, Value> {
    map: std::collections::HashMap<Key, Value>,
}

// ✅ Good: Single letter for simple cases
struct Point<T> {
    x: T,
    y: T,
}
```

## Learning Progression Summary

From Day 16, you should understand:
1. **Generic Syntax**: Type parameters in structs, enums, and functions
2. **Trait Bounds**: Constraining generic types to specific capabilities
3. **Mission5 Integration**: How `HashMap<K, V>` uses generics for flexibility
4. **Monomorphization**: Compile-time specialization for zero-cost abstractions
5. **Advanced Patterns**: Associated types, phantom types, conditional implementations
6. **Performance**: Generic vs trait object trade-offs

**3-Track Learning Integration:**
- **Mission5**: Generic `HashMap<K, V>` implementation with trait bounds
- **Week 3 Progress**: Day 15 (Traits) → Day 16 (Generics) → Day 17 (Lifetimes)
- **Collections Foundation**: Understanding how Vec<T>, HashMap<K,V>, BTreeMap<K,V> achieve type safety

**Cross-References:**
- [[Collections MOC]] - Generic patterns across Vec, HashMap, BTreeMap implementations
- [[mission-5]] - REQ-1 generic support using type parameters and constraints
- [[HashMap Internals]] - Generic bucket storage and type-safe key handling

**Next**: Day 17 will cover **Lifetime Annotations** - ensuring memory safety in generic code!

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[mission-5]] | [[HashMap Internals]] | [[zettel-index]]*

*Tags: #generics #type-parameters #constraints #mission5 #daily-study #week3 #type-system #zero-cost-abstractions*

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
use std::fmt;
use std::collections::HashMap;

// 1. Basic generic struct
#[derive(Debug, Clone)]
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container { items: Vec::new() }
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}

// 2. Conditional implementations based on trait bounds
impl<T> Container<T>
where 
    T: fmt::Display,
{
    fn print_all(&self) {
        println!("Container contents:");
        for (i, item) in self.items.iter().enumerate() {
            println!("  [{}]: {}", i, item);
        }
    }
}

impl<T> Container<T>
where 
    T: PartialEq,
{
    fn contains(&self, item: &T) -> bool {
        self.items.contains(item)
    }
    
    fn remove_first(&mut self, item: &T) -> Option<T> {
        if let Some(pos) = self.items.iter().position(|x| x == item) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }
}

// 3. Generic functions with multiple type parameters
fn pair_up<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn find_max<T>(items: &[T]) -> Option<&T>
where 
    T: PartialOrd + Ord,
{
    items.iter().max()
}

// 4. Mission5-style generic HashMap (simplified)
#[derive(Debug)]
struct SimpleHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> SimpleHashMap<K, V>
where 
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn new() -> Self {
        SimpleHashMap {
            buckets: vec![Vec::new(); 8], // Simple fixed size
            size: 0,
        }
    }
    
    fn hash_key(&self, key: &K) -> usize {
        // Simplified hash function
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }
    
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.hash_key(&key);
        
        // Check if key exists
        for (existing_key, existing_value) in &mut self.buckets[index] {
            if *existing_key == key {
                return Some(std::mem::replace(existing_value, value));
            }
        }
        
        // New entry
        self.buckets[index].push((key, value));
        self.size += 1;
        None
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash_key(key);
        
        for (existing_key, existing_value) in &self.buckets[index] {
            if existing_key == key {
                return Some(existing_value);
            }
        }
        None
    }
    
    fn len(&self) -> usize {
        self.size
    }
}

// 5. Generic algorithms
fn bubble_sort<T>(arr: &mut [T])
where 
    T: PartialOrd,
{
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 6. Generic enum (like Result<T, E>)
#[derive(Debug)]
enum ProcessResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> ProcessResult<T, E> {
    fn is_success(&self) -> bool {
        matches!(self, ProcessResult::Success(_))
    }
    
    fn map<U, F>(self, f: F) -> ProcessResult<U, E>
    where 
        F: FnOnce(T) -> U,
    {
        match self {
            ProcessResult::Success(value) => ProcessResult::Success(f(value)),
            ProcessResult::Failure(err) => ProcessResult::Failure(err),
        }
    }
}

fn main() {
    println!("=== Generic Types Demo from Day 16 ===\n");
    
    // 1. Generic containers with different types
    println!("1. Generic Containers:");
    let mut int_container: Container<i32> = Container::new();
    int_container.add(10);
    int_container.add(20);
    int_container.add(30);
    
    let mut string_container: Container<String> = Container::new();
    string_container.add("Hello".to_string());
    string_container.add("World".to_string());
    
    println!("   Integer container length: {}", int_container.len());
    println!("   String container length: {}", string_container.len());
    
    // 2. Conditional methods based on trait bounds
    println!("\n2. Conditional Implementations:");
    int_container.print_all(); // Available because i32 implements Display
    
    println!("   Contains 20? {}", int_container.contains(&20)); // PartialEq bound
    int_container.remove_first(&20);
    println!("   After removing 20, length: {}", int_container.len());
    
    // 3. Generic functions
    println!("\n3. Generic Functions:");
    let pair1 = pair_up(42, "answer");
    let pair2 = pair_up(3.14, true);
    println!("   Pair 1: {:?}", pair1);
    println!("   Pair 2: {:?}", pair2);
    
    // Find max with different types
    let numbers = [3, 7, 2, 9, 1];
    let chars = ['d', 'a', 'z', 'b'];
    
    println!("   Max number: {:?}", find_max(&numbers));
    println!("   Max char: {:?}", find_max(&chars));
    
    // 4. Mission5-style generic HashMap
    println!("\n4. Generic HashMap (Mission5 Style):");
    let mut map: SimpleHashMap<String, i32> = SimpleHashMap::new();
    
    map.insert("score".to_string(), 100);
    map.insert("lives".to_string(), 3);
    map.insert("level".to_string(), 5);
    
    println!("   Map size: {}", map.len());
    println!("   Score: {:?}", map.get(&"score".to_string()));
    println!("   Lives: {:?}", map.get(&"lives".to_string()));
    
    // Different key-value types
    let mut id_map: SimpleHashMap<u32, String> = SimpleHashMap::new();
    id_map.insert(1001, "Alice".to_string());
    id_map.insert(1002, "Bob".to_string());
    
    println!("   ID 1001: {:?}", id_map.get(&1001));
    
    // 5. Generic algorithms
    println!("\n5. Generic Algorithms:");
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    let mut words = ["rust", "is", "awesome", "and", "fast"];
    
    println!("   Before sorting numbers: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("   After sorting numbers: {:?}", numbers);
    
    println!("   Before sorting words: {:?}", words);
    bubble_sort(&mut words);
    println!("   After sorting words: {:?}", words);
    
    // 6. Generic enums
    println!("\n6. Generic Enums:");
    let success: ProcessResult<i32, String> = ProcessResult::Success(42);
    let failure: ProcessResult<i32, String> = ProcessResult::Failure("Error occurred".to_string());
    
    println!("   Success result: {:?}", success);
    println!("   Is success? {}", success.is_success());
    
    let doubled: ProcessResult<i32, String> = ProcessResult::Success(42).map(|x| x * 2);
    println!("   Doubled result: {:?}", doubled);
    
    println!("\n=== Generics enable type-safe, reusable code! ===");
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day16_demo.rs` and run `rustc day16_demo.rs && ./day16_demo`
3. **In this workspace**: `.\scripts\run_md.bat daily_study\rust_learning_week3_notes\Day16.md`
4. **As Cargo example**: `cargo run --example day16_generics_demo` (if you add it to Mission5_tut)
