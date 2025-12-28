# ⏰ Understanding the 'static Lifetime

**The 'static lifetime - values that live for the entire program duration**

**Tags:** #lifetimes #static #memory #rust-for-rustaceans-ch1 #ownership #references

**Related:** [[lifetime-parameters]], [[ownership-fundamentals]], [[Borrow Checker Fundamentals]], [[variance]], [[Memory Management]]

---

## 🎯 Core Concept

The `'static` lifetime is the **longest possible lifetime** in Rust - it indicates that a reference is valid for the **entire duration of the program**.

```rust
let s: &'static str = "Hello, world!";
// This reference is valid from program start to program end
```

---

## 📋 Two Meanings of 'static

### **1. 'static Lifetime (Reference)**

A **reference** with lifetime `'static`:

```rust
// String literal - stored in program binary
let greeting: &'static str = "Hello";

// Static variable reference
static CONFIG: &str = "production";
let config_ref: &'static str = CONFIG;
```

**Key Point:** The **reference** is valid for the entire program, but the **value** it points to must also live that long.

### **2. 'static Bound (Owned Data)**

A **type** with `'static` bound (no borrowed references, or only `'static` references):

```rust
fn spawn_thread<T: 'static + Send>(data: T) {
    std::thread::spawn(move || {
        // T must not contain non-'static references
    });
}

// ✅ Works - String is owned (no references)
spawn_thread(String::from("hello"));

// ✅ Works - &'static str is 'static
spawn_thread("hello");

// ❌ Won't work - &str might not be 'static
let temp = String::from("temp");
// spawn_thread(&temp);  // Error: temp doesn't live long enough
```

---

## 🗂️ Sources of 'static Data

### **1. String Literals**

Hardcoded strings are embedded in the program binary:

```rust
let s: &'static str = "I live forever";
// Stored in read-only memory (.rodata section)
```

### **2. Static Variables**

```rust
static GLOBAL_COUNT: i32 = 0;
static MESSAGE: &str = "Global message";

fn get_message() -> &'static str {
    MESSAGE  // Valid - MESSAGE has 'static lifetime
}
```

### **3. Constant Promotion**

```rust
const PI: f64 = 3.14159;
let pi_ref: &'static f64 = &PI;  // Promoted to static

// Literal integers/floats can also be promoted
let forty_two: &'static i32 = &42;
```

### **4. Leaked Memory**

```rust
let leaked: &'static str = Box::leak(Box::new(String::from("leaked")));
// Box::leak converts Box<T> to &'static T
// Memory is never freed (intentional memory leak)
```

---

## 🔄 'static in Function Signatures

### **Returning 'static References**

```rust
// ✅ Valid - returns string literal
fn get_greeting() -> &'static str {
    "Hello"
}

// ✅ Valid - returns static variable
static VERSION: &str = "1.0.0";
fn get_version() -> &'static str {
    VERSION
}

// ❌ Invalid - local String doesn't live long enough
fn invalid() -> &'static str {
    let s = String::from("temp");
    // &s  // Error: s is dropped at end of function
    "fallback"
}
```

### **'static Trait Bounds**

```rust
// T: 'static means T contains no non-'static references
fn process<T: 'static>(value: T) {
    // Can safely move T into spawned thread
    std::thread::spawn(move || {
        drop(value);
    });
}

// ✅ Owned types are 'static
process(String::from("hello"));
process(vec![1, 2, 3]);
process(42);

// ❌ References to local data are not 'static
let local = String::from("local");
// process(&local);  // Error: &String is not 'static
```

---

## 🧠 'static vs Owned Data

### **Common Misconception**

`T: 'static` does **NOT** mean "T must be a reference that lives forever"!

It means: "T contains no references, OR all references in T are `'static`"

```rust
// All of these satisfy T: 'static:
fn accepts_static<T: 'static>(value: T) {
    // ...
}

accepts_static(42);                    // i32 has no references
accepts_static(String::from("owned")); // String owns its data
accepts_static("literal");             // &'static str
accepts_static(vec![1, 2, 3]);         // Vec owns its data

struct Container {
    data: String,  // Owns data - no references
}
accepts_static(Container { data: String::from("test") });  // ✅
```

### **What Doesn't Satisfy 'static**

```rust
fn fails_static<T: 'static>(value: T) { }

let s = String::from("temporary");
let reference: &str = &s;

// ❌ &str here is NOT 'static - it borrows from s
// fails_static(reference);  // Error: s doesn't live long enough
```

---

## ⚖️ Variance and 'static

From **Rust for Rustaceans Ch1.5**:

`'static` can **substitute** for any shorter lifetime (covariance):

```rust
fn takes_short_lifetime<'a>(s: &'a str) {
    println!("{}", s);
}

let static_str: &'static str = "Hello";
takes_short_lifetime(static_str);  // ✅ 'static can be used as any 'a
```

But the reverse is **NOT** true:

```rust
fn requires_static(s: &'static str) {
    println!("{}", s);
}

let temp = String::from("temporary");
let temp_ref: &str = &temp;
// requires_static(temp_ref);  // ❌ Error: temp doesn't live long enough
```

**Related:** [[variance]] - How lifetimes relate to each other

---

## 🛠️ Practical Examples

### **Example 1: Configuration Data**

