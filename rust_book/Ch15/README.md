# Chapter 15: Smart Pointers

## 🔗 Zettelkasten Links
- **Overview**: [[zettelkasten/rust_book/rust-book-ch15]]
- **Previous**: [[zettelkasten/rust_book/rust-book-ch14]]
- **Next**: [[zettelkasten/rust_book/rust-book-ch16]]
- **Missions**: [[mission-3]] - Box<T> for BST nodes | [[mission-4]] - Rc<T> and RefCell<T> for linked lists
- **Daily Study**: [[daily-study/Day20]] | [[daily-study/Day21]] | [[daily-study/Day22]]
- **Book MOC**: [[Rust Book MOC]]

## 📚 Overview

Chapter 15 explores **smart pointers**, which are data structures that act like pointers but have additional metadata and capabilities. Unlike ordinary references, smart pointers often own the data they point to and provide automatic memory management through the ownership system.

**Official Reference**: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

---

## 🎯 Learning Objectives

By completing this chapter, you will understand:

1. **Box<T>** - Heap allocation for recursive types and trait objects
2. **Deref Trait** - Treating smart pointers like regular references with automatic dereferencing
3. **Drop Trait** - Running cleanup code automatically when values go out of scope
4. **Rc<T>** - Reference counting for multiple ownership scenarios
5. **RefCell<T>** - Interior mutability pattern for runtime borrow checking
6. **Reference Cycles** - Memory leaks with Rc<T> and how to prevent them using Weak<T>

**Integration Points**: This chapter connects to:
- **[[mission-3]]** - Binary Search Trees use Box<T> for recursive node structures
- **[[mission-4]]** - Linked Lists leverage Rc<T> and RefCell<T> for shared ownership
- **[[daily-study/Day20]]** - Heap allocation patterns and memory management
- **[[zettelkasten/smart-pointers-deep-dive]]** - Comprehensive smart pointer patterns

---

## 🎯 Chapter Concepts

### 15.1. Using Box<T> to Point to Data on the Heap

**Official Definition**: Box<T> is a smart pointer that allocates data on the heap rather than the stack. The box itself (the pointer) is stored on the stack.

**Practical Understanding**: Use Box<T> when you have:
- A type whose size can't be known at compile time (recursive types)
- A large amount of data you want to transfer ownership of without copying
- A value whose concrete type you want to hide (trait objects)

**Key Examples**:

```rust
// Basic heap allocation
fn basic_box_usage() {
    let b = Box::new(5);
    println!("b = {}", b);
    // Box is automatically deallocated when it goes out of scope
}

// Recursive type - MUST use Box to have known size
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn recursive_list_example() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // Without Box, this would be infinitely sized!
}

// Large data transfer without copying
struct LargeData {
    data: [u8; 1024 * 1024], // 1MB of data
}

fn transfer_ownership() {
    let large = Box::new(LargeData { data: [0; 1024 * 1024] });
    // Only the pointer is moved, not the 1MB of data
    let transferred = large;
}
```

**Common Mistakes**:
- **Mistake 1**: Using Box<T> when a simple reference would work - Box allocates heap memory unnecessarily
- **Mistake 2**: Forgetting that Box<T> owns its data - you can't use it after moving the box
- **Mistake 3**: Not recognizing when recursive types require Box - compiler will error with "infinite size"

**Integration**: Box<T> is used in [[mission-3]] for Binary Search Tree node allocation, enabling recursive tree structures.

---

### 15.2. Treating Smart Pointers Like Regular References with Deref

**Official Definition**: The Deref trait allows you to customize the behavior of the dereference operator `*`. By implementing Deref, smart pointers can be treated like regular references.

**Practical Understanding**: Deref coercion automatically converts references to smart pointers into references to their inner values. This enables ergonomic API usage without explicit dereferencing.

**Key Examples**:

