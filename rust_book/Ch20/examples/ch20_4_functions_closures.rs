//! Chapter 20.4: Advanced Functions and Closures
//!
//! Demonstrates:
//! - Function pointers
//! - Returning closures

fn main() {
    println!("=== Chapter 20.4: Advanced Functions and Closures ===\n");

    demonstrate_function_pointers();
    demonstrate_returning_closures();

    println!("\n✅ All function and closure concepts demonstrated!");
}

/// 1. Function Pointers
fn demonstrate_function_pointers() {
    println!("--- 1. Function Pointers ---");

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("  do_twice(add_one, 5) = {}", answer);

    // Function pointer with map
    let numbers = [1, 2, 3];
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("  Map with function pointer: {:?}", strings);

    // Enum initializers as function pointers
    #[allow(dead_code)]
    enum Status {
        Value(u32),
        Stop,
    }

    let statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();
    println!("  Created {} Status values with enum initializer", statuses.len());

    println!("💡 Function pointers: simple, efficient, no captures");
    println!();
}

/// 2. Returning Closures
fn demonstrate_returning_closures() {
    println!("--- 2. Returning Closures ---");

    // Using impl Trait
    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    let f = returns_closure();
    println!("  Closure via impl Trait: f(5) = {}", f(5));

    // Using trait object for multiple types
    fn returns_dynamic(add_value: bool) -> Box<dyn Fn(i32) -> i32> {
        if add_value {
            Box::new(|x| x + 1)
        } else {
            Box::new(|x| x * 2)
        }
    }

    let add_closure = returns_dynamic(true);
    let mul_closure = returns_dynamic(false);

    println!("  Dynamic closure (add): {}", add_closure(5));
    println!("  Dynamic closure (mul): {}", mul_closure(5));

    println!("💡 Return closures: impl Trait or Box<dyn Fn>");
    println!();
}
