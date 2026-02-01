// Example: Day 22 Assembly Inspection (Runtime Version)
//
// Uses runtime input to prevent constant folding, showing actual assembly instructions

use std::hint::black_box;

const PRUNE_MODULO: i64 = 16777216; // 2^24

/// Prune with modulo (compiler should optimize to AND)
#[inline(never)]
pub fn prune_modulo(secret: i64) -> i64 {
    secret % PRUNE_MODULO
}

/// Prune with explicit AND
#[inline(never)]
pub fn prune_and(secret: i64) -> i64 {
    secret & 0xFFFFFF
}

/// Multiply by 64 (should become: shl rax, 6)
#[inline(never)]
pub fn mul_64(val: i64) -> i64 {
    val * 64
}

/// Divide by 32 (should become: sar rax, 5)
#[inline(never)]
pub fn div_32(val: i64) -> i64 {
    val / 32
}

/// XOR operation
#[inline(never)]
pub fn xor_op(a: i64, b: i64) -> i64 {
    a ^ b
}

fn main() {
    // Use black_box to prevent constant folding
    let input = black_box(123i64);

    println!("Runtime value: {}", input);
    println!("prune_modulo: {}", prune_modulo(black_box(100000000)));
    println!("prune_and: {}", prune_and(black_box(100000000)));
    println!("mul_64: {}", mul_64(input));
    println!("div_32: {}", div_32(input));
    println!("xor_op: {}", xor_op(input, black_box(456)));

    println!("\n🔍 View assembly with:");
    println!("cargo asm -p aoc2024 --example day22_asm_runtime --release prune_modulo");
    println!("cargo asm -p aoc2024 --example day22_asm_runtime --release prune_and");
    println!("cargo asm -p aoc2024 --example day22_asm_runtime --release mul_64");
    println!("cargo asm -p aoc2024 --example day22_asm_runtime --release div_32");
    println!("cargo asm -p aoc2024 --example day22_asm_runtime --release xor_op");
}
