# Chapter 20: Advanced Features

## 🔗 Zettelkasten Links
- **Overview**: [[zettelkasten/rust_book/rust-book-ch20]]
- **Previous**: [[zettelkasten/rust_book/rust-book-ch19]]
- **Next**: [[zettelkasten/rust_book/rust-book-ch21]]
- **Missions**: [[mission-4]] - Unsafe pointers in linked lists | [[mission-5]] - Advanced traits for HashMap
- **Daily Study**: [[daily-study/Day29]] | [[daily-study/Day30]]
- **Book MOC**: [[rust-book]]

## 📚 Overview

Chapter 20 covers advanced Rust features that you won't use every day but are essential for specific situations. These features give you powerful tools when you need them: unsafe Rust for low-level control, advanced trait techniques for complex abstractions, type system features for precision, function pointers for flexibility, and macros for metaprogramming.

**Official Reference**: https://doc.rust-lang.org/book/ch20-00-advanced-features.html

---

## 🎯 Learning Objectives

By completing this chapter, you will understand:

1. **Unsafe Rust** - When and how to opt out of Rust's safety guarantees responsibly
2. **Advanced Traits** - Associated types, default parameters, supertraits, and the newtype pattern
3. **Advanced Types** - Type aliases, the never type, and dynamically sized types
4. **Function Pointers** - Passing functions as arguments and returning closures
5. **Macros** - Writing code that writes code at compile time

**Integration Points**: This chapter connects to:
- **[[mission-4]]** - Unsafe code in linked list implementation
- **[[mission-5]]** - Advanced trait patterns in collections
- **[[zettelkasten/unsafe-rust-guidelines]]** - Safe abstraction principles
- **[[zettelkasten/trait-associated-types]]** - Associated types vs generics
- **[[zettelkasten/rust-macros-comprehensive]]** - Macro types and usage

---

## 🗂️ Chapter Structure

### 20.1 - Unsafe Rust
### 20.2 - Advanced Traits  
### 20.3 - Advanced Types
### 20.4 - Advanced Functions and Closures
### 20.5 - Macros

---

## 🎯 Section 20.1: Unsafe Rust

**Official Definition**: Unsafe Rust is a second language hidden inside Rust that doesn't enforce memory safety guarantees at compile time. It gives you five "superpowers" for low-level programming.

**Practical Understanding**: `unsafe` doesn't turn off the borrow checker—it gives you access to operations that the compiler can't verify are safe. You take responsibility for upholding safety invariants.

### The Five Unsafe Superpowers

```rust
// 1. Dereference raw pointers
let mut num = 5;
let r1 = &raw const num;  // *const i32
let r2 = &raw mut num;    // *mut i32

unsafe {
    println!("r1: {}", *r1);
    println!("r2: {}", *r2);
}

// 2. Call unsafe functions
unsafe fn dangerous() {
    // Unsafe operations
}

unsafe {
    dangerous();
}

// 3. Access/modify mutable static variables
static mut COUNTER: u32 = 0;

unsafe fn increment() {
    unsafe {
        COUNTER += 1;
    }
}

// 4. Implement unsafe traits
unsafe trait UnsafeTrait {
    fn method(&self);
}

unsafe impl UnsafeTrait for MyType {
    fn method(&self) { /* ... */ }
}

// 5. Access union fields
union MyUnion {
    i: i32,
    f: f32,
}

let u = MyUnion { i: 42 };
unsafe {
    println!("Integer: {}", u.i);
}
```

### Raw Pointers

**Key Differences from References**:
- Can ignore borrowing rules (multiple mutable pointers allowed)
- Not guaranteed to point to valid memory
- Can be null
- No automatic cleanup

