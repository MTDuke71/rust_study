# Closures in Rust

*Anonymous functions that capture their environment*

---

## 🎯 **What is a Closure?**

A **closure** is an anonymous function that can capture variables from its surrounding environment.

**Syntax**: `|parameters| expression`

```rust
// Closure in Mission2's peek method
self.head.as_ref().map(|node| &node.elem)
//                      └──────────────┘
//                         Closure!
```

## 📖 **Closure Anatomy**

```rust
|node| &node.elem
//│    │ │
//│    │ └─ Body: what the closure does
//│    └─── Separator (pipe symbol)
//└──────── Parameter: what the closure receives
```

### **Breakdown**

- **Parameters**: `node` - receives `&Box<Node<T>>`
- **Body**: `&node.elem` - returns `&T`
- **No explicit types needed** - Rust infers them from context

## 🔄 **Closure vs Named Function**

### **Using Closure (Concise)**

```rust
pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
}
```

### **Using Named Function (Verbose)**

```rust
fn extract_elem<T>(node: &Box<Node<T>>) -> &T {
    &node.elem
}

pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(extract_elem)
}
```

**Result**: Same behavior, but closure is inline and clear!

## 🎨 **How Closures Work with Option::map**

```rust
// If self.head.as_ref() returns Some(node_reference)
self.head.as_ref().map(|node| &node.elem)
//                    │      │
//                    │      └─ Return a reference to the elem field
//                    └──────── Take the node reference as parameter

// Expands to:
match self.head.as_ref() {
    Some(node) => Some(&node.elem),
    None => None,
}
```

## 💡 **Simple Closure Examples**

### **Basic Closure**

```rust
let add_one = |x| x + 1;
println!("{}", add_one(5)); // prints 6
```

### **Closure with Multiple Parameters**

```rust
let add = |a, b| a + b;
println!("{}", add(3, 4)); // prints 7
```

### **Closure with Explicit Types**

```rust
let multiply: fn(i32, i32) -> i32 = |x, y| x * y;
println!("{}", multiply(3, 4)); // prints 12
```

### **Closure with Block Body**

```rust
let complex = |x| {
    let doubled = x * 2;
    let tripled = doubled + x;
    tripled // Return value
};
println!("{}", complex(5)); // prints 15
```

## 🎯 **Capturing Environment**

Unlike regular functions, closures can **capture** variables from their surrounding scope:

```rust
let multiplier = 3;
let multiply = |x| x * multiplier;  // Captures 'multiplier'
println!("{}", multiply(4)); // prints 12
```

### **Three Capture Modes**

#### **1. By Reference (`&T`) - Borrowing**

```rust
let list = vec![1, 2, 3];
let contains_two = |x| list.contains(&x);  // Borrows list
println!("{}", contains_two(2)); // true
println!("{:?}", list); // Can still use list!
```

#### **2. By Mutable Reference (`&mut T`) - Mutable Borrowing**

```rust
let mut count = 0;
let mut increment = || {
    count += 1;  // Mutably borrows count
};
increment();
increment();
println!("{}", count); // 2
```

#### **3. By Value (`T`) - Taking Ownership**

```rust
let list = vec![1, 2, 3];
let consume = move || {
    println!("{:?}", list);  // Takes ownership
};
consume();
// println!("{:?}", list); // ❌ Error: list moved into closure
```

## 🔑 **The `move` Keyword**

Forces closure to take **ownership** of captured variables:

```rust
let name = String::from("Rust");

// Without move - borrows
let print = || println!("{}", name);
print();
println!("{}", name); // ✅ Still available

// With move - takes ownership
let name2 = String::from("Rust");
let print2 = move || println!("{}", name2);
print2();
// println!("{}", name2); // ❌ Error: name2 moved
```

## 🎮 **Real-World Mission2 Examples**

### **Option::map with Closure**

```rust
let maybe_number = Some(42);
let result = maybe_number.map(|n| n * 2); // Some(84)
```

### **Iterator with Closure**

```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter()
    .map(|n| n * 2)
    .collect();
// [2, 4, 6, 8, 10]
```

