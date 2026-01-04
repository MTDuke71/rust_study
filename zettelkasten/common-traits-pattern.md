# Common Traits Pattern - Unsurprising Type Design

**The "Principle of Least Surprise" applied to Rust trait implementations**

> **Core Philosophy**: Types should behave the way users expect. Implement common traits thoughtfully to create intuitive, ergonomic APIs that feel natural to Rust developers.

**Source**: Rust for Rustaceans Ch3 - Designing Interfaces (pp. 38-40)

**Related**: [[Traits]], [[deref-trait]], [[wrapper-pattern]], [[rust-api-guidelines]], [[ergonomic-apis]]

---

## 🎯 The Unsurprising Principle

**Users have expectations** based on Rust's standard library and ecosystem conventions. Violating these expectations creates friction and bugs.

### **Expected Trait Patterns**

| **Type Category** | **Expected Traits** | **Why** |
|-------------------|---------------------|---------|
| **Value types** (Point, Color, UserId) | `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash` | Behave like primitive types |
| **Collections** (Vec, HashMap) | `Debug`, `Clone`, `PartialEq`, `Default`, `IntoIterator` | Standard collection operations |
| **Error types** | `Debug`, `Display`, `Error` | Integrate with `?` operator and error handling |
| **Builder types** | `Debug`, `Default` | Initialization and debugging |
| **Smart pointers** | `Deref`, `Drop`, `Debug` | Transparent access + cleanup |
| **Newtype wrappers** | Same as wrapped type + custom traits | Maintain wrapped type's behavior |

---

## 📋 Common Trait Combinations

### **Pattern 1: Value-Like Types**

**Use Case**: Types that represent values (not resources) and should be copyable.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Score(u32);
```

**Rationale**:
- `Debug` - **Always** for value types (troubleshooting)
- `Clone` - If copying makes sense semantically
- `Copy` - For small types where copy is cheap (≤ 2 pointers)
- `PartialEq`/`Eq` - If equality comparison is meaningful
- `Hash` - If type might be used as HashMap/HashSet key
- `PartialOrd`/`Ord` - If natural ordering exists

**⚠️ Warning**: Don't implement `Copy` for large types or types managing resources!

---

### **Pattern 2: Container/Collection Types**

**Use Case**: Types that hold other values and should behave like standard collections.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<(usize, usize)>,
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl<T> IntoIterator for Graph<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}
```

**Rationale**:
- `Debug` - **Essential** for debugging collection contents
- `Clone` - If deep copying makes sense (watch performance!)
- `PartialEq` - Compare collections element-wise
- `Default` - `new()` vs `default()` - provide both for ergonomics
- `IntoIterator` - For `for` loops and iterator chains
- ❌ **Not** `Copy` - Collections are too large
- ❌ **Not** `Hash` - Usually meaningless for collections

---

### **Pattern 3: Error Types**

**Use Case**: Custom error types for domain-specific failures.

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    InvalidFormat(String),
    MissingField(&'static str),
    OutOfRange { value: i64, min: i64, max: i64 },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidFormat(s) => 
                write!(f, "Invalid format: {}", s),
            ParseError::MissingField(field) => 
                write!(f, "Missing required field: {}", field),
            ParseError::OutOfRange { value, min, max } => 
                write!(f, "Value {} out of range [{}, {}]", value, min, max),
        }
    }
}

impl Error for ParseError {}
```

**Rationale**:
- `Debug` - **Required** by `Error` trait
- `Display` - **Required** by `Error` trait - user-facing message
- `Error` - Enables `?` operator, `source()` chaining
- `Clone` - Allows error to be stored and propagated
- ❌ **Not** `PartialEq` usually - unless testing specific error variants

**Key Insight**: `Debug` for developers, `Display` for users!

---

### **Pattern 4: Builder Types**

**Use Case**: Fluent APIs for complex object construction.

```rust
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn build(self) -> Result<Config, BuildError> {
        Ok(Config {
            host: self.host.ok_or(BuildError::MissingHost)?,
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(30),
        })
    }
}
```

**Rationale**:
- `Debug` - Inspect builder state during construction
- `Default` - `new()` calls `default()` - one implementation
- ❌ **Not** `Clone` - Builders are consumed by `build()`
- ❌ **Not** `PartialEq` - Meaningless for intermediate state

---

### **Pattern 5: Newtype Wrappers**

**Use Case**: Type-safe wrappers around primitives or standard types.

```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Meters(pub f64);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}

// Ergonomic creation
impl From<f64> for Meters {
    fn from(value: f64) -> Self {
        Meters(value)
    }
}

// Math operations
impl std::ops::Add for Meters {
    type Output = Meters;
    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}
