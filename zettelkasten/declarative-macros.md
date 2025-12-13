# Declarative Macros (`macro_rules!`)

*Navigation: [[zettel-index]] | [[macros-introduction]] | [[procedural-macros]] | [[rust_book/rust-book-ch20]]*

---

## Overview

**Declarative macros** (also called "macros by example") use pattern matching to transform code at compile time. They're defined with `macro_rules!` and match against code structure rather than executing code.

**Key Insight**: Declarative macros are like sophisticated find-and-replace with pattern matching on Rust's abstract syntax tree (AST). They're the most common type of macro in Rust.

---

## Basic Syntax

```rust
macro_rules! macro_name {
    // Pattern 1 => Template 1
    (pattern1) => {
        // Code to generate
    };
    
    // Pattern 2 => Template 2
    (pattern2) => {
        // Code to generate
    };
}
```

**Components**:
- **Pattern**: Matches against input syntax
- **Template**: Code to generate when pattern matches
- **Arms**: Each `pattern => template` pair (like match arms)

---

## Pattern Matching Fragments

Declarative macros use **fragment specifiers** to capture different syntax elements:

| Specifier | Matches | Example |
|-----------|---------|---------|
| `$name:ident` | Identifier | `foo`, `MyStruct` |
| `$name:expr` | Expression | `x + 1`, `vec![1, 2, 3]` |
| `$name:ty` | Type | `i32`, `Vec<String>` |
| `$name:pat` | Pattern | `Some(x)`, `(a, b)` |
| `$name:stmt` | Statement | `let x = 5;` |
| `$name:block` | Block | `{ ... }` |
| `$name:item` | Item | `fn foo() {}`, `struct Bar;` |
| `$name:path` | Path | `std::vec::Vec` |
| `$name:tt` | Token tree | Any single token or group |
| `$name:meta` | Attribute contents | Inside `#[...]` |
| `$name:lifetime` | Lifetime | `'a`, `'static` |
| `$name:vis` | Visibility | `pub`, `pub(crate)` |
| `$name:literal` | Literal | `42`, `"hello"` |

---

## Repetition Patterns

**Three repetition operators**:

```rust
$(...)*   // Zero or more repetitions
$(...)+   // One or more repetitions
$(...)?   // Zero or one repetition (optional)
```

**Example - The `vec!` macro**:

```rust
#[macro_export]
macro_rules! vec {
    // Match zero or more comma-separated expressions
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

**Breakdown**:
- `$( $x:expr ),*` - Captures zero or more expressions separated by commas
- `$x` - Each captured expression
- `$( temp_vec.push($x); )*` - Repeat push for each captured expression

**Usage**:
```rust
let v = vec![1, 2, 3, 4, 5];
// Expands to:
// {
//     let mut temp_vec = Vec::new();
//     temp_vec.push(1);
//     temp_vec.push(2);
//     temp_vec.push(3);
//     temp_vec.push(4);
//     temp_vec.push(5);
//     temp_vec
// }
```

---

## Multiple Pattern Arms

Macros can have multiple arms like match expressions:

```rust
macro_rules! create_function {
    // Arm 1: Function with no parameters
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
    
    // Arm 2: Function with parameters
    ($func_name:ident, $($arg:ident : $arg_ty:ty),+) => {
        fn $func_name($($arg : $arg_ty),+) {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// Usage:
create_function!(foo);                    // No params
create_function!(bar, x: i32, y: i32);   // With params
```

---

## Common Patterns

### Pattern 1: Assertion/Testing Macros

```rust
macro_rules! assert_eq_verbose {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(
                        "Assertion failed: {} == {}\n  left: {:?}\n right: {:?}",
                        stringify!($left),
                        stringify!($right),
                        left_val,
                        right_val
                    );
                }
            }
        }
    };
}
```

**Key technique**: Using `stringify!()` to capture expression as string for error messages.

### Pattern 2: DSL Creation

```rust
macro_rules! hashmap {
    // Empty hashmap
    () => {
        std::collections::HashMap::new()
    };
    
    // Hashmap with key-value pairs
    ( $($key:expr => $val:expr),+ $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )+
            map
        }
    };
}

