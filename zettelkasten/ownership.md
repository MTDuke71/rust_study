# ownership

*Created: 2025-11-08*
*Tags: #rust-fundamentals #ownership-system #borrowing #lifetimes #move-semantics #memory-management*

## Overview

Ownership is Rust's **core memory management system** that ensures **memory safety** without garbage collection. It enforces rules at compile time about **who owns data**, **when data can be accessed**, and **when data is cleaned up**, preventing memory leaks, use-after-free, and data races.

## The Three Rules of Ownership

### Rule 1: Each value has a single owner

```rust
fn single_ownership_demo() {
    let s1 = String::from("Hello"); // s1 owns the string
    let s2 = s1;                    // Ownership moves to s2
    // println!("{}", s1);          // ERROR: s1 no longer owns the data
    println!("{}", s2);             // OK: s2 is the owner
}

// Stack vs Heap ownership
fn stack_vs_heap() {
    let x = 5;        // Copy types don't move ownership
    let y = x;        // x is copied, both x and y are valid
    println!("x: {}, y: {}", x, y); // Both valid
    
    let s1 = String::from("hello"); // Heap allocated, ownership matters
    let s2 = s1;                     // s1 moved to s2
    // println!("{}", s1);           // ERROR: s1 is no longer valid
}
```

### Rule 2: When owner goes out of scope, value is dropped

```rust
fn scope_and_cleanup() {
    {
        let s = String::from("hello"); // s comes into scope
        println!("{}", s);             // s is valid here
    } // s goes out of scope and is dropped (memory freed)
    
    // println!("{}", s); // ERROR: s is not in scope
}

// RAII - Resource Acquisition Is Initialization
fn raii_demo() {
    let file = std::fs::File::create("temp.txt").unwrap();
    // File automatically closed when `file` goes out of scope
    // No explicit close() needed
} // File::drop() called automatically here
```

### Rule 3: There can only be one owner at a time

```rust
fn ownership_transfer() {
    let s1 = String::from("hello");
    let s2 = take_ownership(s1);    // s1 moved into function
    // println!("{}", s1);          // ERROR: s1 no longer valid
    println!("{}", s2);             // OK: returned ownership to s2
}

fn take_ownership(s: String) -> String {
    println!("Got: {}", s);
    s // Return ownership to caller
}
```

## Borrowing System

### Immutable Borrowing

```rust
fn immutable_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow s1 (don't take ownership)
    println!("'{}' has length {}", s1, len); // s1 still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't own data, so nothing happens

// Multiple immutable borrows allowed
fn multiple_immutable_borrows() {
    let s = String::from("hello");
    let r1 = &s; // OK
    let r2 = &s; // OK - multiple immutable borrows
    let r3 = &s; // OK
    println!("{}, {}, {}", r1, r2, r3);
}
```

### Mutable Borrowing

```rust
fn mutable_borrowing() {
    let mut s = String::from("hello");
    change_string(&mut s);           // Mutable borrow
    println!("{}", s);               // s is modified
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

// Only one mutable borrow at a time
fn single_mutable_borrow() {
    let mut s = String::from("hello");
    let r1 = &mut s;    // OK
    // let r2 = &mut s; // ERROR: cannot borrow s as mutable more than once
    r1.push_str(" world");
    // println!("{}", r2); // This would be an error if r2 existed
}

// Cannot mix mutable and immutable borrows
fn no_mixed_borrows() {
    let mut s = String::from("hello");
    let r1 = &s;     // OK - immutable borrow
    let r2 = &s;     // OK - another immutable borrow
    // let r3 = &mut s; // ERROR: cannot borrow as mutable while immutable borrows exist
    println!("{}, {}", r1, r2);
}
```

## Mission Integration Examples

### Mission 1: Stack Ownership

