// Chapter 13.3: Improving Our I/O Project
// Run with: cargo run --example ch13_3_io_project

use std::env;
use std::fs;

fn main() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║  Chapter 13.3: Improving I/O Project         ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    // Demonstrate the improvements from Chapter 12 to Chapter 13
    println!("=== Comparing Old vs New Approaches ===\n");
    
    // Part 1: Argument Parsing
    println!("=== Part 1: Argument Parsing Improvements ===");
    demonstrate_arg_parsing();
    
    // Part 2: Iterator Usage in Search
    println!("\n=== Part 2: Using Iterators in Search ===");
    demonstrate_search_improvements();
    
    // Part 3: Performance Comparison
    println!("\n=== Part 3: Iterator Performance ===");
    demonstrate_performance();
}

// ============================================================================
// Part 1: Argument Parsing - Old vs New
// ============================================================================

// OLD WAY: Manual indexing and cloning (Chapter 12)
#[allow(dead_code)]
fn parse_config_old(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    
    let query = args[1].clone();     // Cloning strings
    let file_path = args[2].clone(); // Cloning strings
    
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    
    Ok(Config {
        query,
        file_path,
        ignore_case,
    })
}

// NEW WAY: Using iterators (Chapter 13)
fn parse_config_new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next(); // Skip program name
    
    let query = match args.next() {
        Some(arg) => arg,      // Takes ownership, no clone needed!
        None => return Err("Didn't get a query string"),
    };
    
    let file_path = match args.next() {
        Some(arg) => arg,      // Takes ownership, no clone needed!
        None => return Err("Didn't get a file path"),
    };
    
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    
    Ok(Config {
        query,
        file_path,
        ignore_case,
    })
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

fn demonstrate_arg_parsing() {
    println!("Old way problems:");
    println!("  • Requires indexing (can panic)");
    println!("  • Clones strings (memory overhead)");
    println!("  • Manual bounds checking");
    
    println!("\nNew way benefits:");
    println!("  • Iterator takes ownership (no cloning!)");
    println!("  • Safer with Option handling");
    println!("  • More functional and expressive");
    
    // Simulate command line args
    let args = vec![
        "program".to_string(),
        "search_term".to_string(),
        "filename.txt".to_string(),
    ];
    
    // Old way
    match parse_config_old(&args) {
        Ok(config) => println!("\nOld way result: {:?}", config),
        Err(e) => println!("Error: {}", e),
    }
    
    // New way
    match parse_config_new(args.into_iter()) {
        Ok(config) => println!("New way result: {:?}", config),
        Err(e) => println!("Error: {}", e),
    }
}

// ============================================================================
// Part 2: Search Function - Old vs New
// ============================================================================

// OLD WAY: Explicit looping and mutation (Chapter 12)
#[allow(dead_code)]
fn search_old<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

// NEW WAY: Using iterator adaptors (Chapter 13)
fn search_new<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// Case-insensitive versions
#[allow(dead_code)]
fn search_case_insensitive_old<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
}

fn search_case_insensitive_new<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn demonstrate_search_improvements() {
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";
    
    let query = "duct";
    
    println!("Searching for '{}' in text:", query);
    println!("{}", contents);
    
    // Case-sensitive search
    println!("\n--- Case-sensitive search ---");
    let results_old = search_old(query, contents);
    println!("Old way results: {:?}", results_old);
    
    let results_new = search_new(query, contents);
    println!("New way results: {:?}", results_new);
    
    // Case-insensitive search
    println!("\n--- Case-insensitive search ---");
    let results_old_ci = search_case_insensitive_old(query, contents);
    println!("Old way results: {:?}", results_old_ci);
    
    let results_new_ci = search_case_insensitive_new(query, contents);
    println!("New way results: {:?}", results_new_ci);
    
    println!("\nIterator advantages:");
    println!("  • Less code, more readable");
    println!("  • No mutable state");
    println!("  • Functional composition");
    println!("  • Same performance as loops!");
}

// ============================================================================
// Part 3: Performance Analysis
// ============================================================================

