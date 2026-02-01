// Example 2: Dispatch Mechanisms
// Demonstrates static vs dynamic dispatch, vtables, and object safety

use rustaceans_ch02_types::utils::{print_section, print_subsection};

fn main() {
    print_section("2. DISPATCH MECHANISMS");

    static_dispatch_example();
    dynamic_dispatch_example();
    object_safety_examples();
    performance_comparison();
}

// --- Trait for Examples ---
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "Circle"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> &str {
        "Rectangle"
    }
}

// --- Static Dispatch ---
fn static_dispatch_example() {
    print_subsection("a. Static Dispatch (Monomorphization)");

    fn print_area<T: Shape>(shape: &T) {
        println!("{}: area = {:.2}", shape.name(), shape.area());
    }

    // Compiler generates two versions: print_area::<Circle> and print_area::<Rectangle>
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    print_area(&circle); // Calls monomorphized version for Circle
    print_area(&rect); // Calls monomorphized version for Rectangle

    println!("\n✅ Pros:");
    println!("   - Inlining possible");
    println!("   - No runtime overhead");
    println!("   - Compiler optimizations");

    println!("\n❌ Cons:");
    println!("   - Code bloat (one copy per type)");
    println!("   - Slower compile times");
    println!("   - Cannot store mixed types in collection");
}

// --- Dynamic Dispatch ---
fn dynamic_dispatch_example() {
    print_subsection("b. Dynamic Dispatch (Trait Objects)");

    fn print_area(shape: &dyn Shape) {
        println!("{}: area = {:.2}", shape.name(), shape.area());
    }

    let circle = Circle { radius: 5.0 };
    let rect = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    print_area(&circle); // Uses vtable lookup
    print_area(&rect); // Uses vtable lookup

    println!("\nStoring mixed types in collection:");
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle {
            width: 2.0,
            height: 5.0,
        }),
        Box::new(Circle { radius: 7.0 }),
    ];

    for shape in &shapes {
        print_area(shape.as_ref());
    }

    println!("\n✅ Pros:");
    println!("   - Smaller binary size");
    println!("   - Faster compilation");
    println!("   - Can store heterogeneous types");

    println!("\n❌ Cons:");
    println!("   - Vtable indirection overhead");
    println!("   - No inlining");
    println!("   - Runtime dispatch cost");
}

// --- Object Safety ---
fn object_safety_examples() {
    print_subsection("c. Object Safety");

    println!("Object-safe trait (can use as dyn Trait):");

    #[allow(dead_code)]
    trait ObjectSafe {
        fn method(&self);
        // ✅ No generic methods
        // ✅ No Self return type
        // ✅ No static methods
    }

    println!("  ✅ trait ObjectSafe {{ fn method(&self); }}");

    println!("\nNOT object-safe (cannot use as dyn Trait):");

    // Would fail to compile if uncommented:
    /*
    trait NotObjectSafe {
        fn returns_self(&self) -> Self;        // ❌ Returns Self
        fn generic<T>(&self, x: T);            // ❌ Generic method
        fn static_method();                    // ❌ No &self
    }

    // This would fail:
    // let _x: &dyn NotObjectSafe = ...;
    */

    println!("  ❌ fn returns_self(&self) -> Self");
    println!("  ❌ fn generic<T>(&self, x: T)");
    println!("  ❌ fn static_method() (no &self)");

    println!("\nClone is not object-safe:");
    println!("  Clone::clone(&self) -> Self  (returns Self)");
}

// --- Performance Comparison ---
fn performance_comparison() {
    print_subsection("d. Performance Comparison");

    const ITERATIONS: usize = 1_000_000;

    // Static dispatch
    let circle = Circle { radius: 5.0 };

    let start = std::time::Instant::now();
    let mut sum = 0.0;
    for _ in 0..ITERATIONS {
        sum += calculate_static(&circle);
    }
    let static_time = start.elapsed();

    println!(
        "Static dispatch:  {:?} ({} iterations)",
        static_time, ITERATIONS
    );
    println!("  Sum: {:.0} (prevents optimization)", sum);

    // Dynamic dispatch
    let shape: &dyn Shape = &circle;

    let start = std::time::Instant::now();
    let mut sum = 0.0;
    for _ in 0..ITERATIONS {
        sum += calculate_dynamic(shape);
    }
    let dynamic_time = start.elapsed();

    println!(
        "Dynamic dispatch: {:?} ({} iterations)",
        dynamic_time, ITERATIONS
    );
    println!("  Sum: {:.0} (prevents optimization)", sum);

    let ratio = dynamic_time.as_nanos() as f64 / static_time.as_nanos() as f64;
    println!("\nDynamic is {:.2}x slower (vtable lookup overhead)", ratio);
}

#[inline(never)]
fn calculate_static<T: Shape>(shape: &T) -> f64 {
    shape.area()
}

#[inline(never)]
fn calculate_dynamic(shape: &dyn Shape) -> f64 {
    shape.area()
}
