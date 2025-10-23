//! # Chapter 12.5: Working with Environment Variables
//!
//! Demonstrates how to use environment variables to control program behavior.
//! Shows case-sensitive vs case-insensitive search based on environment settings.
//!
//! Run with: `cargo run needle poem.txt`
//! Run case-insensitive: `CASE_INSENSITIVE=1 cargo run needle poem.txt`

use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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

        // Check environment variable for case sensitivity
        // If CASE_INSENSITIVE is set (to any value), use case-insensitive search
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    let search_type = if config.case_sensitive {
        "case-sensitive"
    } else {
        "case-insensitive"
    };

    if results.is_empty() {
        println!("No matches found for '{}' ({} search)", config.query, search_type);
    } else {
        println!(
            "Found {} matches for '{}' ({} search):",
            results.len(),
            config.query,
            search_type
        );
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

fn example1_environment_basics() {
    println!("🦀 Example 1: Environment Variable Basics");
    println!("==========================================");

    // Show current environment variable status
    match env::var("CASE_INSENSITIVE") {
        Ok(val) => {
            println!("✅ CASE_INSENSITIVE is set to: '{}'", val);
            println!("   → Using case-insensitive search");
        }
        Err(_) => {
            println!("❌ CASE_INSENSITIVE is not set");
            println!("   → Using case-sensitive search (default)");
        }
    }

    // Show other common environment variables
    println!("\n🔍 Other environment variables:");
    if let Ok(user) = env::var("USER") {
        println!("   USER: {}", user);
    } else if let Ok(username) = env::var("USERNAME") {
        println!("   USERNAME: {}", username);
    }

    if let Ok(home) = env::var("HOME") {
        println!("   HOME: {}", home);
    } else if let Ok(userprofile) = env::var("USERPROFILE") {
        println!("   USERPROFILE: {}", userprofile);
    }

    println!();
}

fn example2_config_with_environment() {
    println!("🔧 Example 2: Configuration with Environment");
    println!("=============================================");

    // Simulate different argument scenarios
    let _test_args = ["minigrep".to_string(),
        "rust".to_string(),
        "poem.txt".to_string()];

    // Create a mock config for demonstration
    let query = "rust".to_string();
    let filename = "poem.txt".to_string();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
    let config = Config { query, filename, case_sensitive };
    
    // Show the config
    {
        println!("✅ Config created:");
        println!("   Query: '{}'", config.query);
        println!("   File: '{}'", config.filename);
        println!("   Case sensitive: {}", config.case_sensitive);
    }

    // Demonstrate manual environment variable checking
    println!("\n🔍 Manual environment variable checking:");
    
    let env_vars_to_check = vec![
        "CASE_INSENSITIVE",
        "RUST_LOG", 
        "DEBUG",
        "NONEXISTENT_VAR"
    ];

    for var_name in env_vars_to_check {
        match env::var(var_name) {
            Ok(value) => println!("   {}: '{}'", var_name, value),
            Err(env::VarError::NotPresent) => println!("   {}: (not set)", var_name),
            Err(env::VarError::NotUnicode(_)) => println!("   {}: (invalid unicode)", var_name),
        }
    }

    println!();
}

fn example3_search_behavior_comparison() {
    println!("🎯 Example 3: Search Behavior Comparison");
    println!("========================================");

    let sample_text = "\
Rust is great!
RUST is powerful.
trust me on this.
Must learn Rust.";

    let query = "rust";

    // Always show both search types for comparison
    println!("🔍 Sample text:");
    for (i, line) in sample_text.lines().enumerate() {
        println!("   {}: {}", i + 1, line);
    }

    println!("\n🔍 Searching for '{}':", query);
    
    // Case-sensitive search
    let results_sensitive = search(query, sample_text);
    println!("   Case-sensitive results:");
    if results_sensitive.is_empty() {
        println!("     (no matches)");
    } else {
        for line in &results_sensitive {
            println!("     {}", line);
        }
    }

    // Case-insensitive search
    let results_insensitive = search_case_insensitive(query, sample_text);
    println!("   Case-insensitive results:");
    for line in &results_insensitive {
        println!("     {}", line);
    }

    // Show what the current config would use
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    println!("\n📋 Current configuration would use:");
    if case_sensitive {
        println!("   → Case-sensitive ({} matches)", results_sensitive.len());
    } else {
        println!("   → Case-insensitive ({} matches)", results_insensitive.len());
    }

    println!();
}

fn example4_environment_best_practices() {
    println!("⚠️  Example 4: Environment Variable Best Practices");
    println!("===================================================");

    println!("🎯 Best practices for environment variables:");
    
    // 1. Provide defaults
    println!("\n1️⃣  Provide sensible defaults:");
    let debug_mode = env::var("DEBUG").unwrap_or_else(|_| "false".to_string());
    println!("   DEBUG mode: {}", debug_mode);

    // 2. Handle different value types
    println!("\n2️⃣  Handle boolean-like values:");
    let is_verbose = matches!(env::var("VERBOSE").as_deref(), Ok("1") | Ok("true") | Ok("yes") | Ok("on"));
    println!("   Verbose mode: {}", is_verbose);

    // 3. Parse numeric values safely
    println!("\n3️⃣  Parse numeric values safely:");
    let max_results: usize = env::var("MAX_RESULTS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    println!("   Max results: {}", max_results);

    // 4. Document expected environment variables
    println!("\n4️⃣  Document environment variables:");
    println!("   CASE_INSENSITIVE - If set (any value), enables case-insensitive search");
    println!("   DEBUG - Set to 'true' to enable debug output");
    println!("   VERBOSE - Set to '1' to enable verbose output");
    println!("   MAX_RESULTS - Number (default: 10) for maximum search results");

    // 5. Show all environment variables (sample)
    println!("\n5️⃣  Current environment sample:");
    for (key, value) in env::vars().take(5) {
        println!("   {}={}", key, value);
    }
    println!("   ... ({} total environment variables)", env::vars().count());

    println!();
}

fn main() {
    println!("📚 Chapter 12.5: Working with Environment Variables\n");

    example1_environment_basics();
    example2_config_with_environment();
    example3_search_behavior_comparison();
    example4_environment_best_practices();

    // Create sample file if it doesn't exist
    if !std::path::Path::new("poem.txt").exists() {
        let sample_content = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public like a frog
To tell one's name the livelong day
To an admiring bog!";

        if let Err(e) = fs::write("poem.txt", sample_content) {
            eprintln!("Warning: Could not create poem.txt: {}", e);
        } else {
            println!("📄 Created poem.txt for testing");
        }
    }

    println!("\n🚀 Running minigrep application:");
    println!("=================================");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: minigrep <query> <filename>");
        eprintln!("Environment: Set CASE_INSENSITIVE=1 for case-insensitive search");
        process::exit(1);
    });

    println!("Configuration: {:?}", config);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    println!("\n✅ All examples completed!");
    println!("💡 Try setting CASE_INSENSITIVE=1 and run again!");
    println!("📖 Next: Review this chapter or proceed to Chapter 13");
}

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
}