```rust
static CONFIG: Config = Config {
    max_connections: 100,
    timeout_ms: 5000,
};

fn get_config() -> &'static Config {
    &CONFIG  // Valid - CONFIG has static storage duration
}
```

### **Example 2: Thread-Safe Globals**

```rust
use std::sync::OnceLock;

static INSTANCE: OnceLock<Database> = OnceLock::new();

fn get_db() -> &'static Database {
    INSTANCE.get_or_init(|| Database::connect())
}
```

### **Example 3: String Interning**

```rust
use std::collections::HashMap;
use std::sync::Mutex;

static INTERNED: Mutex<HashMap<String, &'static str>> = Mutex::new(HashMap::new());

fn intern(s: String) -> &'static str {
    let mut map = INTERNED.lock().unwrap();
    *map.entry(s.clone()).or_insert_with(|| {
        Box::leak(s.into_boxed_str())
    })
}
```

### **Example 4: Error Messages**

```rust
#[derive(Debug)]
struct Error {
    message: &'static str,
}

const FILE_NOT_FOUND: Error = Error {
    message: "File not found",
};

const PERMISSION_DENIED: Error = Error {
    message: "Permission denied",
};

fn handle_error(code: i32) -> Error {
    match code {
        404 => FILE_NOT_FOUND,
        403 => PERMISSION_DENIED,
        _ => Error { message: "Unknown error" },
    }
}
```

---

## ⚠️ Common Mistakes

### **Mistake 1: Thinking 'static Means Immutable**

```rust
// 'static doesn't mean immutable!
static mut COUNTER: i32 = 0;

unsafe {
    COUNTER += 1;  // Can mutate (though unsafe)
}
```

### **Mistake 2: Confusing 'static Lifetime with Static Variables**

```rust
// These are different:
static X: i32 = 42;           // Static variable
let y: &'static i32 = &X;     // Reference with 'static lifetime

// Not all 'static references point to static variables:
let z: &'static str = "literal";  // Points to binary data, not a variable
```

### **Mistake 3: Overusing Box::leak**

```rust
// ❌ Bad - unnecessary memory leak
fn process() -> &'static str {
    Box::leak(String::from("temp").into_boxed_str())  // Never freed!
}

// ✅ Better - use string literal if content is known
fn process() -> &'static str {
    "temp"
}

// ✅ Or return owned String
fn process() -> String {
    String::from("temp")
}
```

---

## 🧪 Complete Example: 'static in Practice

```rust
use std::sync::OnceLock;

// Static variable with constant value
static APP_NAME: &str = "MyApp";

// Lazily initialized static
static RUNTIME_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
struct Config {
    version: &'static str,
    max_workers: usize,
}

impl Config {
    fn load() -> Self {
        Config {
            version: "1.0.0",  // String literal - 'static
            max_workers: 4,
        }
    }
}

// Function returning 'static reference
fn get_app_name() -> &'static str {
    APP_NAME
}

// Function with 'static bound
fn spawn_worker<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::spawn(f);
}

fn main() {
    // Access static data
    println!("App: {}", get_app_name());
    
    // Initialize static config once
    let config = RUNTIME_CONFIG.get_or_init(Config::load);
    println!("Config: {:?}", config);
    
    // Spawn thread with owned data (satisfies 'static bound)
    let message = String::from("Hello from thread");
    spawn_worker(move || {
        println!("{}", message);  // message is owned, so T: 'static
    });
    
    // String literal also satisfies 'static
    spawn_worker(|| {
        println!("{}", "Literal message");
    });
}
```

---

## 🎓 Key Takeaways

1. **'static lifetime** = reference valid for entire program
2. **'static bound** (`T: 'static`) = type contains no non-'static references
3. **String literals** are always `&'static str`
4. **Static variables** have `'static` lifetime
5. **Owned types** (String, Vec, etc.) satisfy `T: 'static` bounds
6. **'static is covariant** - can substitute for shorter lifetimes
7. **Not all 'static data is immutable** - `static mut` exists (unsafe)

---

## 🔍 When to Use 'static

✅ **Use 'static when:**
- Storing string literals or compile-time constants
- Defining global configuration
- Thread boundaries require owned or static data
- Building error types with fixed messages

❌ **Avoid 'static when:**
- You can use shorter lifetimes (be specific!)
- You're tempted to leak memory unnecessarily
- Local/temporary data would suffice

---

## 📚 Related Concepts

- **[[lifetime-parameters]]** - General lifetime syntax and usage
- **[[variance]]** - How 'static relates to other lifetimes (covariance)
- **[[Borrow Checker Fundamentals]]** - How borrow checker validates lifetimes
- **[[ownership-fundamentals]]** - Foundation of ownership and borrowing
- **[[Memory Management]]** - Stack, heap, and static memory regions

---

## 🔗 Advanced Topics

- **[[phantom-data-type-safety]]** - Using PhantomData with 'static bounds
- **[[sync-send-traits]]** - Thread safety and 'static requirements
- **[[interior-mutability]]** - Mutable static data with Mutex/RwLock

---

*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[lifetime-parameters]] | [[variance]] | [[ownership-fundamentals]] | [[Borrow Checker Fundamentals]] | [[Memory Management]]*

*Tags: #lifetimes #static #memory #rust-for-rustaceans-ch1 #ownership #references #threads #variance*
