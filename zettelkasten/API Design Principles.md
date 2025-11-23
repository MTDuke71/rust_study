# 🎯 API Design Principles in Rust

**Fundamental principles and guidelines for designing robust, maintainable, and idiomatic Rust APIs**

---

## 📖 Core Concept

API Design Principles are the **foundational guidelines** that inform how we create public interfaces in Rust. While [[API Design Patterns]] provide concrete, reusable solutions, principles offer abstract rules and philosophies that guide decision-making across all API design scenarios.

**Key Distinction:**
- **Principles** → *Why* and *what* (guiding philosophy)
- **Patterns** → *How* (concrete implementation)

---

## 🏛️ **Rust's Core API Design Principles**

### **1. Zero-Cost Abstractions**
*"What you don't use, you don't pay for. And what you do use, you couldn't hand code any better."*

**Principle:** High-level APIs should compile to the same machine code as hand-written low-level code.

```rust
// ✅ Iterator chain compiles to efficient loop
let sum: i32 = vec![1, 2, 3, 4, 5]
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 2)
    .sum();

// Equivalent to hand-written loop - no runtime cost
let mut sum = 0;
for &x in &vec![1, 2, 3, 4, 5] {
    if x % 2 == 0 {
        sum += x * 2;
    }
}
```

**Application in Mission5 (HashMap):**
- Generic type parameters compile to specialized code
- No vtable overhead unless using trait objects
- Inline optimization of small generic functions

---

### **2. Explicit Over Implicit**
*"Make behavior visible, predictable, and intentional."*

**Principle:** APIs should make their effects and costs explicit to the caller.

```rust
// ✅ GOOD: Explicit mutation, allocation, and error handling
pub fn parse_config(content: &str) -> Result<Config, ParseError> {
    // Explicit: might fail
}

pub fn add_entry(&mut self, entry: Entry) {
    // Explicit: mutates self
}

pub fn to_owned_copy(&self) -> Vec<u8> {
    // Explicit: allocates new memory
}

// ❌ BAD: Hidden behavior
pub fn parse_config(content: &str) -> Config {
    // Hidden: can panic (implicit error)
}

pub fn add_entry(&self, entry: Entry) {
    // Hidden: uses interior mutability (unexpected mutation)
}

pub fn get_data(&self) -> Vec<u8> {
    // Hidden: does this allocate or return reference?
}
```

**Mission Applications:**
- **Mission1 (Stack):** `pop()` returns `Option<T>` explicitly handling empty case
- **Mission3 (Binary Search):** Returns `Result<usize, usize>` showing success/failure
- **Mission5 (HashMap):** `insert()` returns `Option<V>` showing if value was replaced

---

### **3. Make Illegal States Unrepresentable**
*"Use the type system to prevent errors at compile time."*

**Principle:** Design types so invalid states cannot be constructed.

```rust
// ❌ BAD: Can represent invalid state
pub struct Connection {
    socket: Option<TcpStream>,
    is_connected: bool,  // Can be out of sync with socket
}

// ✅ GOOD: Type system enforces valid states
pub enum Connection {
    Connected(TcpStream),
    Disconnected,
}

// Or use builder pattern with type states
pub struct ConnectionBuilder<State> {
    _state: PhantomData<State>,
    host: String,
}

pub struct Configured;
pub struct Connected;

impl ConnectionBuilder<Configured> {
    pub fn connect(self) -> Result<ConnectionBuilder<Connected>, Error> {
        // Can only call connect after configuration
    }
}
```

**Mission3 Application (Binary Search):**
```rust
// Sorted wrapper ensures precondition
pub struct Sorted<T>(Vec<T>);

impl<T: Ord> Sorted<T> {
    pub fn new(mut vec: Vec<T>) -> Self {
        vec.sort();
        Sorted(vec)
    }
    
    pub fn binary_search(&self, target: &T) -> Result<usize, usize> {
        // Guaranteed sorted - no runtime check needed
    }
}
```

---

### **4. Pay-for-What-You-Use**
*"Don't impose costs on users who don't need the feature."*

**Principle:** Optional functionality should not burden users who don't need it.

```rust
// ✅ GOOD: Features are opt-in
pub struct HashMap<K, V, S = RandomState> {
    // Default hasher, but can be customized
}

impl<K, V> HashMap<K, V, RandomState> {
    pub fn new() -> Self { /* ... */ }
}

impl<K, V, S> HashMap<K, V, S> 
where 
    S: BuildHasher 
{
    pub fn with_hasher(hasher: S) -> Self { /* ... */ }
}

// Users pay for custom hashers only if they use them
```

