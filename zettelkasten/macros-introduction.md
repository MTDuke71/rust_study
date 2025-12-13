# Macros in Rust: Introduction

*Metaprogramming with declarative and procedural macros*

---

## 🎯 **What are Macros?**

**Macros** are Rust's metaprogramming facility that allow you to write code that writes other code (code generation). They operate at **compile time** and can expand into arbitrary Rust code.

**Key distinction**: Macros are invoked with `!` (e.g., `println!`, `vec!`, `panic!`)

```rust
// Macro invocation
let v = vec![1, 2, 3];  // Expands to Vec creation code

// Function call (no !)
let v = Vec::new();
```

## 🔄 **Macros vs Functions**

| **Feature** | **Functions** | **Macros** |
|-------------|---------------|------------|
| **Invocation** | `function()` | `macro!()` |
| **Evaluation** | Runtime | Compile time |
| **Type checking** | Strict parameter types | Pattern-based matching |
| **Variable arguments** | ❌ (without variadics) | ✅ Yes |
| **Code generation** | ❌ No | ✅ Yes |
| **Syntax extension** | ❌ No | ✅ Yes |
| **Performance** | Direct call | Zero-cost (expanded) |

### **When Macros are Necessary**

```rust
// Functions can't accept variable number of arguments
fn max(a: i32, b: i32) -> i32 { /* ... */ }  // Only 2 args

// Macros can!
let m = max!(1, 2, 3, 4, 5);  // Any number of args
let m = max!(1);              // Even 1 arg
```

## 📖 **Two Types of Macros**

### **1. Declarative Macros** (`macro_rules!`)

Pattern-based code generation using match-like syntax.

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!")
    };
}

say_hello!();  // Expands to println!("Hello!")
```

### **2. Procedural Macros** (proc macros)

Functions that take token streams as input and produce token streams as output.

```rust
// Custom derive macro
#[derive(Debug, Clone)]  // Debug and Clone are procedural macros
struct Point { x: i32, y: i32 }

// Attribute macro
#[route(GET, "/")]
fn index() -> &'static str { "Hello" }

// Function-like macro
let sql = sql!(SELECT * FROM users WHERE id = 1);
```

---

## 🎨 **Declarative Macros** (`macro_rules!`)

Pattern-matching macros that expand based on input patterns.

### **Basic Syntax**

```rust
macro_rules! macro_name {
    (pattern1) => { expansion1 };
    (pattern2) => { expansion2 };
    // ...
}
```

### **Simple Example**

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// Generate functions
create_function!(foo);
create_function!(bar);

foo();  // You called "foo"()
bar();  // You called "bar"()
```

### **Pattern Designators**

| **Designator** | **Matches** | **Example** |
|----------------|-------------|-------------|
| `ident` | Identifier | `x`, `foo`, `MyStruct` |
| `expr` | Expression | `1 + 2`, `foo()`, `x * y` |
| `ty` | Type | `i32`, `Vec<String>`, `&str` |
| `pat` | Pattern | `Some(x)`, `(a, b)` |
| `stmt` | Statement | `let x = 5;` |
| `block` | Block | `{ ... }` |
| `item` | Item (fn, struct, etc.) | `fn foo() {}` |
| `meta` | Attribute metadata | `cfg(test)` |
| `tt` | Token tree | Any valid tokens |
| `literal` | Literal | `42`, `"hello"`, `true` |

### **Variable Arguments**

```rust
macro_rules! vec_of_strings {
    ($($element:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push($element.to_string());
            )*
            v
        }
    };
}

let v = vec_of_strings![1, 2, 3, 4];  // vec!["1", "2", "3", "4"]
```

**Repetition syntax:**
- `$(...)*` - Zero or more repetitions
- `$(...)+` - One or more repetitions
- `$(...)?` - Zero or one repetition

### **Real-World Example: `vec!` Macro (from Rust Book Ch20.5)**

