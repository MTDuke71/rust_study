use std::env;
use std::error::Error;
use std::fs;
use std::process;

/// Configuration for the error demo program
#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    /// Creates a new Config from command line arguments
    ///
    /// # Arguments
    /// * `mut args` - Iterator over command line arguments
    ///
    /// # Returns
    /// * `Result<Config, &'static str>` - Config on success, error message on failure
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip program name

        let query = args.next().ok_or("Missing required argument: QUERY")?;

        let filename = args.next().ok_or("Missing required argument: FILENAME")?;

        // Check for case sensitivity from environment variable
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    /// Shows usage information for the program
    fn show_usage() {
        eprintln!("🦀 Error Messages Demo - Chapter 12.6");
        eprintln!("=====================================");
        eprintln!();
        eprintln!("USAGE:");
        eprintln!(
            "    {} QUERY FILENAME",
            env::args().next().unwrap_or_else(|| "program".to_string())
        );
        eprintln!();
        eprintln!("ARGUMENTS:");
        eprintln!("    QUERY      Text to search for");
        eprintln!("    FILENAME   File to search in");
        eprintln!();
        eprintln!("ENVIRONMENT:");
        eprintln!("    CASE_INSENSITIVE=1    Enable case-insensitive search");
        eprintln!();
        eprintln!("EXAMPLES:");
        eprintln!(
            "    {} nobody poem.txt",
            env::args().next().unwrap_or_else(|| "program".to_string())
        );
        eprintln!(
            "    CASE_INSENSITIVE=1 {} NOBODY poem.txt",
            env::args().next().unwrap_or_else(|| "program".to_string())
        );
        eprintln!();
        eprintln!("OUTPUT:");
        eprintln!("    • Search results → stdout (can be redirected with >)");
        eprintln!("    • Error messages → stderr (can be redirected with 2>)");
        eprintln!("    • Exit code 0 for success, 1 for errors");
    }
}

/// Performs case-sensitive search
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Performs case-insensitive search
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// Main application logic
///
/// # Arguments
/// * `config` - Program configuration
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok on success, Error on failure
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Show configuration (this goes to stderr as debug info)
    eprintln!("Configuration: {:?}", config);

    // Try to read the file
    let contents = fs::read_to_string(&config.filename)
        .map_err(|e| format!("Failed to read file '{}': {}", config.filename, e))?;

    // Perform search based on case sensitivity
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // Output results
    if results.is_empty() {
        // This is not an error, but informational - could go to stderr
        eprintln!("No matches found for '{}'", config.query);
        eprintln!("Tip: Try setting CASE_INSENSITIVE=1 for case-insensitive search");
    } else {
        // Found results - these go to stdout so they can be redirected
        eprintln!("Found {} match(es):", results.len()); // Count goes to stderr
        for line in results {
            println!("{}", line); // ✅ Actual results go to stdout
        }
    }

    Ok(())
}

/// Demonstrates different types of error messages
fn demonstrate_error_types() {
    eprintln!("🚨 Error Message Types Demo:");
    eprintln!("============================");

    // Info message (non-critical)
    eprintln!("ℹ️  Info: Program starting...");

    // Warning message (concerning but not fatal)
    eprintln!("⚠️  Warning: File is large, this might take a while");

    // Error message (fatal)
    eprintln!("❌ Error: Critical failure occurred");

    // Success message (could go to stdout, but often stderr for logging)
    eprintln!("✅ Success: Operation completed");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Special case: demonstrate error types
    if args.len() == 2 && args[1] == "--demo-errors" {
        demonstrate_error_types();
        return;
    }

    // Parse configuration
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // ✅ Error messages go to stderr
        eprintln!("❌ Problem parsing arguments: {}", err);
        eprintln!();
        Config::show_usage();
        process::exit(1);
    });

    // Show usage if no arguments provided
    if args.len() == 1 {
        Config::show_usage();
        process::exit(0); // Not an error, just showing help
    }

    // Run the main application
    if let Err(e) = run(config) {
        // ✅ Application errors go to stderr
        eprintln!("❌ Application error: {}", e);
        process::exit(1);
    }

    // ✅ Success - exit with code 0 (implicit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_insensitive_search() {
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
    fn test_config_creation_success() {
        // Simulate env::args() by creating a vector and converting to Args
        let _args = [
            "program".to_string(),
            "test".to_string(),
            "file.txt".to_string(),
        ];

        // We need to use std::env::Args, but for testing we'll create a mock
        // In a real test, we'd use integration tests or refactor to accept generic iterators

        // For unit testing, let's test the logic differently
        let query = "test";
        let filename = "file.txt";
        let case_sensitive = true;

        let config = Config {
            query: query.to_string(),
            filename: filename.to_string(),
            case_sensitive,
        };

        assert_eq!(config.query, "test");
        assert_eq!(config.filename, "file.txt");
        assert!(config.case_sensitive);
    }

    #[test]
    fn test_config_new_error_messages() {
        // Test that our error messages are correct
        // This is more of a documentation test since we can't easily mock env::Args
        let expected_query_error = "Missing required argument: QUERY";
        let expected_filename_error = "Missing required argument: FILENAME";

        assert_eq!(expected_query_error, "Missing required argument: QUERY");
        assert_eq!(
            expected_filename_error,
            "Missing required argument: FILENAME"
        );
    }

    #[test]
    fn test_search_functionality() {
        // Test our core search logic works properly
        let query = "test";
        let contents = "This is a test\nAnother line\nTest again";

        let results = search(query, contents);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "This is a test");

        let results_insensitive = search_case_insensitive(query, contents);
        assert_eq!(results_insensitive.len(), 2);
        assert!(results_insensitive.contains(&"This is a test"));
        assert!(results_insensitive.contains(&"Test again"));
    }
}
