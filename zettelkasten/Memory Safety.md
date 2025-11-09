# Memory Safety

*Created: 2025-11-08*
*Tags: #rust-safety #memory-management #ownership #borrowing #unsafe-rust #security*

## Overview

Memory safety in Rust is achieved through the **ownership system**, **borrowing rules**, and **compile-time checking** that prevents common memory errors like **buffer overflows**, **use-after-free**, **double-free**, and **data races**. Rust provides memory safety **without garbage collection** or runtime overhead.

## Core Safety Guarantees

### Memory Safety Violations Prevented
```rust
// Use-after-free - PREVENTED at compile time
fn use_after_free_prevented() {
    let data = vec![1, 2, 3];
    let reference = &data[0];
    drop(data); // Move data, invalidating it
    // println!("{}", reference); // ERROR: borrow of moved value
}

// Double-free - PREVENTED by ownership
fn double_free_prevented() {
    let data = Box::new(42);
    let data2 = data; // Ownership moved
    // drop(data);    // ERROR: use of moved value
    drop(data2);      // OK: data2 owns the value
}

// Buffer overflow - PREVENTED by bounds checking
fn buffer_overflow_prevented() {
    let arr = [1, 2, 3, 4, 5];
    // let value = arr[10]; // PANIC: index out of bounds (runtime check)
    
    // Safe alternative
    match arr.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
}

// Data race - PREVENTED by ownership + borrowing
fn data_race_prevented() {
    let mut data = vec![1, 2, 3];
    let reader = &data;
    // let writer = &mut data; // ERROR: cannot borrow as mutable while immutable borrow exists
    println!("Read: {:?}", reader);
}
```

### Ownership System Fundamentals
```rust
// Single ownership prevents aliasing + mutation
fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved to s2
    // println!("{}", s1); // ERROR: s1 no longer valid
    println!("{}", s2);   // OK: s2 owns the string
}

// Borrowing allows temporary access
fn borrowing_example() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow s1
    println!("Length of '{}' is {}", s1, len); // s1 still valid
}

fn calculate_length(s: &String) -> usize {
    s.len() // Can read through reference
} // s goes out of scope, but doesn't own data, so nothing happens

// Mutable borrowing for controlled mutation
fn mutable_borrowing_example() {
    let mut s = String::from("hello");
    change_string(&mut s); // Mutable borrow
    println!("{}", s);     // s is still valid and modified
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}
```

## Mission Integration Examples

### Mission 1: Stack Memory Safety
```rust
pub struct Stack<T> {
    items: Vec<T>, // Vec provides memory safety automatically
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(), // Safe initialization
        }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item); // Safe: Vec handles reallocation
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop() // Safe: Returns Option to handle empty case
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last() // Safe: Returns reference with lifetime tied to &self
    }
    
    // Safe iteration without exposing internal structure
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().rev() // LIFO order, safe iterator
    }
}

// Usage demonstrates memory safety
fn stack_safety_example() {
    let mut stack = Stack::new();
    stack.push(String::from("hello"));
    
    // Safe peek - doesn't consume the value
    if let Some(top) = stack.peek() {
        println!("Top: {}", top);
    }
    
    // Safe pop - returns owned value
    if let Some(value) = stack.pop() {
        println!("Popped: {}", value);
    }
    
    // Safe when empty
    assert_eq!(stack.pop(), None);
}
```

### Mission 4: Linked List with Safe References
```rust
use std::rc::Rc;
use std::cell::RefCell;

// Safe linked list using smart pointers
type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>,
}

pub struct LinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
    
    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));
        
        match self.tail.take() {
            Some(old_tail) => {
                // Safe: RefCell provides runtime borrow checking
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                // First node
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }
    
    // Safe traversal with iterator
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head.clone(),
        }
    }
}

pub struct LinkedListIterator<T> {
    current: Option<NodeRef<T>>,
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = Rc<RefCell<Node<T>>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            // Safe: RefCell ensures no aliasing violations
            self.current = node.borrow().next.clone();
            node
        })
    }
}
```

### Mission 5: HashMap Memory Safety
```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct SafeHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> SafeHashMap<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 16;
        let mut buckets = Vec::with_capacity(INITIAL_CAPACITY);
        buckets.resize_with(INITIAL_CAPACITY, Vec::new);
        
        Self {
            buckets,
            size: 0,
        }
    }
    
    // Safe key hashing
    fn hash_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }
    
    // Safe insertion with ownership transfer
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.hash_key(&key);
        let bucket = &mut self.buckets[index];
        
        // Safe: Find existing key without invalidating references
        for (existing_key, existing_value) in bucket.iter_mut() {
            if *existing_key == key {
                return Some(std::mem::replace(existing_value, value));
            }
        }
        
        // Key doesn't exist - safe to add
        bucket.push((key, value));
        self.size += 1;
        None
    }
    
    // Safe lookup with borrowing
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash_key(key);
        let bucket = &self.buckets[index];
        
        bucket.iter()
            .find(|(k, _)| *k == *key)
            .map(|(_, v)| v)
    }
    
    // Safe removal with ownership transfer
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash_key(key);
        let bucket = &mut self.buckets[index];
        
        if let Some(pos) = bucket.iter().position(|(k, _)| *k == *key) {
            self.size -= 1;
            Some(bucket.swap_remove(pos).1)
        } else {
            None
        }
    }
}
```