```rust
use std::ops::Deref;

// Custom smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement Deref to enable * operator
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn deref_example() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);  // *y is equivalent to *(y.deref())
}

// Deref coercion in action
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_coercion_example() {
    let m = MyBox::new(String::from("Rust"));
    
    // Deref coercion: &MyBox<String> -> &String -> &str
    hello(&m);  // Works without &m.deref() or &(*m)
}

// Implicit deref coercion rules
fn coercion_rules() {
    // &T -> &U when T: Deref<Target=U>
    // &mut T -> &mut U when T: DerefMut<Target=U>
    // &mut T -> &U when T: Deref<Target=U>
}
```

**Common Mistakes**:
- **Mistake 1**: Confusing `*x` with `x.deref()` - `*x` is shorthand for `*(x.deref())`
- **Mistake 2**: Expecting deref coercion to work with value types - only works with references
- **Mistake 3**: Implementing Deref to emulate inheritance - Deref is for smart pointer patterns only

**Integration**: Deref enables ergonomic APIs in [[mission-4]] linked lists and [[mission-5]] hash maps.

---

### 15.3. Running Code on Cleanup with the Drop Trait

**Official Definition**: The Drop trait lets you customize what happens when a value goes out of scope. Rust calls the `drop` method automatically, enabling RAII (Resource Acquisition Is Initialization) patterns.

**Practical Understanding**: Drop is Rust's destructor mechanism. Use it to release resources like files, network connections, or custom memory allocations when values are no longer needed.

**Key Examples**:

```rust
// Custom Drop implementation
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_example() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // d dropped first, then c (reverse order of creation)
}

// Early drop with std::mem::drop
fn early_drop_example() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    
    drop(c);  // Explicitly drop before end of scope
    
    println!("CustomSmartPointer dropped before the end of main.");
}

// Drop order matters for dependent resources
struct Database {
    connection: String,
}

impl Drop for Database {
    fn drop(&mut self) {
        println!("Closing database connection: {}", self.connection);
    }
}

struct Transaction {
    db: Database,
    id: u32,
}

impl Drop for Transaction {
    fn drop(&mut self) {
        println!("Committing transaction {}", self.id);
        // Transaction dropped before Database (fields dropped in declaration order)
    }
}
```

**Common Mistakes**:
- **Mistake 1**: Calling `x.drop()` directly - use `std::mem::drop(x)` instead
- **Mistake 2**: Relying on drop order across different scopes - only guaranteed within same scope
- **Mistake 3**: Implementing Drop with panics - can lead to abort during unwinding

**Integration**: Drop is crucial in [[mission-4]] for proper cleanup of linked list nodes with Rc<T> and RefCell<T>.

---

### 15.4. Rc<T>, the Reference Counted Smart Pointer

**Official Definition**: Rc<T> (Reference Counted) enables multiple ownership of the same data. It keeps track of the number of references and deallocates when the count reaches zero.

**Practical Understanding**: Use Rc<T> when:
- You need multiple parts of your program to read the same data
- You can't determine at compile time which part will finish using the data last
- You're working with immutable shared data (for mutable, use Rc<RefCell<T>>)

**Key Examples**:

```rust
use std::rc::Rc;

// Multiple owners of heap data
fn rc_basic_example() {
    let a = Rc::new(5);
    println!("count after creating a = {}", Rc::strong_count(&a));
    
    let b = Rc::clone(&a);  // Increment reference count
    println!("count after creating b = {}", Rc::strong_count(&a));
    
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }  // c goes out of scope, count decremented
    
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// Shared ownership in data structures
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn shared_list_example() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    // a, b, and c all share ownership of the same tail
}

// Rc only allows immutable access
fn rc_immutability() {
    let data = Rc::new(vec![1, 2, 3]);
    let data_clone = Rc::clone(&data);
    
    // This would NOT compile:
    // data.push(4);  // Error: cannot borrow as mutable
    
    // Can only read
    println!("Data: {:?}", data);
}
```

