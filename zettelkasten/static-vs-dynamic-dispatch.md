# Static vs Dynamic Dispatch - Trait Resolution Mechanisms

*The fundamental trade-off between compile-time type resolution (static dispatch) and runtime polymorphism (dynamic dispatch) in Rust.*

---

## 🎯 **Core Concept**

Dispatch refers to the mechanism by which a program determines which implementation of a polymorphic operation (trait method) to call. Rust provides two mechanisms:

- **Static Dispatch**: The compiler knows the exact concrete type at compile time and generates specialized code (via monomorphization)
- **Dynamic Dispatch**: The exact type is determined at runtime using vtable lookups (via trait objects)

This choice fundamentally affects performance, binary size, flexibility, and the types of abstractions you can create.

## 🧠 **Mental Models**

### The Restaurant Menu Analogy

**Static Dispatch**: You order from a physical menu where each dish has a specific recipe. The chef knows exactly what to cook before you arrive.

**Dynamic Dispatch**: You order "today's special" - the chef looks up what that is (vtable lookup) and prepares it. Slight delay, but more flexibility.

### The Function Call Map

**Static Dispatch**: Direct GPS navigation - the compiler knows the exact destination and plots the fastest route at compile time.

**Dynamic Dispatch**: Asking for directions at runtime - you know the category of place you want ("restaurant"), but check a directory to find which specific one.

## 🔍 **Detailed Content**

### **Static Dispatch**

```rust
// Generic function - compiler generates specific versions
fn print_area<T: Shape>(shape: T) {
    println!("Area: {}", shape.area());
}

// Compiler generates:
// fn print_area_circle(shape: Circle) { ... }
// fn print_area_rectangle(shape: Rectangle) { ... }

let circle = Circle { radius: 5.0 };
print_area(circle); // Direct call to print_area_circle
```

**Characteristics:**
- ✅ **Zero runtime cost** - no vtable lookup
- ✅ **Aggressive inlining** - compiler can optimize across call boundaries
- ✅ **Compile-time type safety** - all types known statically
- ❌ **Code bloat** - separate copy for each type (increases binary size)
- ❌ **Inflexible** - cannot have heterogeneous collections
- ❌ **Longer compile times** - more code to generate and optimize

### **Dynamic Dispatch**

```rust
// Trait object - single function, runtime dispatch
fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

// Single implementation, vtable lookup at runtime
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 4.0, height: 3.0 }),
];

for shape in &shapes {
    print_area(shape.as_ref()); // Vtable lookup for each call
}
```

**Characteristics:**
- ✅ **Heterogeneous collections** - store different types together
- ✅ **Smaller binary size** - single implementation
- ✅ **Plugin architectures** - load types at runtime
- ❌ **Runtime overhead** - vtable indirection (~1-2 ns per call)
- ❌ **No inlining** - compiler can't optimize across trait boundary
- ❌ **Trait object limitations** - not all traits are object-safe

## 📊 **Performance Comparison**

### Benchmark Results (Typical)

```rust
// Static dispatch: ~0.5 ns per call (inlined to nothing)
fn bench_static<T: Shape>(shape: T) {
    black_box(shape.area());
}

// Dynamic dispatch: ~1.5-2.5 ns per call (vtable lookup)
fn bench_dynamic(shape: &dyn Shape) {
    black_box(shape.area());
}
```

**When Performance Matters:**
- **Tight loops**: Static dispatch avoids repeated vtable lookups
- **Small functions**: Dynamic dispatch overhead becomes significant
- **CPU-bound**: Static dispatch allows better optimization
- **I/O-bound**: Dynamic dispatch overhead is negligible

## 🛠️ **Decision Guide**

### Use Static Dispatch When:

1. **Performance is critical** (hot paths, tight loops)
2. **Types are known at compile time**
3. **Binary size is not a concern**
4. **You want maximum inlining and optimization**
5. **Working with simple, homogeneous collections**

