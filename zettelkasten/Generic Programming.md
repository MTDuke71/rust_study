# 🔧 Generic Programming in Rust

**Type parameterization and reusable code patterns through generics, traits, and associated types**

## 🎯 Core Concept

**Generic Programming** enables writing code that works with multiple types while maintaining type safety and zero runtime cost. Rust's generics are resolved at compile time through **monomorphization**, generating specialized code for each concrete type used.

## 📦 Generic Types

### **Generic Structs**

```rust
// Generic data structure
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
}

// Usage with different types
let mut int_stack = Stack::<i32>::new();
int_stack.push(42);

let mut string_stack = Stack::<String>::new();
string_stack.push("hello".to_string());
```

**Mission Applications:**
- [[Mission1]] - Generic Stack<T> implementation
- [[Mission2]] - Generic Queue<T> with RingBuffer<T>
- [[Mission5]] - HashMap<K, V> with generic keys and values

### **Generic Enums**

```rust
// Standard library examples
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Custom generic enum
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}
```

### **Generic Functions**

```rust
// Generic function with type parameter
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

// Generic function with multiple type parameters
fn zip<A, B>(a: Vec<A>, b: Vec<B>) -> Vec<(A, B)> {
    a.into_iter().zip(b.into_iter()).collect()
}

// Generic function returning generic type
fn first<T: Clone>(slice: &[T]) -> Option<T> {
    slice.first().cloned()
}
```

## 🎨 Trait Bounds

### **Simple Trait Bounds**

```rust
// Require T to implement Display
fn print_it<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// Multiple trait bounds
fn compare_and_print<T: PartialOrd + std::fmt::Display>(a: T, b: T) {
    if a > b {
        println!("{} is greater", a);
    } else {
        println!("{} is greater", b);
    }
}
```

### **Where Clauses**

```rust
// Complex bounds with where clause
fn complex_function<T, U>(t: T, u: U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    println!("t: {}", t);
    println!("u: {:?}", u);
    42
}

// Where clause for associated types
fn print_iter<T>(collection: T)
where
    T: IntoIterator,
    T::Item: std::fmt::Display,
{
    for item in collection {
        println!("{}", item);
    }
}
```

### **Trait Bound Patterns**

```rust
// Default trait for generic initialization
fn default_vector<T: Default>() -> Vec<T> {
    vec![T::default(); 10]
}

// Clone trait for duplication
fn duplicate<T: Clone>(item: &T) -> (T, T) {
    (item.clone(), item.clone())
}

// Copy trait for stack-based copying
fn triple<T: Copy>(item: T) -> (T, T, T) {
    (item, item, item)  // Copy happens automatically
}
```

## 🏗️ Generic Implementations

### **Impl Blocks with Generics**

```rust
struct Point<T> {
    x: T,
    y: T,
}

// Generic implementation for all types
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Specialized implementation for specific type
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Implementation with trait bounds
impl<T: std::fmt::Display> Point<T> {
    fn display(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}
```

### **Multiple Type Parameters**

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
    
    fn into_tuple(self) -> (T, U) {
        (self.first, self.second)
    }
}

// Different types can be mixed
let pair1 = Pair::new(42, "hello");
let pair2 = Pair::new(3.14, vec![1, 2, 3]);
```

## 🔗 Associated Types

### **Trait with Associated Types**

```rust
trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

// Implementation specifies concrete associated type
impl Iterator for Counter {
    type Item = u32;  // Concrete type
    
    fn next(&mut self) -> Option<u32> {
        // implementation
    }
}
```

**Associated Types vs Generic Type Parameters:**

```rust
// Generic type parameter - can have multiple implementations per type
trait Add<Rhs> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Associated type - only one implementation per type
trait Graph {
    type Node;
    type Edge;
    
    fn neighbors(&self, node: &Self::Node) -> Vec<Self::Edge>;
}
```

## ⚡ Zero-Cost Abstractions

### **Monomorphization**

Rust generates specialized code for each concrete type at compile time:

```rust
// Generic function
fn print_value<T: std::fmt::Display>(val: T) {
    println!("{}", val);
}

// Compiler generates separate functions:
// fn print_value_i32(val: i32) { println!("{}", val); }
// fn print_value_String(val: String) { println!("{}", val); }

