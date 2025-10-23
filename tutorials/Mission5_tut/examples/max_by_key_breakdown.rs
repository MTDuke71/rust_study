//! # Breaking Down Complex Iterator Patterns
//!
//! **Demonstrates**: Different ways to write the same max_by_key operation

#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    println!("🔍 Breaking Down: max_by_key Pattern Analysis");
    println!("==============================================\n");

    // Sample data
    let mut word_count = HashMap::new();
    word_count.insert("the", 3);
    word_count.insert("fox", 2);
    word_count.insert("quick", 1);
    word_count.insert("brown", 1);

    println!("📊 Word counts: {:?}\n", word_count);

    demonstrate_original_pattern(&word_count);
    demonstrate_verbose_version(&word_count);
    demonstrate_alternative_patterns(&word_count);
    demonstrate_step_by_step(&word_count);
}

/// The original complex line broken down
fn demonstrate_original_pattern(word_count: &HashMap<&str, i32>) {
    println!("🎯 Original Pattern:");
    println!("===================");

    // The original line
    if let Some((most_common_word, max_count)) = word_count.iter().max_by_key(|(_, &count)| count) {
        println!(
            "✅ Most common: '{}' appears {} times",
            most_common_word, max_count
        );
    }

    println!();
}

/// More verbose but clearer version
fn demonstrate_verbose_version(word_count: &HashMap<&str, i32>) {
    println!("📝 Verbose Version (Clearer for Learning):");
    println!("==========================================");

    // Step 1: Get iterator
    let iterator = word_count.iter();
    println!("Step 1: word_count.iter() → Iterator<Item = (&str, &i32)>");

    // Step 2: Find max using explicit closure
    let max_result = iterator.max_by_key(|tuple| {
        let (word, count_ref) = tuple; // Destructure
        let count_value = *count_ref; // Dereference
        println!("  Comparing: '{}' with count {}", word, count_value);
        count_value // Return for comparison
    });

    // Step 3: Handle the Option
    match max_result {
        Some((word, count_ref)) => {
            println!("✅ Winner: '{}' with {} occurrences", word, count_ref);
        }
        None => {
            println!("❌ No words found (empty HashMap)");
        }
    }

    println!();
}

/// Alternative ways to write the same thing
fn demonstrate_alternative_patterns(word_count: &HashMap<&str, i32>) {
    println!("🔄 Alternative Approaches:");
    println!("==========================");

    // Alternative 1: Using map then max
    println!("Alternative 1: Using map + max");
    let max_entry = word_count
        .iter()
        .map(|(word, &count)| (word, count)) // Convert &count to count
        .max_by_key(|(_, count)| *count); // Now count is i32, so deref

    if let Some((word, count)) = max_entry {
        println!("  Result: '{}' → {}", word, count);
    }

    // Alternative 2: Manual comparison
    println!("\nAlternative 2: Manual loop");
    let mut max_word = "";
    let mut max_count = 0;

    for (word, &count) in word_count.iter() {
        if count > max_count {
            max_count = count;
            max_word = word;
        }
    }

    if max_count > 0 {
        println!("  Result: '{}' → {}", max_word, max_count);
    }

    // Alternative 3: Collect and sort
    println!("\nAlternative 3: Collect and sort");
    let mut entries: Vec<_> = word_count.iter().collect();
    entries.sort_by_key(|(_, &count)| std::cmp::Reverse(count)); // Reverse for descending

    if let Some((word, count)) = entries.first() {
        println!("  Result: '{}' → {}", word, count);
    }

    println!();
}

/// Step-by-step explanation of what happens
fn demonstrate_step_by_step(word_count: &HashMap<&str, i32>) {
    println!("🔬 Step-by-Step Execution:");
    println!("==========================");

    println!("Starting with: {:?}", word_count);

    println!("\n1. word_count.iter() produces:");
    for (word, count) in word_count.iter() {
        println!("   ({:?}, {:?}) → (&str, &i32)", word, count);
    }

    println!("\n2. max_by_key closure gets called with each tuple:");
    let mut step = 1;
    let mut current_max = None;

    for (word, count_ref) in word_count.iter() {
        let count_val = *count_ref; // This is what &count does in the pattern
        println!(
            "   Step {}: Processing ('{}', &{}) → closure returns {}",
            step, word, count_ref, count_val
        );

        match current_max {
            None => {
                println!("     → First item, becomes current max");
                current_max = Some((word, count_val));
            }
            Some((_, current_count)) => {
                if count_val > current_count {
                    println!("     → {} > {}, new max!", count_val, current_count);
                    current_max = Some((word, count_val));
                } else {
                    println!(
                        "     → {} ≤ {}, keeping current max",
                        count_val, current_count
                    );
                }
            }
        }
        step += 1;
    }

    if let Some((final_word, final_count)) = current_max {
        println!(
            "\n3. Final result: Some((\"{}\", &{}))",
            final_word, final_count
        );
        println!("4. if let pattern matches and binds:");
        println!("   most_common_word = \"{}\"", final_word);
        println!("   max_count = &{}", final_count);
    }

    println!();
}

/// Understanding the pattern matching syntax
fn pattern_matching_explanation() {
    println!("🧩 Pattern Matching Deep Dive:");
    println!("==============================");

    // Let's see what different patterns do
    let sample_tuple = ("hello", &42i32);

    println!("Sample tuple: {:?}", sample_tuple);
    println!("Type: (&str, &i32)\n");

    // Pattern 1: Basic destructuring
    let (word, count_ref) = sample_tuple;
    println!("Pattern: (word, count_ref)");
    println!("  word = {:?} (type: &str)", word);
    println!("  count_ref = {:?} (type: &i32)", count_ref);

    // Pattern 2: With dereferencing
    let (word2, &count_val) = sample_tuple;
    println!("\nPattern: (word2, &count_val)");
    println!("  word2 = {:?} (type: &str)", word2);
    println!("  count_val = {:?} (type: i32)", count_val);
    println!("  ↳ The & in &count_val dereferences &i32 → i32");

    // Pattern 3: Ignoring parts
    let (_, &just_count) = sample_tuple;
    println!("\nPattern: (_, &just_count)");
    println!("  _ ignores the first element");
    println!("  just_count = {:?} (type: i32)", just_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_understanding() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 3);
        map.insert("c", 2);

        // Test the original pattern
        if let Some((word, &count)) = map.iter().max_by_key(|(_, &count)| count) {
            assert_eq!(*word, "b");
            assert_eq!(count, 3);
        }

        // Test alternative pattern
        if let Some((word, count_ref)) = map.iter().max_by_key(|(_, count_ref)| *count_ref) {
            assert_eq!(*word, "b");
            assert_eq!(*count_ref, 3);
        }
    }
}
