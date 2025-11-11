// Demonstrating different approaches to make HashMap deterministic for debugging
use std::collections::HashMap;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║          Rust HashMap Deterministic Debugging            ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    demonstrate_default_hashmap_randomness();
    demonstrate_hashbrown_fixed_seed();
    demonstrate_btreemap_alternative();
    demonstrate_indexmap_insertion_order();
}

// Default HashMap uses random seeds for security (SipHash)
fn demonstrate_default_hashmap_randomness() {
    println!("=== 1. Default HashMap (Random Seed for Security) ===");
    println!("Standard HashMap uses SipHash with random seed for DOS protection");
    
    for run in 1..=3 {
        let mut map = HashMap::new();
        map.insert("A", 1);
        map.insert("B", 2);  
        map.insert("C", 3);
        map.insert("D", 4);
        
        print!("Run {}: ", run);
        for (key, _) in &map {
            print!("{} ", key);
        }
        println!();
    }
    
    println!("❌ Iteration order varies between runs (non-deterministic)\n");
}

// Using hashbrown with fixed seed (requires feature flag)
fn demonstrate_hashbrown_fixed_seed() {
    println!("=== 2. Fixed Seed Approaches ===");
    
    // Method 1: Environment variable (works with std HashMap)
    println!("Method 1: RUST_HASH_SEED environment variable");
    println!("Set RUST_HASH_SEED=0 before running program:");
    println!("  $env:RUST_HASH_SEED=0; cargo run --example hashmap_deterministic_demo");
    println!("  # or");  
    println!("  RUST_HASH_SEED=0 cargo run --example hashmap_deterministic_demo");
    
    // Method 2: Using ahash with fixed seed
    println!("\nMethod 2: ahash crate with fixed seed");
    println!("// Add to Cargo.toml: ahash = \"0.8\"");
    println!("use ahash::HashMap as AHashMap;");
    println!("use ahash::RandomState;");
    println!("let hasher = RandomState::with_seed(42);");
    println!("let map: AHashMap<&str, i32> = AHashMap::with_hasher(hasher);");
    
    // Method 3: FxHashMap (deterministic by design)
    println!("\nMethod 3: FxHashMap (Firefox's hash, deterministic)");
    println!("// Add to Cargo.toml: rustc-hash = \"1.1\""); 
    println!("use rustc_hash::FxHashMap;");
    println!("let map: FxHashMap<&str, i32> = FxHashMap::default();");
    println!("// Always produces same hash values for same input");
    
    println!();
}

// BTreeMap as deterministic alternative
fn demonstrate_btreemap_alternative() {
    println!("=== 3. BTreeMap (Always Deterministic) ===");
    use std::collections::BTreeMap;
    
    println!("BTreeMap uses comparison-based ordering (always deterministic):");
    
    for run in 1..=3 {
        let mut map = BTreeMap::new();
        map.insert("D", 4);
        map.insert("A", 1);  
        map.insert("C", 3);
        map.insert("B", 2);
        
        print!("Run {}: ", run);
        for (key, _) in &map {
            print!("{} ", key);
        }
        println!();
    }
    
    println!("✅ Always iterates in sorted order (deterministic)");
    println!("Trade-off: O(log n) operations vs HashMap's O(1) average\n");
}

// IndexMap preserves insertion order
fn demonstrate_indexmap_insertion_order() {
    println!("=== 4. IndexMap (Insertion Order Preserved) ==="); 
    println!("// Add to Cargo.toml: indexmap = \"2.0\"");
    println!("use indexmap::IndexMap;");
    println!();
    println!("IndexMap preserves insertion order:");
    println!("let mut map = IndexMap::new();");
    println!("map.insert(\"first\", 1);");
    println!("map.insert(\"second\", 2);");  
    println!("map.insert(\"third\", 3);");
    println!("// Always iterates: first, second, third");
    println!("✅ Deterministic based on insertion order");
    println!("Performance: Nearly HashMap speed with order guarantee\n");
}

// Demonstration of RUST_HASH_SEED behavior
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_seed_environment() {
        // This test shows how RUST_HASH_SEED affects HashMap
        let mut map1 = HashMap::new();
        map1.insert("A", 1);
        map1.insert("B", 2);
        
        let mut map2 = HashMap::new(); 
        map2.insert("A", 1);
        map2.insert("B", 2);
        
        // With RUST_HASH_SEED=0, these should have same iteration order
        let keys1: Vec<_> = map1.keys().collect();
        let keys2: Vec<_> = map2.keys().collect();
        
        if std::env::var("RUST_HASH_SEED").is_ok() {
            assert_eq!(keys1, keys2, "With fixed seed, iteration should be deterministic");
        } else {
            println!("No RUST_HASH_SEED set - iteration order may vary");
        }
    }
    
    #[test] 
    fn test_btreemap_always_deterministic() {
        use std::collections::BTreeMap;
        
        // BTreeMap should always be deterministic
        for _ in 0..10 {
            let mut map = BTreeMap::new();
            map.insert("C", 3);
            map.insert("A", 1);  
            map.insert("B", 2);
            
            let keys: Vec<_> = map.keys().cloned().collect();
            assert_eq!(keys, vec!["A", "B", "C"], "BTreeMap should always sort keys");
        }
    }
}