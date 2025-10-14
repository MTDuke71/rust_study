//! # Chapter 8.3: Hash Maps - Storing Keys with Associated Values
//!
//! Complete examples from The Rust Book demonstrating hash map operations.
//!
//! Run with: `cargo run`

use std::collections::HashMap;

fn example1_creating_hashmaps() {
    println!("🗺️  Example 1: Creating Hash Maps");
    println!("==================================");

    // Creating an empty hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);

    // Creating from vectors with collect()
    let teams = [String::from("Blue"), String::from("Yellow")];
    let initial_scores = [10, 50];

    let scores: HashMap<_, _> = teams
        .iter()
        .zip(initial_scores.iter())
        .map(|(team, score)| (team.clone(), *score))
        .collect();

    println!("Scores from zip: {:?}", scores);

    println!();
}

fn example2_accessing_values() {
    println!("🔍 Example 2: Accessing Values");
    println!("===============================");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Using get() - returns Option<&V>
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("Blue team score: {}", s),
        None => println!("Blue team not found"),
    }

    // Using get with copied() and unwrap_or()
    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("Blue team score (with default): {}", score);

    // Accessing non-existent key
    let score = scores.get(&String::from("Red")).copied().unwrap_or(0);
    println!("Red team score (default): {}", score);

    println!();
}

fn example3_iterating() {
    println!("🔄 Example 3: Iterating Over Hash Maps");
    println!("======================================");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 25);

    println!("All scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    println!();
}

fn example4_ownership() {
    println!("🔗 Example 4: Hash Maps and Ownership");
    println!("=====================================");

    // For types that implement Copy (like i32)
    let mut scores = HashMap::new();
    let key = 42;
    let value = 100;
    scores.insert(key, value);
    println!("After insert, key still valid: {}, value: {}", key, value);

    // For owned values like String
    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    map.insert(field_name, field_value);
    // field_name and field_value are now moved and invalid
    // println!("{}", field_name);  // ❌ Won't compile!

    println!("Map: {:?}", map);

    // Using references
    let mut map = HashMap::new();
    let key = String::from("name");
    let value = String::from("Alice");

    map.insert(&key, &value);
    println!("After insert with refs, still valid: {} = {}", key, value);
    // But references must remain valid for lifetime of map

    println!();
}

fn example5_updating_values() {
    println!("✏️  Example 5: Updating Hash Map Values");
    println!("=======================================");

    let mut scores = HashMap::new();

    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    println!("Initial: {:?}", scores);

    scores.insert(String::from("Blue"), 25);
    println!("After overwrite: {:?}", scores);

    // Only insert if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Won't overwrite

    println!("After or_insert: {:?}", scores);

    println!();
}

fn example6_entry_api() {
    println!("🔑 Example 6: Entry API - Advanced Updates");
    println!("==========================================");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Entry API returns Entry enum
    let blue_entry = scores.entry(String::from("Blue"));
    println!("Blue entry: {:?}", blue_entry);

    // or_insert returns a mutable reference
    let score = scores.entry(String::from("Yellow")).or_insert(50);
    println!("Yellow score (new): {}", score);

    *score += 10; // Modify through mutable reference
    println!("After modification: {:?}", scores);

    println!();
}

fn example7_frequency_counting() {
    println!("📊 Example 7: Frequency Counting Pattern");
    println!("========================================");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Text: '{}'", text);
    println!("Word frequencies:");
    for (word, count) in &map {
        println!("  '{}': {}", word, count);
    }

    // Character frequency
    let text = "hello";
    let mut char_freq = HashMap::new();

    for c in text.chars() {
        let count = char_freq.entry(c).or_insert(0);
        *count += 1;
    }

    println!("\nCharacter frequencies in '{}':", text);
    for (ch, count) in &char_freq {
        println!("  '{}': {}", ch, count);
    }

    println!();
}

