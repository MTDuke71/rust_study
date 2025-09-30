use std::collections::HashMap;

fn main() {
    println!("🔍 unwrap_or vs or_insert: The Key Difference");
    println!("==============================================\n");

    let mut config: HashMap<&str, i32> = HashMap::new();
    config.insert("debug_level", 1);
    config.insert("max_connections", 100);
    
    println!("🏁 Initial HashMap contents:");
    for (key, value) in &config {
        println!("  {} -> {}", key, value);
    }
    println!("📏 HashMap size: {}\n", config.len());

    // METHOD 1: unwrap_or - DOES NOT MODIFY HASHMAP
    println!("🔸 Using unwrap_or:");
    let timeout = config.get("timeout").unwrap_or(&30);
    println!("  timeout variable = {}", timeout);
    println!("  🔍 HashMap after unwrap_or:");
    for (key, value) in &config {
        println!("    {} -> {}", key, value);
    }
    println!("  📏 HashMap size: {} (unchanged!)\n", config.len());

    // METHOD 2: or_insert - MODIFIES HASHMAP
    println!("🔹 Using entry().or_insert():");
    let retry_count = config.entry("retry_count").or_insert(3);
    println!("  retry_count variable = {}", retry_count);
    println!("  🔍 HashMap after or_insert:");
    for (key, value) in &config {
        println!("    {} -> {}", key, value);
    }
    println!("  📏 HashMap size: {} (increased!)\n", config.len());

    // Let's test what happens if key already exists
    println!("🔹 Using or_insert on existing key:");
    let debug = config.entry("debug_level").or_insert(999);
    println!("  debug variable = {} (existing value, not 999!)", debug);
    println!("  🔍 HashMap after or_insert on existing key:");
    for (key, value) in &config {
        println!("    {} -> {}", key, value);
    }
    
    println!("\n📚 Summary:");
    println!("  • unwrap_or: Returns default, HashMap unchanged");
    println!("  • or_insert: Inserts default if missing, returns reference");
    println!("  • or_insert: Returns existing value if key already present");
}