//! # Chapter 12.3: Refactoring to Improve Modularity and Error Handling
//!
//! Demonstrates proper project organization by separating concerns:
//! - main.rs: CLI argument handling and error display
//! - lib.rs: Core application logic and search functionality
//!
//! Run with: `cargo run needle poem.txt`
//! Set CASE_INSENSITIVE=1 for case-insensitive search

use std::env;
use std::process;

// Import our library functionality
use chapter12_refactoring::{Config, run};

fn demonstrate_refactoring() {
    println!("🦀 Chapter 12.3: Separation of Concerns Demonstration");
    println!("======================================================");
    
    println!("✅ Refactoring Benefits:");
    println!("   📁 main.rs: Handles CLI arguments and user interaction");  
    println!("   📚 lib.rs: Contains core search logic and tests");
    println!("   🔧 Modularity: Easy to test business logic separately");
    println!("   🛡️  Error Handling: Clean separation of concerns");
    
    println!("\n📝 Key Refactoring Changes:");
    println!("   • Config struct moved to lib.rs");
    println!("   • run() function moved to lib.rs");  
    println!("   • search() functions moved to lib.rs");
    println!("   • Tests added in lib.rs");
    println!("   • main.rs focuses only on CLI handling");
    
    println!("\n🎯 Usage Instructions:");
    println!("   cargo run <query> <filename>       # Case-sensitive search");
    println!("   CASE_INSENSITIVE=1 cargo run <query> <filename>  # Case-insensitive");
    
    println!();
}

fn main() {
    // Show what refactoring accomplished
    demonstrate_refactoring();
    
    // Parse command line arguments
    let config = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: {} <query> <filename>", env!("CARGO_PKG_NAME"));
        eprintln!("       CASE_INSENSITIVE=1 {} <query> <filename>", env!("CARGO_PKG_NAME"));
        process::exit(1);
    });

    // Run the application - all logic is now in lib.rs
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