fn example8_grouping_data() {
    println!("📦 Example 8: Grouping Data");
    println!("===========================");

    let students = vec![
        ("Alice", "Math"),
        ("Bob", "Science"),
        ("Charlie", "Math"),
        ("Diana", "English"),
        ("Eve", "Science"),
        ("Frank", "Math"),
    ];

    let mut by_subject: HashMap<&str, Vec<&str>> = HashMap::new();

    for (name, subject) in students {
        by_subject.entry(subject).or_default().push(name);
    }

    println!("Students by subject:");
    for (subject, names) in &by_subject {
        println!("  {}: {:?}", subject, names);
    }

    println!();
}

fn example9_caching_expensive_computation() {
    println!("💾 Example 9: Caching with Hash Maps");
    println!("====================================");

    fn expensive_computation(n: u32) -> u32 {
        println!("  Computing for {}...", n);
        n * n // Simulating expensive operation
    }

    let mut cache: HashMap<u32, u32> = HashMap::new();

    let inputs = vec![5, 3, 5, 7, 3, 5];

    println!("Computing with cache:");
    for input in inputs {
        let result = cache
            .entry(input)
            .or_insert_with(|| expensive_computation(input));
        println!("Result for {}: {}", input, result);
    }

    println!("\nCache contents: {:?}", cache);

    println!();
}

fn example10_map_methods() {
    println!("🛠️  Example 10: Useful Hash Map Methods");
    println!("========================================");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 25);

    // Length
    println!("Number of teams: {}", scores.len());

    // Is empty
    println!("Is empty: {}", scores.is_empty());

    // Contains key
    println!("Contains 'Blue': {}", scores.contains_key("Blue"));
    println!("Contains 'Green': {}", scores.contains_key("Green"));

    // Remove
    let removed = scores.remove("Red");
    println!("Removed 'Red': {:?}", removed);
    println!("After remove: {:?}", scores);

    // Get keys and values
    println!("\nKeys:");
    for key in scores.keys() {
        println!("  {}", key);
    }

    println!("Values:");
    for value in scores.values() {
        println!("  {}", value);
    }

    // Collect keys and values
    let keys: Vec<_> = scores.keys().collect();
    let values: Vec<_> = scores.values().collect();
    println!("Keys as vec: {:?}", keys);
    println!("Values as vec: {:?}", values);

    // Clear all
    scores.clear();
    println!("After clear: {:?}", scores);

    println!();
}

fn example11_practical_use_case() {
    println!("💼 Example 11: Practical Use Case - Inventory System");
    println!("====================================================");

    #[derive(Debug)]
    struct Item {
        name: String,
        quantity: u32,
        price: f64,
    }

    let mut inventory: HashMap<u32, Item> = HashMap::new();

    // Add items
    inventory.insert(
        101,
        Item {
            name: String::from("Widget"),
            quantity: 50,
            price: 9.99,
        },
    );

    inventory.insert(
        102,
        Item {
            name: String::from("Gadget"),
            quantity: 30,
            price: 14.99,
        },
    );

    inventory.insert(
        103,
        Item {
            name: String::from("Doohickey"),
            quantity: 20,
            price: 7.50,
        },
    );

    // Display inventory
    println!("Current Inventory:");
    for (id, item) in &inventory {
        println!(
            "  ID {}: {} - Qty: {}, Price: ${:.2}",
            id, item.name, item.quantity, item.price
        );
    }

    // Update quantity
    if let Some(item) = inventory.get_mut(&102) {
        item.quantity += 10;
        println!("\nUpdated Gadget quantity to: {}", item.quantity);
    }

    // Calculate total value
    let total_value: f64 = inventory
        .values()
        .map(|item| item.quantity as f64 * item.price)
        .sum();

    println!("Total inventory value: ${:.2}", total_value);

    println!();
}

fn main() {
    println!("=== Chapter 8.3: Hash Maps ===\n");

    example1_creating_hashmaps();
    example2_accessing_values();
    example3_iterating();
    example4_ownership();
    example5_updating_values();
    example6_entry_api();
    example7_frequency_counting();
    example8_grouping_data();
    example9_caching_expensive_computation();
    example10_map_methods();
    example11_practical_use_case();

    println!("✅ All hash map examples completed!");
}
