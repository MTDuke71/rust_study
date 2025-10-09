# 🎭 Trait Objects - Dynamic dispatch patterns

**Runtime polymorphism in Rust through vtable-based method dispatch**

## 🎯 Core Concept

**Trait Objects** enable **dynamic dispatch** in Rust - the ability to call methods on types whose concrete implementation is determined at runtime rather than compile time.

**Key Distinction:**
```rust
// Static Dispatch (Generics) - Compile-time polymorphism
fn process_generic<T: Display>(item: T) {
    println!("{}", item);  // Compiler generates code for each T
}

// Dynamic Dispatch (Trait Objects) - Runtime polymorphism
fn process_dynamic(item: &dyn Display) {
    println!("{}", item);  // Runtime vtable lookup
}
```

**When to Use:**
- ✅ Heterogeneous collections (different types in same collection)
- ✅ Plugin systems (load implementations at runtime)
- ✅ Callback patterns (store closures with different captures)
- ✅ API flexibility (accept "anything that implements Trait")
- ❌ Performance-critical hot paths (vtable overhead)
- ❌ When generics suffice (prefer zero-cost abstractions)

---

## 📋 Trait Object Syntax

### **Creating Trait Objects**

```rust
use std::fmt::Display;

// 1. Reference to trait object: &dyn Trait
fn print_it(item: &dyn Display) {
    println!("{}", item);
}

// 2. Box<dyn Trait> - Owned trait object on heap
fn create_boxed() -> Box<dyn Display> {
    Box::new(42)  // i32 implements Display
}

// 3. Rc<dyn Trait> / Arc<dyn Trait> - Shared trait object
use std::rc::Rc;
let shared: Rc<dyn Display> = Rc::new("hello");

// 4. Heterogeneous collection
let items: Vec<Box<dyn Display>> = vec![
    Box::new(42),
    Box::new("hello"),
    Box::new(3.14),
];

for item in items {
    println!("{}", item);  // Each calls correct Display::fmt
}
```

### **Type Erasure in Action**

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("Drawing circle: {}", self.radius); }
}

impl Drawable for Square {
    fn draw(&self) { println!("Drawing square: {}", self.side); }
}

// Heterogeneous collection - different types, same trait
let shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Square { side: 10.0 }),
];

// Runtime dispatch to correct implementation
for shape in shapes {
    shape.draw();  // Calls Circle::draw or Square::draw
}
```

---

## ⚙️ How Dynamic Dispatch Works

### **The vtable Mechanism**

When you create a trait object, Rust creates a **vtable (virtual method table)** containing pointers to the concrete implementations:

```
Trait Object Layout:
┌─────────────────────┐
│  Data Pointer       │ ──→ Actual object in memory
├─────────────────────┤
│  VTable Pointer     │ ──→ Method dispatch table
└─────────────────────┘

VTable Structure:
┌─────────────────────┐
│ Drop (destructor)   │ ──→ Type's Drop::drop
├─────────────────────┤
│ Size                │ ──→ sizeof(T)
├─────────────────────┤
│ Alignment           │ ──→ align_of(T)
├─────────────────────┤
│ method1             │ ──→ Concrete::method1
├─────────────────────┤
│ method2             │ ──→ Concrete::method2
└─────────────────────┘
```

**Memory Representation:**
```rust
// Static dispatch - compiler knows exact type
let x: i32 = 42;
println!("{}", x);  // Direct call to <i32 as Display>::fmt
// Size: 4 bytes (just the i32)

// Dynamic dispatch - runtime type lookup
let y: &dyn Display = &42;
println!("{}", y);  // Indirect call through vtable
// Size: 16 bytes on 64-bit (8-byte ptr + 8-byte vtable ptr)
```

---

## 🔒 Object Safety Rules

**Not all traits can become trait objects!** A trait is **object-safe** if it follows these rules:

### **Rule 1: No Return Type of `Self`**

```rust
// ❌ NOT object-safe - returns Self
trait Cloneable {
    fn clone(&self) -> Self;  // Can't know size at compile time
}

