//! Chapter 3: Flexible Interfaces - Ownership & Destructors
//!
//! Topics: Borrowed vs. Owned, Cow, Fallible/Blocking Destructors

use std::borrow::Cow;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    println!("=== Flexible Interfaces: Ownership & Destructors ===\n");

    borrowed_vs_owned();
    cow_example();
    fallible_destructors();
}

/// Borrowed vs. Owned: Only require ownership when necessary
fn borrowed_vs_owned() {
    println!("--- Borrowed vs. Owned ---");

    let data = vec![1, 2, 3, 4, 5];

    // ✅ Flexible: Operates on reference (doesn't need ownership)
    let sum = sum_slice(&data);
    println!("  Sum (borrowed): {}", sum);
    println!("  Original still usable: {:?}", data);

    // Only take ownership when truly needed
    let owned_result = process_owned(data);
    println!("  Processed (consumed): {}", owned_result);
    // println!("  Original moved: {:?}", data);  // ❌ Would error

    println!();
}

fn sum_slice(items: &[i32]) -> i32 {
    items.iter().sum()
}

fn process_owned(mut items: Vec<i32>) -> i32 {
    items.push(100); // Needs to mutate, requires ownership
    items.into_iter().sum()
}

/// Cow: Clone on Write for conditional ownership
fn cow_example() {
    println!("--- Cow (Clone on Write) ---");

    // Sometimes borrowed, sometimes owned based on runtime
    let result1 = add_prefix("Hello", false);
    println!("  No prefix: '{}' (borrowed)", result1);

    let result2 = add_prefix("World", true);
    println!("  With prefix: '{}' (owned)", result2);

    println!();
}

/// Returns borrowed data if no modification, owned if modified
fn add_prefix<'a>(text: &'a str, add: bool) -> Cow<'a, str> {
    if add {
        Cow::Owned(format!("PREFIX: {}", text)) // Need to allocate
    } else {
        Cow::Borrowed(text) // Can return reference
    }
}

/// Fallible Destructors: Drop cannot fail
fn fallible_destructors() {
    println!("--- Fallible and Blocking Destructors ---");

    println!("  Problem: Drop cannot return Result or be async");
    println!("  Solution: Provide explicit close() method\n");

    // Pattern 1: Explicit close for graceful shutdown
    let mut writer = FileWriter::new("test.txt").unwrap();
    writer.write_data("Hello").unwrap();

    // Explicit close allows error handling
    match writer.close() {
        Ok(()) => println!("  ✅ Closed gracefully"),
        Err(e) => println!("  ❌ Error during close: {}", e),
    }

    // Pattern 2: Drop as best-effort fallback
    let writer2 = FileWriter::new("test2.txt").unwrap();
    // Forgot to call close() - Drop will try best-effort cleanup
    drop(writer2);
    println!("  ⚠️  Drop provided best-effort cleanup (errors ignored)");

    println!();
}

/// FileWriter with explicit close() and best-effort Drop
struct FileWriter {
    file: Option<File>,
}

impl FileWriter {
    fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            file: Some(File::create(path)?),
        })
    }

    fn write_data(&mut self, data: &str) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.write_all(data.as_bytes())?;
        }
        Ok(())
    }

    /// Explicit close allows error handling
    fn close(&mut self) -> io::Result<()> {
        if let Some(mut file) = self.file.take() {
            file.flush()?; // Can fail - caller must handle
            file.sync_all()?;
        }
        Ok(())
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        // Best-effort cleanup - cannot return errors
        if let Some(ref mut file) = self.file {
            let _ = file.flush(); // Ignore errors in Drop
        }
    }
}
