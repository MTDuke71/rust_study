# Borrow Checker Patterns and Troubleshooting

*Common patterns, conflict resolution strategies, and practical solutions for borrow checker challenges*

---

## 🎯 **Common Borrow Checker Patterns**

### **Pattern 1: The "Read-Then-Modify" Pattern**

**Problem**: Need to read a value to decide how to modify it

```rust
// ❌ This doesn't work - borrowing issues
let mut map = HashMap::new();
map.insert("key", 0);

if let Some(value) = map.get("key") {    // Immutable borrow starts
    map.insert("key", value + 1);        // ❌ Mutable borrow conflicts
}
```

**Solutions**:

```rust
// Solution 1: Entry API (Recommended)
let mut map = HashMap::new();
map.insert("key", 0);

*map.entry("key").or_insert(0) += 1;     // ✅ Single operation, no conflicts

// Solution 2: Copy the value
let mut map = HashMap::new();
map.insert("key", 0);

let current = map.get("key").copied().unwrap_or(0);  // Copy, don't borrow
map.insert("key", current + 1);          // ✅ No active borrows

// Solution 3: Clone for complex types
let mut map: HashMap<String, Vec<i32>> = HashMap::new();
map.insert("key".to_string(), vec![1, 2, 3]);

if let Some(value) = map.get("key").cloned() {  // Clone the Vec
    let mut new_value = value;
    new_value.push(4);
    map.insert("key".to_string(), new_value);    // ✅ No borrow conflicts
}
```

### **Pattern 2: The "Iterator Invalidation" Pattern**

**Problem**: Modifying a collection while iterating over it

```rust
// ❌ Classic iterator invalidation
let mut vec = vec![1, 2, 3, 4, 5];
for (i, &value) in vec.iter().enumerate() {      // Immutable borrow
    if value % 2 == 0 {
        vec.remove(i);                           // ❌ Mutable borrow conflicts
    }
}
```

**Solutions**:

```rust
// Solution 1: Collect indices first
let mut vec = vec![1, 2, 3, 4, 5];
let to_remove: Vec<usize> = vec.iter()
    .enumerate()
    .filter_map(|(i, &value)| if value % 2 == 0 { Some(i) } else { None })
    .collect();

// Remove in reverse order to maintain indices
for &i in to_remove.iter().rev() {
    vec.remove(i);
}

// Solution 2: Retain method (Recommended)
let mut vec = vec![1, 2, 3, 4, 5];
vec.retain(|&x| x % 2 != 0);             // ✅ Built-in safe method

// Solution 3: Drain filter (nightly)
// vec.drain_filter(|&mut x| x % 2 == 0);

// Solution 4: Create new collection
let vec = vec![1, 2, 3, 4, 5];
let filtered: Vec<i32> = vec.into_iter()
    .filter(|&x| x % 2 != 0)
    .collect();
```

### **Pattern 3: The "Multiple Mutable Access" Pattern**

**Problem**: Need to mutate different parts of the same structure

```rust
// ❌ Can't have two mutable borrows
let mut data = vec![1, 2, 3, 4, 5];
let first = &mut data[0];                // First mutable borrow
let last = &mut data[4];                 // ❌ Second mutable borrow
*first = 10;
*last = 50;
```

**Solutions**:

```rust
// Solution 1: Split borrows (for slices/arrays)
let mut data = vec![1, 2, 3, 4, 5];
let (left, right) = data.split_at_mut(4);
let first = &mut left[0];                // Mutable borrow of left part
let last = &mut right[0];                // Mutable borrow of right part ✅
*first = 10;
*last = 50;

// Solution 2: Sequential access
let mut data = vec![1, 2, 3, 4, 5];
data[0] = 10;                            // Direct assignment
data[4] = 50;                            // No borrows needed ✅

// Solution 3: Index-based access
let mut data = vec![1, 2, 3, 4, 5];
let first_idx = 0;
let last_idx = 4;
data[first_idx] = 10;                    // ✅ No borrows, just indexing
data[last_idx] = 50;

// Solution 4: get_many_mut (unstable)
// let [first, last] = data.get_many_mut([0, 4]).unwrap();
```

### **Pattern 4: The "Self-Referential Struct" Pattern**

**Problem**: Struct that wants to hold references to its own data

```rust
// ❌ This won't compile - self-referential structs are hard
struct SelfRef {
    data: String,
    slice: &str,                         // ❌ Missing lifetime, points where?
}
```

**Solutions**:

