//! Chapter 20.3: Advanced Types
//!
//! Demonstrates:
//! - Type aliases
//! - Never type (!)
//! - Dynamically Sized Types (DST)
//! - Sized trait

fn main() {
    println!("=== Chapter 20.3: Advanced Types ===\n");

    demonstrate_type_aliases();
    demonstrate_never_type();
    demonstrate_dst();

    println!("\n✅ All advanced type concepts demonstrated!");
}

/// 1. Type Aliases
fn demonstrate_type_aliases() {
    println!("--- 1. Type Aliases ---");

    type Kilometers = i32;
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let x: i32 = 5;
    let y: Kilometers = 10;
    println!("  i32 + Kilometers: {} + {} = {}", x, y, x + y);

    let f: Thunk = Box::new(|| println!("  Thunk executed!"));
    f();

    println!("💡 Type aliases: reduce repetition, improve clarity");
    println!();
}

/// 2. Never Type
fn demonstrate_never_type() {
    println!("--- 2. Never Type (!) ---");

    fn returns_never() -> ! {
        loop {
            break; // For demonstration - real ! never breaks
        }
        panic!("This diverges")
    }

    // Never type coerces to any type
    let guess: u32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => {
            println!("  Parse error - continuing...");
            0 // In real match, could use `continue` (type !)
        }
    };

    println!("  Parsed value: {}", guess);
    println!("💡 Never type: represents diverging functions");
    println!();
}

/// 3. Dynamically Sized Types
fn demonstrate_dst() {
    println!("--- 3. Dynamically Sized Types ---");

    // str is a DST - must use behind pointer
    let s1: &str = "Hello";
    let s2: Box<str> = Box::from("World");

    println!("  &str (pointer to DST): {}", s1);
    println!("  Box<str> (heap DST): {}", s2);

    // Generic function with ?Sized
    fn print_slice<T: ?Sized + std::fmt::Display>(value: &T) {
        println!("  Value: {}", value);
    }

    print_slice("DST string");

    println!("💡 DSTs: size known only at runtime, must use behind pointer");
    println!();
}
