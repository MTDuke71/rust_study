# 🧠 Memory Management in Rust

*Comprehensive guide to Rust's memory safety and management patterns*

---

## 🎯 **Core Concepts**

### **The Memory Safety Trinity**
Rust achieves memory safety through three fundamental mechanisms:
1. **Ownership** - Single owner per value
2. **Borrowing** - Temporary access without ownership transfer
3. **Lifetimes** - Ensuring references remain valid

### **Zero-Cost Memory Safety**
- No garbage collector runtime overhead
- Memory errors caught at compile time
- Performance equivalent to manual memory management
- Automatic memory cleanup through RAII (Resource Acquisition Is Initialization)

---

## 📚 **Memory Layout Fundamentals**

### **Stack vs Heap**
```rust
// Stack allocation - fast, fixed size, automatic cleanup
fn stack_example() {
    let x = 42;           // Stored on stack
    let arr = [1, 2, 3];  // Fixed-size array on stack
}   // Automatically deallocated when function ends

// Heap allocation - flexible size, manual management (automated by Rust)
fn heap_example() {
    let vec = Vec::new();     // Heap-allocated, growable
    let boxed = Box::new(42); // Explicit heap allocation
    let string = String::from("hello"); // Heap-allocated string
}   // Drop trait automatically cleans up heap memory
```

### **Memory Regions**
- **Stack**: Function parameters, local variables, return addresses
- **Heap**: Dynamic allocations, growable collections
- **Static/Global**: Program constants, static variables
- **Code Segment**: Compiled program instructions

---

## 🏗️ **Ownership System**

### **Ownership Rules**
1. Each value has exactly one owner
2. When owner goes out of scope, value is dropped
3. Ownership can be transferred (moved) but not duplicated

```rust
// Basic ownership transfer
fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership moved to s2
    // println!("{}", s1); // ❌ Compile error - s1 no longer valid
    println!("{}", s2);   // ✅ s2 owns the string
}

// Function ownership transfer
fn take_ownership(s: String) {
    println!("{}", s);
} // s goes out of scope and is dropped

fn give_ownership() -> String {
    String::from("hello") // Ownership transferred to caller
}
```

### **Copy vs Move Semantics**
```rust
// Copy types (stored on stack)
let x = 5;
let y = x;  // x is copied, both x and y are valid
println!("x: {}, y: {}", x, y); // ✅ Both accessible

// Move types (heap-allocated)
let s1 = String::from("hello");
let s2 = s1; // s1 is moved, only s2 is valid
// println!("{}", s1); // ❌ Compile error

// Explicit cloning for deep copy
let s3 = s2.clone();
println!("s2: {}, s3: {}", s2, s3); // ✅ Both valid after clone
```

---

## 🔗 **Borrowing and References**

### **Immutable Borrowing**
```rust
fn borrowing_example() {
    let s = String::from("hello");
    let len = calculate_length(&s); // Borrow without moving
    println!("Length of '{}' is {}", s, len); // s still valid
}

fn calculate_length(s: &String) -> usize {
    s.len() // Can read but not modify
} // Reference goes out of scope, no cleanup needed
```

### **Mutable Borrowing**
```rust
fn mutable_borrowing() {
    let mut s = String::from("hello");
    change(&mut s); // Mutable borrow
    println!("{}", s); // Modified string
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
```

### **Borrowing Rules**
- Any number of immutable references **OR** exactly one mutable reference
- References must always be valid (no dangling pointers)
- Cannot modify through immutable reference

```rust
// Valid: Multiple immutable borrows
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2); // ✅ Multiple readers allowed

// Invalid: Mixing mutable and immutable borrows
let mut s = String::from("hello");
let r1 = &s;
// let r2 = &mut s; // ❌ Compile error
// println!("{} {}", r1, r2);
```

---

## ⏰ **Lifetimes**

### **Lifetime Annotations**
```rust
// Explicit lifetime parameter
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
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part // Lifetime elision rules apply
    }
}
```

### **Lifetime Elision Rules**
1. Each reference parameter gets its own lifetime
2. If exactly one input lifetime, it's assigned to all outputs
3. If multiple inputs but one is `&self`, its lifetime assigned to outputs

```rust
// These are equivalent due to elision:
fn first_word(s: &str) -> &str { /* ... */ }
fn first_word<'a>(s: &'a str) -> &'a str { /* ... */ }
```

---

## 📦 **Smart Pointers**

### **Box<T> - Heap Allocation**
```rust
// Simple heap allocation
let boxed_int = Box::new(42);
println!("Boxed value: {}", boxed_int);

// Recursive data structures
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
```

### **Rc<T> - Reference Counting**
```rust
use std::rc::Rc;

// Shared ownership
let data = Rc::new(String::from("shared data"));
let reference1 = Rc::clone(&data);
let reference2 = Rc::clone(&data);

println!("Reference count: {}", Rc::strong_count(&data)); // 3
// All references must be dropped before data is deallocated
```

### **RefCell<T> - Interior Mutability**
```rust
use std::cell::RefCell;
use std::rc::Rc;

// Mutable data with shared ownership
let data = Rc::new(RefCell::new(vec![1, 2, 3]));
let reference = Rc::clone(&data);

// Runtime borrowing checks
data.borrow_mut().push(4);
println!("{:?}", reference.borrow()); // [1, 2, 3, 4]
```