```rust
// Solution 1: Lifetime parameters (requires external guarantees)
struct SelfRef<'a> {
    data: String,
    slice: &'a str,                      // Points to external data
}

// Usage requires careful lifetime management:
fn use_self_ref() -> SelfRef<'static> {
    SelfRef {
        data: String::from("hello world"),
        slice: "hello",                  // Must be 'static
    }
}

// Solution 2: Indices instead of references
struct IndexRef {
    data: String,
    slice_start: usize,
    slice_end: usize,
}

impl IndexRef {
    fn slice(&self) -> &str {
        &self.data[self.slice_start..self.slice_end]
    }
}

// Solution 3: Smart pointers (Rc + RefCell)
use std::rc::Rc;
use std::cell::RefCell;

struct SmartRef {
    data: Rc<RefCell<String>>,
    slice_start: usize,
    slice_end: usize,
}

// Solution 4: Pin and unsafe (advanced)
use std::pin::Pin;

struct PinnedRef {
    data: String,
    slice: *const str,                   // Raw pointer (unsafe)
}
```

---

## 🚨 **Troubleshooting Common Borrow Conflicts**

### **Conflict Type 1: Temporary Borrows in Function Calls**

```rust
// ❌ Temporary borrow extends too long
let mut stack = Stack::new();
stack.push(1);

if let Some(top) = stack.peek() {        // Borrow starts here
    if *top > 0 {
        stack.push(2);                   // ❌ Can't mutate while borrowed
    }
}
```

**Diagnostic Approach**:
1. **Identify the borrow**: `stack.peek()` returns `&T`
2. **Find the scope**: Borrow lasts until end of `if let` block
3. **Locate the conflict**: `push()` needs `&mut self`

**Solutions**:

```rust
// Solution 1: Copy the value
let mut stack = Stack::new();
stack.push(1);

let top_value = stack.peek().copied();   // Copy instead of borrowing
if let Some(value) = top_value {
    if value > 0 {
        stack.push(2);                   // ✅ No active borrow
    }
}

// Solution 2: Limit borrow scope
let mut stack = Stack::new();
stack.push(1);

let should_push = {
    stack.peek().map_or(false, |&x| x > 0)  // Borrow ends here
};

if should_push {
    stack.push(2);                       // ✅ Borrow already ended
}

// Solution 3: Restructure logic
let mut stack = Stack::new();
stack.push(1);

match stack.peek().copied() {
    Some(value) if value > 0 => {
        stack.push(2);                   // ✅ peek() borrow ended
    }
    _ => {}
}
```

### **Conflict Type 2: Closure Borrow Issues**

```rust
// ❌ Closure captures by reference, preventing mutation
let mut data = vec![1, 2, 3];
let closure = || data.len();             // Borrows data
data.push(4);                            // ❌ Can't mutate while borrowed by closure
println!("{}", closure());
```

**Solutions**:

```rust
// Solution 1: Move capture
let mut data = vec![1, 2, 3];
let closure = move || data.len();        // Takes ownership
// data.push(4);                         // ❌ data moved into closure
println!("{}", closure());

// Solution 2: Limit closure lifetime
let mut data = vec![1, 2, 3];
{
    let closure = || data.len();
    println!("{}", closure());           // Use closure immediately
}   // Closure dropped, borrow ends
data.push(4);                           // ✅ Now can mutate

// Solution 3: Pass by value
let mut data = vec![1, 2, 3];
let len = data.len();                   // Copy the value
let closure = move || len;              // Closure owns the copy
data.push(4);                           // ✅ No borrow of data
println!("{}", closure());
```

### **Conflict Type 3: Method Chaining Borrow Issues**

```rust
impl<T> Stack<T> {
    // ❌ This method signature causes borrowing issues
    fn peek_and_modify(&mut self) -> Option<&mut T> {
        self.items.last_mut()
    }
}

// Usage problems:
let mut stack = Stack::new();
stack.push(42);
if let Some(top) = stack.peek_and_modify() {  // Mutable borrow starts
    if *top > 0 {
        stack.push(100);                 // ❌ Can't borrow again
    }
}
```

**Solutions**:

```rust
// Solution 1: Split the operation
impl<T> Stack<T> {
    fn peek_value(&self) -> Option<T> 
    where T: Clone 
    {
        self.items.last().cloned()       // Return owned value
    }
    
    fn modify_top<F>(&mut self, f: F) -> bool
    where F: FnOnce(&mut T)
    {
        match self.items.last_mut() {
            Some(top) => { f(top); true }
            None => false,
        }
    }
}

// Usage:
let mut stack = Stack::new();
stack.push(42);

if let Some(value) = stack.peek_value() {
    if value > 0 {
        stack.push(100);                 // ✅ No active borrows
    }
}

// Solution 2: Functional approach
impl<T> Stack<T> {
    fn with_top<R, F>(&mut self, f: F) -> Option<R>
    where F: FnOnce(&T) -> R
    {
        self.items.last().map(f)
    }
}

// Usage:
let should_push = stack.with_top(|&value| value > 0).unwrap_or(false);
if should_push {
    stack.push(100);                     // ✅ Borrow ended
}
```