## AoC Memory Safety Patterns

### Day 18: Game of Life Safe Grid
```rust
pub struct GameGrid {
    current: Vec<Vec<bool>>,
    next: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl GameGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            current: vec![vec![false; width]; height],
            next: vec![vec![false; width]; height],
            width,
            height,
        }
    }
    
    // Safe cell access with bounds checking
    pub fn get_cell(&self, row: usize, col: usize) -> bool {
        self.current.get(row)
            .and_then(|r| r.get(col))
            .copied()
            .unwrap_or(false)
    }
    
    // Safe neighbor counting without buffer overflows
    pub fn count_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        
        // Safe iteration over neighbors using saturating arithmetic
        for dr in -1i32..=1i32 {
            for dc in -1i32..=1i32 {
                if dr == 0 && dc == 0 { continue; }
                
                let new_row = (row as i32 + dr).max(0) as usize;
                let new_col = (col as i32 + dc).max(0) as usize;
                
                if self.get_cell(new_row, new_col) {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    // Safe state update with double buffering
    pub fn step(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let neighbors = self.count_neighbors(row, col);
                let alive = self.get_cell(row, col);
                
                self.next[row][col] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        
        // Safe buffer swap
        std::mem::swap(&mut self.current, &mut self.next);
    }
}
```

### Day 15: Safe Combinatorial Search
```rust
// Safe ingredient combination generation
fn safe_ingredient_search(ingredients: &[Ingredient]) -> Option<i32> {
    let total_teaspoons = 100;
    let ingredient_count = ingredients.len();
    
    if ingredient_count == 0 {
        return None; // Safe handling of empty input
    }
    
    // Generate all valid combinations safely
    generate_combinations(ingredient_count, total_teaspoons)
        .into_iter()
        .map(|amounts| calculate_score_safe(ingredients, &amounts))
        .filter_map(|score| score) // Filter out invalid combinations
        .max()
}

fn generate_combinations(num_ingredients: usize, total: i32) -> Vec<Vec<i32>> {
    let mut combinations = Vec::new();
    
    // Safe recursive generation with bounds checking
    fn backtrack(
        current: &mut Vec<i32>,
        remaining: i32,
        ingredients_left: usize,
        combinations: &mut Vec<Vec<i32>>
    ) {
        if ingredients_left == 1 {
            // Last ingredient gets remaining amount
            if remaining >= 0 {
                current.push(remaining);
                combinations.push(current.clone());
                current.pop();
            }
            return;
        }
        
        // Safe iteration with bounds
        for amount in 0..=remaining {
            current.push(amount);
            backtrack(current, remaining - amount, ingredients_left - 1, combinations);
            current.pop();
        }
    }
    
    let mut current = Vec::with_capacity(num_ingredients);
    backtrack(&mut current, total, num_ingredients, &mut combinations);
    combinations
}

fn calculate_score_safe(ingredients: &[Ingredient], amounts: &[i32]) -> Option<i32> {
    if ingredients.len() != amounts.len() {
        return None; // Safety check for mismatched lengths
    }
    
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    
    // Safe iteration with bounds checking
    for (ingredient, &amount) in ingredients.iter().zip(amounts.iter()) {
        capacity += ingredient.capacity * amount;
        durability += ingredient.durability * amount;
        flavor += ingredient.flavor * amount;
        texture += ingredient.texture * amount;
    }
    
    // Safe score calculation with non-negative guarantee
    if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
        Some(0)
    } else {
        Some(capacity * durability * flavor * texture)
    }
}
```

## Unsafe Rust and Safety Boundaries

### When Unsafe is Necessary
```rust
// Safe wrapper around unsafe operations
pub struct SafeBuffer {
    data: Vec<u8>,
    initialized: usize, // Track initialized portion
}

impl SafeBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            initialized: 0,
        }
    }
    
    // Safe interface for unsafe optimization
    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        let current_len = self.data.len();
        let new_len = current_len + slice.len();
        
        // Ensure capacity
        if new_len > self.data.capacity() {
            self.data.reserve(new_len - self.data.capacity());
        }
        
        unsafe {
            // SAFETY: We've ensured capacity above
            let ptr = self.data.as_mut_ptr().add(current_len);
            std::ptr::copy_nonoverlapping(slice.as_ptr(), ptr, slice.len());
            self.data.set_len(new_len);
        }
        
        self.initialized = new_len;
    }
    
    // Safe access to initialized data
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.initialized]
    }
}

// Unsafe operations must be documented and justified
unsafe fn fast_memory_copy(src: *const u8, dst: *mut u8, len: usize) {
    // SAFETY: Caller must ensure:
    // 1. src is valid for reads of len bytes
    // 2. dst is valid for writes of len bytes  
    // 3. src and dst don't overlap
    std::ptr::copy_nonoverlapping(src, dst, len);
}
```

