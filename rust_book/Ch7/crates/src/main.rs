//! # Chapter 7.2: Defining Modules to Control Scope and Privacy
//!
//! Demonstrates module system from The Rust Book.
//!
//! Run with: `cargo run`

// Define a module
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding customer to waitlist");
        }

        fn seat_at_table() {
            println!("Seating customer at table");
        }
    }

    mod serving {
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

fn example1_basic_module() {
    println!("🏠 Example 1: Basic Module Definition");
    println!("====================================");

    // Modules help organize code and control privacy
    // front_of_house is a module containing other modules
    // hosting is a public module (pub mod)
    // serving is a private module (no pub)

    println!("Module structure:");
    println!("  front_of_house (module)");
    println!("    ├── hosting (pub mod)");
    println!("    │   ├── add_to_waitlist (pub fn)");
    println!("    │   └── seat_at_table (private fn)");
    println!("    └── serving (private mod)");
    println!();
}

fn example2_module_privacy() {
    println!("🔒 Example 2: Module Privacy Rules");
    println!("==================================");

    // Privacy rules:
    // 1. All items (functions, structs, enums, etc.) are private by default
    // 2. Items in a parent module can't use private items in child modules
    // 3. Items in child modules can use items in ancestor modules
    // 4. Use pub to make items public

    println!("Privacy rules:");
    println!("  - Items are private by default");
    println!("  - Use 'pub' to make items public");
    println!("  - Child modules can access ancestor items");
    println!("  - Parent modules can't access private child items");
    println!();
}

fn example3_calling_module_functions() {
    println!("📞 Example 3: Calling Module Functions");
    println!("=====================================");

    // Call public functions from modules
    front_of_house::hosting::add_to_waitlist();

    // This won't work - seat_at_table is private:
    // front_of_house::hosting::seat_at_table();

    println!("✅ Successfully called public function");
    println!("❌ Cannot call private function from outside module");
    println!();
}

fn example4_module_tree() {
    println!("🌳 Example 4: Module Tree Structure");
    println!("==================================");

    // The module tree starts with the crate root
    // For binary crates, the crate root is src/main.rs
    // For library crates, the crate root is src/lib.rs

    println!("Module tree for this binary crate:");
    println!("crate (src/main.rs)");
    println!("└── front_of_house");
    println!("    ├── hosting");
    println!("    └── serving");
    println!();
}

fn example5_absolute_vs_relative_paths() {
    println!("🛤️  Example 5: Absolute vs Relative Paths");
    println!("=========================================");

    // Absolute path: starts with crate
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path: starts with module name
    front_of_house::hosting::add_to_waitlist();

    println!("Both paths work the same way:");
    println!("  Absolute: crate::front_of_house::hosting::add_to_waitlist()");
    println!("  Relative: front_of_house::hosting::add_to_waitlist()");
    println!();
}

fn main() {
    println!("📚 Chapter 7.2: Defining Modules to Control Scope and Privacy\n");

    example1_basic_module();
    example2_module_privacy();
    example3_calling_module_functions();
    example4_module_tree();
    example5_absolute_vs_relative_paths();

    println!("✅ All examples completed!");
    println!("📖 Next: Run examples in ../modules/");
}
