# Function Pointers in Rust

*Stateless, efficient function references without environment capture*

---

## 🎯 **What is a Function Pointer?**

A **function pointer** is a type that points to a function, allowing functions to be passed as arguments or stored in data structures. Unlike closures, function pointers **cannot capture environment** - they only work with parameters passed to them.

**Type syntax**: `fn(T) -> U`

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

// Function pointer type
let fn_ptr: fn(i32) -> i32 = add_one;
fn_ptr(5)  // Returns 6
```

## 🔄 **Function Pointers vs Closures**

### **Key Differences**

| **Feature** | **Function Pointer (`fn`)** | **Closure (`Fn`, `FnMut`, `FnOnce`)** |
|-------------|---------------------------|-----------------------------------|
| **Captures environment** | ❌ No | ✅ Yes |
| **Size** | Fixed (pointer size, 8 bytes) | Variable (depends on captures) |
| **Type** | Concrete type `fn(T) -> U` | Each closure has unique type |
| **Coercion** | Regular functions coerce to `fn` | Closures coerce to trait objects |
| **Performance** | No indirection overhead | May have capture overhead |

### **Comparison Example**

```rust
// Function pointer - no captures
fn add_one(x: i32) -> i32 {
    x + 1
}

let multiplier = 10;

// Closure - captures multiplier
let multiply_by = |x| x * multiplier;  // Captures environment

// Function pointer can't capture
fn multiply_by_fn(x: i32) -> i32 {
    x * multiplier  // ❌ Error: can't find value `multiplier`
}
```

## 🏗️ **Basic Usage Patterns**

### **1. Passing Functions as Arguments**

```rust
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn add_one(x: i32) -> i32 { x + 1 }
fn double(x: i32) -> i32 { x * 2 }

let result1 = do_twice(add_one, 5);  // 12
let result2 = do_twice(double, 5);   // 20
```

### **2. Function Pointers in Collections**

```rust
// Array of function pointers
let operations: [fn(i32) -> i32; 3] = [
    |x| x + 1,
    |x| x * 2,
    |x| x - 1,
];

for op in operations {
    println!("{}", op(10));
}

// Vec of function pointers
let mut funcs: Vec<fn(i32) -> i32> = Vec::new();
funcs.push(add_one);
funcs.push(double);
```

### **3. Method References as Function Pointers**

```rust
// Fully qualified method syntax
let numbers = vec![1, 2, 3];

// Method as function pointer
let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();

// Equivalent to:
let strings: Vec<String> = numbers.iter().map(|n| n.to_string()).collect();
```

## 🎯 **Common Patterns**

### **1. Enum Initializers as Function Pointers**

```rust
enum Status {
    Value(u32),
    Error(String),
}

// Enum variant constructor is a function pointer
let statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();

// Equivalent to:
let statuses: Vec<Status> = (0u32..5).map(|x| Status::Value(x)).collect();
```

### **2. Strategy Pattern**

```rust
struct Calculator {
    operation: fn(i32, i32) -> i32,
}

impl Calculator {
    fn new(op: fn(i32, i32) -> i32) -> Self {
        Calculator { operation: op }
    }
    
    fn calculate(&self, a: i32, b: i32) -> i32 {
        (self.operation)(a, b)
    }
}

fn add(a: i32, b: i32) -> i32 { a + b }
fn multiply(a: i32, b: i32) -> i32 { a * b }

let calc = Calculator::new(add);
println!("{}", calc.calculate(5, 3));  // 8

let calc = Calculator::new(multiply);
println!("{}", calc.calculate(5, 3));  // 15
```

### **3. Callback Registration**

```rust
struct EventHandler {
    on_click: Option<fn()>,
    on_submit: Option<fn(String)>,
}

impl EventHandler {
    fn new() -> Self {
        EventHandler {
            on_click: None,
            on_submit: None,
        }
    }
    
    fn register_click(&mut self, callback: fn()) {
        self.on_click = Some(callback);
    }
    
    fn trigger_click(&self) {
        if let Some(callback) = self.on_click {
            callback();
        }
    }
}

fn handle_click() {
    println!("Button clicked!");
}

let mut handler = EventHandler::new();
handler.register_click(handle_click);
handler.trigger_click();
```

## 🔧 **Advanced Usage**

### **Function Pointer with Multiple Parameters**

```rust
fn apply_operation(
    op: fn(i32, i32) -> i32,
    values: &[i32]
) -> Vec<i32> {
    values.windows(2)
          .map(|pair| op(pair[0], pair[1]))
          .collect()
}

fn add(a: i32, b: i32) -> i32 { a + b }

let nums = vec![1, 2, 3, 4, 5];
let result = apply_operation(add, &nums);  // [3, 5, 7, 9]
```

### **Generic Function Pointers**

```rust
fn apply_to_all<T>(f: fn(T) -> T, items: Vec<T>) -> Vec<T> {
    items.into_iter().map(f).collect()
}

fn double(x: i32) -> i32 { x * 2 }
fn uppercase(s: String) -> String { s.to_uppercase() }

let doubled = apply_to_all(double, vec![1, 2, 3]);
// Can't do this - different types need monomorphization
```

## ⚡ **Performance Characteristics**

### **Zero-Cost Abstraction**

```rust
// Direct function call
fn add(x: i32, y: i32) -> i32 { x + y }
let result = add(5, 3);

