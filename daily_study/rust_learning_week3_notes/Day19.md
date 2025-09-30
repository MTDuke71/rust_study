# Day 19 · Trait Objects (dynamic dispatch with `dyn`)

> **Learning Context**: Day 19 explores dynamic dispatch through trait objects, enabling Mission5's polymorphic collections and runtime flexibility while understanding performance trade-offs.

**Cross-Track Integration:**
- **Mission Focus**: Trait objects enable Mission5's flexible value storage and plugin architectures - see [[Mission5 Overview]]
- **Daily Study**: Builds on Week 3's trait mastery for dynamic programming patterns
- **Rust Book**: Chapter 17 Object-Oriented Programming Features and Chapter 19 Advanced Features

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Dynamic dispatch patterns across data structures
- [[Mission5 Overview]] - REQ-6 flexible APIs using trait objects
- [[zettel-index]] - Main learning hub

## Core Concepts

### Static vs Dynamic Dispatch
- **Static Dispatch**: Compiler knows exact type at compile time (generics)
- **Dynamic Dispatch**: Type determined at runtime through vtable lookup
- **Trait Objects**: `dyn Trait` enables polymorphism with runtime cost
- **Trade-offs**: Runtime flexibility vs compile-time optimization

### Trait Object Syntax
```rust
// Static dispatch - compile-time polymorphism
fn process_static<T: std::fmt::Display>(item: T) {
    println!("{}", item);  // Compiler generates specific version for each T
}

// Dynamic dispatch - runtime polymorphism
fn process_dynamic(item: &dyn std::fmt::Display) {
    println!("{}", item);  // Runtime vtable lookup
}

// Different ways to create trait objects
let items: Vec<Box<dyn std::fmt::Display>> = vec![
    Box::new(42),
    Box::new("hello"),
    Box::new(3.14),
];

for item in items {
    println!("{}", item);  // Dynamic dispatch
}
```

## Object Safety Rules

### What Makes a Trait Object-Safe?
```rust
// ✅ Object-safe trait
trait Drawable {
    fn draw(&self);           // Method takes &self
    fn area(&self) -> f64;    // Returns concrete type
}

// ❌ Not object-safe - generic method
trait NotObjectSafe {
    fn generic_method<T>(&self, item: T);  // Generic methods not allowed
}

// ❌ Not object-safe - Self in return type
trait AlsoNotObjectSafe {
    fn clone_self(&self) -> Self;  // Self in return not allowed
}

// ❌ Not object-safe - associated function
trait StillNotObjectSafe {
    fn new() -> Self;  // No self parameter
}
```

### Making Traits Object-Safe
```rust
// Original non-object-safe trait
trait Clone {
    fn clone(&self) -> Self;  // Returns Self - not object-safe
}

// Object-safe alternative
trait CloneBox {
    fn clone_box(&self) -> Box<dyn CloneBox>;
}

impl<T> CloneBox for T
where 
    T: Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneBox> {
        Box::new(self.clone())
    }
}

// Usage
let drawable: Box<dyn CloneBox> = Box::new("hello");
let cloned = drawable.clone_box();
```

## Creating and Using Trait Objects

### Box<dyn Trait> Pattern
```rust
trait Animal {
    fn name(&self) -> &str;
    fn noise(&self) -> &str;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn noise(&self) -> &str {
        "woof"
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn noise(&self) -> &str {
        "meow"
    }
}

// Collection of different animals
fn animal_sounds() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Buddy".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
    ];
    
    for animal in animals {
        animal.talk();  // Dynamic dispatch
    }
}
```

### Reference Trait Objects
```rust
// Using references instead of Box
fn process_animals(animals: &[&dyn Animal]) {
    for animal in animals {
        animal.talk();
    }
}

// Usage
let dog = Dog { name: "Rex".to_string() };
let cat = Cat { name: "Fluffy".to_string() };

let animal_refs: &[&dyn Animal] = &[&dog, &cat];
process_animals(animal_refs);
```

## Mission5 Integration: Dynamic Value Storage