// ✅ Object-safe version
trait CloneBox {
    fn clone_box(&self) -> Box<dyn CloneBox>;
}

impl<T: Clone + 'static> CloneBox for T {
    fn clone_box(&self) -> Box<dyn CloneBox> {
        Box::new(self.clone())
    }
}
```

### **Rule 2: No Generic Methods**

```rust
// ❌ NOT object-safe - generic method
trait Process {
    fn process<T>(&self, item: T);  // Can't generate vtable for all T
}

// ✅ Object-safe version - no generics
trait Process {
    fn process_string(&self, item: String);
    fn process_int(&self, item: i32);
}
```

### **Rule 3: No Associated Functions (no `self` parameter)**

```rust
// ❌ NOT object-safe - associated function
trait Factory {
    fn new() -> Self;  // No way to call without knowing concrete type
}

// ✅ Object-safe version
trait Factory {
    fn create_instance(&self) -> Box<dyn Factory>;
}
```

### **Rule 4: No `Sized` Bound**

```rust
// ❌ NOT object-safe - Sized bound
trait Process: Sized {
    fn process(&self);
}

// ✅ Object-safe - no Sized bound
trait Process {
    fn process(&self);
}
```

### **Checking Object Safety**

```rust
// Compiler will tell you if trait is not object-safe
fn use_trait_object(item: &dyn Clone) {  
    // ❌ Error: Clone is not object-safe
    // (has clone method returning Self)
}

fn use_trait_object(item: &dyn Display) {
    // ✅ OK: Display is object-safe
}
```

---

## 🎨 Common Patterns

### **Pattern 1: Plugin System**

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&mut self, input: &str) -> String;
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn new() -> Self {
        Self { plugins: Vec::new() }
    }
    
    fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
    
    fn run_all(&mut self, input: &str) {
        for plugin in &mut self.plugins {
            let result = plugin.execute(input);
            println!("{}: {}", plugin.name(), result);
        }
    }
}

// Usage
struct UppercasePlugin;
struct ReversePlugin;

impl Plugin for UppercasePlugin {
    fn name(&self) -> &str { "Uppercase" }
    fn execute(&mut self, input: &str) -> String {
        input.to_uppercase()
    }
}

impl Plugin for ReversePlugin {
    fn name(&self) -> &str { "Reverse" }
    fn execute(&mut self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

let mut manager = PluginManager::new();
manager.register(Box::new(UppercasePlugin));
manager.register(Box::new(ReversePlugin));
manager.run_all("hello");
```

### **Pattern 2: State Pattern**

```rust
trait State {
    fn handle(&self, context: &mut Context) -> Box<dyn State>;
}

struct Context {
    state: Box<dyn State>,
}

impl Context {
    fn new(initial: Box<dyn State>) -> Self {
        Self { state: initial }
    }
    
    fn request(&mut self) {
        let new_state = self.state.handle(self);
        self.state = new_state;
    }
}

struct StateA;
struct StateB;

impl State for StateA {
    fn handle(&self, _: &mut Context) -> Box<dyn State> {
        println!("StateA -> StateB");
        Box::new(StateB)
    }
}

impl State for StateB {
    fn handle(&self, _: &mut Context) -> Box<dyn State> {
        println!("StateB -> StateA");
        Box::new(StateA)
    }
}
```

### **Pattern 3: Callback Registry**

```rust
type Callback = Box<dyn Fn(i32) -> i32>;

struct EventSystem {
    callbacks: Vec<Callback>,
}

impl EventSystem {
    fn new() -> Self {
        Self { callbacks: Vec::new() }
    }
    
    fn register<F>(&mut self, callback: F)
    where
        F: Fn(i32) -> i32 + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }
    
    fn trigger(&self, value: i32) {
        for callback in &self.callbacks {
            let result = callback(value);
            println!("Callback result: {}", result);
        }
    }
}

// Usage
let mut system = EventSystem::new();
system.register(|x| x * 2);
system.register(|x| x + 10);
system.trigger(5);  // Calls both closures
```

### **Pattern 4: Strategy Pattern**

