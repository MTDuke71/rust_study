# Typestate Pattern - Compile-Time State Machines

**Type**: Concept Note  
**Source**: [[rust-for-rustaceans]] Ch2.3 Existential Types, [[impl-trait]]  
**Related**: [[zero-cost-abstractions]] | [[phantom-data]] | [[builder-pattern]] | [[state-machines]]

---

## Overview

The **Typestate Pattern** uses Rust's type system to encode object state as type parameters, making **invalid state transitions impossible to compile**. It transforms runtime errors into compile-time errors with zero runtime cost.

**Core Principle**: Different types represent different states - the compiler enforces that operations are only available in valid states.

---

## Basic Pattern

### Simple State Transition - Door Example

The classic introductory example uses a door that can be locked or unlocked:

```rust
use std::marker::PhantomData;

// States are empty structs (zero-sized types)
struct Locked;
struct Unlocked;

// Door is generic over its state
struct Door<State> {
    _state: PhantomData<State>,
}

impl Door<Locked> {
    fn new() -> Self {
        Door { _state: PhantomData }
    }
    
    fn unlock(self) -> Door<Unlocked> {
        println!("Door unlocked");
        Door { _state: PhantomData }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        println!("Door locked");
        Door { _state: PhantomData }
    }
    
    fn open(&self) {
        println!("Door opened");
    }
}
```

**Usage and Compile-Time Safety**:

```rust
let door = Door::<Locked>::new();

// ❌ COMPILE ERROR - method not found
// door.open();

// ✅ Must unlock first
let door = door.unlock();
door.open();     // Now allowed

// ✅ Can lock again
let door = door.lock();
// door.open();  // ❌ Error again
```

**Key Insight**: Errors happen at **compile time** (method not found), not runtime (panic/Result).

### More Complex Example - Authentication State

```rust
use std::marker::PhantomData;

// State marker types (zero-sized)
struct Unauthenticated;
struct Authenticated;

// Connection carries state as a type parameter
struct Connection<State> {
    url: String,
    state: PhantomData<State>,  // Zero-sized, compile-time only
}

impl Connection<Unauthenticated> {
    // Start in unauthenticated state
    pub fn new(url: String) -> Self {
        Connection {
            url,
            state: PhantomData,
        }
    }
    
    // Transition: Unauthenticated -> Authenticated
    pub fn login(self, password: &str) -> Connection<Authenticated> {
        println!("Authenticating with {}", password);
        Connection {
            url: self.url,
            state: PhantomData,
        }
    }
}

impl Connection<Authenticated> {
    // ONLY available when authenticated
    pub fn make_request(&self, endpoint: &str) -> String {
        format!("GET {}/{}", self.url, endpoint)
    }
    
    // Transition: Authenticated -> Unauthenticated
    pub fn logout(self) -> Connection<Unauthenticated> {
        Connection {
            url: self.url,
            state: PhantomData,
        }
    }
}
```

**Usage**:

```rust
let conn = Connection::new("api.example.com".into());

// ❌ COMPILE ERROR - method not found
// conn.make_request("/data");

// ✅ Must authenticate first
let auth_conn = conn.login("secret123");
auth_conn.make_request("/data");  // Now allowed

// ✅ Can transition back
let logged_out = auth_conn.logout();
// logged_out.make_request("/data");  // ❌ Error again
```

---

## PhantomData Explained

`PhantomData<T>` is a **zero-sized type** that tells the compiler "this struct acts like it owns a `T`" for type-checking purposes, without actually storing it.

```rust
use std::marker::PhantomData;

struct Container<T> {
    // We don't actually store T, but the compiler treats us like we do
    _marker: PhantomData<T>,
}

// Size proof
assert_eq!(std::mem::size_of::<Container<String>>(), 0);
assert_eq!(std::mem::size_of::<Container<Vec<i32>>>(), 0);
```

**Why PhantomData?**

Without it, the compiler complains about unused type parameters:

