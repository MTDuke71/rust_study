//! # Chapter 8.2: Strings - Storing UTF-8 Encoded Text
//!
//! Complete examples from The Rust Book demonstrating string operations.
//!
//! Run with: `cargo run`

fn example1_creating_strings() {
    println!("📝 Example 1: Creating Strings");
    println!("==============================");

    // Creating an empty string
    let s = String::new();
    println!("Empty string: '{}'", s);

    // From string literal with to_string()
    let data = "initial contents";
    let s = data.to_string();
    println!("From to_string(): '{}'", s);

    // From string literal with String::from()
    let s = String::from("initial contents");
    println!("From String::from(): '{}'", s);

    // UTF-8 strings work with any language
    let hello_english = String::from("Hello");
    let hello_russian = String::from("Здравствуйте");
    let hello_hindi = String::from("नमस्ते");
    let hello_japanese = String::from("こんにちは");
    let hello_arabic = String::from("مرحبا");

    println!("\nGreetings in different languages:");
    println!("  English: {}", hello_english);
    println!("  Russian: {}", hello_russian);
    println!("  Hindi: {}", hello_hindi);
    println!("  Japanese: {}", hello_japanese);
    println!("  Arabic: {}", hello_arabic);

    println!();
}

fn example2_updating_strings() {
    println!("✏️  Example 2: Updating Strings");
    println!("===============================");

    // Using push_str to append string slice
    let mut s = String::from("foo");
    println!("Start: '{}'", s);
    s.push_str("bar");
    println!("After push_str(\"bar\"): '{}'", s);

    // push_str takes a string slice, doesn't take ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1: '{}', s2 still valid: '{}'", s1, s2);

    // Using push to add a single character
    let mut s = String::from("lo");
    s.push('l');
    println!("After push('l'): '{}'", s);

    println!();
}

fn example3_concatenation() {
    println!("➕ Example 3: Concatenating Strings");
    println!("====================================");

    // Using + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 moved here, s2 borrowed
    println!("s1 + &s2 = '{}'", s3);
    // s1 is no longer valid here
    println!("s2 still valid: '{}'", s2);

    // Chaining + operations
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Chained: '{}'", s);

    // Using format! macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("With format!: '{}'", s);
    println!("All strings still valid: '{}', '{}', '{}'", s1, s2, s3);

    println!();
}

fn example4_indexing_not_allowed() {
    println!("🚫 Example 4: Why Indexing Isn't Allowed");
    println!("=========================================");

    let _s1 = String::from("hello");
    // let h = _s1[0];  // ❌ This won't compile!

    // Strings are UTF-8, so indexing is ambiguous
    let hello = String::from("Hola");  // 4 bytes
    println!("'{}' is {} bytes", hello, hello.len());

    let hello = String::from("Здравствуйте");  // 24 bytes (12 Cyrillic chars, 2 bytes each)
    println!("'{}' is {} bytes", hello, hello.len());

    let hello = String::from("नमस्ते");  // Devanagari script
    println!("'{}' is {} bytes", hello, hello.len());

    println!("\nRust doesn't allow indexing because:");
    println!("  1. O(1) performance guarantee would be violated");
    println!("  2. Indexing could return invalid UTF-8 sequences");
    println!("  3. Ambiguous: bytes, scalar values, or grapheme clusters?");

    println!();
}

fn example5_slicing_strings() {
    println!("✂️  Example 5: Slicing Strings (with caution)");
    println!("============================================");

    let hello = "Здравствуйте";

    // Valid slices (on character boundaries)
    let s = &hello[0..4];  // "Зд" (each char is 2 bytes)
    println!("Slice [0..4]: '{}'", s);

    let s = &hello[0..2];  // "З" (first character)
    println!("Slice [0..2]: '{}'", s);

    // This would panic at runtime:
    // let s = &hello[0..1];  // ❌ Not on character boundary!

    // Safe: ASCII strings
    let hello = "hello";
    let s = &hello[0..4];
    println!("ASCII slice [0..4]: '{}'", s);

    println!();
}

