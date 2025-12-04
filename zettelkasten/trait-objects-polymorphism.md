# Trait Objects and Runtime Polymorphism

**Using `dyn Trait` for heterogeneous collections and dynamic dispatch in Rust**

> **Key Insight**: Trait objects enable runtime polymorphism through dynamic dispatch, allowing different concrete types to be treated uniformly through a shared trait interface.

---

## 🎯 **Core Concept**

A **trait object** is a pointer to a value that implements a trait, enabling:
- **Heterogeneous collections**: Store different types in the same collection
- **Dynamic dispatch**: Call methods on objects without knowing their concrete type
- **Runtime flexibility**: Decide which implementation to use at runtime

### **Syntax**

```rust
// Trait object types
&dyn Trait           // Reference to trait object
Box<dyn Trait>       // Owned trait object on heap
Arc<dyn Trait>       // Thread-safe shared trait object
Rc<dyn Trait>        // Single-threaded shared trait object
```

---

## 🔧 **Basic Pattern: Heterogeneous Collections**

### **The Problem**

```rust
// This WON'T compile - Vec needs a single type
let items: Vec<???> = vec![button, text_box, slider];
```

### **The Solution: Trait Objects**

```rust
trait Draw {
    fn draw(&self);
}

struct Button { label: String }
struct TextBox { content: String }

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {}", self.label);
    }
}

impl Draw for TextBox {
    fn draw(&self) {
        println!("Drawing text box: {}", self.content);
    }
}

// Heterogeneous collection using trait objects
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in &self.components {
            component.draw();  // Dynamic dispatch
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button { label: "OK".to_string() }),
            Box::new(TextBox { content: "Hello".to_string() }),
        ],
    };
    screen.run();
}
```

---

## 📊 **Static vs Dynamic Dispatch**

### **Comparison Table**

| Aspect | Static Dispatch (Generics) | Dynamic Dispatch (Trait Objects) |
|--------|---------------------------|----------------------------------|
| **Syntax** | `fn foo<T: Trait>(x: T)` | `fn foo(x: &dyn Trait)` |
| **Resolution** | Compile time | Runtime |
| **Performance** | Faster (inlined) | vtable lookup overhead |
| **Binary size** | Larger (monomorphization) | Smaller (single code path) |
| **Heterogeneous** | ❌ No | ✅ Yes |
| **Known at compile** | ✅ Yes | ❌ No |

### **When to Use Which**

**Use Generics (Static Dispatch) when:**
- Performance is critical
- All types known at compile time
- Working with a single type per call site

**Use Trait Objects (Dynamic Dispatch) when:**
- Need heterogeneous collections
- Types determined at runtime (plugins, configuration)
- Want smaller binary size
- Building extensible APIs

---

## 🚫 **Object Safety**

Not all traits can be used as trait objects. A trait is **object-safe** if:

1. **No `Self` in return position**
2. **No generic methods** (without `where Self: Sized`)
3. **All methods are dispatchable**

### **Object-Safe Traits**

```rust
// ✅ Object-safe: can use as dyn Draw
trait Draw {
    fn draw(&self);
    fn bounds(&self) -> (u32, u32);
}

// ✅ Object-safe: associated types are OK
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### **Non-Object-Safe Traits**

```rust
// ❌ NOT object-safe: returns Self
trait Clone {
    fn clone(&self) -> Self;
}

// ❌ NOT object-safe: generic method
trait BadTrait {
    fn process<T>(&self, item: T);
}

// ❌ NOT object-safe: Self in parameter
trait Comparable {
    fn compare(&self, other: &Self) -> bool;
}
```

### **Making Traits Object-Safe**

```rust
// Add `where Self: Sized` to exclude method from vtable
trait PartiallyObjectSafe {
    fn object_safe_method(&self);
    
    // This method won't be available on dyn PartiallyObjectSafe
    fn not_object_safe(&self) -> Self where Self: Sized;
}
```

---

## 🎨 **Common Patterns**

### **Pattern 1: Plugin/Extension System**

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn register(&mut self, plugin: Box<dyn Plugin>) {
        println!("Registered plugin: {}", plugin.name());
        self.plugins.push(plugin);
    }
    
    fn run_all(&self, input: &str) -> Vec<String> {
        self.plugins.iter()
            .map(|p| p.execute(input))
            .collect()
    }
}
```

### **Pattern 2: Event Handlers**

```rust
trait EventHandler {
    fn handle(&self, event: &Event);
    fn can_handle(&self, event: &Event) -> bool;
}

struct EventDispatcher {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventDispatcher {
    fn dispatch(&self, event: &Event) {
        for handler in &self.handlers {
            if handler.can_handle(event) {
                handler.handle(event);
            }
        }
    }
}
```

### **Pattern 3: Strategy Pattern**

```rust
trait Validator {
    fn validate(&self, input: &str) -> Result<(), String>;
}

struct FormField {
    name: String,
    validators: Vec<Box<dyn Validator>>,
}

impl FormField {
    fn validate(&self, input: &str) -> Vec<String> {
        self.validators.iter()
            .filter_map(|v| v.validate(input).err())
            .collect()
    }
}
```

### **Pattern 4: Factory with Trait Objects**

```rust
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

fn create_shape(shape_type: &str) -> Option<Box<dyn Shape>> {
    match shape_type {
        "circle" => Some(Box::new(Circle { radius: 1.0 })),
        "rectangle" => Some(Box::new(Rectangle { width: 2.0, height: 3.0 })),
        _ => None,
    }
}
```

---

## ⚡ **Performance Considerations**

### **vtable Overhead**