```rust
trait CompressionStrategy {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
    fn decompress(&self, data: &[u8]) -> Vec<u8>;
}

struct Compressor {
    strategy: Box<dyn CompressionStrategy>,
}

impl Compressor {
    fn new(strategy: Box<dyn CompressionStrategy>) -> Self {
        Self { strategy }
    }
    
    fn set_strategy(&mut self, strategy: Box<dyn CompressionStrategy>) {
        self.strategy = strategy;
    }
    
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        self.strategy.compress(data)
    }
}

struct GzipStrategy;
struct ZipStrategy;

impl CompressionStrategy for GzipStrategy {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // Gzip compression logic
        data.to_vec()
    }
    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

impl CompressionStrategy for ZipStrategy {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // Zip compression logic
        data.to_vec()
    }
    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}
```

---

## ⚡ Performance Considerations

### **Static vs Dynamic Dispatch Comparison**

```rust
// Static Dispatch (Generics)
// ✅ Pros: Zero-cost abstraction, inlining possible, no vtable
// ❌ Cons: Code bloat (one copy per type), binary size increase
fn process_static<T: Display>(items: &[T]) {
    for item in items {
        println!("{}", item);  // Compiler knows exact type
    }
}

// Dynamic Dispatch (Trait Objects)
// ✅ Pros: Single code copy, heterogeneous collections, smaller binary
// ❌ Cons: Vtable lookup overhead, no inlining, cache misses
fn process_dynamic(items: &[&dyn Display]) {
    for item in items {
        println!("{}", item);  // Runtime vtable lookup
    }
}
```

### **Benchmarking Dynamic Dispatch**

```rust
// Typical overhead: 5-15% for simple method calls
// Worst case: 50%+ for tiny hot-loop methods

// Static dispatch - ~1ns per call
fn static_sum<T: AsRef<i32>>(items: &[T]) -> i32 {
    items.iter().map(|x| *x.as_ref()).sum()
}

// Dynamic dispatch - ~1.2ns per call (20% overhead)
fn dynamic_sum(items: &[&dyn AsRef<i32>]) -> i32 {
    items.iter().map(|x| **x.as_ref()).sum()
}
```

**When to Pay the Cost:**
- Plugin systems loaded at runtime
- Heterogeneous collections (different types together)
- API boundaries where flexibility matters
- Non-critical paths where clarity > speed

**When to Avoid:**
- Tight inner loops (hot paths)
- Performance-critical algorithms
- When all types are known at compile time
- Simple cases where generics suffice

---

## 🔄 Converting Between Static and Dynamic

### **From Generic to Trait Object**

```rust
// Generic function
fn process_generic<T: Display>(item: T) {
    println!("{}", item);
}

// Convert to trait object at call site
let x = 42;
let y: &dyn Display = &x;  // Coercion to trait object
println!("{}", y);

// Store in collection
let items: Vec<Box<dyn Display>> = vec![
    Box::new(42),
    Box::new("hello"),
];
```

### **Boxing Pattern**

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) { println!("Woof!"); }
}

impl Animal for Cat {
    fn speak(&self) { println!("Meow!"); }
}

// Factory function returning trait object
fn create_animal(animal_type: &str) -> Box<dyn Animal> {
    match animal_type {
        "dog" => Box::new(Dog),
        "cat" => Box::new(Cat),
        _ => panic!("Unknown animal"),
    }
}

// Runtime polymorphism
let animal = create_animal("dog");
animal.speak();  // Woof!
```

---

## 🎓 Advanced Patterns

### **Trait Object with Multiple Traits**

```rust
// Combine traits with + operator
trait Draw {
    fn draw(&self);
}

trait Serialize {
    fn serialize(&self) -> String;
}

// Trait object requiring both traits
fn process(item: &(dyn Draw + Serialize)) {
    item.draw();
    let data = item.serialize();
    println!("Serialized: {}", data);
}
```

### **Downcasting Trait Objects**

```rust
use std::any::Any;

trait Plugin: Any {
    fn execute(&self);
    fn as_any(&self) -> &dyn Any;
}