```rust
// Creating raw pointers (safe)
let mut num = 5;
let r1 = &raw const num;  // Immutable raw pointer
let r2 = &raw mut num;    // Mutable raw pointer

// From arbitrary address (unsafe to dereference)
let address = 0x012345usize;
let r = address as *const i32;

// Dereferencing requires unsafe block
unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

**When to Use**:
- FFI (Foreign Function Interface) with C
- Implementing safe abstractions over unsafe code
- Low-level systems programming

### Safe Abstractions Over Unsafe Code

**Example: Implementing `split_at_mut`**:

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

**Why This Works**:
- Borrow checker can't understand non-overlapping slices
- We know mathematically the slices don't overlap
- `unsafe` block is minimal and well-documented
- Function itself is safe to call (safe abstraction)

### Calling External Code with `extern`

```rust
// Calling C standard library
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value: {}", abs(-3));
    }
}

// Marking specific functions as safe
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    // No unsafe block needed - we marked it safe
    println!("Absolute value: {}", abs(-3));
}
```

**Calling Rust from Other Languages**:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

### Static Variables

```rust
// Immutable static (safe)
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("{}", HELLO_WORLD);  // Safe
}

// Mutable static (unsafe)
static mut COUNTER: u32 = 0;

/// SAFETY: Must only call from single thread
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: Only called from main thread
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
```

**Static vs Constants**:
- Static: Fixed memory address, can be mutable
- Const: Can duplicate data, always immutable

### Unsafe Traits

```rust
unsafe trait Foo {
    fn method(&self);
}

unsafe impl Foo for i32 {
    fn method(&self) {
        println!("i32 implementation");
    }
}
```

**When Needed**:
- Implementing `Send` or `Sync` for types with raw pointers
- Guarantees compiler can't verify

### Using Miri for Safety Checking

```bash
# Install Miri
rustup +nightly component add miri

# Run with Miri
cargo +nightly miri run
cargo +nightly miri test
```

**What Miri Detects**:
- Undefined behavior in unsafe code
- Invalid pointer dereferences
- Data races
- Use-after-free

### Safety Documentation Idioms

```rust
/// SAFETY: Caller must ensure `ptr` is valid and aligned
unsafe fn read_ptr(ptr: *const u32) -> u32 {
    unsafe {
        // SAFETY: We trust the caller's guarantee
        *ptr
    }
}
```

**Best Practices**:
1. Keep `unsafe` blocks minimal
2. Document safety requirements with `SAFETY:` comments
3. Use Miri to validate unsafe code
4. Wrap unsafe code in safe abstractions
5. Audit carefully - every unsafe block is a trust boundary

---

## 🎯 Section 20.2: Advanced Traits

### Associated Types

**Definition**: Associated types connect a type placeholder with a trait without using generics.

```rust
// Standard library Iterator with associated type
pub trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
}

// Implementation specifies concrete type
impl Iterator for Counter {
    type Item = u32;  // Concrete type for Item
    
    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

**Associated Types vs Generics**:

```rust
// With generics - can implement multiple times
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator<u32> for Counter { /* ... */ }
impl Iterator<String> for Counter { /* ... */ }  // Multiple impls

// With associated types - only one implementation
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;  // Only one choice
}
```

**When to Use**:
- **Associated types**: One clear relationship (Iterator → Item type)
- **Generics**: Multiple possible implementations needed

### Default Generic Type Parameters

```rust
// Add trait with default type parameter
trait Add<Rhs=Self> {  // Default: Rhs = Self
    type Output;
    
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Using default (adding Point + Point)
impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Overriding default (adding Millimeters + Meters)
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

**Use Cases**:
1. Extending traits without breaking existing code
2. Customization for uncommon cases

### Disambiguating Method Names

**Methods with Self**:

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    
    Pilot::fly(&person);   // "This is your captain speaking."
    Wizard::fly(&person);  // "Up!"
    person.fly();          // "*waving arms furiously*" (default)
}
```

**Fully Qualified Syntax for Associated Functions**:

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("{}", Dog::baby_name());                  // "Spot"
    println!("{}", <Dog as Animal>::baby_name());      // "puppy"
}
```

**Fully Qualified Syntax**:
```
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

### Supertraits

**Definition**: Require one trait as a bound for another trait.

```rust
use std::fmt;

// OutlinePrint requires Display (supertrait)
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();  // Can use Display's to_string
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// Must implement Display first
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Now can implement OutlinePrint
impl OutlinePrint for Point {}
```