fn demonstrate_performance() {
    let large_text = "the quick brown fox jumps over the lazy dog\n".repeat(1000);
    let query = "fox";
    
    // Both approaches have the same performance
    let _results_old = search_old(query, &large_text);
    let _results_new = search_new(query, &large_text);
    
    println!("Performance comparison:");
    println!("  • Iterators are 'zero-cost abstractions'");
    println!("  • Compile to same assembly as hand-written loops");
    println!("  • No runtime overhead");
    println!("  • Compiler optimizations apply equally");
    
    println!("\nWhy iterators are better:");
    println!("  1. More expressive - intent is clearer");
    println!("  2. Less error-prone - no index manipulation");
    println!("  3. Composable - easy to chain operations");
    println!("  4. Functional style - easier to reason about");
    println!("  5. Same speed as loops - thanks to compiler");
}

// ============================================================================
// Complete Improved minigrep Example
// ============================================================================

#[allow(dead_code)]
fn run_minigrep(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive_new(&config.query, &contents)
    } else {
        search_new(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

// Example of how to use in main()
#[allow(dead_code)]
fn minigrep_main() {
    let config = parse_config_new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            std::process::exit(1);
        });
    
    if let Err(e) = run_minigrep(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

// ============================================================================
// Additional Iterator Patterns from I/O Project
// ============================================================================

#[allow(dead_code)]
fn additional_patterns() {
    println!("\n=== Additional Iterator Patterns ===");
    
    // Pattern 1: Filtering and collecting
    let lines = vec!["Hello", "World", "Rust", "is", "awesome"];
    let long_words: Vec<_> = lines
        .iter()
        .filter(|word| word.len() > 4)
        .collect();
    println!("Words longer than 4 chars: {:?}", long_words);
    
    // Pattern 2: Transforming with map
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers
        .iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubled numbers: {:?}", doubled);
    
    // Pattern 3: Chaining operations
    let result: Vec<_> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Squared even numbers: {:?}", result);
    
    // Pattern 4: Using find
    let first_long = lines.iter().find(|word| word.len() > 5);
    println!("First word longer than 5: {:?}", first_long);
    
    // Pattern 5: Using any and all
    let has_rust = lines.iter().any(|word| *word == "Rust");
    let all_caps = lines.iter().all(|word| word.chars().next().unwrap().is_uppercase());
    println!("Has 'Rust': {}, All start with caps: {}", has_rust, all_caps);
}

// ============================================================================
// Key Takeaways
// ============================================================================

#[allow(dead_code)]
fn print_key_takeaways() {
    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║  Key Takeaways from Chapter 13.3             ║");
    println!("╚═══════════════════════════════════════════════╝");
    
    println!("\n1. Iterators eliminate cloning:");
    println!("   - env::args() returns an iterator");
    println!("   - Can take ownership of values directly");
    println!("   - No need for .clone() on arguments");
    
    println!("\n2. Iterator adaptors make code clearer:");
    println!("   - filter() expresses intent better than if statements");
    println!("   - collect() builds results without mut Vec");
    println!("   - Functional style is more declarative");
    
    println!("\n3. Zero-cost abstractions:");
    println!("   - Iterators compile to same code as loops");
    println!("   - No runtime performance penalty");
    println!("   - Can write high-level code with low-level performance");
    
    println!("\n4. Iterator methods are composable:");
    println!("   - Chain operations together");
    println!("   - Build complex logic from simple pieces");
    println!("   - Easier to test individual transformations");
    
    println!("\n5. Safer code:");
    println!("   - No index out of bounds errors");
    println!("   - Type system ensures correctness");
    println!("   - Compiler catches mistakes at compile time");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        let results = search_new(query, contents);
        assert_eq!(results, vec!["safe, fast, productive."]);
    }
    
    #[test]
    fn test_search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        let results = search_case_insensitive_new(query, contents);
        assert_eq!(results, vec!["Rust:", "Trust me."]);
    }
    
    #[test]
    fn test_parse_config() {
        let args = vec![
            "program".to_string(),
            "query".to_string(),
            "file.txt".to_string(),
        ];
        
        let config = parse_config_new(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
    }
    
    #[test]
    fn test_parse_config_error() {
        let args = vec!["program".to_string()];
        let result = parse_config_new(args.into_iter());
        assert!(result.is_err());
    }
    
    #[test]
    fn test_search_comparison() {
        let query = "test";
        let contents = "This is a test\nAnother line\ntest again";
        
        let old_results = search_old(query, contents);
        let new_results = search_new(query, contents);
        
        assert_eq!(old_results, new_results);
    }
}