```rust
// ❌ ERROR: parameter `State` is never used
struct Connection<State> {
    url: String,
}

// ✅ FIX: PhantomData tells compiler State is intentional
struct Connection<State> {
    url: String,
    state: PhantomData<State>,
}
```

---

## Advanced Pattern: Multi-State Builder

Enforce that **all required fields** are set before building:

```rust
use std::marker::PhantomData;

// State markers
struct NoMethod;
struct HasMethod;
struct NoUrl;
struct HasUrl;

struct RequestBuilder<M, U> {
    method: Option<String>,
    url: Option<String>,
    headers: Vec<(String, String)>,
    _method_state: PhantomData<M>,
    _url_state: PhantomData<U>,
}

impl RequestBuilder<NoMethod, NoUrl> {
    pub fn new() -> Self {
        RequestBuilder {
            method: None,
            url: None,
            headers: vec![],
            _method_state: PhantomData,
            _url_state: PhantomData,
        }
    }
}

// Setting method transitions M from NoMethod -> HasMethod
impl<U> RequestBuilder<NoMethod, U> {
    pub fn method(self, method: impl Into<String>) -> RequestBuilder<HasMethod, U> {
        RequestBuilder {
            method: Some(method.into()),
            url: self.url,
            headers: self.headers,
            _method_state: PhantomData,
            _url_state: PhantomData,
        }
    }
}

// Setting URL transitions U from NoUrl -> HasUrl
impl<M> RequestBuilder<M, NoUrl> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<M, HasUrl> {
        RequestBuilder {
            method: self.method,
            url: Some(url.into()),
            headers: self.headers,
            _method_state: PhantomData,
            _url_state: PhantomData,
        }
    }
}

// Headers can be added in any state
impl<M, U> RequestBuilder<M, U> {
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.push((key, value));
        self
    }
}

// Build only available when BOTH method AND url are set
impl RequestBuilder<HasMethod, HasUrl> {
    pub fn build(self) -> Request {
        Request {
            method: self.method.unwrap(),
            url: self.url.unwrap(),
            headers: self.headers,
        }
    }
}

struct Request {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
}
```

**Enforced Compilation Rules**:

```rust
// ✅ VALID - all required fields set
let req = RequestBuilder::new()
    .method("GET")
    .url("/api/users")
    .header("Auth".into(), "Bearer token".into())
    .build();

// ❌ COMPILE ERROR - missing URL
let req = RequestBuilder::new()
    .method("GET")
    .build();  // Error: no method named `build`

// ❌ COMPILE ERROR - missing method
let req = RequestBuilder::new()
    .url("/api/users")
    .build();  // Error: no method named `build`

// ✅ VALID - order doesn't matter
let req = RequestBuilder::new()
    .url("/api/users")
    .method("POST")
    .build();
```

---

## Integration with impl Trait

Hide state types from public API:

```rust
pub trait Builder {
    fn build(self) -> String;
}

struct ConfigBuilder<State> {
    config: String,
    state: PhantomData<State>,
}

struct Complete;

impl Builder for ConfigBuilder<Complete> {
    fn build(self) -> String {
        self.config
    }
}

// Return impl Trait to hide ConfigBuilder<Complete>
pub fn create_builder() -> impl Builder {
    ConfigBuilder {
        config: "default config".into(),
        state: PhantomData::<Complete>,
    }
}

// Users only see the Builder trait interface
let builder = create_builder();
let config = builder.build();
```

**Benefit**: Internal state types remain implementation details.

---

## Performance Characteristics

### Zero Runtime Cost

```rust
// At compile time
struct Grid<State> {
    data: Vec<i32>,
    state: PhantomData<State>,
}

// At runtime - PhantomData disappears
assert_eq!(
    std::mem::size_of::<Grid<Initialized>>(),
    std::mem::size_of::<Vec<i32>>()
);
```

**No overhead compared to unchecked version**:
- PhantomData is zero-sized (optimized away)
- Type parameters are compile-time only
- No runtime state checks needed
- Same assembly as unsafe code

### Comparison with Runtime Validation