```rust
// Simplified version from the Rust Book
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

let v = vec![1, 2, 3];

// Expands to:
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

**Pattern breakdown:**
- `( $( $x:expr ),* )` - Zero or more expressions separated by commas
- `$x:expr` - Each expression is captured as `$x`
- `$( temp_vec.push($x); )*` - Repeated for each captured expression

### **Multiple Patterns**

```rust
macro_rules! test {
    (empty) => {
        println!("Empty case");
    };
    (single $x:expr) => {
        println!("Single: {}", $x);
    };
    (pair $x:expr, $y:expr) => {
        println!("Pair: {} and {}", $x, $y);
    };
}

test!(empty);         // Empty case
test!(single 42);     // Single: 42
test!(pair 1, 2);     // Pair: 1 and 2
```

### **Practical Example: Mini Testing Framework**

```rust
macro_rules! assert_eq_verbose {
    ($left:expr, $right:expr) => {
        {
            let left_val = $left;
            let right_val = $right;
            if left_val != right_val {
                panic!(
                    "Assertion failed: {} != {}\n  left: {:?}\n  right: {:?}",
                    stringify!($left),
                    stringify!($right),
                    left_val,
                    right_val
                );
            }
        }
    };
}

assert_eq_verbose!(2 + 2, 4);  // Passes
// assert_eq_verbose!(2 + 2, 5);  // Panics with detailed message
```

---

## 🔧 **Procedural Macros**

Functions that operate on Rust token streams at compile time.

### **Three Types**

#### **1. Custom Derive Macros**

Automatically implement traits for structs/enums.

```rust
// Usage
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

// What it generates (conceptually)
impl Debug for User { /* ... */ }
impl Clone for User { /* ... */ }
impl Serialize for User { /* ... */ }
impl Deserialize for User { /* ... */ }
```

**Common derive macros:**
- `Debug` - Pretty-print with `{:?}`
- `Clone` - Deep copy capability
- `Copy` - Stack copy for simple types
- `PartialEq`, `Eq` - Equality comparison
- `PartialOrd`, `Ord` - Ordering
- `Hash` - Hashing for HashMap keys
- `Default` - Default values

#### **2. Attribute-Like Macros**

Similar to custom derive, but work on any item (not just structs/enums).

**Example from Rust Book** - web framework route attribute:
```rust
#[route(GET, "/")]
fn index() {
    // Function body
}
```

**Definition:**
```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // attr = GET, "/"
    // item = fn index() { ... }
    // Generate modified code
}
```

**Two TokenStream parameters:**
- First: attribute contents (`GET, "/"`)
- Second: item the attribute is attached to (`fn index() {...}`)

**Other common attribute macros:** More flexible than `macro_rules!`.

**Example from Rust Book** - SQL validation macro:
```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

**Definition:**
```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // Parse SQL inside parentheses
    // Check syntax validity
    // Generate validated code
}
```

**Comparison to declarative macros:**
- `macro_rules!` - Pattern matching only
- Function-like proc macros - Full Rust code manipulation via TokenStream
- Can perform complex validation/parsing that `macro_rules!` cannot

**Other examples:**
```rust
format!("Hello, {}!", name)    // Built-in formatting
println!("Debug: {:?}", value) // Console output
include_str!("file.txt")       // Compile-time file inclusionoken streams.

```rust
// SQL query macro
let query = sql!(SELECT * FROM users WHERE id = $1);

// HTML template macro
let html = html! {
    <div class="container">
        <h1>Hello, World!</h1>
    </div>
};

// Format strings are function-like macros
let msg = format!("Hello, {}!", "world");
```

### **Creating Custom Procedural Macros (HelloMacro Example from Rust Book)**

Requires separate crate with `proc-macro = true`:

```toml
# hello_macro_derive/Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
```

**The trait we want to derive** (in `hello_macro` crate):
```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

**The derive macro** (in `hello_macro_derive` crate):
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse(input).unwrap();
    
    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

**Usage:**
```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();  // Prints: Hello, Macro! My name is Pancakes!
}
```

**Key components:**
- **`syn`** - Parses Rust code from TokenStream into syntax tree
- **`quote!`** - Generates Rust code (templating with `#name` substitution)
- **`proc_macro`** - Compiler API for token manipulation
- **`stringify!`** - Converts expression to string literal at compile time

