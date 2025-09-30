use std::collections::HashMap;

fn main() {
    println!("🧠 Type Inference vs Explicit Types in HashMap");
    println!("===============================================\n");

    // ❌ METHOD 1: Explicit type annotation (more verbose)
    println!("🔹 Method 1: Explicit Types");
    let mut explicit_map: HashMap<i32, &str> = HashMap::new();
    explicit_map.insert(200, "OK");
    explicit_map.insert(404, "Not Found");
    
    println!("  Explicit map: {:?}", explicit_map);

    // ✅ METHOD 2: Type inference (preferred when possible)
    println!("\n🔸 Method 2: Type Inference");
    let mut inferred_map = HashMap::new();
    inferred_map.insert(200, "OK");        // Rust figures out: HashMap<i32, &str>
    inferred_map.insert(404, "Not Found");
    
    println!("  Inferred map: {:?}", inferred_map);

    // 🤔 What if we need to help Rust?
    println!("\n🔍 When Type Inference Needs Help:");
    
    // This would cause a compile error - Rust can't infer!
    // let mut empty_map = HashMap::new();  // ❌ What types?
    // println!("{:?}", empty_map);
    
    // Solution 1: Explicit annotation
    let mut empty_explicit: HashMap<String, i32> = HashMap::new();
    println!("  Empty explicit map: {:?}", empty_explicit);
    
    // Solution 2: Turbofish syntax
    let mut empty_turbofish = HashMap::<String, i32>::new();
    println!("  Empty turbofish map: {:?}", empty_turbofish);
    
    // Solution 3: Insert something to help inference
    let mut empty_with_usage = HashMap::new();
    if false { // Never executed, but helps compiler
        empty_with_usage.insert("key".to_string(), 42);
    }
    println!("  Empty with usage hint: {:?}", empty_with_usage);

    println!("\n📚 Type Inference Guidelines:");
    println!("  ✅ Let Rust infer when usage makes types clear");
    println!("  ✅ Be explicit when needed for clarity or empty collections");
    println!("  ✅ Rust's inference is very smart - trust it!");
    
    // 🎯 Real-world examples
    println!("\n🌍 Real-World Examples:");
    
    // Inferred from usage - very clean!
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    println!("  Game scores: {:?}", scores);
    
    // Mixed types - still inferred!
    let mut mixed = HashMap::new();
    mixed.insert(1, vec!["a", "b"]);
    mixed.insert(2, vec!["c", "d"]);
    println!("  Mixed types: {:?}", mixed);
}