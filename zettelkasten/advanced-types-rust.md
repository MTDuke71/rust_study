# Advanced Types in Rust

Rust's type system includes several advanced features that enable powerful patterns and compile-time guarantees beyond basic types. These features provide flexibility while maintaining Rust's safety guarantees.

## Type Aliases

**Type aliases** create alternative names for existing types, reducing repetition and improving code clarity.

### Basic Syntax
```rust
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

let distance: Kilometers = 100;  // Actually i32
let callback: Thunk = Box::new(|| println!("Called!"));
```

### Common Use Cases

**1. Simplifying Complex Types**
```rust
// Without alias - verbose and error-prone
let f: Box<dyn Fn() + Send + 'static> = /* ... */;

// With alias - clean and maintainable
type Thunk = Box<dyn Fn() + Send + 'static>;
let f: Thunk = /* ... */;
```

**2. Generic Type Parameters**
```rust
// Reduce repetition in generic contexts
type Result<T> = std::result::Result<T, std::io::Error>;

fn read_file() -> Result<String> { /* ... */ }
```

**Key Insight**: Type aliases are **transparent** - `Kilometers` and `i32` are completely interchangeable. The compiler treats them as the same type.

---

## Newtype Pattern

The **newtype pattern** uses tuple structs with a single field to create distinct types, enforcing type safety at compile time.

### Syntax
```rust
struct Kilometers(i32);
struct Miles(i32);

let km = Kilometers(100);
let mi = Miles(62);
// km and mi are DIFFERENT types - cannot mix!
```

### When to Use Newtype

**1. Type Safety / Preventing Confusion**
```rust
struct UserId(u64);
struct OrderId(u64);

fn process_order(order_id: OrderId) { /* ... */ }

let user = UserId(42);
let order = OrderId(100);
// process_order(user);  // ❌ Compile error! UserId ≠ OrderId
process_order(order);    // ✅ Type-safe
```

**2. Trait Implementation on External Types**
```rust
// Can't implement Display for Vec<T> directly (orphan rule)
struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

**3. Abstraction / Encapsulation**
```rust
pub struct Meters(f64);

impl Meters {
    pub fn new(value: f64) -> Self {
        assert!(value >= 0.0, "Meters must be non-negative");
        Meters(value)
    }
    
    pub fn value(&self) -> f64 { self.0 }
}
// Internal representation (f64) is hidden
```

### Type Alias vs Newtype

| Feature | Type Alias | Newtype |
|---------|-----------|---------|
| Type safety | ❌ Same type as original | ✅ Distinct type |
| Zero-cost | ✅ No runtime overhead | ✅ Zero-cost abstraction |
| Trait implementation | ❌ Can't add traits | ✅ Can implement traits |
| Encapsulation | ❌ Transparent | ✅ Can hide internals |

---

## Never Type (`!`)

The **never type** (`!`) represents values that never exist - functions that never return to their caller.

### The Dual Meaning of `!`

**Important**: The `!` symbol has **two different meanings** in Rust:

1. **Macro invocation** (suffix): `println!()`, `panic!()`, `vec![]`
2. **Never type** (standalone): `fn diverge() -> !`

Example showing both:
```rust
panic!("error")
//    ^ The `!` suffix means it's a MACRO (not a function)
//    The macro expands to code with TYPE `!` (never returns)
```

### Functions that Diverge
```rust
fn infinite_loop() -> ! {
    loop {
        // Never exits
    }
}

fn always_panics() -> ! {
    panic!("This function never returns normally")
}

fn exits_process() -> ! {
    std::process::exit(1)
}
```

### Coercion to Any Type

The never type can **coerce to any other type** because it never actually produces a value:

```rust
let x: i32 = if condition {
    42
} else {
    panic!("Failed!")  // Type `!` coerces to `i32`
};

