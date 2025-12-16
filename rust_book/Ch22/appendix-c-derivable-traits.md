# Appendix C: Derivable Traits

The `derive` attribute automatically generates trait implementations for structs and enums. This appendix covers all standard library traits that can be derived.

## Overview

The `derive` attribute provides default implementations for common traits, saving you from writing boilerplate code. These traits cover debugging, comparisons, cloning, hashing, and default values.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

## Derivable Traits

### `Debug` - Programmer Output

**Purpose**: Enable debug formatting with `{:?}` and `{:#?}`.

**Generated behavior**: Prints struct/enum name and all fields.

**When required**:
- Using `assert_eq!` macro (prints values on failure)
- Debugging and troubleshooting
- Logging internal state

**Conditions**: All fields must implement `Debug`.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle { width: 30, height: 50 };
println!("{:?}", rect);  // Rectangle { width: 30, height: 50 }
println!("{:#?}", rect); // Pretty-printed format
```

### `PartialEq` and `Eq` - Equality Comparisons

**`PartialEq` Purpose**: Enable `==` and `!=` operators.

**Generated behavior**:
- **Structs**: Equal if all fields are equal
- **Enums**: Each variant equals itself, not equal to other variants

**When required**:
- Using `assert_eq!` macro
- Comparisons with `==` and `!=`
- Collections that need equality checks

**Conditions**: All fields must implement `PartialEq`.

**`Eq` Purpose**: Signal that equality is reflexive (every value equals itself).

**Key difference**: `PartialEq` allows for values that don't equal themselves (like `f32::NAN`), while `Eq` guarantees all values have reflexive equality.

**When required**:
- Keys in `HashMap<K, V>` (must implement `Eq`)

**Conditions**: Must also implement `PartialEq`. Cannot be implemented for types with non-reflexive equality (e.g., floating-point numbers).

```rust
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,  // Note: i32, not f32 (to allow Eq)
    y: i32,
}

let p1 = Point { x: 5, y: 10 };
let p2 = Point { x: 5, y: 10 };
assert_eq!(p1, p2);  // Uses PartialEq::eq
```

### `PartialOrd` and `Ord` - Ordering Comparisons

**`PartialOrd` Purpose**: Enable `<`, `>`, `<=`, `>=` operators.

**Generated behavior**:
- **Structs**: Compare fields in declaration order (lexicographic)
- **Enums**: Earlier variants are less than later variants

**When required**:
- Sorting and ordering operations
- Range generation with `gen_range` (from `rand` crate)

**Returns**: `Option<Ordering>` - `None` if values can't be ordered (e.g., `NAN`).

**Conditions**: Must also implement `PartialEq`.

**`Ord` Purpose**: Guarantee that any two values have a valid ordering.

**Returns**: `Ordering` (not `Option<Ordering>`) - always produces an ordering.

**When required**:
- Keys in `BTreeSet<T>` and `BTreeMap<K, V>`
- Any algorithm requiring total ordering

**Conditions**: Must also implement `PartialOrd`, `Eq`, and `PartialEq`.

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

let v1 = Version { major: 1, minor: 0, patch: 0 };
let v2 = Version { major: 1, minor: 2, patch: 0 };
assert!(v1 < v2);  // Compares major, then minor, then patch
```

### `Clone` - Explicit Duplication

**Purpose**: Create a deep copy of a value with explicit `.clone()` call.

**Generated behavior**: Calls `clone()` on each field/value recursively.

**When required**:
- `to_vec()` method on slices
- Explicit duplication when ownership transfer isn't desired
- Working with `Rc<T>` or `Arc<T>`

**Conditions**: All fields must implement `Clone`.

**Cost**: May involve heap allocation and arbitrary code execution (deep copy).

```rust
#[derive(Clone)]
struct Buffer {
    data: Vec<u8>,
    size: usize,
}

let buf1 = Buffer { data: vec![1, 2, 3], size: 3 };
let buf2 = buf1.clone();  // Explicit duplication
```

### `Copy` - Implicit Bitwise Duplication

**Purpose**: Enable implicit copying (no move semantics).

**Generated behavior**: Bitwise copy (no custom code execution).

