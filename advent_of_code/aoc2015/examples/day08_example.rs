//! # Day 8 Example: Matchsticks - Character Counting Solution
//!
//! **AoC 2015 Day 8: String Processing with Escape Sequences**
//!
//! ## 📋 **Problem Summary**
//!
//! Count the difference between:
//! - **Code representation**: Raw string literals as they appear in code
//! - **Memory representation**: Characters after processing escape sequences
//!
//! ## 🎯 **Processing Rules**
//!
//! 1. **Quote Removal**: Remove surrounding `"` characters (don't count them)
//! 2. **Standard Escapes**: Replace escape sequences with single characters:
//!    - `\\` → `\` (escaped backslash)
//!    - `\"` → `"` (escaped quote)  
//!    - `\t` → tab character
//!    - `\n` → newline character
//! 3. **Hex Escapes**: Replace `\xHH` with single character where HH = hex digits
//!    - `\x41` → `A` (ASCII 65)
//!    - `\x48` → `H` (ASCII 72)
//!
//! ## 🚀 **Optimization Focus**
//!
//! This solution optimizes for **character counting only** - no string building required.
//! Memory efficient approach suitable for large inputs and AoC competitive programming.

use std::collections::HashMap;

/// Represents the character count comparison between code and memory representations
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterCounts {
    /// Number of characters in the original code representation
    pub code_chars: usize,
    /// Number of characters after escape sequence processing
    pub memory_chars: usize,
}

impl CharacterCounts {
    /// Calculate the difference between code and memory representations
    pub fn difference(&self) -> usize {
        self.code_chars.saturating_sub(self.memory_chars)
    }

    /// Calculate compression ratio as a percentage
    pub fn compression_ratio(&self) -> f64 {
        if self.code_chars == 0 {
            0.0
        } else {
            100.0 * self.difference() as f64 / self.code_chars as f64
        }
    }
}

/// Detailed statistics about escape sequence processing
#[derive(Debug, Default)]
pub struct ProcessingStats {
    pub original_chars: usize,
    pub processed_chars: usize,
    pub quotes_removed: usize,
    pub standard_escapes: usize,
    pub hex_escapes: usize,
    pub invalid_sequences: usize,
}

impl ProcessingStats {
    /// Generate a human-readable summary of processing statistics
    pub fn summarize(&self) -> String {
        format!(
            "Code: {} → Memory: {} ({} chars saved, {:.1}% compression)\n\
             Details: {} quotes, {} standard escapes, {} hex escapes, {} invalid",
            self.original_chars,
            self.processed_chars,
            self.original_chars.saturating_sub(self.processed_chars),
            if self.original_chars > 0 {
                100.0 * (self.original_chars - self.processed_chars) as f64
                    / self.original_chars as f64
            } else {
                0.0
            },
            self.quotes_removed,
            self.standard_escapes,
            self.hex_escapes,
            self.invalid_sequences
        )
    }
}

/// Count characters in processed string representation without building the actual string
///
/// This function processes escape sequences and counts the resulting characters
/// without allocating memory for the processed string, making it memory-efficient
/// for large inputs or when only the count is needed.
///
/// # Arguments
///
/// * `input` - The raw string literal to process (including surrounding quotes)
///
/// # Returns
///
/// `CharacterCounts` struct containing both code and memory character counts
///
/// # Examples
///
/// ```rust
/// let counts = count_processed_characters(r#""Hello\tWorld""#);
/// assert_eq!(counts.code_chars, 13);    // "Hello\tWorld" = 13 chars
/// assert_eq!(counts.memory_chars, 11);  // HelloWorld with tab = 11 chars
/// ```
pub fn count_processed_characters(input: &str) -> CharacterCounts {
    let chars: Vec<char> = input.chars().collect();
    let original_length = chars.len();
    let mut processed_count = 0;
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            // REQ-QUOTE1: Skip surrounding quotes entirely (don't count in memory representation)
            '"' => {
                i += 1; // Move past quote, don't increment processed_count
            }

            // REQ-ESC1: Process escape sequences
            '\\' if i + 1 < chars.len() => {
                match chars[i + 1] {
                    // REQ-STD1: Standard escape sequences (2 code chars → 1 memory char)
                    '\\' => {
                        processed_count += 1;
                        i += 2;
                    } // \\ → \
                    '"' => {
                        processed_count += 1;
                        i += 2;
                    } // \" → "
                    't' => {
                        processed_count += 1;
                        i += 2;
                    } // \t → tab
                    'n' => {
                        processed_count += 1;
                        i += 2;
                    } // \n → newline
                    'r' => {
                        processed_count += 1;
                        i += 2;
                    } // \r → carriage return
                    '0' => {
                        processed_count += 1;
                        i += 2;
                    } // \0 → null
                    '\'' => {
                        processed_count += 1;
                        i += 2;
                    } // \' → '

                    // REQ-HEX1: Hexadecimal escape sequences (4 code chars → 1 memory char)
                    'x' if i + 3 < chars.len() => {
                        let hex_chars = &chars[i + 2..i + 4];
                        if hex_chars.iter().all(|c| c.is_ascii_hexdigit()) {
                            // Valid hex sequence: \xHH → single character
                            processed_count += 1;
                            i += 4; // Skip \x and two hex digits
                        } else {
                            // Invalid hex sequence: treat \ as literal character
                            processed_count += 1;
                            i += 1;
                        }
                    }

                    // REQ-UNKNOWN1: Unknown escape sequences - treat backslash as literal
                    _ => {
                        processed_count += 1;
                        i += 1;
                    }
                }
            }

            // REQ-LITERAL1: Regular characters count as-is
            _ => {
                processed_count += 1;
                i += 1;
            }
        }
    }

    CharacterCounts {
        code_chars: original_length,
        memory_chars: processed_count,
    }
}

