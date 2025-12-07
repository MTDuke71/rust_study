//! Chapter 19.2: Refutability - Whether a Pattern Might Fail to Match
//!
//! This example demonstrates the difference between refutable and irrefutable patterns,
//! where each can be used, and common compiler errors related to refutability.

fn main() {
    println!("=== Chapter 19.2: Refutability ===\n");
    
    demonstrate_irrefutable_patterns();
    demonstrate_refutable_patterns();
    demonstrate_refutability_rules();
    demonstrate_common_errors();
    
    println!("\n✅ Refutability concepts demonstrated!");
}

/// Irrefutable patterns - ALWAYS match
///
/// **Book Reference**: Section 19.2 - Irrefutable Patterns
/// **Key Learning**: Used in let, function parameters, for loops
fn demonstrate_irrefutable_patterns() {
    println!("--- Irrefutable Patterns (Always Match) ---");
    
    // 1. Simple variable binding - always matches
    let x = 5;
    println!("Simple binding: x = {}", x);
    
    // 2. Tuple destructuring - always matches (known size)
    let (a, b, c) = (1, 2, 3);
    println!("Tuple destructuring: ({}, {}, {})", a, b, c);
    
    // 3. Struct destructuring - always matches
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let Point { x, y } = Point { x: 10, y: 20 };
    println!("Struct destructuring: x={}, y={}", x, y);
    
    // 4. Function parameters - must be irrefutable
    fn process_tuple((a, b): (i32, i32)) {
        println!("Function param destructuring: a={}, b={}", a, b);
    }
    process_tuple((5, 10));
    
    // 5. for loops - use irrefutable patterns
    let points = vec![(1, 2), (3, 4), (5, 6)];
    println!("for loop destructuring:");
    for (x, y) in points {
        println!("  ({}, {})", x, y);
    }
    
    println!("💡 Irrefutable patterns ALWAYS succeed - no possibility of failure");
    println!();
}

/// Refutable patterns - MIGHT fail to match
///
/// **Book Reference**: Section 19.2 - Refutable Patterns  
/// **Key Learning**: Used in if let, while let, match arms
fn demonstrate_refutable_patterns() {
    println!("--- Refutable Patterns (Might Not Match) ---");
    
    // 1. if let - handles refutable pattern
    let some_option_value: Option<i32> = Some(5);
    if let Some(x) = some_option_value {
        println!("if let matched: x = {}", x);
    } else {
        println!("if let didn't match");
    }
    
    // 2. while let - loops while pattern matches
    let mut stack = vec![1, 2, 3];
    println!("while let draining stack:");
    while let Some(top) = stack.pop() {
        println!("  Popped: {}", top);
    }
    
    // 3. match arms - each arm can be refutable
    let value: Option<i32> = None;
    match value {
        Some(x) => println!("Got value: {}", x),  // Refutable
        None => println!("Got None"),             // Makes it exhaustive
    }
    
    // 4. Result matching - refutable patterns
    let result: Result<i32, &str> = Ok(42);
    if let Ok(value) = result {
        println!("Success: {}", value);
    }
    
    println!("💡 Refutable patterns might fail - need fallback handling");
    println!();
}

