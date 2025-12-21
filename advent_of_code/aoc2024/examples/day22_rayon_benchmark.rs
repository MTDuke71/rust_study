// Example: Day 22 Rayon Benchmark - Serial vs Parallel PRNG
//
// This file benchmarks serial vs parallel implementations of the Monkey Market PRNG problem.
// 
// Problem: Generate 2000 secret numbers for 2020 buyers
// - Part 1: Sum all 2000th secrets (pure computation, embarrassingly parallel)
// - Part 2: Find best 4-change sequence (HashMap aggregation, interesting parallelization challenge)

use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

const PRUNE_MODULO: i64 = 16777216; // 2^24

#[inline]
fn mix(secret: i64, value: i64) -> i64 {
    secret ^ value
}

#[inline]
fn prune(secret: i64) -> i64 {
    secret % PRUNE_MODULO
}

fn evolve_secret(mut secret: i64) -> i64 {
    let step1 = secret * 64;
    secret = mix(secret, step1);
    secret = prune(secret);
    
    let step2 = secret / 32;
    secret = mix(secret, step2);
    secret = prune(secret);
    
    let step3 = secret * 2048;
    secret = mix(secret, step3);
    secret = prune(secret);
    
    secret
}

fn generate_nth_secret(initial: i64, iterations: usize) -> i64 {
    let mut secret = initial;
    for _ in 0..iterations {
        secret = evolve_secret(secret);
    }
    secret
}

/// Parse input into vector of initial secrets
fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<i64>().expect("Invalid number"))
        .collect()
}

// ========================================================================
// PART 1: Serial vs Parallel Comparison
// ========================================================================

/// Part 1 Serial: Traditional sequential iterator
fn part1_serial(buyers: &[i64]) -> i64 {
    buyers
        .iter()
        .map(|&initial| generate_nth_secret(initial, 2000))
        .sum()
}

/// Part 1 Parallel: Rayon parallel iterator
fn part1_parallel(buyers: &[i64]) -> i64 {
    buyers
        .par_iter()  // <-- Only difference: parallel iterator
        .map(|&initial| generate_nth_secret(initial, 2000))
        .sum()
}

// ========================================================================
// PART 2: HashMap Aggregation Challenge
// ========================================================================

/// Generate price sequences for a single buyer
fn generate_buyer_sequences(initial: i64) -> HashMap<[i8; 4], i64> {
    let mut buyer_sequences = HashMap::new();
    let mut secret = initial;
    let mut prev_price = (secret % 10) as i8;
    let mut changes: Vec<i8> = Vec::with_capacity(2000);
    
    for _ in 0..2000 {
        secret = evolve_secret(secret);
        let price = (secret % 10) as i8;
        let change = price - prev_price;
        changes.push(change);
        prev_price = price;
        
        if changes.len() >= 4 {
            let sequence = [
                changes[changes.len() - 4],
                changes[changes.len() - 3],
                changes[changes.len() - 2],
                changes[changes.len() - 1],
            ];
            buyer_sequences.entry(sequence).or_insert(price as i64);
        }
    }
    
    buyer_sequences
}

/// Part 2 Serial: Sequential processing with HashMap merge
fn part2_serial(buyers: &[i64]) -> i64 {
    let mut sequence_totals: HashMap<[i8; 4], i64> = HashMap::new();
    
    for &initial in buyers {
        let buyer_sequences = generate_buyer_sequences(initial);
        for (sequence, price) in buyer_sequences {
            *sequence_totals.entry(sequence).or_insert(0) += price;
        }
    }
    
    sequence_totals.values().max().copied().unwrap_or(0)
}

/// Part 2 Parallel: Rayon with manual HashMap merging
fn part2_parallel(buyers: &[i64]) -> i64 {
    use std::sync::Mutex;
    
    // Shared HashMap protected by Mutex
    let sequence_totals = Mutex::new(HashMap::new());
    
    buyers.par_iter().for_each(|&initial| {
        let buyer_sequences = generate_buyer_sequences(initial);
        
        // Lock, update, unlock
        let mut totals = sequence_totals.lock().unwrap();
        for (sequence, price) in buyer_sequences {
            *totals.entry(sequence).or_insert(0) += price;
        }
    });
    
    let totals = sequence_totals.into_inner().unwrap();
    totals.values().max().copied().unwrap_or(0)
}

/// Part 2 Parallel Optimized: Thread-local HashMaps with final merge
fn part2_parallel_optimized(buyers: &[i64]) -> i64 {
    // Each thread builds its own HashMap, then we merge at the end
    let thread_maps: Vec<HashMap<[i8; 4], i64>> = buyers
        .par_iter()
        .map(|&initial| generate_buyer_sequences(initial))
        .collect();
    
    // Serial merge of all thread HashMaps
    let mut sequence_totals: HashMap<[i8; 4], i64> = HashMap::new();
    for thread_map in thread_maps {
        for (sequence, price) in thread_map {
            *sequence_totals.entry(sequence).or_insert(0) += price;
        }
    }
    
    sequence_totals.values().max().copied().unwrap_or(0)
}