print_value(42);          // Uses print_value_i32
print_value("hello");     // Uses print_value_String
```

**Benefits:**
- ✅ No runtime overhead
- ✅ Full compiler optimizations for each type
- ✅ Type safety maintained
- ❌ Larger binary size (trade-off)

### **Performance Example**

```rust
// Generic version - zero runtime cost
fn generic_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// After monomorphization, equivalent to:
fn max_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
```

## 📚 Common Generic Patterns

### **Builder Pattern with Generics**

```rust
struct ConfigBuilder<T> {
    value: T,
    debug: bool,
}

impl<T> ConfigBuilder<T> {
    fn new(value: T) -> Self {
        ConfigBuilder { value, debug: false }
    }
    
    fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
    
    fn build(self) -> Config<T> {
        Config {
            value: self.value,
            debug: self.debug,
        }
    }
}
```

### **Type State Pattern**

```rust
struct Locked;
struct Unlocked;

struct Door<State> {
    _state: std::marker::PhantomData<State>,
}

impl Door<Locked> {
    fn new() -> Self {
        Door { _state: std::marker::PhantomData }
    }
    
    fn unlock(self) -> Door<Unlocked> {
        Door { _state: std::marker::PhantomData }
    }
}

impl Door<Unlocked> {
    fn open(&self) {
        println!("Door opened");
    }
    
    fn lock(self) -> Door<Locked> {
        Door { _state: std::marker::PhantomData }
    }
}
```

Related: [[PhantomData Type Safety Patterns]]

### **Generic Collections**

```rust
// Generic wrapper for any collection type
struct CollectionWrapper<T, C>
where
    C: IntoIterator<Item = T>,
{
    collection: C,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, C> CollectionWrapper<T, C>
where
    C: IntoIterator<Item = T>,
{
    fn new(collection: C) -> Self {
        CollectionWrapper {
            collection,
            _phantom: std::marker::PhantomData,
        }
    }
}
```

## 🎯 Mission Integration Examples

### **Mission1: Generic Stack**

```rust
// Generic stack implementation
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

// Works with any type
let mut stack = Stack::<i32>::new();
stack.push(42);

let mut stack = Stack::<String>::new();
stack.push("AoC".to_string());
```

Related: [[Mission1]] | [[Stack Data Structure]]

### **Mission5: Generic HashMap**

```rust
// Generic hash map with key-value pairs
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Implementation
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        // Implementation
    }
}
```

Related: [[Mission5 Overview]] | [[HashMap Internals]]

### **Mission6: Generic Grid**

```rust
// Generic 2D grid
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Grid {
            data: vec![default; width * height],
            width,
            height,
        }
    }
    
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }
}
```

Related: [[Mission 6]]

## 🧪 Testing Generic Code

### **Property-Based Testing**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generic_stack_with_integers() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }
    
    #[test]
    fn test_generic_stack_with_strings() {
        let mut stack = Stack::<String>::new();
        stack.push("first".to_string());
        stack.push("second".to_string());
        assert_eq!(stack.pop(), Some("second".to_string()));
    }
    
    // Test with custom type
    #[derive(Debug, PartialEq)]
    struct CustomType(i32);
    
    #[test]
    fn test_generic_stack_with_custom_type() {
        let mut stack = Stack::<CustomType>::new();
        stack.push(CustomType(42));
        assert_eq!(stack.pop(), Some(CustomType(42)));
    }
}
```

## 🚀 Best Practices

### **Design Guidelines**

1. **Use trait bounds appropriately**
   ```rust
   // Good: Only require traits you actually use
   fn process<T: Clone>(item: T) -> T {
       item.clone()
   }
   
   // Avoid: Over-constraining unnecessarily
   fn process_bad<T: Clone + Copy + Debug + Display>(item: T) -> T {
       item.clone()  // Only uses Clone!
   }
   ```

2. **Prefer associated types for output types**
   ```rust
   // Good: Single implementation per type
   trait Container {
       type Item;
       fn items(&self) -> Vec<Self::Item>;
   }
   
   // Avoid: When only one implementation makes sense
   trait ContainerBad<T> {
       fn items(&self) -> Vec<T>;
   }
   ```