**AoC Applications:**
- Use `Vec<T>` when capacity is unknown (pays for dynamic allocation)
- Use `[T; N]` when size is fixed (zero allocation cost)
- Use `&str` when ownership not needed (no allocation)

---

### **5. Composability**
*"Small, focused APIs that work together seamlessly."*

**Principle:** Design APIs that can be combined to create more complex behavior.

```rust
// ✅ Iterator methods compose elegantly
let result: Vec<_> = data
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.process())
    .take(10)
    .collect();

// Each method:
// - Takes self/&self
// - Returns Self or new iterator type
// - Single responsibility
```

**Mission6 Application (Iterator Patterns):**
```rust
pub trait CustomIterator: Iterator {
    fn custom_filter<P>(self, predicate: P) -> CustomFilter<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
        Self: Sized,
    {
        CustomFilter::new(self, predicate)
    }
}

// Composes with standard library iterators
```

---

### **6. Principle of Least Surprise**
*"Follow conventions and user expectations."*

**Principle:** APIs should behave as users expect based on naming and Rust conventions.

```rust
// ✅ GOOD: Follows conventions
impl<T> Vec<T> {
    pub fn new() -> Self { /* ... */ }           // Constructor, never fails
    pub fn len(&self) -> usize { /* ... */ }     // O(1), no side effects
    pub fn push(&mut self, value: T) { /* ... */ } // Clearly mutating
    pub fn iter(&self) -> Iter<T> { /* ... */ }  // Returns iterator
}

// ❌ BAD: Surprising behavior
impl<T> Vec<T> {
    pub fn create() -> Result<Self, ()> { /* ... */ } // new() should not fail
    pub fn size(&self) -> usize { /* ... */ }          // Should be len()
    pub fn add(&self, value: T) { /* ... */ }          // Mutates without &mut?
    pub fn to_iter(&self) -> Vec<&T> { /* ... */ }     // Confusing name/return
}
```

