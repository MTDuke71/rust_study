// Chapter 13.1: Closures - Anonymous Functions that Capture Their Environment
// Run with: cargo run --example ch13_1_closures

use std::thread;
use std::time::Duration;

fn main() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║  Chapter 13.1: Closures and Environment      ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    // Part 1: Basic Closure Syntax
    println!("=== Part 1: Closure Syntax ===");
    basic_closure_examples();

    // Part 2: Capturing Environment
    println!("\n=== Part 2: Capturing Environment ===");
    environment_capture_examples();

    // Part 3: Move Closures
    println!("\n=== Part 3: Move Closures ===");
    move_closure_examples();

    // Part 4: Fn Traits
    println!("\n=== Part 4: Fn Traits (Fn, FnMut, FnOnce) ===");
    fn_traits_examples();

    // Part 5: Practical Application - Caching/Memoization
    println!("\n=== Part 5: Practical Application - Memoization ===");
    memoization_example();
}

fn basic_closure_examples() {
    // Closure with explicit type annotations
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("add_one(5) = {}", add_one(5));

    // Closure with type inference
    let add_two = |x| x + 2;
    println!("add_two(5) = {}", add_two(5));

    // Multi-line closure
    let complex_calculation = |x: i32| {
        let doubled = x * 2;
        let squared = doubled * doubled;
        squared + 1
    };
    println!("complex_calculation(3) = {}", complex_calculation(3));

    // Closure with multiple parameters
    let multiply = |x, y| x * y;
    println!("multiply(4, 5) = {}", multiply(4, 5));

    // Closure stored in a variable
    let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x - 3),
    ];

    let value = 10;
    println!("\nApplying operations to {}:", value);
    for (i, op) in operations.iter().enumerate() {
        println!("  Operation {}: {}", i + 1, op(value));
    }
}

fn environment_capture_examples() {
    // Immutable borrow
    let list = vec![1, 2, 3];
    println!("Before closure: {:?}", list);

    let only_borrows = || println!("Closure borrowing: {:?}", list);
    only_borrows();

    println!("After closure: {:?}", list); // Can still use list

    // Mutable borrow
    let mut list2 = vec![1, 2, 3];
    println!("\nBefore mutable closure: {:?}", list2);

    let mut borrows_mutably = || list2.push(4);
    borrows_mutably();

    println!("After mutable closure: {:?}", list2);

    // Multiple captures
    let x = 10;
    let y = 20;
    let captures_both = || {
        println!("Captured x: {}", x);
        println!("Captured y: {}", y);
        x + y
    };
    println!("Sum from closure: {}", captures_both());
}

fn move_closure_examples() {
    // Moving ownership into closure
    let list = vec![1, 2, 3];
    println!("Before move closure: {:?}", list);

    let takes_ownership = move || {
        println!("Moved into closure: {:?}", list);
        // list is now owned by closure
    };
    takes_ownership();

    // This would error - list was moved:
    // println!("After move: {:?}", list);

    // Move closure with threads
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("Thread processing: {:?}", data);
        data.iter().sum::<i32>()
    });

    // Wait for thread and get result
    match handle.join() {
        Ok(sum) => println!("Thread calculated sum: {}", sum),
        Err(_) => println!("Thread panicked"),
    }

    // Multiple move closures
    let value = String::from("Hello");
    let value_clone = value.clone();

    let closure1 = move || println!("Closure 1: {}", value);
    let closure2 = move || println!("Closure 2: {}", value_clone);

    closure1();
    closure2();
}

fn fn_traits_examples() {
    // Fn trait - can be called multiple times, immutable borrow
    let x = 10;
    let fn_closure = |y| x + y;
    println!("Fn closure (call 1): {}", fn_closure(5));
    println!("Fn closure (call 2): {}", fn_closure(10));

    // FnMut trait - can be called multiple times, mutable borrow
    let mut counter = 0;
    let mut fn_mut_closure = || {
        counter += 1;
        counter
    };
    println!("FnMut closure (call 1): {}", fn_mut_closure());
    println!("FnMut closure (call 2): {}", fn_mut_closure());
    println!("Counter after closures: {}", counter);

    // FnOnce trait - can only be called once, takes ownership
    let s = String::from("hello");
    let fn_once_closure = || {
        println!("FnOnce closure taking ownership: {}", s);
        // s is consumed here
    };
    fn_once_closure();
    // fn_once_closure(); // Would error - can only call once

    // Demonstrating trait bounds
    demonstrate_fn_trait(|| println!("This is Fn"));
    
    let mut x = 0;
    demonstrate_fn_mut_trait(|| x += 1);
    println!("After FnMut: x = {}", x);
}

fn demonstrate_fn_trait<F>(f: F)
where
    F: Fn(),
{
    println!("Calling Fn trait closure:");
    f();
    f(); // Can call multiple times
}

fn demonstrate_fn_mut_trait<F>(mut f: F)
where
    F: FnMut(),
{
    println!("Calling FnMut trait closure:");
    f();
    f(); // Can call multiple times
}

fn memoization_example() {
    // Simple memoization/caching structure
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => {
                    println!("  (using cached value)");
                    v
                }
                None => {
                    println!("  (calculating and caching)");
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    // Simulate expensive calculation
    let expensive_calculation = |num: u32| {
        println!("  Performing expensive calculation...");
        thread::sleep(Duration::from_millis(100));
        num * 2
    };

    let mut cached_calc = Cacher::new(expensive_calculation);

    println!("First call:");
    let result1 = cached_calc.value(10);
    println!("  Result: {}", result1);

    println!("\nSecond call (should be cached):");
    let result2 = cached_calc.value(10);
    println!("  Result: {}", result2);

    // Improved cacher with HashMap for multiple values
    use std::collections::HashMap;

    struct ImprovedCacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        values: HashMap<u32, u32>,
    }

    impl<T> ImprovedCacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> ImprovedCacher<T> {
            ImprovedCacher {
                calculation,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.values.get(&arg) {
                Some(&v) => {
                    println!("  (cached for input {})", arg);
                    v
                }
                None => {
                    println!("  (calculating for input {})", arg);
                    let v = (self.calculation)(arg);
                    self.values.insert(arg, v);
                    v
                }
            }
        }
    }

    println!("\n--- Improved Cacher with HashMap ---");
    let mut improved_cache = ImprovedCacher::new(|x| x * x);

    println!("Calculate 5²:");
    println!("  Result: {}", improved_cache.value(5));

    println!("\nCalculate 10²:");
    println!("  Result: {}", improved_cache.value(10));

    println!("\nCalculate 5² again:");
    println!("  Result: {}", improved_cache.value(5));
}
