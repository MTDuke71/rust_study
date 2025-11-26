# 🔤 Generics in Rust

**Type parameterization for writing flexible, reusable code while maintaining type safety**

## 🎯 Core Concept

**Generics** allow you to write code that works with multiple types without sacrificing type safety or performance. Rust's generics are **zero-cost abstractions** - the compiler generates specialized code for each concrete type through **monomorphization**, resulting in the same performance as hand-written code for each type.

## 📦 Generic Functions

### **Basic Generic Function**

```rust
// Generic function with type parameter T
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Works with any type that implements PartialOrd
let numbers = vec![1, 5, 3, 9, 2];
let result = largest(&numbers); // 9

let chars = vec!['a', 'm', 'z', 'b'];
let result = largest(&chars); // 'z'
```

### **Multiple Type Parameters**

```rust
fn swap<T, U>(tuple: (T, U)) -> (U, T) {
    (tuple.1, tuple.0)
}

let pair = ("hello", 42);
let swapped = swap(pair); // (42, "hello")
```

## 🏗️ Generic Structs

### **Single Type Parameter**

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// Usage with different types
let integer_point = Point::new(5, 10);
let float_point = Point::new(3.5, 7.2);
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
    
    fn split(self) -> (T, U) {
        (self.first, self.second)
    }
}

let pair = Pair::new("answer", 42);
let (text, number) = pair.split();
```

### **Mixing Generic and Concrete Types**

```rust
struct Container<T> {
    value: T,
    metadata: String, // Always String
}

impl<T> Container<T> {
    fn new(value: T, metadata: String) -> Self {
        Container { value, metadata }
    }
}
```

## 🎭 Generic Enums

### **Option and Result**

```rust
// Standard library generic enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Usage
let some_number: Option<i32> = Some(5);
let result: Result<i32, String> = Ok(42);
```

### **Custom Generic Enum**

```rust
enum Tree<T> {
    Leaf(T),
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T> Tree<T> {
    fn leaf(value: T) -> Self {
        Tree::Leaf(value)
    }
}
```

## 🔧 Generic Implementations

### **Impl Blocks for Generic Types**

```rust
struct Stack<T> {
    items: Vec<T>,
}

// Implementation for any type T
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

// Additional implementation only for specific types
impl Stack<String> {
    fn push_str(&mut self, s: &str) {
        self.items.push(s.to_string());
    }
}
```

### **Methods with Additional Generic Parameters**

```rust
impl<T> Point<T> {
    // Method with its own generic parameter
    fn mix_with<U>(self, other: Point<U>) -> Point<(T, U)> {
        Point {
            x: (self.x, other.x),
            y: (self.y, other.y),
        }
    }
}

let p1 = Point { x: 5, y: 10 };
let p2 = Point { x: 'a', y: 'b' };
let p3 = p1.mix_with(p2); // Point<(i32, char)>
```

## 🎯 Trait Bounds

### **Basic Trait Bounds**

```rust
// Require T to implement Display
fn print_value<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}

// Multiple trait bounds
fn process<T: Clone + std::fmt::Debug>(value: T) {
    let copy = value.clone();
    println!("{:?}", copy);
}
```

### **Where Clauses**

```rust
// More readable for complex bounds
fn complex_function<T, U>(t: T, u: U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    println!("{}", t);
    println!("{:?}", u);
    42
}
```

### **Trait Bounds in Structs**

```rust
struct Container<T: Clone> {
    value: T,
}

impl<T: Clone> Container<T> {
    fn duplicate(&self) -> T {
        self.value.clone()
    }
}
```

## 🚀 Performance and Monomorphization

### **Zero-Cost Abstraction**

```rust
// Generic function
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// Compiler generates specialized versions:
// fn add_i32(a: i32, b: i32) -> i32 { a + b }
// fn add_f64(a: f64, b: f64) -> f64 { a + b }

add(5, 10);    // Uses generated add_i32
add(3.5, 2.1); // Uses generated add_f64
```

**Result**: No runtime cost! Performance equivalent to writing separate functions for each type.

## 📊 Generic Collections

### **Vec, HashMap, HashSet**

```rust
// Vector with generic type
let numbers: Vec<i32> = vec![1, 2, 3];
let strings: Vec<String> = vec!["a".to_string(), "b".to_string()];