### Polymorphic HashMap Values
```rust
use std::hash::Hash;
use std::any::Any;

// Mission5: HashMap with trait object values
trait Serializable {
    fn serialize(&self) -> String;
    fn type_name(&self) -> &str;
}

impl Serializable for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }
    
    fn type_name(&self) -> &str {
        "i32"
    }
}

impl Serializable for String {
    fn serialize(&self) -> String {
        format!("\"{}\"", self)
    }
    
    fn type_name(&self) -> &str {
        "String"
    }
}

impl Serializable for f64 {
    fn serialize(&self) -> String {
        self.to_string()
    }
    
    fn type_name(&self) -> &str {
        "f64"
    }
}

// HashMap storing different types through trait objects
pub struct DynamicHashMap<K>
where 
    K: Hash + Eq,
{
    data: std::collections::HashMap<K, Box<dyn Serializable>>,
}

impl<K> DynamicHashMap<K>
where 
    K: Hash + Eq,
{
    pub fn new() -> Self {
        DynamicHashMap {
            data: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert<V>(&mut self, key: K, value: V)
    where 
        V: Serializable + 'static,
    {
        self.data.insert(key, Box::new(value));
    }
    
    pub fn get(&self, key: &K) -> Option<&dyn Serializable> {
        self.data.get(key).map(|boxed| boxed.as_ref())
    }
    
    pub fn serialize_all(&self) -> Vec<String> {
        self.data.values()
                 .map(|val| format!("{}: {}", val.type_name(), val.serialize()))
                 .collect()
    }
}

// Usage example
fn dynamic_hashmap_example() {
    let mut map: DynamicHashMap<String> = DynamicHashMap::new();
    
    map.insert("count".to_string(), 42i32);
    map.insert("name".to_string(), "Alice".to_string());
    map.insert("score".to_string(), 98.5f64);
    
    // All values can be serialized through trait object
    for serialized in map.serialize_all() {
        println!("{}", serialized);
    }
}
```

### Plugin Architecture with Trait Objects
```rust
// Mission5 extensibility: Plugin system
trait MapPlugin {
    fn name(&self) -> &str;
    fn process(&self, data: &str) -> String;
    fn priority(&self) -> u8 {
        0  // Default priority
    }
}

struct CompressionPlugin;
struct EncryptionPlugin;
struct ValidationPlugin;

impl MapPlugin for CompressionPlugin {
    fn name(&self) -> &str {
        "compression"
    }
    
    fn process(&self, data: &str) -> String {
        format!("COMPRESSED({})", data)
    }
    
    fn priority(&self) -> u8 {
        2
    }
}

impl MapPlugin for EncryptionPlugin {
    fn name(&self) -> &str {
        "encryption"
    }
    
    fn process(&self, data: &str) -> String {
        format!("ENCRYPTED({})", data)
    }
    
    fn priority(&self) -> u8 {
        1
    }
}

impl MapPlugin for ValidationPlugin {
    fn name(&self) -> &str {
        "validation"
    }
    
    fn process(&self, data: &str) -> String {
        if data.len() > 0 {
            format!("VALID({})", data)
        } else {
            "INVALID()".to_string()
        }
    }
    
    fn priority(&self) -> u8 {
        3
    }
}

// Plugin manager using trait objects
struct PluginManager {
    plugins: Vec<Box<dyn MapPlugin>>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }
    
    fn register_plugin(&mut self, plugin: Box<dyn MapPlugin>) {
        self.plugins.push(plugin);
        // Sort by priority
        self.plugins.sort_by_key(|p| p.priority());
    }
    
    fn process_data(&self, data: &str) -> String {
        let mut result = data.to_string();
        
        for plugin in &self.plugins {
            println!("Applying plugin: {}", plugin.name());
            result = plugin.process(&result);
        }
        
        result
    }
}
```

## Advanced Trait Object Patterns

