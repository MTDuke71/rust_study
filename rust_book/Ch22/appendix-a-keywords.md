# Appendix A: Keywords

Rust keywords are reserved words that have special meaning in the language and cannot be used as identifiers (names of variables, functions, types, etc.) unless you use the raw identifier syntax.

## Keywords Currently in Use

### Control Flow Keywords
- **`if`** - Branch based on conditional expression
- **`else`** - Fallback for `if` and `if let` constructs
- **`match`** - Match a value to patterns
- **`loop`** - Loop unconditionally
- **`while`** - Loop conditionally based on expression
- **`for`** - Loop over items from an iterator
- **`break`** - Exit a loop immediately
- **`continue`** - Continue to the next loop iteration
- **`return`** - Return from function

### Declaration Keywords
- **`let`** - Bind a variable
- **`mut`** - Denote mutability in references, raw pointers, or pattern bindings
- **`const`** - Define constant items or constant raw pointers
- **`static`** - Global variable or lifetime lasting the entire program execution
- **`fn`** - Define a function or the function pointer type
- **`struct`** - Define a structure
- **`enum`** - Define an enumeration
- **`union`** - Define a union (keyword only when used in union declaration)
- **`trait`** - Define a trait
- **`type`** - Define a type alias or associated type
- **`impl`** - Implement inherent or trait functionality

### Module System Keywords
- **`mod`** - Define a module
- **`use`** - Bring symbols into scope
- **`pub`** - Denote public visibility in struct fields, `impl` blocks, or modules
- **`crate`** - In a module path, refers to the crate root
- **`super`** - Parent module of the current module
- **`extern`** - Link an external function or variable

### Type System Keywords
- **`Self`** - Type alias for the type we are defining or implementing
- **`self`** - Method subject or current module
- **`dyn`** - Dynamic dispatch to a trait object
- **`ref`** - Bind by reference
- **`move`** - Make a closure take ownership of all its captures
- **`as`** - Perform primitive casting, disambiguate trait items, or rename in `use` statements
- **`where`** - Denote clauses that constrain a type
- **`in`** - Part of `for` loop syntax

### Async/Await Keywords
- **`async`** - Return a `Future` instead of blocking the current thread
- **`await`** - Suspend execution until the result of a `Future` is ready

### Safety Keywords
- **`unsafe`** - Denote unsafe code, functions, traits, or implementations

### Literals
- **`true`** - Boolean true literal
- **`false`** - Boolean false literal

## Keywords Reserved for Future Use

These keywords don't have functionality yet but are reserved by Rust for potential future use:

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

## Raw Identifiers

Raw identifiers let you use keywords where they wouldn't normally be allowed by prefixing them with `r#`.

### Example: Using a Keyword as a Function Name

```rust
// Without raw identifier - DOES NOT COMPILE
// fn match(needle: &str, haystack: &str) -> bool {
//     haystack.contains(needle)
// }

// With raw identifier - COMPILES
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

### Use Cases for Raw Identifiers

1. **Using words that are keywords in newer editions**: If a library uses an identifier that became a keyword in a later edition (e.g., `try` in Rust 2018+), you can call it using `r#try` from newer edition code.

2. **Interfacing with other languages**: When working with FFI or code generation, you might encounter identifiers that are Rust keywords.

3. **Backwards compatibility**: Allows libraries from different Rust editions to work together seamlessly.

### Edition Compatibility Example

```rust
// Library written in Rust 2015 edition has a function named `try`
// In Rust 2018+ editions, call it using raw identifier:
// r#try(operation);
```

## Key Insights

- **Keyword stability**: Most keywords have been in Rust since 1.0, but some were added in later editions (`async`/`await` in 2018)
- **Future-proofing**: Reserved keywords prevent breaking changes when new features are added
- **Raw identifiers**: Provide escape hatch for edge cases without breaking backwards compatibility
- **Edition system**: Keywords can be edition-specific, allowing language evolution while maintaining compatibility

## Related Concepts

- **Identifiers**: Names for variables, functions, types, etc. (must not be keywords unless using raw identifiers)
- **Editions**: Different versions of Rust that can have different keyword sets (see Appendix E)
- **Syntax**: Keywords are part of Rust's core syntax and cannot be overloaded

---

**Book Reference**: [Appendix A: Keywords](https://doc.rust-lang.org/stable/book/appendix-01-keywords.html)

**Zettelkasten Links**: [[rust-keywords-reference]] | [[rust-editions-guide]]