### Use Dynamic Dispatch When:

1. **Need heterogeneous collections** (different types in same Vec)
2. **Plugin architectures** (load types at runtime)
3. **Binary size matters** (embedded systems, WASM)
4. **Trait is used with many types** (reduces code bloat)
5. **Performance overhead is acceptable** (I/O-bound operations)

## 💡 **Common Patterns**

### Pattern 1: Generic Function with Trait Bounds (Static)

```rust
fn process<T: Iterator<Item = i32>>(iter: T) -> i32 {
    iter.sum()
}

// Compiler generates specific version for each iterator type
```

### Pattern 2: Trait Object for Heterogeneity (Dynamic)

```rust
trait Plugin {
    fn execute(&self);
}

struct App {
    plugins: Vec<Box<dyn Plugin>>, // Different plugin types
}
```

### Pattern 3: Hybrid Approach

```rust
// Static dispatch for performance-critical inner loop
fn process_batch<T: Shape>(shapes: &[T]) {
    for shape in shapes {
        // Monomorphized, fast
        compute_expensive(shape);
    }
}

// Dynamic dispatch for flexible API
fn process_mixed(shapes: &[Box<dyn Shape>]) {
    for shape in shapes {
        // Vtable lookup, but enables mixed types
        process_batch(std::slice::from_ref(shape.as_ref()));
    }
}
```

### Pattern 4: Enum Dispatch (Middle Ground)

```rust
// Manual dispatch - faster than dyn, flexible like trait objects
enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(c) => c.area(),
            Shape::Rectangle(r) => r.area(),
        }
    }
}

// Vec<Shape> allows heterogeneity without vtable overhead
```

## 🔧 **Technical Details**

### Vtable Structure

```rust
// For trait object &dyn Shape
// Memory layout:
// [data_ptr, vtable_ptr]
//              |
//              v
//         [destructor, size, align, shape.area(), shape.perimeter(), ...]
```

**Vtable Contents:**
1. Destructor function pointer
2. Size and alignment of type
3. Function pointers for each trait method

### Monomorphization vs Vtable

```rust
// This function with 3 different types:
fn generic<T: Display>(x: T) { println!("{}", x); }

// Static dispatch generates 3 functions:
// generic_i32(x: i32)
// generic_String(x: String)  
// generic_f64(x: f64)

// Dynamic dispatch generates 1 function + 3 vtables:
// generic_dyn(x: &dyn Display)
// vtable_i32, vtable_String, vtable_f64
```

## 🎯 **AoC and Mission Applications**

### Static Dispatch in Missions

**Mission 8 - Graph Trait:**
```rust
// Static dispatch - different graph types optimized independently
pub fn bfs<G: Graph>(graph: &G, start: G::Vertex) -> Vec<G::Vertex> {
    // Compiler generates optimized version for each graph type
}

// Usage with Grid:
bfs(&grid, start); // bfs_for_grid - optimized for 2D array access

// Usage with AdjacencyList:  
bfs(&adj_list, start); // bfs_for_adj_list - optimized for vector iteration
```

### Dynamic Dispatch in AoC

**AoC 2024 Day 13 - Mixed Shape Types:**
```rust
trait Machine {
    fn solve(&self) -> Option<i64>;
}

// Different solving strategies as trait implementations
let machines: Vec<Box<dyn Machine>> = vec![
    Box::new(LinearSolver { /* ... */ }),
    Box::new(CramersRuleSolver { /* ... */ }),
];

for machine in machines {
    if let Some(cost) = machine.solve() {
        total += cost;
    }
}
```

### Enum Dispatch for Performance