```

**Rationale**:
- **Inherit wrapped type's traits** where semantically valid
- `Display` - User-facing representation with units
- `From<T>` - Ergonomic conversions
- Operator traits (`Add`, `Sub`, etc.) - Natural math operations
- Consider `Deref` if wrapper should be transparent (see [[deref-trait]])

**⚠️ Warning**: Don't blindly derive all traits - only those that make sense!

---

## 🔧 Trait Selection Decision Tree

### **Should I implement `Debug`?**

✅ **YES** - Always, unless security-sensitive data (use custom impl that redacts)

### **Should I implement `Clone`?**

- ✅ YES if: Type is value-like, copying makes semantic sense
- ❌ NO if: Type manages resources (file handles, network connections)
- 🤔 MAYBE: For large collections - document performance implications

### **Should I implement `Copy`?**

- ✅ YES if: Type is ≤ 16 bytes AND all fields are `Copy` AND copying is cheap
- ❌ NO if: Type has heap allocations, owns resources, or is large

**Rule**: `Copy` requires `Clone` - if you can't clone cheaply, don't implement `Copy`

### **Should I implement `PartialEq`/`Eq`?**

- ✅ YES if: Equality comparison is meaningful (value semantics)
- ❌ NO if: Identity matters more than value (e.g., file handles)

**Rule**: If `PartialEq` is total (reflexive, symmetric, transitive), also implement `Eq`

### **Should I implement `Hash`?**

- ✅ YES if: Type might be used as HashMap/HashSet key
- **MUST** if: Implementing `Eq` (required for `HashMap`)
- ❌ NO if: Type is mutable or equality isn't well-defined

**Invariant**: If `a == b`, then `hash(a) == hash(b)` - **always**!

### **Should I implement `PartialOrd`/`Ord`?**

- ✅ YES if: Natural ordering exists (numbers, timestamps, priorities)
- 🤔 MAYBE: For domain types - document ordering semantics
- ❌ NO if: Multiple valid orderings exist (use named methods instead)

### **Should I implement `Default`?**

- ✅ YES if: Type has obvious "zero" or "empty" value
- 🤔 MAYBE: For builder types (empty builder)
- ❌ NO if: No sensible default exists (require explicit construction)

---

## 🎨 Ergonomic Implementations

### **Pattern: Display vs Debug**

```rust
use std::fmt;

pub struct User {
    id: u64,
    name: String,
    email: String,
}

// Debug: For developers - show structure
impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("email", &self.email)
            .finish()
    }
}

// Display: For users - friendly representation
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

// Usage
let user = User { id: 1, name: "Alice".into(), email: "alice@example.com".into() };
println!("{}", user);    // Alice <alice@example.com>
println!("{:?}", user);  // User { id: 1, name: "Alice", email: "alice@example.com" }
```

**Guideline**: 
- `Debug` - Machine-readable, show all fields
- `Display` - Human-readable, show what matters to users

---

### **Pattern 6: Marker Traits (Send/Sync)**

**Use Case**: Types that need to work with concurrency and multi-threading.

```rust
use std::sync::Arc;
use std::thread;

// Automatically Send + Sync (all fields are Send + Sync)
#[derive(Debug, Clone)]
pub struct Config {
    host: String,
    port: u16,
}

// Can share across threads safely
let config = Arc::new(Config { host: "localhost".into(), port: 8080 });
let config_clone = Arc::clone(&config);

thread::spawn(move || {
    println!("Host: {}", config_clone.host);  // ✅ Config is Send + Sync
});

// Type that is NOT Send/Sync (contains Rc)
use std::rc::Rc;

pub struct NotThreadSafe {
    data: Rc<String>,  // Rc is !Send, !Sync
}

// Explicitly opt-out of auto traits if needed
pub struct ManualSync {
    ptr: *const i32,  // Raw pointer is !Send, !Sync by default
}

