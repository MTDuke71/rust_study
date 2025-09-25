//! # Step 1: Basic HashMap Operations
//! 
//! **Learning Objective**: Master fundamental HashMap operations and understand when to use them
//! 
//! **Time Estimate**: 20 minutes
//! 
//! **What You'll Learn**:
//! - Creating and initializing HashMaps
//! - Inserting, retrieving, and updating values
//! - Checking for key existence
//! - Safe value access patterns
//! - Basic iteration over keys and values

use std::collections::HashMap;

/// Example 1: Creating and Basic Operations
/// This shows the most fundamental HashMap usage patterns
fn example_basic_operations() {
    println!("🦀 Example 1: Basic HashMap Operations");
    println!("=====================================");
    
    // Create a new HashMap for player scores
    let mut player_scores = HashMap::new();
    
    // Insert some players and their scores
    player_scores.insert("Alice", 100);
    player_scores.insert("Bob", 85);
    player_scores.insert("Charlie", 92);
    
    println!("📊 Player Scores Loaded:");
    for (player, score) in &player_scores {
        println!("  {} -> {}", player, score);
    }
    
    // Access values safely
    match player_scores.get("Alice") {
        Some(score) => println!("🎯 Alice's score: {}", score),
        None => println!("❌ Alice not found"),
    }
    
    // Check if a key exists
    if player_scores.contains_key("Bob") {
        println!("✅ Bob is in the game!");
    }
    
    // Update a value
    player_scores.insert("Alice", 105); // Alice got bonus points!
    println!("🎉 Alice's new score: {}", player_scores["Alice"]);
    
    println!();
}

/// Example 2: Safe Access Patterns
/// Shows different ways to safely access HashMap values
fn example_safe_access() {
    println!("🔒 Example 2: Safe Access Patterns");
    println!("==================================");
    
    // Create a new HashMap for configuration settings
    let mut config: HashMap<&str, i32> = HashMap::new();
    config.insert("debug_level", 1);
    config.insert("max_connections", 100);
    
    // Pattern 1: Using get() with unwrap_or
    let timeout = config.get("timeout").unwrap_or(&30);
    println!("⏰ Timeout setting: {}", timeout);
    
    // Pattern 2: Using get() with match
    match config.get("debug_level") {
        Some(value) => println!("🐛 Debug level: {}", value),
        None => println!("❓ Debug setting not found"),
    }
    
    // Pattern 3: Using entry API for default values
    let retry_count = config.entry("retry_count").or_insert(3);
    println!("🔄 Retry count: {}", retry_count);
    
    println!("📋 Final config:");
    for (key, value) in &config {
        println!("  {} -> {:?}", key, value);
    }
    
    println!();
}

/// Example 3: Working with Different Types
/// Demonstrates type flexibility with HashMap
fn example_different_types() {
    println!("🎨 Example 3: Different Data Types");
    println!("==================================");
    
    // String keys with integer values
    let mut inventory: HashMap<String, u32> = HashMap::new();
    inventory.insert("sword".to_string(), 1);
    inventory.insert("potion".to_string(), 5);
    inventory.insert("gold".to_string(), 100);
    
    println!("🎒 Inventory:");
    for (item, count) in &inventory {
        println!("  {} x{}", item, count);
    }
    
    // Integer keys with string values
    let mut error_codes = HashMap::new();
    error_codes.insert(404, "Not Found");
    error_codes.insert(500, "Internal Server Error");
    error_codes.insert(200, "OK");
    
    println!("📟 Error Codes:");
    for (code, message) in &error_codes {
        println!("  {} -> {}", code, message);
    }
    
    // Tuple keys (useful for coordinates!)
    let mut grid = HashMap::new();
    grid.insert((0, 0), "start");
    grid.insert((1, 1), "treasure");
    grid.insert((-1, 2), "monster");
    
    println!("🗺️  Grid Positions:");
    for ((x, y), content) in &grid {
        println!("  ({}, {}) -> {}", x, y, content);
    }
    
    println!();
}

/// Example 4: Updating and Modifying Values
/// Shows patterns for updating existing values
fn example_updating_values() {
    println!("✏️  Example 4: Updating Values");
    println!("=============================");
    
    let mut word_count = HashMap::new();
    let text = "hello world hello rust world";
    
    // Count word occurrences using entry API
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("📊 Word Frequencies:");
    for (word, count) in &word_count {
        println!("  '{}' appears {} times", word, count);
    }
    
    // Alternative pattern: check then update
    let mut scores = HashMap::new();
    scores.insert("player1", 10);
    
    // Add bonus points
    if let Some(score) = scores.get_mut("player1") {
        *score += 5; // Bonus!
        println!("🎉 Player1's new score: {}", score);
    }
    
    // Remove a player
    if let Some(removed_score) = scores.remove("player1") {
        println!("👋 Removed player1 with score: {}", removed_score);
    }
    
    println!();
}

