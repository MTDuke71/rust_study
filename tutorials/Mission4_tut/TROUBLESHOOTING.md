# Troubleshooting Guide: Linked Lists in Rust

## 🚨 Common Errors and Solutions

### Error 1: "Cannot move out of borrowed content"

**Error Message:**
```
error[E0507]: cannot move out of borrowed content
   --> src/lib.rs:45:13
    |
45  |             self.head.next
    |             ^^^^^^^^^^^^^^ move occurs because `self.head.next` has type `Option<Box<Node<T>>>`, which does not implement the `Copy` trait
```

**What's happening:**
You're trying to move ownership of `next` while it's still borrowed as part of `self`.

**Example of problematic code:**
```rust
fn pop_front(&mut self) -> Option<T> {
    match &self.head {
        Some(node) => {
            self.head = node.next;  // ❌ Can't move out of borrowed content
            Some(node.data)         // ❌ Can't move out of borrowed content
        }
        None => None,
    }
}
```

**Solution - Use `take()`:**
```rust
fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|node| {
        self.head = node.next;  // ✅ We own `node` now
        node.data               // ✅ We can move out of owned data
    })
}
```

**Key Insight:** `take()` moves the value out of `Option`, leaving `None` behind.

---

### Error 2: "Already borrowed: BorrowMutError"

**Error Message:**
```
thread 'main' panicked at 'already borrowed: BorrowMutError', src/lib.rs:23:34
```

**What's happening:**
You're trying to get a mutable borrow while another borrow (mutable or immutable) is still active.

**Example of problematic code:**
```rust
let node_ref = list.get_node_ref(0)?;
let _borrow1 = node_ref.borrow();
let _borrow2 = node_ref.borrow_mut(); // ❌ Panic! Can't mix borrows
```

**Solution 1 - Use try_ methods:**
```rust
let node_ref = list.get_node_ref(0)?;
let _borrow1 = node_ref.try_borrow()?;
match node_ref.try_borrow_mut() {
    Ok(_borrow2) => { /* Use mutable borrow */ }
    Err(_) => { /* Handle borrow conflict gracefully */ }
}
```

**Solution 2 - Scope your borrows:**
```rust
let node_ref = list.get_node_ref(0)?;
{
    let _borrow1 = node_ref.borrow();
    // Use borrow1
} // borrow1 dropped here

let _borrow2 = node_ref.borrow_mut(); // ✅ Now this works
```

**Solution 3 - Design around single borrows:**
```rust
// Instead of holding multiple borrows, extract what you need
let data = {
    let borrow = node_ref.borrow();
    borrow.data.clone() // Extract the data
}; // borrow dropped here

// Now you can get a mutable borrow
let mut_borrow = node_ref.borrow_mut();
```

---

### Error 3: "Cannot borrow as mutable more than once"

**Error Message:**
```
error[E0499]: cannot borrow `*self` as mutable more than once at a time
   --> src/lib.rs:67:9
    |
67  |         self.update_tail();
    |         ^^^^ second mutable borrow occurs here
```

**What's happening:**
You're calling multiple methods that need `&mut self` within the same scope.

**Example of problematic code:**
```rust
impl<T> DoublyLinkedList<T> {
    fn push_back(&mut self, data: T) {
        let new_node = self.create_node(data);  // First &mut self borrow
        self.update_tail(new_node);             // ❌ Second &mut self borrow
    }
}
```

**Solution - Restructure the logic:**
```rust
impl<T> DoublyLinkedList<T> {
    fn push_back(&mut self, data: T) {
        // Do all the work in one method to avoid multiple borrows
        let new_node = Rc::new(RefCell::new(DoublyNode {
            data,
            next: None,
            prev: None,
        }));
        
        // Handle tail update inline
        match &self.tail.as_ref().and_then(|w| w.upgrade()) {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }
        
        self.tail = Some(Rc::downgrade(&new_node));
        self.length += 1;
    }
}
```

---

### Error 4: "Weak reference could not be upgraded"

**What's happening:**
You're trying to upgrade a `Weak<T>` reference, but the original data has been dropped.

**Example of problematic code:**
```rust
let weak_ref = get_some_weak_reference();
let strong_ref = weak_ref.upgrade().unwrap(); // ❌ Might panic!
```

**Solution - Always check upgrade results:**
```rust
let weak_ref = get_some_weak_reference();
match weak_ref.upgrade() {
    Some(strong_ref) => {
        // Use strong_ref safely
    }
    None => {
        // Handle the case where reference is no longer valid
        return Err(LinkedListError::InvalidReference);
    }
}
```

**Better pattern with `and_then`:**
```rust
let result = weak_ref
    .upgrade()
    .map(|strong_ref| {
        // Use strong_ref
        strong_ref.borrow().data.clone()
    })
    .ok_or(LinkedListError::InvalidReference)?;
```

