//! # Chapter 7.1: Packages and Crates
//!
//! Demonstrates packages and crates from The Rust Book.
//!
//! Run with: `cargo run`

fn example1_what_is_package() {
    println!("📦 Example 1: What is a Package?");
    println!("=================================");

    // A package is a bundle of one or more crates
    // This file is a binary crate (has main function)
    // A package can contain:
    // - At most one library crate
    // - Any number of binary crates
    // - Must contain at least one crate (library or binary)

    println!("This is a binary crate within a package");
    println!("Package contains Cargo.toml file");
    println!("Crate is the compilation unit");
    println!();
}

fn example2_binary_vs_library() {
    println!("🔧 Example 2: Binary vs Library Crates");
    println!("======================================");

    // Binary crates:
    // - Have a main() function
    // - Can be executed
    // - Examples: command-line tools, servers

    // Library crates:
    // - Don't have main() function
    // - Provide functionality to other programs
    // - Examples: math libraries, web frameworks

    println!("Binary crate: Has main() function, can be executed");
    println!("Library crate: No main(), provides functionality to others");
    println!();
}

fn example3_cargo_new_structure() {
    println!("🏗️  Example 3: Cargo New Structure");
    println!("==================================");

    // When you run: cargo new my_project
    // Cargo creates:
    // my_project/
    // ├── Cargo.toml    (package configuration)
    // └── src/
    //     └── main.rs   (binary crate)

    println!("cargo new creates:");
    println!("  Cargo.toml - package configuration");
    println!("  src/main.rs - binary crate entry point");
    println!("  .gitignore - git ignore file");
    println!();
}

fn example4_package_manifest() {
    println!("📋 Example 4: Package Manifest (Cargo.toml)");
    println!("===========================================");

    // Cargo.toml is the package manifest
    // It contains metadata about the package
    println!("Package manifest contains:");
    println!("  [package] - package metadata");
    println!("  [dependencies] - external crates");
    println!("  [dev-dependencies] - test dependencies");
    println!("  [workspace] - workspace configuration");
    println!();
}

fn example5_workspace_example() {
    println!("🏢 Example 5: Workspace Example");
    println!("===============================");

    // A workspace is a collection of packages
    // All packages share a Cargo.lock file
    // All packages share a target directory

    println!("Workspace benefits:");
    println!("  Shared dependencies");
    println!("  Single lock file");
    println!("  Coordinated builds");
    println!("  Common configuration");
    println!();
}

fn main() {
    println!("📚 Chapter 7.1: Packages and Crates\n");

    example1_what_is_package();
    example2_binary_vs_library();
    example3_cargo_new_structure();
    example4_package_manifest();
    example5_workspace_example();

    println!("✅ All examples completed!");
    println!("📖 Next: Run examples in ../crates/");
}
