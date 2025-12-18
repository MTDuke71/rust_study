# Rust Book Chapters 19-22: Advanced Patterns, Features, and Project

> **Knowledge Integration**: Mastering pattern matching, unsafe Rust, advanced type system features, and building real-world concurrent applications

---

## 📚 Overview

This review covers the final chapters of The Rust Programming Language book, focusing on advanced language features and practical application. These chapters provide deep dives into Rust's pattern matching system, low-level capabilities, advanced type system features, and culminate in a real-world multithreaded web server project.

**Chapter Coverage:**
- **Chapter 19**: Patterns and Matching - Comprehensive pattern syntax and refutability
- **Chapter 20**: Advanced Features - Unsafe Rust, advanced traits, types, and macros
- **Chapter 21**: Final Project - Building a multithreaded web server with thread pools
- **Chapter 22**: Appendix - Reference material, tools, and Rust ecosystem

**Cross-References:**
- [[rust-book-ch16-18-review]] - Foundation: Concurrency patterns used in Ch21 project
- [[zettelkasten/rust_book/rust-book-ch19]] - Pattern matching deep dive
- [[zettelkasten/rust_book/rust-book-ch20]] - Advanced features comprehensive guide
- [[zettelkasten/rust_book/rust-book-ch21]] - Web server implementation
- [[zettelkasten/pattern-syntax-comprehensive]] - Complete pattern reference

---

## 🎯 Chapter 19: Patterns and Matching

### **Core Philosophy**

Patterns are a special syntax for matching against the structure of types. They make code more expressive and safer by allowing you to destructure complex data, handle variants explicitly, and leverage the compiler's exhaustiveness checking. Patterns aren't just for `match` - they're a fundamental part of Rust's syntax.

**Key Principle**: Patterns express intent clearly and let the compiler verify completeness.

---

### **19.1 - All the Places Patterns Can Be Used**

#### **Six Pattern Locations**

Patterns appear in more places than you might think:

```rust
// 1. match arms - most visible use
match value {
    Some(x) => println!("Got {}", x),
    None => println!("Got nothing"),
}

// 2. if let - conditional pattern matching
if let Some(color) = favorite_color {
    println!("Using color: {}", color);
}

// 3. while let - loop while pattern matches
while let Some(top) = stack.pop() {
    println!("{}", top);
}

// 4. for loops - destructuring iteration
for (index, value) in v.iter().enumerate() {
    println!("{}: {}", index, value);
}

// 5. let statements - always use patterns!
let (x, y, z) = (1, 2, 3);
let Point { x, y } = point;

// 6. Function parameters - patterns in signatures
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("({}, {})", x, y);
}
```

**Key Insight**: `let x = 5` uses a pattern - the variable name `x` is a simple irrefutable pattern.

#### **Choosing the Right Pattern Location**

| Location | Refutability | Use When |
|----------|-------------|----------|
| `match` arms | Refutable (except last) | Multiple possibilities, need exhaustiveness |
| `if let` | Refutable | One case matters, else optional |
| `while let` | Refutable | Loop while pattern succeeds |
| `for` | Irrefutable | Iterate with destructuring |
| `let` | Irrefutable | Always matches, destructure |
| Function params | Irrefutable | Destructure input |

---

### **19.2 - Refutability: Whether a Pattern Might Fail to Match**

#### **Irrefutable Patterns - Always Match**

```rust
// These ALWAYS match - compiler knows they can't fail
let x = 5;                           // Simple binding
let (a, b) = (1, 2);                // Tuple with known size
let Point { x, y } = point;         // Struct destructuring

// Function parameters must be irrefutable
fn process((x, y): (i32, i32)) {    // Always matches tuple
    println!("{}, {}", x, y);
}

// for loops use irrefutable patterns
for (key, value) in map {           // Always matches each pair
    println!("{}: {}", key, value);
}
```

#### **Refutable Patterns - Might Fail**

```rust
// These MIGHT fail to match
if let Some(x) = optional_value {   // Might be None
    println!("Got {}", x);
}

while let Some(top) = stack.pop() { // Might be empty
    println!("{}", top);
}

// match arms (except exhaustive catch-all)
match value {
    Some(n) if n > 0 => println!("Positive"),  // Refutable
    Some(n) => println!("Non-positive"),        // Refutable
    None => println!("Nothing"),                // Makes it exhaustive
}
```