### Downcasting with Any
```rust
use std::any::{Any, TypeId};

trait Processor {
    fn process(&self) -> String;
    fn as_any(&self) -> &dyn Any;  // Enable downcasting
}

struct NumberProcessor {
    value: i32,
}

struct TextProcessor {
    text: String,
}

impl Processor for NumberProcessor {
    fn process(&self) -> String {
        format!("Number: {}", self.value * 2)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Processor for TextProcessor {
    fn process(&self) -> String {
        format!("Text: {}", self.text.to_uppercase())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Function that can downcast trait objects
fn specialized_processing(processor: &dyn Processor) -> String {
    let base_result = processor.process();
    
    // Attempt to downcast
    if let Some(num_proc) = processor.as_any().downcast_ref::<NumberProcessor>() {
        format!("{} (Number: {})", base_result, num_proc.value)
    } else if let Some(text_proc) = processor.as_any().downcast_ref::<TextProcessor>() {
        format!("{} (Length: {})", base_result, text_proc.text.len())
    } else {
        base_result
    }
}
```

### Trait Object Collections
```rust
// Different collection patterns with trait objects

// 1. Homogeneous interface, heterogeneous implementation
trait Drawable {
    fn draw(&self) -> String;
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Square { side: f64 }
struct Triangle { base: f64, height: f64 }

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Circle(r={})", self.radius)
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Square {
    fn draw(&self) -> String {
        format!("Square(s={})", self.side)
    }
    
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Drawable for Triangle {
    fn draw(&self) -> String {
        format!("Triangle(b={}, h={})", self.base, self.height)
    }
    
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// Canvas holding different shapes
struct Canvas {
    shapes: Vec<Box<dyn Drawable>>,
}

impl Canvas {
    fn new() -> Self {
        Canvas {
            shapes: Vec::new(),
        }
    }
    
    fn add_shape(&mut self, shape: Box<dyn Drawable>) {
        self.shapes.push(shape);
    }
    
    fn render(&self) -> Vec<String> {
        self.shapes.iter().map(|shape| shape.draw()).collect()
    }
    
    fn total_area(&self) -> f64 {
        self.shapes.iter().map(|shape| shape.area()).sum()
    }
}
```

## Performance Considerations

### Static vs Dynamic Dispatch Comparison
```rust
// Performance comparison example

trait Processor {
    fn process(&self, data: &[i32]) -> i32;
}

struct AddProcessor;
struct MultiplyProcessor;

impl Processor for AddProcessor {
    fn process(&self, data: &[i32]) -> i32 {
        data.iter().sum()
    }
}

impl Processor for MultiplyProcessor {
    fn process(&self, data: &[i32]) -> i32 {
        data.iter().product()
    }
}

// Static dispatch - compiler can inline and optimize
fn process_static<T: Processor>(processor: &T, data: &[i32]) -> i32 {
    processor.process(data)  // Direct call - can be inlined
}

// Dynamic dispatch - runtime vtable lookup
fn process_dynamic(processor: &dyn Processor, data: &[i32]) -> i32 {
    processor.process(data)  // Virtual call - cannot be inlined
}

// Performance testing
fn performance_comparison() {
    let data = vec![1, 2, 3, 4, 5];
    let add_proc = AddProcessor;
    
    // Static dispatch - faster
    let static_result = process_static(&add_proc, &data);
    
    // Dynamic dispatch - more flexible, slightly slower
    let dynamic_result = process_dynamic(&add_proc, &data);
    
    assert_eq!(static_result, dynamic_result);
}
```

### Memory Layout Considerations
```rust
// Trait object memory layout
struct TraitObjectLayout {
    data_ptr: *const (),    // Pointer to actual data
    vtable_ptr: *const (),  // Pointer to virtual method table
}

// Size implications
fn size_comparison() {
    println!("Size of i32: {}", std::mem::size_of::<i32>());
    println!("Size of Box<i32>: {}", std::mem::size_of::<Box<i32>>());
    println!("Size of &dyn Display: {}", std::mem::size_of::<&dyn std::fmt::Display>());
    println!("Size of Box<dyn Display>: {}", std::mem::size_of::<Box<dyn std::fmt::Display>>());
}
```

## When to Use Trait Objects

