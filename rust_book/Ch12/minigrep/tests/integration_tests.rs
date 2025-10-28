//! # Integration Tests for minigrep
//!
//! These tests verify the complete behavior of minigrep including
//! search operations, configuration handling, and end-to-end functionality.
//!
//! Integration tests are located in the `tests/` directory and test
//! the library as an external user would, focusing on the public API.

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn integration_test_config_creation() {
    let args = vec![
        "minigrep".to_string(),
        "search_term".to_string(),
        "test_file.txt".to_string(),
    ];

    let config = minigrep::Config::new(args.into_iter()).expect("Config creation failed");

    assert_eq!(config.query, "search_term");
    assert_eq!(config.filename, "test_file.txt");
}

// ============================================================================
// Search Integration Tests
// ============================================================================

#[test]
fn test_search_function_integration() {
    let contents = "\
Rust is a systems programming language.
It emphasizes safety and performance.
Rust is loved by many developers.
Python is also popular.
Rust community is welcoming.";

    let results = minigrep::search("Rust", contents);
    
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, 1); // Line 1
    assert_eq!(results[1].0, 3); // Line 3
    assert_eq!(results[2].0, 5); // Line 5
}

#[test]
fn test_case_insensitive_search_integration() {
    let contents = "\
THE quick BROWN fox
Jumps over THE lazy dog
the end";

    let results = minigrep::search_case_insensitive("the", contents);
    
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, 1);
    assert_eq!(results[1].0, 2);
    assert_eq!(results[2].0, 3);
}

#[test]
fn test_search_empty_results() {
    let contents = "apple\nbanana\norange";
    let results = minigrep::search("grape", contents);
    
    assert_eq!(results.len(), 0);
}

#[test]
fn test_search_with_numbers() {
    let contents = "\
Version 1.0.0
Version 2.0.0
Release 3.0.0
Version 4.0.0";

    let results = minigrep::search("Version", contents);
    
    assert_eq!(results.len(), 3);
}

#[test]
fn test_search_with_punctuation() {
    let contents = "\
Hello, World!
Hello everyone.
Say hello?
Greetings!";

    let results = minigrep::search("Hello", contents);
    
    assert_eq!(results.len(), 2);
}

#[test]
fn test_search_entire_line_match() {
    let contents = "\
apple
apple pie
apple sauce
banana";

    let results = minigrep::search("apple", contents);
    
    assert_eq!(results.len(), 3);
}

#[test]
fn test_search_case_sensitive_excludes_different_case() {
    let contents = "\
RUST
Rust
rust
RuSt";

    let results = minigrep::search("Rust", contents);
    
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_multiline_content() {
    let contents = "\
Line 1 content
Line 2 target
Line 3 content
Line 4 target
Line 5 content
Line 6 target";

    let results = minigrep::search("target", contents);
    
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, 2);
    assert_eq!(results[1].0, 4);
    assert_eq!(results[2].0, 6);
}

#[test]
fn test_search_with_whitespace() {
    let contents = "\
   leading spaces
trailing spaces   
	tabs	here
normal line";

    let results = minigrep::search("spaces", contents);
    
    assert_eq!(results.len(), 2);
}

#[test]
fn test_search_empty_query_matches_all() {
    let contents = "line1\nline2\nline3";
    let results = minigrep::search("", contents);
    
    // Empty query matches all lines
    assert_eq!(results.len(), 3);
}

#[test]
fn test_search_special_characters_in_query() {
    let contents = "\
What is Rust?
How much? A lot!
Is it (good)?
Yes!";

    let results = minigrep::search("?", contents);
    
    assert_eq!(results.len(), 3); // Lines 1, 2, and 3 contain "?"
}

#[test]
fn test_case_insensitive_with_symbols() {
    let contents = "\
C++ is fast
C++ is powerful
Rust is also fast
C++ and Rust";

    let results = minigrep::search_case_insensitive("C++", contents);
    
    assert_eq!(results.len(), 3);
}

// ============================================================================
// Error Condition Tests
// ============================================================================

