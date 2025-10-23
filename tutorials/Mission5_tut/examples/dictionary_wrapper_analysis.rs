use std::collections::HashMap;

/// Demonstrating the specific Dictionary wrapper methods you're using
fn main() {
    println!("🔬 Your Dictionary Wrapper - Method by Method Analysis");
    println!("======================================================\n");

    // Let's trace through what each wrapper method does

    println!("🏗️  1. BASIC FORWARDING METHODS");
    println!("===============================");
    println!("These methods just forward calls to the inner HashMap:");

    let mut raw_map = HashMap::new();
    raw_map.insert("key", "value");

    println!("  Dictionary.insert(key, value)");
    println!("    → self.inner.insert(key, value)  // Just forwards to HashMap");
    println!("  Dictionary.get(&key)");
    println!("    → self.inner.get(key)           // Just forwards to HashMap");
    println!("  Dictionary.len()");
    println!("    → self.inner.len()              // Just forwards to HashMap");

    println!("\n🎯 2. ENHANCED CONVENIENCE METHODS");
    println!("==================================");
    println!("These add new functionality not in HashMap:");

    println!("\n🔸 get_or_default() - No more unwrap_or!");

    // HashMap way (what you'd normally write):
    let default_score = 0;
    let score1 = raw_map.get("player1").copied().unwrap_or("default_value");
    println!(
        "  HashMap: map.get(\"player1\").copied().unwrap_or(\"default\") = {}",
        score1
    );

    // Dictionary way (cleaner):
    // dict.get_or_default(&"player1", 0)  // Much cleaner!
    println!("  Dictionary: dict.get_or_default(&\"player1\", 0)  // Much cleaner!");

    println!("\n🔸 insert_many() - Bulk insertion made easy!");

    // HashMap way (verbose):
    let mut map2 = HashMap::new();
    for (k, v) in [("a", 1), ("b", 2), ("c", 3)] {
        map2.insert(k, v);
    }
    println!("  HashMap: Manual loop over [(\"a\", 1), (\"b\", 2), (\"c\", 3)]");

    // Dictionary way (elegant):
    // dict.insert_many([("a", 1), ("b", 2), ("c", 3)])
    println!("  Dictionary: dict.insert_many([(\"a\", 1), (\"b\", 2), (\"c\", 3)])");

    println!("\n🔹 add_to_list() - Multi-value dictionary support!");

    // HashMap way (complex):
    let mut multi_map: HashMap<&str, Vec<String>> = HashMap::new();
    multi_map
        .entry("fruits")
        .or_default()
        .push("apple".to_string());
    multi_map
        .entry("fruits")
        .or_default()
        .push("banana".to_string());
    println!("  HashMap: map.entry(\"fruits\").or_insert_with(Vec::new).push(...)");

    // Dictionary way (simple):
    // dict.add_to_list("fruits", "apple")
    // dict.add_to_list("fruits", "banana")
    println!("  Dictionary: dict.add_to_list(\"fruits\", \"apple\")");

    println!("\n📊 3. THE WRAPPER PATTERN IN YOUR CODE");
    println!("=====================================");

    println!("Your Dictionary struct:");
    println!("  pub struct Dictionary<K, V> {{");
    println!("      inner: HashMap<K, V>,  // ← The actual HashMap");
    println!("  }}");

    println!("\nMethod forwarding example:");
    println!("  pub fn insert(&mut self, key: K, value: V) -> Option<V> {{");
    println!("      self.inner.insert(key, value)  // ← Forwards to HashMap");
    println!("  }}");

    println!("\nValue-added method example:");
    println!("  pub fn get_or_default(&self, key: &K, default: V) -> V {{");
    println!("      self.inner.get(key).cloned().unwrap_or(default)");
    println!("      // ↑ Uses HashMap but adds convenience");
    println!("  }}");

    println!("\n🎪 4. WHY THIS WRAPPER ROCKS");
    println!("============================");

    println!("✅ Advent of Code Benefits:");
    println!("   • get_or_default() - perfect for counters, default configs");
    println!("   • add_to_list() - multi-value mappings (word → anagrams)");
    println!("   • insert_many() - quick setup from parsed input");

    println!("\n✅ Development Benefits:");
    println!("   • Less boilerplate code");
    println!("   • Fewer bugs (no unwrap_or mistakes)");
    println!("   • Domain-specific methods");
    println!("   • All HashMap features still available");

    println!("\n🚀 5. ZERO-COST ABSTRACTION");
    println!("===========================");

    println!("At runtime:");
    println!("  Dictionary<String, i32>  ≡  HashMap<String, i32>");
    println!("  • Same memory layout");
    println!("  • Same performance");
    println!("  • Compiler optimizes away the wrapper");
    println!("  • You get convenience WITHOUT cost!");

    println!("\n🔍 The wrapper pattern is about:");
    println!("   📦 Taking existing functionality (HashMap)");
    println!("   🎁 Wrapping it in your own struct");
    println!("   ⚡ Adding domain-specific enhancements");
    println!("   🆓 While keeping all the original features");

    println!("\nYour Dictionary = HashMap + AoC-friendly convenience methods! 🦀");
}