Each trait object carries a "fat pointer" with two parts:
1. **Data pointer**: Points to the actual value
2. **vtable pointer**: Points to the method dispatch table

```rust
// Size comparison
use std::mem::size_of;

assert_eq!(size_of::<&u64>(), 8);           // Regular reference: 8 bytes
assert_eq!(size_of::<&dyn Draw>(), 16);     // Trait object: 16 bytes (fat pointer)
assert_eq!(size_of::<Box<dyn Draw>>(), 16); // Same for Box
```

### **When Performance Matters**

```rust
// Hot loop with dynamic dispatch - may be slower
fn slow_path(items: &[Box<dyn Process>]) {
    for item in items {
        item.process();  // vtable lookup each iteration
    }
}

// Consider: batch operations, caching, or static dispatch for hot paths
```

---

## 🔗 **Combining with Other Features**

### **Trait Objects + Lifetimes**

```rust
trait Logger {
    fn log(&self, msg: &str);
}

// Trait object with lifetime
fn with_logger<'a>(logger: &'a dyn Logger) {
    logger.log("Using borrowed trait object");
}

// Owned trait object with lifetime bounds
struct Context<'a> {
    logger: Box<dyn Logger + 'a>,
}
```

### **Trait Objects + Send/Sync**

```rust
use std::sync::Arc;

// Thread-safe trait object
trait Task: Send + Sync {
    fn run(&self);
}

struct ThreadPool {
    tasks: Vec<Arc<dyn Task>>,
}

// Require Send for cross-thread transfer
fn spawn_task(task: Box<dyn Task + Send>) {
    std::thread::spawn(move || task.run());
}
```

### **Trait Objects + Downcasting**

```rust
use std::any::Any;

trait Widget: Any {
    fn render(&self);
    fn as_any(&self) -> &dyn Any;
}

impl dyn Widget {
    fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}
```

---

## 🆚 **Trait Objects vs Enums**

Both enable polymorphism, but with different trade-offs:

| Aspect | Trait Objects | Enums |
|--------|--------------|-------|
| **Open/Closed** | Open (new types anytime) | Closed (fixed variants) |
| **Adding types** | ✅ Easy | Requires code change |
| **Adding methods** | Requires trait change | ✅ Easy |
| **Pattern matching** | ❌ No | ✅ Yes |
| **Memory layout** | Heap allocated (usually) | Stack, sized by largest variant |
| **Performance** | Dynamic dispatch | Static, can inline |

### **Decision Guide**

```rust
// Use ENUM when:
// - Set of types is known and fixed
// - Need pattern matching
// - Performance critical
// - Types have similar size

enum Message {
    Text(String),
    Image(Vec<u8>),
    Command { action: String, params: Vec<String> },
}

// Use TRAIT OBJECT when:
// - Types unknown or extensible
// - Plugin/extension system
// - External crates add implementations
// - Types vary significantly in size

trait MessageHandler {
    fn handle(&self, msg: &str);
}
```

---

## 🔗 **Related Concepts**

### **Rust Book**
- **[[rust_book/Ch18/README|Chapter 18 OOP Features]]** - Full treatment of trait objects
- **[[rust-oop-characteristics]]** - Encapsulation and Rust's OOP approach

### **Patterns**
- **[[state-pattern-rust]]** - State machines using trait objects
- **[[Dynamic Programming]]** - When to choose runtime flexibility
- **[[Greedy Algorithms]]** - Contrast: static dispatch is often better for algorithms

### **Data Structures**
- **[[mission-9]]** - Priority queues often use trait objects for custom comparators
- **[[AoC Collection Problems]]** - When heterogeneous collections solve problems

### **Performance**
- **[[Performance Optimization Guide]]** - Static vs dynamic dispatch trade-offs

---

## 📚 **Complete Example: GUI Framework**

```rust
//! A minimal GUI framework demonstrating trait objects

trait Widget {
    fn draw(&self);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

struct Button {
    label: String,
    width: u32,
    height: u32,
}

struct TextInput {
    placeholder: String,
    width: u32,
}

struct Panel {
    children: Vec<Box<dyn Widget>>,
}

impl Widget for Button {
    fn draw(&self) {
        println!("[Button: {}]", self.label);
    }
    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }
}

impl Widget for TextInput {
    fn draw(&self) {
        println!("[____{}____]", self.placeholder);
    }
    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { 30 }
}

impl Widget for Panel {
    fn draw(&self) {
        println!("┌──Panel──┐");
        for child in &self.children {
            print!("│ ");
            child.draw();
            println!(" │");
        }
        println!("└─────────┘");
    }
    
    fn width(&self) -> u32 {
        self.children.iter().map(|c| c.width()).max().unwrap_or(0) + 20
    }
    
    fn height(&self) -> u32 {
        self.children.iter().map(|c| c.height()).sum::<u32>() + 20
    }
}

fn main() {
    let ui = Panel {
        children: vec![
            Box::new(Button { label: "Submit".into(), width: 80, height: 30 }),
            Box::new(TextInput { placeholder: "Enter name".into(), width: 200 }),
            Box::new(Button { label: "Cancel".into(), width: 80, height: 30 }),
        ],
    };
    
    ui.draw();
    println!("Panel size: {}x{}", ui.width(), ui.height());
}
```

---

*Tags: #rust #traits #polymorphism #dyn #trait-objects #oop #dynamic-dispatch #design-patterns*

*Links: [[rust_book/Ch18/README]] | [[rust-oop-characteristics]] | [[state-pattern-rust]] | [[AoC Patterns MOC]] | [[Rust Collections MOC]]*
