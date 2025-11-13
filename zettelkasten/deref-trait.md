# Deref Trait - Smart Pointer Behavior

**Tags:** #deref #smart-pointers #trait #ergonomics #coercion #rust-book-ch15 #zero-cost-abstractions

**Related:** [[drop-trait]], [[box-heap-allocation]], [[rc-shared-ownership]], [[smart-pointers]], [[wrapper-pattern]], [[zero-cost-abstractions]], [[trait-system]]

---

## Core Concept

The **Deref trait** enables types to customize the behavior of the **dereference operator** (`*`) and provides **automatic deref coercion** for ergonomic APIs. It's fundamental to Rust's smart pointer ecosystem and wrapper patterns.

```rust
use std::ops::Deref;

trait Deref {
    type Target;
    fn deref(&self) -> &Self::Target;
}
```

### **Fundamental Principle**
When you use `*value`, Rust calls `*(value.deref())` - converting the dereference into a method call that returns a reference.

---

## Basic Implementation Pattern

### **Custom Smart Pointer Example**
```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0  // Return reference to wrapped value
    }
}

// Usage
let x = 5;
let y = MyBox::new(x);
assert_eq!(5, *y);  // *y becomes *(y.deref())
```

### **Mission Integration: Tracked Wrapper**
```rust
use std::cell::Cell;

struct TrackedBox<T> {
    value: Box<T>,
    access_count: Cell<u32>,
}

impl<T> TrackedBox<T> {
    fn new(value: T) -> Self {
        TrackedBox {
            value: Box::new(value),
            access_count: Cell::new(0),
        }
    }
    
    fn accesses(&self) -> u32 {
        self.access_count.get()
    }
}

impl<T> Deref for TrackedBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.access_count.set(self.access_count.get() + 1);
        &self.value
    }
}

// Automatic tracking of every dereference!
let tracked = TrackedBox::new(String::from("hello"));
println!("{}", tracked.len()); // Deref coercion: &TrackedBox -> &String -> &str
println!("Accesses: {}", tracked.accesses()); // 1
```

---

## Deref Coercion Magic

### **Automatic Type Conversion**
Deref coercion automatically converts `&T` to `&U` when `T: Deref<Target=U>`:

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}

let boxed_string = Box::new(String::from("Rust"));
greet(&boxed_string);  // &Box<String> -> &String -> &str
```

### **Multiple Coercion Chain**
```rust
struct Wrapper<T>(T);

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

fn print_length(s: &str) {
    println!("Length: {}", s.len());
}

let wrapped = Wrapper(Box::new(String::from("Chain example")));
print_length(&wrapped);
// Coercion chain: &Wrapper<Box<String>> -> &Box<String> -> &String -> &str
```

---

## Practical Applications

### **1. Type Safety with Ergonomics**
```rust
struct UserId(u64);
struct Email(String);

impl Deref for Email {
    type Target = str;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Email {
    fn new(s: String) -> Result<Self, &'static str> {
        if s.contains('@') { 
            Ok(Email(s)) 
        } else { 
            Err("Invalid email") 
        }
    }
    
    fn domain(&self) -> &str {
        self.split('@').nth(1).unwrap_or("")
    }
}

// Usage: Get ALL string methods + type safety
let email = Email::new("user@example.com".to_string())?;
println!("Length: {}", email.len());        // String method via deref
println!("Domain: {}", email.domain());     // Custom method
```

### **2. AoC Problem: Configuration Wrapper**
```rust
struct AocConfig {
    input: String,
    debug: bool,
}

impl Deref for AocConfig {
    type Target = str;
    fn deref(&self) -> &Self::Target { &self.input }
}

impl AocConfig {
    fn debug_print(&self, msg: &str) {
        if self.debug {
            println!("[DEBUG] {}", msg);
        }
    }
}

// AoC usage: String methods + configuration
fn solve_day1(config: &AocConfig) -> i32 {
    config.debug_print("Starting Day 1");
    
    config.lines()                    // String method via deref
        .map(|line| line.parse::<i32>().unwrap_or(0))
        .sum()
}
```

### **3. Resource Management with Access Control**
```rust
struct FileHandle {
    file: std::fs::File,
    read_only: bool,
}

impl Deref for FileHandle {
    type Target = std::fs::File;
    fn deref(&self) -> &Self::Target { &self.file }
}

impl FileHandle {
    fn ensure_writable(&self) -> Result<(), &'static str> {
        if self.read_only {
            Err("File is read-only")
        } else {
            Ok(())
        }
    }
}

// Get all File methods + custom validation
```

---

## Deref Coercion Rules

### **The Three Rules**
1. **`&T` → `&U`** when `T: Deref<Target=U>`
2. **`&mut T` → `&mut U`** when `T: DerefMut<Target=U>`
3. **`&mut T` → `&U`** when `T: Deref<Target=U>` *(mutable to immutable)*

### **Key Insight: Rule 3**
```rust
fn read_data(data: &str) { /* read only */ }
fn modify_data(data: &mut str) { /* can modify */ }

let mut s = String::from("hello");
read_data(&mut s);    // ✅ &mut String -> &String -> &str (Rule 3)
// modify_data(&s);   // ❌ Can't convert &str to &mut str
```

**Memory Safety**: Rust allows borrowing immutably from mutable reference, but not the reverse.

---

## Performance Characteristics

### **Zero-Cost Abstraction**
```rust
// These are identical after optimization:
let boxed = Box::new(42);
let value1 = *boxed;           // Manual dereference
let value2 = *(boxed.deref()); // What Rust actually does