// Usage:
let map = hashmap! {
    "name" => "Alice",
    "age" => "30",
};
```

**Note**: `$(,)?` allows optional trailing comma.

### Pattern 3: Type Conversion Helpers

```rust
macro_rules! impl_from {
    ($from_type:ty => $to_type:ty, $field:ident) => {
        impl From<$from_type> for $to_type {
            fn from(value: $from_type) -> Self {
                Self {
                    $field: value,
                }
            }
        }
    };
}

// Usage:
impl_from!(i32 => MyStruct, value);
```

### Pattern 4: Code Generation with Counting

```rust
macro_rules! count_exprs {
    () => (0);
    ($e:expr) => (1);
    ($e:expr, $($rest:expr),+) => (1 + count_exprs!($($rest),+));
}

// Usage:
const COUNT: usize = count_exprs!(1, 2, 3, 4, 5); // 5
```

---

## Advanced Techniques

### Recursive Macros

Macros can call themselves recursively:

```rust
macro_rules! reverse {
    // Base case: empty or single element
    () => { () };
    ($single:expr) => { ($single,) };
    
    // Recursive case
    ($first:expr, $($rest:expr),+) => {
        (reverse!($($rest),+), $first)
    };
}
```

### Internal Rules (Helper Arms)

Use `@` prefix for internal-only rules:

```rust
macro_rules! count {
    // Public API
    ($($x:expr),*) => {
        count!(@internal 0; $($x),*)
    };
    
    // Internal helper (accumulator pattern)
    (@internal $count:expr;) => {
        $count
    };
    (@internal $count:expr; $x:expr $(, $rest:expr)*) => {
        count!(@internal $count + 1; $($rest),*)
    };
}
```

**Pattern**: Use `@` to hide implementation details from users.

---

## Hygiene and Scope

### Hygienic Macros

Rust macros are **hygienic** - they don't accidentally capture variables from the call site:

```rust
macro_rules! using_temp {
    ($e:expr) => {
        {
            let temp = 10;
            $e + temp  // temp is macro's local variable
        }
    };
}

let temp = 5;
let result = using_temp!(temp);  // Uses outer temp (5), not macro's temp (10)
// result = 15 (not 20)
```

### Exporting Macros

```rust
#[macro_export]  // Makes macro available to other crates
macro_rules! my_macro {
    // ...
}
```

**Note**: Exported macros are placed at crate root, regardless of module structure.

---

## Debugging Macros

### Expansion Viewing

```bash
# See macro expansion
cargo expand

# With specific macro
cargo expand my_macro
```

### Compile-Time Debugging

```rust
macro_rules! debug_macro {
    ($($arg:tt)*) => {
        compile_error!(concat!("Debug: ", stringify!($($arg)*)));
    };
}
```

### Runtime Introspection

```rust
macro_rules! dbg_macro {
    ($val:expr) => {
        println!("{}:{} - {} = {:?}", 
            file!(), line!(), stringify!($val), $val);
    };
}
```

---

## Common Pitfalls

### 1. Token Tree Limitations

**Problem**: Can't match arbitrary token sequences:
```rust
// ❌ Can't do this:
macro_rules! bad {
    ($x:expr + $y:expr) => { ... };  // Won't parse "+", wrong pattern!
}
```

**Solution**: Use broader captures:
```rust
// ✅ Correct:
macro_rules! good {
    ($x:expr, $y:expr) => { ... };
}
```

### 2. Repetition Separator Confusion

```rust
// ❌ Wrong: Missing repetition
( $($x:expr)* ) => { vec![$x] };  // Error: $x not in repetition

// ✅ Correct:
( $($x:expr)* ) => { vec![$($x),*] };
```

### 3. Expression vs Statement

```rust
// ❌ Generates statement, not expression
macro_rules! make_num {
    () => { let x = 5; };  // Statement, can't use in expression context
}

