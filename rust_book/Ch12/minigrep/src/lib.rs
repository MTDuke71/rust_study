//! # minigrep Library
//!
//! A command-line text search tool demonstrating Rust's best practices.
//! This library contains the core functionality for searching text files.
//!
//! ## Features
//! - Case-sensitive and case-insensitive search
//! - Returns line numbers with matching lines
//! - Proper error handling with Result types
//! - Environment variable support

use std::error::Error;
use std::fs;

/// Configuration for the minigrep application
///
/// Holds the search query, file path, and search options.
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /// Creates a new Config from command-line arguments
    ///
    /// # Arguments
    /// * `args` - Iterator over command-line arguments
    ///
    /// # Returns
    /// * `Ok(Config)` - Successfully parsed configuration
    /// * `Err(&str)` - Error message if parsing failed
    ///
    /// # Example
    /// ```ignore
    /// let args = std::env::args();
    /// let config = Config::new(args)?;
    /// ```
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

/// Search result containing line number and content
pub type SearchResult = (usize, String);

/// Runs the search application with the given configuration
///
/// # Arguments
/// * `config` - Application configuration
///
/// # Returns
/// * `Ok(())` - Successfully completed
/// * `Err(Box<dyn Error>)` - Error occurred
///
/// # Example
/// ```ignore
/// let config = Config::new(env::args())?;
/// run(config)?;
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    if results.is_empty() {
        println!("No matches found for '{}'", config.query);
    } else {
        println!("Found {} match(es) for '{}':", results.len(), config.query);
        println!();
        for (line_num, line) in results {
            println!("{:4} | {}", line_num, line);
        }
    }

    Ok(())
}

/// Performs a case-sensitive search
///
/// Searches through lines of text for exact matches with the query string.
///
/// # Arguments
/// * `query` - String to search for
/// * `contents` - Text to search in
///
/// # Returns
/// Vector of (line_number, line_content) tuples for matching lines
///
/// # Example
/// ```
/// use minigrep::search;
///
/// let results = search("foo", "foo\nbar\nfoo bar");
/// assert_eq!(results.len(), 2);
/// ```
pub fn search(query: &str, contents: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();

    for (line_num, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((line_num + 1, line.to_string()));
        }
    }

    results
}

/// Performs a case-insensitive search
///
/// Searches through lines of text for matches with the query string,
/// ignoring case differences.
///
/// # Arguments
/// * `query` - String to search for (case ignored)
/// * `contents` - Text to search in
///
/// # Returns
/// Vector of (line_number, line_content) tuples for matching lines
///
/// # Example
/// ```
/// use minigrep::search_case_insensitive;
///
/// let results = search_case_insensitive("RUST", "rust\nRUST\nRust");
/// assert_eq!(results.len(), 3);
/// ```
pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<SearchResult> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (line_num, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push((line_num + 1, line.to_string()));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // Case-Sensitive Search Tests
    // ========================================

    #[test]
    fn test_search_single_match() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let results = search(query, contents);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 2);
        assert!(results[0].1.contains("productive"));
    }

    #[test]
    fn test_search_multiple_matches() {
        let query = "the";
        let contents = "\
The quick brown fox
jumps over the lazy dog
the end of the story";

        let results = search(query, contents);
        assert_eq!(results.len(), 0); // Case-sensitive, no lowercase "the"
    }

    #[test]
    fn test_search_no_matches() {
        let query = "xyz";
        let contents = "\
Rust is great
Rust is fast
Rust is safe";

        let results = search(query, contents);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_returns_line_numbers() {
        let query = "is";
        let contents = "\
Rust is great
Python is easy
Rust is fast";

        let results = search(query, contents);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].0, 1);
        assert_eq!(results[1].0, 2);
        assert_eq!(results[2].0, 3);
    }

    // ========================================
    // Case-Insensitive Search Tests
    // ========================================

    #[test]
    fn test_case_insensitive_single_match() {
        let query = "DUCT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let results = search_case_insensitive(query, contents);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_case_insensitive_mixed_case() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results = search_case_insensitive(query, contents);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 1);
        assert_eq!(results[1].0, 4);
    }

    #[test]
    fn test_case_insensitive_all_lowercase() {
        let query = "rust";
        let contents = "\
RUST is great
Rust is fast
rust is awesome
RuSt rocks";

        let results = search_case_insensitive(query, contents);
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn test_case_insensitive_all_uppercase() {
        let query = "RUST";
        let contents = "\
Rust is great
RUST is fast
rust is awesome";

        let results = search_case_insensitive(query, contents);
        assert_eq!(results.len(), 3);
    }

    // ========================================
    // Edge Cases
    // ========================================

    #[test]
    fn test_search_empty_query() {
        let query = "";
        let contents = "line 1\nline 2\nline 3";

        let results = search(query, contents);
        assert_eq!(results.len(), 3); // Empty string matches all lines
    }

    #[test]
    fn test_search_empty_contents() {
        let query = "test";
        let contents = "";

        let results = search(query, contents);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_multiword_query() {
        let query = "safe fast";
        let contents = "\
Rust is safe, fast, and productive
Go is fast but not as safe
Rust is safe fast and fun";

        let results = search(query, contents);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 3);
    }

    #[test]
    fn test_search_special_characters() {
        let query = "?";
        let contents = "\
What is Rust?
How fast?
Why not!";

        let results = search(query, contents);
        assert_eq!(results.len(), 2);
    }

    // ========================================
    // Config Tests
    // ========================================

    #[test]
    fn test_config_new_valid() {
        let args = vec![
            "minigrep".to_string(),
            "query".to_string(),
            "filename.txt".to_string(),
        ];
        
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename.txt");
    }

    #[test]
    fn test_config_new_missing_query() {
        let args = vec!["minigrep".to_string()];
        
        let result = Config::new(args.into_iter());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Didn't get a query string");
    }

    #[test]
    fn test_config_new_missing_filename() {
        let args = vec!["minigrep".to_string(), "query".to_string()];
        
        let result = Config::new(args.into_iter());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Didn't get a file name");
    }
}