### Newtype Pattern

**Definition**: Wrap external types to implement external traits (bypass orphan rule).

```rust
use std::fmt;

// Wrapper around Vec<String>
struct Wrapper(Vec<String>);

// Can now implement Display (external trait) for Vec (external type)
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
    ]);
    println!("w = {}", w);  // "w = [hello, world]"
}
```

**Pros**:
- Bypass orphan rule
- Type safety (different semantics)
- Hide implementation details

**Cons**:
- Need to delegate methods manually
- Or implement `Deref` for automatic method forwarding

---

## 🎯 Section 20.3: Advanced Types

### Newtype Pattern for Type Safety

```rust
// Different units with type safety
struct Millimeters(u32);
struct Meters(u32);

fn process_distance(mm: Millimeters) {
    println!("Processing {} millimeters", mm.0);
}

fn main() {
    let distance = Millimeters(1000);
    process_distance(distance);  // OK
    // process_distance(Meters(1));  // Compile error!
}
```

### Type Aliases

**Basic Usage**:

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);  // Works - same type
```

**Reducing Repetition**:

```rust
// Without alias - repetitive
fn takes_long(f: Box<dyn Fn() + Send + 'static>) { /* ... */ }
fn returns_long() -> Box<dyn Fn() + Send + 'static> { /* ... */ }

// With alias - cleaner
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long(f: Thunk) { /* ... */ }
fn returns_long() -> Thunk { /* ... */ }
```

**Common Pattern - Result Alias**:

```rust
// std::io does this
type Result<T> = std::result::Result<T, std::io::Error>;

// Shorter signatures
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}
```

### The Never Type `!`

**Definition**: Type with no values - represents functions that never return.

```rust
// Diverging function
fn bar() -> ! {
    panic!("This function never returns!");
}

fn loop_forever() -> ! {
    loop {
        println!("Running forever...");
    }
}
```

**Use in `match`**:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,  // continue has type !
};
```

**Why It Works**:
- `!` coerces to any type
- `continue` never produces a value (moves control back to loop)
- Match arms can have different types if one is `!`

**Other Uses**:

```rust
// unwrap returns T or panics (!)
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

### Dynamically Sized Types (DSTs)

**Definition**: Types whose size is only known at runtime.

```rust
// str is a DST (not &str)
// let s1: str = "Hello";  // ERROR - size unknown at compile time

// Must use behind pointer
let s1: &str = "Hello";      // OK - &str has known size (ptr + len)
let s2: Box<str> = Box::new(*"Hello");  // OK
let s3: Rc<str> = Rc::from("Hello");    // OK
```

**Trait Objects are DSTs**:

```rust
// Must use behind pointer
let draw: &dyn Draw = &Circle { /* ... */ };
let draw: Box<dyn Draw> = Box::new(Circle { /* ... */ });
```

### The `Sized` Trait

**Implicit Bound**:

```rust
// This generic function:
fn generic<T>(t: T) {
    // ...
}

// Is actually:
fn generic<T: Sized>(t: T) {
    // ...
}
```

**Relaxing the Bound**:

```rust
// Accept ?Sized types (must use behind pointer)
fn generic<T: ?Sized>(t: &T) {
    // ...
}
```

**Meaning**:
- `T: Sized` - T must have known size at compile time (default)
- `T: ?Sized` - T may or may not be Sized (opt-out of default)
- Only available for `Sized`, not other traits

---

## 🎯 Section 20.4: Advanced Functions and Closures

### Function Pointers

**Syntax**: `fn(Args) -> ReturnType`

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);  // 12
    println!("The answer is: {}", answer);
}
```

**Function Pointers vs Closures**:

```rust
// Function pointers implement all closure traits
fn takes_closure<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    f(42);
}

fn add_one(x: i32) -> i32 { x + 1 }

fn main() {
    takes_closure(add_one);         // Function pointer
    takes_closure(|x| x + 1);       // Closure
}
```

