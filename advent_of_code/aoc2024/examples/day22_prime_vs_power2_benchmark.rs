// Example: Prime vs Power-of-2 Operations Benchmark
//
// Demonstrates the massive performance difference between operations that
// the compiler can optimize (powers of 2) vs operations it cannot (primes).
//
// Day 22 benefited from power-of-2 optimizations. This shows what happens
// when you're not so lucky!

use std::hint::black_box;
use std::time::Instant;

// ============================================================================
// POWER OF 2 OPERATIONS (Compiler optimizes these)
// ============================================================================

#[inline(never)]
fn modulo_power2(val: i64) -> i64 {
    val % 16777216 // 2^24 - optimizes to: val & 0xFFFFFF
}

#[inline(never)]
fn multiply_power2(val: i64) -> i64 {
    val * 64 // 2^6 - optimizes to: val << 6
}

#[inline(never)]
fn divide_power2(val: i64) -> i64 {
    val / 32 // 2^5 - optimizes to: val >> 5
}

// ============================================================================
// PRIME NUMBER OPERATIONS (Compiler CANNOT optimize these)
// ============================================================================

#[inline(never)]
fn modulo_prime(val: i64) -> i64 {
    val % 16777213 // Prime close to 2^24 - real modulo instruction
}

#[inline(never)]
fn multiply_prime(val: i64) -> i64 {
    val * 67 // Prime close to 64 - real multiplication
}

#[inline(never)]
fn divide_prime(val: i64) -> i64 {
    val / 31 // Prime close to 32 - real division instruction
}

// ============================================================================
// PRNG SIMULATION: Day 22 Style (Power of 2)
// ============================================================================

fn evolve_secret_power2(mut secret: i64) -> i64 {
    // Step 1: Multiply by 64, XOR, modulo 2^24
    let step1 = secret * 64;
    secret ^= step1;
    secret %= 16777216;

    // Step 2: Divide by 32, XOR, modulo 2^24
    let step2 = secret / 32;
    secret ^= step2;
    secret %= 16777216;

    // Step 3: Multiply by 2048, XOR, modulo 2^24
    let step3 = secret * 2048;
    secret ^= step3;
    secret %= 16777216;

    secret
}

// ============================================================================
// PRNG SIMULATION: Prime Numbers (No optimizations)
// ============================================================================

fn evolve_secret_prime(mut secret: i64) -> i64 {
    // Step 1: Multiply by 67, XOR, modulo prime
    let step1 = secret * 67;
    secret ^= step1;
    secret %= 16777213;

    // Step 2: Divide by 31, XOR, modulo prime
    let step2 = secret / 31;
    secret ^= step2;
    secret %= 16777213;

    // Step 3: Multiply by 2053, XOR, modulo prime
    let step3 = secret * 2053;
    secret ^= step3;
    secret %= 16777213;

    secret
}

// ============================================================================
// Benchmarking Infrastructure
// ============================================================================

fn benchmark_operation<F>(name: &str, iterations: usize, mut op: F) -> std::time::Duration
where
    F: FnMut(i64) -> i64,
{
    let mut rng = 12345i64;

    // Warm-up
    for _ in 0..1000 {
        rng = op(black_box(rng));
    }

    // Benchmark
    let start = Instant::now();
    for _ in 0..iterations {
        rng = op(black_box(rng));
    }
    let elapsed = start.elapsed();

    // Use result to prevent dead code elimination
    black_box(rng);

    println!(
        "{:30} {:>12} ops in {:>10?} ({:>8.2} ns/op)",
        name,
        iterations,
        elapsed,
        elapsed.as_nanos() as f64 / iterations as f64
    );

    elapsed
}