```rust
pub struct Stack<T> {
    items: Vec<T>, // Stack owns the Vec, Vec owns the elements
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(), // New Vec owned by Stack
        }
    }
    
    // Takes ownership of item
    pub fn push(&mut self, item: T) {
        self.items.push(item); // item moved into Vec
    }
    
    // Transfers ownership back to caller
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop() // Returns owned value or None
    }
    
    // Borrows without taking ownership
    pub fn peek(&self) -> Option<&T> {
        self.items.last() // Returns reference tied to &self lifetime
    }
    
    // Borrows for iteration
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().rev() // Borrows elements for iteration
    }
}

// Usage demonstrates ownership patterns
fn stack_ownership_demo() {
    let mut stack = Stack::new();
    
    // Ownership transfer into stack
    let data = String::from("hello");
    stack.push(data); // data moved into stack
    // println!("{}", data); // ERROR: data no longer owned here
    
    // Borrowing without ownership transfer
    if let Some(top) = stack.peek() {
        println!("Top: {}", top); // Borrows, doesn't move
    }
    
    // Ownership transfer out of stack
    if let Some(popped) = stack.pop() {
        println!("Popped: {}", popped); // Now we own the string again
    }
}
```

### Mission 4: Linked List Ownership Challenges

```rust
// Traditional approach fails due to ownership rules
// struct Node<T> {
//     data: T,
//     next: Option<Box<Node<T>>>, // Each node owns the next
//     prev: ???,                  // Can't have two owners!
// }

// Solution: Smart pointers for shared ownership
use std::rc::Rc;
use std::cell::RefCell;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>, // Weak reference would be better
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
                // Rc allows shared ownership
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                // First node - both head and tail point to it
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }
    
    // Safe traversal respecting ownership
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head.clone(), // Clone Rc, not data
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
            self.current = node.borrow().next.clone();
            node
        })
    }
}
```

### Mission 5: HashMap Key-Value Ownership

```rust
use std::collections::HashMap;

pub struct SafeHashMap<K, V> {
    inner: HashMap<K, V>, // HashMap owns keys and values
}

impl<K, V> SafeHashMap<K, V> 
where 
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    
    // Takes ownership of key and value
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value) // key and value moved into HashMap
    }
    
    // Borrows key, returns borrowed value
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key) // Borrows value with lifetime tied to &self
    }
    
    // Borrows key, returns owned value (if found)
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key) // Returns owned value from HashMap
    }
    
    // Iterate over borrowed key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.inner.iter() // Borrows keys and values
    }
    
    // Consume HashMap and return owned key-value pairs
    pub fn into_iter(self) -> impl Iterator<Item = (K, V)> {
        self.inner.into_iter() // Transfers ownership of all data
    }
}

// Ownership patterns in usage
fn hashmap_ownership_demo() {
    let mut map = SafeHashMap::new();
    
    // Move ownership into map
    let key = String::from("hello");
    let value = String::from("world");
    map.insert(key, value); // key and value moved
    // println!("{} {}", key, value); // ERROR: moved values
    
    // Borrow from map
    let borrowed_key = String::from("hello");
    if let Some(borrowed_value) = map.get(&borrowed_key) {
        println!("Found: {}", borrowed_value); // Borrowed, not moved
    }
    
    // Remove transfers ownership back
    if let Some(owned_value) = map.remove(&borrowed_key) {
        println!("Removed: {}", owned_value); // Now we own it again
    }
}
```

## AoC Ownership Patterns

### Day 15: Ingredient Ownership in Optimization

```rust
#[derive(Clone, Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

// Function takes ownership of ingredients
fn find_optimal_recipe(ingredients: Vec<Ingredient>) -> (i32, Vec<i32>) {
    let mut best_score = 0;
    let mut best_amounts = vec![0; ingredients.len()];
    
    // Borrow ingredients for calculation
    for amounts in generate_combinations(ingredients.len(), 100) {
        let score = calculate_score(&ingredients, &amounts); // Borrow ingredients and amounts
        if score > best_score {
            best_score = score;
            best_amounts = amounts; // Move amounts (Vec<i32> is moved)
        }
    }
    
    (best_score, best_amounts) // Return ownership of best_amounts
}

// Function borrows ingredients, doesn't take ownership
fn calculate_score(ingredients: &[Ingredient], amounts: &[i32]) -> i32 {
    let mut totals = [0; 4]; // capacity, durability, flavor, texture
    
    // Borrow individual ingredients and amounts
    for (ingredient, &amount) in ingredients.iter().zip(amounts.iter()) {
        totals[0] += ingredient.capacity * amount;
        totals[1] += ingredient.durability * amount;
        totals[2] += ingredient.flavor * amount;
        totals[3] += ingredient.texture * amount;
    }
    
    // Calculate final score
    totals.iter().map(|&x| x.max(0)).product()
}
```