/// Example 5: Advanced Iteration Patterns
/// Demonstrates different ways to iterate over HashMap
fn example_iteration_patterns() {
    println!("🔄 Example 5: Iteration Patterns");
    println!("================================");
    
    let mut grades = HashMap::new();
    grades.insert("Math", 95);
    grades.insert("Science", 87);
    grades.insert("History", 91);
    grades.insert("Art", 78);
    
    // Iterate over key-value pairs
    println!("📚 All Grades:");
    for (subject, grade) in &grades {
        println!("  {} -> {}", subject, grade);
    }
    
    // Iterate over just keys
    println!("📖 Subjects:");
    for subject in grades.keys() {
        println!("  - {}", subject);
    }
    
    // Iterate over just values
    let total: i32 = grades.values().sum();
    let average = total as f32 / grades.len() as f32;
    println!("📊 Average grade: {:.1}", average);
    
    // Filter while iterating
    println!("🏆 High grades (90+):");
    for (subject, grade) in grades.iter().filter(|(_, &grade)| grade >= 90) {
        println!("  {} -> {}", subject, grade);
    }
    
    println!();
}

// 🧪 EXERCISE TIME! 
// Complete these functions to practice what you've learned

/// EXERCISE 1: Build a simple phone book
/// 
/// **Task**: Create a HashMap that maps names to phone numbers
/// **Challenge**: Add at least 3 contacts and implement a lookup function
fn exercise_phone_book() {
    println!("🧪 EXERCISE 1: Phone Book");
    println!("========================");
    
    // TODO: Create a HashMap<&str, &str> for name -> phone number
    let mut phone_book: HashMap<&str, &str> = HashMap::new();
    // TODO: Add these contacts:
    //   - "Alice" -> "555-0123"
    //   - "Bob" -> "555-0456"  
    //   - "Charlie" -> "555-0789"
    
    // TODO: Implement phone lookup
    let _name_to_find = "Alice";
    // TODO: Print Alice's phone number or "Not found" message
    
    // TODO: Check if "David" exists in the phone book

    phone_book.insert("Alice", "555-0123");
    phone_book.insert("Bob", "555-0456");
    phone_book.insert("Charlie", "555-0789");
    
    let name_to_find = "Alice";
    match phone_book.get(name_to_find) {
        Some(number) => println!("📞 {}: {}", name_to_find, number),
        None => println!("❌ {} not found", name_to_find),
    }
    
    if phone_book.contains_key("David") {
        println!("✅ David is in the phone book");
    } else {
        println!("❌ David is not in the phone book");
    }
    
    println!("✅ Exercise 1 complete!\n");
}

/// EXERCISE 2: Shopping cart calculator
/// 
/// **Task**: Calculate the total cost of items in a shopping cart
/// **Challenge**: Handle items that might not be in the price list
fn exercise_shopping_cart() {
    println!("🧪 EXERCISE 2: Shopping Cart");
    println!("===========================");
    
    // Price list
    let mut prices = HashMap::new();
    prices.insert("apple", 1.20);
    prices.insert("banana", 0.80);
    prices.insert("orange", 1.50);
    prices.insert("bread", 2.50);
    
    // Shopping cart (item, quantity)
    let cart = vec![
        ("apple", 3),
        ("banana", 2),
        ("orange", 1),
        ("milk", 1), // This item is not in our price list!
    ];
    
    // TODO: Calculate total cost
    // TODO: Handle missing items gracefully
    // TODO: Print itemized receipt

    let mut total = 0.0;
    println!("🧾 Receipt:");
    
    for (item, quantity) in cart {
        match prices.get(item) {
            Some(price) => {
                let item_total = price * quantity as f64;
                total += item_total;
                println!("  {} x{} @ ${:.2} = ${:.2}", item, quantity, price, item_total);
            },
            None => {
                println!("  {} x{} - PRICE NOT FOUND", item, quantity);
            }
        }
    }
    
    println!("💰 Total: ${:.2}", total);
    
    println!("✅ Exercise 2 complete!\n");
}

