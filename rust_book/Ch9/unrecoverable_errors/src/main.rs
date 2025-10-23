/// # Chapter 9.1 - Unrecoverable Errors with panic!
///
/// This module demonstrates when and how to use panic! for unrecoverable errors.
///
/// ## Key Concepts:
/// 1. When to panic vs when to return Result
/// 2. Stack unwinding vs abort
/// 3. Backtrace information
/// 4. Index out of bounds panics
///
/// ## Examples:
/// - Explicit panics with panic!
/// - Index bounds checking
/// - Safe alternatives to panicking operations
/// - Backtrace debugging techniques
fn main() {
    println!("=== Chapter 9.1: Unrecoverable Errors with panic! ===\n");

    // Example 1: When to use panic!
    demonstrate_panic_use_cases();

    // Example 2: Index out of bounds
    demonstrate_bounds_checking();

    // Example 3: Safe alternatives
    demonstrate_safe_alternatives();

    // Example 4: Backtrace information
    demonstrate_backtrace_info();

    // Example 5: Custom panic hook
    demonstrate_panic_hook();
}

/// Demonstrates appropriate use cases for panic!
fn demonstrate_panic_use_cases() {
    println!("1. When to use panic!:");
    println!("   ✅ Contract violations (programming bugs)");
    println!("   ✅ Impossible states in your logic");
    println!("   ✅ Prototype and example code");
    println!("   ✅ Test assertions");
    println!("   ❌ Expected failures (file not found)");
    println!("   ❌ User input errors\n");

    // Example of a contract violation
    fn divide_by_known_nonzero(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Division by zero - this should never happen in our logic!");
        }
        a / b
    }

    println!(
        "   Safe division example: 10 ÷ 2 = {}",
        divide_by_known_nonzero(10, 2)
    );
    // divide_by_known_nonzero(10, 0);  // Would panic!
    println!("   (Panic case commented out to prevent termination)\n");
}

/// Demonstrates index bounds checking and panics
fn demonstrate_bounds_checking() {
    println!("2. Index bounds checking:");

    let v = vec![1, 2, 3];
    println!("   Vector: {:?}", v);

    // Safe access
    println!("   v[1] = {}", v[1]); // This is safe

    // Unsafe access (would panic)
    println!("   Accessing v[10] would panic with message:");
    println!("   'index out of bounds: the len is 3 but the index is 10'");

    // Demonstrating the panic with a smaller out-of-bounds access
    // Uncomment to see the panic:
    // println!("   v[10] = {}", v[10]);

    println!();
}

/// Demonstrates safe alternatives to operations that might panic
fn demonstrate_safe_alternatives() {
    println!("3. Safe alternatives to panicking operations:");

    let v = [10, 20, 30];

    // Instead of v[index] which panics on invalid index
    println!("   Using .get() instead of direct indexing:");

    for i in 0..=5 {
        match v.get(i) {
            Some(value) => println!("   v.get({}) = Some({})", i, value),
            None => println!("   v.get({}) = None (no panic!)", i),
        }
    }

    println!();

    // Safe division
    println!("   Safe division with Option:");
    fn safe_divide(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }

    let test_cases = vec![(10, 2), (10, 0), (15, 3)];
    for (a, b) in test_cases {
        match safe_divide(a, b) {
            Some(result) => println!("   {} ÷ {} = {}", a, b, result),
            None => println!("   {} ÷ {} = undefined (division by zero)", a, b),
        }
    }

    println!();
}

/// Demonstrates backtrace information for debugging panics
fn demonstrate_backtrace_info() {
    println!("4. Backtrace debugging:");
    println!("   Set environment variable RUST_BACKTRACE=1 for backtrace");
    println!("   Set RUST_BACKTRACE=full for detailed backtrace");
    println!(
        "   
   Example PowerShell commands:
   $env:RUST_BACKTRACE=1; cargo run
   $env:RUST_BACKTRACE=\"full\"; cargo run
   "
    );

    // Function call chain to demonstrate backtrace
    fn level_1() -> i32 {
        level_2()
    }

    fn level_2() -> i32 {
        level_3()
    }

    fn level_3() -> i32 {
        // This would show the call chain in backtrace
        // panic!("This panic will show the call stack!");
        println!("   Call chain: main -> level_1 -> level_2 -> level_3");
        println!("   (Panic commented out to prevent termination)");
        42
    }

    let result = level_1();
    println!("   Result from call chain: {}\n", result);
}

/// Demonstrates custom panic hooks for specialized panic handling
fn demonstrate_panic_hook() {
    println!("5. Custom panic hooks:");

    // Set up a custom panic hook
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("🚨 Custom panic handler activated!");

        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("   Panic message: {}", s);
        }

        if let Some(location) = panic_info.location() {
            eprintln!("   Location: {}:{}", location.file(), location.line());
        }

        eprintln!("   This is where you might log to a file, send to monitoring, etc.");
    }));

    println!("   Custom panic hook installed (handles panic! calls)");
    println!("   You can customize panic behavior for logging, monitoring, etc.");

    // Example of catching panics without terminating
    let result = std::panic::catch_unwind(|| {
        // This would normally panic and terminate
        // panic!("This panic is caught!");
        println!("   std::panic::catch_unwind() can capture panics");
        "Success"
    });

    match result {
        Ok(value) => println!("   Caught panic result: Ok(\"{}\")", value),
        Err(_) => println!("   Caught panic result: Err(...)"),
    }

    // Reset to default panic behavior
    let _ = std::panic::take_hook();
    println!("   Panic hook reset to default\n");
}

#[cfg(test)]
mod tests {
    

    #[test]
    #[should_panic(expected = "This should panic")]
    fn test_expected_panic() {
        panic!("This should panic");
    }

    #[test]
    fn test_panic_catch_unwind() {
        let result = std::panic::catch_unwind(|| {
            panic!("Test panic");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_indexing() {
        let v = [1, 2, 3];
        assert_eq!(v.first(), Some(&1));
        assert_eq!(v.get(10), None);
    }
}