/// Analyze character processing with detailed statistics
///
/// Provides comprehensive breakdown of how different types of escape sequences
/// contribute to character count reduction. Useful for debugging and optimization.
///
/// # Arguments
///
/// * `input` - The raw string literal to analyze
///
/// # Returns
///
/// `ProcessingStats` with detailed breakdown of processing operations
pub fn analyze_character_processing(input: &str) -> ProcessingStats {
    let chars: Vec<char> = input.chars().collect();
    let mut stats = ProcessingStats {
        original_chars: chars.len(),
        ..Default::default()
    };

    let mut processed_count = 0;
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '"' => {
                stats.quotes_removed += 1;
                i += 1;
            }
            '\\' if i + 1 < chars.len() => match chars[i + 1] {
                '\\' | '"' | 't' | 'n' | 'r' | '0' | '\'' => {
                    stats.standard_escapes += 1;
                    processed_count += 1;
                    i += 2;
                }
                'x' if i + 3 < chars.len() => {
                    let hex_chars = &chars[i + 2..i + 4];
                    if hex_chars.iter().all(|c| c.is_ascii_hexdigit()) {
                        stats.hex_escapes += 1;
                        processed_count += 1;
                        i += 4;
                    } else {
                        stats.invalid_sequences += 1;
                        processed_count += 1;
                        i += 1;
                    }
                }
                _ => {
                    stats.invalid_sequences += 1;
                    processed_count += 1;
                    i += 1;
                }
            },
            _ => {
                processed_count += 1;
                i += 1;
            }
        }
    }

    stats.processed_chars = processed_count;
    stats
}

/// Solve AoC Day 8 Part 1: Calculate total difference between code and memory
///
/// Processes multiple string literals and sums the differences between their
/// code representation and memory representation after escape processing.
///
/// # Arguments
///
/// * `input` - Multi-line string where each line is a string literal
///
/// # Returns
///
/// Tuple of (total_code_chars, total_memory_chars, total_difference)
pub fn solve_day8_part1(input: &str) -> (usize, usize, usize) {
    let mut total_code = 0;
    let mut total_memory = 0;

    for line in input.lines() {
        let line = line.trim();
        if !line.is_empty() {
            let counts = count_processed_characters(line);
            total_code += counts.code_chars;
            total_memory += counts.memory_chars;
        }
    }

    (
        total_code,
        total_memory,
        total_code.saturating_sub(total_memory),
    )
}