// HashMap with generic key and value types
use std::collections::HashMap;
let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert("Blue".to_string(), 10);

// HashSet with generic type
use std::collections::HashSet;
let mut unique_numbers: HashSet<i32> = HashSet::new();
unique_numbers.insert(1);
```

## 🎓 Mission Applications

### **Mission 1 - Generic Stack**

```rust
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
```

### **Mission 2 - Generic Queue**

```rust
struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}
```

### **Mission 3 - Generic Binary Search**

```rust
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    
    None
}
```

### **Mission 5 - Generic HashMap**

```rust
use std::hash::Hash;

struct HashMap<K: Hash + Eq, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    fn insert(&mut self, key: K, value: V) {
        // Implementation
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        // Implementation
        None
    }
}
```

## 💡 Best Practices

### **1. Use Generic Functions for Flexibility**

```rust
// Good: Works with any type
fn first<T>(list: &[T]) -> Option<&T> {
    list.first()
}

// Less flexible: Only works with i32
fn first_i32(list: &[i32]) -> Option<&i32> {
    list.first()
}
```

### **2. Add Trait Bounds Only When Needed**

```rust
// Good: Minimal constraints
fn store<T>(value: T) -> Box<T> {
    Box::new(value)
}

// Over-constrained: Unnecessary bounds
fn store_bad<T: Clone + Debug>(value: T) -> Box<T> {
    Box::new(value)
}
```

### **3. Use Type Inference When Possible**

```rust
// Explicit type annotation
let v: Vec<i32> = Vec::new();

// Type inference
let mut v = Vec::new();
v.push(5); // Compiler infers Vec<i32>
```

### **4. Consider Default Type Parameters**

```rust
struct Counter<T = i32> {
    count: T,
}

let c1: Counter<i32> = Counter { count: 0 };
let c2: Counter<_> = Counter { count: 0 }; // Uses default i32
```

## 🔍 Advanced Patterns

### **Phantom Types**

```rust
use std::marker::PhantomData;

struct Container<T> {
    _marker: PhantomData<T>,
    data: Vec<u8>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container {
            _marker: PhantomData,
            data: Vec::new(),
        }
    }
}
```

### **Associated Types vs Generic Parameters**

```rust
// Generic parameter
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// Associated type (preferred)
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## 🐛 Common Pitfalls

### **Lifetime Annotation Confusion**

```rust
// Need lifetime when returning references
fn first<'a, T>(list: &'a [T]) -> Option<&'a T> {
    list.first()
}
```

### **Type Inference Ambiguity**

```rust
// Ambiguous
let v = Vec::new(); // What type?

// Clear
let v: Vec<i32> = Vec::new();
// or
let mut v = Vec::new();
v.push(5); // Now compiler knows it's Vec<i32>
```

### **Recursive Generic Types**

```rust
// Need Box for recursive types
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
```

## 📚 Related Concepts

- [[Generic Programming]] - Comprehensive generic programming patterns
- [[Traits]] - Trait system and trait bounds
- [[zettelkasten/rust_book/rust-book-ch10]] - Rust Book Chapter 10: Generics, Traits, Lifetimes
- [[Ownership and Borrowing]] - Generic types and ownership
- [[rust-concepts-MOC]] - Core language features

---

## 🎯 Key Takeaways

1. **Generics enable type-safe code reuse** without performance cost
2. **Monomorphization creates specialized code** for each concrete type
3. **Trait bounds constrain generic types** to types with specific capabilities
4. **Zero-cost abstraction**: generic code is as fast as specialized code
5. **Type inference reduces verbosity** while maintaining safety
6. **Generic collections are fundamental** to Rust programming (Vec, HashMap, etc.)
7. **Combine generics with traits** for powerful, flexible APIs

---

*Tags: #rust #generics #type-parameters #monomorphization #zero-cost-abstractions*
*Links: [[Generic Programming]] | [[Traits]] | [[zettelkasten/rust_book/rust-book-ch10]] | [[rust-concepts-MOC]]*
