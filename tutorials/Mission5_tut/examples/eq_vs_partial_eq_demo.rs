// Eq vs PartialEq - Understanding the Difference
// A focused example showing why the distinction matters

#![allow(clippy::eq_op)] // Intentional self-comparisons to demonstrate reflexivity

use std::collections::HashMap;

fn main() {
    println!("=== Eq vs PartialEq Explained ===\n");

    // 1. PartialEq without Eq - Floating Point Example
    demo_partial_eq_without_eq();

    // 2. Both PartialEq and Eq - Integer Example
    demo_both_traits();

    // 3. Custom Type Example
    demo_custom_type();

    // 4. Why HashMap/HashSet need Eq
    demo_hash_collection_requirements();
}

fn demo_partial_eq_without_eq() {
    println!("1. PartialEq WITHOUT Eq - Floating Point Numbers:");

    let x = 42.0_f64;
    let y = 42.0_f64;
    let nan = f64::NAN;

    println!("   Regular numbers:");
    println!("   x = {}, y = {}", x, y);
    println!("   x == y: {}", x == y); // true - symmetric
    println!("   x == x: {}", x == x); // true - reflexive (for normal numbers)

    println!("\n   NaN breaks reflexivity:");
    println!("   NaN == NaN: {}", nan == nan); // false! Violates reflexivity
    println!("   NaN != NaN: {}", nan != nan); // true! This is weird but correct per IEEE 754

    println!("   ❌ f64 implements PartialEq but NOT Eq");
    println!("   ❌ Cannot use f64 as HashMap key because reflexivity is broken\n");
}

fn demo_both_traits() {
    println!("2. Both PartialEq AND Eq - Integer Example:");

    let x = 42_i32;
    let y = 42_i32;

    println!("   x = {}, y = {}", x, y);
    println!("   x == y: {}", x == y); // true - symmetric
    println!("   x == x: {}", x == x); // true - reflexive (always!)

    println!("   ✅ i32 implements both PartialEq AND Eq");
    println!("   ✅ Can use i32 as HashMap key because reflexivity is guaranteed\n");
}

fn demo_custom_type() {
    println!("3. Custom Type - Implementing Both Traits:");

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Person {
        id: u32,
        name: String,
    }

    let alice1 = Person {
        id: 1,
        name: "Alice".to_string(),
    };
    let alice2 = Person {
        id: 1,
        name: "Alice".to_string(),
    };
    let bob = Person {
        id: 2,
        name: "Bob".to_string(),
    };

    println!("   alice1 == alice2: {}", alice1 == alice2); // true
    println!("   alice1 == alice1: {}", alice1 == alice1); // true (reflexive)
    println!("   alice1 == bob: {}", alice1 == bob); // false

    println!("   ✅ Person implements both PartialEq and Eq");
    println!("   ✅ Can use Person in HashMap/HashSet\n");
}

fn demo_hash_collection_requirements() {
    println!("4. Why HashMap/HashSet Need Eq:");

    // This works - i32 has Eq
    let mut int_map: HashMap<i32, &str> = HashMap::new();
    int_map.insert(42, "answer");
    println!("   ✅ HashMap<i32, &str> works fine");

    // This would NOT compile - f64 lacks Eq
    // let mut float_map: HashMap<f64, &str> = HashMap::new();  // Compile error!
    println!("   ❌ HashMap<f64, &str> won't compile - f64 lacks Eq trait");

    println!("\n   Why Eq is required for hash collections:");
    println!("   - Hash invariant: if a == b, then hash(a) == hash(b)");
    println!("   - This requires reflexive equality (x == x must always be true)");
    println!("   - f64::NAN violates this, so f64 can't implement Eq");
    println!("   - Without Eq, HashMap can't guarantee correct behavior");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_vs_partial_eq_understanding() {
        // f64 implements PartialEq but not Eq
        let x = 42.0_f64;
        assert_eq!(x, 42.0); // PartialEq works

        let nan = f64::NAN;
        assert_ne!(nan, nan); // NaN is not equal to itself!

        // i32 implements both PartialEq and Eq
        let y = 42_i32;
        assert_eq!(y, 42); // PartialEq works
        assert_eq!(y, y); // Reflexivity guaranteed by Eq
    }

    #[test]
    fn test_hash_map_with_eq_types() {
        use std::collections::HashMap;

        // This compiles because i32 implements Eq
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");

        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
    }
}