---

### Error 5: Memory Leaks (Reference Cycles)

**What's happening:**
You've created a cycle where objects keep each other alive forever.

**Example of problematic code:**
```rust
struct BadNode<T> {
    data: T,
    next: Option<Rc<RefCell<BadNode<T>>>>,
    prev: Option<Rc<RefCell<BadNode<T>>>>, // ❌ Both are strong references!
}
```

**Problem visualization:**
```
Node A -> Node B (via Rc)
Node A <- Node B (via Rc)

Reference counts never reach 0, so neither is dropped!
```

**Solution - Use Weak for back-references:**
```rust
struct GoodNode<T> {
    data: T,
    next: Option<Rc<RefCell<GoodNode<T>>>>,
    prev: Option<Weak<RefCell<GoodNode<T>>>>, // ✅ Weak reference breaks cycle
}
```

**Cycle broken:**
```
Node A -> Node B (via Rc, counts toward ref count)
Node A <- Node B (via Weak, doesn't count toward ref count)

When no external references exist, Node A's ref count reaches 0,
it gets dropped, which drops its reference to Node B,
so Node B's ref count reaches 0 and it gets dropped too.
```

---

### Error 6: "Use of moved value"

**Error Message:**
```
error[E0382]: use of moved value: `node`
   --> src/lib.rs:89:20
    |
89  |         self.head = node.next;
    |                     --------- value moved here
90  |         Some(node.data)
    |              ^^^^^^^^^ value used here after move
```

**What's happening:**
You moved part of a struct but then tried to use another part.

**Example of problematic code:**
```rust
fn pop_front(&mut self) -> Option<T> {
    if let Some(node) = self.head.take() {
        self.head = node.next;    // `next` moved out of `node`
        Some(node.data)           // ❌ Can't use `node.data` after partial move
    } else {
        None
    }
}
```

**Solution - Destructure the struct:**
```rust
fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|node| {
        let Node { data, next } = *node;  // Destructure to get ownership
        self.head = next;
        data
    })
}
```

**Alternative - Extract what you need first:**
```rust
fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|node| {
        self.head = node.next.clone(); // Clone if necessary
        node.data
    })
}
```

---

## 🔍 Debugging Strategies

### 1. Use `Rc::strong_count()` and `Rc::weak_count()`

```rust
fn debug_reference_counts<T>(node: &Rc<RefCell<T>>) {
    println!("Strong count: {}", Rc::strong_count(node));
    println!("Weak count: {}", Rc::weak_count(node));
}
```

This helps identify:
- Memory leaks (counts that never decrease)
- Unexpected sharing (higher counts than expected)

### 2. Add Debug Prints for Borrow State

```rust
fn debug_borrow_state<T: std::fmt::Debug>(node: &Rc<RefCell<T>>) {
    match node.try_borrow() {
        Ok(borrow) => println!("Successfully borrowed: {:?}", *borrow),
        Err(_) => println!("Borrow conflict detected!"),
    }
}
```

### 3. Use Scope Guards for Cleanup

```rust
struct BorrowGuard<T> {
    _borrow: std::cell::Ref<'static, T>,
}

impl<T> Drop for BorrowGuard<T> {
    fn drop(&mut self) {
        println!("Borrow guard dropped - reference released");
    }
}
```

### 4. Visual Debugging with Graphviz

```rust
fn print_list_structure<T: std::fmt::Debug>(list: &RcLinkedList<T>) {
    println!("digraph list {{");
    let mut current = list.head.clone();
    let mut index = 0;
    
    while let Some(node) = current {
        let borrow = node.borrow();
        println!("  node{} [label=\"{:?}\"];", index, borrow.data);
        
        if let Some(ref next) = borrow.next {
            println!("  node{} -> node{};", index, index + 1);
        }
        
        current = borrow.next.clone();
        index += 1;
    }
    
    println!("}}");
}
```

---

## 🛠️ Prevention Patterns

### 1. RAII (Resource Acquisition Is Initialization)

Always acquire and release resources in matching scopes:

```rust
{
    let borrow = node.borrow_mut();
    // Use borrow
    // Automatically dropped at end of scope
}
```

### 2. Early Returns for Error Handling

```rust
fn safe_operation(&mut self) -> Result<(), LinkedListError> {
    let node = self.head.as_ref().ok_or(LinkedListError::EmptyList)?;
    let borrow = node.try_borrow().map_err(|_| LinkedListError::BorrowConflict)?;
    
    // Rest of operation...
    Ok(())
}
```

### 3. Prefer Immutable Operations

When possible, design your API to avoid mutation:

