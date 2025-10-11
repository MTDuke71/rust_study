use std::collections::HashMap;

/// This demonstrates what a "wrapper" does in Rust
///
/// A wrapper is a design pattern where you:
/// 1. Take an existing type (HashMap)
/// 2. Wrap it inside your own struct  
/// 3. Add additional functionality while preserving the original behavior

fn main() {
    println!("🎁 Understanding the Dictionary Wrapper Pattern");
    println!("===============================================\n");

    // Let's compare direct HashMap usage vs the Dictionary wrapper

    println!("📊 Part 1: What HashMap gives you (basic functionality)");
    println!("======================================================");

    let mut basic_map = HashMap::new();
    basic_map.insert("rust", "systems programming");
    basic_map.insert("python", "data science");

    println!("Basic HashMap operations:");
    println!("  • Insert: basic_map.insert(key, value)");
    println!("  • Get: basic_map.get(key) -> Option<&Value>");
    println!("  • Remove: basic_map.remove(key) -> Option<Value>");
    println!("  • Length: basic_map.len() -> usize");

    for (lang, use_case) in &basic_map {
        println!("    {} -> {}", lang, use_case);
    }

    println!("\n🎯 Part 2: What the Dictionary Wrapper adds");
    println!("===========================================");

    println!("The Dictionary struct wraps HashMap and adds:");
    println!("  ✅ get_or_default() - no need for unwrap_or");
    println!("  ✅ insert_many() - bulk insertion from iterators");
    println!("  ✅ add_to_list() - multi-value support");
    println!("  ✅ Better ergonomics for common patterns");

    println!("\n🔍 Part 3: The Wrapper Structure");
    println!("================================");

    println!("Dictionary<K, V> {{");
    println!("    inner: HashMap<K, V>,  // ← The wrapped HashMap");
    println!("}}");

    println!("\n🎭 Part 4: The Wrapper in Action");
    println!("================================");

    // Simulating what Dictionary does internally
    struct SimpleWrapper<K, V> {
        inner: HashMap<K, V>, // The actual HashMap being wrapped
    }

    impl<K, V> SimpleWrapper<K, V>
    where
        K: std::hash::Hash + Eq,
    {
        fn new() -> Self {
            Self {
                inner: HashMap::new(), // Delegate to HashMap
            }
        }

        // Basic operations - just delegate to inner HashMap
        fn insert(&mut self, key: K, value: V) -> Option<V> {
            self.inner.insert(key, value) // Forward to HashMap
        }

        fn get(&self, key: &K) -> Option<&V> {
            self.inner.get(key) // Forward to HashMap
        }

        // Enhanced operations - this is where wrapper adds value!
        fn get_or_default(&self, key: &K, default: V) -> V
        where
            V: Clone,
        {
            // This is MORE convenient than HashMap's approach
            self.inner.get(key).cloned().unwrap_or(default)
        }
    }

    let mut wrapper = SimpleWrapper::new();
    wrapper.insert("timeout", 30);

    // Compare the convenience:
    println!("HashMap way (more verbose):");
    let timeout1 = wrapper.inner.get("missing_key").unwrap_or(&45);
    println!(
        "  let timeout = map.get(\"missing_key\").unwrap_or(&45); // {}",
        timeout1
    );

    println!("\nWrapper way (more convenient):");
    let timeout2 = wrapper.get_or_default(&"missing_key", 45);
    println!(
        "  let timeout = dict.get_or_default(&\"missing_key\", 45); // {}",
        timeout2
    );

    println!("\n💡 Part 5: Why Use a Wrapper?");
    println!("=============================");

    println!("🎯 Benefits:");
    println!("  • Add domain-specific methods (get_or_default, add_to_list)");
    println!("  • Improve ergonomics for common use cases");
    println!("  • Maintain type safety while adding functionality");
    println!("  • Perfect for Advent of Code - common patterns made easy");
    println!("  • Zero-cost abstraction - no performance penalty");

    println!("\n🔧 Implementation Strategy:");
    println!("  1. Wrap the existing type (HashMap) in your struct");
    println!("  2. Forward basic operations to the inner type");
    println!("  3. Add your own enhanced methods");
    println!("  4. Implement common traits (Default, FromIterator, etc.)");

    println!("\n🚀 Real-World Usage:");
    println!("  • Configuration managers that wrap HashMap");
    println!("  • Cache wrappers that add TTL functionality");
    println!("  • Database connection pools");
    println!("  • Any time you want to extend existing functionality!");

    println!("\n🦀 Rust's Zero-Cost Abstraction:");
    println!("  • At runtime, Dictionary is just a HashMap");
    println!("  • No performance overhead");
    println!("  • All the convenience, none of the cost!");
}