### Memory Safety Validation
```rust
// Compile-time safety validation
fn compile_time_safety_demo() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // These would be caught at compile time:
    // let reference = &data[0];
    // data.clear(); // Would invalidate reference
    // println!("{}", reference); // ERROR: use after invalidation
    
    // Safe pattern: limit scope of borrows
    {
        let reference = &data[0];
        println!("First element: {}", reference);
    } // reference scope ends here
    
    data.clear(); // Safe: no outstanding references
}

// Runtime safety validation
fn runtime_safety_demo() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Safe array access with bounds checking
    match data.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"), // Handled gracefully
    }
    
    // Safe string slicing
    let text = "Hello, 世界!";
    match text.get(0..5) {
        Some(slice) => println!("Slice: {}", slice),
        None => println!("Invalid slice bounds"),
    }
}
```

## Concurrency Safety

### Thread Safety Guarantees
```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Safe shared state with Arc + Mutex
fn safe_concurrent_access() {
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = shared_data.clone();
        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap(); // Safe exclusive access
            data.push(i);
            println!("Thread {} modified data", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final data: {:?}", shared_data.lock().unwrap());
}

// Send and Sync traits ensure thread safety
fn thread_safety_bounds<T: Send + Sync + 'static>(data: T) {
    thread::spawn(move || {
        // T can be safely moved to another thread (Send)
        // References to T can be safely shared between threads (Sync)
        println!("Processing data in thread");
    });
}
```

### Data Race Prevention
```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

// Lock-free counter - no data races possible
struct SafeCounter {
    value: AtomicUsize,
}

impl SafeCounter {
    fn new() -> Self {
        Self {
            value: AtomicUsize::new(0),
        }
    }
    
    fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::Relaxed)
    }
    
    fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }
}

// Usage is automatically thread-safe
fn concurrent_counter_demo() {
    let counter = Arc::new(SafeCounter::new());
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment(); // No data races possible
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final count: {}", counter.get()); // Always 10000
}
```

## Integration with Other Concepts

- **[[ownership]]**: Core memory safety mechanism
- **[[interior-mutability]]**: Safe shared mutability patterns
- **[[zero-cost-abstractions]]**: Safety without performance cost
- **[[Performance Patterns]]**: Safe optimization techniques
- **[[String Manipulation]]**: Safe string handling

## Daily Study Applications

### Week 1: Ownership and Borrowing
- Understanding move semantics and memory safety
- Preventing use-after-free with ownership
- Safe reference management

### Week 3: Advanced Safety Patterns
- Smart pointers for complex ownership scenarios
- Interior mutability with runtime safety checks
- Custom safe abstractions over unsafe code

### Week 4: Concurrency Safety
- Thread safety with Send and Sync traits
- Data race prevention with ownership system
- Safe concurrent data structures

## Mission Safety Requirements

### Memory Safety Validation Checklist
1. **No raw pointers** in safe interfaces
2. **Bounds checking** for all array/vector access
3. **Option/Result** for operations that can fail
4. **Lifetime annotations** for reference validity
5. **Safe defaults** for uninitialized data
6. **Input validation** for public APIs
7. **Resource cleanup** through RAII

### Safety Testing Patterns
```rust
#[cfg(test)]
mod safety_tests {
    use super::*;
    
    #[test]
    fn test_bounds_safety() {
        let stack = Stack::<i32>::new();
        assert_eq!(stack.pop(), None); // Safe when empty
        assert_eq!(stack.peek(), None); // Safe when empty
    }
    
    #[test]
    fn test_memory_cleanup() {
        // RAII ensures cleanup even with early returns
        let _data = vec![1; 1_000_000]; // Large allocation
        if true {
            return; // Early return - memory still cleaned up
        }
    }
    
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic_safety() {
        let arr = [1, 2, 3];
        let _ = arr[10]; // Panic is caught, program doesn't crash
    }
}
```

## Further Reading

- **[[ownership]]**: Fundamental memory safety mechanism
- **[[interior-mutability]]**: Safe shared mutability patterns
- **[[zero-cost-abstractions]]**: Performance without compromising safety
- **[[Performance Patterns]]**: Safe optimization techniques

---

*Memory Safety Links:*
- [[ownership]] - Core safety mechanism
- [[interior-mutability]] - Safe shared mutability
- [[zero-cost-abstractions]] - Safety without cost
- [[Performance Patterns]] - Safe optimization
- [[String Manipulation]] - Safe text processing
- [[mission-1]] - Stack safety examples
- [[mission-4]] - Linked list safety patterns
- [[mission-5]] - HashMap memory safety