let guess: u32 = match input.parse() {
    Ok(num) => num,           // Type: u32
    Err(_) => continue,       // Type: !, coerces to u32
};
```

### Common Diverging Expressions

All of these have type `!`:
- `panic!("error")` - aborts or unwinds program
- `return` - returns from function
- `continue` - jumps to next loop iteration
- `break` (without value) - exits loop
- `loop { }` - infinite loop without breaks
- `std::process::exit(1)` - terminates process
- `unreachable!()` - indicates unreachable code
- `todo!()` - placeholder for unimplemented code

### Why `!` Was Chosen

The choice of `!` for the never type is deliberate:

1. **Type Theory Precedent**: Represents the "bottom type" (⊥) - an uninhabited type with no values
2. **Visual "Stop" Signal**: Exclamation mark naturally conveys urgency, finality, and termination
3. **Already Associated with Divergence**: Macros that never return (`panic!`, `unreachable!`, `todo!`) use `!`
4. **Symmetry with Boolean NOT**: 
   - `!` operator: negation (not true/false)
   - `!` type: negation of return (doesn't return)
5. **Conciseness**: Single character, ASCII-safe
6. **Distinction from Unit**: `()` returns nothing useful, `!` never returns at all

**The Empty Set Concept**: The never type represents the **empty set of values** - no value of type `!` can actually exist. This is why it coerces to any type: it never needs to provide a value, so the compiler can safely treat it as any type.

---

## Dynamically Sized Types (DSTs)

**Dynamically Sized Types** have sizes that are only known at runtime, not compile time.

### Examples of DSTs
```rust
str      // String slice (not String)
[T]      // Slice (not Vec<T> or array [T; N])
dyn Trait // Trait objects
```

### The Golden Rule: DSTs Must Live Behind Pointers

DSTs cannot exist on the stack directly because Rust needs to know stack frame sizes at compile time:

```rust
// ❌ Cannot do this - size unknown at compile time
let s: str = "hello";
let slice: [i32] = [1, 2, 3];

// ✅ Must use behind pointers
let s: &str = "hello";              // Reference
let s: Box<str> = Box::from("hi"); // Box (heap)
let s: Rc<str> = Rc::from("yo");   // Reference counted
```

### Working with DSTs

**String Slices**
```rust
fn print_str(s: &str) {  // &str is sized (pointer + length)
    println!("{}", s);
}

let string = String::from("hello");
print_str(&string);      // String coerces to &str
print_str("world");      // String literal is &str
```

**Slice Parameters**
```rust
fn sum_slice(values: &[i32]) -> i32 {  // &[i32] is sized
    values.iter().sum()
}

sum_slice(&[1, 2, 3]);        // Array coerces to slice
sum_slice(&vec![4, 5, 6]);    // Vec coerces to slice
```

**Trait Objects**
```rust
trait Draw {
    fn draw(&self);
}

// ❌ Cannot do this - size unknown
// let drawable: dyn Draw = /* ... */;

// ✅ Must use pointers
let drawable: Box<dyn Draw> = Box::new(button);
let drawable: &dyn Draw = &circle;
```

### The `Sized` Trait

The `Sized` trait is a special compiler-known trait that marks types with known size at compile time.

**Automatic Bound**
```rust
// These are equivalent:
fn generic<T>(t: T) { }
fn generic<T: Sized>(t: T) { }  // Sized is implicit!
```

**Relaxing with `?Sized`**

To work with DSTs in generics, use `?Sized` (special syntax meaning "might not be Sized"):

```rust
fn print_anything<T: ?Sized + Display>(value: &T) {
    //                ^^^^^^^
    //                T might be unsized (DST)
    println!("{}", value);
}

print_anything("hello");           // &str (DST)
print_anything(&42);               // &i32 (sized)
print_anything(&String::from("hi")); // &String (sized)
```

**Why `&T` when T is `?Sized`**: Since `T` might be unsized, we can't put it directly on the stack. We must always take it by reference `&T` (which IS sized - pointer + metadata).

### DST Memory Layout

**Sized Types**
```
Stack: [value data]  // Fixed size known at compile time
```

**DSTs Behind Pointers (Fat Pointers)**
```
Stack: [pointer | metadata]  // Fixed size (2 words)
  ↓
