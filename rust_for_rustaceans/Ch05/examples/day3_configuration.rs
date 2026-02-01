//! Day 3: Project Configuration - Metadata, Profiles, and Patching
//!
//! This example demonstrates:
//! - Crate metadata configuration
//! - Build profiles (dev vs release)
//! - Performance implications
//! - Dependency patching concepts

use std::hint::black_box;
use std::time::Instant;

/// Demonstrates the impact of optimization levels
fn optimization_demo() {
    println!("=== Optimization Level Demo ===\n");

    let iterations = 1_000_000;

    // NOTE: Without black_box, the compiler performs constant folding!
    // In release mode, LLVM recognizes the arithmetic series pattern
    // and replaces the entire loop with: sum = n*(n-1)/2 = 499,999,500,000
    // This results in a single `mov` instruction instead of a loop.
    // See assembly: `cargo rustc --example day3_configuration --release -- --emit asm`
    // Line: `movabsq $499999500000, %rax`
    //
    // Using black_box prevents this optimization, forcing actual iteration.
    //
    // Performance comparison (1M iterations):
    // - Debug mode (opt-level = 0):   ~3.6 ms  (no optimizations)
    // - Release mode (opt-level = 3): ~202 µs  (18x faster with optimizations)
    // - Without black_box (release):  ~100 ns  (constant folding - not measuring loop!)
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..black_box(iterations) {
        sum += black_box(i);
    }
    let duration = start.elapsed();

    println!("Sum of 0..{}: {}", iterations, sum);
    println!("Time: {:?}", duration);

    if cfg!(debug_assertions) {
        println!("✓ Running in DEBUG mode (opt-level = 0)");
        println!("  Fast compilation, slow execution");
    } else {
        println!("✓ Running in RELEASE mode (opt-level = 3)");
        println!("  Slow compilation, fast execution");
    }
}

/// Demonstrates panic behavior configuration
fn panic_demo() {
    println!("\n=== Panic Behavior ===\n");

    if cfg!(debug_assertions) {
        println!("Panic mode: unwind (default in dev)");
        println!("  - Stack unwinding enabled");
        println!("  - Drop destructors run");
        println!("  - Larger binary size");
    } else {
        println!("Panic mode: can be set to 'abort' in release");
        println!("  - Immediate program termination");
        println!("  - No stack unwinding");
        println!("  - Smaller binary size");
    }
}

/// Simulates code-generation unit impact
fn codegen_units_demo() {
    println!("\n=== Codegen Units ===\n");

    println!("Dev profile (default: 256 units):");
    println!("  ✓ Parallel compilation");
    println!("  ✓ Fast builds");
    println!("  ✗ Suboptimal code generation");

    println!("\nRelease profile (recommended: 1 unit):");
    println!("  ✓ Better optimization across crate");
    println!("  ✓ Smaller binary");
    println!("  ✗ Slower compilation");
}

/// Demonstrates LTO (Link-Time Optimization)
fn lto_demo() {
    println!("\n=== Link-Time Optimization (LTO) ===\n");

    println!("Options:");
    println!("  lto = false    # No LTO (fast build)");
    println!("  lto = \"thin\"    # Thin LTO (good balance)");
    println!("  lto = true     # Full LTO (best optimization, slowest)");

    println!("\nBenefits:");
    println!("  ✓ Cross-crate inlining");
    println!("  ✓ Dead code elimination");
    println!("  ✓ Smaller binary size");
    println!("  ✗ Much longer build times");
}

fn main() {
    println!("🎯 Day 3: Project Configuration Example\n");

    // Show actual profile being used
    optimization_demo();
    panic_demo();
    codegen_units_demo();
    lto_demo();

    println!("\n=== Cargo.toml Configuration Examples ===\n");

    println!("1. Crate Metadata:");
    println!("   [package]");
    println!("   name = \"my-crate\"");
    println!("   version = \"1.0.0\"");
    println!("   edition = \"2021\"");
    println!("   rust-version = \"1.70\"");
    println!("   include = [\"src/**/*\", \"LICENSE\", \"README.md\"]");

    println!("\n2. Dev Profile (fast builds):");
    println!("   [profile.dev]");
    println!("   opt-level = 0");
    println!("   debug = true");

    println!("\n3. Release Profile (optimized):");
    println!("   [profile.release]");
    println!("   opt-level = 3");
    println!("   lto = true");
    println!("   codegen-units = 1");
    println!("   strip = true  # Remove debug symbols");

    println!("\n4. Custom Profile (balanced):");
    println!("   [profile.dev.package.\"*\"]");
    println!("   opt-level = 1  # Optimize dependencies in dev mode");

    println!("\n=== Dependency Patching ===\n");

    println!("Use [patch] to override dependencies:");
    println!("   [patch.crates-io]");
    println!("   tokio = {{ git = \"https://github.com/tokio-rs/tokio\", branch = \"main\" }}");
    println!("   serde = {{ path = \"../serde-local-fix\" }}");

    println!("\nCommon use cases:");
    println!("  • Testing unreleased bug fixes");
    println!("  • Using local forks");
    println!("  • Applying patches to transitive dependencies");

    println!("\n=== Build Performance Tips ===\n");

    println!("1. Speed up debug builds:");
    println!("   [profile.dev.package.\"*\"]");
    println!("   opt-level = 1  # Optimize dependencies");

    println!("\n2. Faster release builds:");
    println!("   [profile.release]");
    println!("   lto = \"thin\"  # Instead of full LTO");
    println!("   codegen-units = 16  # Instead of 1");

    println!("\n3. Minimize binary size:");
    println!("   [profile.release]");
    println!("   opt-level = \"z\"  # Optimize for size");
    println!("   lto = true");
    println!("   strip = true");
    println!("   panic = \"abort\"");

    println!("\n📝 Try comparing build times and binary sizes:");
    println!("  cargo build                      # Debug build");
    println!("  cargo build --release            # Release build");
    println!("  ls -lh target/debug/rfr-ch05-project-structure");
    println!("  ls -lh target/release/rfr-ch05-project-structure");
}
