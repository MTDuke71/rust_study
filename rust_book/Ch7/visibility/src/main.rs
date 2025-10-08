//! # Chapter 7.5: Separating Modules into Different Files
//!
//! Demonstrates module organization and file structure from The Rust Book.
//!
//! Run with: `cargo run`

// This would typically be in separate files, but we'll show the structure here
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding customer to waitlist");
        }

        fn seat_at_table() {
            println!("Seating customer at table");
        }
    }

    pub mod serving {
        fn take_order() {
            println!("Taking order");
        }

        fn serve_order() {
            println!("Serving order");
        }

        fn take_payment() {
            println!("Taking payment");
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn example1_module_file_organization() {
    println!("📁 Example 1: Module File Organization");
    println!("=====================================");

    // Modules can be organized in different ways:
    // 1. Inline in the same file (like this example)
    // 2. In separate files with mod declarations
    // 3. In directories with mod.rs files

    println!("Module organization options:");
    println!("  1. Inline modules (mod {{ ... }})");
    println!("  2. Separate files (mod module_name;)");
    println!("  3. Directory modules (mod module_name/;)");
    println!();
}

fn example2_separate_files_structure() {
    println!("🗂️  Example 2: Separate Files Structure");
    println!("======================================");

    // For separate files, the structure would be:
    // src/
    // ├── main.rs
    // ├── front_of_house.rs
    // ├── front_of_house/
    // │   ├── mod.rs
    // │   ├── hosting.rs
    // │   └── serving.rs
    // └── back_of_house.rs

    println!("Separate files structure:");
    println!("  src/main.rs - crate root");
    println!("  src/front_of_house.rs - module file");
    println!("  src/front_of_house/ - module directory");
    println!("    ├── mod.rs - module root");
    println!("    ├── hosting.rs - submodule");
    println!("    └── serving.rs - submodule");
    println!();
}

fn example3_mod_declarations() {
    println!("📝 Example 3: mod Declarations");
    println!("=============================");

    // In main.rs, you would have:
    // mod front_of_house;  // tells Rust to look for front_of_house.rs
    // mod back_of_house;   // tells Rust to look for back_of_house.rs

    println!("mod declarations tell Rust where to find modules:");
    println!("  mod front_of_house;  // looks for front_of_house.rs");
    println!("  mod back_of_house;   // looks for back_of_house.rs");
    println!();
}

fn example4_module_directories() {
    println!("📂 Example 4: Module Directories");
    println!("===============================");

    // For complex modules, use directories:
    // front_of_house/
    // ├── mod.rs          // module root, contains submodule declarations
    // ├── hosting.rs      // submodule file
    // └── serving.rs      // submodule file

    println!("Module directories for complex modules:");
    println!("  front_of_house/mod.rs - contains 'pub mod hosting;'");
    println!("  front_of_house/hosting.rs - hosting module implementation");
    println!("  front_of_house/serving.rs - serving module implementation");
    println!();
}

fn example5_public_api_design() {
    println!("🎯 Example 5: Public API Design");
    println!("==============================");

    // Design your public API carefully
    // Use pub use to re-export for convenience
    // Hide internal implementation details

    // Re-export commonly used items
    pub use crate::front_of_house::hosting;

    // This makes the API more convenient for users
    hosting::add_to_waitlist();

    println!("Public API design principles:");
    println!("  1. Use pub use to re-export common items");
    println!("  2. Hide internal implementation details");
    println!("  3. Make the API convenient for users");
    println!("  4. Keep the internal structure flexible");
    println!();
}

fn example6_workspace_organization() {
    println!("🏢 Example 6: Workspace Organization");
    println!("===================================");

    // For large projects, use workspaces:
    // my_project/
    // ├── Cargo.toml          (workspace root)
    // ├── src/
    // │   └── main.rs
    // ├── front_of_house/
    // │   ├── Cargo.toml      (package)
    // │   └── src/
    // │       └── lib.rs
    // └── back_of_house/
    //     ├── Cargo.toml      (package)
    //     └── src/
    //         └── lib.rs

    println!("Workspace organization for large projects:");
    println!("  - Multiple packages in one workspace");
    println!("  - Shared dependencies and configuration");
    println!("  - Coordinated builds and testing");
    println!("  - Clear separation of concerns");
    println!();
}

fn example7_best_practices() {
    println!("✅ Example 7: Module Organization Best Practices");
    println!("===============================================");

    println!("Best practices for module organization:");
    println!("  1. Start simple with inline modules");
    println!("  2. Move to separate files as modules grow");
    println!("  3. Use directories for complex modules");
    println!("  4. Design clear public APIs");
    println!("  5. Use workspaces for large projects");
    println!("  6. Keep related functionality together");
    println!("  7. Use descriptive module names");
    println!();
}

fn main() {
    println!("📚 Chapter 7.5: Separating Modules into Different Files\n");

    example1_module_file_organization();
    example2_separate_files_structure();
    example3_mod_declarations();
    example4_module_directories();
    example5_public_api_design();
    example6_workspace_organization();
    example7_best_practices();

    println!("✅ All examples completed!");
    println!("📖 Next: Read Chapter 8 or run examples in other chapters");
}
