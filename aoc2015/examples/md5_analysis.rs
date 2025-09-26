//! # Why MD5 Mining Has No Shortcuts
//! 
//! **Demonstrates**: Mathematical analysis of why brute force is required

use std::collections::HashMap;

fn main() {
    println!("🔬 Mathematical Analysis: Why No Shortcuts Exist");
    println!("===============================================\n");
    
    analyze_hash_distribution();
    demonstrate_unpredictability();
    show_probability_math();
    compare_approaches();
}

/// Shows that MD5 hashes appear random and uniformly distributed
fn analyze_hash_distribution() {
    println!("📊 Hash Distribution Analysis");
    println!("============================");
    
    let secret = "test";
    let mut first_digit_counts = [0; 16]; // Count 0-9, a-f
    
    // Sample first 10000 hashes
    for i in 1..=10000 {
        let hash = format!("{:x}", md5::compute(format!("{secret}{i}")));
        let first_char = hash.chars().next().unwrap();
        let digit = first_char.to_digit(16).unwrap() as usize;
        first_digit_counts[digit] += 1;
    }
    
    println!("First hex digit distribution (should be roughly equal):");
    for (digit, count) in first_digit_counts.iter().enumerate() {
        let hex_char = std::char::from_digit(digit as u32, 16).unwrap();
        println!("  {}: {} times ({:.1}%)", hex_char, count, (*count as f32 / 10000.0) * 100.0);
    }
    
    println!();
}

/// Shows that consecutive hashes are completely unpredictable
fn demonstrate_unpredictability() {
    println!("🎲 Hash Unpredictability");
    println!("========================");
    
    let secret = "demo";
    println!("Consecutive hashes show no pattern:");
    
    for i in 1..=10 {
        let hash = format!("{:x}", md5::compute(format!("{secret}{i}")));
        println!("  {secret}{:2}: {}", i, &hash[..8]); // Show first 8 chars
    }
    
    println!("\n✅ Notice: No pattern, no way to predict next hash!");
    println!();
}

/// Calculate the mathematical probabilities
fn show_probability_math() {
    println!("📐 Probability Mathematics");
    println!("=========================");
    
    // For leading zeros in hexadecimal
    for zeros in 1..=6 {
        let probability = 1.0 / (16.0_f64.powi(zeros));
        let expected_attempts = 1.0 / probability;
        
        println!("For {} leading zeros:", zeros);
        println!("  Probability: 1 in {:.0}", 1.0 / probability);
        println!("  Expected attempts: {:.0}", expected_attempts);
        
        if zeros == 5 {
            println!("  ← This is Part 1 difficulty");
        }
        if zeros == 6 {
            println!("  ← This is Part 2 difficulty");
        }
        println!();
    }
}

/// Compare different approaches (all still brute force)
fn compare_approaches() {
    println!("⚡ Approach Comparison");
    println!("====================");
    
    let secret = "benchmark";
    let target = "000"; // Just 3 zeros for quick demo
    
    // Approach 1: Simple loop
    let start = std::time::Instant::now();
    let result1 = simple_brute_force(secret, target);
    let time1 = start.elapsed();
    
    // Approach 2: Iterator-based
    let start = std::time::Instant::now();
    let result2 = iterator_brute_force(secret, target);
    let time2 = start.elapsed();
    
    println!("Simple loop: found {} in {:?}", result1.unwrap(), time1);
    println!("Iterator:    found {} in {:?}", result2.unwrap(), time2);
    println!("\n💡 Both approaches are still brute force!");
    println!("💡 The only difference is implementation style and performance.");
    
    println!("\n🚫 What DOESN'T work:");
    println!("  ❌ Mathematical formulas");
    println!("  ❌ Pattern recognition");
    println!("  ❌ Shortcuts or tricks");
    println!("  ❌ Predicting the answer");
    
    println!("\n✅ What CAN help:");
    println!("  ✅ Faster MD5 implementations");
    println!("  ✅ Parallel processing");
    println!("  ✅ Better hardware (faster CPU)");
    println!("  ✅ Optimized algorithms (but still brute force)");
}

// Simple brute force approach
fn simple_brute_force(secret: &str, prefix: &str) -> Option<u32> {
    let mut count = 1;
    loop {
        let hash = format!("{:x}", md5::compute(format!("{secret}{count}")));
        if hash.starts_with(prefix) {
            return Some(count);
        }
        count += 1;
        if count > 100_000 { break; } // Safety limit for demo
    }
    None
}

// Iterator-based brute force
fn iterator_brute_force(secret: &str, prefix: &str) -> Option<u32> {
    (1..=100_000)
        .find(|&i| {
            let hash = format!("{:x}", md5::compute(format!("{secret}{i}")));
            hash.starts_with(prefix)
        })
}

/// Demonstrate why rainbow tables don't help
#[allow(dead_code)]
fn why_rainbow_tables_dont_help() {
    println!("🌈 Why Rainbow Tables Don't Help");
    println!("================================");
    
    // Rainbow tables are for reversing known hashes
    // But we don't have a known hash - we have a pattern!
    
    let mut table = HashMap::new();
    let secret = "example";
    
    // We'd need to compute ALL possible hashes anyway
    for i in 1..=1000 {
        let input = format!("{secret}{i}");
        let hash = format!("{:x}", md5::compute(&input));
        table.insert(hash, input);
    }
    
    // Now we'd search the table for hashes starting with zeros
    // But we still had to compute all the hashes!
    
    println!("Problem with rainbow tables:");
    println!("  - We need hashes with PATTERN, not specific hash");
    println!("  - Must pre-compute for each secret key");
    println!("  - Still requires same number of hash calculations");
    println!("  - Actually slower due to storage overhead");
}

/// Show the fundamental cryptographic principle
fn cryptographic_principle() {
    println!("🔐 Cryptographic Principle");
    println!("=========================");
    
    println!("MD5 Design Goals:");
    println!("  1. One-way function (easy to compute hash, hard to reverse)");
    println!("  2. Avalanche effect (small input change → big hash change)");
    println!("  3. Uniform distribution (all hash values equally likely)");
    println!("  4. Collision resistance (hard to find two inputs with same hash)");
    
    println!("\n💡 These properties INTENTIONALLY make shortcuts impossible!");
    println!("💡 If shortcuts existed, MD5 would be 'broken' for cryptography.");
    println!("💡 AoC uses this mathematical hardness as the puzzle!");
}