struct MyPlugin {
    data: i32,
}

impl Plugin for MyPlugin {
    fn execute(&self) {
        println!("Executing with data: {}", self.data);
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Downcast from trait object to concrete type
fn downcast_example(plugin: &dyn Plugin) {
    if let Some(my_plugin) = plugin.as_any().downcast_ref::<MyPlugin>() {
        println!("Downcasted! Data: {}", my_plugin.data);
    }
}
```

### **Interior Mutability with Trait Objects**

```rust
use std::cell::RefCell;

trait Counter {
    fn increment(&mut self);
    fn value(&self) -> i32;
}

struct SimpleCounter {
    count: i32,
}

impl Counter for SimpleCounter {
    fn increment(&mut self) {
        self.count += 1;
    }
    fn value(&self) -> i32 {
        self.count
    }
}

// Store mutable trait objects with RefCell
let counter: RefCell<Box<dyn Counter>> = 
    RefCell::new(Box::new(SimpleCounter { count: 0 }));

counter.borrow_mut().increment();
println!("Count: {}", counter.borrow().value());
```

---

## 🔗 Real-World Applications

### **1. GUI Event Handlers**
```rust
trait EventHandler {
    fn handle(&mut self, event: &Event);
}

struct Button {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl Button {
    fn on_click(&mut self, event: &Event) {
        for handler in &mut self.handlers {
            handler.handle(event);
        }
    }
}
```

### **2. Serialization Framework**
```rust
trait Serialize {
    fn to_json(&self) -> String;
    fn to_xml(&self) -> String;
}

struct Serializer {
    objects: Vec<Box<dyn Serialize>>,
}
```

### **3. Testing Mock Objects**
```rust
trait Database {
    fn query(&self, sql: &str) -> Vec<String>;
}

struct MockDatabase {
    responses: Vec<Vec<String>>,
}

impl Database for MockDatabase {
    fn query(&self, _sql: &str) -> Vec<String> {
        self.responses[0].clone()
    }
}

// Test with trait object
fn test_query(db: &dyn Database) {
    let results = db.query("SELECT * FROM users");
    assert!(!results.is_empty());
}
```

---

## 📚 Connected Concepts

### **Related Zettelkasten Pages**
- [[Week 3 Overview]] - Day 19 covers trait objects in depth
- [[Mission5 Overview]] - REQ-6 flexible APIs using trait objects
- [[Generic Programming]] - Static dispatch alternative
- [[Collections MOC]] - Heterogeneous collections

### **Rust Book References**
- Chapter 17.2 - Using Trait Objects That Allow for Values of Different Types
- Chapter 19.2 - Advanced Traits (object safety)

### **Mission Integration**
- **Mission5 (HashMap)**: Trait objects for flexible value storage
- **Mission7 (Graphs)**: Plugin-based graph algorithms
- **AoC Solutions**: Dynamic input handlers

---

## 💡 Key Takeaways

1. **Trait Objects = Runtime Polymorphism**: Type determined at runtime, not compile time
2. **vtable Overhead**: Small performance cost (5-15%) for flexibility
3. **Object Safety Required**: Not all traits can be trait objects
4. **Heterogeneous Collections**: Store different types implementing same trait
5. **Use When Needed**: Prefer generics unless runtime flexibility required
6. **Pattern Rich**: Plugin systems, state machines, callbacks, strategies

**When to Use Trait Objects:**
```
✅ Need heterogeneous collections
✅ Plugin/extension systems
✅ Runtime type selection
✅ Simpler API boundaries

❌ Performance-critical code
❌ All types known at compile time
❌ Trait is not object-safe
❌ Generics provide same functionality
```

---

*Tags: #trait-objects #dynamic-dispatch #polymorphism #vtable #object-safety #design-patterns #week3 #advanced-traits*

*Links: [[zettel-index]] | [[Week 3 Overview]] | [[Generic Programming]] | [[Mission5 Overview]] | [[Collections MOC]] | [[Rust Trinity - Struct Trait Impl]]*