fn example6_iterating_over_strings() {
    println!("🔄 Example 6: Iterating Over Strings");
    println!("====================================");

    // Iterating over characters (Unicode scalar values)
    let s = "नमस्ते";
    println!("String: '{}'", s);
    print!("Characters: ");
    for c in s.chars() {
        print!("[{}] ", c);
    }
    println!();

    // Iterating over bytes
    print!("Bytes: ");
    for b in s.bytes() {
        print!("[{}] ", b);
    }
    println!();

    // English text
    let s = "Hello";
    println!("\nString: '{}'", s);
    print!("Characters: ");
    for c in s.chars() {
        print!("[{}] ", c);
    }
    println!();

    // Collecting into Vec
    let letters: Vec<char> = s.chars().collect();
    println!("As Vec: {:?}", letters);

    println!();
}

fn example7_string_methods() {
    println!("🛠️  Example 7: Useful String Methods");
    println!("====================================");

    let s = String::from("  Hello, World!  ");

    // Trimming whitespace
    let trimmed = s.trim();
    println!("Original: '{}'", s);
    println!("Trimmed: '{}'", trimmed);

    // Converting case
    let lower = s.to_lowercase();
    let upper = s.to_uppercase();
    println!("Lowercase: '{}'", lower);
    println!("Uppercase: '{}'", upper);

    // Replacing
    let s = String::from("I like cookies");
    let replaced = s.replace("cookies", "cake");
    println!("Replace: '{}' → '{}'", s, replaced);

    // Checking contents
    let s = String::from("Hello, World!");
    println!("\nString: '{}'", s);
    println!("Starts with 'Hello': {}", s.starts_with("Hello"));
    println!("Ends with 'World!': {}", s.ends_with("World!"));
    println!("Contains 'lo, W': {}", s.contains("lo, W"));

    // Splitting
    let s = "Hello-World-Rust";
    let parts: Vec<&str> = s.split('-').collect();
    println!("\nSplit '{}' by '-': {:?}", s, parts);

    // Splitting whitespace
    let s = "Hello   World  Rust";
    let words: Vec<&str> = s.split_whitespace().collect();
    println!("Split by whitespace: {:?}", words);

    // Repeating
    let s = "ha".repeat(3);
    println!("\n\"ha\".repeat(3) = '{}'", s);

    println!();
}

fn example8_string_vs_str() {
    println!("📊 Example 8: String vs &str");
    println!("=============================");

    // String - owned, mutable, heap-allocated
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("String (owned): '{}'", s);

    // &str - borrowed, immutable, can point to string literals or String
    let s_ref: &str = &s;
    println!("&str (borrowed): '{}'", s_ref);

    // String literals are &str
    let literal: &str = "hello world";
    println!("String literal: '{}'", literal);

    // Function taking &str is more flexible
    fn print_string(s: &str) {
        println!("  Received: '{}'", s);
    }

    let owned = String::from("owned string");
    let slice = "string slice";

    println!("\nFunction accepting &str:");
    print_string(&owned);  // Pass String reference
    print_string(slice);   // Pass string slice directly

    println!();
}

fn example9_practical_text_processing() {
    println!("💼 Example 9: Practical Text Processing");
    println!("=======================================");

    let text = "The quick brown fox jumps over the lazy dog";

    // Count words
    let word_count = text.split_whitespace().count();
    println!("Text: '{}'", text);
    println!("Word count: {}", word_count);

    // Find longest word
    let longest = text
        .split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap();
    println!("Longest word: '{}'", longest);

    // Reverse words
    let reversed: Vec<&str> = text.split_whitespace().rev().collect();
    println!("Words reversed: {:?}", reversed);

    // Count specific letter
    let letter_count = text.chars().filter(|&c| c == 'o').count();
    println!("Letter 'o' appears {} times", letter_count);

    // Build a formatted message
    let name = "Alice";
    let age = 30;
    let message = format!("Hello, {}! You are {} years old.", name, age);
    println!("\nFormatted message: '{}'", message);

    // Multi-line string with line breaks
    let poem = "Roses are red,\nViolets are blue,\nRust is awesome,\nAnd so are you!";
    println!("\nPoem:");
    for line in poem.lines() {
        println!("  {}", line);
    }

    println!();
}

fn main() {
    println!("=== Chapter 8.2: Strings ===\n");

    example1_creating_strings();
    example2_updating_strings();
    example3_concatenation();
    example4_indexing_not_allowed();
    example5_slicing_strings();
    example6_iterating_over_strings();
    example7_string_methods();
    example8_string_vs_str();
    example9_practical_text_processing();

    println!("✅ All string examples completed!");
}