**AoC Pattern:**
```rust
// Better than dyn for known, fixed set of types
enum Instruction {
    Add(AddOp),
    Mul(MulOp),
    Jump(JumpOp),
}

impl Instruction {
    fn execute(&mut self, vm: &mut VM) {
        match self {
            Instruction::Add(op) => op.execute(vm),
            Instruction::Mul(op) => op.execute(vm),
            Instruction::Jump(op) => op.execute(vm),
        }
    }
}
// Faster than Vec<Box<dyn Executable>> - no vtable indirection
```

## 📚 **Rust Book Integration**

### Chapter 10 - Generics and Traits
- Generic functions use static dispatch by default
- Trait bounds enable monomorphization

### Chapter 17 - Object-Oriented Features
- `dyn Trait` syntax for trait objects
- Object safety requirements
- Vtable mechanism explanation

### Chapter 19 - Advanced Features
- Performance implications of dispatch choices
- Assembly-level understanding

## 🔗 **Related Concepts**

**Builds On:**
- [[monomorphization]] - Mechanism for static dispatch
- [[trait-objects]] - Mechanism for dynamic dispatch
- [[zero-cost-abstractions]] - Why static dispatch is "zero cost"

**Enables:**
- [[object-safety]] - Requirements for dynamic dispatch
- [[enum-dispatch-pattern]] - Alternative to both approaches
- [[performance-optimization]] - Choosing the right dispatch

**Related Patterns:**
- [[generic-programming]] - Primary use of static dispatch
- [[plugin-architecture]] - Primary use of dynamic dispatch
- [[heterogeneous-collections]] - When you need dyn

## 🎓 **Learning Journey**

### Beginner Understanding
- Static = compile time, fast
- Dynamic = runtime, flexible
- Choose based on whether types are known

### Intermediate Understanding
- Vtable mechanics and memory layout
- Performance measurement with benchmarks
- Binary size implications

### Advanced Understanding
- Enum dispatch as middle ground
- Custom vtables for specialized needs
- Hybrid approaches for optimal performance

## 🔬 **Verification Techniques**

### Assembly Inspection

```bash
# Compare assembly output
cargo rustc --release -- --emit asm

# Static dispatch: direct call or inline
# Dynamic dispatch: call through memory
```

### Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_dispatch(c: &mut Criterion) {
    c.bench_function("static", |b| {
        let shape = Circle { radius: 5.0 };
        b.iter(|| black_box(shape.area()));
    });
    
    c.bench_function("dynamic", |b| {
        let shape: &dyn Shape = &Circle { radius: 5.0 };
        b.iter(|| black_box(shape.area()));
    });
}
```

### Size Comparison

```bash
# Check binary size impact
cargo bloat --release
```

## 💡 **Key Takeaways**

1. **Static dispatch = speed, dynamic dispatch = flexibility**
2. **Default to static dispatch** unless you need heterogeneous collections
3. **Consider enum dispatch** for known, fixed sets of types
4. **Measure before optimizing** - profile to find if dispatch matters
5. **Trait objects have constraints** - not all traits are object-safe
6. **Binary size vs runtime speed** - fundamental trade-off
7. **Inlining is powerful** - static dispatch enables aggressive optimization

## 🏷️ **Tags**

*Tags: #rust #dispatch #performance #trait-objects #generics #monomorphization #zero-cost-abstractions #vtable #optimization #design-patterns*

---

## 📎 **Links**

**Referenced By:**
- [[rust_for_rustaceans/Ch02]] - Types and dispatch mechanisms
- [[zettelkasten/weekly plans/2026-W01]] - Week 1 learning objectives
- [[monomorphization]] - Static dispatch mechanism
- [[zero-cost-abstractions]] - Static dispatch as zero-cost

**See Also:**
- [[trait-objects]] - Dynamic dispatch implementation
- [[object-safety]] - Constraints on dynamic dispatch
- [[generic-programming]] - Static dispatch usage
- [[performance-patterns]] - When to choose each approach
- [[enum-dispatch-pattern]] - Alternative dispatch mechanism
- [[Week 3 Overview]] - Week covering trait objects and dispatch
