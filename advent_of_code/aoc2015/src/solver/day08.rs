// Day 08: Matchsticks
// https://adventofcode.com/2015/day/8

use anyhow::Result;

/// Day 08 Part 1: Hello World Placeholder
pub fn solve_part1(input: &str) -> Result<String> {
    println!("🎄 Day 8 Part 1:");
    println!("Input length: {} bytes", input.len());
    let mut total_code = 0;
    let mut total_memory = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Calculate code length (including surrounding quotes)
        let code_len = line.len();
        total_code += code_len;

        // Calculate memory length (count characters, not UTF-8 bytes)
        let memory_chars = count_memory_characters(line);
        println!(
            "Code: {}, Memory: {}, Diff: {} | {}",
            code_len,
            memory_chars,
            code_len - memory_chars,
            line
        );
        total_memory += memory_chars;
    }
    let difference = total_code - total_memory;
    println!("Total code length: {}", total_code);
    println!("Total memory length: {}", total_memory);
    println!("Difference: {}", difference);

    // TODO: Implement actual solution
    // Expected: Calculate difference between code representation and in-memory string length

    Ok(difference.to_string())
}

/// Day 08 Part 2: Encode strings and calculate length difference
pub fn solve_part2(input: &str) -> Result<String> {
    println!("🎄 Day 8 Part 2: String Encoding!");
    println!("Input length: {} bytes", input.len());

    let mut total_original = 0;
    let mut total_encoded = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Original code length
        let original_len = line.len();
        total_original += original_len;

        // Encoded length
        let encoded_len = encode_string_length(line);
        total_encoded += encoded_len;

        println!(
            "Code: {}, Encoded: {}, Diff: +{} | {}",
            original_len,
            encoded_len,
            encoded_len - original_len,
            line
        );
    }

    let difference = total_encoded - total_original;
    println!("\nTotal original length: {}", total_original);
    println!("Total encoded length: {}", total_encoded);
    println!("Difference: {}", difference);

    Ok(difference.to_string())
}

// ============================================================================
// PLACEHOLDER FOR FUTURE IMPLEMENTATION
// ============================================================================
//
// Day 8 typically involves:
// - String escape sequence parsing
// - Counting characters in different representations
// - Encoding/decoding string literals
//
// Part 1: Calculate code_length - memory_length for all strings
// Part 2: Calculate encoded_length - code_length for all strings
//

/// Calculate the length of an encoded string
/// Encoding rules:
/// - Wrap the entire string in quotes: +2
/// - Escape each backslash: \ becomes \\ (+1 per backslash)
/// - Escape each quote: " becomes \" (+1 per quote)
/// Example: "abc" becomes "\"abc\""
///   Original: "abc" = 5 chars
///   Encoded: "\"abc\"" = 9 chars (quote + backslash + quote + a + b + c + backslash + quote + quote)
fn encode_string_length(input: &str) -> usize {
    let mut count = 2; // Start with 2 for the surrounding quotes

    for c in input.chars() {
        match c {
            '\\' => count += 2, // \ becomes \\
            '"' => count += 2,  // " becomes \"
            _ => count += 1,    // Regular character stays as-is
        }
    }

    count
}

/// Count the number of characters in memory after unescaping
/// This counts logical characters, not UTF-8 bytes
/// For example: "\xc4" is 1 character in memory, even though it's 2 UTF-8 bytes
fn count_memory_characters(input: &str) -> usize {
    let mut count = 0;
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '"' {
            // Skip opening and closing quotes
            continue;
        }

        if c == '\\' {
            match chars.next() {
                Some('"') | Some('\\') => {
                    // \" or \\ both represent 1 character in memory
                    count += 1;
                }
                Some('x') => {
                    // \xHH represents 1 character in memory
                    chars.next(); // consume first hex digit
                    chars.next(); // consume second hex digit
                    count += 1;
                }
                _ => {}
            }
        } else {
            // Regular character
            count += 1;
        }
    }

    count
}

// ============================================================================