---

## 🔧 **Advanced Borrow Checker Workarounds**

### **Technique 1: Interior Mutability (RefCell)**

When you need to mutate through shared references:

```rust
use std::cell::RefCell;

struct Container {
    data: RefCell<Vec<i32>>,             // Interior mutability
}

impl Container {
    fn add(&self, value: i32) {          // &self, not &mut self
        self.data.borrow_mut().push(value);  // Runtime borrow checking
    }
    
    fn get(&self, index: usize) -> Option<i32> {
        self.data.borrow().get(index).copied()
    }
}

// Usage allows mutation through shared reference:
let container = Container { data: RefCell::new(vec![1, 2, 3]) };
let shared_ref = &container;             // Shared reference
shared_ref.add(4);                      // ✅ Can mutate through RefCell
```

**Trade-offs**:
- ✅ Bypasses compile-time borrow checking
- ❌ Runtime overhead for borrow checking
- ❌ Can panic if borrow rules violated at runtime

### **Technique 2: Smart Pointers (Rc for Shared Ownership)**

```rust
use std::rc::Rc;
use std::cell::RefCell;

type SharedVec = Rc<RefCell<Vec<i32>>>;

let data: SharedVec = Rc::new(RefCell::new(vec![1, 2, 3]));
let data_ref1 = Rc::clone(&data);       // Shared ownership
let data_ref2 = Rc::clone(&data);       // Shared ownership

// All references can mutate the same data:
data_ref1.borrow_mut().push(4);
data_ref2.borrow_mut().push(5);
println!("{:?}", data.borrow());        // [1, 2, 3, 4, 5]
```

### **Technique 3: Indices Instead of References**

```rust
// Instead of holding references:
struct GraphNode {
    neighbors: Vec<usize>,               // Indices instead of &Node
}

struct Graph {
    nodes: Vec<GraphNode>,
}

impl Graph {
    fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(node) = self.nodes.get_mut(from) {
            node.neighbors.push(to);     // No borrowing issues
        }
    }
    
    fn get_neighbors(&self, node: usize) -> &[usize] {
        &self.nodes[node].neighbors
    }
}
```

### **Technique 4: Generational Indices (Arena Pattern)**

```rust
struct Generation(u32);
struct Index { index: usize, generation: Generation }

struct Arena<T> {
    items: Vec<Option<(T, Generation)>>,
    generation: Generation,
}

impl<T> Arena<T> {
    fn add(&mut self, item: T) -> Index {
        let index = self.items.len();
        self.generation.0 += 1;
        self.items.push(Some((item, self.generation)));
        Index { index, generation: self.generation }
    }
    
    fn get(&self, idx: Index) -> Option<&T> {
        self.items.get(idx.index)?
            .as_ref()
            .filter(|(_, gen)| gen.0 == idx.generation.0)
            .map(|(item, _)| item)
    }
}
```

---

## 🛠️ **Borrow Checker Testing Strategies**

### **Testing Borrow Conflicts**

```rust
#[cfg(test)]
mod borrow_conflict_tests {
    use super::*;
    
    #[test]
    fn test_multiple_immutable_borrows() {
        let data = vec![1, 2, 3];
        let r1 = &data;
        let r2 = &data;                  // ✅ Multiple immutable OK
        assert_eq!(r1.len(), r2.len());
    }
    
    #[test] 
    fn test_sequential_mutable_borrows() {
        let mut data = vec![1, 2, 3];
        {
            let r1 = &mut data;
            r1.push(4);
        }   // r1 scope ends
        let r2 = &mut data;              // ✅ Sequential mutable OK
        r2.push(5);
    }
    
    // This test won't compile (which is what we want):
    // #[test]
    // fn test_simultaneous_mutable_borrows() {
    //     let mut data = vec![1, 2, 3];
    //     let r1 = &mut data;
    //     let r2 = &mut data;          // ❌ Compilation error
    // }
}
```

### **Testing Ownership Transfer**

```rust
#[test]
fn test_move_semantics() {
    let data = vec![1, 2, 3];
    let moved_data = data;               // Move occurs
    // assert_eq!(data.len(), 3);        // ❌ Won't compile
    assert_eq!(moved_data.len(), 3);     // ✅ Use moved value
}

#[test]
fn test_clone_vs_move() {
    let data = vec![1, 2, 3];
    let cloned_data = data.clone();      // Clone, don't move
    assert_eq!(data.len(), 3);           // ✅ Original still valid
    assert_eq!(cloned_data.len(), 3);    // ✅ Clone also valid
}
```

---

## 📊 **Performance Implications of Borrow Patterns**

### **Zero-Cost Abstractions**

```rust
// High-level iterator code:
let result: i32 = data.iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)
    .sum();

// Borrow checker enables this to compile to:
// Tight loop with no bounds checking or null pointer checks
```