**When to Use `fn` Type**:
- Interfacing with C (no closures in C)
- When you specifically don't want to accept closures

**Practical Examples**:

```rust
// Using closure
let list_of_strings: Vec<String> = 
    vec![1, 2, 3]
        .iter()
        .map(|i| i.to_string())
        .collect();

// Using function pointer
let list_of_strings: Vec<String> = 
    vec![1, 2, 3]
        .iter()
        .map(ToString::to_string)
        .collect();

// Enum initializers are function pointers
enum Status {
    Value(u32),
    Stop,
}

let statuses: Vec<Status> = 
    (0u32..20)
        .map(Status::Value)  // Function pointer!
        .collect();
```

### Returning Closures

**Using `impl Trait`**:

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

**When You Need Trait Objects** (multiple closure types):

```rust
// ERROR - different opaque types
fn returns_closure() -> impl Fn(i32) -> i32 {
    if condition {
        |x| x + 1
    } else {
        |x| x + 2  // Different closure type!
    }
}

// FIX - use trait object
fn returns_closure(condition: bool) -> Box<dyn Fn(i32) -> i32> {
    if condition {
        Box::new(|x| x + 1)
    } else {
        Box::new(|x| x + 2)
    }
}
```

**Multiple Closure Return Types**:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

fn main() {
    let handlers = vec![
        returns_closure(),
        returns_initialized_closure(123),
    ];
    
    for handler in handlers {
        println!("{}", handler(5));
    }
}
```

---

## 🎯 Section 20.5: Macros

### Macros vs Functions

**Macros**:
- Variable number of arguments
- Expand before compilation
- Can implement traits (compile-time)
- More complex to write
- Must define before use in file

**Functions**:
- Fixed number of typed parameters
- Called at runtime
- Can be defined anywhere

### Declarative Macros (`macro_rules!`)

**Basic Structure**:

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

**Pattern Breakdown**:
- `$( ... ),*` - Match zero or more comma-separated items
- `$x:expr` - Match Rust expression, bind to `$x`
- `$()*` - Repeat code for each matched item

**Expansion Example**:

```rust
// vec![1, 2, 3] expands to:
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

### Procedural Macros

**Three Types**:
1. **Custom `#[derive]`** - Generate trait implementations
2. **Attribute-like** - Define custom attributes
3. **Function-like** - Look like functions but operate on tokens

**Basic Structure**:

```rust
use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // Transform input tokens to output tokens
}
```

### Custom Derive Macros

**Goal**: Auto-implement traits

**Setup**:

```bash
# Main crate
cargo new hello_macro --lib

# Derive macro crate
cd hello_macro
cargo new hello_macro_derive --lib
```

**Trait Definition** (`hello_macro/src/lib.rs`):

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

**Derive Implementation** (`hello_macro_derive/Cargo.toml`):

```toml
[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
```

**Derive Code** (`hello_macro_derive/src/lib.rs`):

```rust
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
```

**Usage**:

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();  // "Hello, Macro! My name is Pancakes!"
}
```

### Attribute-Like Macros

**Usage**:

```rust
#[route(GET, "/")]
fn index() {
    // Web framework route handler
}
```

**Definition**:

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // attr = 'GET, "/"'
    // item = 'fn index() { }'
}
```

### Function-Like Macros

**Usage**:

```rust
let sql = sql!(SELECT * FROM posts WHERE id = 1);
```

**Definition**:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // Parse SQL and generate Rust code
}
```

---

## 📝 Practice Exercises

### Exercise 1: Safe Unsafe Abstraction

Create a safe wrapper around unsafe pointer arithmetic:

```rust
// TODO: Implement this safely
pub struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
}

impl SafeBuffer {
    pub fn new(len: usize) -> Self {
        // HINT: Use Vec::into_raw_parts or similar
        todo!()
    }
    
    pub fn get(&self, index: usize) -> Option<u8> {
        // HINT: Bounds check before unsafe dereference
        todo!()
    }
    
