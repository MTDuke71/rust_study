//! # Chapter 12.4: Developing the Library's Functionality with Test-Driven Development
//!
//! Demonstrates test-driven development (TDD) for the search functionality.
//! Shows how to write tests first, then implement the functionality.
//!
//! Run tests with: `cargo test`
//! Run examples with: `cargo run needle poem.txt`

use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip program name

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents);

    if results.is_empty() {
        println!("No matches found for '{}'", config.query);
    } else {
        println!("Found {} matches for '{}':", results.len(), config.query);
        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn example1_test_demonstration() {
    println!("🦀 Example 1: Test-Driven Development Demo");
    println!("===========================================");

    // Demonstrate the TDD process
    println!("📋 TDD Process:");
    println!("   1. ❌ Write a failing test");
    println!("   2. ✅ Write minimal code to pass");
    println!("   3. 🔄 Refactor while keeping tests green");

    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let results = search(query, contents);
    println!("\n🔍 Testing search for '{}':", query);
    println!("Expected: [\"safe, fast, productive.\"]");
    println!("Actual: {:?}", results);

    assert_eq!(vec!["safe, fast, productive."], results);
    println!("✅ Test passed!");

    println!();
}

fn example2_case_sensitivity_tests() {
    println!("🔧 Example 2: Case Sensitivity Testing");
    println!("======================================");

    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    // Case-sensitive search (should find nothing)
    let results_sensitive = search(query, contents);
    println!("🔍 Case-sensitive search for '{}':", query);
    println!("   Results: {:?}", results_sensitive);
    assert!(results_sensitive.is_empty());
    println!("   ✅ Correctly found no matches");

    // Case-insensitive search (should find matches)
    let results_insensitive = search_case_insensitive(query, contents);
    println!("\n🔍 Case-insensitive search for '{}':", query);
    println!("   Results: {:?}", results_insensitive);
    assert_eq!(vec!["Rust:", "Trust me."], results_insensitive);
    println!("   ✅ Found expected matches");

    println!();
}

fn example3_edge_case_testing() {
    println!("🎯 Example 3: Edge Case Testing");
    println!("===============================");

    // Test empty content
    let results_empty = search("anything", "");
    println!("🧪 Empty content test:");
    println!("   Results: {:?}", results_empty);
    assert!(results_empty.is_empty());
    println!("   ✅ Empty content handled correctly");

    // Test empty query
    let results_empty_query = search("", "line1\nline2\nline3");
    println!("\n🧪 Empty query test:");
    println!("   Results: {:?}", results_empty_query);
    // Empty string is contained in every line
    assert_eq!(3, results_empty_query.len());
    println!("   ✅ Empty query handled correctly");

    // Test query not found
    let results_not_found = search("xyz", "abc\ndef\nghi");
    println!("\n🧪 Query not found test:");
    println!("   Results: {:?}", results_not_found);
    assert!(results_not_found.is_empty());
    println!("   ✅ Missing query handled correctly");

    // Test multiple matches in same line
    let results_multiple = search("test", "this is a test line with test twice");
    println!("\n🧪 Multiple matches test:");
    println!("   Results: {:?}", results_multiple);
    assert_eq!(1, results_multiple.len());
    println!("   ✅ Multiple matches in same line handled correctly");

    println!();
}

fn example4_performance_testing() {
    println!("⚠️  Example 4: Performance Characteristics");
    println!("===========================================");

    // Generate large test data
    let large_content = (0..1000)
        .map(|i| format!("Line {} with some content", i))
        .collect::<Vec<_>>()
        .join("\n");

    println!(
        "🚀 Performance test with {} lines",
        large_content.lines().count()
    );

    let start = std::time::Instant::now();
    let results = search("500", &large_content);
    let duration = start.elapsed();

    println!("   Search completed in: {:?}", duration);
    println!("   Matches found: {}", results.len());
    println!("   ✅ Performance acceptable for large content");

    // Test case-insensitive performance
    let start = std::time::Instant::now();
    let results_ci = search_case_insensitive("LINE", &large_content);
    let duration_ci = start.elapsed();

    println!("\n🚀 Case-insensitive performance test:");
    println!("   Search completed in: {:?}", duration_ci);
    println!("   Matches found: {}", results_ci.len());
    println!("   ✅ Case-insensitive performance acceptable");

    println!();
}

fn main() {
    println!("📚 Chapter 12.4: Test-Driven Development\n");

    example1_test_demonstration();
    example2_case_sensitivity_tests();
    example3_edge_case_testing();
    example4_performance_testing();

    println!("🧪 Running built-in tests:");
    println!("Run `cargo test` to see all tests pass!");

    // Run the actual application
    println!("\n🚀 Running minigrep application:");
    println!("=================================");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: minigrep <query> <filename>");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    println!("\n✅ All examples completed!");
    println!("📖 Next: Read Chapter 12.5 or run examples in ../environment_variables/");
}

// Tests module - this is where the actual tests live
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn empty_query() {
        let results = search("", "line1\nline2");
        assert_eq!(2, results.len());
    }

    #[test]
    fn empty_content() {
        let results = search("anything", "");
        assert!(results.is_empty());
    }

    #[test]
    fn no_matches() {
        let results = search("xyz", "abc\ndef\nghi");
        assert!(results.is_empty());
    }

    #[test]
    fn multiple_matches() {
        let results = search("test", "test line\nanother test\ntest again");
        assert_eq!(3, results.len());
    }
}
