// Example: Day 22 Assembly Inspection
// 
// Demonstrates compiler optimizations for PRNG operations:
// 1. Modulo 2^24 → Bitwise AND
// 2. Multiply/Divide by powers of 2 → Bit shifts
// 3. XOR inlining

const PRUNE_MODULO: i64 = 16777216; // 2^24

/// Mix operation: XOR (should inline completely)
#[inline(never)]  // Prevent inlining so we can see it in assembly
pub fn mix_noinline(secret: i64, value: i64) -> i64 {
    secret ^ value
}

/// Prune operation: Modulo 2^24
/// Compiler should optimize to: secret & 0xFFFFFF
#[inline(never)]
pub fn prune_noinline(secret: i64) -> i64 {
    secret % PRUNE_MODULO
}

/// Prune with explicit bitwise AND (what compiler generates)
#[inline(never)]
pub fn prune_optimized(secret: i64) -> i64 {
    secret & 0xFFFFFF  // 16777215 = 2^24 - 1
}

/// Multiply by 64 (should become: shl rax, 6)
#[inline(never)]
pub fn multiply_64(secret: i64) -> i64 {
    secret * 64
}

/// Divide by 32 (should become: sar rax, 5)
#[inline(never)]
pub fn divide_32(secret: i64) -> i64 {
    secret / 32
}

/// Multiply by 2048 (should become: shl rax, 11)
#[inline(never)]
pub fn multiply_2048(secret: i64) -> i64 {
    secret * 2048
}

/// Full evolve_secret with no inlining to see all operations
#[inline(never)]
pub fn evolve_secret_noinline(mut secret: i64) -> i64 {
    // Step 1: Multiply by 64, mix, prune
    let step1 = secret * 64;
    secret ^= step1;
    secret %= PRUNE_MODULO;
    
    // Step 2: Divide by 32, mix, prune
    let step2 = secret / 32;
    secret ^= step2;
    secret %= PRUNE_MODULO;
    
    // Step 3: Multiply by 2048, mix, prune
    let step3 = secret * 2048;
    secret ^= step3;
    secret %= PRUNE_MODULO;
    
    secret
}

/// Optimized version with explicit bit operations
#[inline(never)]
pub fn evolve_secret_optimized(mut secret: i64) -> i64 {
    // Step 1: Shift left 6 (multiply by 64), XOR, AND mask
    let step1 = secret << 6;
    secret ^= step1;
    secret &= 0xFFFFFF;
    
    // Step 2: Shift right 5 (divide by 32), XOR, AND mask
    let step2 = secret >> 5;
    secret ^= step2;
    secret &= 0xFFFFFF;
    
    // Step 3: Shift left 11 (multiply by 2048), XOR, AND mask
    let step3 = secret << 11;
    secret ^= step3;
    secret &= 0xFFFFFF;
    
    secret
}

fn main() {
    println!("🔍 Day 22: Assembly Inspection Examples");
    println!("=========================================\n");
    
    let test_secret = 123i64;
    
    println!("Testing with secret: {}", test_secret);
    println!();
    
    // Test individual operations
    println!("Individual Operations:");
    println!("  mix({}, 456) = {}", test_secret, mix_noinline(test_secret, 456));
    println!("  prune({}) = {}", 100000000, prune_noinline(100000000));
    println!("  prune_optimized({}) = {}", 100000000, prune_optimized(100000000));
    println!();
    
    // Test arithmetic operations
    println!("Arithmetic Operations (should become bit shifts):");
    println!("  {} * 64 = {}", test_secret, multiply_64(test_secret));
    println!("  {} / 32 = {}", test_secret, divide_32(test_secret));
    println!("  {} * 2048 = {}", test_secret, multiply_2048(test_secret));
    println!();
    
    // Test full evolution
    let result1 = evolve_secret_noinline(test_secret);
    let result2 = evolve_secret_optimized(test_secret);
    
    println!("Full Evolution:");
    println!("  evolve_secret_noinline({}) = {}", test_secret, result1);
    println!("  evolve_secret_optimized({}) = {}", test_secret, result2);
    println!();
    
    if result1 == result2 {
        println!("✅ Both implementations produce identical results!");
    } else {
        println!("❌ Results differ! Check logic.");
    }
    
    println!("\n📋 To view assembly:");
    println!("  cargo asm -p aoc2024 --example day22_assembly_inspection --release mix_noinline");
    println!("  cargo asm -p aoc2024 --example day22_assembly_inspection --release prune_noinline");
    println!("  cargo asm -p aoc2024 --example day22_assembly_inspection --release prune_optimized");
    println!("  cargo asm -p aoc2024 --example day22_assembly_inspection --release multiply_64");
    println!("  cargo asm -p aoc2024 --example day22_assembly_inspection --release divide_32");
    println!("  cargo asm -p aoc2024 --example day22_assembly_inspection --release evolve_secret_noinline");
    println!("  cargo asm -p aoc2024 --example day22_assembly_inspection --release evolve_secret_optimized");
}