### Day 11: String Ownership in Password Generation

```rust
// Takes ownership of current password, returns new owned password
fn increment_password(mut password: String) -> String {
    let bytes = unsafe { password.as_bytes_mut() }; // Mutable borrow of bytes
    
    // Modify in place
    for i in (0..bytes.len()).rev() {
        if bytes[i] == b'z' {
            bytes[i] = b'a';
        } else {
            bytes[i] += 1;
            break;
        }
    }
    
    password // Return ownership of modified string
}

// Alternative: borrow mutably instead of taking ownership
fn increment_password_borrow(password: &mut String) {
    let bytes = unsafe { password.as_bytes_mut() };
    
    for i in (0..bytes.len()).rev() {
        if bytes[i] == b'z' {
            bytes[i] = b'a';
        } else {
            bytes[i] += 1;
            break;
        }
    }
    // No return needed - modified in place
}

// Usage demonstrates different ownership patterns
fn password_ownership_demo() {
    // Pattern 1: Transfer ownership
    let password = String::from("abcdefgh");
    let new_password = increment_password(password); // password moved
    // println!("{}", password); // ERROR: password moved
    println!("New: {}", new_password);
    
    // Pattern 2: Mutable borrowing  
    let mut password2 = String::from("abcdefgh");
    increment_password_borrow(&mut password2); // Borrow mutably
    println!("Modified: {}", password2); // password2 still owned here
}
```

## Lifetimes and Advanced Ownership

### Lifetime Annotations

```rust
// Explicit lifetime annotations for references
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime parameter
struct ImportantExcerpt<'a> {
    part: &'a str, // Reference must live as long as struct
}

impl<'a> ImportantExcerpt<'a> {
    // Lifetime elision rules apply
    fn level(&self) -> i32 {
        3
    }
    
    // Explicit lifetime annotations when needed
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part // Return borrowed data with 'a lifetime
    }
}
```

### Ownership Transfer Patterns

```rust
// Builder pattern with ownership transfer
pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            timeout: None,
        }
    }
    
    // Take ownership of self, return ownership of modified self
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host); // host moved into self
        self // Return ownership of self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    // Consume self and return final config
    pub fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.unwrap_or_else(|| "localhost".to_string()),
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(30),
        })
    }
}

// Usage shows fluid ownership transfer
fn builder_ownership_demo() {
    let config = ConfigBuilder::new()
        .host("example.com".to_string()) // Builder consumed and returned
        .port(3000)                      // Builder consumed and returned
        .timeout(60)                     // Builder consumed and returned
        .build()                         // Builder finally consumed
        .unwrap();
        
    println!("Config: {:?}", config);
}
```

## Common Ownership Patterns

### Option and Result Ownership

```rust
// Option<T> owns T when Some(T)
fn option_ownership_demo() {
    let maybe_string = Some(String::from("hello"));
    
    match maybe_string {
        Some(s) => {
            println!("Got string: {}", s); // s is owned here
            // s is dropped when match arm ends
        }
        None => println!("No string"),
    }
    // maybe_string consumed by match
}

// Result<T, E> owns both success and error values
fn result_ownership_demo() -> Result<String, String> {
    let input = "42";
    
    match input.parse::<i32>() {
        Ok(num) => Ok(format!("Number: {}", num)), // Return owned String
        Err(e) => Err(format!("Parse error: {}", e)), // Return owned String
    }
}
```

### Iterator Ownership Patterns