// ✅ Generates expression
macro_rules! make_num {
    () => { 5 };  // Expression
}
```

### 4. Type Inference Issues

```rust
// Sometimes need explicit type annotations in macro output
macro_rules! make_vec {
    () => {
        Vec::new()  // May cause type inference failures
    };
}

// Better:
macro_rules! make_vec {
    ($t:ty) => {
        Vec::<$t>::new()  // Explicit type parameter
    };
}
```

---

## When to Use Declarative Macros

**✅ Use When**:
- Need compile-time code generation
- Reducing boilerplate with simple patterns
- Creating mini-DSLs
- Need variadic arguments (variable number of arguments)

**❌ Avoid When**:
- Complex syntax transformations (use procedural macros)
- Need to inspect/modify item structure (use proc macros)
- Runtime code generation needed (use functions)
- Simple generic functions would work (prefer generics)

---

## Performance Characteristics

- **Compile time**: Macro expansion happens during compilation
- **Runtime**: Zero cost - expanded code is identical to hand-written code
- **Binary size**: Can increase if macro generates lots of code
- **Debugging**: Harder to debug than regular functions (use `cargo expand`)

---

## Comparison with Functions

| Aspect | Macros | Functions |
|--------|--------|-----------|
| **Evaluation time** | Compile-time | Runtime |
| **Type checking** | After expansion | Before call |
| **Arguments** | Token trees | Typed values |
| **Variadic** | Yes (via repetition) | No (without traits) |
| **Return type** | Any syntax | Specific type |
| **Performance** | Zero overhead | Function call overhead (usually optimized away) |
| **Debugging** | Harder | Easier |
| **Code visibility** | Less clear | Clear |

---

## Integration with Rust Book Ch20.5

From the Rust Book examples:

**`vec!` Macro Explained**:
```rust
// Simplified version from the book
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

**Key insights**:
- Pattern `$( $x:expr ),*` captures comma-separated expressions
- Inner `$( ... )*` repeats code for each captured expression
- Generates efficient code equivalent to manual `Vec::new()` + multiple `push()` calls
- Zero runtime overhead - all work done at compile time

---

## Real-World Examples

### 1. Testing Framework (similar to `assert_eq!`)

```rust
macro_rules! test_case {
    ($name:ident: $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(my_function($input), $expected);
        }
    };
}

test_case!(test_zero: 0 => 0);
test_case!(test_positive: 5 => 25);
test_case!(test_negative: -3 => 9);
```

### 2. Configuration Builder

```rust
macro_rules! config {
    ($($key:ident : $value:expr),* $(,)?) => {
        {
            let mut cfg = Config::default();
            $(
                cfg.$key = $value;
            )*
            cfg
        }
    };
}

let config = config! {
    timeout: 30,
    retries: 3,
    debug: true,
};
```

### 3. Enum Variant Iteration

```rust
macro_rules! enum_variants {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        impl $name {
            pub fn variants() -> Vec<Self> {
                vec![$($name::$variant),*]
            }
        }
    };
}

enum Color { Red, Green, Blue }
enum_variants!(Color { Red, Green, Blue });
```

---

## Related Concepts

### Macro Fundamentals
- [[macros-introduction]] - Overview of macro types and use cases
- [[procedural-macros]] - Custom derive, attribute-like, and function-like proc macros

### Advanced Rust
- [[rust-metaprogramming]] - Compile-time code generation techniques
- [[Rust Traits]] - Often better than macros for polymorphism

### Code Generation
- [[build-scripts]] - Alternative code generation approach
- [[const-evaluation]] - Compile-time computation without macros

---

## Further Reading

- **Rust Book Ch19.5**: Original macro introduction (expanded in Ch20.5)
- **The Little Book of Rust Macros**: Comprehensive macro guide
- **Rust Reference**: Macros by Example chapter
- **`cargo expand`**: Essential tool for debugging macro expansions

---

*Tags: #macros #declarative-macros #macro-rules #metaprogramming #compile-time #code-generation #rust-advanced*

*Links: [[zettel-index]] | [[macros-introduction]] | [[procedural-macros]] | [[rust_book/rust-book-ch20]] | [[../../rust_book/Ch20/examples/ch20_5_macros]]*