// ========================================================================
// Benchmarking
// ========================================================================

fn benchmark_part1(buyers: &[i64], _iterations: usize) {
    println!("\n📊 Part 1 Benchmark: Pure Computation (2020 buyers × 2000 secrets)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Warm-up
    let _ = part1_serial(buyers);
    let _ = part1_parallel(buyers);
    
    // Serial benchmark
    let start = Instant::now();
    let result_serial = part1_serial(buyers);
    let serial_time = start.elapsed();
    
    // Parallel benchmark
    let start = Instant::now();
    let result_parallel = part1_parallel(buyers);
    let parallel_time = start.elapsed();
    
    println!("Serial:   {:?} → Result: {}", serial_time, result_serial);
    println!("Parallel: {:?} → Result: {}", parallel_time, result_parallel);
    
    assert_eq!(result_serial, result_parallel, "Results must match!");
    
    let speedup = serial_time.as_secs_f64() / parallel_time.as_secs_f64();
    println!("\n🚀 Speedup: {:.2}x", speedup);
    
    if speedup > 1.5 {
        println!("✅ Parallelization is beneficial!");
    } else if speedup > 0.8 {
        println!("⚠️  Marginal benefit - overhead comparable to gain");
    } else {
        println!("❌ Parallelization slower due to overhead");
    }
}

fn benchmark_part2(buyers: &[i64], _iterations: usize) {
    println!("\n📊 Part 2 Benchmark: HashMap Aggregation Challenge");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Warm-up
    let _ = part2_serial(buyers);
    let _ = part2_parallel(buyers);
    let _ = part2_parallel_optimized(buyers);
    
    // Serial benchmark
    let start = Instant::now();
    let result_serial = part2_serial(buyers);
    let serial_time = start.elapsed();
    
    // Parallel with Mutex benchmark
    let start = Instant::now();
    let result_parallel = part2_parallel(buyers);
    let parallel_time = start.elapsed();
    
    // Parallel optimized (thread-local) benchmark
    let start = Instant::now();
    let result_optimized = part2_parallel_optimized(buyers);
    let optimized_time = start.elapsed();
    
    println!("Serial:              {:?} → Result: {}", serial_time, result_serial);
    println!("Parallel (Mutex):    {:?} → Result: {}", parallel_time, result_parallel);
    println!("Parallel (Optimized): {:?} → Result: {}", optimized_time, result_optimized);
    
    assert_eq!(result_serial, result_parallel, "Parallel result must match serial!");
    assert_eq!(result_serial, result_optimized, "Optimized result must match serial!");
    
    let speedup_mutex = serial_time.as_secs_f64() / parallel_time.as_secs_f64();
    let speedup_optimized = serial_time.as_secs_f64() / optimized_time.as_secs_f64();
    
    println!("\n🚀 Speedup (Mutex):     {:.2}x", speedup_mutex);
    println!("🚀 Speedup (Optimized): {:.2}x", speedup_optimized);
    
    println!("\n💡 Analysis:");
    if speedup_mutex < 1.0 {
        println!("   - Mutex approach is slower due to lock contention");
    }
    if speedup_optimized > speedup_mutex {
        println!("   - Thread-local HashMaps reduce contention");
    }
    if speedup_optimized > 1.5 {
        println!("   ✅ Optimized parallelization is beneficial!");
    } else if speedup_optimized > 0.8 {
        println!("   ⚠️  Marginal benefit - consider workload size");
    } else {
        println!("   ❌ Serial may be better for this problem size");
    }
}

fn main() {
    println!("🍌 Day 22: Monkey Market - Rayon Parallelization Benchmark");
    println!("═══════════════════════════════════════════════════════════════");
    
    // Load real puzzle input
    let input = std::fs::read_to_string("inputs/day22_example.txt")
        .expect("Failed to read input file");
    let buyers = parse_input(&input);
    
    println!("\n📋 Problem Size:");
    println!("   - Buyers: {}", buyers.len());
    println!("   - Secrets per buyer: 2000");
    println!("   - Total PRNG operations: {} × 2000 × 3 steps = {} operations",
             buyers.len(), buyers.len() * 2000 * 3);
    
    let iterations = 5;
    
    // Benchmark Part 1: Pure computation
    benchmark_part1(&buyers, iterations);
    
    // Benchmark Part 2: HashMap aggregation
    benchmark_part2(&buyers, iterations);
    
    println!("\n{}", "═".repeat(60));
    println!("🎓 Key Learnings:");
    println!("1. Part 1: Embarrassingly parallel - each buyer independent");
    println!("2. Part 2: Shared state (HashMap) creates contention");
    println!("3. Thread-local pattern reduces lock contention significantly");
    println!("4. Always measure - parallel isn't always faster!");
}
