# Copilot Rules for Rust Study Project

## 📋 **Naming Conventions**

### **Example Names Must Be Unique**
When creating new examples in `Cargo.toml` files, **ALWAYS** use unique names across the entire workspace to avoid filename collisions.

**✅ CORRECT Examples:**
```toml
[[example]]
name = "mission3_demo"  # Prefixed with package context
path = "examples/demo.rs"

[[example]]
name = "aoc_pattern_simple_demo"  # Descriptive and unique
path = "examples/simple_demo.rs"

[[example]]
name = "mission4_performance_comparison"  # Specific to package
path = "examples/performance_comparison.rs"
```

**❌ INCORRECT Examples:**
```toml
[[example]]
name = "demo"  # Too generic, will collide with other packages
path = "examples/demo.rs"

[[example]]
name = "simple_demo"  # Common name, will collide
path = "examples/simple_demo.rs"

[[example]]
name = "performance_comparison"  # Generic, will collide
path = "examples/performance_comparison.rs"
```

### **Naming Pattern Rules:**

1. **Use package context prefix**: `{package_name}_{example_name}`
2. **Be descriptive**: Include the purpose or functionality
3. **Avoid generic names**: `demo`, `test`, `example`, `simple`, `basic`
4. **Check existing names**: Search workspace for existing example names before creating new ones

### **Example Name Templates:**

- **Mission packages**: `mission{N}_{descriptive_name}`
- **Tutorial packages**: `mission{N}_tut_{descriptive_name}`
- **AoC packages**: `aoc_{descriptive_name}`
- **Advanced examples**: `{package_name}_{descriptive_name}`

### **Validation Command:**
Before committing, run:
```bash
cargo test --workspace --no-run
```
This will catch filename collisions early.

---

## 🛠️ **Code Quality Rules**

### **Error Handling**
- Always derive `PartialEq` only when all variants support it
- Use `assert!(result.is_ok())` instead of `assert_eq!(result, Ok(value))` when testing `Result` types with complex error types
- Move `impl` blocks outside of functions to module level

### **Clippy Warnings**
- Fix all clippy warnings before committing
- Use `#[allow(...)]` attributes only when necessary and document why
- Prefer iterator methods over manual loops

### **Type Annotations**
- Provide explicit type annotations for complex generic types
- Use type aliases for complex return types to improve readability

---

## 📁 **File Organization**

### **Workspace Structure**
- Keep related examples in the same package
- Use consistent naming across similar packages
- Maintain clear separation between different learning modules

### **Documentation**
- Include comprehensive doc comments for examples
- Use consistent formatting for code examples
- Provide clear explanations of concepts being demonstrated

---

## ⚠️ **Common Pitfalls to Avoid**

1. **Filename Collisions**: The most common issue - always use unique example names
2. **Generic Names**: Avoid `demo`, `test`, `example` without context
3. **Missing Type Annotations**: Complex generics need explicit types
4. **Trait Bound Issues**: Ensure all types in error enums can derive required traits
5. **Scope Issues**: Keep `impl` blocks at module level, not inside functions

---

## 🔍 **Quick Checklist**

Before creating a new example:
- [ ] Check if example name already exists in workspace
- [ ] Use descriptive, unique name with package context
- [ ] Verify compilation with `cargo check`
- [ ] Run clippy to catch warnings
- [ ] Test with `cargo test --workspace --no-run`

This prevents the most common issues that cause build failures in our Rust learning workspace.