**Common Mistakes**:
- **Mistake 1**: Using Rc<T> in multithreaded code - use Arc<T> instead for thread safety
- **Mistake 2**: Expecting Rc<T> to provide mutability - use Rc<RefCell<T>> for interior mutability
- **Mistake 3**: Creating reference cycles - leads to memory leaks (see section 15.6)

**Integration**: Rc<T> is essential in [[mission-4]] for implementing doubly-linked lists with multiple references to nodes.

---

### 15.5. RefCell<T> and the Interior Mutability Pattern

**Official Definition**: RefCell<T> provides interior mutability, allowing you to mutate data even when there are immutable references to it. It enforces borrowing rules at runtime instead of compile time.

**Practical Understanding**: RefCell<T> is for scenarios where you know your code follows borrowing rules, but the compiler can't verify it. Common in:
- Mock objects for testing
- Data structures with complex sharing patterns
- Implementing patterns that require mutation through shared references

**Key Examples**:

```rust
use std::cell::RefCell;

// Basic RefCell usage
fn refcell_basic() {
    let data = RefCell::new(5);
    
    // Borrow immutably
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("r1: {}, r2: {}", r1, r2);
    
    // Borrow mutably (must drop immutable borrows first)
    drop(r1);
    drop(r2);
    *data.borrow_mut() += 10;
    println!("Modified: {}", data.borrow());
}

// Interior mutability pattern
pub trait Messenger {
    fn send(&self, msg: &str);  // Takes &self, not &mut self
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// Mock messenger using RefCell for testing
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Interior mutability: mutate through &self
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Combining Rc<T> and RefCell<T> for shared mutable data
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn rc_refcell_example() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Mutate shared value
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

**Common Mistakes**:
- **Mistake 1**: Holding onto RefCell borrows too long - can cause runtime panics
- **Mistake 2**: Multiple mutable borrows at runtime - RefCell will panic
- **Mistake 3**: Using RefCell when compile-time checks would work - prefer compile-time safety

**Integration**: RefCell<T> combined with Rc<T> enables [[mission-4]]'s doubly-linked list with mutable nodes.

---

### 15.6. Reference Cycles Can Leak Memory

**Official Definition**: Reference cycles occur when Rc<T> pointers reference each other in a cycle, preventing reference counts from reaching zero. This creates memory leaks since the data is never deallocated.

**Practical Understanding**: While Rust prevents data races and use-after-free, it doesn't prevent memory leaks. Reference cycles are the primary way to leak memory with safe Rust. Use Weak<T> to break cycles.

**Key Examples**:

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// Tree node with parent and children
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,  // Weak to prevent cycle
    children: RefCell<Vec<Rc<Node>>>,  // Strong references to children
}

fn tree_example() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // branch dropped here, weak reference becomes invalid
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

// Creating a reference cycle (DO NOT DO THIS)
#[derive(Debug)]
struct CycleNode {
    value: i32,
    next: RefCell<Option<Rc<CycleNode>>>,
}

fn reference_cycle_leak() {
    let a = Rc::new(CycleNode {
        value: 5,
        next: RefCell::new(None),
    });

    let b = Rc::new(CycleNode {
        value: 10,
        next: RefCell::new(Some(Rc::clone(&a))),
    });

    *a.next.borrow_mut() = Some(Rc::clone(&b));
    
    // MEMORY LEAK: a and b reference each other
    // Reference counts never reach zero
}

// Breaking cycles with Weak<T>
fn weak_example() {
    let strong = Rc::new(5);
    let weak = Rc::downgrade(&strong);

    // Weak can be upgraded to Rc if value still exists
    assert_eq!(weak.upgrade(), Some(Rc::clone(&strong)));

    drop(strong);

    // After dropping strong reference, weak upgrade fails
    assert_eq!(weak.upgrade(), None);
}
```

**Common Mistakes**:
- **Mistake 1**: Not recognizing potential cycles in data structures - always analyze ownership graphs
- **Mistake 2**: Using Rc<T> for parent-child relationships without Weak<T> - causes memory leaks
- **Mistake 3**: Not testing with memory profilers - leaks may not be obvious during development