```rust
// ❌ RUNTIME APPROACH - errors during execution
struct Connection {
    url: String,
    authenticated: bool,
}

impl Connection {
    pub fn make_request(&self) -> Result<String, Error> {
        if !self.authenticated {
            return Err(Error::NotAuthenticated);  // Runtime check
        }
        Ok("data".into())
    }
}
// Cost: Branch prediction, error propagation, potential panic

// ✅ TYPESTATE APPROACH - errors at compile time
// (shown above with Connection<State>)
// Cost: Zero - impossible to call in invalid state
```

---

## Common Patterns

### 1. Linear State Progression

```rust
struct Uninitialized;
struct Configured;
struct Running;
struct Stopped;

struct Server<State> {
    port: u16,
    state: PhantomData<State>,
}

impl Server<Uninitialized> {
    pub fn new() -> Self { /* ... */ }
    pub fn configure(self, port: u16) -> Server<Configured> { /* ... */ }
}

impl Server<Configured> {
    pub fn start(self) -> Server<Running> { /* ... */ }
}

impl Server<Running> {
    pub fn stop(self) -> Server<Stopped> { /* ... */ }
}

// Must follow: Uninitialized -> Configured -> Running -> Stopped
```

### 2. Optional Feature States

```rust
struct WithLogging;
struct WithoutLogging;

struct App<L> {
    name: String,
    logging: PhantomData<L>,
}

impl<L> App<L> {
    pub fn enable_logging(self) -> App<WithLogging> { /* ... */ }
}

impl App<WithLogging> {
    pub fn log(&self, msg: &str) { /* ... */ }
}
// log() only available when logging is enabled
```

### 3. Transaction States (ACID)

```rust
struct Open;
struct Committed;
struct RolledBack;

struct Transaction<State> {
    changes: Vec<String>,
    state: PhantomData<State>,
}

impl Transaction<Open> {
    pub fn begin() -> Self { /* ... */ }
    pub fn add_change(&mut self, change: String) { /* ... */ }
    pub fn commit(self) -> Transaction<Committed> { /* ... */ }
    pub fn rollback(self) -> Transaction<RolledBack> { /* ... */ }
}

// Can only commit/rollback open transactions
// Cannot make changes after commit/rollback
```

---

## Mission Integration

### Mission 6: Grid Initialization Safety

```rust
struct Uninitialized;
struct Initialized;

struct Grid<T, State> {
    data: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
    state: PhantomData<State>,
}

impl<T> Grid<T, Uninitialized> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid {
            data: Vec::new(),
            rows,
            cols,
            state: PhantomData,
        }
    }
    
    pub fn initialize(mut self, default: T) -> Grid<T, Initialized> 
    where T: Clone {
        self.data = vec![vec![default; self.cols]; self.rows];
        Grid {
            data: self.data,
            rows: self.rows,
            cols: self.cols,
            state: PhantomData,
        }
    }
}

// Operations only available on initialized grids
impl<T> Grid<T, Initialized> {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row)?.get(col)
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|row| row.iter())
    }
}

// ❌ Cannot iterate uninitialized grid
let grid = Grid::<i32, Uninitialized>::new(10, 10);
// grid.iter();  // Compile error

// ✅ Must initialize first
let grid = Grid::new(10, 10).initialize(0);
grid.iter();  // Works
```

### Mission 10: UnionFind Validated Operations

```rust
struct Unvalidated;
struct Validated;

struct UnionFind<State> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    state: PhantomData<State>,
}

impl UnionFind<Unvalidated> {
    pub fn new(size: usize) -> Self { /* ... */ }
    
    pub fn validate(self) -> Result<UnionFind<Validated>, Error> {
        // Check invariants
        UnionFind {
            parent: self.parent,
            rank: self.rank,
            state: PhantomData,
        }
    }
}

// Only validated UnionFind can run queries
impl UnionFind<Validated> {
    pub fn find(&mut self, x: usize) -> usize { /* ... */ }
    pub fn union(&mut self, x: usize, y: usize) { /* ... */ }
}
```

---

## Async Rust Connection (Ch17)

Futures use typestate internally:

```rust
// Simplified Future state machine
struct NotPolled;
struct Polling;
struct Ready;

struct MyFuture<State> {
    data: String,
    state: PhantomData<State>,
}

// Transitions: NotPolled -> Polling -> Ready
// Runtime enforces through Poll<T>
```

---

## When to Use Typestate

### ✅ Use Typestate When:
- **Invalid states should be impossible** (authentication, initialization)
- **Sequential operations must be enforced** (server start/stop)
- **Builder pattern with required fields** (request builders)
- **Resource lifecycle management** (file open/close, transactions)
- **Safety-critical state transitions** (embedded systems, protocols)

### ❌ Consider Alternatives When:
- **Many possible states** (combinatorial explosion of types)
- **Dynamic state transitions** (runtime-dependent paths)
- **State doesn't affect API** (internal implementation detail)
- **Simple validation** (regular Result<T, E> is sufficient)

---

## Comparison with Other Patterns

| Pattern | Enforcement | Cost | Flexibility |
|---------|-------------|------|-------------|
| **Typestate** | Compile-time | Zero | Low (rigid transitions) |
| **Runtime bool flags** | Runtime | Branch overhead | High |
| **Result<T, E>** | Runtime | Error propagation | High |
| **Enum variants** | Compile-time (match) | Zero | Medium |
| **Invariant checks** | Runtime (panic) | Assertion cost | High |

---

## Limitations

### 1. Type Complexity

```rust
// Can become verbose with many states
struct Builder<Method, Url, Body, Auth> {
    // 4 type parameters = 2^4 = 16 possible combinations
    _m: PhantomData<Method>,
    _u: PhantomData<Url>,
    _b: PhantomData<Body>,
    _a: PhantomData<Auth>,
}
```

### 2. State Explosion

Each combination of states requires separate implementations:

```rust
// With 3 binary states (Logged/Not, Cached/Not, Validated/Not)
// Need 2^3 = 8 different impl blocks for full flexibility
```

**Mitigation**: Use sealed traits or macro-generated implementations.

### 3. Cannot Store Mixed States

```rust
// ❌ Cannot have Vec of different states
let connections: Vec<Connection<???>> = vec![
    Connection<Authenticated>,  // Different types!
    Connection<Unauthenticated>,
];

// Workaround: Use enums or trait objects
enum AnyConnection {
    Authenticated(Connection<Authenticated>),
    Unauthenticated(Connection<Unauthenticated>),
}
```

---

## Real-World Examples

### Diesel ORM (Database)

```rust
// Queries must have FROM clause before WHERE
let query = users::table      // HasFrom
    .filter(...)              // Still HasFrom, can add WHERE
    .select(...);             // Can execute

// Cannot filter before setting table
// let query = diesel::select(...).filter(...);  // Error
```

### Tokio AsyncWrite

```rust
// Must call shutdown() before closing
impl AsyncWrite {
    async fn shutdown(&mut self) -> io::Result<()>;
}
// Type system ensures proper cleanup sequence
```

### Embedded HAL (Hardware Abstraction Layer)

```rust
struct Pin<Mode> {
    pin_num: u8,
    mode: PhantomData<Mode>,
}

struct Input;
struct Output;

impl Pin<Output> {
    fn set_high(&mut self) { /* ... */ }
}

// Cannot call set_high() on input pin - compile error
```

---

## Key Takeaways

1. **Typestate = States as Types** - Different types for different states
2. **PhantomData is Zero-Sized** - No runtime cost, compile-time only
3. **Invalid States Don't Compile** - Catches errors before runtime
4. **API Safety Through Types** - Impossible to misuse
5. **Integrator Philosophy** - Compose validated components with guaranteed contracts

---

*Links*: [[rust-for-rustaceans]] | [[impl-trait]] | [[zero-cost-abstractions]] | [[phantom-data]] | [[builder-pattern]] | [[state-machines]] | [[static-vs-dynamic-dispatch]]

*Tags*: #rust #type-system #typestate #phantom-data #api-design #compile-time-safety #zero-cost #builder-pattern #state-machines