#### **Common Compiler Errors**

```rust
// ❌ Refutable pattern in irrefutable position
// let Some(x) = some_option;  // Compile error!

// ✅ Use if let instead
if let Some(x) = some_option {
    println!("{}", x);
}

// ❌ Irrefutable pattern warning in match
// match value {
//     x => println!("{}", x),  // Should be if let
// }

// ✅ Match with multiple arms or use if let
if value > 0 {
    println!("{}", value);
}
```

**Rule of Thumb**:
- If it could fail → use `if let`, `while let`, or `match`
- If it always succeeds → use `let`, `for`, or function parameters

---

### **19.3 - Pattern Syntax**

Rust's pattern syntax is incredibly rich and expressive:

#### **Matching Literals**

```rust
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything else"),
}
```

#### **Multiple Patterns with `|`**

```rust
match x {
    1 | 2 => println!("one or two"),
    3 | 4 | 5 => println!("three through five"),
    _ => println!("something else"),
}
```

#### **Matching Ranges with `..=`**

```rust
match x {
    1..=5 => println!("one through five"),
    'a'..='j' => println!("early ASCII letter"),
    _ => println!("something else"),
}
```

#### **Destructuring Structs**

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

// Full destructuring
let Point { x, y } = p;

// Shorthand when variable names match
let Point { x, y } = p;  // Creates x and y variables

// Literal matching with destructuring
match p {
    Point { x: 0, y } => println!("On y-axis at {}", y),
    Point { x, y: 0 } => println!("On x-axis at {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}

// Partial destructuring with ..
let Point { x, .. } = p;  // Ignore y
```

#### **Destructuring Enums**

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Text: {}", text),
    Message::ChangeColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
}
```

#### **Nested Destructuring**

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    ChangeColor(Color),
}

match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => {
        println!("RGB: ({}, {}, {})", r, g, b);
    }
    Message::ChangeColor(Color::Hsv(h, s, v)) => {
        println!("HSV: ({}, {}, {})", h, s, v);
    }
}
```

#### **Ignoring Values**

```rust
// Ignore entire value with _
fn foo(_: i32, y: i32) {
    println!("y: {}", y);
}

// Ignore parts of tuple
let (x, _, z) = (1, 2, 3);

// Ignore remaining struct fields with ..
let Point { x, .. } = point;

// Suppress unused variable warning
let _unused = expensive_computation();

// Ignore rest of array/slice
match numbers {
    [first, .., last] => println!("First: {}, Last: {}", first, last),
}
```

#### **Match Guards - Extra Conditions**

```rust
match num {
    Some(x) if x % 2 == 0 => println!("Even: {}", x),
    Some(x) => println!("Odd: {}", x),
    None => println!("None"),
}

// Match guard applies to all patterns with |
match x {
    4 | 5 | 6 if y => println!("yes"),  // Guard applies to 4, 5, AND 6
    _ => println!("no"),
}
```

#### **@ Bindings - Bind AND Test**

```rust
enum Message {
    Hello { id: i32 },
}

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,  // Bind to id_variable AND test range
    } => println!("Found id in range: {}", id_variable),

    Message::Hello { id: 10..=12 } => {
        println!("Found id in range 10-12 (but can't use value)");
    }

    Message::Hello { id } => println!("Other id: {}", id),
}
```

**Pattern Syntax Summary:**

| Pattern | Syntax | Example |
|---------|--------|---------|
| Literal | `1`, `'a'`, `true` | `match x { 1 => ... }` |
| Variable | `x`, `name` | `let x = 5;` |
| Wildcard | `_` | `_ => println!("default")` |
| Range | `1..=5`, `'a'..='z'` | `1..=100 => println!("1-100")` |
| Multiple | `\|` | `1 \| 2 \| 3 => ...` |
| Struct | `Point { x, y }` | `Point { x: 0, y } => ...` |
| Tuple | `(x, y, z)` | `let (a, b, c) = tuple;` |
| Enum | `Some(x)`, `None` | `Some(v) => v` |
| Rest | `..` | `Point { x, .. }` |
| Guard | `if condition` | `Some(x) if x > 5 => ...` |
| Binding | `@` | `id @ 3..=7 => ...` |

---

## ⚙️ Chapter 20: Advanced Features

### **Core Philosophy**

These advanced features give you precise control when you need it. Unsafe Rust provides low-level access, advanced traits enable sophisticated abstractions, the type system offers tools for precise modeling, and macros enable compile-time code generation. Use these features judiciously - they're powerful but add complexity.

**Key Principle**: Reach for advanced features when simple Rust isn't sufficient, not by default.

---

### **20.1 - Unsafe Rust**

#### **The Five Unsafe Superpowers**

Unsafe doesn't turn off the borrow checker - it grants five additional capabilities:

```rust
// 1. Dereference raw pointers
let mut num = 5;
let r1 = &raw const num;  // *const i32
let r2 = &raw mut num;    // *mut i32