fn main() {
    println!("🔬 Prime vs Power-of-2 Operations Benchmark");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("This benchmark demonstrates the MASSIVE performance difference");
    println!("between operations the compiler can optimize vs those it cannot.");
    println!();

    let iterations = 10_000_000;

    // ========================================================================
    // INDIVIDUAL OPERATION BENCHMARKS
    // ========================================================================

    println!("📊 Individual Operations ({} iterations)", iterations);
    println!("───────────────────────────────────────────────────────────────");

    println!("\n🟢 MODULO OPERATIONS:");
    let mod_power2 = benchmark_operation("Modulo Power-of-2 (2^24)", iterations, modulo_power2);
    let mod_prime = benchmark_operation("Modulo Prime (16777213)", iterations, modulo_prime);
    let mod_speedup = mod_prime.as_secs_f64() / mod_power2.as_secs_f64();
    println!(
        "   └─> Power-of-2 is {:.2}x FASTER (AND vs DIV)\n",
        mod_speedup
    );

    println!("🟢 MULTIPLICATION OPERATIONS:");
    let mul_power2 = benchmark_operation("Multiply Power-of-2 (64)", iterations, multiply_power2);
    let mul_prime = benchmark_operation("Multiply Prime (67)", iterations, multiply_prime);
    let mul_speedup = mul_prime.as_secs_f64() / mul_power2.as_secs_f64();
    println!(
        "   └─> Power-of-2 is {:.2}x FASTER (SHL vs IMUL)\n",
        mul_speedup
    );

    println!("🟢 DIVISION OPERATIONS:");
    let div_power2 = benchmark_operation("Divide Power-of-2 (32)", iterations, divide_power2);
    let div_prime = benchmark_operation("Divide Prime (31)", iterations, divide_prime);
    let div_speedup = div_prime.as_secs_f64() / div_power2.as_secs_f64();
    println!(
        "   └─> Power-of-2 is {:.2}x FASTER (SAR vs IDIV)\n",
        div_speedup
    );

    // ========================================================================
    // FULL PRNG SIMULATION
    // ========================================================================

    println!("\n📊 Full PRNG Simulation (2000 iterations × 2020 secrets)");
    println!("───────────────────────────────────────────────────────────────");

    let secrets = 2020;
    let evolutions = 2000;

    // Power-of-2 version (like Day 22)
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..secrets {
        let mut secret = black_box(i * 1000 + 123);
        for _ in 0..evolutions {
            secret = evolve_secret_power2(secret);
        }
        sum = sum.wrapping_add(secret);
    }
    let power2_time = start.elapsed();
    black_box(sum);

    // Prime version
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..secrets {
        let mut secret = black_box(i * 1000 + 123);
        for _ in 0..evolutions {
            secret = evolve_secret_prime(secret);
        }
        sum = sum.wrapping_add(secret);
    }
    let prime_time = start.elapsed();
    black_box(sum);

    println!("\n🟢 POWER-OF-2 PRNG (Day 22 style):");
    println!("   Time: {:?}", power2_time);
    println!(
        "   Operations: {} × {} = {} total evolutions",
        secrets,
        evolutions,
        secrets * evolutions
    );

    println!("\n🔴 PRIME NUMBER PRNG (no optimizations):");
    println!("   Time: {:?}", prime_time);
    println!(
        "   Operations: {} × {} = {} total evolutions",
        secrets,
        evolutions,
        secrets * evolutions
    );

    let prng_speedup = prime_time.as_secs_f64() / power2_time.as_secs_f64();
    println!(
        "\n🚀 SPEEDUP: Power-of-2 is {:.2}x FASTER than prime version!",
        prng_speedup
    );

    // ========================================================================
    // ANALYSIS
    // ========================================================================

    println!("\n{}", "═".repeat(63));
    println!("💡 KEY INSIGHTS:");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!(
        "1️⃣  MODULO: AND instruction is ~{:.0}x faster than DIV",
        mod_speedup
    );
    println!("   • Power-of-2: val & 0xFFFFFF (1 cycle)");
    println!("   • Prime: val % 16777213 (~20-40 cycles)");
    println!();
    println!(
        "2️⃣  MULTIPLICATION: SHL is ~{:.0}x faster than IMUL",
        mul_speedup
    );
    println!("   • Power-of-2: val << 6 (1 cycle)");
    println!("   • Prime: val * 67 (~3-5 cycles)");
    println!();
    println!("3️⃣  DIVISION: SAR is ~{:.0}x faster than IDIV", div_speedup);
    println!("   • Power-of-2: val >> 5 (1 cycle)");
    println!("   • Prime: val / 31 (~20-40 cycles)");
    println!();
    println!(
        "4️⃣  COMBINED EFFECT: {:.2}x slowdown for prime-based PRNG",
        prng_speedup
    );
    println!("   • Day 22 got LUCKY with power-of-2 constants!");
    println!(
        "   • If AoC used primes, solution would be {}x SLOWER",
        prng_speedup as i32
    );
    println!();
    println!("🎯 LESSON: Choose your constants wisely! Powers of 2 enable");
    println!("   massive compiler optimizations. Primes force expensive operations.");
    println!();

    println!("📋 View assembly comparison:");
    println!(
        "   cargo asm -p aoc2024 --example day22_prime_vs_power2_benchmark --release modulo_power2"
    );
    println!(
        "   cargo asm -p aoc2024 --example day22_prime_vs_power2_benchmark --release modulo_prime"
    );
}