/// Rules for refutability
///
/// **Book Reference**: Section 19.2 - Where Patterns Are Allowed
/// **Key Learning**: Compiler enforces appropriate pattern types
fn demonstrate_refutability_rules() {
    println!("--- Refutability Rules ---");
    
    // RULE 1: let requires IRREFUTABLE patterns
    println!("✅ let accepts irrefutable patterns:");
    let (_x, _y) = (1, 2);  // ✅ Always matches
    println!("  let (x, y) = (1, 2) - works!");
    
    // ❌ let Some(x) = some_option;  // ERROR: refutable pattern in let
    println!("❌ let Some(x) = value would be compiler error");
    
    // RULE 2: if let accepts REFUTABLE patterns
    println!("\n✅ if let accepts refutable patterns:");
    if let Some(x) = Some(5) {
        println!("  if let Some(x) = Some(5) - works! x = {}", x);
    }
    
    // RULE 3: match accepts BOTH but requires exhaustiveness
    println!("\n✅ match requires exhaustive patterns:");
    let value = Some(10);
    match value {
        Some(x) => println!("  Matched Some({})", x),  // Refutable
        None => println!("  Matched None"),            // Exhaustive
    }
    
    // RULE 4: Function parameters require IRREFUTABLE
    println!("\n✅ Function parameters must be irrefutable:");
    fn take_pair((a, b): (i32, i32)) {  // ✅ Always matches
        println!("  Function got: ({}, {})", a, b);
    }
    take_pair((7, 8));
    
    // ❌ fn take_option(Some(x): Option<i32>) { }  // ERROR
    println!("❌ fn take_option(Some(x): Option<i32>) would error");
    
    println!("\n💡 Compiler enforces appropriate pattern types in each context");
    println!();
}

/// Common refutability errors and fixes
///
/// **Book Reference**: Section 19.2 - Fixing Refutability Errors
/// **Key Learning**: Converting between refutable and irrefutable contexts
fn demonstrate_common_errors() {
    println!("--- Common Errors and Fixes ---");
    
    // ERROR 1: Refutable pattern in let
    println!("Error 1: Refutable pattern in irrefutable context");
    let some_value = Some(5);
    
    // ❌ This would error:
    // let Some(x) = some_value;
    println!("  ❌ let Some(x) = value - COMPILER ERROR");
    
    // ✅ Fix: Use if let
    if let Some(x) = some_value {
        println!("  ✅ if let Some(x) = value - works! x = {}", x);
    }
    
    // ERROR 2: Irrefutable pattern in match (warning)
    println!("\nError 2: Irrefutable pattern in match");
    let value = 5;
    
    // ❌ This triggers warning (match unnecessary):
    match value {
        x => println!("  ⚠️  match with single irrefutable pattern: {}", x),
    }
    
    // ✅ Fix: Just use let
    let x = value;
    println!("  ✅ let x = value - simpler and clearer: {}", x);
    
    // ERROR 3: Missing None case in match
    println!("\nError 3: Non-exhaustive match");
    let opt_value: Option<i32> = Some(10);
    
    // ❌ This would error (non-exhaustive):
    // match opt_value {
    //     Some(x) => println!("{}", x),
    // }
    println!("  ❌ Missing None case - COMPILER ERROR");
    
    // ✅ Fix 1: Add None case
    match opt_value {
        Some(x) => println!("  ✅ Has value: {}", x),
        None => println!("  ✅ No value"),
    }
    
    // ✅ Fix 2: Use if let if only care about Some
    if let Some(x) = opt_value {
        println!("  ✅ if let - only handle Some case: {}", x);
    }
    
    println!("\n💡 Understanding refutability prevents common compilation errors");
    println!();
}

/// Practical example: Processing optional configuration
fn _demonstrate_practical_example() {
    println!("--- Practical Example: Configuration Processing ---");
    
    #[derive(Debug)]
    struct Config {
        database_url: Option<String>,
        port: Option<u16>,
        workers: Option<usize>,
    }
    
    let config = Config {
        database_url: Some("postgres://localhost".to_string()),
        port: None,
        workers: Some(4),
    };
    
    // Using refutable patterns appropriately
    println!("Processing configuration:");
    
    // if let for optional values
    if let Some(ref url) = config.database_url {
        println!("  Database: {}", url);
    } else {
        println!("  Database: using default");
    }
    
    // Unwrap or default pattern
    let port = config.port.unwrap_or(8080);
    println!("  Port: {}", port);
    
    // match for comprehensive handling
    match config.workers {
        Some(n) if n > 0 => println!("  Workers: {}", n),
        Some(0) => println!("  Workers: invalid (0)"),
        None => println!("  Workers: using default (1)"),
        _ => unreachable!(),
    }
    
    println!("\n💡 Choose refutable/irrefutable patterns based on context");
}
