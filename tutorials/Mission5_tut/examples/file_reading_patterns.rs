// File Reading and Line Processing Patterns in Rust
// =================================================

use std::fs;
use std::io::{self, BufRead, BufReader};
use anyhow::Result;

fn main() -> Result<()> {
    println!("📁 File Reading Patterns in Rust");
    println!("=================================");
    
    // Create a sample file for demonstration
    create_sample_file()?;
    
    // Pattern 1: Read entire file as string, then split lines
    pattern1_read_all_then_split()?;
    
    // Pattern 2: Read file using BufReader (memory efficient for large files)
    pattern2_buffered_reader()?;
    
    // Pattern 3: AoC-style processing (most common pattern)
    pattern3_aoc_style()?;
    
    // Pattern 4: Collecting lines into Vec<String>
    pattern4_collect_lines()?;
    
    // Pattern 5: Processing with error handling
    pattern5_with_error_handling()?;
    
    Ok(())
}

fn create_sample_file() -> Result<()> {
    let content = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp  
haegwjzuvuyypxyu
dvszwmarrgswjxmb
line with spaces
another line
final line";
    
    fs::write("sample.txt", content)?;
    println!("✅ Created sample.txt for demonstration\n");
    Ok(())
}

/// Pattern 1: Read entire file as string, then split lines
/// ✅ Simple and common for small files
/// ❌ Loads entire file into memory
fn pattern1_read_all_then_split() -> Result<()> {
    println!("🎯 Pattern 1: Read all, then split lines");
    println!("----------------------------------------");
    
    // Read entire file as a single string
    let input = fs::read_to_string("sample.txt")?;
    
    // Split into lines and process
    let lines: Vec<&str> = input.lines().collect();
    
    println!("Total lines: {}", lines.len());
    for (i, line) in lines.iter().enumerate() {
        println!("Line {}: '{}'", i + 1, line);
    }
    
    // Alternative: process directly without collecting
    println!("\nDirect processing:");
    let nice_count = input.lines()
        .filter(|line| is_nice_demo(line))
        .count();
    
    println!("Nice strings: {}\n", nice_count);
    Ok(())
}

/// Pattern 2: BufReader for memory-efficient line-by-line reading
/// ✅ Memory efficient for large files
/// ✅ Can handle very large files
/// ❌ Slightly more complex
fn pattern2_buffered_reader() -> Result<()> {
    println!("🎯 Pattern 2: Buffered Reader (memory efficient)");
    println!("-----------------------------------------------");
    
    let file = std::fs::File::open("sample.txt")?;
    let reader = BufReader::new(file);
    
    let mut nice_count = 0;
    let mut total_lines = 0;
    
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential I/O errors
        total_lines += 1;
        
        println!("Processing: '{}'", line);
        if is_nice_demo(&line) {
            nice_count += 1;
        }
    }
    
    println!("Processed {} lines, {} are nice\n", total_lines, nice_count);
    Ok(())
}

/// Pattern 3: AoC-style processing (most common for competitive programming)
/// ✅ Simple and efficient for typical AoC inputs
/// ✅ Easy to understand and modify
fn pattern3_aoc_style() -> Result<()> {
    println!("🎯 Pattern 3: AoC-style (recommended for competitions)");
    println!("----------------------------------------------------");
    
    let input = fs::read_to_string("sample.txt")?;
    
    // This is the pattern you see in most AoC solutions
    let result = input
        .lines()                           // Iterator over lines
        .filter(|line| !line.is_empty())   // Skip empty lines
        .filter(|line| is_nice_demo(line)) // Apply your logic
        .count();                          // Count matches
    
    println!("AoC-style result: {} nice strings", result);
    
    // More complex example: collecting and processing
    let processed_lines: Vec<String> = input
        .lines()
        .map(|line| line.trim())           // Remove whitespace
        .filter(|line| !line.is_empty())   // Skip empty lines
        .map(|line| format!("Processed: {}", line)) // Transform
        .collect();
    
    println!("Processed {} lines:", processed_lines.len());
    for line in &processed_lines[0..3] { // Show first 3
        println!("  {}", line);
    }
    println!();
    
    Ok(())
}

/// Pattern 4: Collecting lines for multiple passes
/// ✅ Good when you need to process the same data multiple times
/// ❌ Uses more memory
fn pattern4_collect_lines() -> Result<()> {
    println!("🎯 Pattern 4: Collect lines into Vec<String>");
    println!("--------------------------------------------");
    
    let input = fs::read_to_string("sample.txt")?;
    
    // Collect all lines into owned strings
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string()) // Convert to owned String
        .filter(|line| !line.is_empty())     // Skip empty lines
        .collect();
    
    println!("Collected {} lines", lines.len());
    
    // Now you can do multiple passes
    let total_chars: usize = lines.iter().map(|line| line.len()).sum();
    let nice_count = lines.iter().filter(|line| is_nice_demo(line)).count();
    let long_lines = lines.iter().filter(|line| line.len() > 10).count();
    
    println!("Total characters: {}", total_chars);
    println!("Nice strings: {}", nice_count);
    println!("Long lines (>10 chars): {}", long_lines);
    println!();
    
    Ok(())
}

/// Pattern 5: With comprehensive error handling
/// ✅ Production-ready error handling
/// ✅ Handles various failure modes
fn pattern5_with_error_handling() -> Result<()> {
    println!("🎯 Pattern 5: With error handling");
    println!("---------------------------------");
    
    // Check if file exists first
    if !std::path::Path::new("sample.txt").exists() {
        println!("❌ File doesn't exist!");
        return Ok(());
    }
    
    // Read with proper error handling
    match fs::read_to_string("sample.txt") {
        Ok(content) => {
            println!("✅ File read successfully");
            
            let results: Result<Vec<_>, _> = content
                .lines()
                .enumerate()
                .map(|(i, line)| -> Result<String> {
                    if line.is_empty() {
                        return Err(anyhow::anyhow!("Empty line at line {}", i + 1));
                    }
                    
                    Ok(format!("Line {}: {} chars", i + 1, line.len()))
                })
                .collect();
                
            match results {
                Ok(processed) => {
                    println!("Successfully processed {} lines", processed.len());
                    for result in &processed[0..3] { // Show first 3
                        println!("  {}", result);
                    }
                }
                Err(e) => println!("❌ Processing error: {}", e),
            }
        }
        Err(e) => println!("❌ Failed to read file: {}", e),
    }
    
    println!();
    Ok(())
}

// Demo function for AoC Day 5 logic
fn is_nice_demo(s: &str) -> bool {
    let vowel_count = s.chars().filter(|&c| "aeiou".contains(c)).count();
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    let forbidden = ["ab", "cd", "pq", "xy"];
    let has_forbidden = forbidden.iter().any(|&bad| s.contains(bad));
    
    vowel_count >= 3 && has_double && !has_forbidden
}