unsafe {
    println!("r1: {}", *r1);  // Unsafe dereference
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

unsafe {
    COUNTER += 1;
    println!("COUNTER: {}", COUNTER);
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

#### **Raw Pointers vs References**

**Differences:**
- Can ignore borrowing rules (multiple mutable pointers allowed)
- Not guaranteed to point to valid memory
- Can be null
- No automatic cleanup

```rust
let mut num = 5;

// Creating raw pointers is safe
let r1 = &raw const num;
let r2 = &raw mut num;

// From arbitrary address
let address = 0x012345usize;
let r = address as *const i32;

// Dereferencing requires unsafe
unsafe {
    println!("r1: {}", *r1);  // Only unsafe to dereference
}
```

#### **Safe Abstractions Over Unsafe Code**

**Example**: Implementing `split_at_mut` safely

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);  // Safety check

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

**Why This Works:**
- Borrow checker can't verify non-overlapping slices
- We verify mathematically they don't overlap
- `unsafe` block is minimal and well-documented
- Function itself is safe to call

#### **FFI - Foreign Function Interface**

```rust
// Calling C functions
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value: {}", abs(-3));
    }
}

// Exposing Rust to C
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Called from C!");
}
```

#### **Unsafe Best Practices**

```rust
/// SAFETY: Caller must ensure `ptr` is valid and aligned
unsafe fn read_ptr(ptr: *const u32) -> u32 {
    unsafe {
        // SAFETY: We trust the caller's guarantee
        *ptr
    }
}
```

**Guidelines:**
1. Keep `unsafe` blocks minimal
2. Document safety requirements with `SAFETY:` comments
3. Use Miri to validate unsafe code (`cargo +nightly miri test`)
4. Wrap unsafe code in safe abstractions
5. Audit carefully - every unsafe block is a trust boundary

---

### **20.2 - Advanced Traits**

#### **Associated Types**

Connect a type placeholder with a trait without generics:

```rust
// Standard library Iterator
pub trait Iterator {
    type Item;  // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

// Implementation specifies concrete type
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

**Associated Types vs Generics:**

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

**When to Use:**
- **Associated types**: One-to-one relationship (Iterator has one Item type)
- **Generics**: Multiple implementations needed

#### **Default Generic Type Parameters**

```rust
// Default type parameter
trait Add<Rhs=Self> {  // Default: Rhs = Self
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

// Using default (Point + Point)
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Overriding default (Millimeters + Meters)
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

**Use Cases:**
1. Extending traits without breaking existing code
2. Customization for uncommon cases

#### **Disambiguating Method Names**

**Fully Qualified Syntax:**

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
    println!("{}", Dog::baby_name());              // "Spot"
    println!("{}", <Dog as Animal>::baby_name());  // "puppy"
}
```

**Syntax**: `<Type as Trait>::function(receiver_if_method, next_arg, ...)`

#### **Supertraits**

Require one trait as a bound for another:

```rust
use std::fmt;

// OutlinePrint requires Display
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
```

#### **Newtype Pattern**

Wrap external types to implement external traits (bypass orphan rule):

```rust
use std::fmt;

// Wrapper around Vec<String>
struct Wrapper(Vec<String>);

// Can now implement Display for Vec (external type)
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);  // "w = [hello, world]"
}
```

---

### **20.3 - Advanced Types**

#### **Type Aliases**

```rust
// Reduce repetition
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long(f: Thunk) { /* ... */ }
fn returns_long() -> Thunk { /* ... */ }

// Common pattern - Result alias
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}
```

#### **The Never Type `!`**

Type with no values - represents functions that never return:

```rust
fn bar() -> ! {
    panic!("This function never returns!");
}