// UNSAFE: Manually implement Send if you guarantee thread safety
unsafe impl Send for ManualSync {}
unsafe impl Sync for ManualSync {}
```

**Marker Traits**:
- `Send` - Type can be **transferred** across thread boundaries (ownership moves)
- `Sync` - Type can be **referenced** from multiple threads (`&T` is `Send`)
- Automatically derived for types where all fields are `Send`/`Sync`
- **Raw pointers** are `!Send` and `!Sync` by default (safety!)

**Common Patterns**:

| Type | Send | Sync | Why |
|------|------|------|-----|
| `i32`, `String`, `Vec<T>` | ✅ | ✅ | No shared state |
| `Rc<T>` | ❌ | ❌ | Non-atomic reference counting |
| `Arc<T>` | ✅ | ✅ | Atomic reference counting |
| `Cell<T>`, `RefCell<T>` | ✅ | ❌ | Interior mutability without atomics |
| `Mutex<T>`, `RwLock<T>` | ✅ | ✅ | Thread-safe interior mutability |
| `*const T`, `*mut T` | ❌ | ❌ | Raw pointers (no safety guarantees) |

**When to Think About Send/Sync**:
- Building concurrent data structures
- Sharing state across threads
- Working with `Arc`, `Mutex`, channels
- FFI with C libraries

**Rule**: Most types are automatically `Send + Sync`. Only worry about this when:
1. Using `Rc`, `Cell`, `RefCell` (single-threaded primitives)
2. Working with raw pointers
3. Interfacing with C code
4. Building custom concurrent types

**⚠️ Warning**: Implementing `Send` or `Sync` manually requires `unsafe` - you're making safety guarantees to the compiler!

---

### **Pattern: Forwarding Implementations**

When wrapping types, forward trait implementations appropriately:

```rust
#[derive(Debug)]
pub struct TrackedVec<T> {
    inner: Vec<T>,
    access_count: std::cell::Cell<usize>,
}

// Forward Clone to inner Vec (tracking doesn't copy)
impl<T: Clone> Clone for TrackedVec<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            access_count: std::cell::Cell::new(0), // Reset counter!
        }
    }
}

// Forward PartialEq to inner Vec (tracking doesn't affect equality)
impl<T: PartialEq> PartialEq for TrackedVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
        // Deliberately ignore access_count in comparison
    }
}

// Forward Deref for transparent access (see [[deref-trait]])
impl<T> std::ops::Deref for TrackedVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        self.access_count.set(self.access_count.get() + 1);
        &self.inner
    }
}
```

**Key Decision**: Which wrapper fields participate in trait semantics?

---

## 🚨 Common Anti-Patterns

### **❌ Anti-Pattern 1: Copy on Large Types**

```rust
// BAD: Copy on 32-byte struct
#[derive(Clone, Copy)]  // ❌ Don't do this!
struct LargeData {
    buffer: [u8; 32],
    metadata: (u64, u64),
}

// GOOD: Clone only, Copy is expensive
#[derive(Clone)]  // ✅ Explicit cloning
struct LargeData {
    buffer: [u8; 32],
    metadata: (u64, u64),
}
```

**Rule of Thumb**: Don't implement `Copy` for types > 16 bytes

---

### **❌ Anti-Pattern 2: Implementing Eq Without Hash**

```rust
// BAD: Can compare but not hash
#[derive(PartialEq, Eq)]  // ❌ Useless in HashMap!
struct Point {
    x: i32,
    y: i32,
}

// GOOD: Both Eq and Hash
#[derive(PartialEq, Eq, Hash)]  // ✅ Can use as key
struct Point {
    x: i32,
    y: i32,
}
```

**Rule**: If `Eq`, almost always want `Hash` too

---

### **❌ Anti-Pattern 3: Partial Trait Implementation**

```rust
// BAD: Implements some but not all related traits
impl PartialEq for MyType { /* ... */ }
// Missing Eq even though equality is total!

// GOOD: Complete trait family
impl PartialEq for MyType { /* ... */ }
impl Eq for MyType {}  // Marker trait asserting totality
```

**Trait Families**:
- `PartialEq` → `Eq` (if total)
- `PartialOrd` → `Ord` (if total)
- `Clone` → `Copy` (if cheap)

---

### **❌ Anti-Pattern 4: Breaking Hash/Eq Invariant**

```rust
// BAD: Hash doesn't match Eq
impl PartialEq for BadType {
    fn eq(&self, other: &Self) -> bool {
        self.field1 == other.field1  // Only compares field1
    }
}

impl Hash for BadType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.field1.hash(state);
        self.field2.hash(state);  // ❌ Hashes field2 but Eq ignores it!
    }
}

// GOOD: Consistent Eq and Hash
impl PartialEq for GoodType {
    fn eq(&self, other: &Self) -> bool {
        self.field1 == other.field1 && self.field2 == other.field2
    }
}

impl Hash for GoodType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.field1.hash(state);
        self.field2.hash(state);  // ✅ Same fields in Eq and Hash
    }
}
```

**Invariant**: Fields used in `eq()` **must** be the same as fields in `hash()`

---

## 🎯 Mission Integration Examples

### **Mission 1: Stack Traits**

```rust
// Value-like type - full trait suite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackConfig {
    capacity: usize,
    allow_resize: bool,
}

// Collection type - partial trait suite
#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()  // Forward to new()
    }
}
```

---

### **Mission 5: HashMap Traits**

```rust
// Keys MUST have Eq + Hash
pub struct HashMap<K: Eq + Hash, V> {
    // ...
}

// Entry wrapper inherits Debug for debugging
#[derive(Debug)]
pub struct Entry<'a, K, V> {
    // ...
}
```

---

### **Mission 10: Union-Find Traits**

```rust
// Value-like ID type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