---

## 🔧 **RAII and Drop Trait**

### **Automatic Resource Management**
```rust
struct CustomResource {
    name: String,
}

impl Drop for CustomResource {
    fn drop(&mut self) {
        println!("Cleaning up resource: {}", self.name);
    }
}

fn raii_example() {
    let _resource1 = CustomResource {
        name: String::from("Resource 1"),
    };
    let _resource2 = CustomResource {
        name: String::from("Resource 2"),
    };
    // Resources automatically cleaned up in reverse order
    // Output: "Cleaning up resource: Resource 2"
    //         "Cleaning up resource: Resource 1"
}
```

### **Manual Memory Management**
```rust
use std::alloc::{alloc, dealloc, Layout};

unsafe fn manual_allocation() {
    let layout = Layout::new::<i32>();
    let ptr = alloc(layout) as *mut i32;
    
    if !ptr.is_null() {
        *ptr = 42;
        println!("Value: {}", *ptr);
        dealloc(ptr as *mut u8, layout); // Manual cleanup required
    }
}
```

---

## 🔄 **Memory Patterns and Best Practices**

### **Avoiding Common Pitfalls**
```rust
// ❌ Dangling reference
// fn dangling_reference() -> &String {
//     let s = String::from("hello");
//     &s // s will be dropped, reference becomes invalid
// }

// ✅ Return owned data instead
fn proper_return() -> String {
    String::from("hello") // Ownership transferred to caller
}

// ❌ Use after move
// fn use_after_move() {
//     let s = String::from("hello");
//     let t = s; // s moved to t
//     println!("{}", s); // Error: s no longer valid
// }

// ✅ Clone or borrow when needed
fn proper_usage() {
    let s = String::from("hello");
    let t = s.clone(); // Explicit copy
    println!("s: {}, t: {}", s, t); // Both valid
}
```

### **Performance Considerations**
```rust
// Prefer borrowing for read operations
fn process_data(data: &Vec<i32>) -> i32 {
    data.iter().sum() // No ownership transfer needed
}

// Use owned parameters when taking ownership
fn consume_data(mut data: Vec<i32>) -> Vec<i32> {
    data.push(42);
    data // Return modified data
}

// Consider Cow for flexible ownership
use std::borrow::Cow;

fn process_string(data: Cow<str>) -> String {
    match data {
        Cow::Borrowed(s) => format!("Processed: {}", s),
        Cow::Owned(s) => format!("Processed: {}", s),
    }
}
```

---

## 🧪 **Memory-Related Testing Patterns**

### **Testing Drop Behavior**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_drop_order() {
        let drop_order = Arc::new(Mutex::new(Vec::new()));
        
        {
            let _resource1 = TrackingResource::new("first", Arc::clone(&drop_order));
            let _resource2 = TrackingResource::new("second", Arc::clone(&drop_order));
        } // Resources dropped here
        
        let order = drop_order.lock().unwrap();
        assert_eq!(*order, vec!["second", "first"]); // LIFO order
    }
}

struct TrackingResource {
    name: String,
    tracker: Arc<Mutex<Vec<String>>>,
}

impl TrackingResource {
    fn new(name: &str, tracker: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            name: name.to_string(),
            tracker,
        }
    }
}

impl Drop for TrackingResource {
    fn drop(&mut self) {
        self.tracker.lock().unwrap().push(self.name.clone());
    }
}
```

---

## 🔗 **Integration with Mission Work**

### **Mission Connections**
- **[[missions/Mission1 Overview|Mission1]]**: Stack-based memory management in stack implementation
- **[[mission-4]]**: Heap allocation patterns in linked list nodes
- **[[mission-5]]**: Memory layout optimization in HashMap implementation
- **[[Box Smart Pointer Patterns]]**: Detailed Box<T> usage patterns

### **Daily Study Connections**
- **[[daily-study/Day02]]**: Variables and memory layout fundamentals
- **[[daily-study/Day03]]**: References and borrowing basics
- **[[rust_book/rust-book-ch4]]**: Ownership system foundations
- **[[rust_book/rust-book-ch15]]**: Smart pointers and advanced patterns

### **Real-World Applications**
- **Resource management**: File handles, network connections
- **Performance optimization**: Zero-copy operations, memory pools
- **Safety guarantees**: Preventing memory leaks and use-after-free
- **Concurrent programming**: Shared state management

---

## 🎯 **Key Takeaways**

1. **Ownership prevents**: Use-after-free, double-free, memory leaks
2. **Borrowing enables**: Efficient access without ownership transfer
3. **Lifetimes ensure**: References remain valid throughout usage
4. **RAII guarantees**: Automatic resource cleanup
5. **Smart pointers provide**: Flexible ownership patterns when needed

Memory management in Rust is about **compile-time guarantees** rather than runtime overhead, enabling both safety and performance.

---

*Tags: #memory-management #ownership #borrowing #lifetimes #smart-pointers #raii #heap #stack #performance #safety*
*Links: [[zettel-index]] | [[missions/Mission1 Overview|Mission1]] | [[mission-4]] | [[Box Smart Pointer Patterns]] | [[rust_book/rust-book-ch4]] | [[rust_book/rust-book-ch15]]*