```rust
// Different iterator methods have different ownership semantics
fn iterator_ownership_demo() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // iter() borrows elements
    for item in vec.iter() {
        println!("Borrowed: {}", item); // item is &i32
    }
    println!("vec still owned: {:?}", vec);
    
    // into_iter() takes ownership of elements
    for item in vec.into_iter() {
        println!("Owned: {}", item); // item is i32
    }
    // println!("vec moved: {:?}", vec); // ERROR: vec moved
    
    // iter_mut() borrows elements mutably
    let mut vec2 = vec![1, 2, 3, 4, 5];
    for item in vec2.iter_mut() {
        *item *= 2; // item is &mut i32
    }
    println!("vec2 modified: {:?}", vec2);
}
```

## Integration with Other Concepts

- **[[Memory Safety]]**: Ownership prevents memory safety violations
- **[[interior-mutability]]**: Relaxing ownership rules with runtime checks
- **[[zero-cost-abstractions]]**: Ownership enables zero-cost memory management
- **[[Performance Patterns]]**: Ownership-guided optimization techniques
- **[[String Manipulation]]**: String ownership and borrowing patterns

## Daily Study Applications

### Week 1: Ownership Fundamentals

- Move semantics and ownership transfer
- Borrowing rules and lifetime basics
- Stack vs heap allocation patterns

### Week 2: Advanced Ownership Patterns

- Smart pointers for complex ownership scenarios
- Reference counting with Rc<T>
- Interior mutability with RefCell<T>

### Week 3: Lifetime Management

- Explicit lifetime annotations
- Lifetime elision rules
- Static lifetimes and global data

## Mission Ownership Requirements

### Ownership Design Checklist

1. **Clear ownership boundaries** for each data structure
2. **Minimal cloning** - prefer borrowing when possible
3. **Appropriate smart pointers** for shared ownership scenarios
4. **RAII compliance** - automatic cleanup on scope exit
5. **Iterator ownership semantics** - iter(), iter_mut(), into_iter()
6. **Builder patterns** for complex construction
7. **Error handling** with owned vs borrowed data

### Testing Ownership Patterns

```rust
#[cfg(test)]
mod ownership_tests {
    use super::*;
    
    #[test]
    fn test_ownership_transfer() {
        let mut stack = Stack::new();
        let data = String::from("test");
        
        stack.push(data); // data moved
        // assert_eq!(data, "test"); // This would be a compile error
        
        let popped = stack.pop().unwrap();
        assert_eq!(popped, "test"); // Now we own it again
    }
    
    #[test]
    fn test_borrowing_patterns() {
        let stack = Stack::new();
        let peek1 = stack.peek(); // Borrows
        let peek2 = stack.peek(); // Multiple borrows OK
        
        assert_eq!(peek1, peek2); // Both references valid
    }
    
    #[test]
    fn test_iterator_ownership() {
        let mut vec = vec![1, 2, 3];
        
        // Borrowed iteration
        let sum1: i32 = vec.iter().sum();
        assert_eq!(sum1, 6);
        assert_eq!(vec.len(), 3); // vec still valid
        
        // Owned iteration  
        let sum2: i32 = vec.into_iter().sum();
        assert_eq!(sum2, 6);
        // vec is now moved and invalid
    }
}
```

## Further Reading

- **[[Memory Safety]]**: How ownership enables memory safety
- **[[interior-mutability]]**: Relaxing ownership with runtime checks
- **[[zero-cost-abstractions]]**: Performance benefits of ownership
- **[[String Manipulation]]**: String ownership patterns

---

*ownership Links:*

- [[Memory Safety]] - Safety through ownership
- [[interior-mutability]] - Flexible ownership patterns
- [[zero-cost-abstractions]] - Performance benefits
- [[Performance Patterns]] - Ownership-guided optimization
- [[String Manipulation]] - String ownership patterns
- [[mission-1]] - Stack ownership examples
- [[mission-4]] - Complex ownership with smart pointers
- [[mission-5]] - HashMap key-value ownership