### **Filter with Closure**

```rust
let numbers = vec![1, 2, 3, 4, 5];
let evens: Vec<&i32> = numbers.iter()
    .filter(|n| *n % 2 == 0)
    .collect();
// [2, 4]
```

### **Custom Closure in Queue**

```rust
// Process all queue elements
while let Some(value) = queue.dequeue() {
    let processor = |v| println!("Processing: {}", v);
    processor(value);
}
```

## 📊 **Closure Types**

Rust has **three closure traits** that determine how closures are used:

### **1. `Fn` - Can be called multiple times without modifying captured data**

```rust
fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,  // F implements Fn
{
    f(f(x))
}

let double = |x| x * 2;
println!("{}", apply_twice(double, 5)); // 20
```

### **2. `FnMut` - Can be called multiple times and may modify captured data**

```rust
fn apply_to_each<F>(f: &mut F, items: &[i32])
where
    F: FnMut(i32),
{
    for item in items {
        f(*item);
    }
}

let mut sum = 0;
let mut accumulate = |x| sum += x;
apply_to_each(&mut accumulate, &[1, 2, 3]);
println!("{}", sum); // 6
```

### **3. `FnOnce` - Can be called once, may consume captured data**

```rust
fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f(); // Can only call once
}

let data = vec![1, 2, 3];
let consume = move || drop(data);  // Consumes data
call_once(consume);
// call_once(consume); // ❌ Error: consume already called
```

## 🔄 **Closure in Method Chains**

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let result: i32 = numbers.iter()
    .filter(|n| *n % 2 == 0)      // Keep evens
    .map(|n| n * n)                // Square them
    .sum();                        // Sum them up
println!("{}", result); // 4 + 16 + 36 = 56
```

## 🎯 **Why Closures are Powerful**

### **1. Concise**

No need to define separate functions for simple operations

### **2. Inline**

Logic stays close to where it's used

### **3. Capture Environment**

Can access variables from surrounding scope

### **4. Functional Style**

Enables chaining operations elegantly

### **5. Zero-Cost Abstractions**

Compiled to efficient machine code

## 💡 **Best Practices**

### **Use Type Inference**

```rust
// ✅ Good - let Rust infer types
let add = |a, b| a + b;

// ⚠️ Verbose - explicit types rarely needed
let add: fn(i32, i32) -> i32 = |a, b| a + b;
```

### **Keep Closures Simple**

```rust
// ✅ Good - simple, readable
.map(|n| n * 2)

// ⚠️ Complex - consider named function
.map(|n| {
    let doubled = n * 2;
    let tripled = doubled + n;
    let result = tripled * 4;
    result + 10
})
```

### **Use move When Needed**

```rust
// ✅ Good - move into thread
std::thread::spawn(move || {
    println!("{}", data);
});

// ❌ Bad - would create dangling reference
std::thread::spawn(|| {
    println!("{}", data); // Error!
});
```

## 🧠 **Mental Model**

Think of closures as:

> **"Inline functions with memory"**

They can:

- Take parameters (like functions)
- Return values (like functions)
- Remember variables from their environment (unlike functions)

## 🎮 **Practical Mission2 Pattern**

```rust
pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
}
```

**What's happening**:

1. `self.head.as_ref()` → `Option<&Box<Node<T>>>`
2. `.map(...)` → Takes a closure to transform the value
3. `|node|` → Receives `&Box<Node<T>>`
4. `&node.elem` → Returns `&T` through deref coercion
5. **Result**: `Option<&T>` - a reference to the element if it exists

**Why it's perfect**:

- **Short and readable** - one line!
- **Safe** - maintains borrowing semantics
- **Efficient** - compiles to optimal machine code
- **Functional** - clear data transformation

---

*Tags: #closures #functional-programming #mission2 #iterators #higher-order-functions #capture-environment*

*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[Iterator Patterns]] | [[Option and Result Handling]] | [[../missions/Mission2/README|Mission2 Queue]] | [[Functional Programming in Rust]]*
