# 🔧 Common Errors & Troubleshooting Guide

This document catalogs frequently encountered errors in the Rust Study workspace and their proven solutions. Organized by category for quick reference.

---

## 📋 Table of Contents

1. [Type System Errors](#type-system-errors)
2. [Struct & Field Errors](#struct--field-errors)
3. [Compilation & Build Errors](#compilation--build-errors)
4. [Test & Documentation Errors](#test--documentation-errors)
5. [Cargo Workspace Errors](#cargo-workspace-errors)
6. [Tutorial-Specific Errors](#tutorial-specific-errors)
7. [Daily Study Errors](#daily-study-errors)

---

## Type System Errors

### Error: Cannot infer type for generic parameter

**Symptom**:
```
error[E0282]: type annotations needed
  --> src/main.rs:10:9
   |
10 |     let visited = HashSet::new();
   |         ^^^^^^^ cannot infer type for type parameter `T`
```

**Cause**: Compiler can't determine the generic type `T` for collection constructors.

**Solution**:
```rust
// Option 1: Type annotation
let visited: HashSet<NodeId> = HashSet::new();

// Option 2: Turbofish syntax
let visited = HashSet::<NodeId>::new();

// Option 3: Use type inference from usage
let visited = HashSet::new();
visited.insert(node_id);  // Compiler infers HashSet<NodeId>
```

**Prevention**: Always annotate types for generic `::new()` methods unless the type is immediately clear from usage.

---

### Error: Mismatched types in function call

**Symptom**:
```
error[E0308]: mismatched types
  --> src/lib.rs:45:23
   |
45 |     some_function(value);
   |                   ^^^^^ expected `&T`, found `T`
```

**Cause**: Function expects a reference but received an owned value (or vice versa).

**Solution**:
```rust
// If function needs &T but you have T:
some_function(&value);

// If function needs T but you have &T:
some_function(value.clone());  // Or use *value if T: Copy
```

**Prevention**: Check function signatures carefully. Use `&` for borrowing, omit for ownership transfer.

---

## Struct & Field Errors

### Error: No such field in struct

**Symptom**:
```
error[E0560]: struct `MyStruct` has no field named `field_name`
  --> src/main.rs:20:9
   |
20 |         field_name: value,
   |         ^^^^^^^^^^ `MyStruct` does not have this field
```

**Cause**: Field name in instantiation doesn't match struct definition (often due to renaming with `_` prefix).

**Solution**:
```rust
// Check struct definition
struct MyStruct {
    _unused_field: i32,  // Note the underscore prefix!
    used_field: String,
}

// Update instantiation to match
let instance = MyStruct {
    _unused_field: 42,   // Must include underscore
    used_field: String::from("hello"),
};
```

**Prevention**: When renaming struct fields:
1. Update the struct definition
2. Use global find-replace for ALL instantiations
3. Run `cargo check` to catch any misses

---

### Error: Missing structure fields

**Symptom**:
```
error[E0063]: missing fields `field1`, `field2` in initializer of `MyStruct`
  --> src/main.rs:25:20
   |
25 |     let instance = MyStruct { field3: value };
   |                    ^^^^^^^^ missing `field1`, `field2`
```

**Cause**: Struct instantiation missing required fields.

**Solution**:
```rust
// Include ALL fields
let instance = MyStruct {
    field1: default_value,
    field2: another_value,
    field3: value,
};

// Or use struct update syntax
let instance = MyStruct {
    field3: value,
    ..Default::default()  // Requires #[derive(Default)]
};
```

**Prevention**: Always check struct definition and include all non-optional fields.

---

## Compilation & Build Errors

### Error: Unused imports

**Symptom**:
```
warning: unused import: `GraphType`
  --> examples/step4.rs:10:5
   |
10 | use crate::{GraphType, NodeId};
   |             ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default
```

**Cause**: Imported item never used in code.

**Solution**:
```rust
// Remove unused imports
use crate::NodeId;  // Keep only what's used

// Or allow if temporarily unused during development
#[allow(unused_imports)]
use crate::{GraphType, NodeId};
```

**Prevention**: 
- Run `cargo clippy` regularly
- Remove imports immediately when refactoring removes their usage
- Use IDE features to auto-remove unused imports

---

### Error: Variable does not need to be mutable

**Symptom**:
```
warning: variable does not need to be mutable
  --> src/main.rs:15:9
   |
15 |     let mut result = calculate();
   |         ----^^^^^^
   |         |
   |         help: remove this `mut`
```

**Cause**: Variable declared `mut` but never modified.

**Solution**:
```rust
// Remove mut if not modifying
let result = calculate();
println!("{}", result);  // Only reading, not modifying

// Keep mut if actually modifying
let mut result = calculate();
result += 10;  // Modifying the value
```

**Prevention**: Only use `mut` when you actually intend to modify the variable.

---

### Error: Format string placeholder mismatch

**Symptom**:
```
error: 3 positional arguments in format string, but 2 arguments were supplied
  --> src/main.rs:30:20
   |
30 |     format!("#{}{}{}", arg1, arg2);
   |                ^^^^^^   ----  ---- supplied 2 arguments
   |                |
   |                3 positional arguments in format string
```

**Cause**: Number of `{}` placeholders doesn't match number of arguments.

**Solution**:
```rust
// Count placeholders: # {} {} {} = 3 placeholders
// Must provide 3 arguments:
format!("#{}{}{}", arg1, arg2, arg3);

// Or adjust format string to match arguments:
format!("#{}{}", arg1, arg2);
```

**Prevention**: Count format placeholders before writing argument list.

---

## Test & Documentation Errors

### Error: Doctest failed to compile

**Symptom**:
```
error: cannot find function `my_function` in this scope
  --> src/lib.rs:15:1
   |
15 | / /// # Examples
16 | | ///
17 | | /// ```
18 | | /// my_function(42);
   | | ^^^^^^^^^^^^^^^^^ not found in this scope
```

**Cause**: Doctest example missing necessary imports or using wrong scope.

**Solution**:
```rust
/// # Examples
///
/// ```
/// use crate::my_function;  // Add import
/// 
/// my_function(42);
/// ```

// Or use hidden lines (won't appear in docs but will compile)
/// # Examples
///
/// ```
/// # use crate::my_function;
/// my_function(42);
/// ```
```

**Prevention**: Always test doctests with `cargo test --doc`.

---

### Error: Test not found

**Symptom**:
```bash
$ cargo test req1
warning: test filter `req1` does not match any tests
```

**Cause**: Test name doesn't match filter pattern or test is in wrong location.

**Solution**:
```bash
# Check test exists in tests/ directory
ls tests/

# Run all tests to see available test names
cargo test -- --list

# Make sure test follows naming convention
# tests/req1_feature_name.rs
```

**Prevention**: Follow naming convention `req{N}_*` for requirement tests.

---

## Cargo Workspace Errors

### Error: Package not found in workspace

**Symptom**:
```bash
$ cargo run -p mission5
error: package `mission5` is not a member of the workspace
```

**Cause**: Package name doesn't match `Cargo.toml` workspace member or wrong name used.

**Solution**:
```bash
# Check workspace members in root Cargo.toml
cat Cargo.toml | grep members

# Package names use exact directory path
cargo run -p missions/mission5  # If in subdirectory

# Or check package name in package's Cargo.toml
cat missions/Mission5/Cargo.toml | grep name
# Shows: name = "mission5" (lowercase!)
```

**Prevention**: Use correct package name (often lowercase despite directory being capitalized).

---

### Error: Cannot build workspace

**Symptom**:
```
error: failed to select a version for the requirement `some_crate = "^1.0"`
```

**Cause**: Dependency version conflict or missing dependency.

**Solution**:
```bash
# Update Cargo.lock
cargo update

# Or clean and rebuild
cargo clean
cargo build --workspace

# Check for conflicting versions
cargo tree | grep some_crate
```

**Prevention**: Keep dependencies consistent across workspace members.

---

## Tutorial-Specific Errors

### Error: Example not found

**Symptom**:
```bash
$ cargo run -p mission5_tut --example step3_hashmap
error: no example target named `step3_hashmap`
```

**Cause**: Example file not in correct location or name mismatch.

**Solution**:
```bash
# Check examples directory
ls tutorials/Mission5_tut/examples/

# Example files must be in examples/ directory
# tutorials/Mission5_tut/examples/step3_hashmap.rs

# Run without .rs extension
cargo run -p mission5_tut --example step3_hashmap
```

**Prevention**: 
- Place example files in `examples/` directory
- Name them `stepN_description.rs`
- Reference without `.rs` extension

---

## Daily Study Errors

### Error: Markdown code extraction failed

**Symptom**:
```bash
$ .\scripts\run_md.bat Day10.md
Error: No "Complete Runnable Example" section found
```

**Cause**: Markdown file missing required section or wrong format.

**Solution**:
```markdown
## 🚀 **Complete Runnable Example**

```rust
fn main() {
    println!("Example code here");
}
```

### **🛠️ How to Run This Code:**
1. **Online**: Copy to Rust Playground
2. **Local**: Save as `dayX_demo.rs` and run
3. **Workspace**: `.\scripts\run_md.bat path\to\file.md`
```

**Prevention**: Use template from `.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md`.

---

### Error: Code block not executable

**Symptom**:
```
error[E0425]: cannot find function `helper` in this scope
```

**Cause**: Code block missing helper functions or dependencies.

**Solution**:
```rust
// Include ALL helper functions in the complete runnable example
fn main() {
    helper();
}

fn helper() {  // Don't forget helper functions!
    println!("Helper");
}
```

**Prevention**: Make examples **self-contained** with all required functions.

---

## 🔍 Debugging Workflow

When encountering an error:

1. **Read the error message** - Rust errors are detailed and helpful
2. **Check this guide** - See if it's a known pattern
3. **Run relevant cargo command**:
   - `cargo check` - Fast syntax checking
   - `cargo clippy` - Catch common mistakes
   - `cargo test` - Run tests
   - `cargo build` - Full compilation
4. **Search error code** - `E0282`, `E0308`, etc. in Rust docs
5. **Isolate the problem** - Create minimal reproduction
6. **Check recent changes** - What was modified last?

---

## 📝 Adding New Error Patterns

If you encounter a new recurring error:

1. Document the symptom (exact error message)
2. Explain the root cause
3. Provide concrete solution with code examples
4. Add prevention strategy
5. Submit to this document

---

## 🔗 Related Resources

- [Rust Error Index](https://doc.rust-lang.org/error-index.html) - Official error explanations
- [Rust Compiler Error Messages](https://doc.rust-lang.org/stable/error_codes/) - Detailed error codes
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
- [[RUST_DOCUMENTATION_STANDARDS.md]] - Documentation requirements
- [[copilot-instructions.md]] - Full development guide

---

*Last Updated: October 12, 2025*
*Contributions welcome - add patterns as they're discovered!*