fn loop_forever() -> ! {
    loop {
        println!("Running forever...");
    }
}

// Use in match - ! coerces to any type
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,  // continue has type !
};
```

#### **Dynamically Sized Types (DSTs)**

Types whose size is only known at runtime:

```rust
// str is a DST (not &str)
// let s1: str = "Hello";  // ERROR

// Must use behind pointer
let s1: &str = "Hello";      // OK - &str has known size
let s2: Box<str> = Box::from("Hello");
let s3: Rc<str> = Rc::from("Hello");

// Trait objects are DSTs
let draw: &dyn Draw = &Circle { /* ... */ };
```

#### **The `Sized` Trait**

```rust
// Implicit bound - T must have known size
fn generic<T>(t: T) {
    // Actually: fn generic<T: Sized>(t: T)
}

// Relax the bound - T may be unsized (must use behind pointer)
fn generic<T: ?Sized>(t: &T) {
    // T may or may not be Sized
}
```

---

### **20.4 - Advanced Functions and Closures**

#### **Function Pointers**

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);  // 12
}
```

**Function Pointers vs Closures:**

Function pointers implement all closure traits (`Fn`, `FnMut`, `FnOnce`):

```rust
fn takes_closure<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    f(42);
}

takes_closure(add_one);      // Function pointer
takes_closure(|x| x + 1);    // Closure
```

**Practical Examples:**

```rust
// Using method as function pointer
let list_of_strings: Vec<String> =
    (0..10)
        .map(ToString::to_string)  // Method reference
        .collect();

// Enum constructors are function pointers
let statuses: Vec<Status> =
    (0u32..20)
        .map(Status::Value)  // Constructor as fn pointer
        .collect();
```

#### **Returning Closures**

```rust
// Single closure type - use impl Trait
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// Multiple closure types - use trait object
fn returns_closure(condition: bool) -> Box<dyn Fn(i32) -> i32> {
    if condition {
        Box::new(|x| x + 1)
    } else {
        Box::new(|x| x + 2)
    }
}
```

---

### **20.5 - Macros**

#### **Macros vs Functions**

| Feature | Macros | Functions |
|---------|--------|-----------|
| Arguments | Variable number | Fixed number |
| Execution | Compile-time expansion | Runtime call |
| Capabilities | Generate code, implement traits | Execute logic |
| Complexity | More complex | Simpler |
| Definition order | Before use | Anywhere |

#### **Declarative Macros (`macro_rules!`)**

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

// vec![1, 2, 3] expands to:
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

**Pattern Syntax:**
- `$( ... ),*` - Match zero or more comma-separated items
- `$x:expr` - Match expression, bind to `$x`
- `$()*` - Repeat code for each matched item

#### **Procedural Macros**

Three types:

1. **Custom `#[derive]`** - Generate trait implementations
2. **Attribute-like** - Define custom attributes
3. **Function-like** - Look like functions but operate on tokens

**Custom Derive Example:**

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

**Usage:**

```rust
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();  // "Hello, Macro! My name is Pancakes!"
}
```

---

## 🌐 Chapter 21: Final Project - Multithreaded Web Server

### **Core Philosophy**

This project synthesizes concepts from the entire book into a real-world application. You'll build a TCP-based web server that evolves from single-threaded (simple but slow) to multithreaded with a thread pool (production-ready), demonstrating practical application of Rust's ownership, concurrency, and safety features.

**Key Principle**: Start simple, identify problems, apply appropriate solutions.

---

### **21.1 - Building a Single-Threaded Web Server**

#### **TCP Listener Basics**

```rust
use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\nHello!";
    stream.write_all(response.as_bytes()).unwrap();
}
```

#### **HTTP Request Format**

```
GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0
```

- **Request line**: `METHOD PATH HTTP_VERSION`
- **Headers**: Key-value pairs
- **Blank line**: Separator
- **Body**: (optional)

#### **HTTP Response Format**

```
HTTP/1.1 200 OK
Content-Length: 140

<!DOCTYPE html>
<html>...</html>
```

#### **The Blocking Problem**

```rust
// Simulated slow request
if request_line == "GET /sleep HTTP/1.1" {
    thread::sleep(Duration::from_secs(5));
    // Blocks ALL other requests!
}
```

