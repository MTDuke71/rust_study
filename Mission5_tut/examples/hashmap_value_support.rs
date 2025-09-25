use std::collections::HashMap;

fn main() {
    println!("🔍 HashMap Value Support: Vectors, Lists & Multi-Value Pattern");
    println!("==============================================================\n");

    println!("📊 Part 1: HashMap's Built-in Value Support");
    println!("============================================");
    
    // HashMap supports ANY type as values by default - no special traits needed!
    let mut basic_map: HashMap<&str, Vec<String>> = HashMap::new();
    
    println!("✅ HashMap ALREADY supports vectors as values:");
    basic_map.insert("fruits", vec!["apple".to_string(), "banana".to_string()]);
    basic_map.insert("colors", vec!["red".to_string(), "blue".to_string()]);
    
    for (key, values) in &basic_map {
        println!("  {} -> {:?}", key, values);
    }
    
    println!("\n🎯 HashMap Value Type Requirements:");
    println!("===================================");
    println!("For VALUES (V in HashMap<K, V>):");
    println!("  ❌ NO trait requirements at all!");
    println!("  ✅ Can be ANY type: Vec<T>, String, i32, custom structs, etc.");
    
    println!("\nFor KEYS (K in HashMap<K, V>):");
    println!("  ✅ MUST implement: Eq + Hash");
    println!("  ❌ Vec<T> cannot be a KEY (not hashable)");
    println!("  ✅ Vec<T> CAN be a VALUE (no constraints)");
    
    println!("\n🆚 Part 2: Raw HashMap vs Dictionary Wrapper");
    println!("============================================");
    
    println!("🔸 Raw HashMap approach (verbose):");
    let mut raw_map: HashMap<&str, Vec<String>> = HashMap::new();
    
    // Adding to a list - the manual way
    raw_map.entry("animals").or_insert_with(Vec::new).push("cat".to_string());
    raw_map.entry("animals").or_insert_with(Vec::new).push("dog".to_string());
    
    println!("  map.entry(\"animals\").or_insert_with(Vec::new).push(\"cat\".to_string());");
    println!("  map.entry(\"animals\").or_insert_with(Vec::new).push(\"dog\".to_string());");
    
    // Getting a list - the manual way
    let animals = raw_map.get("animals").cloned().unwrap_or_default();
    println!("  let animals = map.get(\"animals\").cloned().unwrap_or_default();");
    println!("  Result: {:?}", animals);
    
    println!("\n🔹 Dictionary wrapper approach (convenient):");
    // This simulates what your Dictionary does
    println!("  dict.add_to_list(\"animals\", \"cat\");");
    println!("  dict.add_to_list(\"animals\", \"dog\");");
    println!("  let animals = dict.get_list(&\"animals\");");
    
    println!("\n💡 Part 3: What the Dictionary Wrapper Actually Does");
    println!("===================================================");
    
    println!("The multi-value Dictionary methods are CONVENIENCE wrappers:");
    
    println!("\n🔧 add_to_list() implementation:");
    println!("  pub fn add_to_list(&mut self, key: K, value: &str) {{");
    println!("      self.inner.entry(key).or_insert_with(Vec::new).push(value.to_string());");
    println!("      //     ↑ This is just standard HashMap entry API!");
    println!("  }}");
    
    println!("\n🔧 get_list() implementation:");
    println!("  pub fn get_list(&self, key: &K) -> Vec<String> {{");
    println!("      self.inner.get(key).cloned().unwrap_or_default()");
    println!("      //     ↑ This is just standard HashMap get!");
    println!("  }}");
    
    println!("\n🚀 Part 4: HashMap's Native Multi-Value Patterns");
    println!("===============================================");
    
    println!("HashMap ALREADY supports these patterns natively:");
    
    // Pattern 1: Vec<T> values
    let mut word_groups: HashMap<char, Vec<String>> = HashMap::new();
    word_groups.entry('a').or_insert_with(Vec::new).push("apple".to_string());
    word_groups.entry('a').or_insert_with(Vec::new).push("avocado".to_string());
    
    // Pattern 2: HashSet<T> values (for uniqueness)
    use std::collections::HashSet;
    let mut unique_tags: HashMap<&str, HashSet<&str>> = HashMap::new();
    unique_tags.entry("post1").or_insert_with(HashSet::new).insert("rust");
    unique_tags.entry("post1").or_insert_with(HashSet::new).insert("programming");
    
    // Pattern 3: Custom struct values
    #[derive(Debug)]
    struct PlayerData {
        scores: Vec<i32>,
        achievements: Vec<String>,
    }
    
    let mut players: HashMap<&str, PlayerData> = HashMap::new();
    players.insert("alice", PlayerData {
        scores: vec![100, 85, 92],
        achievements: vec!["first_win".to_string(), "high_scorer".to_string()],
    });
    
    println!("✅ HashMap<char, Vec<String>>        // Letters -> word lists");
    println!("✅ HashMap<&str, HashSet<&str>>      // Posts -> unique tags");
    println!("✅ HashMap<&str, PlayerData>         // Players -> complex data");
    
    println!("\n📈 Part 5: The Value Proposition");
    println!("=================================");
    
    println!("HashMap gives you the CAPABILITY:");
    println!("  • Any type can be a value (Vec, custom structs, etc.)");
    println!("  • Full control over how you manage complex values");
    
    println!("\nDictionary wrapper gives you CONVENIENCE:");
    println!("  • Simplified API for common multi-value patterns");
    println!("  • Less boilerplate code");
    println!("  • Consistent error handling");
    
    println!("\n🎯 Answer to Your Question:");
    println!("===========================");
    println!("• HashMap supports vectors as values BY DEFAULT ✅");
    println!("• No special traits needed for values ✅");
    println!("• Dictionary wrapper adds CONVENIENCE, not capability ✅");
    println!("• The multi-value support exists because of ERGONOMICS, not REQUIREMENTS ✅");
    
    println!("\n🔍 Proof - This works without any wrapper:");
    let mut proof: HashMap<&str, Vec<i32>> = HashMap::new();
    proof.insert("numbers", vec![1, 2, 3]);
    proof.get_mut("numbers").unwrap().push(4);
    println!("  Raw HashMap with Vec values: {:?}", proof.get("numbers"));
    
    println!("\n🦀 HashMap is incredibly flexible with values - your Dictionary just makes common patterns easier!");
}