3. **Use `where` clauses for clarity**
   ```rust
   // Good: Readable with where clause
   fn complex<T, U>(t: T, u: U)
   where
       T: Clone + Debug,
       U: Display + Default,
   {
       // implementation
   }
   
   // Avoid: Hard to read inline bounds
   fn complex_bad<T: Clone + Debug, U: Display + Default>(t: T, u: U) {
       // implementation
   }
   ```

## ⚠️ Common Pitfalls

### **Lifetime Elision in Generics**

```rust
// Needs explicit lifetime
fn first_ref<'a, T>(slice: &'a [T]) -> Option<&'a T> {
    slice.first()
}

// Compiler can infer in simple cases
fn first_ref_elided<T>(slice: &[T]) -> Option<&T> {
    slice.first()
}
```

Related: [[Day 04 - Lifetimes]] | [[Multiple Lifetimes Deep Dive]]

### **Generic Type Inference**

```rust
// Explicit type annotation needed
let numbers: Vec<i32> = vec![1, 2, 3]
    .into_iter()
    .map(|x| x * 2)
    .collect();  // collect needs to know target type

// Or use turbofish syntax
let numbers = vec![1, 2, 3]
    .into_iter()
    .map(|x| x * 2)
    .collect::<Vec<i32>>();
```

### **Trait Object Limitations**

```rust
// Not object-safe (generic method)
trait NotObjectSafe {
    fn generic_method<T>(&self, item: T);
}

// Object-safe alternative
trait ObjectSafe {
    fn concrete_method(&self, item: i32);
}
```

Related: [[Day 19 - Trait Objects]]

## 🔗 Related Concepts

### **Type System Foundations**
- [[Rust Trinity - Struct Trait Impl]] - The three fundamental building blocks
- [[Day 15 - Traits Fundamentals]] - Trait definition and implementation
- [[Day 16 - Generic Types]] - Generic type parameters and constraints
- [[Day 18 - Advanced Traits]] - Associated types and advanced patterns
- [[Day 19 - Trait Objects]] - Dynamic dispatch with `dyn Trait`

### **Mission Applications**
- [[Mission1]] - Generic Stack<T> implementation
- [[Mission2]] - Generic Queue<T> and RingBuffer<T>
- [[Mission3]] - Generic binary search algorithms
- [[Mission4]] - Generic LinkedList<T> with smart pointers
- [[Mission5 Overview]] - Generic HashMap<K, V> and HashSet<T>
- [[Mission 6]] - Generic Grid<T> for 2D arrays

### **Collections & Iterators**
- [[Collections MOC]] - Overview of generic collections
- [[Day 13 - Advanced Iterators]] - Generic iterator patterns
- [[Iterator Traits]] - Custom iterator implementations
- [[HashMap Internals]] - Generic hash table implementation

### **Advanced Patterns**
- [[PhantomData Type Safety Patterns]] - Zero-cost type state
- [[Clone vs Copy]] - Value semantics with generics
- [[Zero-Cost Abstractions]] - Monomorphization and performance
- [[Box Smart Pointer Patterns]] - Heap allocation with generics

### **Learning Resources**
- [[Daily Study MOC]] - Progressive learning track
- [[Rust Concepts MOC]] - Core language features
- [[Rust Book]] - The Rust Programming Language (Chapter 10)
- [[AoC Patterns MOC]] - Competitive programming with generics

## 📊 Complexity Analysis

**Compile-Time Cost:**
- Monomorphization increases compilation time
- Each concrete type generates specialized code
- Binary size grows with type usage

**Runtime Cost:**
- ✅ Zero runtime overhead (monomorphization)
- ✅ Fully optimized specialized code
- ✅ No dynamic dispatch (unlike trait objects)
- ✅ Inline-friendly for hot paths

## 🎓 Learning Path

1. **Foundations** → [[Day 16 - Generic Types]]
2. **Trait Bounds** → [[Day 15 - Traits Fundamentals]]
3. **Associated Types** → [[Day 18 - Advanced Traits]]
4. **Practice** → [[Mission1]], [[Mission5 Overview]]
5. **Advanced** → [[PhantomData Type Safety Patterns]]

---

*Tags: #generics #type-parameters #trait-bounds #associated-types #monomorphization #zero-cost-abstractions #rust-type-system #reusable-code*

*Links: [[zettel-index]] | [[Rust Concepts MOC]] | [[Rust Trinity - Struct Trait Impl]] | [[Day 16 - Generic Types]] | [[Collections MOC]]*
