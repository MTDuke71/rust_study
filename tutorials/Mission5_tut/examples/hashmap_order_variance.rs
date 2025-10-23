use std::collections::HashMap;

fn main() {
    println!("🔄 Why HashMap Order Varies Between Runs");
    println!("=========================================\n");

    println!("🧪 Let's run the same HashMap operations multiple times:");

    for run in 1..=3 {
        println!("\n🔢 Run #{}:", run);
        let mut map = HashMap::new();

        // Insert the same keys in the same order every time
        map.insert("First", 1);
        map.insert("Second", 2);
        map.insert("Third", 3);
        map.insert("Fourth", 4);
        map.insert("Fifth", 5);

        print!("   Order: ");
        for key in map.keys() {
            print!("{} ", key);
        }
        println!();
    }

    println!("\n🔍 Why This Happens:");
    println!("===================");

    // Demonstrate hash values
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let keys = ["First", "Second", "Third", "Fourth", "Fifth"];

    println!("\n📊 Hash Values (these determine storage location):");
    for key in &keys {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        println!(
            "   '{}' -> hash: {} (last 4 digits: {})",
            key,
            hash_value,
            hash_value % 10000
        );
    }

    println!("\n🎲 Randomization Factors:");
    println!("   • Hash seed randomization (security feature)");
    println!("   • Memory layout variations");
    println!("   • ASLR (Address Space Layout Randomization)");
    println!("   • Different hash states between runs");

    println!("\n⚡ Key Insight:");
    println!("   HashMap prioritizes SPEED over order consistency!");
    println!("   This randomization also provides DoS attack protection.");
}