**Integration**: Understanding reference cycles is critical in [[mission-4]] for implementing graph structures and complex linked data structures without memory leaks.

---

## 🧪 **Exercises and Practice**

### **Basic Exercises**

Run examples with: `cargo run --example [example_name]`

1. **Exercise 1**: Box<T> Fundamentals
   - **Goal**: Understand heap allocation and recursive types
   - **Task**: Implement a cons list using Box<T>
   - **Validation**: Code compiles and correctly represents linked structure

2. **Exercise 2**: Custom Smart Pointer
   - **Goal**: Implement Deref trait for custom type
   - **Task**: Create MyBox<T> with Deref implementation
   - **Validation**: Deref coercion works in function calls

3. **Exercise 3**: Drop Order
   - **Goal**: Understand Drop trait execution order
   - **Task**: Create types with Drop that print order of cleanup
   - **Validation**: Observe reverse-order dropping

4. **Exercise 4**: Shared Ownership with Rc<T>
   - **Goal**: Use Rc<T> for multiple ownership
   - **Task**: Build shared list structure with multiple paths to same data
   - **Validation**: Reference counts behave as expected

5. **Exercise 5**: Interior Mutability
   - **Goal**: Apply RefCell<T> pattern
   - **Task**: Implement mock object pattern for testing
   - **Validation**: Can mutate through immutable reference safely

### **Integration Exercises**

Run tests with: `cargo test`

1. **Integration 1**: Combine concepts with [[daily-study/Day20]]
   - Build binary search tree using Box<T> for recursive structure
   - Apply Drop trait for proper cleanup

2. **Integration 2**: Apply patterns to [[mission-4]] requirements
   - Implement doubly-linked list with Rc<RefCell<Node>>
   - Prevent reference cycles using Weak<T> for back pointers

---

## 🔗 **Cross-References**

### **Prerequisites**
- **[[rust_book/Ch4]]**: Ownership fundamentals required for understanding smart pointer ownership
- **[[rust_book/Ch10]]**: Traits and generics used extensively in smart pointer implementations
- **[[daily-study/Day15]]**: Basic pointer concepts and memory management

### **Applications**
- **[[mission-3]]**: Uses Box<T> for Binary Search Tree recursive node allocation
- **[[mission-4]]**: Leverages Rc<T> and RefCell<T> for doubly-linked list implementation
- **[[advanced_examples/graph-structures]]**: Demonstrates smart pointers in complex data structures

### **Reinforcement**
- **[[zettelkasten/smart-pointers-deep-dive]]**: Comprehensive smart pointer patterns and use cases
- **[[tutorials/Mission4_tut]]**: Step-by-step application of Rc<T> and RefCell<T>
- **[[daily-study/Day21]]**: Interior mutability patterns in practice

---

## 📊 **Chapter Summary**

### **Key Takeaways**

1. **Box<T>** enables heap allocation and is required for recursive types
2. **Deref trait** allows smart pointers to act like regular references with automatic coercion
3. **Drop trait** provides automatic cleanup and RAII patterns
4. **Rc<T>** enables multiple ownership through reference counting (single-threaded only)
5. **RefCell<T>** provides interior mutability with runtime borrow checking
6. **Weak<T>** prevents reference cycles and memory leaks in Rc<T> graphs

### **Smart Pointer Decision Tree**

```
Need heap allocation?
├─ Yes, for recursive type → Box<T>
├─ Yes, for large data → Box<T>
└─ Need multiple owners?
   ├─ Yes, immutable data → Rc<T>
   └─ Yes, mutable data → Rc<RefCell<T>>
      └─ Has cycles? → Use Weak<T> for back references
```

---

*Run all examples*: `cargo run --example ch15_[section]`  
*Run all tests*: `cargo test`  
*Check quality*: `cargo clippy -- -D warnings`

*Summary Note*: [[rust_book/Ch15/CHAPTER_SUMMARY]]
