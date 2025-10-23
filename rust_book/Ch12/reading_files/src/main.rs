//! # Chapter 12.2: Reading a File
//!
//! Demonstrates file I/O operations from The Rust Book.
//! Building on argument parsing to read file contents.
//!
//! Run with: `cargo run searchstring poem.txt`
//! Make sure poem.txt exists or create it with sample content.

use std::env;
use std::fs;

fn example1_basic_file_reading() {
    println!("🦀 Example 1: Basic File Reading");
    println!("=================================");

    // Create a sample file for demonstration
    let sample_content = "I'm nobody! Who are you?\nAre you nobody, too?\nThen there's a pair of us - don't tell!\nThey'd banish us, you know.";
    
    if let Err(e) = fs::write("poem.txt", sample_content) {
        println!("❌ Failed to create sample file: {}", e);
        return;
    }
    
    // Now read it back
    match fs::read_to_string("poem.txt") {
        Ok(contents) => {
            println!("✅ Successfully read file:");
            println!("   File size: {} bytes", contents.len());
            println!("   Line count: {}", contents.lines().count());
            println!("\n📄 Contents:");
            for (i, line) in contents.lines().enumerate() {
                println!("   {}: {}", i + 1, line);
            }
        }
        Err(e) => println!("❌ Error reading file: {}", e),
    }
    
    println!();
}

fn example2_file_with_arguments() {
    println!("🔧 Example 2: Reading File from Arguments");
    println!("==========================================");

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("❌ Usage: {} <search_query> <filename>", args[0]);
        return;
    }
    
    let query = &args[1];
    let filename = &args[2];
    
    println!("🔍 Searching for '{}' in file '{}'", query, filename);
    
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("✅ File read successfully:");
            println!("   Size: {} bytes", contents.len());
            println!("   Lines: {}", contents.lines().count());
            
            // Simple search demonstration
            let matching_lines: Vec<&str> = contents
                .lines()
                .filter(|line| line.contains(query))
                .collect();
            
            if matching_lines.is_empty() {
                println!("   No matches found for '{}'", query);
            } else {
                println!("   Found {} matching lines:", matching_lines.len());
                for (i, line) in matching_lines.iter().enumerate() {
                    println!("     {}: {}", i + 1, line);
                }
            }
        }
        Err(e) => {
            println!("❌ Error reading '{}': {}", filename, e);
            println!("   Make sure the file exists and is readable");
        }
    }
    
    println!();
}

fn example3_error_handling_patterns() {
    println!("🎯 Example 3: Robust Error Handling");
    println!("====================================");

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("❌ Insufficient arguments");
        return;
    }
    
    let filename = &args[2];
    
    // Different error handling approaches
    println!("🔍 Analyzing file: '{}'", filename);
    
    // Method 1: Match on Result
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("✅ Method 1 (match): File read successfully");
            println!("   Preview: {}", 
                    contents.lines().next().unwrap_or("(empty file)"));
        }
        Err(e) => {
            println!("❌ Method 1 (match): {}", e);
            
            // Provide helpful context based on error type
            use std::io::ErrorKind;
            match e.kind() {
                ErrorKind::NotFound => {
                    println!("   💡 Suggestion: Check if '{}' exists in current directory", filename);
                }
                ErrorKind::PermissionDenied => {
                    println!("   💡 Suggestion: Check file permissions");
                }
                _ => {
                    println!("   💡 Error details: {}", e);
                }
            }
        }
    }
    
    println!();
}

fn example4_file_operations() {
    println!("⚠️  Example 4: Advanced File Operations");
    println!("=======================================");

    // Demonstrate various file operations
    println!("🔧 File Operation Examples:");
    
    // Check if file exists before reading
    let test_filename = "poem.txt";
    
    if std::path::Path::new(test_filename).exists() {
        println!("✅ File '{}' exists", test_filename);
        
        // Get file metadata
        match fs::metadata(test_filename) {
            Ok(metadata) => {
                println!("   Size: {} bytes", metadata.len());
                println!("   Read-only: {}", metadata.permissions().readonly());
                
                if let Ok(modified) = metadata.modified() {
                    println!("   Modified: {:?}", modified);
                }
            }
            Err(e) => println!("   ❌ Cannot get metadata: {}", e),
        }
        
        // Read file with different methods
        match fs::read(test_filename) {
            Ok(bytes) => {
                println!("   ✅ Raw bytes read: {} bytes", bytes.len());
                // Check if it's valid UTF-8
                match String::from_utf8(bytes) {
                    Ok(_string) => println!("   ✅ Valid UTF-8 content"),
                    Err(_) => println!("   ⚠️  Contains non-UTF-8 bytes"),
                }
            }
            Err(e) => println!("   ❌ Cannot read bytes: {}", e),
        }
    } else {
        println!("❌ File '{}' does not exist", test_filename);
        
        // Create sample file
        let sample_data = "Sample content for testing\nLine 2 with more data\nFinal line";
        match fs::write(test_filename, sample_data) {
            Ok(_) => println!("   ✅ Created sample file '{}'", test_filename),
            Err(e) => println!("   ❌ Failed to create file: {}", e),
        }
    }
    
    println!();
}

fn main() {
    println!("📚 Chapter 12.2: Reading a File\n");

    example1_basic_file_reading();
    example2_file_with_arguments();
    example3_error_handling_patterns();
    example4_file_operations();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 12.3 or run examples in ../refactoring/");
    println!("💡 The poem.txt file has been created for testing");
}