**Standard Naming Conventions:**
- `new()` → Constructor (doesn't fail)
- `with_*()` → Alternative constructor
- `from_*()` → Conversion constructor
- `to_*()` → Expensive conversion (allocates)
- `as_*()` → Cheap conversion (reference)
- `into_*()` → Consuming conversion (moves)
- `is_*()`, `has_*()` → Boolean predicates
- `get_*()` → Getter, returns `Option` if might fail

---

### **7. Memory Safety Without Garbage Collection**
*"Compile-time guarantees, zero runtime overhead."*

**Principle:** APIs enforce memory safety through ownership and borrowing rules.

```rust
// ✅ GOOD: Clear ownership semantics
pub fn take_ownership(data: Vec<u8>) { /* consumes data */ }
pub fn borrow_data(data: &[u8]) { /* reads data */ }
pub fn mutate_data(data: &mut Vec<u8>) { /* modifies data */ }

// Compiler enforces:
let data = vec![1, 2, 3];
borrow_data(&data);       // OK: shared reference
mutate_data(&mut data);   // ERROR: can't mut borrow after immut borrow
take_ownership(data);     // OK: moves ownership
// data is now invalid - compiler prevents use-after-move
```

**Mission Applications:**
- **Mission1:** Stack owns its data, provides borrowing methods
- **Mission4:** LinkedList manages node lifetimes safely
- **Mission5:** HashMap manages heap-allocated buckets

---

### **8. Errors Are Values**
*"Make error handling explicit and composable."*

**Principle:** Represent failures as values (`Result`, `Option`), not exceptions.

```rust
// ✅ GOOD: Explicit error handling
pub fn parse_config(path: &Path) -> Result<Config, ConfigError> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

// Caller must handle errors explicitly
match parse_config(Path::new("config.toml")) {
    Ok(config) => { /* use config */ },
    Err(e) => { /* handle error */ },
}

// Or propagate with ?
let config = parse_config(path)?;
```

**Error Design Guidelines:**
- Use `Result<T, E>` for recoverable errors
- Use `Option<T>` for absence of value (not an error)
- Use `panic!` only for unrecoverable programmer errors
- Provide custom error types with context

**Mission3 Application:**
```rust
pub enum BinarySearchError {
    EmptyArray,
    NotSorted,
}

pub fn binary_search<T: Ord>(
    arr: &[T], 
    target: &T
) -> Result<usize, BinarySearchError> {
    if arr.is_empty() {
        return Err(BinarySearchError::EmptyArray);
    }
    // search logic
}
```

---

### **9. Trait Coherence and Consistency**
*"One implementation per type per trait."*

**Principle:** Rust's orphan rules and trait coherence ensure consistent behavior.

```rust
// ✅ GOOD: Implement standard traits consistently
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

// If you implement PartialEq, consider Eq
// If you implement PartialOrd, consider Ord
// If you implement Clone, consider Copy (if applicable)

// Trait implementations should be coherent
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.cmp(&other.x) {
            Ordering::Equal => Some(self.y.cmp(&other.y)),
            other => Some(other),
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // Must be consistent with PartialOrd
        self.partial_cmp(other).unwrap()
    }
}
```

**Standard Trait Guidelines:**
- `Debug` → Almost always implement (debugging)
- `Clone` → If cheap to clone or necessary
- `Copy` → Only for small, stack-only types
- `PartialEq/Eq` → For value comparison
- `PartialOrd/Ord` → For ordering
- `Hash` → If used as map key
- `Default` → For sensible default construction

---

### **10. Progressive Disclosure**
*"Simple things simple, complex things possible."*

**Principle:** Provide simple defaults with escape hatches for advanced use.

```rust
// ✅ Simple case: easy
let map = HashMap::new();
map.insert("key", "value");

// ✅ Advanced case: possible
let map = HashMap::with_capacity_and_hasher(
    100,
    RandomState::new()
);

// ✅ Expert case: full control
let map = HashMap::with_hasher(MyCustomHasher::new());
```

**Mission5 Application (HashMap):**
```rust
// Level 1: Simple API
impl<K, V> HashMap<K, V> {
    pub fn new() -> Self { /* ... */ }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> { /* ... */ }
    pub fn get(&self, k: &K) -> Option<&V> { /* ... */ }
}

// Level 2: Performance optimization
impl<K, V> HashMap<K, V> {
    pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
    pub fn reserve(&mut self, additional: usize) { /* ... */ }
}

// Level 3: Advanced customization
impl<K, V, S> HashMap<K, V, S> where S: BuildHasher {
    pub fn with_hasher(hash_builder: S) -> Self { /* ... */ }
}
```

---

## 🎯 **SOLID Principles in Rust Context**

### **Single Responsibility Principle (SRP)**
*Each type/module should have one reason to change.*

```rust
// ✅ GOOD: Separated concerns
pub struct User {
    id: UserId,
    name: String,
    email: String,
}

pub struct UserRepository {
    // Handles persistence
}

pub struct UserValidator {
    // Handles validation
}

// ❌ BAD: Mixed concerns
pub struct User {
    id: UserId,
    name: String,
    db_connection: DbConnection,  // Persistence concern
    validation_rules: Rules,       // Validation concern
}
```

### **Open/Closed Principle (OCP)**
*Open for extension, closed for modification.*

```rust
// ✅ GOOD: Extensible via traits
pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle { radius: f64 }
pub struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

// Can add new shapes without modifying existing code
```

### **Liskov Substitution Principle (LSP)**
*Subtypes must be substitutable for their base types.*

```rust
// ✅ GOOD: All implementors satisfy trait contract
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Any Iterator implementation can be used where Iterator is expected
fn process<I: Iterator>(iter: I) {
    // Works with any Iterator implementation
}
```

### **Interface Segregation Principle (ISP)**
*Clients shouldn't depend on methods they don't use.*

```rust
// ✅ GOOD: Small, focused traits
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

// Implement only what's needed
impl Read for TcpStream { /* ... */ }
impl Write for TcpStream { /* ... */ }
// TcpStream doesn't implement Seek (not seekable)

// ❌ BAD: Monolithic trait
pub trait IO {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>; // Not all IO is seekable!
}
```

### **Dependency Inversion Principle (DIP)**
*Depend on abstractions, not concretions.*

```rust
// ✅ GOOD: Depends on trait (abstraction)
pub struct Service<S: Storage> {
    storage: S,
}

pub trait Storage {
    fn save(&mut self, data: &[u8]) -> Result<(), Error>;
    fn load(&self) -> Result<Vec<u8>, Error>;
}

// Can use different storage implementations
impl Service<FileStorage> { /* ... */ }
impl Service<MemoryStorage> { /* ... */ }
impl Service<DatabaseStorage> { /* ... */ }

// ❌ BAD: Depends on concrete type
pub struct Service {
    storage: FileStorage,  // Locked into one implementation
}
```

---

## 🔍 **Additional Rust-Specific Principles**

### **11. Leverage the Type System**
*Use types to encode invariants and contracts.*

```rust
// ✅ GOOD: Type-level guarantees
pub struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn new(first: T) -> Self {
        NonEmptyVec { first, rest: Vec::new() }
    }
    
    pub fn first(&self) -> &T {
        &self.first  // Always succeeds - no Option needed
    }
}
```

### **12. Provide Escape Hatches**
*Safe by default, unsafe when necessary.*

```rust
// ✅ Safe API for common case
pub fn get(&self, index: usize) -> Option<&T> {
    if index < self.len() {
        Some(&self.data[index])
    } else {
        None
    }
}

// ✅ Unsafe API for performance-critical code
pub unsafe fn get_unchecked(&self, index: usize) -> &T {
    // SAFETY: Caller must ensure index < len
    &self.data[index]
}
```

### **13. Make Common Operations Cheap**
*Optimize for the expected use case.*

```rust
// ✅ Common operations are O(1) or cheap
impl<T> Vec<T> {
    pub fn len(&self) -> usize { self.len }  // O(1)
    pub fn push(&mut self, value: T) { /* amortized O(1) */ }
    pub fn pop(&mut self) -> Option<T> { /* O(1) */ }
}

// Less common operations can be more expensive
impl<T> Vec<T> {
    pub fn insert(&mut self, index: usize, value: T) { /* O(n) */ }
    pub fn remove(&mut self, index: usize) -> T { /* O(n) */ }
}
```

---

## 📊 **Applying Principles Across Missions**

### **Mission1 (Stack) - Principles Applied:**
- **Explicit:** `pop()` returns `Option<T>` showing empty state
- **Zero-cost:** Generic `Stack<T>` compiles to specialized code
- **Memory safety:** Ownership prevents use-after-free
- **Composability:** Works with iterators

### **Mission3 (Binary Search) - Principles Applied:**
- **Errors as values:** Returns `Result<usize, usize>`
- **Explicit:** Clear success/failure semantics
- **Make illegal states unrepresentable:** Could use `Sorted<T>` wrapper

### **Mission5 (HashMap) - Principles Applied:**
- **Pay-for-what-you-use:** Generic hasher parameter
- **Progressive disclosure:** Simple API, advanced options available
- **Zero-cost:** Monomorphization for generic types
- **Composability:** Implements standard traits

### **Mission9 (Pathfinding) - Principles Applied:**
- **Single responsibility:** Separate graph, algorithm, and visualization
- **Dependency inversion:** Algorithm depends on trait, not concrete graph
- **Leverage type system:** Type parameters for node/edge types

---

## 🎓 **Best Practices Summary**

### **DO:**
- ✅ Use descriptive, conventional naming
- ✅ Make mutation explicit (`&mut self`)
- ✅ Return `Result` for fallible operations
- ✅ Return `Option` for missing values
- ✅ Implement standard traits when appropriate
- ✅ Provide documentation with examples
- ✅ Use generics for reusable code
- ✅ Design APIs that are hard to misuse

### **DON'T:**
- ❌ Panic in library code (use `Result` instead)
- ❌ Use `unwrap()` in public APIs
- ❌ Hide mutation or allocation costs
- ❌ Surprise users with unconventional behavior
- ❌ Force users to pay for unused features
- ❌ Return raw pointers unless absolutely necessary
- ❌ Expose implementation details

---

## 📚 **Related Concepts**

- **[[API Design Patterns]]** - Concrete patterns implementing these principles
- **[[rust-concepts-MOC]]** - Core Rust concepts
- **[[Error Handling Patterns]]** - Applying error-as-value principle
- **[[Ownership and Borrowing]]** - Memory safety principle
- **[[Generic Programming]]** - Zero-cost abstraction principle
- **[[Traits]]** - Abstraction and composability
- **[[mission-5]]** - HashMap API design case study
- **[[Iterator Patterns]]** - Composability and zero-cost abstractions
- **[[Testing Patterns]]** - Ensuring principles are upheld

---

## 🔗 **Learning Resources**

### **Rust API Guidelines**
- [Official Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Rust Book Chapter 7 - Managing Growing Projects
- [[zettelkasten/rust_book/rust-book-ch10]] - Traits and Generics

### **Mission Applications**
- Review Mission5 HashMap for comprehensive API design
- Study Mission9 graph algorithms for trait-based design
- Examine Mission1 Stack for ownership clarity

---

*Tags: #api-design #principles #rust-patterns #solid #best-practices #design-philosophy #idiomatic-rust #type-safety*

*Links: [[API Design Patterns]] | [[rust-concepts-MOC]] | [[Error Handling Patterns]] | [[Ownership and Borrowing]] | [[Generic Programming]] | [[Traits]] | [[Iterator Patterns]] | [[zettel-index]]*