    pub fn set(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        // HINT: Bounds check before unsafe write
        todo!()
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        // HINT: Reconstruct Vec to drop properly
        todo!()
    }
}
```

### Exercise 2: Associated Types vs Generics

Design a trait system for graph algorithms:

```rust
// TODO: Choose between associated types and generics
trait Graph {
    type Node;
    type Edge;
    
    fn nodes(&self) -> Vec<Self::Node>;
    fn edges(&self, node: &Self::Node) -> Vec<Self::Edge>;
}

// OR

trait Graph<N, E> {
    fn nodes(&self) -> Vec<N>;
    fn edges(&self, node: &N) -> Vec<E>;
}

// Which is better and why?
```

### Exercise 3: Macro Creation

Create a `hashmap!` macro similar to `vec!`:

```rust
#[macro_export]
macro_rules! hashmap {
    // TODO: Implement pattern matching for key => value pairs
    // HINT: Use $( $key:expr => $value:expr ),*
}

// Should work like:
let map = hashmap! {
    "key1" => "value1",
    "key2" => "value2",
};
```

---

## 🔗 Integration with Missions

### Mission 4: Linked Lists with Unsafe Code

```rust
// Interior mutability with raw pointers
struct Node<T> {
    data: T,
    next: *mut Node<T>,  // Raw pointer for flexibility
    prev: *mut Node<T>,
}

impl<T> DoublyLinkedList<T> {
    unsafe fn link_nodes(&mut self, prev: *mut Node<T>, next: *mut Node<T>) {
        (*prev).next = next;
        (*next).prev = prev;
    }
}
```

### Mission 5: Advanced HashMap Traits

```rust
// Associated type for key-value relationship
trait Map {
    type Key;
    type Value;
    
    fn insert(&mut self, key: Self::Key, value: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

// Default type parameter for hasher
impl<K, V, H = DefaultHasher> HashMap<K, V, H>
where
    H: Hasher,
{
    // ...
}
```

---

## 🧪 Testing Examples

```bash
# Run all chapter examples
cd rust_book/Ch20/examples
cargo run --example ch20_1_unsafe_rust
cargo run --example ch20_2_advanced_traits
cargo run --example ch20_3_advanced_types
cargo run --example ch20_4_functions_closures
cargo run --example ch20_5_macros

# Test with Miri
cargo +nightly miri test

# Run with all features
cargo test --all-features
```

---

## 🎓 Key Takeaways

1. **Unsafe Rust**:
   - Only use when necessary (FFI, safe abstractions, low-level)
   - Keep unsafe blocks minimal
   - Document safety invariants with `SAFETY:` comments
   - Validate with Miri

2. **Advanced Traits**:
   - Associated types for one-to-one relationships
   - Default type parameters for customization
   - Supertraits for trait dependencies
   - Newtype pattern to bypass orphan rule

3. **Advanced Types**:
   - Type aliases reduce repetition
   - `!` enables control flow that never returns
   - DSTs must live behind pointers
   - `?Sized` opt-out for generic flexibility

4. **Functions & Closures**:
   - Function pointers for simple cases
   - `impl Trait` for single closure returns
   - `Box<dyn Fn>` for multiple closure types

5. **Macros**:
   - Declarative macros for pattern matching code
   - Procedural macros for code generation
   - Choose based on complexity and needs

---

## 📚 Additional Resources

- **Rustonomicon**: https://doc.rust-lang.org/nomicon/ (Unsafe Rust guide)
- **The Little Book of Rust Macros**: https://veykril.github.io/tlborm/
- **Syn Documentation**: https://docs.rs/syn/ (Parsing Rust code)
- **Quote Documentation**: https://docs.rs/quote/ (Generating Rust code)
- **Miri**: https://github.com/rust-lang/miri (Undefined behavior detector)

---

## 🎯 Next Steps

- [ ] Complete all practice exercises
- [ ] Run examples with Miri to understand unsafe behavior
- [ ] Create zettelkasten notes for key concepts
- [ ] Apply to Mission 4 (unsafe pointers) and Mission 5 (advanced traits)
- [ ] Move to Chapter 21 for final project