---

## 🎯 **Common Built-in Macros**

### **Output Macros**

```rust
println!("Hello, {}!", name);       // Print with newline
print!("No newline");                // Print without newline
eprintln!("Error: {}", err);        // Print to stderr
format!("Formatted {}", string);     // Return String
```

### **Assertion Macros**

```rust
assert!(condition);                  // Panic if false
assert_eq!(left, right);            // Panic if not equal
assert_ne!(left, right);            // Panic if equal
debug_assert!(condition);            // Only in debug builds
```

### **Control Flow Macros**

```rust
panic!("Fatal error: {}", msg);     // Unrecoverable error
unimplemented!();                    // Not yet implemented
unreachable!();                      // Should never reach
todo!();                             // Placeholder for later
```

### **Code Generation Macros**

```rust
vec![1, 2, 3];                      // Create vector
format!("Template {}", var);         // Format string
include_str!("file.txt");           // Include file as &str
include_bytes!("data.bin");         // Include file as &[u8]
env!("PATH");                        // Environment variable at compile time
```

### **Metaprogramming Macros**

```rust
stringify!(expr);                    // Expression to string literal
concat!("Hello", " ", "World");     // Concatenate literals
file!();                             // Current file name
line!();                             // Current line number
column!();                           // Current column number
module_path!();                      // Current module path
```

---

## ⚡ **Performance & Compilation**

### **Zero-Cost Abstraction**

Macros expand at compile time - **no runtime cost**.

```rust
// This macro expansion
let v = vec![1, 2, 3];

// Compiles to same code as
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);
```

### **Compile Time vs Runtime**

```rust
// Compile-time string concatenation (zero runtime cost)
const MESSAGE: &str = concat!("Hello", " ", "World");

// Runtime string concatenation (allocates)
let message = format!("{} {}", "Hello", "World");
```

---

## 🚫 **Common Pitfalls**

### **1. Macro Hygiene**

```rust
macro_rules! bad_macro {
    () => {
        let x = 42;  // This 'x' might conflict with caller's 'x'
    };
}

// Potential name collision
let x = 10;
bad_macro!();
println!("{}", x);  // Which x?
```

**Solution**: Rust's macro system is mostly hygienic, but be careful with identifiers.

### **2. Debugging Difficulties**

```rust
// Error messages can be cryptic
my_macro!(some complex syntax);
// Error: no rules expected this token
```

**Solution**: Use `cargo exp20]]** - Chapter 20.5 covers macros (this note's source)
- **[[rust_book/Ch20/examples/ch20_5_macros]]** - Runnable examples from the chapter
### **3. Compile Time Increase**

Large procedural macros can slow compilation significantly.

---

## 🔗 **Related Concepts**

### **Rust Book Integration**

- **[[rust_book/rust-book-ch19]]** - Advanced features including macros
- **[[rust_book/rust-book-ch20]]** - Metaprogramming patterns

### **Macro Types**

- **[[declarative-macros]]** - Pattern-based `macro_rules!`
- **[[procedural-macros]]** - Token stream processing
- **[[derive-macros]]** - Custom trait derivation

### **Practical Applications**

- **[[testing-macros]]** - Custom test frameworks
- **[[DSL-with-macros]]** - Domain-specific languages
- **[[compile-time-computation]]** - Const evaluation

---

## 📚 **Key Takeaways**

1. **Macros enable metaprogramming** - code that writes code
2. **Two categories**: Declarative (`macro_rules!`) and Procedural (derive, attribute, function-like)
3. **Compile-time expansion** - zero runtime overhead
4. **Pattern matching syntax** for declarative macros
5. **Token stream processing** for procedural macros
6. **Use `!` for invocation** - distinguishes from functions
7. **Powerful but complex** - use judiciously

---

*Tags: #macros #metaprogramming #declarative-macros #procedural-macros #compile-time #code-generation #rust-book-ch19 #macro_rules #derive #attributes*

*Links: [[declarative-macros]] | [[procedural-macros]] | [[derive-macros]] | [[rust_book/rust-book-ch19]] | [[rust-concepts-MOC]]*