**Issue**: Single-threaded server processes requests sequentially. One slow request blocks all others.

---

### **21.2 - Turning Our Single-Threaded Server into a Multithreaded Server**

#### **Thread Pool Pattern**

Better than spawning unlimited threads (resource exhaustion):

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
```

#### **Worker Pattern**

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");
            job();
        });

        Worker { id, thread: Some(thread) }
    }
}
```

**Key Points:**
- `Arc<Mutex<Receiver>>` - Shared ownership + safe concurrent access
- Each worker locks mutex to receive job
- Only one worker gets each job
- Loop keeps threads alive

#### **Using the Thread Pool**

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

**Improvement**: Multiple requests handled concurrently!

---

### **21.3 - Graceful Shutdown and Cleanup**

#### **Termination Message Pattern**

```rust
enum Message {
    NewJob(Job),
    Terminate,
}

// Updated Worker loop
loop {
    let message = receiver.lock().unwrap().recv().unwrap();

    match message {
        Message::NewJob(job) => {
            println!("Worker {id} got a job; executing.");
            job();
        }
        Message::Terminate => {
            println!("Worker {id} terminating.");
            break;
        }
    }
}
```

#### **Drop Trait for Cleanup**

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // Send termination to all workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // Join all threads
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

**Pattern:**
- Drop automatically called when ThreadPool goes out of scope
- Send termination messages
- Join all threads to wait for completion
- Ensures clean shutdown

---

## 📖 Chapter 22: Appendix - Reference Material

### **Appendix A: Keywords**

Complete list of reserved words in Rust:

**Control Flow**: `if`, `else`, `match`, `loop`, `while`, `for`, `break`, `continue`, `return`

**Declaration**: `let`, `mut`, `const`, `static`, `fn`, `struct`, `enum`, `union`, `trait`, `type`, `impl`

**Module System**: `mod`, `use`, `pub`, `crate`, `super`, `extern`

**Type System**: `Self`, `self`, `dyn`, `ref`, `move`, `as`, `where`

**Async**: `async`, `await`

**Safety**: `unsafe`

**Raw Identifiers**: Use `r#keyword` to use keywords as identifiers:

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

r#match("foo", "foobar");
```

---

### **Appendix C: Derivable Traits**

Standard library traits that can be automatically derived:

#### **Debug** - Programmer output

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

println!("{:?}", point);   // Point { x: 5, y: 10 }
println!("{:#?}", point);  // Pretty-printed
```

#### **PartialEq, Eq** - Equality

```rust
#[derive(PartialEq, Eq)]
struct Point { x: i32, y: i32 }

assert_eq!(p1, p2);
```

- `PartialEq`: Enables `==` and `!=`
- `Eq`: Guarantees reflexive equality (no NaN-like values)

#### **PartialOrd, Ord** - Ordering

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Version { major: u32, minor: u32, patch: u32 }

assert!(v1 < v2);
```

- `PartialOrd`: Enables `<`, `>`, `<=`, `>=`
- `Ord`: Guarantees total ordering

#### **Clone** - Explicit duplication

```rust
#[derive(Clone)]
struct Buffer { data: Vec<u8> }

let buf2 = buf1.clone();
```

#### **Copy** - Implicit bitwise copy

```rust
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

let p2 = p1;  // p1 still valid
```

**Requirement**: All fields must be `Copy`, no heap resources

#### **Hash** - For hash-based collections

```rust
#[derive(Hash, PartialEq, Eq)]
struct UserId(u64);

let mut map = HashMap::new();
map.insert(UserId(42), "Alice");
```

#### **Default** - Default values

```rust
#[derive(Default)]
struct Config {
    timeout: u64,    // 0
    verbose: bool,   // false
}

let config = Config { timeout: 5000, ..Default::default() };
```

#### **Common Combinations**

```rust
// Basic struct
#[derive(Debug, PartialEq)]

// HashMap key
#[derive(Hash, PartialEq, Eq)]

// BTreeMap key
#[derive(PartialEq, Eq, PartialOrd, Ord)]

// Small copyable type
#[derive(Debug, Copy, Clone, PartialEq)]

