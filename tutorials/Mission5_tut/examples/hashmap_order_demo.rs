use std::collections::HashMap;

fn main() {
    println!("🔍 Demonstrating HashMap Order Behavior");
    println!("=======================================\n");

    // Create a new HashMap
    let mut map = HashMap::new();
    
    println!("📥 Inserting keys in this order: A, B, C, D, E");
    map.insert("A", 1);
    map.insert("B", 2);
    map.insert("C", 3);
    map.insert("D", 4);
    map.insert("E", 5);
    
    println!("📤 Iterating over HashMap (order NOT preserved):");
    for (key, value) in &map {
        print!("{}:{} ", key, value);
    }
    println!("\n");
    
    println!("🔄 Let's try again with different data:");
    let mut numbers = HashMap::new();
    
    println!("📥 Inserting numbers in order: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10");
    for i in 1..=10 {
        numbers.insert(i, i * 10);
    }
    
    println!("📤 HashMap iteration (unpredictable order):");
    for (key, value) in &numbers {
        print!("{}:{} ", key, value);
    }
    println!("\n");
    
    println!("📝 Key Points:");
    println!("• HashMap uses hashing for fast O(1) access");
    println!("• Hash function determines internal storage location");
    println!("• Iteration order is NOT insertion order");
    println!("• Order may vary between program runs!");
    println!("• Use LinkedHashMap or BTreeMap if you need order");
}