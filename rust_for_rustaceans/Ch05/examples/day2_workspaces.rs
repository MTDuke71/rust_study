//! Day 2: Workspaces - Multi-Crate Project Organization
//!
//! This example demonstrates workspace concepts through documentation and simulation.
//! Real workspaces require directory structures that are best explored in the parent project.

/// Simulates a core library that would be in a workspace
pub mod core_lib {
    pub fn core_functionality() -> &'static str {
        "Core library functionality"
    }

    pub fn shared_utility(x: i32, y: i32) -> i32 {
        x + y
    }
}

/// Simulates a utilities crate that would be in a workspace
pub mod utilities {
    pub fn helper_function(s: &str) -> String {
        format!("Processed: {}", s)
    }

    pub fn common_algorithm(data: &[i32]) -> i32 {
        data.iter().sum()
    }
}

/// Simulates an API crate that depends on both core and utilities
pub mod api {
    use super::{core_lib, utilities};

    pub fn api_endpoint(input: &str) -> String {
        let core_msg = core_lib::core_functionality();
        let processed = utilities::helper_function(input);
        format!("{} | {}", core_msg, processed)
    }

    pub fn api_calculation(numbers: &[i32]) -> i32 {
        let sum = utilities::common_algorithm(numbers);
        core_lib::shared_utility(sum, 100)
    }
}

fn main() {
    println!("🎯 Day 2: Workspaces Example\n");

    println!("=== Workspace Structure ===");
    println!("my-workspace/");
    println!("├── Cargo.toml         (workspace root)");
    println!("├── Cargo.lock         (shared lock file)");
    println!("├── target/            (shared build directory)");
    println!("├── core-lib/");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/lib.rs");
    println!("├── utilities/");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/lib.rs");
    println!("└── api/");
    println!("    ├── Cargo.toml     (depends on core-lib and utilities)");
    println!("    └── src/lib.rs\n");

    println!("=== Workspace Cargo.toml ===");
    println!("[workspace]");
    println!("members = [");
    println!("    \"core-lib\",");
    println!("    \"utilities\",");
    println!("    \"api\",");
    println!("]");
    println!("resolver = \"2\"\n");

    println!("=== Path Dependencies ===");
    println!("# In api/Cargo.toml:");
    println!("[dependencies]");
    println!("core-lib = {{ path = \"../core-lib\" }}");
    println!("utilities = {{ path = \"../utilities\" }}\n");

    println!("=== Simulated Workspace Execution ===\n");

    // Simulate using workspace crates
    println!("1. Core library:");
    let core_result = core_lib::core_functionality();
    println!("   {}", core_result);

    println!("\n2. Utilities:");
    let util_result = utilities::helper_function("test data");
    println!("   {}", util_result);

    println!("\n3. API (using both core and utilities):");
    let api_result = api::api_endpoint("user input");
    println!("   {}", api_result);

    let numbers = vec![1, 2, 3, 4, 5];
    let calc_result = api::api_calculation(&numbers);
    println!("   Calculation: {:?} -> {}", numbers, calc_result);

    println!("\n=== Workspace Benefits ===");
    println!("✓ Single Cargo.lock for all crates");
    println!("✓ Shared target/ directory (compile dependencies once)");
    println!("✓ Easy cross-crate refactoring");
    println!("✓ Consistent dependency versions");
    println!("✓ Fast incremental builds");

    println!("\n=== Real-World Example ===");
    println!("The rust_study workspace has 80+ crates:");
    println!("  - missions/Mission*");
    println!("  - tutorials/Mission*_tut");
    println!("  - rust_book/Ch*/*");
    println!("  - advent_of_code/aoc*");
    println!("\nAll sharing one Cargo.lock and target/ directory!");

    println!("\n📝 Common Commands:");
    println!("  cargo build --workspace          # Build all crates");
    println!("  cargo test --workspace           # Test all crates");
    println!("  cargo build -p core-lib          # Build specific crate");
    println!("  cargo tree                       # Show dependency tree");
}