// Resource-managing collection
#[derive(Debug, Clone)]  // Clone is deep copy - expensive!
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

// NO Copy - managing Vec resources
// NO PartialEq - structural equality is meaningless
// NO Hash - not used as key
```

---

## 📊 AoC Pattern: Input Wrappers

```rust
#[derive(Debug, Clone)]
pub struct AocInput {
    raw: String,
    year: u16,
    day: u8,
}

impl AocInput {
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.raw.lines()
    }
}

// Display for logging
impl fmt::Display for AocInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AoC {}/{:02} ({} bytes)", self.year, self.day, self.raw.len())
    }
}

// Deref to str for convenience (see [[deref-trait]])
impl std::ops::Deref for AocInput {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
```

**Trait Choices**:
- ✅ `Debug` - Essential for troubleshooting
- ✅ `Clone` - Might need to reparse input
- ✅ `Display` - Logging and output
- ✅ `Deref` - Transparent string access
- ❌ `Copy` - String is heap-allocated
- ❌ `PartialEq` - Not comparing inputs
- ❌ `Hash` - Not using as key

---

## 🔍 Checklist for New Types

When creating a new type, ask:

- [ ] **Debug**: Can I print this for debugging? → Almost always YES
- [ ] **Clone**: Does copying make semantic sense? → Value types: YES, Resources: NO
- [ ] **Copy**: Is it ≤ 16 bytes AND cheap to copy? → Primitives: YES, Heap types: NO
- [ ] **PartialEq/Eq**: Is equality meaningful? → Value types: YES, Identity types: NO
- [ ] **Hash**: Will this be used in HashMap/HashSet? → If Eq: probably YES
- [ ] **PartialOrd/Ord**: Is there natural ordering? → Numbers: YES, Arbitrary: NO
- [ ] **Default**: Is there an obvious "empty" value? → Builders: YES, Required data: NO
- [ ] **Display**: Do users need to see this? → Errors/output: YES, Internal: NO
- [ ] **Deref**: Is this a smart pointer? → See [[deref-trait]] for guidance
- [ ] **Send/Sync**: Will this be shared across threads? → Usually automatic, check with `Rc`/`Cell`/raw pointers

---

## 📚 Reference Tables

### **Trait Implementation Costs**

| Trait | Derive Cost | Manual Cost | Runtime Cost |
|-------|-------------|-------------|--------------|
| `Debug` | Free | Low | None (debug builds only) |
| `Clone` | Free | Medium | O(n) for deep copies |
| `Copy` | Free | N/A | Zero (bitwise copy) |
| `PartialEq` | Free | Low | O(n) field comparisons |
| `Eq` | Free | None | Zero (marker trait) |
| `Hash` | Free | Medium | O(n) field hashing |
| `Default` | N/A | Low | Depends on initialization |
| `Display` | N/A | Medium | String formatting |

### **Common Trait Combinations**

```rust
// Primitive-like value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]

// Complex value (no Copy)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

// Ordered value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

// Collection
#[derive(Debug, Clone, PartialEq, Default)]

// Builder
#[derive(Debug, Default)]

// Error
#[derive(Debug, Clone)]  // + Display + Error manual impls
```

---

## 🔗 Related Concepts

### **Foundation**
- [[Traits]] - Complete trait reference and mechanics
- [[deref-trait]] - Transparent wrapper pattern
- [[wrapper-pattern]] - Type-safe wrappers

### **Design Philosophy**
- [[rust-api-guidelines]] - Official API design guidelines
- [[ergonomic-apis]] - Creating pleasant Rust APIs
- [[zero-cost-abstractions]] - Performance implications

### **Practical Applications**
- [[mission-3]] - Trait bounds in binary search
- [[mission-5]] - Hash and Eq requirements
- [[error-handling-patterns]] - Error trait implementations
- [[AoC Patterns MOC]] - Input wrappers and utility types

---

## 🎓 Learning Progression

1. **Understand trait semantics** - What each trait means conceptually
2. **Learn common patterns** - Value types, collections, errors, builders
3. **Apply decision tree** - Systematic trait selection process
4. **Recognize anti-patterns** - Avoid common mistakes
5. **Refine through usage** - Discover what users actually need

**From Rust for Rustaceans**: "The traits you implement communicate your type's semantics. Choose wisely to create unsurprising, ergonomic APIs."

---

*Tags: #traits #design-patterns #ergonomics #api-design #rust-for-rustaceans #common-traits #derive #unsurprising*

*Links: [[zettel-index]] | [[Traits]] | [[deref-trait]] | [[wrapper-pattern]] | [[rust-api-guidelines]] | [[rust-for-rustaceans-ch3]]*
