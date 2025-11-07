//! Debug Symbols vs Debug Assertions Demonstration
//! 
//! This example shows the difference between debug symbols and debug assertions

fn main() {
    println!("=== Debug Symbols vs Debug Assertions ===\n");
    
    // Debug assertions - these are RUNTIME checks
    demo_debug_assertions();
    
    // Debug symbols - these affect DEBUGGING but not runtime behavior
    demo_debug_symbols();
}

fn demo_debug_assertions() {
    println!("1. DEBUG ASSERTIONS (runtime checks):");
    
    let x = 5;
    let y = 1; // Changed to avoid panic for demo
    
    // This will check in dev mode, be removed in release mode
    debug_assert!(x > 0, "x must be positive");
    debug_assert_ne!(y, 0, "y cannot be zero for division");
    
    // Check if debug assertions are compiled in
    #[cfg(debug_assertions)]
    println!("   ✅ Debug assertions are ENABLED - runtime checks active");
    
    #[cfg(not(debug_assertions))]
    println!("   ❌ Debug assertions are DISABLED - runtime checks removed");
    
    // Integer overflow checking (also a debug assertion)
    let a: u8 = 200;
    let b: u8 = 100;
    
    #[cfg(debug_assertions)]
    {
        println!("   📊 Overflow checks: ENABLED");
        // This would panic in debug mode if it overflowed
        let _result = a.wrapping_add(b); // Use wrapping to avoid panic for demo
    }
    
    #[cfg(not(debug_assertions))]
    {
        println!("   📊 Overflow checks: DISABLED");
        let _result = a.wrapping_add(b); // Use wrapping to demonstrate concept
    }
    
    println!();
}

fn demo_debug_symbols() {
    println!("2. DEBUG SYMBOLS (debugging metadata):");
    
    // These function calls will have different debugging info based on debug symbols
    let result = calculate_something(42);
    let processed = process_data(result);
    
    println!("   📁 Binary contains debug symbols: {}", has_debug_symbols());
    println!("   🎯 Stack traces will show: {}", stack_trace_quality());
    println!("   🔍 Debugger can show: {}", debugger_features());
    println!("   📏 Binary size impact: {}", binary_size_impact());
    println!("   ⚡ Runtime performance impact: NONE");
    
    println!("   Final result: {}", processed);
}

fn calculate_something(input: i32) -> i32 {
    // With debug symbols: debugger can step through this, show variable names
    // Without debug symbols: debugger sees assembly only
    let intermediate = input * 2;
    
    intermediate + 10
}

fn process_data(data: i32) -> String {
    // With debug symbols: stack traces show function names and line numbers
    // Without debug symbols: stack traces show memory addresses only
    format!("Processed: {}", data)
}

fn has_debug_symbols() -> &'static str {
    // This is determined at compile time by the profile's debug setting
    if cfg!(debug_assertions) {
        "YES (debug profile usually includes symbols)"
    } else {
        "DEPENDS on profile.debug setting"
    }
}

fn stack_trace_quality() -> &'static str {
    if cfg!(debug_assertions) {
        "Function names, file names, line numbers"
    } else {
        "May only show memory addresses (unless debug=true in profile)"
    }
}

fn debugger_features() -> &'static str {
    if cfg!(debug_assertions) {
        "Variable names, types, source code mapping"
    } else {
        "Limited (unless debug=true in profile)"
    }
}

fn binary_size_impact() -> &'static str {
    if cfg!(debug_assertions) {
        "Larger binary (debug info included)"
    } else {
        "Smaller binary (unless debug=true in profile)"
    }
}