// Configuration
#[derive(Debug, Clone, PartialEq, Default)]
```

---

### **Appendix D: Development Tools**

#### **rustfmt** - Code formatting

```bash
rustfmt src/main.rs
cargo fmt
```

#### **clippy** - Linting

```bash
cargo clippy
cargo clippy -- -D warnings  # Treat warnings as errors
```

#### **rustfix** - Automatic fixes

```bash
cargo fix
```

#### **rust-analyzer** - IDE support

Language server for IDEs providing:
- Autocomplete
- Go to definition
- Inline errors
- Refactoring

---

### **Appendix E: Editions**

Rust's edition system allows language evolution:

- **Rust 2015**: Original edition
- **Rust 2018**: Async/await, improved module system, NLL
- **Rust 2021**: Disjoint capture, panic macros, reservations

```toml
[package]
edition = "2021"
```

**Key Points:**
- Editions are opt-in
- All editions interoperate
- Migration is automated with `cargo fix --edition`

---

## 🔗 Integration: Connecting the Chapters

### **Pattern Matching in Practice**

Patterns from Ch19 appear throughout the book:

```rust
// Ch21 web server - destructuring HTTP requests
let request_line = match buffer.lines().next() {
    Some(line) => line,
    None => return,
};

// Pattern matching for routing
match &request_line[..] {
    "GET / HTTP/1.1" => (status_line, filename),
    "GET /sleep HTTP/1.1" => {
        thread::sleep(Duration::from_secs(5));
        (status_line, filename)
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
}
```

### **Unsafe Rust with Concurrency**

Ch20 unsafe + Ch21 thread pool:

```rust
// Raw pointers in custom allocators (advanced)
struct CustomAllocator {
    ptr: *mut u8,
    capacity: usize,
}

unsafe impl Send for CustomAllocator {}  // Ch20 unsafe trait
unsafe impl Sync for CustomAllocator {}

// Use in thread pool from Ch21
pool.execute(move || {
    // Safe abstraction over unsafe allocator
});
```

### **Advanced Traits in Real Projects**

```rust
// Ch20 associated types + Ch21 worker pattern
trait Worker {
    type Job;

    fn execute(&self, job: Self::Job);
}

// Ch20 newtype pattern for type safety
struct TaskId(u64);
struct WorkerId(usize);

impl Worker for ThreadWorker {
    type Job = Box<dyn FnOnce() + Send>;

    fn execute(&self, job: Self::Job) {
        job();
    }
}
```

### **Macros for Boilerplate Reduction**

```rust
// Ch20 macros for Ch21 route handling
macro_rules! route {
    ($path:expr => $handler:expr) => {
        if request_line.starts_with($path) {
            return $handler(stream);
        }
    };
}

// Usage
route!("GET / " => handle_index);
route!("GET /about " => handle_about);
route!("GET /sleep " => handle_sleep);
```

---

## 📊 Quick Reference

### **Chapter 19: Pattern Essentials**

```rust
// Pattern locations
match value { Some(x) => x, None => 0 }
if let Some(x) = value { /* ... */ }
while let Some(x) = stack.pop() { /* ... */ }
for (i, v) in iter.enumerate() { /* ... */ }
let (x, y) = tuple;
fn foo((x, y): (i32, i32)) { /* ... */ }

// Pattern syntax
x @ 3..=7         // Bind and test range
Some(x) if x > 0  // Match guard
Point { x, .. }   // Ignore remaining fields
[first, .., last] // Array destructuring
```

### **Chapter 20: Advanced Features Essentials**

```rust
// Unsafe
unsafe {
    *raw_ptr
}

// Associated types
trait Iterator {
    type Item;
}

// Type alias
type Result<T> = std::result::Result<T, MyError>;

// Function pointer
fn takes_fn(f: fn(i32) -> i32) { }

// Declarative macro
macro_rules! vec {
    ( $( $x:expr ),* ) => { /* ... */ };
}
```

### **Chapter 21: Web Server Essentials**

```rust
// TCP listener
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

// Thread pool
let pool = ThreadPool::new(4);
pool.execute(|| handle_connection(stream));

// Graceful shutdown with Drop
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send termination, join threads
    }
}
```

### **Chapter 22: Reference Essentials**

```rust
// Derive common traits
#[derive(Debug, Clone, PartialEq, Hash, Default)]

// Raw identifiers
r#match("pattern", "text")