### Design Decision Guidelines
```rust
// ✅ Use trait objects when:
// 1. Need heterogeneous collections
let mixed_shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Square { side: 3.0 }),
];

// 2. Plugin/extension systems
trait Extension {
    fn handle(&self, input: &str) -> String;
}

struct PluginRegistry {
    extensions: Vec<Box<dyn Extension>>,
}

// 3. Abstract interfaces for libraries
trait DatabaseConnection {
    fn execute(&self, query: &str) -> Result<String, String>;
}

// ✅ Use static dispatch when:
// 1. Performance is critical
fn high_performance_process<T: Processor>(proc: T, data: &[i32]) -> i32 {
    proc.process(data)  // Can be inlined
}

// 2. Compile-time type checking is important
fn type_safe_process<T: Clone + std::fmt::Debug>(item: T) -> T {
    println!("Processing: {:?}", item);
    item.clone()
}
```

## Real-World Applications

### State Machine with Trait Objects
```rust
trait State {
    fn handle_input(&self, input: char) -> Box<dyn State>;
    fn description(&self) -> &str;
}

struct IdleState;
struct ProcessingState;
struct ErrorState;

impl State for IdleState {
    fn handle_input(&self, input: char) -> Box<dyn State> {
        match input {
            's' => Box::new(ProcessingState),
            _ => Box::new(ErrorState),
        }
    }
    
    fn description(&self) -> &str {
        "Idle"
    }
}

impl State for ProcessingState {
    fn handle_input(&self, input: char) -> Box<dyn State> {
        match input {
            'e' => Box::new(IdleState),
            _ => Box::new(ErrorState),
        }
    }
    
    fn description(&self) -> &str {
        "Processing"
    }
}

impl State for ErrorState {
    fn handle_input(&self, _input: char) -> Box<dyn State> {
        Box::new(IdleState)  // Reset on any input
    }
    
    fn description(&self) -> &str {
        "Error"
    }
}

struct StateMachine {
    current_state: Box<dyn State>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: Box::new(IdleState),
        }
    }
    
    fn input(&mut self, input: char) {
        println!("Current state: {}", self.current_state.description());
        self.current_state = self.current_state.handle_input(input);
        println!("New state: {}", self.current_state.description());
    }
}
```

## Best Practices

### Trait Object Design Guidelines
```rust
// ✅ Good: Simple, focused trait objects
trait Renderer {
    fn render(&self) -> String;
}

// ✅ Good: Include common functionality
trait Component {
    fn render(&self) -> String;
    
    // Default implementations work with trait objects
    fn with_id(&self, id: &str) -> String {
        format!("<div id='{}'>{}</div>", id, self.render())
    }
}

// ❌ Avoid: Complex trait hierarchies
trait OverlyComplex: Clone + Send + Sync + std::fmt::Debug {
    fn method1(&self) -> String;
    fn method2(&self) -> i32;
    fn method3(&self) -> f64;
    // Too many constraints, hard to implement
}

// ✅ Better: Composition over inheritance
trait Renderable {
    fn render(&self) -> String;
}

trait Identifiable {
    fn id(&self) -> &str;
}

// Combine when needed
fn render_with_id(item: &(dyn Renderable + Identifiable)) -> String {
    format!("<div id='{}'>{}</div>", item.id(), item.render())
}
```

## Learning Progression Summary

From Day 19, you should understand:
1. **Dynamic Dispatch**: Runtime polymorphism through vtable lookup
2. **Object Safety**: Rules that make traits usable as trait objects
3. **Syntax Patterns**: `Box<dyn Trait>`, `&dyn Trait` usage
4. **Mission5 Integration**: Dynamic value storage and plugin architectures
5. **Performance Trade-offs**: Flexibility vs compile-time optimization
6. **Design Patterns**: When to choose trait objects vs generics

**3-Track Learning Integration:**
- **Mission5**: Trait objects enable flexible HashMap value types and extensibility
- **Week 3 Completion**: Days 15-19 provide complete trait system mastery
- **Real-World Applications**: Plugin systems, state machines, heterogeneous collections

**Cross-References:**
- [[Collections MOC]] - Dynamic dispatch patterns in collection designs
- [[Mission5 Overview]] - REQ-6 flexible APIs using trait objects for extensibility
- [[HashMap Internals]] - Polymorphic value storage and plugin architecture patterns