**When allowed**:
- All fields implement `Copy`
- Type doesn't own heap resources
- Simple stack-only data

**Conditions**: Must also implement `Clone`. Cannot contain non-`Copy` types.

**Benefits**:
- Values duplicated automatically on assignment
- No need to call `.clone()`
- Performance optimization

```rust
#[derive(Copy, Clone)]  // Copy requires Clone
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 5, y: 10 };
let p2 = p1;  // Implicit copy, p1 still valid
println!("{}", p1.x);  // Works because Point is Copy
```

### `Hash` - Hashing for Collections

**Purpose**: Map a value to a fixed-size hash for use in hash-based collections.

**Generated behavior**: Combines hash of all fields.

**When required**:
- Keys in `HashMap<K, V>` and `HashSet<T>`
- Custom hash-based data structures

**Conditions**: All fields must implement `Hash`.

```rust
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct UserId(u64);

let mut users = HashMap::new();
users.insert(UserId(42), "Alice");
```

### `Default` - Default Values

**Purpose**: Create a default (initial) value for a type.

**Generated behavior**: Calls `default()` on each field.

**When required**:
- Struct update syntax: `Struct { field: value, ..Default::default() }`
- `unwrap_or_default()` on `Option<T>`
- Builder patterns and initialization

**Conditions**: All fields must implement `Default`.

```rust
#[derive(Default)]
struct Config {
    timeout: u64,      // Defaults to 0
    retries: u32,      // Defaults to 0
    verbose: bool,     // Defaults to false
}

let config = Config {
    timeout: 5000,
    ..Default::default()  // Use default for retries and verbose
};
```

## Trait Dependencies

Some traits require others to be implemented:

```
Eq requires PartialEq
Ord requires PartialOrd + Eq + PartialEq
Copy requires Clone
```

## Common Derive Combinations

```rust
// Basic struct (debugging + equality)
#[derive(Debug, PartialEq)]
struct Data { /* ... */ }

// HashMap key (hashing + equality)
#[derive(Hash, PartialEq, Eq)]
struct Key { /* ... */ }

// BTreeMap key (total ordering)
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SortedKey { /* ... */ }

// Copyable value type (small stack types)
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

// Complete derive set for a configuration struct
#[derive(Debug, Clone, PartialEq, Default)]
struct Config { /* ... */ }
```

## Non-Derivable Traits

Some traits cannot be derived because they require custom logic:

- **`Display`**: User-facing formatting requires human-readable decisions
- **`Deref`/`DerefMut`**: Requires specifying target type
- **`Drop`**: Cleanup logic is type-specific
- **`Iterator`**: Iteration logic varies by type
- **`From`/`Into`**: Conversion logic is context-dependent

## Custom Derive Macros

Libraries can provide custom derive macros:

```rust
// Example from serde
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}
```

See Chapter 20 for implementing custom derive macros using procedural macros.

## Key Insights

- **DRY principle**: `derive` eliminates boilerplate for common patterns
- **Consistency**: Derived implementations follow predictable rules
- **Composability**: Traits work together (e.g., `Hash` with `Eq` for hash maps)
- **Constraints**: Derivability propagates - all fields must support the trait
- **Extensibility**: Libraries can add custom derivable traits via procedural macros

## Decision Guide: Which Traits to Derive?

| Use Case | Derive |
|----------|--------|
| **Debugging/development** | `Debug` |
| **Value comparison** | `PartialEq` (+ `Eq` if no floats) |
| **Sorting/ordering** | `PartialOrd` (+ `Ord` if total order) |
| **HashMap/HashSet keys** | `Hash, Eq, PartialEq` |
| **BTreeMap/BTreeSet keys** | `Ord, PartialOrd, Eq, PartialEq` |
| **Small stack types** | `Copy, Clone` |
| **Large/heap types** | `Clone` only |
| **Configuration structs** | `Default, Debug, Clone` |

---

**Book Reference**: [Appendix C: Derivable Traits](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html)

**Zettelkasten Links**: [[derivable-traits-summary]] | [[trait-implementation]] | [[hash-maps-and-sets]]