/// Generate comprehensive test data for escape sequence processing
///
/// Creates a variety of test cases covering different escape sequence types
/// and edge cases for thorough validation of the counting algorithms.
fn generate_test_data() -> Vec<(&'static str, &'static str)> {
    vec![
        // (input, description)
        (r#""""#, "Empty string with quotes"),
        (r#""abc""#, "Simple string with quotes"),
        (r#""aaa\"aaa""#, "String with escaped quote"),
        (r#""\x27""#, "Single hex escape (apostrophe)"),
        (r#""\"""#, "Just an escaped quote"),
        (r#""\\""#, "Just an escaped backslash"),
        (r#""Hello\tWorld""#, "String with tab escape"),
        (r#""Line1\nLine2""#, "String with newline escape"),
        (r#""Path\\File\\Name""#, "String with multiple backslashes"),
        (r#""\x41\x42\x43""#, "Multiple hex escapes (ABC)"),
        (
            r#""Mixed: \t\x48\x65\x6C\x6C\x6F \\ end""#,
            "Complex mixed escapes",
        ),
        (
            r#""\x48\x65\x6C\x6C\x6F\x20\x57\x6F\x72\x6C\x64""#,
            "Hello World in hex",
        ),
        (
            r#""Invalid: \xGG and \z sequences""#,
            "Invalid escape sequences",
        ),
        (r#""Incomplete: \x4""#, "Incomplete hex sequence"),
    ]
}

fn main() {
    println!("=== AoC 2015 Day 8: Character Counting Solution ===\n");

    // REQ-DEMO1: Demonstrate basic character counting
    println!("📊 Basic Character Counting Examples:");
    println!("{:-<60}", "");

    let basic_examples = vec![r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];

    for (i, input) in basic_examples.iter().enumerate() {
        let counts = count_processed_characters(input);
        println!("Example {}: {}", i + 1, input);
        println!("  Code chars:   {} characters", counts.code_chars);
        println!("  Memory chars: {} characters", counts.memory_chars);
        println!("  Difference:   {} characters", counts.difference());
        println!();
    }

    // REQ-DEMO2: Comprehensive test suite with detailed analysis
    println!("🔍 Comprehensive Analysis:");
    println!("{:-<60}", "");

    let test_data = generate_test_data();
    let mut total_code = 0;
    let mut total_memory = 0;

    for (input, description) in &test_data {
        let stats = analyze_character_processing(input);
        total_code += stats.original_chars;
        total_memory += stats.processed_chars;

        println!("Test: {}", description);
        println!("  Input: {:?}", input);
        println!("  {}", stats.summarize());
        println!();
    }

    // REQ-SOLUTION1: Final AoC solution calculation
    println!("🎯 AoC Day 8 Part 1 Solution:");
    println!("{:-<60}", "");
    println!("Total code characters:   {}", total_code);
    println!("Total memory characters: {}", total_memory);
    println!("Final answer:            {}", total_code - total_memory);
    println!();

    // REQ-PERF1: Performance demonstration with batch processing
    println!("⚡ Batch Processing Example:");
    println!("{:-<60}", "");

    let batch_input = r#""abc"
"aaa\"aaa"  
"\x27"
"\""
"\\"
"Hello\tWorld"
"\x41\x42\x43""#;

    let (code, memory, difference) = solve_day8_part1(batch_input);
    println!("Batch input processed:");
    println!("  Lines processed: {}", batch_input.lines().count());
    println!("  Total code:      {} chars", code);
    println!("  Total memory:    {} chars", memory);
    println!("  Difference:      {} chars", difference);
    println!(
        "  Compression:     {:.1}%",
        100.0 * difference as f64 / code as f64
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-BASIC1: Basic counting functionality
    fn test_basic_character_counting() {
        let test_cases = vec![
            (r#""""#, 2, 0),          // Empty string
            (r#""abc""#, 5, 3),       // Simple string
            (r#""aaa\"aaa""#, 10, 7), // Escaped quote
            (r#""\x27""#, 6, 1),      // Hex escape
        ];

        for (input, expected_code, expected_memory) in test_cases {
            let counts = count_processed_characters(input);
            assert_eq!(
                counts.code_chars, expected_code,
                "Code count mismatch for: {}",
                input
            );
            assert_eq!(
                counts.memory_chars, expected_memory,
                "Memory count mismatch for: {}",
                input
            );
        }
    }

    #[test] // REQ-ESC1: Escape sequence processing
    fn test_escape_sequences() {
        let counts = count_processed_characters(r#""Hello\tWorld\n""#);
        assert_eq!(counts.code_chars, 16); // Full string length
        assert_eq!(counts.memory_chars, 12); // Hello + tab + World + newline
    }

    #[test] // REQ-HEX1: Hexadecimal escape sequences
    fn test_hex_escapes() {
        let counts = count_processed_characters(r#""\x41\x42\x43""#);
        assert_eq!(counts.code_chars, 14); // Full string with \x sequences
        assert_eq!(counts.memory_chars, 3); // Just ABC
    }

    #[test] // REQ-BATCH1: Batch processing functionality
    fn test_batch_processing() {
        let input = r#""abc"
"aaa\"aaa"
"\x27""#;
        let (code, memory, diff) = solve_day8_part1(input);
        assert_eq!(code, 21); // 5 + 10 + 6
        assert_eq!(memory, 11); // 3 + 7 + 1
        assert_eq!(diff, 10); // 21 - 11
    }

    #[test] // REQ-STATS1: Statistics generation
    fn test_processing_stats() {
        let stats = analyze_character_processing(r#""Test\t\x41""#);
        assert_eq!(stats.original_chars, 11);
        assert_eq!(stats.processed_chars, 6); // Test + tab + A
        assert_eq!(stats.quotes_removed, 2);
        assert_eq!(stats.standard_escapes, 1); // \t
        assert_eq!(stats.hex_escapes, 1); // \x41
    }

    #[test] // REQ-EDGE1: Edge cases and error handling
    fn test_edge_cases() {
        // Empty input
        let counts = count_processed_characters("");
        assert_eq!(counts.code_chars, 0);
        assert_eq!(counts.memory_chars, 0);

        // Invalid hex sequence
        let counts = count_processed_characters(r#""\xGG""#);
        assert_eq!(counts.code_chars, 6);
        assert_eq!(counts.memory_chars, 4); // \xGG treated as 4 chars

        // Incomplete hex sequence
        let counts = count_processed_characters(r#""\x4""#);
        assert_eq!(counts.code_chars, 5);
        assert_eq!(counts.memory_chars, 3); // \x4 treated as 3 chars
    }
}