**Next**: Day 20 will cover **Advanced Lifetimes** - elision rules, `'static`, and complex lifetime relationships!

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[Mission5 Overview]] | [[HashMap Internals]] | [[zettel-index]]*

*Tags: #trait-objects #dynamic-dispatch #polymorphism #object-safety #mission5 #daily-study #week3 #dyn-keyword*

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file

fn main() {
    println!("=== Trait Objects Demo from Day 19 ===\n");
    
    basic_trait_objects_demo();
    dynamic_hashmap_demo();
    plugin_system_demo();
    canvas_demo();
    state_machine_demo();
}

// 1. Basic trait objects
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn speak(&self) {
        println!("{} says '{}'", self.name(), self.sound());
    }
}

struct Dog { name: String }
struct Cat { name: String }
struct Bird { name: String }

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "Woof!" }
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "Meow!" }
}

impl Animal for Bird {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "Tweet!" }
}

fn basic_trait_objects_demo() {
    println!("1. Basic Trait Objects:");
    
    // Collection of different animals through trait objects
    let zoo: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Buddy".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
        Box::new(Bird { name: "Tweety".to_string() }),
    ];
    
    for animal in zoo {
        animal.speak();  // Dynamic dispatch
    }
    println!();
}

// 2. Dynamic HashMap with trait object values
trait Displayable {
    fn display(&self) -> String;
    fn type_name(&self) -> &str;
}

impl Displayable for i32 {
    fn display(&self) -> String { self.to_string() }
    fn type_name(&self) -> &str { "Integer" }
}

impl Displayable for String {
    fn display(&self) -> String { self.clone() }
    fn type_name(&self) -> &str { "String" }
}

impl Displayable for f64 {
    fn display(&self) -> String { format!("{:.2}", self) }
    fn type_name(&self) -> &str { "Float" }
}

struct DynamicMap {
    data: std::collections::HashMap<String, Box<dyn Displayable>>,
}

impl DynamicMap {
    fn new() -> Self {
        DynamicMap {
            data: std::collections::HashMap::new(),
        }
    }
    
    fn insert<T: Displayable + 'static>(&mut self, key: String, value: T) {
        self.data.insert(key, Box::new(value));
    }
    
    fn display_all(&self) {
        for (key, value) in &self.data {
            println!("   {}: {} ({})", key, value.display(), value.type_name());
        }
    }
}

fn dynamic_hashmap_demo() {
    println!("2. Dynamic HashMap with Trait Object Values:");
    
    let mut map = DynamicMap::new();
    map.insert("count".to_string(), 42);
    map.insert("name".to_string(), "Alice".to_string());
    map.insert("pi".to_string(), 3.14159);
    
    map.display_all();
    println!();
}

// 3. Plugin system with trait objects
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
    fn priority(&self) -> u8 { 0 }
}

struct UppercasePlugin;
struct ReversePlugin;
struct PrefixPlugin { prefix: String }

impl Plugin for UppercasePlugin {
    fn name(&self) -> &str { "uppercase" }
    fn execute(&self, input: &str) -> String {
        input.to_uppercase()
    }
    fn priority(&self) -> u8 { 1 }
}

impl Plugin for ReversePlugin {
    fn name(&self) -> &str { "reverse" }
    fn execute(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
    fn priority(&self) -> u8 { 2 }
}

impl Plugin for PrefixPlugin {
    fn name(&self) -> &str { "prefix" }
    fn execute(&self, input: &str) -> String {
        format!("{}{}", self.prefix, input)
    }
    fn priority(&self) -> u8 { 3 }
}

struct PluginEngine {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginEngine {
    fn new() -> Self {
        PluginEngine { plugins: Vec::new() }
    }
    
    fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
        self.plugins.sort_by_key(|p| p.priority());
    }
    
