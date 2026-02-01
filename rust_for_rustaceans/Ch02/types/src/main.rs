// Main binary - runs all examples in sequence
use rustaceans_ch02_types::utils::print_section;

fn main() {
    print_section("RUST FOR RUSTACEANS - CHAPTER 2: TYPES");

    println!("\nRun individual examples with:");
    println!("  cargo run --example types_in_memory");
    println!("  cargo run --example dispatch_mechanisms");
    println!("  cargo run --example generic_traits");
    println!("  cargo run --example coherence_orphan_rule");
    println!("  cargo run --example trait_bounds_hrtb");
    println!("  cargo run --example marker_traits");
    println!("  cargo run --example existential_types");
    println!("  cargo run --example full_chapter");

    println!("\nOr run all examples:");
    println!("  cargo run --example full_chapter");
}
