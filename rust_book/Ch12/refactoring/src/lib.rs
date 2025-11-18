//! # MiniGrep Library
//!
//! A simple text search library demonstrating separation of concerns.
//! This library provides the core functionality for searching text files.

use std::env;
use std::error::Error;
use std::fs;

/// Configuration struct to hold program arguments
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /// Creates a new Config from command line arguments
    ///
    /// # Arguments
    /// * `args` - A slice of String arguments from the command line
    ///
    /// # Returns
    /// * `Result<Config, &'static str>` - Success with Config or error message
    ///
    /// # Examples
    /// ```
    /// use chapter12_refactoring::Config;
    ///
    /// let args = vec![
    ///     String::from("minigrep"),
    ///     String::from("searchterm"),
    ///     String::from("poem.txt"),
    /// ];
    /// let config = Config::new(&args).unwrap();
    /// ```
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // Check for case sensitivity via environment variable
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    /// Alternative constructor using iterators (more idiomatic Rust)
    ///
    /// # Arguments  
    /// * `mut args` - An iterator over command line arguments
    ///
    /// # Returns
    /// * `Result<Config, &'static str>` - Success with Config or error message
    pub fn from_args(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Main application logic - reads file and performs search
///
/// # Arguments
/// * `config` - Configuration struct containing search parameters
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Success or error details
///
/// # Examples
/// ```no_run
/// use chapter12_refactoring::{Config, run};
///
/// let config = Config::new(&["minigrep", "rust", "poem.txt"]
///     .iter().map(|&s| s.into()).collect::<Vec<String>>()).unwrap();
/// run(config).unwrap();
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// Performs case-sensitive search for query in contents
///
/// # Arguments
/// * `query` - The string to search for
/// * `contents` - The text content to search in
///
/// # Returns
/// * `Vec<&str>` - Vector of lines containing the query
///
/// # Examples
/// ```
/// use chapter12_refactoring::search;
///
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Performs case-insensitive search for query in contents
///
/// # Arguments
/// * `query` - The string to search for (will be converted to lowercase)
/// * `contents` - The text content to search in
///
/// # Returns
/// * `Vec<&str>` - Vector of lines containing the query (case-insensitive)
///
/// # Examples
/// ```
/// use chapter12_refactoring::search_case_insensitive;
///
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
///
/// assert_eq!(
///     vec!["Rust:", "Trust me."],
///     search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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

    #[test]
    fn config_creation_success() {
        let args = vec![
            String::from("minigrep"),
            String::from("needle"),
            String::from("haystack.txt"),
        ];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "needle");
        assert_eq!(config.filename, "haystack.txt");
    }

    #[test]
    fn config_creation_failure() {
        let args = vec![String::from("minigrep")];
        assert!(Config::new(&args).is_err());
    }

    #[test]
    fn multiple_search_results() {
        let query = "the";
        let contents = "\
The quick brown fox
jumps over the lazy dog.
The end.";

        let results = search(query, contents);
        assert_eq!(results.len(), 1); // Only "jumps over the lazy dog." contains lowercase "the"
    }
}