```rust
// Instead of modifying in place
fn modify_data(&mut self, f: impl Fn(&mut T)) { ... }

// Prefer transformation
fn map_data<U>(&self, f: impl Fn(&T) -> U) -> RcLinkedList<U> { ... }
```

### 4. Use Type-Safe Builders

```rust
pub struct ListBuilder<T> {
    items: Vec<T>,
}

impl<T> ListBuilder<T> {
    pub fn new() -> Self { Self { items: Vec::new() } }
    
    pub fn add(mut self, item: T) -> Self {
        self.items.push(item);
        self
    }
    
    pub fn build(self) -> RcLinkedList<T> {
        let mut list = RcLinkedList::new();
        for item in self.items {
            list.push_back(item).unwrap(); // Safe because we control the building
        }
        list
    }
}
```

---

## 📚 Additional Resources

- [The Rust Reference - Smart Pointers](https://doc.rust-lang.org/reference/types/pointer.html)
- [Rust By Example - Rc](https://doc.rust-lang.org/rust-by-example/std/rc.html)
- [The Rustonomicon - Ownership and Lifetimes](https://doc.rust-lang.org/nomicon/ownership.html)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)

## 🎯 Quick Reference

| Problem | Likely Cause | Solution |
|---------|--------------|----------|
| "Cannot move out of borrowed content" | Trying to move while borrowed | Use `take()` or destructuring |
| "Already borrowed" panic | Multiple active borrows | Use `try_` methods and scope management |
| "Cannot borrow mutably more than once" | Multiple `&mut self` in same scope | Restructure to single borrow |
| Weak reference fails to upgrade | Original data was dropped | Always check `upgrade()` result |
| Memory leaks | Reference cycles with `Rc` | Use `Weak` for back-references |
| "Use of moved value" | Partial move from struct | Destructure or clone before moving |

---

## 🔗 Related Resources

**Mission4 Tutorial:**
- [[README|README]] - Main tutorial overview
- [[COMPILE_ERROR_ANALYSIS|COMPILE_ERROR_ANALYSIS]] - Understanding unsafe pointer errors
- [[TYPE_BREAKDOWN|TYPE_BREAKDOWN]] - `Option<Box<Node<T>>>` explained
- [[Compilation Stages|compilation_stages/README]] - From source to executable
- [[QUICK_DEBUG_START|QUICK_DEBUG_START]] - Fast debugging reference

**Mission4 Implementation:**
- [[Mission4 Overview|../../missions/Mission4/README]] - Main linked list V-Cycle documentation

**Zettelkasten Core Concepts:**
- [[Ownership and Borrowing|../../zettelkasten/Ownership and Borrowing]] - Core ownership rules
- [[Borrow Checker Fundamentals|../../zettelkasten/Borrow Checker Fundamentals]] - The borrow checker explained
- [[Box Smart Pointer|../../zettelkasten/Box Smart Pointer]] - Unique ownership
- [[Rc and RefCell|../../zettelkasten/Rc and RefCell]] - Shared ownership and interior mutability
- [[Memory Safety|../../zettelkasten/Memory Safety]] - Rust's safety guarantees
- [[Error Handling Deep Dive|../../zettelkasten/Error Handling Deep Dive]] - Result and error patterns

**Common Error Patterns:**
- [[Copy vs Clone vs Move|../../zettelkasten/Copy vs Clone vs Move]] - Data transfer semantics
- [[Lifetime Errors|../../zettelkasten/Lifetime Errors]] - Common lifetime issues
- [[Borrow Checker Patterns and Troubleshooting|../../zettelkasten/Borrow Checker Patterns and Troubleshooting]] - Working with borrows

**Rust Book Integration:**
- [[Chapter 4|../../rust_book/Ch4/README]] - Understanding Ownership
- [[Chapter 15|../../rust_book/Ch15/README]] - Smart Pointers (Box, Rc, RefCell)
- [[Week 2 Overview|../../zettelkasten/Week 2 Overview]] - Advanced ownership patterns

**Related Missions:**
- [[mission-1|../../zettelkasten/Mission1 Overview]] - Stack (ownership basics)
- [[mission-2|../../zettelkasten/Mission2 Overview]] - Queue (ownership in practice)
- [[Mission5 Overview|../../zettelkasten/Mission5 Overview]] - HashMap (complex ownership)

**Learning Support:**
- [[Rust Concepts MOC|../../zettelkasten/Rust Concepts MOC]] - Navigate all concepts
- [[TDD (Test-Driven Development)|../../zettelkasten/Test-Driven Development]] - TDD approach

---

*Tags: #mission4 #troubleshooting #linked-list #borrowing #rc #refcell #error-solutions #debugging*
*Links: [[README|README]] | [[Mission4 Overview|../../missions/Mission4/README]] | [[Ownership and Borrowing|../../zettelkasten/Ownership and Borrowing]]*