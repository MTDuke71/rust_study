//! Chapter 3: Flexible Interfaces - Generic Arguments & Object Safety
//!
//! Topics: Generic Arguments, Object Safety, Dynamic Dispatch

fn main() {
    println!("=== Flexible Interfaces: Generics & Object Safety ===\n");

    generic_arguments();
    object_safety();
    dynamic_dispatch();
}

/// Generic Arguments: Accept wider range of inputs
fn generic_arguments() {
    println!("--- Generic Arguments ---");

    // ❌ Inflexible: fn greet(name: String)
    // ✅ Flexible: fn greet(name: impl AsRef<str>)

    greet("Alice"); // &str
    greet(String::from("Bob")); // String
    greet(String::from("Charlie")); // String (generic accepts owned too)

    println!();
}

/// Accept any string-like type using AsRef
fn greet<S: AsRef<str>>(name: S) {
    println!("  Hello, {}!", name.as_ref());
}

/// Object Safety: Design traits for dyn Trait usage
fn object_safety() {
    println!("--- Object Safety ---");

    // Object-safe trait can be used with dyn
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        }),
    ];

    for shape in shapes {
        println!("  Area: {:.2}", shape.area());
    }

    println!();
}

/// Dynamic Dispatch: Trade-off between generics and dyn Trait
fn dynamic_dispatch() {
    println!("--- Dynamic Dispatch Trade-offs ---");

    println!("  Static dispatch (generics):");
    println!("    + Faster (inlining, no vtable lookup)");
    println!("    + Can use generic methods");
    println!("    - Binary bloat (monomorphization)");

    println!("\n  Dynamic dispatch (dyn Trait):");
    println!("    + Smaller binary size");
    println!("    + Runtime polymorphism");
    println!("    - Slower (vtable indirection)");
    println!("    - Trait must be object-safe");

    println!();
}

// === Object-Safe Trait ===

trait Shape {
    fn area(&self) -> f64; // Object-safe: no generics, no Self return
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// === Non-Object-Safe Example (Commented) ===

#[allow(dead_code)] // Demonstration trait not used
trait NotObjectSafe {
    // ❌ Returns Self - not object-safe
    // fn clone_self(&self) -> Self;

    // ❌ Generic method - not object-safe
    // fn compare<T>(&self, other: &T) -> bool;

    // ✅ Make it object-safe with where Self: Sized
    fn clone_self(&self) -> Self
    where
        Self: Sized,
        Self: Clone,
    {
        self.clone()
    }
}