### **When to Clone vs Borrow**

```rust
// Benchmark different approaches:
#[bench]
fn bench_clone_approach(b: &mut Bencher) {
    let data = vec![1; 1000];
    b.iter(|| {
        let cloned = data.clone();       // O(n) copy
        process(cloned)                  // No borrow conflicts
    });
}

#[bench]
fn bench_borrow_approach(b: &mut Bencher) {
    let mut data = vec![1; 1000];
    b.iter(|| {
        let result = process_ref(&data); // O(1) borrow
        data[0] += 1;                    // Modify after borrow ends
        result
    });
}
```

**Guidelines**:
- **Cheap to clone** (i32, bool, small structs): Prefer cloning
- **Expensive to clone** (Vec, HashMap, large structs): Prefer borrowing
- **Frequent mutations**: Consider RefCell or redesign for ownership transfer

---

## 🎯 **Mission-Specific Borrow Patterns**

### **Mission 1: Stack Borrow Patterns**

```rust
impl<T> Stack<T> {
    // Pattern: Separate peek from modification
    pub fn peek(&self) -> Option<&T> {           // Immutable borrow
        self.items.last()
    }
    
    pub fn peek_mut(&mut self) -> Option<&mut T> { // Mutable borrow
        self.items.last_mut()
    }
    
    // Pattern: Return owned values when possible
    pub fn pop(&mut self) -> Option<T> {         // Returns owned value
        self.items.pop()
    }
}

// Usage patterns:
let mut stack = Stack::new();
stack.push(42);

// Good: Separate read and write phases
if let Some(&value) = stack.peek() {     // Borrow for reading
    println!("Top: {}", value);
}   // Borrow ends
if some_condition {
    stack.push(value * 2);               // ✅ Can mutate now
}

// Good: Use owned values when possible
while let Some(value) = stack.pop() {    // Gets owned value
    process(value);                      // No borrow conflicts
    if should_continue(value) {
        stack.push(transform(value));    // ✅ Can modify stack
    }
}
```

### **Mission 4: LinkedList Borrow Challenges**

```rust
// The challenge: Traditional linked list operations need mutable borrows
// of multiple nodes simultaneously, which Rust prevents

// Problem case:
pub fn remove_after(&mut self, node: &mut Node<T>) -> Option<T> {
    // Would need: &mut node AND &mut node.next simultaneously
    // Rust prevents this for memory safety
}

// Solution patterns:

// Pattern 1: Use indices or IDs instead of references
pub struct LinkedList<T> {
    nodes: Vec<Option<Node<T>>>,
    free_list: Vec<usize>,
}

// Pattern 2: Use unsafe blocks with careful invariants
pub unsafe fn remove_after_unchecked(&mut self, node: *mut Node<T>) -> Option<T> {
    // Unsafe, but can express the operation
}

// Pattern 3: Use Rc<RefCell<T>> for shared mutability
pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}
```

---

## 🔗 **Integration with Broader Concepts**

### **Builds On**
- [[Borrow Checker Fundamentals]] - Basic understanding of ownership rules
- [[Ownership Transfer Patterns]] - When to move vs borrow
- [[Stack vs Heap Memory]] - Memory layout implications

### **Enables**
- [[Smart Pointers]] - Advanced ownership patterns (Box, Rc, Arc)
- [[interior-mutability]] - RefCell, Cell, and Mutex patterns
- [[Concurrent Programming]] - Thread-safe sharing with Arc + Mutex
- [[Lifetime Management]] - Advanced lifetime relationships

### **Related Troubleshooting**
- [[Debugging Lessons]] - General debugging strategies
- [[Compiler Error Patterns]] - Understanding error messages
- [[Performance Optimization]] - When borrow patterns impact performance

---

## 💡 **Key Takeaways**

1. **Pattern Recognition**: Learn to recognize common borrow conflict patterns
2. **Solution Toolkit**: Multiple approaches for each conflict type (copy, clone, restructure, scope limiting)
3. **Trade-off Awareness**: Understand performance and complexity costs of each solution
4. **Design Impact**: Let borrow checker influence your API design for the better
5. **Testing Strategy**: Write tests that verify your borrow patterns work correctly
6. **Advanced Tools**: RefCell, Rc, and Arena patterns for complex scenarios
7. **Performance Mindset**: Remember that compile-time checking enables runtime optimizations

**Remember**: Borrow checker conflicts are design feedback - they often point to better ways to structure your code for clarity and performance.

---

*Tags: #borrow-checker #troubleshooting #patterns #conflict-resolution #rust-advanced #memory-safety #performance #API-design*

*Links: [[zettel-index]] | [[Borrow Checker Fundamentals]] | [[Smart Pointers]] | [[interior-mutability]] | [[Ownership Transfer Patterns]] | [[Debugging Lessons]]*