/// EXERCISE 3: Letter frequency analyzer
/// 
/// **Task**: Count how many times each letter appears in a string
/// **Challenge**: Ignore spaces and convert to lowercase
fn exercise_letter_frequency() {
    println!("🧪 EXERCISE 3: Letter Frequency");
    println!("==============================");
    
    let text = "Hello World Programming";
    
    // TODO: Create HashMap<char, u32> for letter -> count
    // TODO: Process each character:
    //   - Skip spaces
    //   - Convert to lowercase
    //   - Count frequency
    
    // TODO: Print results in alphabetical order (hint: collect and sort)


    let mut letter_count = HashMap::new();
    
    for ch in text.chars() {
        if ch != ' ' {
            let lower_ch = ch.to_lowercase().next().unwrap();
            let count = letter_count.entry(lower_ch).or_insert(0);
            *count += 1;
        }
    }
    
    let mut letters: Vec<_> = letter_count.iter().collect();
    letters.sort_by_key(|&(ch, _)| ch);
    
    println!("📊 Letter frequencies:");
    for (letter, count) in letters {
        println!("  '{}': {}", letter, count);
    }
    
    println!("✅ Exercise 3 complete!\n");
}

// 📝 SOLUTION SECTION
// Don't look here until you've tried the exercises!

#[cfg(feature = "solutions")]
mod solutions {
    use super::*;
    
    fn exercise_phone_book_solution() {
        let mut phone_book = HashMap::new();
        phone_book.insert("Alice", "555-0123");
        phone_book.insert("Bob", "555-0456");
        phone_book.insert("Charlie", "555-0789");
        
        let name_to_find = "Alice";
        match phone_book.get(name_to_find) {
            Some(number) => println!("📞 {}: {}", name_to_find, number),
            None => println!("❌ {} not found", name_to_find),
        }
        
        if phone_book.contains_key("David") {
            println!("✅ David is in the phone book");
        } else {
            println!("❌ David is not in the phone book");
        }
    }
    
    fn exercise_shopping_cart_solution() {
        let mut prices = HashMap::new();
        prices.insert("apple", 1.20);
        prices.insert("banana", 0.80);
        prices.insert("orange", 1.50);
        prices.insert("bread", 2.50);
        
        let cart = vec![
            ("apple", 3),
            ("banana", 2),
            ("orange", 1),
            ("milk", 1),
        ];
        
        let mut total = 0.0;
        println!("🧾 Receipt:");
        
        for (item, quantity) in cart {
            match prices.get(item) {
                Some(price) => {
                    let item_total = price * quantity as f64;
                    total += item_total;
                    println!("  {} x{} @ ${:.2} = ${:.2}", item, quantity, price, item_total);
                },
                None => {
                    println!("  {} x{} - PRICE NOT FOUND", item, quantity);
                }
            }
        }
        
        println!("💰 Total: ${:.2}", total);
    }
    
    fn exercise_letter_frequency_solution() {
        let text = "Hello World Programming";
        let mut letter_count = HashMap::new();
        
        for ch in text.chars() {
            if ch != ' ' {
                let lower_ch = ch.to_lowercase().next().unwrap();
                let count = letter_count.entry(lower_ch).or_insert(0);
                *count += 1;
            }
        }
        
        let mut letters: Vec<_> = letter_count.iter().collect();
        letters.sort_by_key(|&(ch, _)| ch);
        
        println!("📊 Letter frequencies:");
        for (letter, count) in letters {
            println!("  '{}': {}", letter, count);
        }
    }
}

/// Main function - runs all examples and exercises
fn main() {
    println!("🦀 Step 1: Basic HashMap Operations");
    println!("===================================\n");
    
    // Run all examples
    example_basic_operations();
    example_safe_access();
    example_different_types();
    example_updating_values();
    example_iteration_patterns();
    
    // Exercise time!
    println!("🎯 Now it's your turn! Complete these exercises:");
    println!("================================================\n");
    
    exercise_phone_book();
    exercise_shopping_cart();
    exercise_letter_frequency();
    
    println!("🎉 Congratulations! You've completed Step 1!");
    println!("📚 Ready for Step 2? Run: cargo run --example step2_hashset_operations");
    println!();
    
    // Uncomment to see solutions:
    // #[cfg(feature = "solutions")]
    // {
    //     println!("💡 Solutions (only look after trying!):");
    //     solutions::exercise_phone_book_solution();
    //     solutions::exercise_shopping_cart_solution();
    //     solutions::exercise_letter_frequency_solution();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_hashmap_operations() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
        assert_eq!(map.get("missing"), None);
        assert!(map.contains_key("key"));
        assert!(!map.contains_key("missing"));
    }
    
    #[test]
    fn test_word_counting() {
        let mut word_count = HashMap::new();
        let words = vec!["hello", "world", "hello"];
        
        for word in words {
            let count = word_count.entry(word).or_insert(0);
            *count += 1;
        }
        
        assert_eq!(word_count.get("hello"), Some(&2));
        assert_eq!(word_count.get("world"), Some(&1));
    }
}