Heap/Data: [actual data...]   // Variable size
```

Metadata varies by DST type:
- `&str`: pointer + **length**
- `&[T]`: pointer + **length**
- `&dyn Trait`: pointer + **vtable pointer**

---

## Practical Comparison

### When to Use Each

| Feature | Use Case | Example |
|---------|----------|---------|
| **Type Alias** | Reduce repetition, improve clarity | `type Result<T> = std::result::Result<T, Error>;` |
| **Newtype** | Type safety, trait implementation | `struct UserId(u64);` prevents mixing with `OrderId(u64)` |
| **Never Type** | Diverging functions, type coercion | `fn server_loop() -> !` for infinite server |
| **DST** | Generic code over slices/str/traits | `fn process(items: &[T])` works for arrays, Vecs, slices |

---

## Code Examples

### Complete Type Alias Example
```rust
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 10;
    println!("Sum: {}", x + y);  // Works - same type!
    
    let callback: Thunk = Box::new(|| {
        println!("Callback executed!");
    });
    callback();
}
```

### Complete Newtype Example
```rust
struct Meters(f64);
struct Seconds(f64);

impl Meters {
    fn new(value: f64) -> Self {
        assert!(value >= 0.0);
        Meters(value)
    }
}

fn calculate_speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0  // Access inner value with .0
}

fn main() {
    let d = Meters::new(100.0);
    let t = Seconds(10.0);
    // calculate_speed(t, d);  // ❌ Type error! Prevents bugs
    let speed = calculate_speed(d, t);  // ✅ Type-safe
}
```

### Complete Never Type Example
```rust
fn diverges() -> ! {
    panic!("This function never returns")
}

fn server() -> ! {
    loop {
        handle_request();
    }
}

fn process_input(input: &str) -> u32 {
    match input.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input!");
            return 0;  // Type !, coerces to u32
        }
    }
}
```

### Complete DST Example
```rust
// Generic function working with DSTs
fn print_display<T: ?Sized + Display>(value: &T) {
    println!("Value: {}", value);
}

fn process_items<T: Display>(items: &[T]) {  // &[T] is DST
    for item in items {
        print_display(item);
    }
}

fn main() {
    // str is DST
    let s1: &str = "Hello";
    let s2: Box<str> = Box::from("World");
    
    // [T] is DST
    let arr = [1, 2, 3];
    let vec = vec![4, 5, 6];
    process_items(&arr);   // Array coerces to slice
    process_items(&vec);   // Vec coerces to slice
    
    // Works with DSTs
    print_display("string slice");
    print_display(&42);
}
```

---

## Integration with Learning

**Rust Book**: Chapter 20.3 - Advanced Types
**Related Concepts**: 
- [[rust-book-ch20]] - Advanced Features overview
- [[trait-objects-rust]] - Dynamic dispatch with DSTs
- [[ownership-borrowing]] - Why DSTs need pointers
- [[zero-cost-abstractions]] - Newtype pattern runtime cost

**AoC Application**:
- Type aliases used throughout for Result types with anyhow
- Never type in diverging parsers or error handling
- DSTs in generic functions accepting slices (`&[T]`)
- Newtype pattern could improve type safety in grid coordinates

**Real-World Usage**:
- Type aliases: Database connection pools, callback types
- Newtype: Strong typing for IDs, measurements, validated strings
- Never type: Server main loops, signal handlers, CLI error paths
- DSTs: Library APIs accepting flexible input (str, slices, trait objects)

---

## Key Takeaways

1. **Type aliases** are transparent - reduce typing, not type safety
2. **Newtype pattern** creates distinct types - compile-time safety with zero runtime cost
3. **Never type (`!`)** represents divergence - dual meaning as macro indicator and type
4. **DSTs** are flexible but must live behind pointers - `&str`, `&[T]`, `&dyn Trait`
5. **`?Sized`** relaxes the implicit `Sized` bound for generic DST support
6. All these features maintain Rust's zero-cost abstraction guarantee

---

*Created*: 2025-12-12  
*Source*: Rust Book Chapter 20.3 + hands-on examples  
*Status*: Complete - covers type aliases, newtype, never type, DSTs

*Tags*: #rust #advanced-types #type-system #dst #never-type #newtype #zero-cost

*Links*: [[rust-book-ch20]] | [[2025-12-12]] | [[trait-objects-rust]] | [[ownership-borrowing]] | [[zero-cost-abstractions]] | [[../../rust_book/Ch20/examples/ch20_3_advanced_types.rs]]