// Edition in Cargo.toml
edition = "2021"
```

---

## 🎯 Key Takeaways

### **Chapter 19**
1. **Patterns are everywhere** - match, if let, while let, for, let, function params
2. **Refutability matters** - know when patterns can fail
3. **Rich syntax** - literals, destructuring, ranges, guards, @ bindings
4. **Compiler enforces exhaustiveness** - catch all cases

### **Chapter 20**
1. **Unsafe is surgical** - five specific superpowers, not blanket permission
2. **Associated types for one-to-one** - generics for many-to-many
3. **Type aliases reduce noise** - especially for complex types
4. **Macros operate at compile time** - generate code, not execute it

### **Chapter 21**
1. **TCP is the foundation** - HTTP builds on TCP connections
2. **Thread pools beat unlimited spawning** - fixed overhead, controlled resources
3. **Arc<Mutex<T>> for shared state** - safe concurrent access
4. **Drop enables RAII** - automatic cleanup when scope ends

### **Chapter 22**
1. **Keywords are reserved** - use raw identifiers when needed
2. **Derive eliminates boilerplate** - for standard traits
3. **Tooling matters** - rustfmt, clippy, rust-analyzer improve workflow
4. **Editions enable evolution** - language grows without breaking changes

---

## 📚 Related Resources

### **Zettelkasten Connections**
- [[pattern-matching-locations]] - Complete guide to pattern usage
- [[refutable-vs-irrefutable-patterns]] - Deep dive on refutability
- [[unsafe-rust-guidelines]] - Safe abstraction principles
- [[trait-associated-types]] - Associated types vs generics
- [[rust-macros-comprehensive]] - Macro types and usage
- [[tcp-listener]] - TCP listener fundamentals
- [[thread-pool-pattern]] - Worker pool architecture
- [[rust-keywords-reference]] - Complete keyword list
- [[derivable-traits-summary]] - Trait derivation guide

### **Code Examples**
- `rust_book/Ch19/` - Pattern matching examples and exercises
- `rust_book/Ch20/` - Unsafe, traits, types, macros demonstrations
- `rust_book/Ch21/web_server/` - Complete multithreaded web server
- `rust_book/Ch22/` - Appendix reference materials

### **Mission Connections**
- [[mission-3]] - Binary search using pattern matching (Ch19)
- [[mission-4]] - Linked lists with unsafe code (Ch20)
- [[mission-8]] - Graph algorithms with trait-based design (Ch20)
- Web server project concepts apply to any networked application

### **AoC Applications**
- Pattern matching used extensively in puzzle input parsing
- Advanced traits for algorithm abstractions
- Unsafe code rarely needed but available when required

---

## 🎓 Chapter Completion Criteria

### **Chapter 19**
- [ ] Understand all six pattern locations
- [ ] Distinguish refutable from irrefutable patterns
- [ ] Master pattern syntax (destructuring, guards, @ bindings)
- [ ] Apply patterns in match, if let, for loops

### **Chapter 20**
- [ ] Understand when and why to use unsafe
- [ ] Implement safe abstractions over unsafe code
- [ ] Use associated types appropriately
- [ ] Create declarative macros for repetitive patterns

### **Chapter 21**
- [ ] Build single-threaded web server
- [ ] Experience blocking problem
- [ ] Implement thread pool pattern
- [ ] Add graceful shutdown with Drop

### **Chapter 22**
- [ ] Reference keywords when needed
- [ ] Derive appropriate traits
- [ ] Use rustfmt and clippy regularly
- [ ] Understand edition system

---

## 🚀 Beyond the Book

### **Advanced Pattern Techniques**
- Slice patterns with advanced destructuring
- Combining multiple pattern features
- Pattern matching in macro rules

### **Unsafe Rust Mastery**
- Writing FFI bindings
- Custom allocators
- Lock-free data structures
- Using Miri for validation

### **Production Web Servers**
- Request routing and middleware
- Async I/O with tokio
- HTTPS with TLS
- Load balancing and health checks

### **Macro Ecosystem**
- Writing custom derive macros
- Attribute macros for frameworks
- Procedural macros for DSLs

---

*Tags: #rust-book #ch19-22 #patterns #unsafe #advanced-features #web-server #appendix*

*Links: [[rust-book-ch16-18-review]] | [[rust-book-ch19]] | [[rust-book-ch20]] | [[rust-book-ch21]] | [[pattern-syntax-comprehensive]] | [[unsafe-rust-guidelines]]*
