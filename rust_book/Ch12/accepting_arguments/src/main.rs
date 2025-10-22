//! # Chapter 12.1: Accepting Command Line Arguments
//!
//! Demonstrates basic command-line argument parsing from The Rust Book.
//! This is the first step in building our minigrep tool.
//!
//! Run with: `cargo run searchstring example-filename.txt`

use std::env;

fn example1_basic_args() {
    println!("🦀 Example 1: Basic Argument Collection");
    println!("=======================================");

    let args: Vec<String> = env::args().collect();
    
    println!("Program name: {}", args[0]);
    println!("Total arguments: {}", args.len());
    
    if args.len() > 1 {
        println!("Arguments provided:");
        for (i, arg) in args.iter().enumerate().skip(1) {
            println!("  [{}]: {}", i, arg);
        }
    } else {
        println!("No arguments provided");
    }
    
    println!();
}

fn example2_argument_parsing() {
    println!("🔧 Example 2: Parsing Expected Arguments");
    println!("=========================================");

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("❌ Usage: {} <search_query> <filename>", args[0]);
        println!("   Example: {} needle haystack.txt", args[0]);
        return;
    }
    
    let query = &args[1];
    let filename = &args[2];
    
    println!("✅ Parsed arguments successfully:");
    println!("   Search query: '{}'", query);
    println!("   Target file: '{}'", filename);
    
    println!();
}

fn example3_argument_validation() {
    println!("🎯 Example 3: Argument Validation");
    println!("==================================");

    let args: Vec<String> = env::args().collect();
    
    match validate_args(&args) {
        Ok((query, filename)) => {
            println!("✅ Valid arguments:");
            println!("   Query: '{}' (length: {})", query, query.len());
            println!("   File: '{}' (extension: {})", 
                    filename, 
                    filename.split('.').last().unwrap_or("none"));
        }
        Err(error_msg) => {
            println!("❌ Validation error: {}", error_msg);
        }
    }
    
    println!();
}

fn validate_args(args: &[String]) -> Result<(&str, &str), &'static str> {
    if args.len() < 3 {
        return Err("Not enough arguments. Expected: <query> <filename>");
    }
    
    if args.len() > 3 {
        return Err("Too many arguments. Expected: <query> <filename>");
    }
    
    let query = &args[1];
    let filename = &args[2];
    
    if query.is_empty() {
        return Err("Search query cannot be empty");
    }
    
    if !filename.contains('.') {
        return Err("Filename should have an extension");
    }
    
    Ok((query, filename))
}

fn example4_real_world_patterns() {
    println!("⚠️  Example 4: Real-World Argument Patterns");
    println!("============================================");

    let args: Vec<String> = env::args().collect();
    
    // Show different ways to handle arguments
    println!("🔍 Argument Analysis:");
    
    // Method 1: Direct indexing (risky)
    println!("❌ Risky approach (can panic):");
    println!("   // let query = &args[1];  // Could panic if no args!");
    
    // Method 2: Safe indexing with get()
    println!("✅ Safe approach with get():");
    if let Some(query) = args.get(1) {
        println!("   Query found: '{}'", query);
    } else {
        println!("   No query provided");
    }
    
    // Method 3: Pattern matching
    println!("✅ Pattern matching approach:");
    match args.len() {
        1 => println!("   No arguments provided"),
        2 => println!("   Only query provided: '{}'", args[1]),
        3 => println!("   Query: '{}', File: '{}'", args[1], args[2]),
        _ => println!("   Too many arguments: {}", args.len() - 1),
    }
    
    println!();
}

fn main() {
    println!("📚 Chapter 12.1: Accepting Command Line Arguments\n");

    example1_basic_args();
    example2_argument_parsing();
    example3_argument_validation();
    example4_real_world_patterns();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 12.2 or run examples in ../reading_files/");
    println!("💡 Try running: cargo run rust poem.txt");
}