// Deref coercion is compile-time only - no runtime cost!
```

### **When NOT to Use Deref**
```rust
// ❌ Don't use Deref for conversions that might fail
impl Deref for ResultWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.result.as_ref().unwrap()  // Can panic!
    }
}

// ✅ Use explicit methods instead
impl ResultWrapper<T> {
    fn get(&self) -> Option<&T> {
        self.result.as_ref().ok()
    }
}
```

---

## Integration with Other Traits

### **Deref + Drop Pattern**
```rust
struct ManagedResource<T> {
    resource: T,
    cleanup_fn: Option<Box<dyn FnOnce(&T)>>,
}

impl<T> Deref for ManagedResource<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.resource }
}

impl<T> Drop for ManagedResource<T> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup_fn.take() {
            cleanup(&self.resource);
        }
    }
}

// Automatic cleanup + transparent access
```

### **Mission Connection: Union-Find Enhancement**
```rust
struct TrackedUnionFind<T> {
    inner: UnionFind<T>,
    operations: Cell<usize>,
}

impl<T> Deref for TrackedUnionFind<T> {
    type Target = UnionFind<T>;
    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T> DerefMut for TrackedUnionFind<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.operations.set(self.operations.get() + 1);
        &mut self.inner
    }
}

// All UnionFind methods work + automatic operation counting!
let mut uf = TrackedUnionFind::new(100);
uf.union(1, 2);  // DerefMut -> increment counter -> call UnionFind::union
uf.union(3, 4);  
println!("Operations: {}", uf.operations.get()); // 2
```

---

## Testing Patterns

### **Deref Behavior Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref_behavior() {
        let wrapped = MyBox::new(String::from("test"));
        
        // Direct dereference
        assert_eq!("test", *wrapped);
        
        // Deref coercion
        assert_eq!(4, wrapped.len());
        
        // Method calls work transparently
        assert!(wrapped.starts_with("te"));
    }
    
    #[test]
    fn test_deref_coercion_chain() {
        let nested = Wrapper(Box::new(String::from("nested")));
        
        fn takes_str(s: &str) -> usize { s.len() }
        
        assert_eq!(6, takes_str(&nested));
    }
}
```

---

## Common Patterns & Idioms

### **Smart Pointer Composition**
```rust
// Layer multiple smart pointer behaviors
type LoggedTrackedBox<T> = LogWrapper<TrackedBox<T>>;

struct LogWrapper<T>(T);

impl<T: Deref> Deref for LogWrapper<T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        println!("Accessing wrapped value");
        self.0.deref()
    }
}

// Automatic logging + tracking + boxing in one type!
```

### **AoC Input Parsing Wrapper**
```rust
struct AocInput(String);

impl AocInput {
    fn lines(&self) -> impl Iterator<Item = &str> {
        self.0.lines()
    }
    
    fn numbers(&self) -> impl Iterator<Item = i32> + '_ {
        self.0.split_whitespace()
            .filter_map(|s| s.parse().ok())
    }
}

impl Deref for AocInput {
    type Target = str;
    fn deref(&self) -> &Self::Target { &self.0 }
}

// Usage in AoC solutions
let input = AocInput(std::fs::read_to_string("input.txt")?);
let total: i32 = input.numbers().sum();        // Custom method
let line_count = input.lines().count();       // Standard str method
```

---

## Best Practices

### **✅ When to Implement Deref**
- **Smart pointers** that own and provide access to data
- **Wrapper types** that should be transparent
- **Newtype patterns** for domain modeling with ergonomics

### **❌ When NOT to Implement Deref**
- **Conversions that might fail** or are expensive
- **Types that aren't conceptually "smart pointers"**
- **When you want explicit conversion** for clarity

### **Design Guidelines**
1. **Deref should be cheap** - it's called implicitly and often
2. **Target type should be obvious** - clear conceptual relationship
3. **Don't break expectations** - `*ptr` should behave like direct access
4. **Consider DerefMut** when mutable access makes sense

---

## Learning Progression

### **Foundation → Application**
1. **Understand `*` operator** and how Rust desugars it
2. **Implement basic Deref** for simple wrapper types
3. **Master deref coercion** and its automatic nature
4. **Apply to real problems** like AoC input handling
5. **Combine with other traits** for powerful abstractions

### **Mission Integration Path**
- **Mission 4**: `Rc<RefCell<T>>` uses Deref extensively
- **Mission 10**: Could enhance Union-Find with tracking wrappers
- **AoC Problems**: Input parsing and configuration wrappers
- **Production Code**: API clients, database connections, file handles

---

## Advanced Topics

### **Deref and Lifetimes**
```rust
struct BorrowedWrapper<'a, T> {
    data: &'a T,
}

impl<'a, T> Deref for BorrowedWrapper<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { self.data }
}

// Lifetime flows through deref automatically
```

### **Generic Deref Implementations**
```rust
struct Container<T, U> {
    inner: T,
    _marker: std::marker::PhantomData<U>,
}

impl<T, U> Deref for Container<T, U> 
where 
    T: Deref<Target = U>
{
    type Target = U;
    fn deref(&self) -> &Self::Target { self.inner.deref() }
}
```

---

**Core Concepts:** [[smart-pointers]] | [[wrapper-pattern]] | [[zero-cost-abstractions]] | [[trait-system]]  
**Applications:** [[aoc-input-parsing]] | [[configuration-management]] | [[resource-wrappers]] | [[type-safety-patterns]]  
**Integration:** [[drop-trait]] | [[box-heap-allocation]] | [[mission-4]] | [[rust-book-ch15]]

*Links: [[deref-coercion]] | [[smart-pointer-composition]] | [[ergonomic-apis]] | [[zero-cost-wrapper-pattern]] | [[automatic-type-conversion]]*