//! # Chapter 12.3: Refactoring to Improve Modularity and Error Handling
//!
//! Demonstrates proper project organization by separating concerns.
//! This shows the evolution from a monolithic main.rs to a proper library structure.
//!
//! Run with: `cargo run needle poem.txt`

use std::env;
use std::error::Error;
use std::fs;
use std::process;

// Configuration struct to hold program arguments
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }

    // Alternative constructor using iterators (more idiomatic)
    pub fn from_iterator(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config { query, filename })
    }
}

// Main application logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("🔍 Searching for '{}' in file contents:", config.query);
    println!("========================================");

    let results = search(&config.query, &contents);
    
    if results.is_empty() {
        println!("No matches found.");
    } else {
        println!("Found {} matches:", results.len());
        for (line_num, line) in results {
            println!("  Line {}: {}", line_num, line);
        }
    }

    Ok(())
}

// Search function that returns line numbers and content
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    for (line_num, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((line_num + 1, line));
        }
    }

    results
}

// Case-insensitive search variant
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (line_num, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push((line_num + 1, line));
        }
    }

    results
}

fn example1_config_creation() {
    println!("🦀 Example 1: Configuration Management");
    println!("======================================");

    let test_args = vec![
        String::from("minigrep"),
        String::from("rust"),
        String::from("poem.txt"),
    ];

    match Config::new(&test_args) {
        Ok(config) => {
            println!("✅ Config created successfully:");
            println!("   Query: '{}'", config.query);
            println!("   Filename: '{}'", config.filename);
        }
        Err(e) => {
            println!("❌ Configuration error: {}", e);
        }
    }

    // Test error handling
    let invalid_args = vec![String::from("minigrep")];
    match Config::new(&invalid_args) {
        Ok(_) => println!("This shouldn't happen"),
        Err(e) => println!("✅ Properly caught error: {}", e),
    }

    println!();
}

fn example2_iterator_constructor() {
    println!("🔧 Example 2: Iterator-based Configuration");
    println!("===========================================");

    // Simulate command line args
    let _mock_args = ["minigrep".to_string(),
        "search_term".to_string(),
        "target_file.txt".to_string()];

    // For demonstration purposes, create a simple config
    println!("✅ Iterator config created (mock):");
    println!("   Query: 'search_term'");
    println!("   File: 'target_file.txt'");
    
    // Show how the real iterator version would work
    println!("   📝 Note: Config::from_iterator() would work with env::args()");
    
    // Test the from_iterator method with actual env::args (if available)
    match Config::from_iterator(env::args()) {
        Ok(config) => {
            println!("   ✅ Real iterator config would work:");
            println!("      Query: '{}'", config.query);
            println!("      File: '{}'", config.filename);
        }
        Err(e) => {
            println!("   ✅ Expected error (not enough args): {}", e);
        }
    }

    println!();
}

fn example3_search_functionality() {
    println!("🎯 Example 3: Search Function Testing");
    println!("=====================================");

    let sample_text = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape is silver.
Trust me on this.";

    // Test case-sensitive search
    let query1 = "rust";
    let results1 = search(query1, sample_text);
    println!("🔍 Case-sensitive search for '{}':", query1);
    if results1.is_empty() {
        println!("   No matches found");
    } else {
        for (line_num, line) in results1 {
            println!("   Line {}: {}", line_num, line);
        }
    }

    // Test case-insensitive search
    let query2 = "rust";
    let results2 = search_case_insensitive(query2, sample_text);
    println!("\n🔍 Case-insensitive search for '{}':", query2);
    for (line_num, line) in results2 {
        println!("   Line {}: {}", line_num, line);
    }

    println!();
}

fn example4_error_propagation() {
    println!("⚠️  Example 4: Error Propagation Patterns");
    println!("==========================================");

    println!("🔧 Demonstrating error handling approaches:");

    // Method 1: Using ? operator (requires Result return type)
    fn read_and_process(filename: &str) -> Result<usize, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?; // Error propagates up
        Ok(contents.lines().count())
    }

    // Test with existing file
    match read_and_process("poem.txt") {
        Ok(line_count) => println!("   ✅ File processed: {} lines", line_count),
        Err(e) => println!("   ❌ Processing error: {}", e),
    }

    // Test with non-existent file
    match read_and_process("nonexistent.txt") {
        Ok(line_count) => println!("   ✅ File processed: {} lines", line_count),
        Err(e) => println!("   ✅ Expected error caught: {}", e),
    }

    println!();
}

fn main() {
    println!("📚 Chapter 12.3: Refactoring for Modularity\n");

    example1_config_creation();
    example2_iterator_constructor();
    example3_search_functionality();
    example4_error_propagation();

    println!("🚀 Now running the actual minigrep application:");
    println!("===============================================");

    // Run the actual application logic
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: {} <query> <filename>", env::args().next().unwrap_or_else(|| "minigrep".to_string()));
        process::exit(1);
    });

    println!("Configuration: {:?}", config);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    println!("\n✅ All examples completed!");
    println!("📖 Next: Read Chapter 12.4 or run examples in ../testing/");
}