#[test]
fn test_config_new_with_insufficient_args() {
    let args = vec!["minigrep".to_string()];
    let result = minigrep::Config::new(args.into_iter());
    
    assert!(result.is_err());
}

#[test]
fn test_config_new_with_only_query() {
    let args = vec!["minigrep".to_string(), "query".to_string()];
    let result = minigrep::Config::new(args.into_iter());
    
    assert!(result.is_err());
}

#[test]
fn test_config_new_with_extra_args_ignored() {
    let args = vec![
        "minigrep".to_string(),
        "query".to_string(),
        "file.txt".to_string(),
        "extra".to_string(),
        "args".to_string(),
    ];
    
    let config = minigrep::Config::new(args.into_iter()).unwrap();
    
    assert_eq!(config.query, "query");
    assert_eq!(config.filename, "file.txt");
}

// ============================================================================
// Behavior Tests
// ============================================================================

#[test]
fn test_search_returns_correct_line_content() {
    let contents = "\
First line
Second line with search term
Third line";

    let results = minigrep::search("search", contents);
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, 2);
    assert!(results[0].1.contains("Second line with search term"));
}

#[test]
fn test_search_preserves_original_line() {
    let original = "This is a TEST line";
    let contents = format!("{}\nAnother line", original);
    
    let results = minigrep::search("TEST", &contents);
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].1, original);
}

#[test]
fn test_case_insensitive_with_mixed_case_query() {
    let contents = "\
HELLO world
hello WORLD
Hello World
hElLo WoRlD";

    let results = minigrep::search_case_insensitive("HeLLo", contents);
    
    assert_eq!(results.len(), 4);
}

// ============================================================================
// Performance and Edge Case Tests
// ============================================================================

#[test]
fn test_search_in_large_content() {
    let mut contents = String::new();
    for i in 0..1000 {
        if i == 500 {
            contents.push_str("TARGET\n");
        } else {
            contents.push_str(&format!("Line {}\n", i));
        }
    }

    let results = minigrep::search("TARGET", &contents);
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, 501); // Line 501 (0-indexed becomes 1-indexed)
}

#[test]
fn test_search_many_matches() {
    let contents = "\
apple
apple pie
apple sauce
baked apple
apple juice
apple cake
apple tart";

    let results = minigrep::search("apple", contents);
    
    assert_eq!(results.len(), 7);
}

#[test]
fn test_search_unicode_content() {
    let contents = "\
Hello 世界
Rust 编程
Hello Rust
世界 Hello";

    let results = minigrep::search("Hello", contents);
    
    assert_eq!(results.len(), 3); // Lines 1, 3, and 4 contain "Hello"
}

#[test]
fn test_search_with_unicode_query() {
    let contents = "\
世界 is world
Hello 世界
程序 means program
编程 means programming";

    let results = minigrep::search("世界", contents);
    
    assert_eq!(results.len(), 2);
}

// ============================================================================
// Real-World Usage Tests
// ============================================================================

#[test]
fn test_log_file_search() {
    let contents = "\
[INFO] Application started
[DEBUG] Loading configuration
[ERROR] Connection failed
[INFO] Retrying connection
[ERROR] Maximum retries exceeded
[INFO] Shutting down";

    let error_results = minigrep::search("[ERROR]", contents);
    
    assert_eq!(error_results.len(), 2);
}

#[test]
fn test_code_search() {
    let contents = "\
fn main() {
    println!(\"Hello\");
}

fn helper() {
    println!(\"Helper\");
}

fn main_function() {
    println!(\"Main function\");
}";

    let results = minigrep::search("fn ", contents);
    
    assert_eq!(results.len(), 3);
}

#[test]
fn test_configuration_file_search() {
    let contents = "\
database_url=localhost:5432
database_user=admin
database_password=secret
api_key=abc123
api_timeout=30";

    let results = minigrep::search("database", contents);
    
    assert_eq!(results.len(), 3);
}

#[test]
fn test_regex_like_pattern() {
    let contents = "\
Email: user@example.com
Contact: admin@site.org
Support: help@service.net
Username: john_doe";

    let results = minigrep::search("@", contents);
    
    assert_eq!(results.len(), 3);
}