// Function pointer call - same assembly code after optimization
let fn_ptr = add;
let result = fn_ptr(5, 3);
```

### **Fixed Size**

```rust
use std::mem::size_of;

// All function pointers are pointer-sized
println!("{}", size_of::<fn()>());              // 8 bytes (64-bit)
println!("{}", size_of::<fn(i32) -> i32>());    // 8 bytes
println!("{}", size_of::<fn(String, Vec<i32>) -> bool>());  // 8 bytes

// Compare to closures (variable size)
let x = 10;
let closure = |y| y + x;  // Size includes captured `x`
```

## 🚫 **Limitations**

### **1. No Environment Capture**

```rust
let multiplier = 10;

// ❌ Function can't capture
fn multiply(x: i32) -> i32 {
    x * multiplier  // Error: can't find value `multiplier`
}

// ✅ Closure can capture
let multiply_closure = |x| x * multiplier;  // Works
```

### **2. No Generics in Function Pointer Type**

```rust
// ❌ Can't have generic function pointer
// let generic_fn: fn<T>(T) -> T = ...;  // Not allowed

// ✅ Use trait bounds instead
fn accept_generic<F, T>(f: F, arg: T) -> T
where
    F: Fn(T) -> T
{
    f(arg)
}
```

### **3. All Closures Coerce to Trait, Not fn**

```rust
fn takes_fn_pointer(f: fn(i32) -> i32) {
    println!("{}", f(10));
}

// ✅ Regular function works
fn add_one(x: i32) -> i32 { x + 1 }
takes_fn_pointer(add_one);

// ✅ Non-capturing closure works
takes_fn_pointer(|x| x + 1);

// ❌ Capturing closure doesn't work
let y = 5;
takes_fn_pointer(|x| x + y);  // Error: closure captures environment
```

## 🎯 **When to Use Function Pointers**

### **✅ Use fn when:**

- Function doesn't need to capture environment
- You need fixed-size storage (arrays, known-size structs)
- Interop with C FFI (extern "C" fn)
- Maximum performance (no capture overhead)
- Simple callback mechanisms

### **❌ Use Fn/FnMut/FnOnce traits when:**

- Need to capture environment variables
- Working with closures (most iterator methods)
- Need flexibility (trait objects, generics)
- Don't know at compile time if captures needed

## 🔗 **Related Concepts**

### **Closure Traits**

- **[[Closures in Rust]]** - Environment-capturing anonymous functions
- **[[Fn vs FnMut vs FnOnce]]** - Closure trait hierarchy
- **[[Returning Closures]]** - Using `impl Trait` and `Box<dyn Fn>`

### **Function Types**

- **[[trait-objects]]** - Dynamic dispatch with `dyn Fn`
- **[[generic-functions]]** - Generic parameters with trait bounds
- **[[Higher-Order Functions]]** - Functions that take/return functions

### **Rust Book Integration**

- **[[rust_book/rust-book-ch20]]** - Advanced functions chapter
- **[[rust_book/rust-book-ch13]]** - Closures and iterators

## 🧪 **Practice Examples**

### **Example 1: Filter with Function Pointer**

```rust
fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

fn is_positive(n: &i32) -> bool {
    *n > 0
}

fn filter_numbers(numbers: &[i32], predicate: fn(&i32) -> bool) -> Vec<i32> {
    numbers.iter().filter(predicate).copied().collect()
}

let nums = vec![-2, -1, 0, 1, 2, 3, 4];
let evens = filter_numbers(&nums, is_even);     // [-2, 0, 2, 4]
let positives = filter_numbers(&nums, is_positive);  // [1, 2, 3, 4]
```

### **Example 2: Command Pattern**

```rust
struct Command {
    name: String,
    execute: fn(&str),
}

fn print_command(msg: &str) {
    println!("Executing: {}", msg);
}

fn log_command(msg: &str) {
    eprintln!("[LOG] {}", msg);
}

let commands = vec![
    Command { name: "print".to_string(), execute: print_command },
    Command { name: "log".to_string(), execute: log_command },
];

for cmd in commands {
    (cmd.execute)(&cmd.name);
}
```

### **Example 3: Mathematical Operations**

```rust
fn compose(f: fn(i32) -> i32, g: fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f(g(x))
}

fn square(x: i32) -> i32 { x * x }
fn add_three(x: i32) -> i32 { x + 3 }

let composed = compose(square, add_three);
println!("{}", composed(2));  // square(add_three(2)) = square(5) = 25
```

## 📚 **Key Takeaways**

1. **Function pointers are stateless** - no environment capture
2. **Fixed size (8 bytes on 64-bit)** - predictable memory layout
3. **Zero-cost abstraction** - compiles to direct calls when optimized
4. **Coercion from regular functions** - automatic conversion
5. **Use `fn` for simple callbacks** - use `Fn` traits for closures
6. **Enum constructors are function pointers** - useful with iterators

---

*Tags: #function-pointers #closures #callbacks #higher-order-functions #rust-book-ch20 #performance #zero-cost-abstraction #stateless*

*Links: [[Closures in Rust]] | [[Fn vs FnMut vs FnOnce]] | [[Returning Closures]] | [[trait-objects]] | [[rust_book/rust-book-ch20]] | [[rust-concepts-MOC]]*