    fn process(&self, input: &str) -> String {
        let mut result = input.to_string();
        
        for plugin in &self.plugins {
            println!("   Applying plugin: {}", plugin.name());
            result = plugin.execute(&result);
        }
        
        result
    }
}

fn plugin_system_demo() {
    println!("3. Plugin System with Dynamic Dispatch:");
    
    let mut engine = PluginEngine::new();
    engine.register(Box::new(UppercasePlugin));
    engine.register(Box::new(ReversePlugin));
    engine.register(Box::new(PrefixPlugin { 
        prefix: ">> ".to_string() 
    }));
    
    let result = engine.process("hello world");
    println!("   Final result: '{}'", result);
    println!();
}

// 4. Canvas with different shapes
trait Shape {
    fn draw(&self) -> String;
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { base: f64, height: f64 }

impl Shape for Circle {
    fn draw(&self) -> String {
        format!("Circle(radius={})", self.radius)
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn draw(&self) -> String {
        format!("Rectangle({}x{})", self.width, self.height)
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn draw(&self) -> String {
        format!("Triangle(base={}, height={})", self.base, self.height)
    }
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Canvas {
    shapes: Vec<Box<dyn Shape>>,
}

impl Canvas {
    fn new() -> Self {
        Canvas { shapes: Vec::new() }
    }
    
    fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }
    
    fn render(&self) {
        println!("   Canvas contains {} shapes:", self.shapes.len());
        for (i, shape) in self.shapes.iter().enumerate() {
            println!("     {}: {} (area: {:.2})", i + 1, shape.draw(), shape.area());
        }
        
        let total_area: f64 = self.shapes.iter().map(|s| s.area()).sum();
        println!("   Total area: {:.2}", total_area);
    }
}

fn canvas_demo() {
    println!("4. Canvas with Heterogeneous Shapes:");
    
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 5.0 }));
    canvas.add_shape(Box::new(Rectangle { width: 4.0, height: 6.0 }));
    canvas.add_shape(Box::new(Triangle { base: 3.0, height: 8.0 }));
    
    canvas.render();
    println!();
}

// 5. State machine with trait objects
trait State {
    fn name(&self) -> &str;
    fn handle(&self, input: char) -> Box<dyn State>;
}

struct IdleState;
struct ProcessingState;
struct CompleteState;

impl State for IdleState {
    fn name(&self) -> &str { "Idle" }
    fn handle(&self, input: char) -> Box<dyn State> {
        match input {
            's' => {
                println!("     Starting processing...");
                Box::new(ProcessingState)
            },
            _ => {
                println!("     Invalid input in Idle state");
                Box::new(IdleState)
            }
        }
    }
}

impl State for ProcessingState {
    fn name(&self) -> &str { "Processing" }
    fn handle(&self, input: char) -> Box<dyn State> {
        match input {
            'f' => {
                println!("     Processing finished!");
                Box::new(CompleteState)
            },
            'c' => {
                println!("     Processing cancelled");
                Box::new(IdleState)
            },
            _ => {
                println!("     Still processing...");
                Box::new(ProcessingState)
            }
        }
    }
}

impl State for CompleteState {
    fn name(&self) -> &str { "Complete" }
    fn handle(&self, _input: char) -> Box<dyn State> {
        println!("     Task complete, returning to idle");
        Box::new(IdleState)
    }
}

struct StateMachine {
    current_state: Box<dyn State>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: Box::new(IdleState),
        }
    }
    
    fn input(&mut self, input: char) {
        println!("   State: {} -> Input: '{}'", self.current_state.name(), input);
        self.current_state = self.current_state.handle(input);
        println!("   New state: {}", self.current_state.name());
    }
}

fn state_machine_demo() {
    println!("5. State Machine with Dynamic Dispatch:");
    
    let mut machine = StateMachine::new();
    
    // Simulate state transitions
    machine.input('s');  // Idle -> Processing
    machine.input('x');  // Processing -> Processing (invalid)
    machine.input('f');  // Processing -> Complete
    machine.input('r');  // Complete -> Idle (any input)
    
    println!();
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day19_demo.rs` and run `rustc day19_demo.rs && ./day19_demo`
3. **In this workspace**: `.\scripts\run_md.bat daily_study\rust_learning_week3_notes\Day19.md`
4. **As Cargo example**: `cargo run --example day19_trait_objects_demo` (if you add it to Mission5_tut)
